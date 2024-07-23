/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CONFIGFS_H_
#define _VKMS_CONFIGFS_H_

#include <linux/configfs.h>
#include <linux/mutex.h>

int vkms_configfs_register(void);
void vkms_configfs_unregister(void);

#endif /* _VKMS_CONFIGFS_H_ */
