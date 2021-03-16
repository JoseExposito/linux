#!/bin/sh

busybox insmod rust_example_3.ko my_i32=345543 my_str=🦀mod
busybox insmod rust_example_4.ko my_i32=456654 my_usize=84 my_array=1,2,3
busybox  rmmod rust_example_3.ko
busybox  rmmod rust_example_4.ko

busybox reboot -f
