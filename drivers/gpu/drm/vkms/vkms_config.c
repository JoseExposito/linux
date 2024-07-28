// SPDX-License-Identifier: GPL-2.0+

#include <linux/slab.h>

#include <drm/drm_print.h>
#include <drm/drm_debugfs.h>

#include "vkms_config.h"
#include "vkms_drv.h"

struct vkms_config *vkms_config_create(char *dev_name)
{
	struct vkms_config *config;

	config = kzalloc(sizeof(*config), GFP_KERNEL);
	if (!config)
		return ERR_PTR(-ENOMEM);

	config->dev_name = dev_name;
	config->crtcs = (struct list_head)LIST_HEAD_INIT(config->crtcs);

	return config;
}

struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay)
{
	struct vkms_config *config;
	struct vkms_config_crtc *crtc_cfg;

	config = vkms_config_create(DEFAULT_DEVICE_NAME);
	if (IS_ERR(config))
		return config;

	config->cursor = enable_cursor;
	config->overlay = enable_overlay;

	crtc_cfg = vkms_config_add_crtc(config, enable_writeback);
	if (IS_ERR(crtc_cfg))
		return ERR_CAST(crtc_cfg);

	return config;
}

void vkms_config_destroy(struct vkms_config *config)
{
	struct vkms_config_crtc *crtc_cfg, *crtc_tmp;

	list_for_each_entry_safe(crtc_cfg, crtc_tmp, &config->crtcs, list)
		vkms_config_destroy_crtc(config, crtc_cfg);

	kfree(config);
}

static int vkms_config_show(struct seq_file *m, void *data)
{
	struct drm_debugfs_entry *entry = m->private;
	struct drm_device *dev = entry->dev;
	struct vkms_device *vkmsdev = drm_device_to_vkms_device(dev);
	struct vkms_config_crtc *crtc_cfg;
	int n;

	seq_printf(m, "dev_name=%s\n", vkmsdev->config->dev_name);
	seq_printf(m, "cursor=%d\n", vkmsdev->config->cursor);
	seq_printf(m, "overlay=%d\n", vkmsdev->config->overlay);

	n = 0;
	list_for_each_entry(crtc_cfg, &vkmsdev->config->crtcs, list) {
		seq_printf(m, "crtc(%d).writeback=%d\n", n,
			   crtc_cfg->writeback);
		n++;
	}

	return 0;
}

static const struct drm_debugfs_info vkms_config_debugfs_list[] = {
	{ "vkms_config", vkms_config_show, 0 },
};

void vkms_config_debugfs_init(struct vkms_device *vkms_device)
{
	drm_debugfs_add_files(&vkms_device->drm, vkms_config_debugfs_list,
			      ARRAY_SIZE(vkms_config_debugfs_list));
}

struct vkms_config_crtc *vkms_config_add_crtc(struct vkms_config *config,
					      bool enable_writeback)
{
	struct vkms_config_crtc *crtc_cfg;

	crtc_cfg = kzalloc(sizeof(*crtc_cfg), GFP_KERNEL);
	if (!crtc_cfg)
		return ERR_PTR(-ENOMEM);

	crtc_cfg->writeback = enable_writeback;

	crtc_cfg->index = 0;
	if (!list_empty(&config->crtcs)) {
		struct vkms_config_crtc *last;

		last = list_last_entry(&config->crtcs, struct vkms_config_crtc,
				       list);
		crtc_cfg->index = last->index + 1;
	}

	list_add_tail(&crtc_cfg->list, &config->crtcs);

	return crtc_cfg;
}

void vkms_config_destroy_crtc(struct vkms_config *config,
			      struct vkms_config_crtc *crtc_cfg)
{
	list_del(&crtc_cfg->list);
	kfree(crtc_cfg);
}
