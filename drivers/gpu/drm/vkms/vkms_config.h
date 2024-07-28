/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONFIG_H_
#define _VKMS_CONFIG_H_

#include <linux/list.h>
#include <linux/types.h>

struct vkms_device;

struct vkms_config_crtc {
	struct list_head list;
	bool writeback;
};

struct vkms_config_encoder {
	struct list_head list;
	uint32_t possible_crtcs;
};

struct vkms_config {
	char *dev_name;
	bool cursor;
	bool overlay;
	struct list_head crtcs;
	struct list_head encoders;
	/* only set when instantiated */
	struct vkms_device *dev;
};

struct vkms_config *vkms_config_create(char *dev_name);
struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay);
void vkms_config_destroy(struct vkms_config *config);

void vkms_config_debugfs_init(struct vkms_device *vkms_device);

int vkms_config_add_crtc(struct vkms_config *config, bool enable_writeback);
int vkms_config_add_encoder(struct vkms_config *config, uint32_t possible_crtcs);

#endif /* _VKMS_CONFIG_H_ */
