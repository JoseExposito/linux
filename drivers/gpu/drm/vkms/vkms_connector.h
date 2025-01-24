/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONNECTOR_H_
#define _VKMS_CONNECTOR_H_

#include "vkms_drv.h"

struct vkms_config_connector;

/**
 * struct vkms_connector - VKMS custom type wrapping around the DRM connector
 *
 * @drm: Base DRM connector
 */
struct vkms_connector {
	struct drm_connector base;
};

/**
 * vkms_connector_init() - Initialize a connector
 * @vkmsdev: VKMS device containing the connector
 *
 * Returns:
 * The connector or an error on failure.
 */
struct vkms_connector *vkms_connector_init(struct vkms_device *vkmsdev);

/**
 * vkms_connector_hot_add() - Create a connector after the device is created
 * @vkmsdev: Device to hot-add the connector to
 * @connector_cfg: Connector's configuration
 *
 * Returns:
 * The connector or an error on failure.
 */
struct vkms_connector *vkms_connector_hot_add(struct vkms_device *vkmsdev,
					      struct vkms_config_connector *connector_cfg);

#endif /* _VKMS_CONNECTOR_H_ */
