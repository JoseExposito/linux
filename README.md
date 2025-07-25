**Follow on...** [![Twitter](.github/images/twitter.png "Twitter")](https://twitter.com/Jose__Exposito) <a href="https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=FT2KS37PVG8PU&currency_code=EUR&source=url"><img align="right" src="https://www.paypalobjects.com/en_US/i/btn/btn_donate_LG.gif"></a>

<br/>
<img src=".github/images/tux.png" align="right" />

# My Linux kernel source tree

Welcome to my fork of the Linux kernel.


## Patches

| Branch | Target | Details | Status |
| - | - | - | - |
| [patch-kunit-drm-format-helper-endian-fixes](https://github.com/JoseExposito/linux/tree/patch-kunit-drm-format-helper-endian-fixes) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Fix endian issues in `drm_test_fb_xrgb8888_to_xrgb2101010()` | ⏳ |
| [patch-vkms-configfs-connector-hot-add-remove](https://github.com/JoseExposito/linux/tree/patch-vkms-configfs-connector-hot-add-remove) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Testing with connector hot-add/remove | 🔨 |
| [patch-vkms-configfs](https://github.com/JoseExposito/linux/tree/patch-vkms-configfs) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Allow to configure VKMS using ConfigFS | ⏳ |
| [patch-vkms-config](https://github.com/JoseExposito/linux/tree/patch-vkms-config) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Allow to configure VKMS in preparation for ConfigFS | 🔀 |
| [patch-vkms-init-error-double-free](https://github.com/JoseExposito/linux/tree/patch-vkms-init-error-double-free) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Fix use after free and double free on VKMS init error | 🔀 |
| [patch-vkms-drop-drm_crtc_cleanup](https://github.com/JoseExposito/linux/tree/patch-vkms-drop-drm_crtc_cleanup) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Drop unnecessary call to drm_crtc_cleanup() in VKMS| 🔀 |
| [patch-drmm-connector-docs](https://github.com/JoseExposito/linux/tree/patch-drmm-connector-docs) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Document the destroy hook in drmm init functions | 🔀 |
| [patch-amd-display-bounds-checking](https://github.com/JoseExposito/linux/tree/patch-amd-display-bounds-checking) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Add bounds checking in various amd/display `create_stream_encoder()` functions | ⏳ |
| [patch-sparse-le-warnings](https://github.com/JoseExposito/linux/tree/patch-sparse-le-warnings) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | Fix cpu_to_le16/le16_to_cpu warnings | 🔀 |
| [patch-uclogic-multiple-modules-warning](https://github.com/JoseExposito/linux/tree/patch-uclogic-multiple-modules-warning) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fix warning `hid-uclogic-[rdesc/params].o is added to multiple modules: hid-uclogic hid-uclogic-test` | 🔀 |
| [patch-logitech-dj-memory-leak](https://github.com/JoseExposito/linux/tree/patch-logitech-dj-memory-leak) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fix memory leak in `logi_dj_recv_switch_to_dj_mode()` | 🔀 |
| [patch-huion-touch-strip-usage-rx-ry](https://github.com/JoseExposito/linux/tree/patch-huion-touch-strip-usage-rx-ry) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | [libinput #989:](https://gitlab.freedesktop.org/libinput/libinput/-/issues/989) Use Rx/Ry to report touch strip events | 🔀 |
| [patch-uclogic-20-buttons](https://github.com/JoseExposito/linux/tree/patch-uclogic-20-buttons) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | [libinput #989:](https://gitlab.freedesktop.org/libinput/libinput/-/issues/989) Support HUION tablets with up to 20 buttons | 🔀 |
| [patch-libwacom-fw-name](https://github.com/JoseExposito/linux/tree/patch-libwacom-fw-name) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | [libwacom #610:](https://github.com/linuxwacom/libwacom/issues/610) Expose the device firmware name to match devices in user-space | 🔀 |
| [patch-rust-kunit-support](https://github.com/JoseExposito/linux/tree/patch-rust-kunit-support) | [upstream-rust](https://github.com/JoseExposito/linux/tree/upstream-rust) | Allow to run unit tests using KUnit with a user-space like syntax | 🔀 |
| [patch-uclogic-tests-use-KUNIT_EXPECT_MEMEQ](https://github.com/JoseExposito/linux/tree/patch-uclogic-tests-use-KUNIT_EXPECT_MEMEQ) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Use KUNIT_EXPECT_MEMEQ in the HID subsystem tests | 🔀 |
| [patch-xppen-deco-01-v2](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-01-v2) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | XP-PEN Deco 01 v2 drawing tablet support ([libinput #839](https://gitlab.freedesktop.org/libinput/libinput/-/issues/839)) | 🔀 |
| [patch-xppen-deco-pro-mw](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-mw) | [patch-xppen-deco-pro-sw](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-sw) | XP-PEN Deco Pro MW drawing tablet support | 🔀 |
| [patch-xppen-deco-pro-sw](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-sw) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | XP-PEN Deco Pro SW drawing tablet support | 🔀 |
| [patch-rust-linked-list-examples](https://github.com/JoseExposito/linux/tree/patch-rust-linked-list-examples) | [upstream-rust](https://github.com/JoseExposito/linux/tree/upstream-rust) | Rust: Improve linked_list docs and tests | ⏳ |
| [patch-rust-kunitconfig](https://github.com/JoseExposito/linux/tree/patch-rust-kunitconfig) | [upstream-rust](https://github.com/JoseExposito/linux/tree/upstream-rust) | Rust: Add rust/.kunitconfig and unit test docs | ⏳ |
| [patch-fix-asus-exportbook-p2451fa-trackpoint](https://github.com/JoseExposito/linux/tree/patch-fix-asus-exportbook-p2451fa-trackpoint) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | [libinput #825:](https://gitlab.freedesktop.org/libinput/libinput/-/issues/825) Fix Asus ExpertBook P2 P2451FA trackpoint | 🔀 |
| [patch-fix-hid-sony-warning](https://github.com/JoseExposito/linux/tree/patch-fix-hid-sony-warning) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | HID: Fix Sony compiler warning | 🔀 |
| [patch-hp-envy-x360-ignore-battery](https://github.com/JoseExposito/linux/tree/patch-hp-envy-x360-ignore-battery) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | [libinput #823:](https://gitlab.freedesktop.org/libinput/libinput/-/issues/823) Ignore HP Envy x360 eu0009nv stylus battery | 🔀 |
| [patch-uclogic-force-hidinput](https://github.com/JoseExposito/linux/tree/patch-uclogic-force-hidinput) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | UCLogic: Force input quirk for digitizers | 🔀 |
| [patch-hid-kunit-tests-prefix](https://github.com/JoseExposito/linux/tree/patch-hid-kunit-tests-prefix) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | UCLogic: Standardize test name prefix | 🔀 |
| [patch-uclogic-fix-big-endian-frame-template](https://github.com/JoseExposito/linux/tree/patch-uclogic-fix-big-endian-frame-template) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | UCLogic: Fix button template on big endian architectures | 🔀 |
| [patch-vc4_hdmi_reset_link-null-dereference](https://github.com/JoseExposito/linux/tree/patch-vc4_hdmi_reset_link-null-dereference) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | DRM: VC4: Fix NULL pointer dereference | 🔀 |
| [patch-battery-charge-status](https://github.com/JoseExposito/linux/tree/patch-battery-charge-status) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | HID core: Report battery charging/discharging status | 🔀 |
| [patch-magicmouse-double-report-fix](https://github.com/JoseExposito/linux/tree/patch-magicmouse-double-report-fix) | [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | [libinput #811:](https://gitlab.freedesktop.org/libinput/libinput/-/issues/811) Fix Magic Trackpad sending multiple middle clicks | 🔀 |
| [patch-starfive-drm-more-cleanup](https://github.com/JoseExposito/linux/tree/patch-starfive-drm-more-cleanup) | [upstream-starfive-visionfive](https://github.com/JoseExposito/linux/tree/upstream-starfive-visionfive) | StarFive VisionFive DRM driver more clean ups  | 🔀 |
| [patch-starfive-drm-cleanup](https://github.com/JoseExposito/linux/tree/patch-starfive-drm-cleanup) | [upstream-starfive-visionfive](https://github.com/JoseExposito/linux/tree/upstream-starfive-visionfive) | StarFive VisionFive DRM driver clean ups  | 🔀 |
| [patch-xppen-deco-pro-lw](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-lw) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | XP-PEN Deco Pro LW drawing tablet support | 🔀 |
| [patch-kunit-drm-format-helper](https://github.com/JoseExposito/linux/tree/patch-kunit-drm-format-helper) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | KUnit tests for RGB888, XRGB2101010 and GRAY8 | 🔀 |
| [patch-uclogic-digitalizer-suffix](https://github.com/JoseExposito/linux/tree/patch-uclogic-digitalizer-suffix) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | UCLogic: Fix digitalizer suffix | 🔀 |
| [patch-uclogic_rdesc_template_apply-sparse-warning](https://github.com/JoseExposito/linux/tree/patch-uclogic_rdesc_template_apply-sparse-warning) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | UCLogic: Fix Sparse warning | 🔀 |
| [patch-drm-format-helper-rgb565-kunit](https://github.com/JoseExposito/linux/tree/patch-drm-format-helper-rgb565-kunit) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | KUnit tests for drm_format_helper (DRM_FORMAT_RGB565) | 🔀 |
| [patch-drm-rect-init-helper](https://github.com/JoseExposito/linux/tree/patch-drm-rect-init-helper) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | DRM: Add DRM_RECT_INIT helper macro | 🔀 |
| [patch-drm-kunit-docs](https://github.com/JoseExposito/linux/tree/patch-drm-kunit-docs) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | DRM: Add docs for the KUnit tests | 🔀 |
| [patch-drm-format-helper-kunit](https://github.com/JoseExposito/linux/tree/patch-drm-format-helper-kunit) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | KUnit tests for drm_format_helper (DRM_FORMAT_RGB332) | 🔀 |
| [patch-xppen-deco-mini4](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-mini4) | [patch-xppen-deco-l](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-l) | XP-PEN Deco Mini 4 drawing tablet support | 🔨 |
| [patch-parblo-a610-pro](https://github.com/JoseExposito/linux/tree/patch-parblo-a610-pro) | [patch-xppen-deco-pro-s](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-s) | Parblo A610 PRO drawing tablet support | 🔀 |
| [patch-xppen-deco-pro-s](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-pro-s) | [patch-xppen-deco-l](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-l) | XP-PEN Deco Pro S drawing tablet support | 🔀 |
| [patch-xppen-deco-l](https://github.com/JoseExposito/linux/tree/patch-xppen-deco-l) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | XP-PEN Deco L drawing tablet support | 🔀 |
| [patch-drm_detect_hdmi_monitor-to-is_hdmi](https://github.com/JoseExposito/linux/tree/patch-drm_detect_hdmi_monitor-to-is_hdmi) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | DRM: Replace drm_detect_hdmi_monitor() with drm_display_info.is_hdmi | 🔀 |
| [patch-vc4-drm_detect_hdmi_monitor-to-is_hdmi](https://github.com/JoseExposito/linux/tree/patch-vc4-drm_detect_hdmi_monitor-to-is_hdmi) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | DRM: VC4: Replace drm_detect_hdmi_monitor() with drm_display_info.is_hdmi | 🔀 |
| [patch-drm_panel_bridge_add](https://github.com/JoseExposito/linux/tree/patch-drm_panel_bridge_add) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | DRM: Migrate from `drm_of_find_panel_or_bridge` + `drm_panel_bridge_add` to `devm_drm_of_get_bridge` | 🔀 |
| [patch-parblo-a610-plus-v2](https://github.com/JoseExposito/linux/tree/patch-parblo-a610-plus-v2) | [patch-digimend-parblo-patches-mailing-list](https://github.com/JoseExposito/linux/tree/patch-digimend-parblo-patches-mailing-list) | Parblo A610 plus v2 drawing tablet support | 🔨 |
| [patch-digimend-parblo-patches-mailing-list](https://github.com/JoseExposito/linux/tree/patch-digimend-parblo-patches-mailing-list) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Version of [patch-digimend-parblo-patches](https://github.com/JoseExposito/linux/tree/patch-digimend-parblo-patches) to be upstreamed, including `checkpatch` fixes and review comments | 🔀 |
| [patch-magic-keyboard-alu-and-2015-fn-key-mapping](https://github.com/JoseExposito/linux/tree/patch-magic-keyboard-alu-and-2015-fn-key-mapping) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Magic Keyboard first generation and 2015 function key mapping | 🔀 |
| [patch-magic-keyboard-2021-fingerprint-fn-key-mapping](https://github.com/JoseExposito/linux/tree/patch-magic-keyboard-2021-fingerprint-fn-key-mapping) | [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | Add funtion key mapping for the Magic Keyboard 2021 with fingerprint reader and/or numpad | 🔀 |
| [patch-magic-keyboard-2021-fingerprint-usb-batery](https://github.com/JoseExposito/linux/tree/patch-magic-keyboard-2021-fingerprint-usb-baterty) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Keyboard 2021 with fingerprint reader battery when connected over USB | 🔀 |
| [patch-magic-keyboard-2021-usb-batery](https://github.com/JoseExposito/linux/tree/patch-magic-keyboard-2021-usb-baterty) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Keyboard 2021 battery when connected over USB | 🔀 |
| [patch-vkms-zpos-plane-prop](https://github.com/JoseExposito/linux/tree/patch-vkms-zpos-plane-prop) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | VKMS: Set zpos plane property | ⏳ |
| [patch-drm-missing-format-mod-supported](https://github.com/JoseExposito/linux/tree/patch-drm-missing-format-mod-supported) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | DRM: Avoid exposing a bogus IN_FORMATS if format_mod_supported is not implemented | 🔀 |
| [patch-vkms-add-linear-mod-v1](https://github.com/JoseExposito/linux/tree/patch-vkms-add-linear-mod-v1) | [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | VKMS: Set IN_FORMATS | ⏳ |
| [patch-support-magic-trackpad-2021-v1](https://github.com/JoseExposito/linux/tree/patch-support-magic-trackpad-2021-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Magic Trackpad 2021 (v3) support | 🔀 |
| [patch-vkms-multiple-overlay-planes-v1](https://github.com/JoseExposito/linux/tree/patch-vkms-multiple-overlay-planes-v1) | [upstream-drm-misc-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-fixes) | VKMS: Add support for multiple overlay planes | 🔀 |
| [patch-flex-array-prestera-v1](https://github.com/JoseExposito/linux/tree/patch-flex-array-prestera-v1) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | [KSPP #78](https://github.com/KSPP/linux/issues/78)  | 🔀 |
| [patch-buttonpad-single-button-v1](https://github.com/JoseExposito/linux/tree/patch-buttonpad-single-button-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Clear extra button on buttonpads  | 🔀 |
| [patch-drm-docs-ttm-acronym](https://github.com/JoseExposito/linux/tree/patch-drm-docs-ttm-acronym) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | Translation Table ~~Maps~~ Manager | 🔀 |
| [patch-hid-magicmouse-usb-battery-v1](https://github.com/JoseExposito/linux/tree/patch-hid-magicmouse-usb-battery-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Mouse/Trackpad 2 battery when connected over USB | 🔀 |
| [patch-hid-apple-usb-battery-v1](https://github.com/JoseExposito/linux/tree/patch-hid-apple-usb-battery-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fetch Magic Keyboard battery when connected over USB | 🔀 |
| [patch-uperfect-y-sticky-fingers-v1](https://github.com/JoseExposito/linux/tree/patch-uperfect-y-sticky-fingers-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Disable sticky fingers for UPERFECT Y | 🔀 |
| [patch-intel-hid-deny-list-v1](https://github.com/JoseExposito/linux/tree/patch-intel-hid-deny-list-v1) | [upstream-pdx86-for-next](https://github.com/JoseExposito/linux/tree/upstream-pdx86-for-next) | libinput [issue 662](https://gitlab.freedesktop.org/libinput/libinput/-/issues/662) | 🔀 |
| [patch-magic-mouse-high-resolution-scroll-v1](https://github.com/JoseExposito/linux/tree/patch-magic-mouse-high-resolution-scroll-v1) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Emulate high-resolution scroll on the Magic Mouse for a smoother scrolling experience | 🔀 |
| [patch-magic-trackpad-crash-disconnecting-usb](https://github.com/JoseExposito/linux/tree/patch-magic-trackpad-crash-disconnecting-usb) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Fix NULL pointer reference when disconnecting the Magic Trackpad 2 from the USB port | 🔀 |
| [patch-coverity-1443831-1443827-1443804-1443763](https://github.com/JoseExposito/linux/tree/patch-coverity-1443831-1443827-1443804-1443804) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Coverity #1443831, #1443827, #1443804 and 1443763  | 🔀 |
| [patch-coverity-1493892](https://github.com/JoseExposito/linux/tree/patch-coverity-1493892) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1493892  | 🔀 |
| [patch-coverity-1474582](https://github.com/JoseExposito/linux/tree/patch-coverity-1474582) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1474582  | 🔀 |
| [patch-coverity-1443943](https://github.com/JoseExposito/linux/tree/patch-coverity-1443943) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1443943  | 🔀 |
| [patch-coverity-1475685](https://github.com/JoseExposito/linux/tree/patch-coverity-1475685) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1475685  | 🔀 |
| [patch-coverity-1484720](https://github.com/JoseExposito/linux/tree/patch-coverity-1484720) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1484720  | 🔀 |
| [patch-coverity-1474639](https://github.com/JoseExposito/linux/tree/patch-coverity-1474639) | [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | Coverity #1474639  | 🔀 |
| [patch-clk-mt8192-leaks](https://github.com/JoseExposito/linux/tree/patch-clk-mt8192-leaks) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1491825  | 🔀 |
| [patch-coverity-1492899](https://github.com/JoseExposito/linux/tree/patch-coverity-1492899) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1492899  | 🔀 |
| [patch-coverity-1493352](https://github.com/JoseExposito/linux/tree/patch-coverity-1493352) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1493352  | 🔀 |
| [patch-coverity-1493934](https://github.com/JoseExposito/linux/tree/patch-coverity-1493934) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1493934  | 🔀 |
| [patch-coverity-1494000](https://github.com/JoseExposito/linux/tree/coverity-1494000) | [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | Coverity #1494000  | 🔀 |
| [patch-coverity-1493909](https://github.com/JoseExposito/linux/tree/patch-coverity-1493909) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | Coverity #1493909  | 🔀 |
| [patch-coverity-1493866](https://github.com/JoseExposito/linux/tree/patch-coverity-1493866) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | Coverity #1493866  | 🔀 |
| [patch-coverity-1493860](https://github.com/JoseExposito/linux/tree/patch-coverity-1493860) | [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | Coverity #1493860  | 🔀 |

🔨 Under development &nbsp;|&nbsp; ⏳ Waiting for review &nbsp;|&nbsp; 🔀 Merged upstream



## Branches

### Upstream

Upstream code is saved in `upstream-*` branches to easily rebase and generate patches:

| Branch | Remote | Pull command | 
| - | - | - |
| [upstream-torvalds-master](https://github.com/JoseExposito/linux/tree/upstream-torvalds-master) | git remote add torvalds git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git | git pull --rebase torvalds master |
| [upstream-linux-next](https://github.com/JoseExposito/linux/tree/upstream-linux-next) | git remote add linux-next https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git | git pull --rebase linux-next master |
| [upstream-hid-master](https://github.com/JoseExposito/linux/tree/upstream-hid-master) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid master |
| [upstream-hid-for-next](https://github.com/JoseExposito/linux/tree/upstream-hid-for-next) | git remote add hid git://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git | git pull --rebase hid for-next |
| [upstream-pdx86-master](https://github.com/JoseExposito/linux/tree/upstream-pdx86-master) | git remote add pdx86 git://git.kernel.org/pub/scm/linux/kernel/git/pdx86/platform-drivers-x86.git | git pull --rebase pdx86 master |
| [upstream-pdx86-for-next](https://github.com/JoseExposito/linux/tree/upstream-pdx86-for-next) | git remote add pdx86 git://git.kernel.org/pub/scm/linux/kernel/git/pdx86/platform-drivers-x86.git | git pull --rebase pdx86 for-next |
| [upstream-kselftest-master](https://github.com/JoseExposito/linux/tree/upstream-kselftest-master) | git remote add kselftest https://git.kernel.org/pub/scm/linux/kernel/git/shuah/linux-kselftest.git/ | git pull --rebase kselftest master |
| [upstream-drm-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-fixes) | git remote add drm git://anongit.freedesktop.org/drm/drm | git pull --rebase drm drm-fixes |
| [upstream-drm-misc-fixes](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-fixes) | git remote add drm-misc git://anongit.freedesktop.org/drm/drm-misc | git pull --rebase drm-misc drm-misc-fixes |
| [upstream-drm-misc-next](https://github.com/JoseExposito/linux/tree/upstream-drm-misc-next) | git remote add drm-misc git://anongit.freedesktop.org/drm/drm-misc | git pull --rebase drm-misc drm-misc-next |
| [upstream-rpi-5.10.y](https://github.com/JoseExposito/linux/tree/upstream-rpi-5.10.y) | git remote add rpi https://github.com/raspberrypi/linux.git | git pull --rebase rpi rpi-5.10.y |
| [upstream-rpi-5.15.y](https://github.com/JoseExposito/linux/tree/upstream-rpi-5.15.y) | git remote add rpi https://github.com/raspberrypi/linux.git | git pull --rebase rpi rpi-5.15.y |
| [upstream-rpi-6.1.y](https://github.com/JoseExposito/linux/tree/upstream-rpi-6.1.y) | git remote add rpi https://github.com/raspberrypi/linux.git | git pull --rebase rpi rpi-6.1.y |
| [upstream-starfive-visionfive](https://github.com/JoseExposito/linux/tree/upstream-starfive-visionfive) | git remote add starfive https://github.com/starfive-tech/linux.git | git pull --rebase starfive visionfive |
| [upstream-rust](https://github.com/JoseExposito/linux/tree/upstream-rust) | git remote add rust https://github.com/Rust-for-Linux/linux.git | git pull --rebase rust rust |

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
| [config-5.16.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.16.0-elementaryOS6) | 5.16.0 | elementary OS 6 |
| [config-5.17.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.17.0-elementaryOS6) | 5.17.0 | elementary OS 6 |
| [config-5.18.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.18.0-elementaryOS6) | 5.18.0 | elementary OS 6 |
| [config-5.19.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-5.19.0-elementaryOS6) | 5.19.0 | elementary OS 6 |
| [config-6.0.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-6.0.0-elementaryOS6) | 6.0.0 | elementary OS 6 |
| [config-6.1.0-elementaryOS6](https://github.com/JoseExposito/linux/tree/config-6.1.0-elementaryOS6) | 6.1.0 | elementary OS 6 |
| [config-6.1.0-fedora-37](https://github.com/JoseExposito/linux/tree/config-6.1.0-fedora-37) | 6.1.0 | Fedora 37 |
| [config-6.3.0-fedora-38](https://github.com/JoseExposito/linux/tree/config-6.3.0-fedora-38) | 6.3.0 | Fedora 38 |
| [config-6.4.0-fedora-38](https://github.com/JoseExposito/linux/tree/config-6.4.0-fedora-38) | 6.4.0 | Fedora 38 |
| [config-6.5.0-fedora-38](https://github.com/JoseExposito/linux/tree/config-6.5.0-fedora-38) | 6.5.0 | Fedora 38 |
| [config-6.6.0-fedora-38](https://github.com/JoseExposito/linux/tree/config-6.6.0-fedora-38) | 6.6.0 | Fedora 38 |
| [config-6.7.0-fedora-39](https://github.com/JoseExposito/linux/tree/config-6.7.0-fedora-39) | 6.7.0 | Fedora 39 |
| [config-6.8.0-fedora-39](https://github.com/JoseExposito/linux/tree/config-6.8.0-fedora-39) | 6.8.0 | Fedora 39 |
| [config-6.9.0-fedora-39](https://github.com/JoseExposito/linux/tree/config-6.9.0-fedora-39) | 6.9.0 | Fedora 39 |
| [config-6.10.0-fedora-40](https://github.com/JoseExposito/linux/tree/config-6.10.0-fedora-40) | 6.10.0 | Fedora 40 |
| [config-6.11.0-fedora-40](https://github.com/JoseExposito/linux/tree/config-6.11.0-fedora-40) | 6.11.0 | Fedora 40 |
| [config-6.13.0-qemu-fedora-41](https://github.com/JoseExposito/linux/tree/config-6.13.0-qemu-fedora-41) | 6.13.0 | QEMU (Fedora 41) |
| [config-6.14.0-qemu-fedora-41](https://github.com/JoseExposito/linux/tree/config-6.14.0-qemu-fedora-41) | 6.14.0 | QEMU (Fedora 41) |
| [config-6.14.0-qemu-fedora-41](https://github.com/JoseExposito/linux/tree/config-6.16.0-qemu-fedora-42) | 6.16.0 | QEMU (Fedora 42) |
| [config-6.1.0-rust-fedora-37](https://github.com/JoseExposito/linux/tree/config-6.1.0-rust-fedora-37) | 6.1.0 | Rust - Fedora 37 |
| [config-5.10.y-raspberrypi4](https://github.com/JoseExposito/linux/tree/config-5.10.y-raspberrypi4) | 5.10.y | Raspberry Pi 4 |
| [config-5.15.y-raspberrypi4](https://github.com/JoseExposito/linux/tree/config-5.15.y-raspberrypi4) | 5.15.y | Raspberry Pi 4 |
| [config-6.0.0-starfive-visionfive](https://github.com/JoseExposito/linux/tree/config-6.0.0-starfive-visionfive) | 6.0.0 | StarFive VisionFive |
| [config-6.1.0-starfive-visionfive](https://github.com/JoseExposito/linux/tree/config-6.1.0-starfive-visionfive) | 6.1.0 | StarFive VisionFive |
