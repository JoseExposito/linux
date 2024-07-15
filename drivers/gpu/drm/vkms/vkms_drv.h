/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_DRV_H_
#define _VKMS_DRV_H_

#include <linux/hrtimer.h>

#include <drm/drm.h>
#include <drm/drm_framebuffer.h>
#include <drm/drm_gem.h>
#include <drm/drm_gem_atomic_helper.h>
#include <drm/drm_encoder.h>
#include <drm/drm_writeback.h>

#include "vkms_formats.h"

#define XRES_MIN    10
#define YRES_MIN    10

#define XRES_DEF  1024
#define YRES_DEF   768

#define XRES_MAX  8192
#define YRES_MAX  8192

#define NUM_OVERLAY_PLANES 8

#define VKMS_LUT_SIZE 256

/**
 * struct vkms_output - Internal representation of all output components in vkms
 *
 * @crtc: Base crtc in drm
 * @encoder: DRM encoder used for this output
 * @connector: DRM connector used for this output
 * @wb_connector: DRM writeback connector used for this output
 * @vblank_hrtimer: Timer used to simulate vblank events
 * @period_ns: vblank period, in nanoseconds
 * @composer_workq: Ordered workqueue for composer_work
 * @lock: Lock used to project concurrent access to the composer
 * @composer_enabled: Protected by @lock.
 * @composer_state: CRTC state
 * @composer_lock: Lock used internally to protect @composer_state members
 */
struct vkms_output {
	struct drm_crtc crtc;
	struct drm_encoder encoder;
	struct drm_connector connector;
	struct drm_writeback_connector wb_connector;
	struct hrtimer vblank_hrtimer;
	ktime_t period_ns;
	struct workqueue_struct *composer_workq;
	spinlock_t lock;

	bool composer_enabled;
	struct vkms_crtc_state *composer_state;

	spinlock_t composer_lock;
};

struct vkms_device;

/**
 * struct vkms_config - General configuration for VKMS driver
 *
 * @writeback: If true, a writeback buffer can be attached to the CRTC
 * @cursor: If true, a cursor plane is created in the VKMS device
 * @overlay: If true, NUM_OVERLAY_PLANES will be created for the VKMS device
 * @dev: Used to store the current vkms device. Only set when the device is instantiated
 */
struct vkms_config {
	bool writeback;
	bool cursor;
	bool overlay;
	struct vkms_device *dev;
};

/**
 * struct vkms_device - Description of a vkms device
 *
 * @drm: Base device in drm
 * @platform: Associated platform device
 * @output: Configuration and sub-components of the vkms device
 * @config: Configuration used in this vkms device
 */
struct vkms_device {
	struct drm_device drm;
	struct platform_device *platform;
	struct vkms_output output;
	const struct vkms_config *config;
};

/*
 * The following helpers are used to convert a member of a struct into its parent.
 */

#define drm_crtc_to_vkms_output(target) \
	container_of(target, struct vkms_output, crtc)

#define drm_device_to_vkms_device(target) \
	container_of(target, struct vkms_device, drm)

/**
 * vkms_output_init - Initialize all sub-components needed for a vkms device.
 *
 * @vkmsdev: vkms device to initialize
 * @possible_crtc: CRTC which can be attached to the planes. The caller must
 * ensure that possible_crtc is less than or equal to 31.
 */
int vkms_output_init(struct vkms_device *vkmsdev, u32 possible_crtc);

/* CRC Support */
const char *const *vkms_get_crc_sources(struct drm_crtc *crtc,
					size_t *count);
int vkms_set_crc_source(struct drm_crtc *crtc, const char *src_name);
int vkms_verify_crc_source(struct drm_crtc *crtc, const char *source_name,
			   size_t *values_cnt);

/* Composer Support */
void vkms_composer_worker(struct work_struct *work);
void vkms_set_composer(struct vkms_output *out, bool enabled);

#endif /* _VKMS_DRV_H_ */
