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
 * @planes: List of planes configured for the device
 * @crtcs: List of CRTCs configured for the device
 * @encoders: List of encoders configured for the device
 * @connectors: List of connectors configured for the device
 * @dev: Used to store the current VKMS device. Only set when the device is instantiated.
 */
struct vkms_config {
	const char *dev_name;
	struct list_head planes;
	struct list_head crtcs;
	struct list_head encoders;
	struct list_head connectors;
	struct vkms_device *dev;
};

/**
 * struct vkms_config_plane
 *
 * @link: Link to the others planes in vkms_config
 * @type: Type of the plane. The creator of configuration needs to ensures that
 *        at least one primary plane is present.
 * @possible_crtcs: Array of CRTCs that can be used with this plane
 * @plane: Internal usage. This pointer should never be considered as valid.
 *         It can be used to store a temporary reference to a VKMS plane during
 *         device creation. This pointer is not managed by the configuration and
 *         must be managed by other means.
 */
struct vkms_config_plane {
	struct list_head link;

	enum drm_plane_type type;
	struct xarray possible_crtcs;

	/* Internal usage */
	struct vkms_plane *plane;
};

/**
 * struct vkms_config_crtc
 *
 * @link: Link to the others CRTCs in vkms_config
 * @writeback: If true, a writeback buffer can be attached to the CRTC
 * @crtc: Internal usage. This pointer should never be considered as valid.
 *        It can be used to store a temporary reference to a VKMS CRTC during
 *        device creation. This pointer is not managed by the configuration and
 *        must be managed by other means.
 */
struct vkms_config_crtc {
	struct list_head link;

	bool writeback;

	/* Internal usage */
	struct vkms_output *crtc;
};

/**
 * struct vkms_config_encoder
 *
 * @link: Link to the others encoders in vkms_config
 * @possible_crtcs: Array of CRTCs that can be used with this encoder
 * @encoder: Internal usage. This pointer should never be considered as valid.
 *           It can be used to store a temporary reference to a VKMS encoder
 *           during device creation. This pointer is not managed by the
 *           configuration and must be managed by other means.
 */
struct vkms_config_encoder {
	struct list_head link;

	struct xarray possible_crtcs;

	/* Internal usage */
	struct drm_encoder *encoder;
};

/**
 * struct vkms_config_connector
 *
 * @link: Link to the others connector in vkms_config
 * @enabled: Connector are a different from planes, CRTCs and encoders because
 *           they can be added and removed once the device is created.
 *           This flag represents if they are part of the device or not.
 * @possible_encoders: Array of encoders that can be used with this connector
 * @connector: Internal usage. This pointer should never be considered as valid.
 *             It can be used to store a temporary reference to a VKMS connector
 *             during device creation. This pointer is not managed by the
 *             configuration and must be managed by other means.
 */
struct vkms_config_connector {
	struct list_head link;

	bool enabled;
	struct xarray possible_encoders;

	/* Internal usage */
	struct vkms_connector *connector;
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
 * vkms_config_get_num_crtcs() - Return the number of CRTCs in the configuration
 * @config: Configuration to get the number of CRTCs from
 */
static inline size_t vkms_config_get_num_crtcs(struct vkms_config *config)
{
	return list_count_nodes(&config->crtcs);
}

/**
 * vkms_config_get_crtcs() - Return the array of CRTCs of the device
 * @config: Configuration to get the CRTCs from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_crtc **vkms_config_get_crtcs(const struct vkms_config *config,
						size_t *out_length);

/**
 * vkms_config_get_encoders() - Return the array of encoders of the device
 * @config: Configuration to get the encoders from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_encoder **vkms_config_get_encoders(const struct vkms_config *config,
						      size_t *out_length);

/**
 * vkms_config_get_connectors() - Return the array of connectors of the device
 * @config: Configuration to get the connectors from
 * @out_length: Length of the returned array
 *
 * Note that only enabled connectors are returned.
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_connector **vkms_config_get_connectors(const struct vkms_config *config,
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

/**
 * vkms_config_plane_attach_crtc - Attach a plane to a CRTC
 * @plane_cfg: Plane to attach
 * @crtc_cfg: CRTC to attach @plane_cfg to
 */
int __must_check vkms_config_plane_attach_crtc(struct vkms_config_plane *plane_cfg,
					       struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_plane_attach_crtc - Detach a plane from a CRTC
 * @plane_cfg: Plane to detach
 * @crtc_cfg: CRTC to detach @plane_cfg from
 */
void vkms_config_plane_detach_crtc(struct vkms_config_plane *plane_cfg,
				   struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_plane_get_possible_crtcs() - Return the array of possible CRTCs
 * @plane_cfg: Plane to get the possible CRTCs from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_crtc **vkms_config_plane_get_possible_crtcs(struct vkms_config_plane *plane_cfg,
							       size_t *out_length);

/**
 * vkms_config_add_crtc() - Add a new CRTC configuration
 * @config: Configuration to add the CRTC to
 *
 * Returns:
 * The new CRTC configuration or an error. Call vkms_config_destroy_crtc() to
 * free the returned CRTC configuration.
 */
struct vkms_config_crtc *vkms_config_add_crtc(struct vkms_config *config);

/**
 * vkms_config_destroy_crtc() - Remove and free a CRTC configuration
 * @config: Configuration to remove the CRTC from
 * @crtc_cfg: CRTC configuration to destroy
 */
void vkms_config_destroy_crtc(struct vkms_config *config,
			      struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_crtc_get_writeback() - If a writeback connector will be created
 * @crtc_cfg: CRTC with or without a writeback connector
 */
static inline bool
vkms_config_crtc_get_writeback(struct vkms_config_crtc *crtc_cfg)
{
	return crtc_cfg->writeback;
}

/**
 * vkms_config_crtc_set_writeback() - If a writeback connector will be created
 * @crtc_cfg: Target CRTC
 * @writeback: Enable or disable the writeback connector
 */
static inline void
vkms_config_crtc_set_writeback(struct vkms_config_crtc *crtc_cfg,
			       bool writeback)
{
	crtc_cfg->writeback = writeback;
}

/**
 * vkms_config_crtc_primary_plane() - Return the primary plane for a CRTC
 * @config: Configuration containing the CRTC
 * @crtc_config: Target CRTC
 *
 * Returns:
 * The primary plane or NULL if none is assigned yet.
 */
struct vkms_config_plane *vkms_config_crtc_primary_plane(const struct vkms_config *config,
							 struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_crtc_cursor_plane() - Return the cursor plane for a CRTC
 * @config: Configuration containing the CRTC
 * @crtc_config: Target CRTC
 *
 * Returns:
 * The cursor plane or NULL if none is assigned yet.
 */
struct vkms_config_plane *vkms_config_crtc_cursor_plane(const struct vkms_config *config,
							struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_add_encoder() - Add a new encoder configuration
 * @config: Configuration to add the encoder to
 *
 * Returns:
 * The new encoder configuration or an error. Call vkms_config_destroy_encoder()
 * to free the returned encoder configuration.
 */
struct vkms_config_encoder *vkms_config_add_encoder(struct vkms_config *config);

/**
 * vkms_config_destroy_encoder() - Remove and free a encoder configuration
 * @config: Configuration to remove the encoder from
 * @encoder_cfg: Encoder configuration to destroy
 */
void vkms_config_destroy_encoder(struct vkms_config *config,
				 struct vkms_config_encoder *encoder_cfg);

/**
 * vkms_config_encoder_attach_crtc - Attach a encoder to a CRTC
 * @encoder_cfg: Encoder to attach
 * @crtc_cfg: CRTC to attach @encoder_cfg to
 */
int __must_check vkms_config_encoder_attach_crtc(struct vkms_config_encoder *encoder_cfg,
						 struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_encoder_detach_crtc - Detach a encoder from a CRTC
 * @encoder_cfg: Encoder to detach
 * @crtc_cfg: CRTC to detach @encoder_cfg from
 */
void vkms_config_encoder_detach_crtc(struct vkms_config_encoder *encoder_cfg,
				     struct vkms_config_crtc *crtc_cfg);

/**
 * vkms_config_encoder_get_possible_crtcs() - Return the array of possible CRTCs
 * @encoder_cfg: Encoder to get the possible CRTCs from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_crtc **
vkms_config_encoder_get_possible_crtcs(struct vkms_config_encoder *encoder_cfg,
				       size_t *out_length);

/**
 * vkms_config_add_connector() - Add a new connector configuration
 * @config: Configuration to add the connector to
 *
 * Returns:
 * The new connector configuration or an error. Call
 * vkms_config_destroy_connector() to free the returned connector configuration.
 */
struct vkms_config_connector *vkms_config_add_connector(struct vkms_config *config);

/**
 * vkms_config_destroy_connector() - Remove and free a connector configuration
 * @connector_cfg: Connector configuration to destroy
 */
void vkms_config_destroy_connector(struct vkms_config_connector *connector_cfg);

/**
 * vkms_config_connector_is_enabled() - If the connector is part of the device
 * @connector_cfg: The connector
 */
static inline bool
vkms_config_connector_is_enabled(struct vkms_config_connector *connector_cfg)
{
	return connector_cfg->enabled;
}

/**
 * vkms_config_connector_set_enabled() - If the connector is part of the device
 * @crtc_cfg: Target connector
 * @enabled: Add or remove the connector
 */
static inline void
vkms_config_connector_set_enabled(struct vkms_config_connector *connector_cfg,
				  bool enabled)
{
	connector_cfg->enabled = enabled;
}

/**
 * vkms_config_connector_attach_encoder - Attach a connector to an encoder
 * @connector_cfg: Connector to attach
 * @encoder_cfg: Encoder to attach @connector_cfg to
 */
int __must_check vkms_config_connector_attach_encoder(struct vkms_config_connector *connector_cfg,
						      struct vkms_config_encoder *encoder_cfg);

/**
 * vkms_config_connector_detach_encoder - Detach a connector from an encoder
 * @connector_cfg: Connector to detach
 * @encoder_cfg: Encoder to detach @connector_cfg from
 */
void vkms_config_connector_detach_encoder(struct vkms_config_connector *connector_cfg,
					  struct vkms_config_encoder *encoder_cfg);

/**
 * vkms_config_connector_get_possible_encoders() - Return the array of possible
 * encoders
 * @connector_cfg: Connector to get the possible encoders from
 * @out_length: Length of the returned array
 *
 * Returns:
 * A list of pointers to the configurations. On success, the caller is
 * responsible to free the returned array, but not its contents. On error,
 * it returns an error and @out_length is invalid.
 */
struct vkms_config_encoder **
vkms_config_connector_get_possible_encoders(struct vkms_config_connector *connector_cfg,
					    size_t *out_length);

#endif /* _VKMS_CONFIG_H_ */
