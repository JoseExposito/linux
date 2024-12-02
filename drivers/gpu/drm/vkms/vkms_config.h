/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONFIG_H_
#define _VKMS_CONFIG_H_

#include <linux/list.h>
#include <linux/types.h>

#include "vkms_drv.h"

/**
 * struct vkms_config - General configuration for VKMS driver
 *
 * @dev_name: Name of the device
 * @writeback: If true, a writeback buffer can be attached to the CRTC
 * @planes: List of planes configured for the device
 * @dev: Used to store the current VKMS device. Only set when the device is instantiated.
 */
struct vkms_config {
	const char *dev_name;
	bool writeback;
	struct list_head planes;
	struct vkms_device *dev;
};

/**
 * struct vkms_config_plane
 *
 * @link: Link to the others planes in vkms_config
 * @type: Type of the plane. The creator of configuration needs to ensures that
 *        at least one primary plane is present.
 * @plane: Internal usage. This pointer should never be considered as valid.
 *         It can be used to store a temporary reference to a VKMS plane during
 *         device creation. This pointer is not managed by the configuration and
 *         must be managed by other means.
 */
struct vkms_config_plane {
	struct list_head link;

	enum drm_plane_type type;

	/* Internal usage */
	struct vkms_plane *plane;
};

/**
 * vkms_config_create() - Create a new VKMS configuration
 * @dev_name: Name of the device
 *
 * Returns:
 * The new vkms_config or an error. Call vkms_config_destroy() to free the
 * returned configuration.
 */
struct vkms_config *vkms_config_create(const char *dev_name);

/**
 * vkms_config_default_create() - Create the configuration for the default device
 * @enable_cursor: Create or not a cursor plane
 * @enable_writeback: Create or not a writeback connector
 * @enable_overlay: Create or not overlay planes
 *
 * Returns:
 * The default vkms_config or an error. Call vkms_config_destroy() to free the
 * returned configuration.
 */
struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay);

/**
 * vkms_config_destroy() - Free a VKMS configuration
 * @config: vkms_config to free
 */
void vkms_config_destroy(struct vkms_config *config);

/**
 * vkms_config_get_device_name() - Return the name of the device
 * @config: Configuration to get the device name from
 */
static inline const char *
vkms_config_get_device_name(struct vkms_config *config)
{
	return config->dev_name;
}

/**
 * vkms_config_planes() - Return the array of planes of the device
 * @config: Configuration to get the planes from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_plane **vkms_config_get_planes(const struct vkms_config *config,
						  size_t *out_length);

/**
 * vkms_config_is_valid() - Validate a configuration
 * @config: Configuration to validate
 *
 * Returns:
 * Whether the configuration is valid or not.
 * For example, a configuration without primary planes is not valid.
 */
bool vkms_config_is_valid(struct vkms_config *config);

/**
 * vkms_config_register_debugfs() - Register a debugfs file to show the device's
 * configuration
 * @vkms_device: Device to register
 */
void vkms_config_register_debugfs(struct vkms_device *vkms_device);

/**
 * vkms_config_add_plane() - Add a new plane configuration
 * @config: Configuration to add the plane to
 *
 * Returns:
 * The new plane configuration or an error. Call vkms_config_destroy_plane() to
 * free the returned plane configuration.
 */
struct vkms_config_plane *vkms_config_add_plane(struct vkms_config *config);

/**
 * vkms_config_destroy_plane() - Remove and free a plane configuration
 * @plane_cfg: Plane configuration to destroy
 */
void vkms_config_destroy_plane(struct vkms_config_plane *plane_cfg);

/**
 * vkms_config_plane_type() - Return the plane type
 * @plane_cfg: Plane to get the type from
 */
static inline enum drm_plane_type
vkms_config_plane_get_type(struct vkms_config_plane *plane_cfg)
{
	return plane_cfg->type;
}

/**
 * vkms_config_plane_set_type() - Set the plane type
 * @plane_cfg: Plane to set the type to
 * @type: New plane type
 */
static inline void
vkms_config_plane_set_type(struct vkms_config_plane *plane_cfg,
			   enum drm_plane_type type)
{
	plane_cfg->type = type;
}

#endif /* _VKMS_CONFIG_H_ */
