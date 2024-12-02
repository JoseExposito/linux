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
	struct drm_encoder *encoder;
	struct vkms_crtc *vkms_crtc;
	struct vkms_plane *primary = NULL, *cursor = NULL;
	struct vkms_config_plane *plane_cfg;
	int ret;
	int writeback;

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, link) {
		plane_cfg->plane = vkms_plane_init(vkmsdev, plane_cfg->type);
		if (IS_ERR(plane_cfg->plane)) {
			DRM_DEV_ERROR(dev->dev, "Failed to init vkms plane\n");
			return PTR_ERR(plane_cfg->plane);
		}

		if (plane_cfg->type == DRM_PLANE_TYPE_PRIMARY)
			primary = plane_cfg->plane;
		else if (plane_cfg->type == DRM_PLANE_TYPE_CURSOR)
			cursor = plane_cfg->plane;
	}

	vkms_crtc = vkms_crtc_init(dev, &primary->base,
				   cursor ? &cursor->base : NULL);
	if (IS_ERR(vkms_crtc)) {
		DRM_ERROR("Failed to allocate CRTC\n");
		return PTR_ERR(vkms_crtc);
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

	encoder = drmm_kzalloc(dev, sizeof(*encoder), GFP_KERNEL);
	if (!encoder) {
		DRM_ERROR("Failed to allocate encoder\n");
		return -ENOMEM;
	}
	ret = drmm_encoder_init(dev, encoder, NULL,
				DRM_MODE_ENCODER_VIRTUAL, NULL);
	if (ret) {
		DRM_ERROR("Failed to init encoder\n");
		return ret;
	}
	encoder->possible_crtcs = drm_crtc_mask(&vkms_crtc->base);

	/* Attach the encoder and the connector */
	ret = drm_connector_attach_encoder(connector, encoder);
	if (ret) {
		DRM_ERROR("Failed to attach connector to encoder\n");
		return ret;
	}

	/* Initialize the writeback component */
	if (vkmsdev->config->writeback) {
		writeback = vkms_enable_writeback_connector(vkmsdev, vkms_crtc);
		if (writeback) {
			DRM_ERROR("Failed to init writeback connector\n");
			return ret;
		}
	}

	drm_mode_config_reset(dev);

	return 0;
}
