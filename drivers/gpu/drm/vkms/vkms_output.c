// SPDX-License-Identifier: GPL-2.0+

#include "vkms_config.h"
#include "vkms_connector.h"
#include "vkms_drv.h"
#include <drm/drm_managed.h>

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
	if (IS_ERR(plane_cfgs))
		return PTR_ERR(plane_cfgs);

	crtc_cfgs = vkms_config_get_crtcs(vkmsdev->config, &n_crtcs);
	if (IS_ERR(crtc_cfgs)) {
		ret = PTR_ERR(crtc_cfgs);
		goto err_free;
	}

	encoder_cfgs = vkms_config_get_encoders(vkmsdev->config, &n_encoders);
	if (IS_ERR(encoder_cfgs)) {
		ret = PTR_ERR(encoder_cfgs);
		goto err_free;
	}

	connector_cfgs = vkms_config_get_connectors(vkmsdev->config,
						    &n_connectors);
	if (IS_ERR(connector_cfgs)) {
		ret = PTR_ERR(connector_cfgs);
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
			if (writeback)
				DRM_ERROR("Failed to init writeback connector\n");
		}
	}

	for (n = 0; n < n_planes; n++) {
		struct vkms_config_plane *plane_cfg;
		struct vkms_config_crtc **possible_crtcs;
		size_t n_possible_crtcs;

		plane_cfg = plane_cfgs[n];
		possible_crtcs = vkms_config_plane_get_possible_crtcs(plane_cfg,
								      &n_possible_crtcs);
		if (IS_ERR(possible_crtcs)) {
			ret = PTR_ERR(possible_crtcs);
			goto err_free;
		}

		for (i = 0; i < n_possible_crtcs; i++) {
			struct vkms_config_crtc *possible_crtc;

			possible_crtc = possible_crtcs[i];
			plane_cfg->plane->base.possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->crtc);
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
		if (IS_ERR(possible_crtcs)) {
			ret = PTR_ERR(possible_crtcs);
			goto err_free;
		}

		for (i = 0; i < n_possible_crtcs; i++) {
			struct vkms_config_crtc *possible_crtc;

			possible_crtc = possible_crtcs[i];
			encoder_cfg->encoder->possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->crtc);
		}

		kfree(possible_crtcs);
	}

	for (n = 0; n < n_connectors; n++) {
		struct vkms_config_connector *connector_cfg;
		struct vkms_config_encoder **possible_encoders;
		size_t n_possible_encoders;

		connector_cfg = connector_cfgs[n];

		connector_cfg->connector = vkms_connector_init(vkmsdev);
		if (IS_ERR(connector_cfg->connector)) {
			DRM_ERROR("Failed to init connector\n");
			ret = PTR_ERR(connector_cfg->connector);
			goto err_free;
		}

		possible_encoders =
			vkms_config_connector_get_possible_encoders(connector_cfg,
								    &n_possible_encoders);
		if (IS_ERR(possible_encoders)) {
			ret = PTR_ERR(possible_encoders);
			goto err_free;
		}

		for (i = 0; i < n_possible_encoders; i++) {
			struct vkms_config_encoder *possible_encoder;

			possible_encoder = possible_encoders[i];
			ret = drm_connector_attach_encoder(&connector_cfg->connector->base,
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
