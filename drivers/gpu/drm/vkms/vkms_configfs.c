// SPDX-License-Identifier: GPL-2.0+

#include <linux/slab.h>

#include "vkms_drv.h"
#include "vkms_config.h"
#include "vkms_configfs.h"

/* To avoid registering configfs more than once or unregistering on error */
static bool is_configfs_registered;

/**
 * struct vkms_configfs - Configfs configuration for a VKMS device
 *
 * @group: Top level configuration group. Initialized when a new directory is
 * created under "/config/vkms/". Represents a VKMS device
 * @vkms_config: Configuration of the VKMS device
 * @lock: Lock used to project concurrent access to the configuration attributes
 * @enabled: Protected by @lock. The device is created or destroyed when this
 * option changes
 */
struct vkms_configfs {
	struct config_group group;
	struct vkms_config *vkms_config;

	/* protected by @lock */
	struct mutex lock;
	bool enabled;
};

#define config_item_to_vkms_configfs(item) \
	container_of(to_config_group(item), struct vkms_configfs, group)

static ssize_t configfs_device_enabled_show(struct config_item *item, char *page)
{
	struct vkms_configfs *configfs = config_item_to_vkms_configfs(item);

	return sprintf(page, "%d\n", configfs->enabled);
}

static ssize_t configfs_device_enabled_store(struct config_item *item,
					     const char *page, size_t count)
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

CONFIGFS_ATTR(configfs_device_, enabled);

static struct configfs_attribute *configfs_device_attr[] = {
	&configfs_device_attr_enabled,
	NULL,
};

static const struct config_item_type configfs_device = {
	.ct_attrs = configfs_device_attr,
	.ct_owner = THIS_MODULE,
};

static struct config_group *make_configfs(struct config_group *group,
					  const char *name)
{
	struct vkms_configfs *configfs;
	char *config_name;

	if (strcmp(name, DEFAULT_DEVICE_NAME) == 0)
		return ERR_PTR(-EINVAL);

	configfs = kzalloc(sizeof(*configfs), GFP_KERNEL);
	if (!configfs)
		return ERR_PTR(-ENOMEM);

	config_name = config_item_name(&configfs->group.cg_item);
	configfs->vkms_config = vkms_config_create(config_name);
	if (!configfs->vkms_config) {
		kfree(configfs);
		return ERR_PTR(-ENOMEM);
	}

	config_group_init_type_name(&configfs->group, name, &configfs_device);
	mutex_init(&configfs->lock);

	return &configfs->group;
}

static void drop_configfs(struct config_group *group, struct config_item *item)
{
	struct vkms_configfs *configfs = config_item_to_vkms_configfs(item);

	mutex_lock(&configfs->lock);

	if (configfs->enabled)
		vkms_destroy(configfs->vkms_config);

	kfree(configfs->vkms_config);

	mutex_unlock(&configfs->lock);

	kfree(configfs);
}

static struct configfs_group_operations root_group_ops = {
	.make_group = &make_configfs,
	.drop_item = &drop_configfs,
};

static struct config_item_type vkms_type = {
	.ct_group_ops = &root_group_ops,
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
