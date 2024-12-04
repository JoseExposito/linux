// SPDX-License-Identifier: GPL-2.0+

#include <kunit/test.h>

#include "../vkms_config.h"

MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

struct default_config_case {
	bool enable_cursor;
	bool enable_writeback;
	bool enable_overlay;
};

static void vkms_config_test_empty_config(struct kunit *test)
{
	struct vkms_config *config;
	const char *dev_name = "test";

	config = vkms_config_create(dev_name);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* The dev_name string and the config have different live times */
	dev_name = NULL;
	KUNIT_EXPECT_STREQ(test, vkms_config_get_device_name(config), "test");

	KUNIT_EXPECT_TRUE(test, list_empty(&config->planes));
	KUNIT_EXPECT_TRUE(test, list_empty(&config->crtcs));
	KUNIT_EXPECT_TRUE(test, list_empty(&config->encoders));
	KUNIT_EXPECT_TRUE(test, list_empty(&config->connectors));

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static struct default_config_case default_config_cases[] = {
	{ false, false, false },
	{ true, false, false },
	{ true, true, false },
	{ true, false, true },
	{ false, true, false },
	{ false, true, true },
	{ false, false, true },
	{ true, true, true },
};

KUNIT_ARRAY_PARAM(default_config, default_config_cases, NULL);

static void vkms_config_test_default_config(struct kunit *test)
{
	const struct default_config_case *params = test->param_value;
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_connector *connector_cfg;
	int n_primaries = 0;
	int n_cursors = 0;
	int n_overlays = 0;

	config = vkms_config_default_create(params->enable_cursor,
					    params->enable_writeback,
					    params->enable_overlay);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* Planes */
	list_for_each_entry(plane_cfg, &config->planes, link) {
		switch (vkms_config_plane_get_type(plane_cfg)) {
		case DRM_PLANE_TYPE_PRIMARY:
			n_primaries++;
			break;
		case DRM_PLANE_TYPE_CURSOR:
			n_cursors++;
			break;
		case DRM_PLANE_TYPE_OVERLAY:
			n_overlays++;
			break;
		default:
			KUNIT_FAIL_AND_ABORT(test, "Unknown plane type");
		}
	}
	KUNIT_EXPECT_EQ(test, n_primaries, 1);
	KUNIT_EXPECT_EQ(test, n_cursors, params->enable_cursor ? 1 : 0);
	KUNIT_EXPECT_EQ(test, n_overlays, params->enable_overlay ? 8 : 0);

	/* CRTCs */
	crtc_cfg = list_first_entry(&config->crtcs, typeof(*crtc_cfg), link);

	KUNIT_EXPECT_EQ(test, list_count_nodes(&config->crtcs), 1);
	KUNIT_EXPECT_EQ(test, vkms_config_crtc_get_writeback(crtc_cfg),
			params->enable_writeback);

	list_for_each_entry(plane_cfg, &config->planes, link) {
		struct vkms_config_crtc *possible_crtc;
		int n_possible_crtcs = 0;
		unsigned long idx = 0;

		xa_for_each(&plane_cfg->possible_crtcs, idx, possible_crtc) {
			KUNIT_EXPECT_PTR_EQ(test, crtc_cfg, possible_crtc);
			n_possible_crtcs++;
		}
		KUNIT_EXPECT_EQ(test, n_possible_crtcs, 1);
	}

	/* Encoders */
	KUNIT_EXPECT_EQ(test, list_count_nodes(&config->encoders), 1);

	/* Connectors */
	KUNIT_EXPECT_EQ(test, list_count_nodes(&config->connectors), 1);
	connector_cfg = list_first_entry(&config->connectors,
					 typeof(*connector_cfg), link);
	KUNIT_EXPECT_TRUE(test, vkms_config_connector_is_enabled(connector_cfg));

	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_get_planes(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg1, *plane_cfg2;
	struct vkms_config_plane **array;
	size_t length;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	array = vkms_config_get_planes(config, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	plane_cfg1 = vkms_config_add_plane(config);
	array = vkms_config_get_planes(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], plane_cfg1);
	kfree(array);

	plane_cfg2 = vkms_config_add_plane(config);
	array = vkms_config_get_planes(config, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], plane_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], plane_cfg2);
	kfree(array);

	vkms_config_destroy_plane(plane_cfg1);
	array = vkms_config_get_planes(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], plane_cfg2);
	kfree(array);

	vkms_config_destroy(config);
}

static void vkms_config_test_get_crtcs(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_crtc *crtc_cfg1, *crtc_cfg2;
	struct vkms_config_crtc **array;
	size_t length;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	array = vkms_config_get_crtcs(config, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	crtc_cfg1 = vkms_config_add_crtc(config);
	array = vkms_config_get_crtcs(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	kfree(array);

	crtc_cfg2 = vkms_config_add_crtc(config);
	array = vkms_config_get_crtcs(config, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], crtc_cfg2);
	kfree(array);

	vkms_config_destroy_crtc(config, crtc_cfg2);
	array = vkms_config_get_crtcs(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	kfree(array);

	vkms_config_destroy(config);
}

static void vkms_config_test_get_encoders(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_encoder *encoder_cfg1, *encoder_cfg2;
	struct vkms_config_encoder **array;
	size_t length;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	array = vkms_config_get_encoders(config, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	encoder_cfg1 = vkms_config_add_encoder(config);
	array = vkms_config_get_encoders(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], encoder_cfg1);
	kfree(array);

	encoder_cfg2 = vkms_config_add_encoder(config);
	array = vkms_config_get_encoders(config, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], encoder_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], encoder_cfg2);
	kfree(array);

	vkms_config_destroy_encoder(config, encoder_cfg2);
	array = vkms_config_get_encoders(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], encoder_cfg1);
	kfree(array);

	vkms_config_destroy(config);
}

static void vkms_config_test_get_connectors(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_connector *connector_cfg1, *connector_cfg2;
	struct vkms_config_connector **array;
	size_t length;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	array = vkms_config_get_connectors(config, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	connector_cfg1 = vkms_config_add_connector(config);
	array = vkms_config_get_connectors(config, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	vkms_config_connector_set_enabled(connector_cfg1, true);
	array = vkms_config_get_connectors(config, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], connector_cfg1);
	kfree(array);

	connector_cfg2 = vkms_config_add_connector(config);
	vkms_config_connector_set_enabled(connector_cfg2, true);
	array = vkms_config_get_connectors(config, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], connector_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], connector_cfg2);
	kfree(array);

	vkms_config_connector_set_enabled(connector_cfg1, false);
	vkms_config_destroy_connector(connector_cfg2);
	array = vkms_config_get_connectors(config, &length);
	KUNIT_ASSERT_NULL(test, array);

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_plane_number(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	int n;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* Invalid: No planes */
	plane_cfg = list_first_entry(&config->planes, typeof(*plane_cfg), link);
	vkms_config_destroy_plane(plane_cfg);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Invalid: Too many planes */
	for (n = 0; n <= 32; n++)
		vkms_config_add_plane(config);

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_plane_type(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;
	struct vkms_config_encoder *encoder_cfg;
	int err;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	plane_cfg = list_first_entry(&config->planes, typeof(*plane_cfg), link);
	vkms_config_destroy_plane(plane_cfg);

	crtc_cfg = list_first_entry(&config->crtcs, typeof(*crtc_cfg), link);

	/* Invalid: No primary plane */
	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_OVERLAY);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Invalid: Multiple primary planes */
	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_PRIMARY);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);

	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_PRIMARY);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Valid: One primary plane */
	vkms_config_destroy_plane(plane_cfg);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Invalid: Multiple cursor planes */
	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_CURSOR);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);

	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_CURSOR);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Valid: One primary and one cursor plane */
	vkms_config_destroy_plane(plane_cfg);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Invalid: Second CRTC without primary plane */
	crtc_cfg = vkms_config_add_crtc(config);
	encoder_cfg = vkms_config_add_encoder(config);
	err = vkms_config_encoder_attach_crtc(encoder_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Valid: Second CRTC with a primary plane */
	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_PRIMARY);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_plane_possible_crtcs(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	plane_cfg = list_first_entry(&config->planes, typeof(*plane_cfg), link);
	crtc_cfg = list_first_entry(&config->crtcs, typeof(*crtc_cfg), link);

	/* Invalid: Primary plane without a possible CRTC */
	vkms_config_plane_detach_crtc(plane_cfg, crtc_cfg);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_crtc_number(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_crtc *crtc_cfg;
	int n;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* Invalid: No CRTCs */
	crtc_cfg = list_first_entry(&config->crtcs, typeof(*crtc_cfg), link);
	vkms_config_destroy_crtc(config, crtc_cfg);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Invalid: Too many CRTCs */
	for (n = 0; n <= 32; n++)
		vkms_config_add_crtc(config);

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_encoder_number(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_encoder *encoder_cfg;
	int n;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* Invalid: No encoders */
	encoder_cfg = list_first_entry(&config->encoders, typeof(*encoder_cfg), link);
	vkms_config_destroy_encoder(config, encoder_cfg);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Invalid: Too many encoders */
	for (n = 0; n <= 32; n++)
		vkms_config_add_encoder(config);

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_encoder_possible_crtcs(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg;
	struct vkms_config_crtc *crtc_cfg1, *crtc_cfg2;
	struct vkms_config_encoder *encoder_cfg;
	int err;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	crtc_cfg1 = list_first_entry(&config->crtcs, typeof(*crtc_cfg1), link);

	/* Invalid: Encoder without a possible CRTC */
	encoder_cfg = vkms_config_add_encoder(config);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Valid: Second CRTC with shared encoder */
	crtc_cfg2 = vkms_config_add_crtc(config);

	plane_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(plane_cfg, DRM_PLANE_TYPE_PRIMARY);
	err = vkms_config_plane_attach_crtc(plane_cfg, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	err = vkms_config_encoder_attach_crtc(encoder_cfg, crtc_cfg1);
	KUNIT_EXPECT_EQ(test, err, 0);

	err = vkms_config_encoder_attach_crtc(encoder_cfg, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Invalid: Second CRTC without encoders */
	vkms_config_encoder_detach_crtc(encoder_cfg, crtc_cfg2);
	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	/* Valid: First CRTC with 2 possible encoder */
	vkms_config_destroy_plane(plane_cfg);
	vkms_config_destroy_crtc(config, crtc_cfg2);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_valid_connector_number(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_connector *connector_cfg;
	int n;

	config = vkms_config_default_create(false, false, false);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	/* Valid: No connectors */
	connector_cfg = list_first_entry(&config->connectors, typeof(*connector_cfg), link);
	vkms_config_destroy_connector(connector_cfg);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Valid: Only a disabled connector */
	connector_cfg = vkms_config_add_connector(config);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Valid: The connector is enabled */
	vkms_config_connector_set_enabled(connector_cfg, true);
	KUNIT_EXPECT_TRUE(test, vkms_config_is_valid(config));

	/* Invalid: Too many connectors */
	for (n = 0; n <= 32; n++) {
		connector_cfg = vkms_config_add_connector(config);
		vkms_config_connector_set_enabled(connector_cfg, true);
	}

	KUNIT_EXPECT_FALSE(test, vkms_config_is_valid(config));

	vkms_config_destroy(config);
}

static void vkms_config_test_plane_attach_crtc(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *overlay_cfg;
	struct vkms_config_plane *primary_cfg;
	struct vkms_config_plane *cursor_cfg;
	struct vkms_config_crtc *crtc_cfg;
	int err;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	overlay_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(overlay_cfg, DRM_PLANE_TYPE_OVERLAY);
	primary_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(primary_cfg, DRM_PLANE_TYPE_PRIMARY);
	cursor_cfg = vkms_config_add_plane(config);
	vkms_config_plane_set_type(cursor_cfg, DRM_PLANE_TYPE_CURSOR);

	crtc_cfg = vkms_config_add_crtc(config);

	/* No primary or cursor planes */
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_primary_plane(config, crtc_cfg));
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_cursor_plane(config, crtc_cfg));

	/* Overlay plane, but no primary or cursor planes */
	err = vkms_config_plane_attach_crtc(overlay_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_primary_plane(config, crtc_cfg));
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_cursor_plane(config, crtc_cfg));

	/* Primary plane, attaching it twice must fail */
	err = vkms_config_plane_attach_crtc(primary_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	err = vkms_config_plane_attach_crtc(primary_cfg, crtc_cfg);
	KUNIT_EXPECT_NE(test, err, 0);
	KUNIT_EXPECT_PTR_EQ(test,
			    vkms_config_crtc_primary_plane(config, crtc_cfg),
			    primary_cfg);
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_cursor_plane(config, crtc_cfg));

	/* Primary and cursor planes */
	err = vkms_config_plane_attach_crtc(cursor_cfg, crtc_cfg);
	KUNIT_EXPECT_EQ(test, err, 0);
	KUNIT_EXPECT_PTR_EQ(test,
			    vkms_config_crtc_primary_plane(config, crtc_cfg),
			    primary_cfg);
	KUNIT_EXPECT_PTR_EQ(test,
			    vkms_config_crtc_cursor_plane(config, crtc_cfg),
			    cursor_cfg);

	/* Detach primary and destroy cursor plane */
	vkms_config_plane_detach_crtc(overlay_cfg, crtc_cfg);
	vkms_config_plane_detach_crtc(primary_cfg, crtc_cfg);
	vkms_config_destroy_plane(cursor_cfg);
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_primary_plane(config, crtc_cfg));
	KUNIT_EXPECT_NULL(test, vkms_config_crtc_cursor_plane(config, crtc_cfg));

	vkms_config_destroy(config);
}

static void vkms_config_test_plane_get_possible_crtcs(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_plane *plane_cfg1, *plane_cfg2;
	struct vkms_config_crtc *crtc_cfg1, *crtc_cfg2;
	struct vkms_config_crtc **array;
	size_t length;
	int err;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	plane_cfg1 = vkms_config_add_plane(config);
	plane_cfg2 = vkms_config_add_plane(config);
	crtc_cfg1 = vkms_config_add_crtc(config);
	crtc_cfg2 = vkms_config_add_crtc(config);

	/* No possible CRTCs */
	array = vkms_config_plane_get_possible_crtcs(plane_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	array = vkms_config_plane_get_possible_crtcs(plane_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	/* Plane 1 attached to CRTC 1 and 2 */
	err = vkms_config_plane_attach_crtc(plane_cfg1, crtc_cfg1);
	KUNIT_EXPECT_EQ(test, err, 0);
	err = vkms_config_plane_attach_crtc(plane_cfg1, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	array = vkms_config_plane_get_possible_crtcs(plane_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], crtc_cfg2);
	kfree(array);

	array = vkms_config_plane_get_possible_crtcs(plane_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	/* Plane 1 attached to CRTC 1 and plane 2 to CRTC 2 */
	vkms_config_plane_detach_crtc(plane_cfg1, crtc_cfg2);

	array = vkms_config_plane_get_possible_crtcs(plane_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	kfree(array);

	err = vkms_config_plane_attach_crtc(plane_cfg2, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	array = vkms_config_plane_get_possible_crtcs(plane_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg2);
	kfree(array);

	vkms_config_destroy(config);
}

static void vkms_config_test_encoder_get_possible_crtcs(struct kunit *test)
{
	struct vkms_config *config;
	struct vkms_config_encoder *encoder_cfg1, *encoder_cfg2;
	struct vkms_config_crtc *crtc_cfg1, *crtc_cfg2;
	struct vkms_config_crtc **array;
	size_t length;
	int err;

	config = vkms_config_create("test");
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, config);

	encoder_cfg1 = vkms_config_add_encoder(config);
	encoder_cfg2 = vkms_config_add_encoder(config);
	crtc_cfg1 = vkms_config_add_crtc(config);
	crtc_cfg2 = vkms_config_add_crtc(config);

	/* No possible CRTCs */
	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	/* Encoder 1 attached to CRTC 1 and 2 */
	err = vkms_config_encoder_attach_crtc(encoder_cfg1, crtc_cfg1);
	KUNIT_EXPECT_EQ(test, err, 0);
	err = vkms_config_encoder_attach_crtc(encoder_cfg1, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 2);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	KUNIT_ASSERT_PTR_EQ(test, array[1], crtc_cfg2);
	kfree(array);

	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 0);
	KUNIT_ASSERT_NULL(test, array);

	/* Encoder 1 attached to CRTC 1 and encoder 2 to CRTC 2 */
	vkms_config_encoder_detach_crtc(encoder_cfg1, crtc_cfg2);

	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg1, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg1);
	kfree(array);

	err = vkms_config_encoder_attach_crtc(encoder_cfg2, crtc_cfg2);
	KUNIT_EXPECT_EQ(test, err, 0);

	array = vkms_config_encoder_get_possible_crtcs(encoder_cfg2, &length);
	KUNIT_ASSERT_EQ(test, length, 1);
	KUNIT_ASSERT_PTR_EQ(test, array[0], crtc_cfg2);
	kfree(array);

	vkms_config_destroy(config);
}

static struct kunit_case vkms_config_test_cases[] = {
	KUNIT_CASE(vkms_config_test_empty_config),
	KUNIT_CASE_PARAM(vkms_config_test_default_config,
			 default_config_gen_params),
	KUNIT_CASE(vkms_config_test_get_planes),
	KUNIT_CASE(vkms_config_test_get_crtcs),
	KUNIT_CASE(vkms_config_test_get_encoders),
	KUNIT_CASE(vkms_config_test_get_connectors),
	KUNIT_CASE(vkms_config_test_valid_plane_number),
	KUNIT_CASE(vkms_config_test_valid_plane_type),
	KUNIT_CASE(vkms_config_test_valid_plane_possible_crtcs),
	KUNIT_CASE(vkms_config_test_valid_crtc_number),
	KUNIT_CASE(vkms_config_test_valid_encoder_number),
	KUNIT_CASE(vkms_config_test_valid_encoder_possible_crtcs),
	KUNIT_CASE(vkms_config_test_valid_connector_number),
	KUNIT_CASE(vkms_config_test_plane_attach_crtc),
	KUNIT_CASE(vkms_config_test_plane_get_possible_crtcs),
	KUNIT_CASE(vkms_config_test_encoder_get_possible_crtcs),
	{}
};

static struct kunit_suite vkms_config_test_suite = {
	.name = "vkms-config",
	.test_cases = vkms_config_test_cases,
};

kunit_test_suite(vkms_config_test_suite);

MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Kunit test for vkms config utility");
