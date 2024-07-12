/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_PLANE_H_
#define _VKMS_PLANE_H_

#include "vkms_drv.h"

struct vkms_plane {
	struct drm_plane base;
};

/**
 * struct vkms_plane_state - Driver specific plane state
 * @base: base plane state
 * @frame_info: data required for composing computation
 */
struct vkms_plane_state {
	struct drm_shadow_plane_state base;
	struct vkms_frame_info *frame_info;
	void (*pixel_read)(u8 *src_buffer, struct pixel_argb_u16 *out_pixel);
};

struct vkms_frame_info {
	struct drm_framebuffer *fb;
	struct drm_rect src, dst;
	struct drm_rect rotated;
	struct iosys_map map[DRM_FORMAT_MAX_PLANES];
	unsigned int rotation;
	unsigned int offset;
	unsigned int pitch;
	unsigned int cpp;
};

#define drm_plane_state_to_vkms_plane_state(target)\
	container_of(target, struct vkms_plane_state, base.base)

/**
 * vkms_plane_init - Initialize a plane
 *
 * @vkmsdev: vkms device containing the plane
 * @type: type of plane to initialize
 * @possible_crtc: Crtc which can be attached to the plane. The caller must
 * ensure that possible_crtc is less that or equal to 31.
 */
struct vkms_plane *vkms_plane_init(struct vkms_device *vkmsdev,
				   enum drm_plane_type type, u32 possible_crtc);

#endif /* _VKMS_PLANE_H_ */
