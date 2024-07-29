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
	config->encoders = (struct list_head)LIST_HEAD_INIT(config->encoders);
	config->connectors = (struct list_head)LIST_HEAD_INIT(config->connectors);

	return config;
}

struct vkms_config *vkms_config_default_create(bool enable_cursor,
					       bool enable_writeback,
					       bool enable_overlay)
{
	struct vkms_config *config;
	int ret;

	config = vkms_config_create(DEFAULT_DEVICE_NAME);
	if (IS_ERR(config))
		return config;

	config->cursor = enable_cursor;
	config->overlay = enable_overlay;

	ret = vkms_config_add_crtc(config, enable_writeback);
	if (ret)
		return ERR_PTR(ret);

	ret = vkms_config_add_encoder(config, BIT(0));
	if (ret)
		return ERR_PTR(ret);

	ret = vkms_config_add_connector(config, BIT(0));
	if (ret)
		return ERR_PTR(ret);

	return config;
}

void vkms_config_destroy(struct vkms_config *config)
{
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;
	struct vkms_config_connector *connector_cfg;

	list_for_each_entry(crtc_cfg, &config->crtcs, list)
		kfree(crtc_cfg);

	list_for_each_entry(encoder_cfg, &config->encoders, list)
		kfree(encoder_cfg);

	list_for_each_entry(connector_cfg, &config->connectors, list)
		kfree(connector_cfg);

	kfree(config);
}

static int vkms_config_show(struct seq_file *m, void *data)
{
	struct drm_debugfs_entry *entry = m->private;
	struct drm_device *dev = entry->dev;
	struct vkms_device *vkmsdev = drm_device_to_vkms_device(dev);
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;
	struct vkms_config_connector *connector_cfg;
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

	n = 0;
	list_for_each_entry(encoder_cfg, &vkmsdev->config->encoders, list) {
		seq_printf(m, "encoder(%d).possible_crtcs=%d\n", n,
			   encoder_cfg->possible_crtcs);
		n++;
	}

	n = 0;
	list_for_each_entry(connector_cfg, &vkmsdev->config->connectors, list) {
		seq_printf(m, "connector(%d).possible_encoders=%d\n", n,
			   connector_cfg->possible_encoders);
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

int vkms_config_add_crtc(struct vkms_config *config, bool enable_writeback)
{
	struct vkms_config_crtc *crtc_cfg;

	crtc_cfg = kzalloc(sizeof(*crtc_cfg), GFP_KERNEL);
	if (!crtc_cfg)
		return -ENOMEM;

	crtc_cfg->writeback = enable_writeback;
	list_add_tail(&crtc_cfg->list, &config->crtcs);

	return 0;
}

int vkms_config_add_encoder(struct vkms_config *config, uint32_t possible_crtcs)
{
	struct vkms_config_encoder *encoder_cfg;

	encoder_cfg = kzalloc(sizeof(*encoder_cfg), GFP_KERNEL);
	if (!encoder_cfg)
		return -ENOMEM;

	encoder_cfg->possible_crtcs = possible_crtcs;
	list_add_tail(&encoder_cfg->list, &config->encoders);

	return 0;
}

int vkms_config_add_connector(struct vkms_config *config,
			      uint32_t possible_encoders)
{
	struct vkms_config_connector *connector_cfg;

	connector_cfg = kzalloc(sizeof(*connector_cfg), GFP_KERNEL);
	if (!connector_cfg)
		return -ENOMEM;

	connector_cfg->possible_encoders = possible_encoders;
	list_add_tail(&connector_cfg->list, &config->connectors);

	return 0;
}
