// SPDX-License-Identifier: GPL-2.0

#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/uaccess.h>

void rust_helper_BUG(void)
{
	BUG();
}
EXPORT_SYMBOL(rust_helper_BUG);

int rust_helper_access_ok(const void __user *addr, unsigned long n)
{
	return access_ok(addr, n);
}
EXPORT_SYMBOL(rust_helper_access_ok);

// See https://github.com/rust-lang/rust-bindgen/issues/1671
static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
	"size_t must match uintptr_t, what architecture is this??");
