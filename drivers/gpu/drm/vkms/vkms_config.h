/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONFIG_H_
#define _VKMS_CONFIG_H_

#include <linux/types.h>

struct vkms_device;

struct vkms_config {
	char *dev_name;
	bool writeback;
	bool cursor;
	bool overlay;
	/* only set when instantiated */
	struct vkms_device *dev;
};

struct vkms_config *vkms_config_create(char *dev_name);
struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay);
void vkms_config_destroy(struct vkms_config *config);

void vkms_config_debugfs_init(struct vkms_device *vkms_device);

#endif /* _VKMS_CONFIG_H_ */
