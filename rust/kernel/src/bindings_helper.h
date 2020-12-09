/* SPDX-License-Identifier: GPL-2.0 */

#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/slab.h>
#include <linux/sysctl.h>
#include <linux/uaccess.h>
#include <linux/version.h>

// bindgen gets confused at certain things
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;
