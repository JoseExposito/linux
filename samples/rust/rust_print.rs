// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {
    type: RustPrint,
    name: b"rust_print",
    author: b"Rust for Linux Contributors",
    description: b"Rust printing macros sample",
    license: b"GPL v2",
    params: {
    },
}

struct RustPrint;

impl KernelModule for RustPrint {
    fn init() -> KernelResult<Self> {
        info!("Rust printing macros sample (init)");

        emerg!("Emergency message (level 0) with newline w/o args");
        alert!("Alert message (level 1) with newline w/o args");
        crit!("Critical message (level 2) with newline w/o args");
        err!("Error message (level 3) with newline w/o args");
        warn!("Warning message (level 4) with newline w/o args");
        notice!("Notice message (level 5) with newline w/o args");
        info!("Info message (level 6) with newline w/o args");

        pr_info!("A line that");
        pr_cont!(" is continued");
        cont!(" with newline w/o args");

        pr_emerg!("Emergency message (level 0) w/o newline w/o args\n");
        pr_alert!("Alert message (level 1) w/o newline w/o args\n");
        pr_crit!("Critical message (level 2) w/o newline w/o args\n");
        pr_err!("Error message (level 3) w/o newline w/o args\n");
        pr_warn!("Warning message (level 4) w/o newline w/o args\n");
        pr_notice!("Notice message (level 5) w/o newline w/o args\n");
        pr_info!("Info message (level 6) w/o newline w/o args\n");

        pr_info!("A line that");
        pr_cont!(" is continued");
        pr_cont!(" w/o newline w/o args\n");

        emerg!(
            "{} message (level {}) with newline with args",
            "Emergency",
            0
        );
        alert!("{} message (level {}) with newline with args", "Alert", 1);
        crit!(
            "{} message (level {}) with newline with args",
            "Critical",
            2
        );
        err!("{} message (level {}) with newline with args", "Error", 3);
        warn!("{} message (level {}) with newline with args", "Warning", 4);
        notice!("{} message (level {}) with newline with args", "Notice", 5);
        info!("{} message (level {}) with newline with args", "Info", 6);

        pr_info!("A {} that", "line");
        pr_cont!(" is {}", "continued");
        cont!(" with {} with args", "newline");

        pr_emerg!(
            "{} message (level {}) w/o newline with args\n",
            "Emergency",
            0
        );
        pr_alert!("{} message (level {}) w/o newline with args\n", "Alert", 1);
        pr_crit!(
            "{} message (level {}) w/o newline with args\n",
            "Critical",
            2
        );
        pr_err!("{} message (level {}) w/o newline with args\n", "Error", 3);
        pr_warn!(
            "{} message (level {}) w/o newline with args\n",
            "Warning",
            4
        );
        pr_notice!("{} message (level {}) w/o newline with args\n", "Notice", 5);
        pr_info!("{} message (level {}) w/o newline with args\n", "Info", 6);

        pr_info!("A {} that", "line");
        pr_cont!(" is {}", "continued");
        pr_cont!(" w/o {} with args\n", "newline");

        Ok(RustPrint)
    }
}

impl Drop for RustPrint {
    fn drop(&mut self) {
        info!("Rust printing macros sample (exit)");
    }
}
