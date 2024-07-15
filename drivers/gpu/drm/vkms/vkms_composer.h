/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_COMPOSER_H_
#define _VKMS_COMPOSER_H_

#include "vkms_drv.h"
#include "vkms_crtc.h"

void vkms_composer_worker(struct work_struct *work);
void vkms_set_composer(struct vkms_output *out, bool enabled);

/* CRC Support */
const char *const *vkms_get_crc_sources(struct drm_crtc *crtc,
					size_t *count);
int vkms_set_crc_source(struct drm_crtc *crtc, const char *src_name);
int vkms_verify_crc_source(struct drm_crtc *crtc, const char *source_name,
			   size_t *values_cnt);

#endif /* _VKMS_COMPOSER_H_ */
