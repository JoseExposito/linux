**Follow on...** [![Twitter](.github/images/twitter.png "Twitter")](https://twitter.com/Jose__Exposito) <a href="https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=FT2KS37PVG8PU&currency_code=EUR&source=url"><img align="right" src="https://www.paypalobjects.com/en_US/i/btn/btn_donate_LG.gif"></a>

<br/>
<img src=".github/images/tux.png" align="right" />

# My Linux kernel source tree

Welcome to my fork of the Linux kernel.

Here I work on my patches. I'm a newbie, so don't expect great changes, just humble contributions.


## Patches

| Branch | Target | Details | Status |
| - | - | - | - |
| [patch-magic-mouse-cleanup-v1](https://github.com/JoseExposito/linux/tree/patch-magic-mouse-cleanup-v1) | [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | Magic Mouse/Trackpad driver cleanup | ⏳ |
| [patch-magic-mouse-high-resolution-scroll-v1](https://github.com/JoseExposito/linux/tree/patch-magic-mouse-high-resolution-scroll-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Emulate high-resolution scroll on the Magic Mouse for a smoother scrolling experience | 🔀 |
| [patch-magic-trackpad-crash-disconnecting-usb](https://github.com/JoseExposito/linux/tree/patch-magic-trackpad-crash-disconnecting-usb) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fix NULL pointer reference when disconnecting the Magic Trackpad 2 from the USB port | 🔀 |

🔨 Under development &nbsp;|&nbsp; ⏳ Waiting for review &nbsp;|&nbsp; 🔀 Merged upstream



## Branches

### Upstream

Upstream code is saved in `upstream-*` branches to easily rebase and generate patches:

| Branch | Remote | Pull command | 
| - | - | - |
| [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | GitHub fork | Use GitHub's UI |
| [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid master |
| [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid for-next |

Depending on the subsystem, the maintainer could have its own tree. For example, to modify the Magic
Mouse/Trackpad driver:

```bash
$ ./scripts/get_maintainer.pl -f drivers/hid/hid-magicmouse.c
   Jiri Kosina <jikos@kernel.org> (maintainer:HID CORE LAYER)
   [...]
```

Check the [MAINTAINERS](https://github.com/torvalds/linux/blob/master/MAINTAINERS) file and clone
the right tree:

```
    USB HID/HIDBP DRIVERS (USB KEYBOARDS, MICE, REMOTE CONTROLS, ...)
    M:	Jiri Kosina <jikos@kernel.org>
    M:	Benjamin Tissoires <benjamin.tissoires@redhat.com>
    L:	linux-usb@vger.kernel.org
    S:	Maintained
    T:	git git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git
```

And create a branch to be able to generate patches:

```bash
$ git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git
$ git fetch hid
$ git checkout -b upstream-hid-master --track hid/master
$ git pull --rebase hid master
$ git push --set-upstream origin upstream-hid-master
```


### Config

My personal working configs are save in `config-*` branches:

| Branch | Kernel version | OS |
| - | - | - |
| [config-5.13.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.13.0-elementaryOS6) | 5.13.0 | elementary OS 6 |
