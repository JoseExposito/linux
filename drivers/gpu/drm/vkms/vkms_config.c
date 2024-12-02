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

	return config;
}

struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	int n;

	config = vkms_config_create(DEFAULT_DEVICE_NAME);
	if (IS_ERR(config))
		return config;

	config->writeback = enable_writeback;

	plane_cfg = vkms_config_add_plane(config);
	if (IS_ERR(plane_cfg))
		goto err_alloc;
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_PRIMARY);

	if (enable_overlay) {
		for (n = 0; n < NUM_OVERLAY_PLANES; n++) {
			plane_cfg = vkms_config_add_plane(config);
			if (IS_ERR(plane_cfg))
				goto err_alloc;
			vkms_config_plane_set_type(plane_cfg,
						   DRM_PLANE_TYPE_OVERLAY);
		}
	}

	if (enable_cursor) {
		plane_cfg = vkms_config_add_plane(config);
		if (IS_ERR(plane_cfg))
			goto err_alloc;
		vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_CURSOR);
	}

	return config;

err_alloc:
	vkms_config_destroy(config);
	return ERR_PTR(-ENOMEM);
}

void vkms_config_destroy(struct vkms_config *config)
{
	struct vkms_config_plane *plane_cfg, *plane_tmp;

	list_for_each_entry_safe(plane_cfg, plane_tmp, &config->planes, link)
		vkms_config_destroy_plane(plane_cfg);

	kfree_const(config->dev_name);
	kfree(config);
}

struct vkms_config_plane **vkms_config_get_planes(const struct vkms_config *config,
						  size_t *out_length)
{
	struct vkms_config_plane **array;
	struct vkms_config_plane *plane_cfg;
	size_t length;
	int n = 0;

	length = list_count_nodes((struct list_head *)&config->planes);
	if (length == 0) {
		*out_length = length;
		return NULL;
	}

	array = kmalloc_array(length, sizeof(*array), GFP_KERNEL);
	if (!array)
		return ERR_PTR(-ENOMEM);

	list_for_each_entry(plane_cfg, &config->planes, link) {
		array[n] = plane_cfg;
		n++;
	}

	*out_length = length;
	return array;
}

static bool valid_plane_number(struct vkms_config *config)
{
	size_t n_planes;

	n_planes = list_count_nodes(&config->planes);
	if (n_planes <= 0 || n_planes >= 32) {
		pr_err("The number of planes must be between 1 and 31\n");
		return false;
	}

	return true;
}

static bool valid_plane_type(struct vkms_config *config)
{
	struct vkms_config_plane *plane_cfg;
	bool has_primary_plane = false;
	bool has_cursor_plane = false;

	list_for_each_entry(plane_cfg, &config->planes, link) {
		enum drm_plane_type type;

		type = vkms_config_plane_get_type(plane_cfg);

		if (type == DRM_PLANE_TYPE_PRIMARY) {
			if (has_primary_plane) {
				pr_err("Multiple primary planes\n");
				return false;
			}

			has_primary_plane = true;
		} else if (type == DRM_PLANE_TYPE_CURSOR) {
			if (has_cursor_plane) {
				pr_err("Multiple cursor planes\n");
				return false;
			}

			has_cursor_plane = true;
		}
	}

	if (!has_primary_plane) {
		pr_err("Primary plane not found\n");
		return false;
	}

	return true;
}

bool vkms_config_is_valid(struct vkms_config *config)
{
	if (!valid_plane_number(config))
		return false;

	if (!valid_plane_type(config))
		return false;

	return true;
}

static int vkms_config_show(struct seq_file *m, void *data)
{
	struct drm_debugfs_entry *entry = m->private;
	struct drm_device *dev = entry->dev;
	struct vkms_device *vkmsdev = drm_device_to_vkms_device(dev);
	const char *dev_name;
	struct vkms_config_plane *plane_cfg;

	dev_name = vkms_config_get_device_name((struct vkms_config *)vkmsdev->config);
	seq_printf(m, "dev_name=%s\n", dev_name);
	seq_printf(m, "writeback=%d\n", vkmsdev->config->writeback);

	list_for_each_entry(plane_cfg, &vkmsdev->config->planes, link) {
		seq_puts(m, "plane:\n");
		seq_printf(m, "\ttype=%d\n",
			   vkms_config_plane_get_type(plane_cfg));
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

	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_OVERLAY);

	list_add_tail(&plane_cfg->link, &config->planes);

	return plane_cfg;
}

void vkms_config_destroy_plane(struct vkms_config_plane *plane_cfg)
{
	list_del(&plane_cfg->link);
	kfree(plane_cfg);
}
