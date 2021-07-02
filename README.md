# Config

My configuration to build the Linux kernel 5.19.0 on elementary OS 6 (based on Ubuntu 20.04.1 LTS).

DO NOT USE IT, this config is for my hardware and your kernel won't boot with it.


## Steps to generate it

First, plug in all your hardware and connect your Bluetooth devices to make sure all relevant
modules are loaded, then:

```bash
$ cd <kernel dir>
$ make localmodconfig
```

The usual `cp /boot/config-$(uname -r) .config` didn't work.
