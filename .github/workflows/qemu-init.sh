#!/bin/sh

busybox insmod rust_example_3.ko my_i32=345543
busybox insmod rust_example_4.ko my_i32=456654
busybox  rmmod rust_example_3.ko
busybox  rmmod rust_example_4.ko

busybox reboot -f
