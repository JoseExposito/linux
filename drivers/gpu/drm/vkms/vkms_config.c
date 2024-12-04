// SPDX-License-Identifier: GPL-2.0+

#include <linux/slab.h>

#include <drm/drm_print.h>
#include <drm/drm_debugfs.h>

#include "vkms_config.h"

struct vkms_config *vkms_config_create(const char *dev_name)
{
	struct vkms_config *config;

	config = kzalloc(sizeof(*config), GFP_KERNEL);
	if (!config)
		return ERR_PTR(-ENOMEM);

	config->dev_name = kstrdup_const(dev_name, GFP_KERNEL);
	if (!config->dev_name) {
		kfree(config);
		return ERR_PTR(-ENOMEM);
	}

	INIT_LIST_HEAD(&config->planes);
	INIT_LIST_HEAD(&config->crtcs);
	INIT_LIST_HEAD(&config->encoders);

	return config;
}

struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;
	int n;

	config = vkms_config_create(DEFAULT_DEVICE_NAME);
	if (IS_ERR(config))
		return config;

	plane_cfg = vkms_config_add_plane(config);
	if (IS_ERR(plane_cfg))
		goto err_alloc;
	plane_cfg->type = DRM_PLANE_TYPE_PRIMARY;

	crtc_cfg = vkms_config_add_crtc(config);
	if (IS_ERR(crtc_cfg))
		goto err_alloc;
	crtc_cfg->writeback = enable_writeback;

	if (vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg))
		goto err_alloc;

	if (enable_overlay) {
		for (n = 0; n < NUM_OVERLAY_PLANES; n++) {
			plane_cfg = vkms_config_add_plane(config);
			if (IS_ERR(plane_cfg))
				goto err_alloc;
			plane_cfg->type = DRM_PLANE_TYPE_OVERLAY;

			if (vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg))
				goto err_alloc;
		}
	}

	if (enable_cursor) {
		plane_cfg = vkms_config_add_plane(config);
		if (IS_ERR(plane_cfg))
			goto err_alloc;
		plane_cfg->type = DRM_PLANE_TYPE_CURSOR;

		if (vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg))
			goto err_alloc;
	}

	encoder_cfg = vkms_config_add_encoder(config);
	if (IS_ERR(encoder_cfg))
		goto err_alloc;

	if (vkms_config_encoder_attach_crtc(encoder_cfg, crtc_cfg))
		goto err_alloc;

	return config;

err_alloc:
	vkms_config_destroy(config);
	return ERR_PTR(-ENOMEM);
}

void vkms_config_destroy(struct vkms_config *config)
{
	struct vkms_config_plane *plane_cfg, *plane_tmp;
	struct vkms_config_crtc *crtc_cfg, *crtc_tmp;
	struct vkms_config_encoder *encoder_cfg, *encoder_tmp;

	list_for_each_entry_safe(plane_cfg, plane_tmp, &config->planes, link)
		vkms_config_destroy_plane(plane_cfg);

	list_for_each_entry_safe(crtc_cfg, crtc_tmp, &config->crtcs, link)
		vkms_config_destroy_crtc(config, crtc_cfg);

	list_for_each_entry_safe(encoder_cfg, encoder_tmp, &config->encoders, link)
		vkms_config_destroy_encoder(config, encoder_cfg);

	kfree(config->dev_name);
	kfree(config);
}

static bool valid_plane_number(struct vkms_config *config)
{
	if (list_count_nodes(&config->planes) == 0) {
		pr_err("The number of plane must at least 1");
		return false;
	}

	return true;
}

static bool valid_plane_type(struct vkms_config *config,
			     struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_plane *plane_cfg;
	bool has_primary_plane = false;
	bool has_cursor_plane = false;

	list_for_each_entry(plane_cfg, &config->planes, link) {
		struct vkms_config_crtc *possible_crtc;
		unsigned long idx = 0;

		xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
			if (possible_crtc != crtc_cfg)
				continue;

			if (plane_cfg->type == DRM_PLANE_TYPE_PRIMARY) {
				if (has_primary_plane) {
					pr_err("Multiple primary planes");
					return false;
				}

				has_primary_plane = true;
			} else if (plane_cfg->type == DRM_PLANE_TYPE_CURSOR) {
				if (has_cursor_plane) {
					pr_err("Multiple cursor planes");
					return false;
				}

				has_cursor_plane = true;
			}
		}
	}

	if (!has_primary_plane) {
		pr_err("Primary plane not found");
		return false;
	}

	return true;
}

static bool valid_plane_possible_crtcs(struct vkms_config *config)
{
	struct vkms_config_plane *plane_cfg;

	list_for_each_entry(plane_cfg, &config->planes, link) {
		if (xa_empty(&plane_cfg->possible_crtcs)) {
			pr_err("All planes must have at least one possible CRTC");
			return false;
		}
	}

	return true;
}

static bool valid_crtc_number(struct vkms_config *config)
{
	size_t n_crtcs;

	n_crtcs = list_count_nodes(&config->crtcs);
	if (n_crtcs <= 0 || n_crtcs >= 32) {
		pr_err("The number of CRTCs must be between 1 and 31");
		return false;
	}

	return true;
}

static bool valid_encoder_number(struct vkms_config *config)
{
	size_t n_encoders;

	n_encoders = list_count_nodes(&config->encoders);
	if (n_encoders <= 0 || n_encoders >= 32) {
		pr_err("The number of encoders must be between 1 and 31");
		return false;
	}

	return true;
}

static bool valid_encoder_possible_crtcs(struct vkms_config *config)
{
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;

	list_for_each_entry(encoder_cfg, &config->encoders, link) {
		if (xa_empty(&encoder_cfg->possible_crtcs)) {
			pr_err("All encoders must have at least one possible CRTC");
			return false;
		}
	}

	list_for_each_entry(crtc_cfg, &config->crtcs, link) {
		bool crtc_has_encoder = false;

		list_for_each_entry(encoder_cfg, &config->encoders, link) {
			struct vkms_config_crtc *possible_crtc;
			unsigned long idx = 0;

			xa_for_each(&encoder_cfg->possible_crtcs, idx, possible_crtc) {
				if (possible_crtc == crtc_cfg)
					crtc_has_encoder = true;
			}
		}

		if (!crtc_has_encoder) {
			pr_err("All CRTCs must have at least one possible encoder");
			return false;
		}
	}

	return true;
}

bool vkms_config_is_valid(struct vkms_config *config)
{
	struct vkms_config_crtc *crtc_cfg;

	if (!valid_plane_number(config))
		return false;

	if (!valid_crtc_number(config))
		return false;

	if (!valid_encoder_number(config))
		return false;

	if (!valid_plane_possible_crtcs(config))
		return false;

	list_for_each_entry(crtc_cfg, &config->crtcs, link) {
		if (!valid_plane_type(config, crtc_cfg))
			return false;
	}

	if (!valid_encoder_possible_crtcs(config))
		return false;

	return true;
}

static int vkms_config_show(struct seq_file *m, void *data)
{
	struct drm_debugfs_entry *entry = m->private;
	struct drm_device *dev = entry->dev;
	struct vkms_device *vkmsdev = drm_device_to_vkms_device(dev);
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;

	seq_printf(m, "dev_name=%s\n", vkmsdev->config->dev_name);

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, link) {
		seq_puts(m, "plane:\n");
		seq_printf(m, "\ttype=%d\n", plane_cfg->type);
	}

	list_for_each_entry(crtc_cfg, &vkmsdev->config->crtcs, link) {
		seq_puts(m, "crtc:\n");
		seq_printf(m, "\twriteback=%d\n", crtc_cfg->writeback);
	}

	list_for_each_entry(encoder_cfg, &vkmsdev->config->encoders, link) {
		seq_puts(m, "encoder\n");
	}

	return 0;
}

static const struct drm_debugfs_info vkms_config_debugfs_list[] = {
	{ "vkms_config", vkms_config_show, 0 },
};

void vkms_config_register_debugfs(struct vkms_device *vkms_device)
{
	drm_debugfs_add_files(&vkms_device->drm, vkms_config_debugfs_list,
			      ARRAY_SIZE(vkms_config_debugfs_list));
}

struct vkms_config_plane *vkms_config_add_plane(struct vkms_config *config)
{
	struct vkms_config_plane *plane_cfg;

	plane_cfg = kzalloc(sizeof(*plane_cfg), GFP_KERNEL);
	if (!plane_cfg)
		return ERR_PTR(-ENOMEM);

	plane_cfg->type = DRM_PLANE_TYPE_OVERLAY;
	xa_init_flags(&plane_cfg->possible_crtcs, XA_FLAGS_ALLOC);

	list_add_tail(&plane_cfg->link, &config->planes);

	return plane_cfg;
}

void vkms_config_destroy_plane(struct vkms_config_plane *plane_cfg)
{
	xa_destroy(&plane_cfg->possible_crtcs);
	list_del(&plane_cfg->link);
	kfree(plane_cfg);
}

int __must_check vkms_config_plane_attach_crtc(struct vkms_config_plane *plane_cfg,
					       struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_crtc *possible_crtc;
	unsigned long idx = 0;
	u32 crtc_idx = 0;

	xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
		if (possible_crtc == crtc_cfg)
			return -EINVAL;
	}

	return xa_alloc(&plane_cfg->possible_crtcs, &crtc_idx, crtc_cfg,
			xa_limit_32b, GFP_KERNEL);
}

void vkms_config_plane_detach_crtc(struct vkms_config_plane *plane_cfg,
				   struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_crtc *possible_crtc;
	unsigned long idx = 0;

	xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
		if (possible_crtc == crtc_cfg)
			xa_erase(&plane_cfg->possible_crtcs, idx);
	}
}

struct vkms_config_crtc *vkms_config_add_crtc(struct vkms_config *config)
{
	struct vkms_config_crtc *crtc_cfg;

	crtc_cfg = kzalloc(sizeof(*crtc_cfg), GFP_KERNEL);
	if (!crtc_cfg)
		return ERR_PTR(-ENOMEM);

	crtc_cfg->writeback = false;

	list_add_tail(&crtc_cfg->link, &config->crtcs);

	return crtc_cfg;
}

void vkms_config_destroy_crtc(struct vkms_config *config,
			      struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_encoder *encoder_cfg;

	list_for_each_entry(plane_cfg, &config->planes, link)
		vkms_config_plane_detach_crtc(plane_cfg, crtc_cfg);

	list_for_each_entry(encoder_cfg, &config->encoders, link)
		vkms_config_encoder_detach_crtc(encoder_cfg, crtc_cfg);

	list_del(&crtc_cfg->link);
	kfree(crtc_cfg);
}

static struct vkms_config_plane *vkms_config_crtc_get_plane(const struct vkms_config *config,
							    struct vkms_config_crtc *crtc_cfg,
							    enum drm_plane_type type)
{
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *possible_crtc;
	unsigned long idx;

	list_for_each_entry(plane_cfg, &config->planes, link) {
		xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
			if (possible_crtc == crtc_cfg && plane_cfg->type == type)
				return plane_cfg;
		}
	}

	return NULL;
}

struct vkms_config_plane *vkms_config_crtc_primary_plane(const struct vkms_config *config,
							 struct vkms_config_crtc *crtc_cfg)
{
	return vkms_config_crtc_get_plane(config, crtc_cfg, DRM_PLANE_TYPE_PRIMARY);
}

struct vkms_config_plane *vkms_config_crtc_cursor_plane(const struct vkms_config *config,
							struct vkms_config_crtc *crtc_cfg)
{
	return vkms_config_crtc_get_plane(config, crtc_cfg, DRM_PLANE_TYPE_CURSOR);
}

struct vkms_config_encoder *vkms_config_add_encoder(struct vkms_config *config)
{
	struct vkms_config_encoder *encoder_cfg;

	encoder_cfg = kzalloc(sizeof(*encoder_cfg), GFP_KERNEL);
	if (!encoder_cfg)
		return ERR_PTR(-ENOMEM);

	xa_init_flags(&encoder_cfg->possible_crtcs, XA_FLAGS_ALLOC);

	list_add_tail(&encoder_cfg->link, &config->encoders);

	return encoder_cfg;
}

void vkms_config_destroy_encoder(struct vkms_config *config,
				 struct vkms_config_encoder *encoder_cfg)
{
	xa_destroy(&encoder_cfg->possible_crtcs);
	list_del(&encoder_cfg->link);
	kfree(encoder_cfg);
}

int __must_check vkms_config_encoder_attach_crtc(struct vkms_config_encoder *encoder_cfg,
						 struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_crtc *possible_crtc;
	unsigned long idx = 0;
	u32 crtc_idx = 0;

	xa_for_each(&encoder_cfg->possible_crtcs, idx, possible_crtc) {
		if (possible_crtc == crtc_cfg)
			return -EINVAL;
	}

	return xa_alloc(&encoder_cfg->possible_crtcs, &crtc_idx, crtc_cfg,
			xa_limit_32b, GFP_KERNEL);
}

void vkms_config_encoder_detach_crtc(struct vkms_config_encoder *encoder_cfg,
				     struct vkms_config_crtc *crtc_cfg)
{
	struct vkms_config_crtc *possible_crtc;
	unsigned long idx = 0;

	xa_for_each(&encoder_cfg->possible_crtcs, idx, possible_crtc) {
		if (possible_crtc == crtc_cfg)
			xa_erase(&encoder_cfg->possible_crtcs, idx);
	}
}
