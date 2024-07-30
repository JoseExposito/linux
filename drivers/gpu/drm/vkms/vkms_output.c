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

	count = drm_add_modes_noedid(connector, XRES_MAX, YRES_MAX);
	drm_set_preferred_mode(connector, XRES_DEF, YRES_DEF);

	return count;
}

static const struct drm_connector_helper_funcs vkms_conn_helper_funcs = {
	.get_modes    = vkms_conn_get_modes,
};

static struct drm_connector *vkms_connector_init(struct vkms_device *vkms_device,
						 uint32_t possible_encoders)
{
	struct drm_connector *connector;
	int ret;

	connector = drmm_kzalloc(&vkms_device->drm, sizeof(*connector), GFP_KERNEL);
	if (!connector) {
		DRM_ERROR("Failed to allocate connector\n");
		return ERR_PTR(-ENOMEM);
	}

	ret = drmm_connector_init(&vkms_device->drm, connector,
				  &vkms_connector_funcs,
				  DRM_MODE_CONNECTOR_VIRTUAL, NULL);
	if (ret) {
		DRM_ERROR("Failed to init connector\n");
		kfree(connector);
		return ERR_PTR(ret);
	}

	connector->possible_encoders = possible_encoders;
	drm_connector_helper_add(connector, &vkms_conn_helper_funcs);

	return connector;
}

static struct drm_encoder *vkms_encoder_init(struct vkms_device *vkms_device,
					     uint32_t possible_crtcs)
{
	struct drm_encoder *encoder;
	int ret;

	encoder = drmm_kzalloc(&vkms_device->drm, sizeof(*encoder), GFP_KERNEL);
	if (!encoder) {
		DRM_ERROR("Failed to allocate encoder\n");
		return ERR_PTR(-ENOMEM);
	}

	ret = drmm_encoder_init(&vkms_device->drm, encoder, NULL,
				DRM_MODE_ENCODER_VIRTUAL, NULL);
	if (ret) {
		DRM_ERROR("Failed to init encoder\n");
		kfree(encoder);
		return ERR_PTR(ret);
	}

	encoder->possible_crtcs = possible_crtcs;

	return encoder;
}

static int vkms_add_overlay_plane(struct vkms_device *vkmsdev,
				  uint32_t possible_crtcs)
{
	struct vkms_plane *overlay;

	overlay = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_OVERLAY, possible_crtcs);
	if (IS_ERR(overlay))
		return PTR_ERR(overlay);

	if (!overlay->base.possible_crtcs)
		overlay->base.possible_crtcs = possible_crtcs;

	return 0;
}

int vkms_output_init(struct vkms_device *vkmsdev)
{
	struct drm_device *dev = &vkmsdev->drm;
	struct drm_connector *connector;
	struct vkms_config_connector *connector_cfg;
	struct drm_encoder *encoder;
	struct vkms_config_encoder *encoder_cfg;
	struct vkms_crtc *vkms_crtc;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_plane *primary, *cursor = NULL;
	struct vkms_config_plane *plane_cfg;
	int ret;
	int writeback;

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, list) {
		ret = vkms_add_overlay_plane(vkmsdev, plane_cfg->possible_crtcs);
		if (ret)
			return ret;
	}

	list_for_each_entry(crtc_cfg, &vkmsdev->config->crtcs, list) {
		primary = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_PRIMARY, 0);
		if (IS_ERR(primary))
			return PTR_ERR(primary);

		if (crtc_cfg->cursor) {
			cursor = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_CURSOR, 0);
			if (IS_ERR(cursor))
				return PTR_ERR(cursor);
		}

		vkms_crtc = vkms_crtc_init(dev, &primary->base, &cursor->base);
		if (IS_ERR(vkms_crtc))
			return PTR_ERR(vkms_crtc);

		list_add_tail(&vkms_crtc->list, &vkmsdev->crtcs);

		if (crtc_cfg->writeback) {
			writeback = vkms_enable_writeback_connector(vkms_crtc);
			if (writeback)
				DRM_ERROR("Failed to init writeback connector\n");
		}

		cursor = NULL;
	}

	list_for_each_entry(encoder_cfg, &vkmsdev->config->encoders, list) {
		encoder = vkms_encoder_init(vkmsdev, encoder_cfg->possible_crtcs);
		if (IS_ERR(encoder))
			return PTR_ERR(encoder);
	}

	list_for_each_entry(connector_cfg, &vkmsdev->config->connectors, list) {
		connector = vkms_connector_init(vkmsdev, connector_cfg->possible_encoders);
		if (IS_ERR(connector))
			return PTR_ERR(connector);
	}

	drm_mode_config_reset(dev);

	return 0;
}
