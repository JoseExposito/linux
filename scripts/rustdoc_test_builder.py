#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""rustdoc_test_builder - Test builder for `rustdoc`-generated tests.
"""

import json
import pathlib
import re
import sys

RUST_DIR = pathlib.Path("rust")
TESTS_DIR = RUST_DIR / "test" / "doctests" / "kernel"

# `[^\s]*` removes the prefix (e.g. `_doctest_main_`) plus any
# leading path (for `O=` builds).
TEST_NAME_RE = re.compile(r"fn [^\s]*rust_kernel_([a-zA-Z0-9_]+)\(\)")

def main():
    content = sys.stdin.read()
    matches = TEST_NAME_RE.findall(content)
    if len(matches) == 0:
        raise Exception("No test name found.")
    if len(matches) > 1:
        raise Exception("More than one test name found.")

    test_name = f"rust_kernel_doctest_{matches[0]}"

    # Qualify `Result` to avoid the collision with our own `Result`
    # coming from the prelude.
    test_body = content.replace(
        f'rust_kernel_{matches[0]}() -> Result<(), impl core::fmt::Debug> {{',
        f'rust_kernel_{matches[0]}() -> core::result::Result<(), impl core::fmt::Debug> {{',
    )

    with open(TESTS_DIR / f"{test_name}.json", "w") as fd:
        json.dump({
            "name": test_name,
            "body": test_body,
        }, fd, sort_keys=True, indent=4)

if __name__ == "__main__":
    main()
