// SPDX-License-Identifier: GPL-2.0+

#include "vkms_config.h"
#include "vkms_connector.h"
#include "vkms_drv.h"
#include <drm/drm_managed.h>

int vkms_output_init(struct vkms_device *vkmsdev)
{
	struct drm_device *dev = &vkmsdev->drm;
	struct vkms_connector *connector;
	struct drm_encoder *encoder;
	struct vkms_output *output;
	struct vkms_plane *primary = NULL, *cursor = NULL;
	struct vkms_config_plane **plane_cfgs = NULL;
	size_t n_planes;
	int ret = 0;
	int writeback;
	unsigned int n;

	plane_cfgs = vkms_config_get_planes(vkmsdev->config, &n_planes);
	if (IS_ERR(plane_cfgs))
		return PTR_ERR(plane_cfgs);

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

		if (type == DRM_PLANE_TYPE_PRIMARY)
			primary = plane_cfg->plane;
		else if (type == DRM_PLANE_TYPE_CURSOR)
			cursor = plane_cfg->plane;
	}

	output = vkms_crtc_init(dev, &primary->base,
				cursor ? &cursor->base : NULL);
	if (IS_ERR(output)) {
		DRM_ERROR("Failed to allocate CRTC\n");
		ret = PTR_ERR(output);
		goto err_free;
	}

	connector = vkms_connector_init(vkmsdev);
	if (IS_ERR(connector)) {
		DRM_ERROR("Failed to init connector\n");
		ret = PTR_ERR(connector);
		goto err_free;
	}

	encoder = drmm_kzalloc(dev, sizeof(*encoder), GFP_KERNEL);
	if (!encoder) {
		DRM_ERROR("Failed to allocate encoder\n");
		ret = -ENOMEM;
		goto err_free;
	}
	ret = drmm_encoder_init(dev, encoder, NULL,
				DRM_MODE_ENCODER_VIRTUAL, NULL);
	if (ret) {
		DRM_ERROR("Failed to init encoder\n");
		goto err_free;
	}
	encoder->possible_crtcs = drm_crtc_mask(&output->crtc);

	/* Attach the encoder and the connector */
	ret = drm_connector_attach_encoder(&connector->base, encoder);
	if (ret) {
		DRM_ERROR("Failed to attach connector to encoder\n");
		goto err_free;
	}

	/* Initialize the writeback component */
	if (vkmsdev->config->writeback) {
		writeback = vkms_enable_writeback_connector(vkmsdev, output);
		if (writeback)
			DRM_ERROR("Failed to init writeback connector\n");
	}

	drm_mode_config_reset(dev);

err_free:
	kfree(plane_cfgs);

	return ret;
}
