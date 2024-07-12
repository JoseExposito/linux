/* SPDX-License-Identifier: GPL-2.0+ */

#ifndef _VKMS_FORMATS_H_
#define _VKMS_FORMATS_H_

struct vkms_plane_state;
struct vkms_writeback_job;

struct pixel_argb_u16 {
	u16 a, r, g, b;
};

struct line_buffer {
	size_t n_pixels;
	struct pixel_argb_u16 *pixels;
};

struct vkms_color_lut {
	struct drm_color_lut *base;
	size_t lut_length;
	s64 channel_value2index_ratio;
};

void *get_pixel_conversion_function(u32 format);

void *get_pixel_write_function(u32 format);

void vkms_compose_row(struct line_buffer *stage_buffer, struct vkms_plane_state *plane, int y);
void vkms_writeback_row(struct vkms_writeback_job *wb, const struct line_buffer *src_buffer, int y);

#endif /* _VKMS_FORMATS_H_ */
