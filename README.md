# Config

My configuration to build the Linux kernel 6.17.0 on QEMU (Fedora 42).

DO NOT USE IT, this config is for my hardware and your kernel won't boot with it.


## Steps to generate it

First, plug in all your hardware and connect your Bluetooth devices to make sure all relevant
modules are loaded, then:

```bash
$ cd <kernel dir>
$ make localmodconfig
```
