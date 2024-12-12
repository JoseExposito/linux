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
 * struct vkms_configfs_device - Configfs representation of a VKMS device
 *
 * @group: Top level configuration group that represents a VKMS device.
 * Initialized when a new directory is created under "/config/vkms/"
 * @planes_group: Default subgroup of @group at "/config/vkms/planes"
 * @crtcs_group: Default subgroup of @group at "/config/vkms/crtcs"
 * @encoders_group: Default subgroup of @group at "/config/vkms/encoders"
 * @connectors_group: Default subgroup of @group at "/config/vkms/connectors"
 * @lock: Lock used to project concurrent access to the configuration attributes
 * @config: Protected by @lock. Configuration of the VKMS device
 * @enabled: Protected by @lock. The device is created or destroyed when this
 * option changes
 */
struct vkms_configfs_device {
	struct config_group group;
	struct config_group planes_group;
	struct config_group crtcs_group;
	struct config_group encoders_group;
	struct config_group connectors_group;

	struct mutex lock;
	struct vkms_config *config;
	bool enabled;
};

/**
 * struct vkms_configfs_plane - Configfs representation of a plane
 *
 * @group: Top level configuration group that represents a plane.
 * Initialized when a new directory is created under "/config/vkms/planes"
 * @possible_crtcs_group: Default subgroup of @group at "plane/possible_crtcs"
 * @dev: The vkms_configfs_device this plane belongs to
 * @config: Configuration of the VKMS plane
 */
struct vkms_configfs_plane {
	struct config_group group;
	struct config_group possible_crtcs_group;
	struct vkms_configfs_device *dev;
	struct vkms_config_plane *config;
};

/**
 * struct vkms_configfs_crtc - Configfs representation of a CRTC
 *
 * @group: Top level configuration group that represents a CRTC.
 * Initialized when a new directory is created under "/config/vkms/crtcs"
 * @dev: The vkms_configfs_device this CRTC belongs to
 * @config: Configuration of the VKMS CRTC
 */
struct vkms_configfs_crtc {
	struct config_group group;
	struct vkms_configfs_device *dev;
	struct vkms_config_crtc *config;
};

/**
 * struct vkms_configfs_encoder - Configfs representation of a encoder
 *
 * @group: Top level configuration group that represents a encoder.
 * Initialized when a new directory is created under "/config/vkms/encoders"
 * @possible_crtcs_group: Default subgroup of @group at "encoder/possible_crtcs"
 * @dev: The vkms_configfs_device this encoder belongs to
 * @config: Configuration of the VKMS encoder
 */
struct vkms_configfs_encoder {
	struct config_group group;
	struct config_group possible_crtcs_group;
	struct vkms_configfs_device *dev;
	struct vkms_config_encoder *config;
};

/**
 * struct vkms_configfs_connector - Configfs representation of a connector
 *
 * @group: Top level configuration group that represents a connector.
 * Initialized when a new directory is created under "/config/vkms/connectors"
 * @possible_encoders_group: Default subgroup of @group at
 * "connector/possible_encoders"
 * @dev: The vkms_configfs_device this connector belongs to
 * @config: Configuration of the VKMS connector
 */
struct vkms_configfs_connector {
	struct config_group group;
	struct config_group possible_encoders_group;
	struct vkms_configfs_device *dev;
	struct vkms_config_connector *config;
};

#define device_item_to_vkms_configfs_device(item) \
	container_of(to_config_group((item)), struct vkms_configfs_device, \
		     group)

#define child_group_to_vkms_configfs_device(group) \
	device_item_to_vkms_configfs_device((&(group)->cg_item)->ci_parent)

#define plane_item_to_vkms_configfs_plane(item) \
	container_of(to_config_group((item)), struct vkms_configfs_plane, group)

#define plane_possible_crtcs_item_to_vkms_configfs_plane(item) \
	container_of(to_config_group((item)), struct vkms_configfs_plane, \
		     possible_crtcs_group)

#define crtc_item_to_vkms_configfs_crtc(item) \
	container_of(to_config_group((item)), struct vkms_configfs_crtc, group)

#define encoder_item_to_vkms_configfs_encoder(item) \
	container_of(to_config_group((item)), struct vkms_configfs_encoder, \
		     group)

#define encoder_possible_crtcs_item_to_vkms_configfs_encoder(item) \
	container_of(to_config_group((item)), struct vkms_configfs_encoder, \
		     possible_crtcs_group)

#define connector_item_to_vkms_configfs_connector(item) \
	container_of(to_config_group((item)), struct vkms_configfs_connector, \
		     group)

#define connector_possible_encoders_item_to_vkms_configfs_connector(item) \
	container_of(to_config_group((item)), struct vkms_configfs_connector, \
		     possible_encoders_group)

static ssize_t crtc_writeback_show(struct config_item *item, char *page)
{
	struct vkms_configfs_crtc *crtc;
	bool writeback;

	crtc = crtc_item_to_vkms_configfs_crtc(item);

	mutex_lock(&crtc->dev->lock);
	writeback = vkms_config_crtc_get_writeback(crtc->config);
	mutex_unlock(&crtc->dev->lock);

	return sprintf(page, "%d\n", writeback);
}

static ssize_t crtc_writeback_store(struct config_item *item, const char *page,
				    size_t count)
{
	struct vkms_configfs_crtc *crtc;
	bool writeback;

	crtc = crtc_item_to_vkms_configfs_crtc(item);

	if (kstrtobool(page, &writeback))
		return -EINVAL;

	mutex_lock(&crtc->dev->lock);

	if (crtc->dev->enabled) {
		mutex_unlock(&crtc->dev->lock);
		return -EPERM;
	}

	vkms_config_crtc_set_writeback(crtc->config, writeback);

	mutex_unlock(&crtc->dev->lock);

	return (ssize_t)count;
}

CONFIGFS_ATTR(crtc_, writeback);

static struct configfs_attribute *crtc_item_attrs[] = {
	&crtc_attr_writeback,
	NULL,
};

static void crtc_release(struct config_item *item)
{
	struct vkms_configfs_crtc *crtc;
	struct mutex *lock;

	crtc = crtc_item_to_vkms_configfs_crtc(item);
	lock = &crtc->dev->lock;

	mutex_lock(lock);
	vkms_config_destroy_crtc(crtc->dev->config, crtc->config);
	kfree(crtc);
	mutex_unlock(lock);
}

static struct configfs_item_operations crtc_item_operations = {
	.release	= &crtc_release,
};

static const struct config_item_type crtc_item_type = {
	.ct_attrs	= crtc_item_attrs,
	.ct_item_ops	= &crtc_item_operations,
	.ct_owner	= THIS_MODULE,
};

static struct config_group *make_crtc_group(struct config_group *group,
					    const char *name)
{
	struct vkms_configfs_device *dev;
	struct vkms_configfs_crtc *crtc;
	int ret;

	dev = child_group_to_vkms_configfs_device(group);

	mutex_lock(&dev->lock);

	if (dev->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	crtc = kzalloc(sizeof(*crtc), GFP_KERNEL);
	if (!crtc) {
		ret = -ENOMEM;
		goto err_unlock;
	}

	crtc->dev = dev;

	crtc->config = vkms_config_create_crtc(dev->config);
	if (IS_ERR(crtc->config)) {
		ret = PTR_ERR(crtc->config);
		goto err_free;
	}

	config_group_init_type_name(&crtc->group, name, &crtc_item_type);

	mutex_unlock(&dev->lock);

	return &crtc->group;

err_free:
	kfree(crtc);
err_unlock:
	mutex_unlock(&dev->lock);
	return ERR_PTR(ret);
}

static struct configfs_group_operations crtcs_group_operations = {
	.make_group	= &make_crtc_group,
};

static const struct config_item_type crtc_group_type = {
	.ct_group_ops	= &crtcs_group_operations,
	.ct_owner	= THIS_MODULE,
};

static int plane_possible_crtcs_allow_link(struct config_item *src,
					   struct config_item *target)
{
	struct vkms_configfs_plane *plane;
	struct vkms_configfs_crtc *crtc;
	int ret;

	if (target->ci_type != &crtc_item_type)
		return -EINVAL;

	plane = plane_possible_crtcs_item_to_vkms_configfs_plane(src);
	crtc = crtc_item_to_vkms_configfs_crtc(target);

	mutex_lock(&plane->dev->lock);

	if (plane->dev->enabled) {
		mutex_unlock(&plane->dev->lock);
		return -EPERM;
	}

	ret = vkms_config_plane_attach_crtc(plane->config, crtc->config);
	mutex_unlock(&plane->dev->lock);

	return ret;
}

static void plane_possible_crtcs_drop_link(struct config_item *src,
					   struct config_item *target)
{
	struct vkms_configfs_plane *plane;
	struct vkms_configfs_crtc *crtc;

	plane = plane_possible_crtcs_item_to_vkms_configfs_plane(src);
	crtc = crtc_item_to_vkms_configfs_crtc(target);

	mutex_lock(&plane->dev->lock);
	vkms_config_plane_detach_crtc(plane->config, crtc->config);
	mutex_unlock(&plane->dev->lock);
}

static struct configfs_item_operations plane_possible_crtcs_item_operations = {
	.allow_link	= plane_possible_crtcs_allow_link,
	.drop_link	= plane_possible_crtcs_drop_link,
};

static const struct config_item_type plane_possible_crtcs_group_type = {
	.ct_item_ops	= &plane_possible_crtcs_item_operations,
	.ct_owner	= THIS_MODULE,
};

static ssize_t plane_type_show(struct config_item *item, char *page)
{
	struct vkms_configfs_plane *plane;
	enum drm_plane_type type;

	plane = plane_item_to_vkms_configfs_plane(item);

	mutex_lock(&plane->dev->lock);
	type = vkms_config_plane_get_type(plane->config);
	mutex_unlock(&plane->dev->lock);

	return sprintf(page, "%u", type);
}

static ssize_t plane_type_store(struct config_item *item, const char *page,
				size_t count)
{
	struct vkms_configfs_plane *plane;
	enum drm_plane_type type;

	plane = plane_item_to_vkms_configfs_plane(item);

	if (kstrtouint(page, 10, &type))
		return -EINVAL;

	if (type != DRM_PLANE_TYPE_OVERLAY && type != DRM_PLANE_TYPE_PRIMARY &&
	    type != DRM_PLANE_TYPE_CURSOR)
		return -EINVAL;

	mutex_lock(&plane->dev->lock);

	if (plane->dev->enabled) {
		mutex_unlock(&plane->dev->lock);
		return -EPERM;
	}

	vkms_config_plane_set_type(plane->config, type);

	mutex_unlock(&plane->dev->lock);

	return (ssize_t)count;
}

CONFIGFS_ATTR(plane_, type);

static struct configfs_attribute *plane_item_attrs[] = {
	&plane_attr_type,
	NULL,
};

static void plane_release(struct config_item *item)
{
	struct vkms_configfs_plane *plane;
	struct mutex *lock;

	plane = plane_item_to_vkms_configfs_plane(item);
	lock = &plane->dev->lock;

	mutex_lock(lock);
	vkms_config_destroy_plane(plane->config);
	kfree(plane);
	mutex_unlock(lock);
}

static struct configfs_item_operations plane_item_operations = {
	.release	= &plane_release,
};

static const struct config_item_type plane_item_type = {
	.ct_attrs	= plane_item_attrs,
	.ct_item_ops	= &plane_item_operations,
	.ct_owner	= THIS_MODULE,
};

static struct config_group *make_plane_group(struct config_group *group,
					     const char *name)
{
	struct vkms_configfs_device *dev;
	struct vkms_configfs_plane *plane;
	int ret;

	dev = child_group_to_vkms_configfs_device(group);

	mutex_lock(&dev->lock);

	if (dev->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	plane = kzalloc(sizeof(*plane), GFP_KERNEL);
	if (!plane) {
		ret = -ENOMEM;
		goto err_unlock;
	}

	plane->dev = dev;

	plane->config = vkms_config_create_plane(dev->config);
	if (IS_ERR(plane->config)) {
		ret = PTR_ERR(plane->config);
		goto err_free;
	}

	config_group_init_type_name(&plane->group, name, &plane_item_type);

	config_group_init_type_name(&plane->possible_crtcs_group,
				    "possible_crtcs",
				    &plane_possible_crtcs_group_type);
	configfs_add_default_group(&plane->possible_crtcs_group, &plane->group);

	mutex_unlock(&dev->lock);

	return &plane->group;

err_free:
	kfree(plane);
err_unlock:
	mutex_unlock(&dev->lock);
	return ERR_PTR(ret);
}

static struct configfs_group_operations planes_group_operations = {
	.make_group	= &make_plane_group,
};

static const struct config_item_type plane_group_type = {
	.ct_group_ops	= &planes_group_operations,
	.ct_owner	= THIS_MODULE,
};

static int encoder_possible_crtcs_allow_link(struct config_item *src,
					     struct config_item *target)
{
	struct vkms_configfs_encoder *encoder;
	struct vkms_configfs_crtc *crtc;
	int ret;

	if (target->ci_type != &crtc_item_type)
		return -EINVAL;

	encoder = encoder_possible_crtcs_item_to_vkms_configfs_encoder(src);
	crtc = crtc_item_to_vkms_configfs_crtc(target);

	mutex_lock(&encoder->dev->lock);

	if (encoder->dev->enabled) {
		mutex_unlock(&encoder->dev->lock);
		return -EPERM;
	}

	ret = vkms_config_encoder_attach_crtc(encoder->config, crtc->config);

	mutex_unlock(&encoder->dev->lock);

	return ret;
}

static void encoder_possible_crtcs_drop_link(struct config_item *src,
					     struct config_item *target)
{
	struct vkms_configfs_encoder *encoder;
	struct vkms_configfs_crtc *crtc;

	encoder = encoder_possible_crtcs_item_to_vkms_configfs_encoder(src);
	crtc = crtc_item_to_vkms_configfs_crtc(target);

	mutex_lock(&encoder->dev->lock);
	vkms_config_encoder_detach_crtc(encoder->config, crtc->config);
	mutex_unlock(&encoder->dev->lock);
}

static struct configfs_item_operations encoder_possible_crtcs_item_operations = {
	.allow_link	= encoder_possible_crtcs_allow_link,
	.drop_link	= encoder_possible_crtcs_drop_link,
};

static const struct config_item_type encoder_possible_crtcs_group_type = {
	.ct_item_ops	= &encoder_possible_crtcs_item_operations,
	.ct_owner	= THIS_MODULE,
};

static void encoder_release(struct config_item *item)
{
	struct vkms_configfs_encoder *encoder;
	struct mutex *lock;

	encoder = encoder_item_to_vkms_configfs_encoder(item);
	lock = &encoder->dev->lock;

	mutex_lock(lock);
	vkms_config_destroy_encoder(encoder->dev->config, encoder->config);
	kfree(encoder);
	mutex_unlock(lock);
}

static struct configfs_item_operations encoder_item_operations = {
	.release	= &encoder_release,
};

static const struct config_item_type encoder_item_type = {
	.ct_item_ops	= &encoder_item_operations,
	.ct_owner	= THIS_MODULE,
};

static struct config_group *make_encoder_group(struct config_group *group,
					       const char *name)
{
	struct vkms_configfs_device *dev;
	struct vkms_configfs_encoder *encoder;
	int ret;

	dev = child_group_to_vkms_configfs_device(group);

	mutex_lock(&dev->lock);

	if (dev->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	encoder = kzalloc(sizeof(*encoder), GFP_KERNEL);
	if (!encoder) {
		ret = -ENOMEM;
		goto err_unlock;
	}

	encoder->dev = dev;

	encoder->config = vkms_config_create_encoder(dev->config);
	if (IS_ERR(encoder->config)) {
		ret = PTR_ERR(encoder->config);
		goto err_free;
	}

	config_group_init_type_name(&encoder->group, name, &encoder_item_type);

	config_group_init_type_name(&encoder->possible_crtcs_group,
				    "possible_crtcs",
				    &encoder_possible_crtcs_group_type);
	configfs_add_default_group(&encoder->possible_crtcs_group,
				   &encoder->group);

	mutex_unlock(&dev->lock);

	return &encoder->group;

err_free:
	kfree(encoder);
err_unlock:
	mutex_unlock(&dev->lock);
	return ERR_PTR(ret);
}

static struct configfs_group_operations encoders_group_operations = {
	.make_group	= &make_encoder_group,
};

static const struct config_item_type encoder_group_type = {
	.ct_group_ops	= &encoders_group_operations,
	.ct_owner	= THIS_MODULE,
};

static void connector_release(struct config_item *item)
{
	struct vkms_configfs_connector *connector;
	struct mutex *lock;

	connector = connector_item_to_vkms_configfs_connector(item);
	lock = &connector->dev->lock;

	mutex_lock(lock);
	vkms_config_destroy_connector(connector->config);
	kfree(connector);
	mutex_unlock(lock);
}

static struct configfs_item_operations connector_item_operations = {
	.release	= &connector_release,
};

static const struct config_item_type connector_item_type = {
	.ct_item_ops	= &connector_item_operations,
	.ct_owner	= THIS_MODULE,
};

static int connector_possible_encoders_allow_link(struct config_item *src,
						  struct config_item *target)
{
	struct vkms_configfs_connector *connector;
	struct vkms_configfs_encoder *encoder;
	int ret;

	if (target->ci_type != &encoder_item_type)
		return -EINVAL;

	connector = connector_possible_encoders_item_to_vkms_configfs_connector(src);
	encoder = encoder_item_to_vkms_configfs_encoder(target);

	mutex_lock(&connector->dev->lock);

	if (connector->dev->enabled) {
		mutex_unlock(&connector->dev->lock);
		return -EPERM;
	}

	ret = vkms_config_connector_attach_encoder(connector->config,
						   encoder->config);

	mutex_unlock(&connector->dev->lock);

	return ret;
}

static void connector_possible_encoders_drop_link(struct config_item *src,
						  struct config_item *target)
{
	struct vkms_configfs_connector *connector;
	struct vkms_configfs_encoder *encoder;

	connector = connector_possible_encoders_item_to_vkms_configfs_connector(src);
	encoder = encoder_item_to_vkms_configfs_encoder(target);

	mutex_lock(&connector->dev->lock);
	vkms_config_connector_detach_encoder(connector->config,
					     encoder->config);
	mutex_unlock(&connector->dev->lock);
}

static struct configfs_item_operations connector_possible_encoders_item_operations = {
	.allow_link	= connector_possible_encoders_allow_link,
	.drop_link	= connector_possible_encoders_drop_link,
};

static const struct config_item_type connector_possible_encoders_group_type = {
	.ct_item_ops	= &connector_possible_encoders_item_operations,
	.ct_owner	= THIS_MODULE,
};

static struct config_group *make_connector_group(struct config_group *group,
						 const char *name)
{
	struct vkms_configfs_device *dev;
	struct vkms_configfs_connector *connector;
	int ret;

	dev = child_group_to_vkms_configfs_device(group);

	mutex_lock(&dev->lock);

	if (dev->enabled) {
		ret = -EINVAL;
		goto err_unlock;
	}

	connector = kzalloc(sizeof(*connector), GFP_KERNEL);
	if (!connector) {
		ret = -ENOMEM;
		goto err_unlock;
	}

	connector->dev = dev;

	connector->config = vkms_config_create_connector(dev->config);
	if (IS_ERR(connector->config)) {
		ret = PTR_ERR(connector->config);
		goto err_free;
	}

	config_group_init_type_name(&connector->group, name,
				    &connector_item_type);

	config_group_init_type_name(&connector->possible_encoders_group,
				    "possible_encoders",
				    &connector_possible_encoders_group_type);
	configfs_add_default_group(&connector->possible_encoders_group,
				   &connector->group);

	mutex_unlock(&dev->lock);

	return &connector->group;

err_free:
	kfree(connector);
err_unlock:
	mutex_unlock(&dev->lock);
	return ERR_PTR(ret);
}

static struct configfs_group_operations connectors_group_operations = {
	.make_group	= &make_connector_group,
};

static const struct config_item_type connector_group_type = {
	.ct_group_ops	= &connectors_group_operations,
	.ct_owner	= THIS_MODULE,
};

static ssize_t device_enabled_show(struct config_item *item, char *page)
{
	struct vkms_configfs_device *dev;
	bool enabled;

	dev = device_item_to_vkms_configfs_device(item);

	mutex_lock(&dev->lock);
	enabled = dev->enabled;
	mutex_unlock(&dev->lock);

	return sprintf(page, "%d\n", enabled);
}

static ssize_t device_enabled_store(struct config_item *item, const char *page,
				    size_t count)
{
	struct vkms_configfs_device *dev;
	bool enabled;
	int ret = 0;

	dev = device_item_to_vkms_configfs_device(item);

	if (kstrtobool(page, &enabled))
		return -EINVAL;

	mutex_lock(&dev->lock);

	if (!dev->enabled && enabled) {
		if (!vkms_config_is_valid(dev->config)) {
			ret = -EINVAL;
			goto err_unlock;
		}

		ret = vkms_create(dev->config);
	} else if (dev->enabled && !enabled) {
		vkms_destroy(dev->config);
	}

	if (ret)
		goto err_unlock;

	dev->enabled = enabled;

	mutex_unlock(&dev->lock);

	return (ssize_t)count;

err_unlock:
	mutex_unlock(&dev->lock);
	return ret;
}

CONFIGFS_ATTR(device_, enabled);

static struct configfs_attribute *device_item_attrs[] = {
	&device_attr_enabled,
	NULL,
};

static void device_release(struct config_item *item)
{
	struct vkms_configfs_device *dev;

	dev = device_item_to_vkms_configfs_device(item);

	if (dev->enabled)
		vkms_destroy(dev->config);

	mutex_destroy(&dev->lock);
	vkms_config_destroy(dev->config);
	kfree(dev);
}

static struct configfs_item_operations device_item_operations = {
	.release	= &device_release,
};

static const struct config_item_type device_item_type = {
	.ct_attrs	= device_item_attrs,
	.ct_item_ops	= &device_item_operations,
	.ct_owner	= THIS_MODULE,
};

static struct config_group *make_device_group(struct config_group *group,
					      const char *name)
{
	struct vkms_configfs_device *dev;

	if (strcmp(name, DEFAULT_DEVICE_NAME) == 0)
		return ERR_PTR(-EINVAL);

	dev = kzalloc(sizeof(*dev), GFP_KERNEL);
	if (!dev)
		return ERR_PTR(-ENOMEM);

	dev->config = vkms_config_create(name);
	if (IS_ERR(dev->config)) {
		kfree(dev);
		return ERR_CAST(dev->config);
	}

	config_group_init_type_name(&dev->group, name, &device_item_type);
	mutex_init(&dev->lock);

	config_group_init_type_name(&dev->planes_group, "planes",
				    &plane_group_type);
	configfs_add_default_group(&dev->planes_group, &dev->group);

	config_group_init_type_name(&dev->crtcs_group, "crtcs",
				    &crtc_group_type);
	configfs_add_default_group(&dev->crtcs_group, &dev->group);

	config_group_init_type_name(&dev->encoders_group, "encoders",
				    &encoder_group_type);
	configfs_add_default_group(&dev->encoders_group, &dev->group);

	config_group_init_type_name(&dev->connectors_group, "connectors",
				    &connector_group_type);
	configfs_add_default_group(&dev->connectors_group, &dev->group);

	return &dev->group;
}

static struct configfs_group_operations device_group_ops = {
	.make_group = &make_device_group,
};

static const struct config_item_type device_group_type = {
	.ct_group_ops	= &device_group_ops,
	.ct_owner	= THIS_MODULE,
};

static struct configfs_subsystem vkms_subsys = {
	.su_group = {
		.cg_item = {
			.ci_name = "vkms",
			.ci_type = &device_group_type,
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
