/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONFIG_H_
#define _VKMS_CONFIG_H_

#include <linux/list.h>
#include <linux/types.h>

struct vkms_device;

struct vkms_config_crtc {
	struct list_head list;
	unsigned int index;
	bool writeback;
};

struct vkms_config {
	char *dev_name;
	bool cursor;
	bool overlay;
	struct list_head crtcs;
	/* only set when instantiated */
	struct vkms_device *dev;
};

/* VKMS Config */
struct vkms_config *vkms_config_create(char *dev_name);
struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay);
void vkms_config_destroy(struct vkms_config *config);

/* DebugFS */
void vkms_config_debugfs_init(struct vkms_device *vkms_device);

/* CRTCs */
struct vkms_config_crtc *vkms_config_add_crtc(struct vkms_config *config,
					      bool enable_writeback);
void vkms_config_destroy_crtc(struct vkms_config *config,
			      struct vkms_config_crtc *crtc_cfg);

#endif /* _VKMS_CONFIG_H_ */
