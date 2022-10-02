// SPDX-License-Identifier: GPL-2.0+

/*
 *  HID driver for UC-Logic devices not fully compliant with HID standard
 *
 *  Copyright (c) 2022 José Expósito <jose.exposito89@gmail.com>
 */

#include <kunit/test.h>
#include "./hid-uclogic-params.h"

#define MAX_EVENT_SIZE 12

struct uclogic_filter_raw_event_test {
	u8 event[MAX_EVENT_SIZE];
	size_t size;
	bool expected;
};

static struct uclogic_filter_raw_event_test filter_events[] = {
	{
		.event = { 0xA1, 0xB2, 0xC3, 0xD4 },
		.size = 4,
	},
	{
		.event = { 0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A },
		.size = 6,
	},
};

static struct uclogic_filter_raw_event_test test_events[] = {
	{
		.event = { 0xA1, 0xB2, 0xC3, 0xD4 },
		.size = 4,
		.expected = true,
	},
	{
		.event = { 0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A },
		.size = 6,
		.expected = true,
	},
	{
		.event = { 0xA1, 0xB2, 0xC3 },
		.size = 3,
		.expected = false,
	},
	{
		.event = { 0xA1, 0xB2, 0xC3, 0xD4, 0x00 },
		.size = 5,
		.expected = false,
	},
	{
		.event = { 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x1F },
		.size = 6,
		.expected = false,
	},
};

static void uclogic_filter_event_test(struct kunit *test)
{
	struct uclogic_params p = {0, };
	struct uclogic_filter_raw_event *filter;
	bool res;
	int n;

	/* Initialize the list of events to ignore */
	p.filter_events = kunit_kzalloc(test, sizeof(*p.filter_events), GFP_KERNEL);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, p.filter_events);
	INIT_LIST_HEAD(&p.filter_events->list);

	for (n = 0; n < ARRAY_SIZE(filter_events); n++) {
		filter = kunit_kzalloc(test, sizeof(*filter), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filter);

		filter->size = filter_events[n].size;
		filter->event = kunit_kzalloc(test, filter->size, GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filter->event);
		memcpy(filter->event, &filter_events[n].event[0], filter->size);

		list_add_tail(&filter->list, &p.filter_events->list);
	}

	/* Test uclogic_filter_event() */
	for (n = 0; n < ARRAY_SIZE(test_events); n++) {
		res = uclogic_filter_event(&p, &test_events[n].event[0],
					   test_events[n].size);
		KUNIT_ASSERT_EQ(test, res, test_events[n].expected);
	}
}

static struct kunit_case hid_uclogic_core_test_cases[] = {
	KUNIT_CASE(uclogic_filter_event_test),
	{}
};

static struct kunit_suite hid_uclogic_core_test_suite = {
	.name = "hid_uclogic_core_test",
	.test_cases = hid_uclogic_core_test_cases,
};

kunit_test_suite(hid_uclogic_core_test_suite);

MODULE_DESCRIPTION("KUnit tests for the UC-Logic driver");
MODULE_LICENSE("GPL");
MODULE_AUTHOR("José Expósito <jose.exposito89@gmail.com>");
