// SPDX-License-Identifier: GPL-2.0+

#include <linux/configfs.h>
#include <linux/mutex.h>
#include <linux/slab.h>

#include "vkms_drv.h"
#include "vkms_config.h"
#include "vkms_configfs.h"

/* To avoid registering configfs more than once or unregistering on error */
static bool is_configfs_registered;

/**
 * struct vkms_configfs - Configfs configuration for a VKMS device
 *
 * @vkms_config: Configuration of the VKMS device
 * @device_group: Top level configuration group that represents a VKMS device.
 * Initialized when a new directory is created under "/config/vkms/"
 * @crtcs_group: Default subgroup of @device_group at "/config/vkms/crtcs".
 * Each of its items represent a CRTC
 * @encoders_group: Default subgroup of @device_group at
 * "/config/vkms/encoders". Each of its items represent a encoder
 * @lock: Lock used to project concurrent access to the configuration attributes
 * @enabled: Protected by @lock. The device is created or destroyed when this
 * option changes
 */
struct vkms_configfs {
	struct vkms_config *vkms_config;
	struct config_group device_group;
	struct config_group crtcs_group;
	struct config_group encoders_group;

	/* protected by @lock */
	struct mutex lock;
	bool enabled;
};

#define config_item_to_vkms_configfs(item) \
	container_of(to_config_group(item), struct vkms_configfs, device_group)

#define crtcs_group_to_vkms_configfs(group) \
	container_of(group, struct vkms_configfs, crtcs_group)

#define crtcs_item_to_vkms_configfs(item) \
	container_of(to_config_group(item), struct vkms_configfs, crtcs_group)

#define crtcs_item_to_vkms_config_crtc(item) \
	container_of(to_config_group(item), struct vkms_config_crtc, crtc_group)

#define encoders_group_to_vkms_configfs(group) \
	container_of(group, struct vkms_configfs, encoders_group)

#define encoders_item_to_vkms_config_encoder(item) \
	container_of(to_config_group(item), struct vkms_config_encoder, encoder_group)

#define encoder_possible_crtcs_item_to_vkms_config_encoder(item) \
	container_of(to_config_group(item), struct vkms_config_encoder, possible_crtcs_group)

static ssize_t crtc_cursor_show(struct config_item *item, char *page)
{
	struct vkms_config_crtc *crtc_cfg = crtcs_item_to_vkms_config_crtc(item);

	return sprintf(page, "%d\n", crtc_cfg->cursor);
}

static ssize_t crtc_cursor_store(struct config_item *item, const char *page,
				 size_t count)
{
	struct vkms_configfs *configfs = crtcs_item_to_vkms_configfs(item->ci_parent);
	struct vkms_config_crtc *crtc_cfg = crtcs_item_to_vkms_config_crtc(item);
	bool cursor;

	if (kstrtobool(page, &cursor))
		return -EINVAL;

	mutex_lock(&configfs->lock);

	if (configfs->enabled) {
		mutex_unlock(&configfs->lock);
		return -EINVAL;
	}

	crtc_cfg->cursor = cursor;

	mutex_unlock(&configfs->lock);

	return (ssize_t)count;
}

static ssize_t crtc_writeback_show(struct config_item *item, char *page)
{
	struct vkms_config_crtc *crtc_cfg = crtcs_item_to_vkms_config_crtc(item);

	return sprintf(page, "%d\n", crtc_cfg->writeback);
}

static ssize_t crtc_writeback_store(struct config_item *item, const char *page,
				    size_t count)
{
	struct vkms_configfs *configfs = crtcs_item_to_vkms_configfs(item->ci_parent);
	struct vkms_config_crtc *crtc_cfg = crtcs_item_to_vkms_config_crtc(item);
	bool writeback;

	if (kstrtobool(page, &writeback))
		return -EINVAL;

	mutex_lock(&configfs->lock);

	if (configfs->enabled) {
		mutex_unlock(&configfs->lock);
		return -EINVAL;
	}

	crtc_cfg->writeback = writeback;

	mutex_unlock(&configfs->lock);

	return (ssize_t)count;
}

CONFIGFS_ATTR(crtc_, cursor);
CONFIGFS_ATTR(crtc_, writeback);

static struct configfs_attribute *crtc_group_attrs[] = {
	&crtc_attr_cursor,
	&crtc_attr_writeback,
	NULL,
};

static const struct config_item_type crtc_group_type = {
	.ct_attrs = crtc_group_attrs,
	.ct_owner = THIS_MODULE,
};

static struct config_group *make_crtcs_group(struct config_group *group,
					     const char *name)
{
	struct vkms_configfs *configfs = crtcs_group_to_vkms_configfs(group);
	struct vkms_config_crtc *crtc_cfg;
	int ret;

	mutex_lock(&configfs->lock);

	if (configfs->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	crtc_cfg = vkms_config_add_crtc(configfs->vkms_config, false, false);
	if (IS_ERR(crtc_cfg)) {
		ret = PTR_ERR(crtc_cfg);
		goto err_unlock;
	}

	config_group_init_type_name(&crtc_cfg->crtc_group, name, &crtc_group_type);

	mutex_unlock(&configfs->lock);

	return &crtc_cfg->crtc_group;

err_unlock:
	mutex_unlock(&configfs->lock);
	return ERR_PTR(ret);
}

static void drop_crtcs_group(struct config_group *group,
			     struct config_item *item)
{
	struct vkms_configfs *configfs = crtcs_group_to_vkms_configfs(group);
	struct vkms_config_crtc *crtc_cfg = crtcs_item_to_vkms_config_crtc(item);

	vkms_config_destroy_crtc(configfs->vkms_config, crtc_cfg);
}

static struct configfs_group_operations crtcs_group_ops = {
	.make_group = &make_crtcs_group,
	.drop_item = &drop_crtcs_group,
};

static struct config_item_type crtcs_group_type = {
	.ct_group_ops = &crtcs_group_ops,
	.ct_owner = THIS_MODULE,
};

static int encoder_possible_crtcs_allow_link(struct config_item *src,
					     struct config_item *target)
{
	struct vkms_config_encoder *encoder_cfg;
	struct vkms_config_crtc *crtc_cfg;

	if (target->ci_type != &crtc_group_type)
		return -EINVAL;

	encoder_cfg = encoder_possible_crtcs_item_to_vkms_config_encoder(src);
	crtc_cfg = crtcs_item_to_vkms_config_crtc(target);

	if (encoder_cfg->possible_crtcs & BIT(crtc_cfg->index))
		return -EINVAL;

	encoder_cfg->possible_crtcs |= BIT(crtc_cfg->index);

	return 0;
}

static void encoder_possible_crtcs_drop_link(struct config_item *src,
					     struct config_item *target)
{
	struct vkms_config_encoder *encoder_cfg;
	struct vkms_config_crtc *crtc_cfg;

	encoder_cfg = encoder_possible_crtcs_item_to_vkms_config_encoder(src);
	crtc_cfg = crtcs_item_to_vkms_config_crtc(target);

	encoder_cfg->possible_crtcs &= ~BIT(crtc_cfg->index);
}

static struct configfs_item_operations encoder_possible_crtcs_item_ops = {
	.allow_link = &encoder_possible_crtcs_allow_link,
	.drop_link = &encoder_possible_crtcs_drop_link,
};

static struct config_item_type encoder_possible_crtcs_group_type = {
	.ct_item_ops = &encoder_possible_crtcs_item_ops,
	.ct_owner = THIS_MODULE,
};

static const struct config_item_type encoder_group_type = {
	.ct_owner = THIS_MODULE,
};

static struct config_group *make_encoders_group(struct config_group *group,
						const char *name)
{
	struct vkms_configfs *configfs = encoders_group_to_vkms_configfs(group);
	struct vkms_config_encoder *encoder_cfg;
	int ret;

	mutex_lock(&configfs->lock);

	if (configfs->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	encoder_cfg = vkms_config_add_encoder(configfs->vkms_config, 0);
	if (IS_ERR(encoder_cfg)) {
		ret = PTR_ERR(encoder_cfg);
		goto err_unlock;
	}

	config_group_init_type_name(&encoder_cfg->encoder_group, name,
				    &encoder_group_type);

	config_group_init_type_name(&encoder_cfg->possible_crtcs_group,
				    "possible_crtcs",
				    &encoder_possible_crtcs_group_type);
	configfs_add_default_group(&encoder_cfg->possible_crtcs_group,
				   &encoder_cfg->encoder_group);

	mutex_unlock(&configfs->lock);

	return &encoder_cfg->encoder_group;

err_unlock:
	mutex_unlock(&configfs->lock);
	return ERR_PTR(ret);
}

static void drop_encoders_group(struct config_group *group,
				struct config_item *item)
{
	struct vkms_configfs *configfs = encoders_group_to_vkms_configfs(group);
	struct vkms_config_encoder *encoder_cfg =
		encoders_item_to_vkms_config_encoder(item);

	vkms_config_destroy_encoder(configfs->vkms_config, encoder_cfg);
}

static struct configfs_group_operations encoders_group_ops = {
	.make_group = &make_encoders_group,
	.drop_item = &drop_encoders_group,
};

static struct config_item_type encoders_group_type = {
	.ct_group_ops = &encoders_group_ops,
	.ct_owner = THIS_MODULE,
};

static ssize_t device_enabled_show(struct config_item *item, char *page)
{
	struct vkms_configfs *configfs = config_item_to_vkms_configfs(item);

	return sprintf(page, "%d\n", configfs->enabled);
}

static ssize_t device_enabled_store(struct config_item *item, const char *page,
				    size_t count)
{
	struct vkms_configfs *configfs = config_item_to_vkms_configfs(item);
	bool enabled;
	int ret = 0;

	if (kstrtobool(page, &enabled))
		return -EINVAL;

	mutex_lock(&configfs->lock);

	if (!configfs->enabled && enabled)
		ret = vkms_create(configfs->vkms_config);
	else if (configfs->enabled && !enabled)
		vkms_destroy(configfs->vkms_config);

	if (ret)
		goto err_unlock;

	configfs->enabled = enabled;

	mutex_unlock(&configfs->lock);

	return (ssize_t)count;

err_unlock:
	mutex_unlock(&configfs->lock);
	return ret;
}

CONFIGFS_ATTR(device_, enabled);

static struct configfs_attribute *device_group_attrs[] = {
	&device_attr_enabled,
	NULL,
};

static const struct config_item_type device_group_type = {
	.ct_attrs = device_group_attrs,
	.ct_owner = THIS_MODULE,
};

static struct config_group *make_device_group(struct config_group *group,
					      const char *name)
{
	struct vkms_configfs *configfs;
	struct vkms_config_connector *connector_cfg = NULL;
	char *config_name;
	int ret;

	if (strcmp(name, DEFAULT_DEVICE_NAME) == 0)
		return ERR_PTR(-EINVAL);

	configfs = kzalloc(sizeof(*configfs), GFP_KERNEL);
	if (!configfs)
		return ERR_PTR(-ENOMEM);

	config_group_init_type_name(&configfs->device_group, name, &device_group_type);
	mutex_init(&configfs->lock);

	config_name = config_item_name(&configfs->device_group.cg_item);
	configfs->vkms_config = vkms_config_create(config_name);
	if (IS_ERR(configfs->vkms_config)) {
		ret = PTR_ERR(configfs->vkms_config);
		goto err_kfree;
	}

	config_group_init_type_name(&configfs->crtcs_group, "crtcs",
				    &crtcs_group_type);
	configfs_add_default_group(&configfs->crtcs_group,
				   &configfs->device_group);

	config_group_init_type_name(&configfs->encoders_group, "encoders",
				    &encoders_group_type);
	configfs_add_default_group(&configfs->encoders_group,
				   &configfs->device_group);

	connector_cfg = vkms_config_add_connector(configfs->vkms_config, BIT(0),
						  connector_status_connected);
	if (IS_ERR(connector_cfg)) {
		ret = PTR_ERR(connector_cfg);
		goto err_kfree;
	}

	return &configfs->device_group;

err_kfree:
	kfree(configfs);
	kfree(connector_cfg);
	return ERR_PTR(ret);
}

static void drop_device_group(struct config_group *group,
			      struct config_item *item)
{
	struct vkms_configfs *configfs = config_item_to_vkms_configfs(item);

	mutex_lock(&configfs->lock);

	if (configfs->enabled)
		vkms_destroy(configfs->vkms_config);

	kfree(configfs->vkms_config);

	mutex_unlock(&configfs->lock);

	kfree(configfs);
}

static struct configfs_group_operations device_group_ops = {
	.make_group = &make_device_group,
	.drop_item = &drop_device_group,
};

static struct config_item_type vkms_type = {
	.ct_group_ops = &device_group_ops,
	.ct_owner = THIS_MODULE,
};

static struct configfs_subsystem vkms_subsys = {
	.su_group = {
		.cg_item = {
			.ci_name = "vkms",
			.ci_type = &vkms_type,
		},
	},
	.su_mutex = __MUTEX_INITIALIZER(vkms_subsys.su_mutex),
};

int vkms_configfs_register(void)
{
	int ret;

	if (is_configfs_registered)
		return 0;

	config_group_init(&vkms_subsys.su_group);
	ret = configfs_register_subsystem(&vkms_subsys);

	is_configfs_registered = ret == 0;

	return ret;
}

void vkms_configfs_unregister(void)
{
	if (is_configfs_registered)
		configfs_unregister_subsystem(&vkms_subsys);
}
