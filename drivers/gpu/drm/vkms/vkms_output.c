// SPDX-License-Identifier: GPL-2.0+

#include "vkms_config.h"
#include "vkms_drv.h"
#include <drm/drm_atomic_helper.h>
#include <drm/drm_edid.h>
#include <drm/drm_managed.h>
#include <drm/drm_probe_helper.h>

static const struct drm_connector_funcs vkms_connector_funcs = {
	.fill_modes = drm_helper_probe_single_connector_modes,
	.reset = drm_atomic_helper_connector_reset,
	.atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
	.atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
};

static int vkms_conn_get_modes(struct drm_connector *connector)
{
	int count;

	/* Use the default modes list from DRM */
	count = drm_add_modes_noedid(connector, XRES_MAX, YRES_MAX);
	drm_set_preferred_mode(connector, XRES_DEF, YRES_DEF);

	return count;
}

static const struct drm_connector_helper_funcs vkms_conn_helper_funcs = {
	.get_modes    = vkms_conn_get_modes,
};

int vkms_output_init(struct vkms_device *vkmsdev)
{
	struct drm_device *dev = &vkmsdev->drm;
	struct vkms_config_plane **plane_cfgs = NULL;
	size_t n_planes;
	struct vkms_config_crtc **crtc_cfgs = NULL;
	size_t n_crtcs;
	struct vkms_config_encoder **encoder_cfgs = NULL;
	size_t n_encoders;
	struct vkms_config_connector **connector_cfgs = NULL;
	size_t n_connectors;
	int ret = 0;
	int writeback;
	unsigned int n, i;

	plane_cfgs = vkms_config_get_planes(vkmsdev->config, &n_planes);
	if (!plane_cfgs)
		return -ENOMEM;

	crtc_cfgs = vkms_config_get_crtcs(vkmsdev->config, &n_crtcs);
	if (!crtc_cfgs) {
		ret = -ENOMEM;
		goto err_free;
	}

	encoder_cfgs = vkms_config_get_encoders(vkmsdev->config, &n_encoders);
	if (!encoder_cfgs) {
		ret = -ENOMEM;
		goto err_free;
	}

	connector_cfgs = vkms_config_get_connectors(vkmsdev->config,
						    &n_connectors);
	if (!connector_cfgs) {
		ret = -ENOMEM;
		goto err_free;
	}

	for (n = 0; n < n_planes; n++) {
		struct vkms_config_plane *plane_cfg;
		enum drm_plane_type type;

		plane_cfg = plane_cfgs[n];
		type = vkms_config_plane_get_type(plane_cfg);

		plane_cfg->plane = vkms_plane_init(vkmsdev, type);
		if (IS_ERR(plane_cfg->plane)) {
			DRM_DEV_ERROR(dev->dev, "Failed to init vkms plane\n");
			ret = PTR_ERR(plane_cfg->plane);
			goto err_free;
		}
	}

	for (n = 0; n < n_crtcs; n++) {
		struct vkms_config_crtc *crtc_cfg;
		struct vkms_config_plane *primary, *cursor;

		crtc_cfg = crtc_cfgs[n];
		primary = vkms_config_crtc_primary_plane(vkmsdev->config, crtc_cfg);
		cursor = vkms_config_crtc_cursor_plane(vkmsdev->config, crtc_cfg);

		crtc_cfg->crtc = vkms_crtc_init(dev, &primary->plane->base,
						cursor ? &cursor->plane->base : NULL);
		if (IS_ERR(crtc_cfg->crtc)) {
			DRM_ERROR("Failed to allocate CRTC\n");
			ret = PTR_ERR(crtc_cfg->crtc);
			goto err_free;
		}

		/* Initialize the writeback component */
		if (vkms_config_crtc_get_writeback(crtc_cfg)) {
			writeback = vkms_enable_writeback_connector(vkmsdev, crtc_cfg->crtc);
			if (writeback) {
				DRM_ERROR("Failed to init writeback connector\n");
				goto err_free;
			}
		}
	}

	for (n = 0; n < n_planes; n++) {
		struct vkms_config_plane *plane_cfg;
		struct vkms_config_crtc **possible_crtcs;
		size_t n_possible_crtcs;

		plane_cfg = plane_cfgs[n];
		possible_crtcs = vkms_config_plane_get_possible_crtcs(plane_cfg,
								      &n_possible_crtcs);
		if (!possible_crtcs) {
			ret = -ENOMEM;
			goto err_free;
		}

		for (i = 0; i < n_possible_crtcs; i++) {
			struct vkms_config_crtc *possible_crtc;

			possible_crtc = possible_crtcs[i];
			plane_cfg->plane->base.possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->base);
		}

		kfree(possible_crtcs);
	}

	for (n = 0; n < n_encoders; n++) {
		struct vkms_config_encoder *encoder_cfg;
		struct vkms_config_crtc **possible_crtcs;
		size_t n_possible_crtcs;

		encoder_cfg = encoder_cfgs[n];

		encoder_cfg->encoder = drmm_kzalloc(dev, sizeof(*encoder_cfg->encoder), GFP_KERNEL);
		if (!encoder_cfg->encoder) {
			DRM_ERROR("Failed to allocate encoder\n");
			ret = -ENOMEM;
			goto err_free;
		}
		ret = drmm_encoder_init(dev, encoder_cfg->encoder, NULL,
					DRM_MODE_ENCODER_VIRTUAL, NULL);
		if (ret) {
			DRM_ERROR("Failed to init encoder\n");
			goto err_free;
		}

		possible_crtcs = vkms_config_encoder_get_possible_crtcs(encoder_cfg,
									&n_possible_crtcs);
		if (!possible_crtcs) {
			ret = -ENOMEM;
			goto err_free;
		}

		for (i = 0; i < n_possible_crtcs; i++) {
			struct vkms_config_crtc *possible_crtc;

			possible_crtc = possible_crtcs[i];
			encoder_cfg->encoder->possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->base);
		}

		kfree(possible_crtcs);
	}

	for (n = 0; n < n_connectors; n++) {
		struct vkms_config_connector *connector_cfg;
		struct vkms_config_encoder **possible_encoders;
		size_t n_possible_encoders;

		connector_cfg = connector_cfgs[n];

		connector_cfg->connector = drmm_kzalloc(dev,
							sizeof(*connector_cfg->connector),
							GFP_KERNEL);
		if (!connector_cfg->connector) {
			DRM_ERROR("Failed to allocate connector\n");
			ret = -ENOMEM;
			goto err_free;
		}

		ret = drmm_connector_init(dev, connector_cfg->connector,
					  &vkms_connector_funcs,
					  DRM_MODE_CONNECTOR_VIRTUAL, NULL);
		if (ret) {
			DRM_ERROR("Failed to init connector\n");
			goto err_free;
		}

		drm_connector_helper_add(connector_cfg->connector,
					 &vkms_conn_helper_funcs);

		possible_encoders =
			vkms_config_connector_get_possible_encoders(connector_cfg,
								    &n_possible_encoders);
		if (!possible_encoders) {
			ret = -ENOMEM;
			goto err_free;
		}

		for (i = 0; i < n_possible_encoders; i++) {
			struct vkms_config_encoder *possible_encoder;

			possible_encoder = possible_encoders[i];
			ret = drm_connector_attach_encoder(connector_cfg->connector,
							   possible_encoder->encoder);
			if (ret) {
				DRM_ERROR("Failed to attach connector to encoder\n");
				kfree(possible_encoders);
				goto err_free;
			}
		}

		kfree(possible_encoders);
	}

	drm_mode_config_reset(dev);

err_free:
	kfree(plane_cfgs);
	kfree(crtc_cfgs);
	kfree(encoder_cfgs);
	kfree(connector_cfgs);

	return ret;
}
