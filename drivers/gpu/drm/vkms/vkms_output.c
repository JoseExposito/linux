// SPDX-License-Identifier: GPL-2.0+

#include "vkms_config.h"
#include "vkms_drv.h"
#include <drm/drm_atomic_helper.h>
#include <drm/drm_edid.h>
#include <drm/drm_managed.h>
#include <drm/drm_probe_helper.h>

static const struct drm_connector_funcs vkms_connector_funcs = {
	.fill_modes = drm_helper_probe_single_connector_modes,
	.destroy = drm_connector_cleanup,
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

static int vkms_add_overlay_plane(struct vkms_device *vkmsdev, int index)
{
	struct vkms_plane *overlay;

	overlay = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_OVERLAY, index);
	if (IS_ERR(overlay))
		return PTR_ERR(overlay);

	if (!overlay->base.possible_crtcs)
		overlay->base.possible_crtcs = BIT(index);

	return 0;
}

int vkms_output_init(struct vkms_device *vkmsdev, int index)
{
	struct vkms_output *output = &vkmsdev->output;
	struct drm_device *dev = &vkmsdev->drm;
	struct drm_connector *connector = &output->connector;
	struct drm_encoder *encoder;
	struct vkms_crtc *vkms_crtc;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_plane *primary, *cursor = NULL;
	int ret;
	int writeback;
	unsigned int n;

	primary = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_PRIMARY, index);
	if (IS_ERR(primary))
		return PTR_ERR(primary);

	if (vkmsdev->config->overlay) {
		for (n = 0; n < NUM_OVERLAY_PLANES; n++) {
			ret = vkms_add_overlay_plane(vkmsdev, index);
			if (ret)
				return ret;
		}
	}

	if (vkmsdev->config->cursor) {
		cursor = vkms_plane_init(vkmsdev, DRM_PLANE_TYPE_CURSOR, index);
		if (IS_ERR(cursor))
			return PTR_ERR(cursor);
	}

	list_for_each_entry(crtc_cfg, &vkmsdev->config->crtcs, list) {
		vkms_crtc = vkms_crtc_init(dev, &primary->base, &cursor->base);
		if (IS_ERR(vkms_crtc))
			return PTR_ERR(vkms_crtc);

		list_add_tail(&vkms_crtc->list, &vkmsdev->crtcs);

		if (crtc_cfg->writeback) {
			writeback = vkms_enable_writeback_connector(vkms_crtc);
			if (writeback)
				DRM_ERROR("Failed to init writeback connector\n");
		}
	}

	ret = drm_connector_init(dev, connector, &vkms_connector_funcs,
				 DRM_MODE_CONNECTOR_VIRTUAL);
	if (ret) {
		DRM_ERROR("Failed to init connector\n");
		return ret;
	}

	drm_connector_helper_add(connector, &vkms_conn_helper_funcs);

	encoder = vkms_encoder_init(vkmsdev, BIT(0));
	if (IS_ERR(encoder))
		return PTR_ERR(encoder);

	ret = drm_connector_attach_encoder(connector, encoder);
	if (ret) {
		DRM_ERROR("Failed to attach connector to encoder\n");
		goto err_attach;
	}

	drm_mode_config_reset(dev);

	return 0;

err_attach:
	drm_connector_cleanup(connector);

	return ret;
}
