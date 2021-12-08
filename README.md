**Follow on...** [![Twitter](.github/images/twitter.png "Twitter")](https://twitter.com/Jose__Exposito) <a href="https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=FT2KS37PVG8PU&currency_code=EUR&source=url"><img align="right" src="https://www.paypalobjects.com/en_US/i/btn/btn_donate_LG.gif"></a>

<br/>
<img src=".github/images/tux.png" align="right" />

# My Linux kernel source tree

Welcome to my fork of the Linux kernel.

Here I work on my patches. I'm a newbie, so don't expect great changes, just humble contributions.


## Patches

| Branch | Target | Details | Status |
| - | - | - | - |
| [patch-coverity-1484720](https://github.com/JoseExposito/linux/tree/patch-coverity-1484720) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1484720  | ⏳ |
| [patch-clk-mt8192-leaks](https://github.com/JoseExposito/linux/tree/patch-clk-mt8192-leaks) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1491825  | ⏳ |
| [patch-coverity-1492899](https://github.com/JoseExposito/linux/tree/patch-coverity-1492899) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1492899  | ⏳ |
| [patch-coverity-1493352](https://github.com/JoseExposito/linux/tree/patch-coverity-1493352) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1493352  | ⏳ |
| [patch-coverity-1493934](https://github.com/JoseExposito/linux/tree/patch-coverity-1493934) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1493934  | ⏳ |
| [patch-coverity-1494000](https://github.com/JoseExposito/linux/tree/coverity-1494000) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1494000  | ⏳ |
| [patch-flex-array-prestera-v1](https://github.com/JoseExposito/linux/tree/patch-flex-array-prestera-v1) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | [KSPP #78](https://github.com/KSPP/linux/issues/78)  | ⏳ |
| [patch-buttonpad-single-button-v1](https://github.com/JoseExposito/linux/tree/patch-buttonpad-single-button-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Clear extra button on buttonpads  | ⏳ |
| [patch-drm-docs-ttm-acronym](https://github.com/JoseExposito/linux/tree/patch-drm-docs-ttm-acronym) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | Translation Table ~~Maps~~ Manager | ⏳ |
| [patch-hid-magicmouse-usb-battery-v1](https://github.com/JoseExposito/linux/tree/patch-hid-magicmouse-usb-battery-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Mouse/Trackpad 2 battery when connected over USB | 🔀 |
| [patch-hid-apple-usb-battery-v1](https://github.com/JoseExposito/linux/tree/patch-hid-apple-usb-battery-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Keyboard battery when connected over USB | 🔀 |
| [patch-uperfect-y-sticky-fingers-v1](https://github.com/JoseExposito/linux/tree/patch-uperfect-y-sticky-fingers-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Disable sticky fingers for UPERFECT Y | 🔀 |
| [patch-intel-hid-deny-list-v1](https://github.com/JoseExposito/linux/tree/patch-intel-hid-deny-list-v1) | [upstream-pdx86-for-next](https://github.com/JoseExposito/linux/tree/upstream-pdx86-for-next) | libinput [issue 662](https://gitlab.freedesktop.org/libinput/libinput/-/issues/662) | 🔀 |
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
| [upstream-linux-next](https://github.com/JoseExposito/linux/tree/upstream-linux-next) | git remote add linux-next https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git | git pull --rebase linux-next master |
| [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid master |
| [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid for-next |
| [upstream-pdx86-master](https://github.com/JoseExposito/linux/tree/upstream-pdx86-master) | git remote add pdx86 git://git.kernel.org/pub/scm/linux/kernel/git/pdx86/platform-drivers-x86.git | git pull --rebase pdx86 master |
| [upstream-pdx86-for-next](https://github.com/JoseExposito/linux/tree/upstream-pdx86-for-next) | git remote add pdx86 git://git.kernel.org/pub/scm/linux/kernel/git/pdx86/platform-drivers-x86.git | git pull --rebase pdx86 for-next |
| [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | git remote add drm git://anongit.freedesktop.org/drm/drm | git pull --rebase drm drm-fixes |

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
| [config-5.14.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.14.0-elementaryOS6) | 5.14.0 | elementary OS 6 |
