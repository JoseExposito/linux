// SPDX-License-Identifier: GPL-2.0

#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/uaccess.h>

void rust_helper_BUG(void)
{
	BUG();
}

int rust_helper_access_ok(const void __user *addr, unsigned long n)
{
	return access_ok(addr, n);
}

unsigned long rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return copy_from_user(to, from, n);
}

unsigned long rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return copy_to_user(to, from, n);
}

// See https://github.com/rust-lang/rust-bindgen/issues/1671
static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
	"size_t must match uintptr_t, what architecture is this??");
