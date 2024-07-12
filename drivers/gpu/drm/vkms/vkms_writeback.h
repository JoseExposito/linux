/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_WRITEBACK_H_
#define _VKMS_WRITEBACK_H_

#include "vkms_drv.h"
#include "vkms_formats.h"
#include "vkms_plane.h"

struct vkms_writeback_job {
	struct iosys_map data[DRM_FORMAT_MAX_PLANES];
	struct vkms_frame_info wb_frame_info;
	void (*pixel_write)(u8 *dst_pixels, struct pixel_argb_u16 *in_pixel);
};

int vkms_enable_writeback_connector(struct vkms_device *vkmsdev);

#endif /* _VKMS_WRITEBACK_H_ */
