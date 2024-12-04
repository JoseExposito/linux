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
	struct drm_connector *connector;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;
	int ret;
	int writeback;

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, link) {
		plane_cfg->plane = vkms_plane_init(vkmsdev, plane_cfg->type);
		if (IS_ERR(plane_cfg->plane)) {
			DRM_DEV_ERROR(dev->dev, "Failed to init vkms plane\n");
			return PTR_ERR(plane_cfg->plane);
		}
	}

	list_for_each_entry(crtc_cfg, &vkmsdev->config->crtcs, link) {
		struct vkms_config_plane *primary, *cursor;

		primary = vkms_config_crtc_primary_plane(vkmsdev->config, crtc_cfg);
		cursor = vkms_config_crtc_cursor_plane(vkmsdev->config, crtc_cfg);

		crtc_cfg->crtc = vkms_crtc_init(dev, &primary->plane->base,
						cursor ? &cursor->plane->base : NULL);
		if (IS_ERR(crtc_cfg->crtc)) {
			DRM_ERROR("Failed to allocate CRTC\n");
			return PTR_ERR(crtc_cfg->crtc);
		}

		/* Initialize the writeback component */
		if (crtc_cfg->writeback) {
			writeback = vkms_enable_writeback_connector(vkmsdev, crtc_cfg->crtc);
			if (writeback) {
				DRM_ERROR("Failed to init writeback connector\n");
				return ret;
			}
		}
	}

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, link) {
		struct vkms_config_crtc *possible_crtc;
		unsigned long idx;

		xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
			plane_cfg->plane->base.possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->base);
		}
	}

	list_for_each_entry(encoder_cfg, &vkmsdev->config->encoders, link) {
		struct vkms_config_crtc *possible_crtc;
		unsigned long idx;

		encoder_cfg->encoder = drmm_kzalloc(dev, sizeof(*encoder_cfg->encoder), GFP_KERNEL);
		if (!encoder_cfg->encoder) {
			DRM_ERROR("Failed to allocate encoder\n");
			return -ENOMEM;
		}
		ret = drmm_encoder_init(dev, encoder_cfg->encoder, NULL,
					DRM_MODE_ENCODER_VIRTUAL, NULL);
		if (ret) {
			DRM_ERROR("Failed to init encoder\n");
			return ret;
		}

		xa_for_each(&encoder_cfg->possible_crtcs, idx, possible_crtc) {
			encoder_cfg->encoder->possible_crtcs |=
				drm_crtc_mask(&possible_crtc->crtc->base);
		}
	}

	connector = drmm_kzalloc(dev, sizeof(*connector), GFP_KERNEL);
	if (!connector) {
		DRM_ERROR("Failed to allocate connector\n");
		return -ENOMEM;
	}

	ret = drmm_connector_init(dev, connector, &vkms_connector_funcs,
				  DRM_MODE_CONNECTOR_VIRTUAL, NULL);
	if (ret) {
		DRM_ERROR("Failed to init connector\n");
		return ret;
	}

	drm_connector_helper_add(connector, &vkms_conn_helper_funcs);

	/* Attach the encoder and the connector */
	encoder_cfg = list_first_entry(&vkmsdev->config->encoders, typeof(*encoder_cfg), link);
	ret = drm_connector_attach_encoder(connector, encoder_cfg->encoder);
	if (ret) {
		DRM_ERROR("Failed to attach connector to encoder\n");
		return ret;
	}

	drm_mode_config_reset(dev);

	return 0;
}
