/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_CRTC_H_
#define _VKMS_CRTC_H_

#include "vkms_plane.h"
#include "vkms_writeback.h"

/**
 * struct vkms_crtc_state - Driver specific CRTC state
 * @base: base CRTC state
 * @composer_work: work struct to compose and add CRC entries
 * @num_active_planes: Number of active planes
 * @active_planes: List containing all the active planes (counted by
 *  @num_active_planes). They should be stored in z-order
 * @active_writeback: Current active writeback job
 * @gamma_lut: Look up table for gamma used in this CRTC
 * @crc_pending: Protected by @vkms_output.composer_lock
 * @wb_pending: Protected by @vkms_output.composer_lock
 * @frame_start: Protected by @vkms_output.composer_lock
 * @frame_end: Protected by @vkms_output.composer_lock
 */
struct vkms_crtc_state {
	struct drm_crtc_state base;
	struct work_struct composer_work;

	int num_active_planes;
	struct vkms_plane_state **active_planes;
	struct vkms_writeback_job *active_writeback;
	struct vkms_color_lut gamma_lut;

	/* below four are protected by vkms_output.composer_lock */
	bool crc_pending;
	bool wb_pending;
	u64 frame_start;
	u64 frame_end;
};

#define to_vkms_crtc_state(target)\
	container_of(target, struct vkms_crtc_state, base)

/**
 * vkms_crtc_init - Initialize a CRTC for vkms
 * @dev: drm_device associated with the vkms buffer
 * @crtc: uninitialized crtc device
 * @primary: primary plane to attach to the crtc
 * @cursor: cursor plane to attach to the crtc
 */
int vkms_crtc_init(struct drm_device *dev, struct drm_crtc *crtc,
		   struct drm_plane *primary, struct drm_plane *cursor);

#endif /* _VKMS_CRTC_H_ */
