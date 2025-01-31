// SPDX-License-Identifier: GPL-2.0+

#include <drm/drm_atomic_helper.h>
#include <drm/drm_edid.h>
#include <drm/drm_managed.h>
#include <drm/drm_probe_helper.h>

#include "vkms_config.h"
#include "vkms_connector.h"

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

static struct drm_encoder *vkms_conn_best_encoder(struct drm_connector *connector)
{
	struct drm_encoder *encoder;

	drm_connector_for_each_possible_encoder(connector, encoder)
		return encoder;

	return NULL;
}

static const struct drm_connector_helper_funcs vkms_conn_helper_funcs = {
	.get_modes    = vkms_conn_get_modes,
	.best_encoder = vkms_conn_best_encoder,
};

struct vkms_connector *vkms_connector_init(struct vkms_device *vkmsdev)
{
	struct drm_device *dev = &vkmsdev->drm;
	struct vkms_connector *connector;
	int ret;

	connector = drmm_kzalloc(dev, sizeof(*connector), GFP_KERNEL);
	if (!connector)
		return ERR_PTR(-ENOMEM);

	ret = drmm_connector_init(dev, &connector->base, &vkms_connector_funcs,
				  DRM_MODE_CONNECTOR_VIRTUAL, NULL);
	if (ret)
		return ERR_PTR(ret);

	drm_connector_helper_add(&connector->base, &vkms_conn_helper_funcs);

	return connector;
}

static const struct drm_connector_funcs vkms_dynamic_connector_funcs = {
	.fill_modes = drm_helper_probe_single_connector_modes,
	.reset = drm_atomic_helper_connector_reset,
	.atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
	.atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
	.destroy = drm_connector_cleanup,
};

struct vkms_connector *vkms_connector_hot_add(struct vkms_device *vkmsdev,
					      struct vkms_config_connector *connector_cfg)
{
	struct vkms_config_encoder **possible_encoders = NULL;
	size_t n_possible_encoders;
	struct vkms_config_encoder *possible_encoder;
	struct vkms_connector *connector;
	int ret;
	int i;

	possible_encoders = vkms_config_connector_get_possible_encoders(connector_cfg,
									&n_possible_encoders);
	if (IS_ERR(possible_encoders)) {
		connector = ERR_CAST(possible_encoders);
		goto out_free;
	}

	// connector = vkms_connector_init(vkmsdev);
	// if (IS_ERR(connector))
	// 	goto out_free;
	//
	// Instead of drmm_connector_init(), use drm_connector_dynamic_init().
	// Unfortunately, this function doesn't have a drmm version yet.
	// 
	// {
	connector = kzalloc(sizeof(*connector), GFP_KERNEL);
	if (!connector) {
		connector = ERR_PTR(-ENOMEM);
		goto out_free;
	}
		
	ret = drm_connector_dynamic_init(&vkmsdev->drm, &connector->base, &vkms_dynamic_connector_funcs, DRM_MODE_CONNECTOR_VIRTUAL, NULL);
	if (ret) {
		// TODO: Free memory
		connector = ERR_PTR(ret);
		goto out_free;
	}

	drm_connector_helper_add(&connector->base, &vkms_conn_helper_funcs);
	// }

	for (i = 0; i < n_possible_encoders; i++) {
		possible_encoder = possible_encoders[i];
		ret = drm_connector_attach_encoder(&connector->base,
						   possible_encoder->encoder);
		if (ret) {
			connector = ERR_PTR(ret);
			goto out_free;
		}
	}

	drm_mode_config_reset(&vkmsdev->drm);

	// drm_connector_register(&connector->base);
	// {
	ret = drm_connector_dynamic_register(&connector->base);
	if (ret) {
		// TODO: Free memory
		connector = ERR_PTR(ret);
		goto out_free;
	}
	// }

out_free:
	kfree(possible_encoders);
	return connector;
}

void vkms_connector_hot_remove(struct vkms_device *vkmsdev,
			       struct vkms_connector *connector)
{
	drm_connector_unregister(&connector->base);
	drm_mode_config_reset(&vkmsdev->drm);
	drm_connector_put(&connector->base);
}

int vkms_connector_hot_attach_encoder(struct vkms_device *vkmsdev,
				      struct vkms_connector *connector,
				      struct drm_encoder *encoder)
{
	int ret;

	ret = drm_connector_attach_encoder(&connector->base, encoder);
	if (ret)
		return ret;

	drm_mode_config_reset(&vkmsdev->drm);

	return ret;
}
