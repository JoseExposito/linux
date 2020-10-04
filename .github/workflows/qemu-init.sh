#!/bin/sh

if [ -f rust_example.ko ]; then
    busybox insmod rust_example.ko my_i32=123321
    busybox rmmod rust_example.ko
fi

busybox reboot -f
