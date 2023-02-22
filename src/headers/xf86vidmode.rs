// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::*;
use crate::lazy::*;
use std::os::raw::*;

/// The name of the `XF86VidMode` extension.
pub const XCB_XF86VIDMODE_NAME: &[u8] = b"XFree86-VidModeExtension";

/// The name of the `XF86VidMode` extension.
pub const XCB_XF86VIDMODE_NAME_STR: &str = "XFree86-VidModeExtension";


/// The `XF86VidMode::SYNCRANGE` type.
pub type xcb_xf86vidmode_syncrange_t = u32;

/// An iterator over `XF86VidMode::SYNCRANGE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_syncrange_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xf86vidmode_syncrange_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xf86vidmode_syncrange_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::DOTCLOCK` type.
pub type xcb_xf86vidmode_dotclock_t = u32;

/// An iterator over `XF86VidMode::DOTCLOCK` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_dotclock_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xf86vidmode_dotclock_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xf86vidmode_dotclock_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::ModeFlag` enum.
///
/// This enum has the following variants:
///
/// - [`XF86VidMode::ModeFlag::Positive_HSync`](XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_H_SYNC)
/// - [`XF86VidMode::ModeFlag::Negative_HSync`](XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_H_SYNC)
/// - [`XF86VidMode::ModeFlag::Positive_VSync`](XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_V_SYNC)
/// - [`XF86VidMode::ModeFlag::Negative_VSync`](XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_V_SYNC)
/// - [`XF86VidMode::ModeFlag::Interlace`](XCB_XF86VIDMODE_MODE_FLAG_INTERLACE)
/// - [`XF86VidMode::ModeFlag::Composite_Sync`](XCB_XF86VIDMODE_MODE_FLAG_COMPOSITE_SYNC)
/// - [`XF86VidMode::ModeFlag::Positive_CSync`](XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_C_SYNC)
/// - [`XF86VidMode::ModeFlag::Negative_CSync`](XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_C_SYNC)
/// - [`XF86VidMode::ModeFlag::HSkew`](XCB_XF86VIDMODE_MODE_FLAG_H_SKEW)
/// - [`XF86VidMode::ModeFlag::Broadcast`](XCB_XF86VIDMODE_MODE_FLAG_BROADCAST)
/// - [`XF86VidMode::ModeFlag::Pixmux`](XCB_XF86VIDMODE_MODE_FLAG_PIXMUX)
/// - [`XF86VidMode::ModeFlag::Double_Clock`](XCB_XF86VIDMODE_MODE_FLAG_DOUBLE_CLOCK)
/// - [`XF86VidMode::ModeFlag::Half_Clock`](XCB_XF86VIDMODE_MODE_FLAG_HALF_CLOCK)
pub type xcb_xf86vidmode_mode_flag_t = u32;
/// The `XF86VidMode::ModeFlag::Positive_HSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_H_SYNC: xcb_xf86vidmode_mode_flag_t = 1;
/// The `XF86VidMode::ModeFlag::Negative_HSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_H_SYNC: xcb_xf86vidmode_mode_flag_t = 2;
/// The `XF86VidMode::ModeFlag::Positive_VSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_V_SYNC: xcb_xf86vidmode_mode_flag_t = 4;
/// The `XF86VidMode::ModeFlag::Negative_VSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_V_SYNC: xcb_xf86vidmode_mode_flag_t = 8;
/// The `XF86VidMode::ModeFlag::Interlace` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_INTERLACE: xcb_xf86vidmode_mode_flag_t = 16;
/// The `XF86VidMode::ModeFlag::Composite_Sync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_COMPOSITE_SYNC: xcb_xf86vidmode_mode_flag_t = 32;
/// The `XF86VidMode::ModeFlag::Positive_CSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_C_SYNC: xcb_xf86vidmode_mode_flag_t = 64;
/// The `XF86VidMode::ModeFlag::Negative_CSync` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_C_SYNC: xcb_xf86vidmode_mode_flag_t = 128;
/// The `XF86VidMode::ModeFlag::HSkew` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_H_SKEW: xcb_xf86vidmode_mode_flag_t = 256;
/// The `XF86VidMode::ModeFlag::Broadcast` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_BROADCAST: xcb_xf86vidmode_mode_flag_t = 512;
/// The `XF86VidMode::ModeFlag::Pixmux` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_PIXMUX: xcb_xf86vidmode_mode_flag_t = 1024;
/// The `XF86VidMode::ModeFlag::Double_Clock` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_DOUBLE_CLOCK: xcb_xf86vidmode_mode_flag_t = 2048;
/// The `XF86VidMode::ModeFlag::Half_Clock` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_mode_flag_t`].
pub const XCB_XF86VIDMODE_MODE_FLAG_HALF_CLOCK: xcb_xf86vidmode_mode_flag_t = 4096;

/// The `XF86VidMode::ClockFlag` enum.
///
/// This enum has the following variants:
///
/// - [`XF86VidMode::ClockFlag::Programable`](XCB_XF86VIDMODE_CLOCK_FLAG_PROGRAMABLE)
pub type xcb_xf86vidmode_clock_flag_t = u32;
/// The `XF86VidMode::ClockFlag::Programable` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_clock_flag_t`].
pub const XCB_XF86VIDMODE_CLOCK_FLAG_PROGRAMABLE: xcb_xf86vidmode_clock_flag_t = 1;

/// The `XF86VidMode::Permission` enum.
///
/// This enum has the following variants:
///
/// - [`XF86VidMode::Permission::Read`](XCB_XF86VIDMODE_PERMISSION_READ)
/// - [`XF86VidMode::Permission::Write`](XCB_XF86VIDMODE_PERMISSION_WRITE)
pub type xcb_xf86vidmode_permission_t = u32;
/// The `XF86VidMode::Permission::Read` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_permission_t`].
pub const XCB_XF86VIDMODE_PERMISSION_READ: xcb_xf86vidmode_permission_t = 1;
/// The `XF86VidMode::Permission::Write` enum variant.
///
/// This is a variant of [`xcb_xf86vidmode_permission_t`].
pub const XCB_XF86VIDMODE_PERMISSION_WRITE: xcb_xf86vidmode_permission_t = 2;

/// The `XF86VidMode::ModeInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_mode_info_t {
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u32,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 4],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_mode_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `XF86VidMode::ModeInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_mode_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xf86vidmode_mode_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xf86vidmode_mode_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_query_version_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_query_version_request_t`].
pub const XCB_XF86VIDMODE_QUERY_VERSION: u8 = 0i32 as u8;

/// The `XF86VidMode::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xf86vidmode_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_xf86vidmode_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetModeLine` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_mode_line_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_mode_line_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_mode_line_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_mode_line_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetModeLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_mode_line_request_t`].
pub const XCB_XF86VIDMODE_GET_MODE_LINE: u8 = 1i32 as u8;

/// The `XF86VidMode::GetModeLine` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_mode_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetModeLine` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad1: [u8; 2],
    pub flags: u32,
    pub pad2: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_get_mode_line_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ModModeLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_mod_mode_line_request_t`].
pub const XCB_XF86VIDMODE_MOD_MODE_LINE: u8 = 2i32 as u8;

/// The `XF86VidMode::ModModeLine` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_mod_mode_line_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_mod_mode_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SwitchMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_switch_mode_request_t`].
pub const XCB_XF86VIDMODE_SWITCH_MODE: u8 = 3i32 as u8;

/// The `XF86VidMode::SwitchMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_switch_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub zoom: u16,
}

impl Default for xcb_xf86vidmode_switch_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetMonitor` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_monitor_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_monitor_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_monitor_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_monitor_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetMonitor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_monitor_request_t`].
pub const XCB_XF86VIDMODE_GET_MONITOR: u8 = 4i32 as u8;

/// The `XF86VidMode::GetMonitor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_monitor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetMonitor` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `hsync`
/// - `vsync`
/// - `vendor`
/// - `alignment_pad`
/// - `model`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub vendor_length: u8,
    pub model_length: u8,
    pub num_hsync: u8,
    pub num_vsync: u8,
    pub pad1: [u8; 20],
}

impl Default for xcb_xf86vidmode_get_monitor_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::LockModeSwitch` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_lock_mode_switch_request_t`].
pub const XCB_XF86VIDMODE_LOCK_MODE_SWITCH: u8 = 5i32 as u8;

/// The `XF86VidMode::LockModeSwitch` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_lock_mode_switch_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub lock: u16,
}

impl Default for xcb_xf86vidmode_lock_mode_switch_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetAllModeLines` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_all_mode_lines_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_all_mode_lines_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_all_mode_lines_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_all_mode_lines_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetAllModeLines` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_all_mode_lines_request_t`].
pub const XCB_XF86VIDMODE_GET_ALL_MODE_LINES: u8 = 6i32 as u8;

/// The `XF86VidMode::GetAllModeLines` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_all_mode_lines_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetAllModeLines` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `modeinfo`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub modecount: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xf86vidmode_get_all_mode_lines_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::AddModeLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_add_mode_line_request_t`].
pub const XCB_XF86VIDMODE_ADD_MODE_LINE: u8 = 7i32 as u8;

/// The `XF86VidMode::AddModeLine` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_add_mode_line_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
    pub after_dotclock: xcb_xf86vidmode_dotclock_t,
    pub after_hdisplay: u16,
    pub after_hsyncstart: u16,
    pub after_hsyncend: u16,
    pub after_htotal: u16,
    pub after_hskew: u16,
    pub after_vdisplay: u16,
    pub after_vsyncstart: u16,
    pub after_vsyncend: u16,
    pub after_vtotal: u16,
    pub pad2: [u8; 2],
    pub after_flags: u32,
    pub pad3: [u8; 12],
}

impl Default for xcb_xf86vidmode_add_mode_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::DeleteModeLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_delete_mode_line_request_t`].
pub const XCB_XF86VIDMODE_DELETE_MODE_LINE: u8 = 8i32 as u8;

/// The `XF86VidMode::DeleteModeLine` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_delete_mode_line_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_delete_mode_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::ValidateModeLine` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_validate_mode_line_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_validate_mode_line_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_validate_mode_line_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_validate_mode_line_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ValidateModeLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_validate_mode_line_request_t`].
pub const XCB_XF86VIDMODE_VALIDATE_MODE_LINE: u8 = 9i32 as u8;

/// The `XF86VidMode::ValidateModeLine` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_validate_mode_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::ValidateModeLine` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xf86vidmode_validate_mode_line_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SwitchToMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_switch_to_mode_request_t`].
pub const XCB_XF86VIDMODE_SWITCH_TO_MODE: u8 = 10i32 as u8;

/// The `XF86VidMode::SwitchToMode` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `private`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_switch_to_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub dotclock: xcb_xf86vidmode_dotclock_t,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 12],
    pub privsize: u32,
}

impl Default for xcb_xf86vidmode_switch_to_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetViewPort` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_view_port_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_view_port_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_view_port_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_view_port_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetViewPort` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_view_port_request_t`].
pub const XCB_XF86VIDMODE_GET_VIEW_PORT: u8 = 11i32 as u8;

/// The `XF86VidMode::GetViewPort` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_view_port_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetViewPort` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: u32,
    pub y: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_xf86vidmode_get_view_port_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SetViewPort` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_set_view_port_request_t`].
pub const XCB_XF86VIDMODE_SET_VIEW_PORT: u8 = 12i32 as u8;

/// The `XF86VidMode::SetViewPort` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_set_view_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
    pub x: u32,
    pub y: u32,
}

impl Default for xcb_xf86vidmode_set_view_port_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetDotClocks` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_dot_clocks_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_dot_clocks_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_dot_clocks_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_dot_clocks_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetDotClocks` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_dot_clocks_request_t`].
pub const XCB_XF86VIDMODE_GET_DOT_CLOCKS: u8 = 13i32 as u8;

/// The `XF86VidMode::GetDotClocks` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_dot_clocks_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetDotClocks` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `clock`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub flags: u32,
    pub clocks: u32,
    pub maxclocks: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xf86vidmode_get_dot_clocks_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SetClientVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_set_client_version_request_t`].
pub const XCB_XF86VIDMODE_SET_CLIENT_VERSION: u8 = 14i32 as u8;

/// The `XF86VidMode::SetClientVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_set_client_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major: u16,
    pub minor: u16,
}

impl Default for xcb_xf86vidmode_set_client_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SetGamma` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_set_gamma_request_t`].
pub const XCB_XF86VIDMODE_SET_GAMMA: u8 = 15i32 as u8;

/// The `XF86VidMode::SetGamma` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_set_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xf86vidmode_set_gamma_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetGamma` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_gamma_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_gamma_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_gamma_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_gamma_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetGamma` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_gamma_request_t`].
pub const XCB_XF86VIDMODE_GET_GAMMA: u8 = 16i32 as u8;

/// The `XF86VidMode::GetGamma` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 26],
}

impl Default for xcb_xf86vidmode_get_gamma_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetGamma` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xf86vidmode_get_gamma_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetGammaRamp` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_gamma_ramp_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_gamma_ramp_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_gamma_ramp_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetGammaRamp` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_gamma_ramp_request_t`].
pub const XCB_XF86VIDMODE_GET_GAMMA_RAMP: u8 = 17i32 as u8;

/// The `XF86VidMode::GetGammaRamp` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub size: u16,
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetGammaRamp` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `red`
/// - `green`
/// - `blue`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::SetGammaRamp` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_set_gamma_ramp_request_t`].
pub const XCB_XF86VIDMODE_SET_GAMMA_RAMP: u8 = 18i32 as u8;

/// The `XF86VidMode::SetGammaRamp` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `red`
/// - `green`
/// - `blue`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_set_gamma_ramp_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub size: u16,
}

impl Default for xcb_xf86vidmode_set_gamma_ramp_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetGammaRampSize` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_gamma_ramp_size_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_gamma_ramp_size_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_gamma_ramp_size_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetGammaRampSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_gamma_ramp_size_request_t`].
pub const XCB_XF86VIDMODE_GET_GAMMA_RAMP_SIZE: u8 = 19i32 as u8;

/// The `XF86VidMode::GetGammaRampSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetGammaRampSize` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_xf86vidmode_get_gamma_ramp_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86VidMode::GetPermissions` request.
///
/// Pass this cookie to [`xcb_xf86vidmode_get_permissions_reply`] to retrieve the reply.
///
/// [`xcb_xf86vidmode_get_permissions_reply`]: XcbXf86Vidmode::xcb_xf86vidmode_get_permissions_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86vidmode_get_permissions_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::GetPermissions` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86Vidmode::xcb_xf86vidmode_id()`], then the type of the request is
/// [`xcb_xf86vidmode_get_permissions_request_t`].
pub const XCB_XF86VIDMODE_GET_PERMISSIONS: u8 = 20i32 as u8;

/// The `XF86VidMode::GetPermissions` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xf86vidmode_get_permissions_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XF86VidMode::GetPermissions` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub permissions: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xf86vidmode_get_permissions_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::BadClock` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_bad_clock_error_t`].
pub const XCB_XF86VIDMODE_BAD_CLOCK: u8 = 0i32 as u8;

/// The `XF86VidMode::BadClock` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_bad_clock_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_bad_clock_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::BadHTimings` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_bad_h_timings_error_t`].
pub const XCB_XF86VIDMODE_BAD_H_TIMINGS: u8 = 1i32 as u8;

/// The `XF86VidMode::BadHTimings` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_bad_h_timings_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_bad_h_timings_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::BadVTimings` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_bad_v_timings_error_t`].
pub const XCB_XF86VIDMODE_BAD_V_TIMINGS: u8 = 2i32 as u8;

/// The `XF86VidMode::BadVTimings` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_bad_v_timings_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_bad_v_timings_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ModeUnsuitable` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_mode_unsuitable_error_t`].
pub const XCB_XF86VIDMODE_MODE_UNSUITABLE: u8 = 3i32 as u8;

/// The `XF86VidMode::ModeUnsuitable` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_mode_unsuitable_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_mode_unsuitable_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ExtensionDisabled` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_extension_disabled_error_t`].
pub const XCB_XF86VIDMODE_EXTENSION_DISABLED: u8 = 4i32 as u8;

/// The `XF86VidMode::ExtensionDisabled` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_extension_disabled_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_extension_disabled_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ClientNotLocal` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_client_not_local_error_t`].
pub const XCB_XF86VIDMODE_CLIENT_NOT_LOCAL: u8 = 5i32 as u8;

/// The `XF86VidMode::ClientNotLocal` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_client_not_local_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_client_not_local_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86VidMode::ZoomLocked` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xf86vidmode_zoom_locked_error_t`].
pub const XCB_XF86VIDMODE_ZOOM_LOCKED: u8 = 6i32 as u8;

/// The `XF86VidMode::ZoomLocked` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86vidmode_zoom_locked_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_xf86vidmode_zoom_locked_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xf86vidmode")]
pub(crate) struct XcbXf86VidmodeXf86Vidmode {
    xcb_xf86vidmode_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xf86vidmode_syncrange_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_xf86vidmode_syncrange_iterator_t, )>,
    xcb_xf86vidmode_syncrange_end: LazySymbol<unsafe extern "C" fn(i: xcb_xf86vidmode_syncrange_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_dotclock_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_xf86vidmode_dotclock_iterator_t, )>,
    xcb_xf86vidmode_dotclock_end: LazySymbol<unsafe extern "C" fn(i: xcb_xf86vidmode_dotclock_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_mode_info_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_xf86vidmode_mode_info_iterator_t, )>,
    xcb_xf86vidmode_mode_info_end: LazySymbol<unsafe extern "C" fn(i: xcb_xf86vidmode_mode_info_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_query_version: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_xf86vidmode_query_version_cookie_t>,
    xcb_xf86vidmode_query_version_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_xf86vidmode_query_version_cookie_t>,
    xcb_xf86vidmode_query_version_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_query_version_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_query_version_reply_t>,
    xcb_xf86vidmode_get_mode_line_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_get_mode_line: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_mode_line_cookie_t>,
    xcb_xf86vidmode_get_mode_line_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_mode_line_cookie_t>,
    xcb_xf86vidmode_get_mode_line_private: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> *mut u8>,
    xcb_xf86vidmode_get_mode_line_private_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_mode_line_private_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_mode_line_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_mode_line_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_mode_line_reply_t>,
    xcb_xf86vidmode_mod_mode_line_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_mod_mode_line_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_mod_mode_line: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_mod_mode_line_private: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> *mut u8>,
    xcb_xf86vidmode_mod_mode_line_private_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> c_int>,
    xcb_xf86vidmode_mod_mode_line_private_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_switch_mode_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, zoom: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_switch_mode: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, zoom: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_get_monitor_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_get_monitor: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_monitor_cookie_t>,
    xcb_xf86vidmode_get_monitor_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_monitor_cookie_t>,
    xcb_xf86vidmode_get_monitor_hsync: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut xcb_xf86vidmode_syncrange_t>,
    xcb_xf86vidmode_get_monitor_hsync_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_monitor_hsync_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_monitor_vsync: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut xcb_xf86vidmode_syncrange_t>,
    xcb_xf86vidmode_get_monitor_vsync_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_monitor_vsync_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_monitor_vendor: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_char>,
    xcb_xf86vidmode_get_monitor_vendor_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_monitor_vendor_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_monitor_alignment_pad: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_void>,
    xcb_xf86vidmode_get_monitor_alignment_pad_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_monitor_alignment_pad_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_monitor_model: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_char>,
    xcb_xf86vidmode_get_monitor_model_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_monitor_model_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_monitor_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_monitor_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_monitor_reply_t>,
    xcb_xf86vidmode_lock_mode_switch_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, lock: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_lock_mode_switch: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, lock: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_get_all_mode_lines_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_get_all_mode_lines: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t>,
    xcb_xf86vidmode_get_all_mode_lines_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t>,
    xcb_xf86vidmode_get_all_mode_lines_modeinfo: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> *mut xcb_xf86vidmode_mode_info_t>,
    xcb_xf86vidmode_get_all_mode_lines_modeinfo_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> xcb_xf86vidmode_mode_info_iterator_t>,
    xcb_xf86vidmode_get_all_mode_lines_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_all_mode_lines_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_all_mode_lines_reply_t>,
    xcb_xf86vidmode_add_mode_line_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_add_mode_line_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, after_dotclock: xcb_xf86vidmode_dotclock_t, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_add_mode_line: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, after_dotclock: xcb_xf86vidmode_dotclock_t, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_add_mode_line_private: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> *mut u8>,
    xcb_xf86vidmode_add_mode_line_private_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> c_int>,
    xcb_xf86vidmode_add_mode_line_private_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_delete_mode_line_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_delete_mode_line_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_delete_mode_line: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_delete_mode_line_private: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> *mut u8>,
    xcb_xf86vidmode_delete_mode_line_private_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> c_int>,
    xcb_xf86vidmode_delete_mode_line_private_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_validate_mode_line_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_validate_mode_line: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_xf86vidmode_validate_mode_line_cookie_t>,
    xcb_xf86vidmode_validate_mode_line_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_xf86vidmode_validate_mode_line_cookie_t>,
    xcb_xf86vidmode_validate_mode_line_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_validate_mode_line_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_validate_mode_line_reply_t>,
    xcb_xf86vidmode_switch_to_mode_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_switch_to_mode_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_switch_to_mode: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_switch_to_mode_private: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> *mut u8>,
    xcb_xf86vidmode_switch_to_mode_private_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> c_int>,
    xcb_xf86vidmode_switch_to_mode_private_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_view_port: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_view_port_cookie_t>,
    xcb_xf86vidmode_get_view_port_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_view_port_cookie_t>,
    xcb_xf86vidmode_get_view_port_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_view_port_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_view_port_reply_t>,
    xcb_xf86vidmode_set_view_port_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, x: u32, y: u32, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_view_port: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, x: u32, y: u32, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_get_dot_clocks_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_get_dot_clocks: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_dot_clocks_cookie_t>,
    xcb_xf86vidmode_get_dot_clocks_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_dot_clocks_cookie_t>,
    xcb_xf86vidmode_get_dot_clocks_clock: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> *mut u32>,
    xcb_xf86vidmode_get_dot_clocks_clock_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_dot_clocks_clock_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_dot_clocks_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_dot_clocks_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_dot_clocks_reply_t>,
    xcb_xf86vidmode_set_client_version_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, major: u16, minor: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_client_version: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, major: u16, minor: u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_gamma_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, red: u32, green: u32, blue: u32, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_gamma: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, red: u32, green: u32, blue: u32, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_get_gamma: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_cookie_t>,
    xcb_xf86vidmode_get_gamma_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_cookie_t>,
    xcb_xf86vidmode_get_gamma_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_reply_t>,
    xcb_xf86vidmode_get_gamma_ramp_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_get_gamma_ramp: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, size: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t>,
    xcb_xf86vidmode_get_gamma_ramp_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, size: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t>,
    xcb_xf86vidmode_get_gamma_ramp_red: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16>,
    xcb_xf86vidmode_get_gamma_ramp_red_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_gamma_ramp_red_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_gamma_ramp_green: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16>,
    xcb_xf86vidmode_get_gamma_ramp_green_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_gamma_ramp_green_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_gamma_ramp_blue: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16>,
    xcb_xf86vidmode_get_gamma_ramp_blue_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int>,
    xcb_xf86vidmode_get_gamma_ramp_blue_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_gamma_ramp_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_ramp_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_ramp_reply_t>,
    xcb_xf86vidmode_set_gamma_ramp_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_xf86vidmode_set_gamma_ramp_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, size: u16, red: *const u16, green: *const u16, blue: *const u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_gamma_ramp: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, size: u16, red: *const u16, green: *const u16, blue: *const u16, ) -> xcb_void_cookie_t>,
    xcb_xf86vidmode_set_gamma_ramp_red: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16>,
    xcb_xf86vidmode_set_gamma_ramp_red_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int>,
    xcb_xf86vidmode_set_gamma_ramp_red_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_set_gamma_ramp_green: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16>,
    xcb_xf86vidmode_set_gamma_ramp_green_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int>,
    xcb_xf86vidmode_set_gamma_ramp_green_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_set_gamma_ramp_blue: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16>,
    xcb_xf86vidmode_set_gamma_ramp_blue_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int>,
    xcb_xf86vidmode_set_gamma_ramp_blue_end: LazySymbol<unsafe extern "C" fn(r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t>,
    xcb_xf86vidmode_get_gamma_ramp_size: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t>,
    xcb_xf86vidmode_get_gamma_ramp_size_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t>,
    xcb_xf86vidmode_get_gamma_ramp_size_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_ramp_size_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_ramp_size_reply_t>,
    xcb_xf86vidmode_get_permissions: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_permissions_cookie_t>,
    xcb_xf86vidmode_get_permissions_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_permissions_cookie_t>,
    xcb_xf86vidmode_get_permissions_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_permissions_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_permissions_reply_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.xf86vidmode.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self.xf86vidmode.$f.exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xf86vidmode")]
impl XcbXf86Vidmode {
/// The libxcb identifier of the `XF86VidMode` extension.
    #[inline]
    pub fn xcb_xf86vidmode_id(&self, ) -> *mut xcb_extension_t { unsafe { sym!(self, xcb_xf86vidmode_id) } }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_id(&self) -> bool { has_sym!(self, xcb_xf86vidmode_id) }

/// Advances a `xcb_xf86vidmode_syncrange_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_syncrange_next(&self, i: *mut xcb_xf86vidmode_syncrange_iterator_t, ) { sym!(self, xcb_xf86vidmode_syncrange_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_syncrange_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_syncrange_next(&self) -> bool { has_sym!(self, xcb_xf86vidmode_syncrange_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xf86vidmode_syncrange_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_syncrange_end(&self, i: xcb_xf86vidmode_syncrange_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_syncrange_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_syncrange_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_syncrange_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_syncrange_end) }

/// Advances a `xcb_xf86vidmode_dotclock_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_dotclock_next(&self, i: *mut xcb_xf86vidmode_dotclock_iterator_t, ) { sym!(self, xcb_xf86vidmode_dotclock_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_dotclock_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_dotclock_next(&self) -> bool { has_sym!(self, xcb_xf86vidmode_dotclock_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xf86vidmode_dotclock_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_dotclock_end(&self, i: xcb_xf86vidmode_dotclock_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_dotclock_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_dotclock_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_dotclock_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_dotclock_end) }

/// Advances a `xcb_xf86vidmode_mode_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mode_info_next(&self, i: *mut xcb_xf86vidmode_mode_info_iterator_t, ) { sym!(self, xcb_xf86vidmode_mode_info_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mode_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mode_info_next(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mode_info_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xf86vidmode_mode_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mode_info_end(&self, i: xcb_xf86vidmode_mode_info_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_mode_info_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mode_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mode_info_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mode_info_end) }

/// Sends a `XF86VidMode::QueryVersion` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_query_version_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_query_version_reply`]: Self::xcb_xf86vidmode_query_version_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_query_version(&self, c: *mut xcb_connection_t, ) -> xcb_xf86vidmode_query_version_cookie_t { sym!(self, xcb_xf86vidmode_query_version)(c, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_query_version(&self) -> bool { has_sym!(self, xcb_xf86vidmode_query_version) }

/// Sends a `XF86VidMode::QueryVersion` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_query_version_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_query_version_reply`]: Self::xcb_xf86vidmode_query_version_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_query_version_unchecked(&self, c: *mut xcb_connection_t, ) -> xcb_xf86vidmode_query_version_cookie_t { sym!(self, xcb_xf86vidmode_query_version_unchecked)(c, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_query_version_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_query_version_unchecked) }

/// Waits for the reply to a `XF86VidMode::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_query_version_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_query_version_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_query_version_reply_t { sym!(self, xcb_xf86vidmode_query_version_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_query_version_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_query_version_reply) }

/// Computes the size of a `xcb_xf86vidmode_get_mode_line_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_get_mode_line_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_sizeof) }

/// Sends a `XF86VidMode::GetModeLine` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_mode_line_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_mode_line_reply`]: Self::xcb_xf86vidmode_get_mode_line_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_mode_line_cookie_t { sym!(self, xcb_xf86vidmode_get_mode_line)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line) }

/// Sends a `XF86VidMode::GetModeLine` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_mode_line_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_mode_line_reply`]: Self::xcb_xf86vidmode_get_mode_line_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_mode_line_cookie_t { sym!(self, xcb_xf86vidmode_get_mode_line_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_unchecked) }

/// Returns a pointer to the `private` field of a `xcb_xf86vidmode_get_mode_line_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_private(&self, r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> *mut u8 { sym!(self, xcb_xf86vidmode_get_mode_line_private)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_private(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_private) }

/// Returns the number of elements of the `private` field of a `xcb_xf86vidmode_get_mode_line_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_private_length(&self, r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_mode_line_private_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_private_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_private_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `private` field of a `xcb_xf86vidmode_get_mode_line_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_private_end(&self, r: *const xcb_xf86vidmode_get_mode_line_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_mode_line_private_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_private_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_private_end) }

/// Waits for the reply to a `XF86VidMode::GetModeLine` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_mode_line_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_mode_line_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_mode_line_reply_t { sym!(self, xcb_xf86vidmode_get_mode_line_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_mode_line_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_mode_line_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_mode_line_reply) }

/// Computes the size of a `xcb_xf86vidmode_mod_mode_line_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_mod_mode_line_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line_sizeof) }

/// Sends a `XF86VidMode::ModModeLine` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line_checked(&self, c: *mut xcb_connection_t, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_mod_mode_line_checked)(c, screen, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line_checked) }

/// Sends a `XF86VidMode::ModModeLine` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line(&self, c: *mut xcb_connection_t, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_mod_mode_line)(c, screen, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line) }

/// Returns a pointer to the `private` field of a `xcb_xf86vidmode_mod_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line_private(&self, r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> *mut u8 { sym!(self, xcb_xf86vidmode_mod_mode_line_private)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line_private(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line_private) }

/// Returns the number of elements of the `private` field of a `xcb_xf86vidmode_mod_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line_private_length(&self, r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_mod_mode_line_private_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line_private_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line_private_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `private` field of a `xcb_xf86vidmode_mod_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_mod_mode_line_private_end(&self, r: *const xcb_xf86vidmode_mod_mode_line_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_mod_mode_line_private_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_mod_mode_line_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_mod_mode_line_private_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_mod_mode_line_private_end) }

/// Sends a `XF86VidMode::SwitchMode` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_mode_checked(&self, c: *mut xcb_connection_t, screen: u16, zoom: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_switch_mode_checked)(c, screen, zoom, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_mode_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_mode_checked) }

/// Sends a `XF86VidMode::SwitchMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_mode(&self, c: *mut xcb_connection_t, screen: u16, zoom: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_switch_mode)(c, screen, zoom, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_mode(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_mode) }

/// Computes the size of a `xcb_xf86vidmode_get_monitor_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_sizeof) }

/// Sends a `XF86VidMode::GetMonitor` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_monitor_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_monitor_reply`]: Self::xcb_xf86vidmode_get_monitor_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_monitor_cookie_t { sym!(self, xcb_xf86vidmode_get_monitor)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor) }

/// Sends a `XF86VidMode::GetMonitor` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_monitor_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_monitor_reply`]: Self::xcb_xf86vidmode_get_monitor_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_monitor_cookie_t { sym!(self, xcb_xf86vidmode_get_monitor_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_unchecked) }

/// Returns a pointer to the `hsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_hsync(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut xcb_xf86vidmode_syncrange_t { sym!(self, xcb_xf86vidmode_get_monitor_hsync)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_hsync` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_hsync(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_hsync) }

/// Returns the number of elements of the `hsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_hsync_length(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_hsync_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_hsync_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_hsync_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_hsync_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `hsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_hsync_end(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_monitor_hsync_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_hsync_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_hsync_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_hsync_end) }

/// Returns a pointer to the `vsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vsync(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut xcb_xf86vidmode_syncrange_t { sym!(self, xcb_xf86vidmode_get_monitor_vsync)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vsync` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vsync(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vsync) }

/// Returns the number of elements of the `vsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vsync_length(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_vsync_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vsync_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vsync_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vsync_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `vsync` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vsync_end(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_monitor_vsync_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vsync_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vsync_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vsync_end) }

/// Returns a pointer to the `vendor` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vendor(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_char { sym!(self, xcb_xf86vidmode_get_monitor_vendor)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vendor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vendor(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vendor) }

/// Returns the number of elements of the `vendor` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vendor_length(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_vendor_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vendor_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vendor_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vendor_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `vendor` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_vendor_end(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_monitor_vendor_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_vendor_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_vendor_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_vendor_end) }

/// Returns a pointer to the `alignment_pad` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_void { sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_alignment_pad` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_alignment_pad(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad) }

/// Returns the number of elements of the `alignment_pad` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_length(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_alignment_pad_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_alignment_pad_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `alignment_pad` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_end(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_alignment_pad_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_alignment_pad_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_alignment_pad_end) }

/// Returns a pointer to the `model` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_model(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> *mut c_char { sym!(self, xcb_xf86vidmode_get_monitor_model)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_model` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_model(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_model) }

/// Returns the number of elements of the `model` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_model_length(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_monitor_model_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_model_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_model_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_model_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `model` field of a `xcb_xf86vidmode_get_monitor_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_model_end(&self, r: *const xcb_xf86vidmode_get_monitor_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_monitor_model_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_model_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_model_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_model_end) }

/// Waits for the reply to a `XF86VidMode::GetMonitor` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_monitor_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_monitor_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_monitor_reply_t { sym!(self, xcb_xf86vidmode_get_monitor_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_monitor_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_monitor_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_monitor_reply) }

/// Sends a `XF86VidMode::LockModeSwitch` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_lock_mode_switch_checked(&self, c: *mut xcb_connection_t, screen: u16, lock: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_lock_mode_switch_checked)(c, screen, lock, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_lock_mode_switch_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_lock_mode_switch_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_lock_mode_switch_checked) }

/// Sends a `XF86VidMode::LockModeSwitch` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_lock_mode_switch(&self, c: *mut xcb_connection_t, screen: u16, lock: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_lock_mode_switch)(c, screen, lock, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_lock_mode_switch` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_lock_mode_switch(&self) -> bool { has_sym!(self, xcb_xf86vidmode_lock_mode_switch) }

/// Computes the size of a `xcb_xf86vidmode_get_all_mode_lines_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_get_all_mode_lines_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_sizeof) }

/// Sends a `XF86VidMode::GetAllModeLines` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_all_mode_lines_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_all_mode_lines_reply`]: Self::xcb_xf86vidmode_get_all_mode_lines_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t { sym!(self, xcb_xf86vidmode_get_all_mode_lines)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines) }

/// Sends a `XF86VidMode::GetAllModeLines` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_all_mode_lines_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_all_mode_lines_reply`]: Self::xcb_xf86vidmode_get_all_mode_lines_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t { sym!(self, xcb_xf86vidmode_get_all_mode_lines_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_unchecked) }

/// Returns a pointer to the `modeinfo` field of a `xcb_xf86vidmode_get_all_mode_lines_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo(&self, r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> *mut xcb_xf86vidmode_mode_info_t { sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_modeinfo` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_modeinfo(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo) }

/// Returns the number of elements of the `modeinfo` field of a `xcb_xf86vidmode_get_all_mode_lines_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_length(&self, r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_modeinfo_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_modeinfo_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo_length) }

/// Returns an iterator over the elements of the
/// `modeinfo` field of a `xcb_xf86vidmode_get_all_mode_lines_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator(&self, r: *const xcb_xf86vidmode_get_all_mode_lines_reply_t, ) -> xcb_xf86vidmode_mode_info_iterator_t { sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator) }

/// Waits for the reply to a `XF86VidMode::GetAllModeLines` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_all_mode_lines_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_all_mode_lines_reply_t { sym!(self, xcb_xf86vidmode_get_all_mode_lines_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_all_mode_lines_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_all_mode_lines_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_all_mode_lines_reply) }

/// Computes the size of a `xcb_xf86vidmode_add_mode_line_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_add_mode_line_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line_sizeof) }

/// Sends a `XF86VidMode::AddModeLine` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line_checked(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, after_dotclock: xcb_xf86vidmode_dotclock_t, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_add_mode_line_checked)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, after_dotclock, after_hdisplay, after_hsyncstart, after_hsyncend, after_htotal, after_hskew, after_vdisplay, after_vsyncstart, after_vsyncend, after_vtotal, after_flags, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line_checked) }

/// Sends a `XF86VidMode::AddModeLine` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, after_dotclock: xcb_xf86vidmode_dotclock_t, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_add_mode_line)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, after_dotclock, after_hdisplay, after_hsyncstart, after_hsyncend, after_htotal, after_hskew, after_vdisplay, after_vsyncstart, after_vsyncend, after_vtotal, after_flags, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line) }

/// Returns a pointer to the `private` field of a `xcb_xf86vidmode_add_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line_private(&self, r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> *mut u8 { sym!(self, xcb_xf86vidmode_add_mode_line_private)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line_private(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line_private) }

/// Returns the number of elements of the `private` field of a `xcb_xf86vidmode_add_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line_private_length(&self, r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_add_mode_line_private_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line_private_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line_private_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `private` field of a `xcb_xf86vidmode_add_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_add_mode_line_private_end(&self, r: *const xcb_xf86vidmode_add_mode_line_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_add_mode_line_private_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_add_mode_line_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_add_mode_line_private_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_add_mode_line_private_end) }

/// Computes the size of a `xcb_xf86vidmode_delete_mode_line_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_delete_mode_line_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line_sizeof) }

/// Sends a `XF86VidMode::DeleteModeLine` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line_checked(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_delete_mode_line_checked)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line_checked) }

/// Sends a `XF86VidMode::DeleteModeLine` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_delete_mode_line)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line) }

/// Returns a pointer to the `private` field of a `xcb_xf86vidmode_delete_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line_private(&self, r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> *mut u8 { sym!(self, xcb_xf86vidmode_delete_mode_line_private)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line_private(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line_private) }

/// Returns the number of elements of the `private` field of a `xcb_xf86vidmode_delete_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line_private_length(&self, r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_delete_mode_line_private_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line_private_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line_private_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `private` field of a `xcb_xf86vidmode_delete_mode_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_delete_mode_line_private_end(&self, r: *const xcb_xf86vidmode_delete_mode_line_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_delete_mode_line_private_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_delete_mode_line_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_delete_mode_line_private_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_delete_mode_line_private_end) }

/// Computes the size of a `xcb_xf86vidmode_validate_mode_line_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_validate_mode_line_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_validate_mode_line_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_validate_mode_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_validate_mode_line_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_validate_mode_line_sizeof) }

/// Sends a `XF86VidMode::ValidateModeLine` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_validate_mode_line_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_validate_mode_line_reply`]: Self::xcb_xf86vidmode_validate_mode_line_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_validate_mode_line(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_xf86vidmode_validate_mode_line_cookie_t { sym!(self, xcb_xf86vidmode_validate_mode_line)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_validate_mode_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_validate_mode_line(&self) -> bool { has_sym!(self, xcb_xf86vidmode_validate_mode_line) }

/// Sends a `XF86VidMode::ValidateModeLine` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_validate_mode_line_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_validate_mode_line_reply`]: Self::xcb_xf86vidmode_validate_mode_line_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_validate_mode_line_unchecked(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_xf86vidmode_validate_mode_line_cookie_t { sym!(self, xcb_xf86vidmode_validate_mode_line_unchecked)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_validate_mode_line_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_validate_mode_line_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_validate_mode_line_unchecked) }

/// Waits for the reply to a `XF86VidMode::ValidateModeLine` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_validate_mode_line_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_validate_mode_line_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_validate_mode_line_reply_t { sym!(self, xcb_xf86vidmode_validate_mode_line_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_validate_mode_line_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_validate_mode_line_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_validate_mode_line_reply) }

/// Computes the size of a `xcb_xf86vidmode_switch_to_mode_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_switch_to_mode_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode_sizeof) }

/// Sends a `XF86VidMode::SwitchToMode` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode_checked(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_switch_to_mode_checked)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode_checked) }

/// Sends a `XF86VidMode::SwitchToMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode(&self, c: *mut xcb_connection_t, screen: u32, dotclock: xcb_xf86vidmode_dotclock_t, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, privsize: u32, private: *const u8, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_switch_to_mode)(c, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize, private, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode) }

/// Returns a pointer to the `private` field of a `xcb_xf86vidmode_switch_to_mode_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode_private(&self, r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> *mut u8 { sym!(self, xcb_xf86vidmode_switch_to_mode_private)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode_private(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode_private) }

/// Returns the number of elements of the `private` field of a `xcb_xf86vidmode_switch_to_mode_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode_private_length(&self, r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_switch_to_mode_private_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode_private_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode_private_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `private` field of a `xcb_xf86vidmode_switch_to_mode_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_switch_to_mode_private_end(&self, r: *const xcb_xf86vidmode_switch_to_mode_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_switch_to_mode_private_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_switch_to_mode_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_switch_to_mode_private_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_switch_to_mode_private_end) }

/// Sends a `XF86VidMode::GetViewPort` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_view_port_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_view_port_reply`]: Self::xcb_xf86vidmode_get_view_port_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_view_port(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_view_port_cookie_t { sym!(self, xcb_xf86vidmode_get_view_port)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_view_port` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_view_port(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_view_port) }

/// Sends a `XF86VidMode::GetViewPort` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_view_port_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_view_port_reply`]: Self::xcb_xf86vidmode_get_view_port_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_view_port_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_view_port_cookie_t { sym!(self, xcb_xf86vidmode_get_view_port_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_view_port_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_view_port_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_view_port_unchecked) }

/// Waits for the reply to a `XF86VidMode::GetViewPort` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_view_port_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_view_port_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_view_port_reply_t { sym!(self, xcb_xf86vidmode_get_view_port_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_view_port_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_view_port_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_view_port_reply) }

/// Sends a `XF86VidMode::SetViewPort` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_view_port_checked(&self, c: *mut xcb_connection_t, screen: u16, x: u32, y: u32, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_view_port_checked)(c, screen, x, y, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_view_port_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_view_port_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_view_port_checked) }

/// Sends a `XF86VidMode::SetViewPort` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_view_port(&self, c: *mut xcb_connection_t, screen: u16, x: u32, y: u32, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_view_port)(c, screen, x, y, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_view_port` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_view_port(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_view_port) }

/// Computes the size of a `xcb_xf86vidmode_get_dot_clocks_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_get_dot_clocks_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_sizeof) }

/// Sends a `XF86VidMode::GetDotClocks` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_dot_clocks_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_dot_clocks_reply`]: Self::xcb_xf86vidmode_get_dot_clocks_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_dot_clocks_cookie_t { sym!(self, xcb_xf86vidmode_get_dot_clocks)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks) }

/// Sends a `XF86VidMode::GetDotClocks` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_dot_clocks_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_dot_clocks_reply`]: Self::xcb_xf86vidmode_get_dot_clocks_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_dot_clocks_cookie_t { sym!(self, xcb_xf86vidmode_get_dot_clocks_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_unchecked) }

/// Returns a pointer to the `clock` field of a `xcb_xf86vidmode_get_dot_clocks_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock(&self, r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> *mut u32 { sym!(self, xcb_xf86vidmode_get_dot_clocks_clock)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_clock` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_clock(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_clock) }

/// Returns the number of elements of the `clock` field of a `xcb_xf86vidmode_get_dot_clocks_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_length(&self, r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_dot_clocks_clock_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_clock_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_clock_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_clock_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `clock` field of a `xcb_xf86vidmode_get_dot_clocks_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_end(&self, r: *const xcb_xf86vidmode_get_dot_clocks_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_dot_clocks_clock_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_clock_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_clock_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_clock_end) }

/// Waits for the reply to a `XF86VidMode::GetDotClocks` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_dot_clocks_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_dot_clocks_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_dot_clocks_reply_t { sym!(self, xcb_xf86vidmode_get_dot_clocks_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_dot_clocks_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_dot_clocks_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_dot_clocks_reply) }

/// Sends a `XF86VidMode::SetClientVersion` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_client_version_checked(&self, c: *mut xcb_connection_t, major: u16, minor: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_client_version_checked)(c, major, minor, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_client_version_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_client_version_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_client_version_checked) }

/// Sends a `XF86VidMode::SetClientVersion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_client_version(&self, c: *mut xcb_connection_t, major: u16, minor: u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_client_version)(c, major, minor, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_client_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_client_version(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_client_version) }

/// Sends a `XF86VidMode::SetGamma` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_checked(&self, c: *mut xcb_connection_t, screen: u16, red: u32, green: u32, blue: u32, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_gamma_checked)(c, screen, red, green, blue, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_checked) }

/// Sends a `XF86VidMode::SetGamma` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma(&self, c: *mut xcb_connection_t, screen: u16, red: u32, green: u32, blue: u32, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_gamma)(c, screen, red, green, blue, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma) }

/// Sends a `XF86VidMode::GetGamma` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_reply`]: Self::xcb_xf86vidmode_get_gamma_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma) }

/// Sends a `XF86VidMode::GetGamma` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_reply`]: Self::xcb_xf86vidmode_get_gamma_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_unchecked) }

/// Waits for the reply to a `XF86VidMode::GetGamma` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_reply_t { sym!(self, xcb_xf86vidmode_get_gamma_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_reply) }

/// Computes the size of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_get_gamma_ramp_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_sizeof) }

/// Sends a `XF86VidMode::GetGammaRamp` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_ramp_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_ramp_reply`]: Self::xcb_xf86vidmode_get_gamma_ramp_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp(&self, c: *mut xcb_connection_t, screen: u16, size: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp)(c, screen, size, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp) }

/// Sends a `XF86VidMode::GetGammaRamp` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_ramp_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_ramp_reply`]: Self::xcb_xf86vidmode_get_gamma_ramp_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_unchecked(&self, c: *mut xcb_connection_t, screen: u16, size: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_unchecked)(c, screen, size, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_unchecked) }

/// Returns a pointer to the `red` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_get_gamma_ramp_red)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_red` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_red(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_red) }

/// Returns the number of elements of the `red` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_length(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_gamma_ramp_red_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_red_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_red_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_red_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `red` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_end(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_red_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_red_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_red_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_red_end) }

/// Returns a pointer to the `green` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_get_gamma_ramp_green)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_green` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_green(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_green) }

/// Returns the number of elements of the `green` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_length(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_gamma_ramp_green_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_green_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_green_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_green_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `green` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_end(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_green_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_green_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_green_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_green_end) }

/// Returns a pointer to the `blue` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_blue` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_blue(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue) }

/// Returns the number of elements of the `blue` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_length(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> c_int { sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_blue_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_blue_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `blue` field of a `xcb_xf86vidmode_get_gamma_ramp_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_end(&self, r: *const xcb_xf86vidmode_get_gamma_ramp_reply_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_blue_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_blue_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_blue_end) }

/// Waits for the reply to a `XF86VidMode::GetGammaRamp` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_ramp_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_ramp_reply_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_reply) }

/// Computes the size of a `xcb_xf86vidmode_set_gamma_ramp_request_t` object.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_xf86vidmode_set_gamma_ramp_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_sizeof(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_sizeof) }

/// Sends a `XF86VidMode::SetGammaRamp` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_checked(&self, c: *mut xcb_connection_t, screen: u16, size: u16, red: *const u16, green: *const u16, blue: *const u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_gamma_ramp_checked)(c, screen, size, red, green, blue, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_checked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_checked) }

/// Sends a `XF86VidMode::SetGammaRamp` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp(&self, c: *mut xcb_connection_t, screen: u16, size: u16, red: *const u16, green: *const u16, blue: *const u16, ) -> xcb_void_cookie_t { sym!(self, xcb_xf86vidmode_set_gamma_ramp)(c, screen, size, red, green, blue, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp) }

/// Returns a pointer to the `red` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_red(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_set_gamma_ramp_red)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_red` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_red(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_red) }

/// Returns the number of elements of the `red` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_red_length(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_set_gamma_ramp_red_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_red_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_red_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_red_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `red` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_red_end(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_set_gamma_ramp_red_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_red_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_red_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_red_end) }

/// Returns a pointer to the `green` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_green(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_set_gamma_ramp_green)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_green` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_green(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_green) }

/// Returns the number of elements of the `green` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_green_length(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_set_gamma_ramp_green_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_green_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_green_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_green_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `green` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_green_end(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_set_gamma_ramp_green_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_green_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_green_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_green_end) }

/// Returns a pointer to the `blue` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_blue(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> *mut u16 { sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_blue` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_blue(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue) }

/// Returns the number of elements of the `blue` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_blue_length(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> c_int { sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_blue_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_blue_length(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue_length) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of the
/// `blue` field of a `xcb_xf86vidmode_set_gamma_ramp_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_blue_end(&self, r: *const xcb_xf86vidmode_set_gamma_ramp_request_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue_end)(r, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_set_gamma_ramp_blue_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_set_gamma_ramp_blue_end(&self) -> bool { has_sym!(self, xcb_xf86vidmode_set_gamma_ramp_blue_end) }

/// Sends a `XF86VidMode::GetGammaRampSize` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_ramp_size_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_ramp_size_reply`]: Self::xcb_xf86vidmode_get_gamma_ramp_size_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_size)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_size(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_size) }

/// Sends a `XF86VidMode::GetGammaRampSize` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_gamma_ramp_size_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_gamma_ramp_size_reply`]: Self::xcb_xf86vidmode_get_gamma_ramp_size_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_size_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_size_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_size_unchecked) }

/// Waits for the reply to a `XF86VidMode::GetGammaRampSize` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_gamma_ramp_size_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_gamma_ramp_size_reply_t { sym!(self, xcb_xf86vidmode_get_gamma_ramp_size_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_gamma_ramp_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_gamma_ramp_size_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_gamma_ramp_size_reply) }

/// Sends a `XF86VidMode::GetPermissions` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_permissions_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_permissions_reply`]: Self::xcb_xf86vidmode_get_permissions_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_permissions(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_permissions_cookie_t { sym!(self, xcb_xf86vidmode_get_permissions)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_permissions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_permissions(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_permissions) }

/// Sends a `XF86VidMode::GetPermissions` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_xf86vidmode_get_permissions_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_xf86vidmode_get_permissions_reply`]: Self::xcb_xf86vidmode_get_permissions_reply
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_permissions_unchecked(&self, c: *mut xcb_connection_t, screen: u16, ) -> xcb_xf86vidmode_get_permissions_cookie_t { sym!(self, xcb_xf86vidmode_get_permissions_unchecked)(c, screen, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_permissions_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_permissions_unchecked(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_permissions_unchecked) }

/// Waits for the reply to a `XF86VidMode::GetPermissions` request.
    #[inline]
    pub unsafe fn xcb_xf86vidmode_get_permissions_reply(&self, c: *mut xcb_connection_t, cookie: xcb_xf86vidmode_get_permissions_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_xf86vidmode_get_permissions_reply_t { sym!(self, xcb_xf86vidmode_get_permissions_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_xf86vidmode_get_permissions_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86vidmode_get_permissions_reply(&self) -> bool { has_sym!(self, xcb_xf86vidmode_get_permissions_reply) }
}

#[cfg(feature = "xcb_xf86vidmode")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXf86Vidmode::load().unwrap() };
        assert!(lib.has_xcb_xf86vidmode_id());
        assert!(lib.has_xcb_xf86vidmode_syncrange_next());
        assert!(lib.has_xcb_xf86vidmode_syncrange_end());
        assert!(lib.has_xcb_xf86vidmode_dotclock_next());
        assert!(lib.has_xcb_xf86vidmode_dotclock_end());
        assert!(lib.has_xcb_xf86vidmode_mode_info_next());
        assert!(lib.has_xcb_xf86vidmode_mode_info_end());
        assert!(lib.has_xcb_xf86vidmode_query_version());
        assert!(lib.has_xcb_xf86vidmode_query_version_unchecked());
        assert!(lib.has_xcb_xf86vidmode_query_version_reply());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_sizeof());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_private());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_private_length());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_private_end());
        assert!(lib.has_xcb_xf86vidmode_get_mode_line_reply());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line_sizeof());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line_checked());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line_private());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line_private_length());
        assert!(lib.has_xcb_xf86vidmode_mod_mode_line_private_end());
        assert!(lib.has_xcb_xf86vidmode_switch_mode_checked());
        assert!(lib.has_xcb_xf86vidmode_switch_mode());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_sizeof());
        assert!(lib.has_xcb_xf86vidmode_get_monitor());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_hsync());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_hsync_length());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_hsync_end());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vsync());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vsync_length());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vsync_end());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vendor());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vendor_length());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_vendor_end());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_alignment_pad());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_alignment_pad_length());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_alignment_pad_end());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_model());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_model_length());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_model_end());
        assert!(lib.has_xcb_xf86vidmode_get_monitor_reply());
        assert!(lib.has_xcb_xf86vidmode_lock_mode_switch_checked());
        assert!(lib.has_xcb_xf86vidmode_lock_mode_switch());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_sizeof());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_modeinfo());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_modeinfo_length());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator());
        assert!(lib.has_xcb_xf86vidmode_get_all_mode_lines_reply());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line_sizeof());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line_checked());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line_private());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line_private_length());
        assert!(lib.has_xcb_xf86vidmode_add_mode_line_private_end());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line_sizeof());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line_checked());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line_private());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line_private_length());
        assert!(lib.has_xcb_xf86vidmode_delete_mode_line_private_end());
        assert!(lib.has_xcb_xf86vidmode_validate_mode_line_sizeof());
        assert!(lib.has_xcb_xf86vidmode_validate_mode_line());
        assert!(lib.has_xcb_xf86vidmode_validate_mode_line_unchecked());
        assert!(lib.has_xcb_xf86vidmode_validate_mode_line_reply());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode_sizeof());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode_checked());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode_private());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode_private_length());
        assert!(lib.has_xcb_xf86vidmode_switch_to_mode_private_end());
        assert!(lib.has_xcb_xf86vidmode_get_view_port());
        assert!(lib.has_xcb_xf86vidmode_get_view_port_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_view_port_reply());
        assert!(lib.has_xcb_xf86vidmode_set_view_port_checked());
        assert!(lib.has_xcb_xf86vidmode_set_view_port());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_sizeof());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_clock());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_clock_length());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_clock_end());
        assert!(lib.has_xcb_xf86vidmode_get_dot_clocks_reply());
        assert!(lib.has_xcb_xf86vidmode_set_client_version_checked());
        assert!(lib.has_xcb_xf86vidmode_set_client_version());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_checked());
        assert!(lib.has_xcb_xf86vidmode_set_gamma());
        assert!(lib.has_xcb_xf86vidmode_get_gamma());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_reply());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_sizeof());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_red());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_red_length());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_red_end());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_green());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_green_length());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_green_end());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_blue());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_blue_length());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_blue_end());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_reply());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_sizeof());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_checked());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_red());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_red_length());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_red_end());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_green());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_green_length());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_green_end());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_blue());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_blue_length());
        assert!(lib.has_xcb_xf86vidmode_set_gamma_ramp_blue_end());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_size());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_size_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_gamma_ramp_size_reply());
        assert!(lib.has_xcb_xf86vidmode_get_permissions());
        assert!(lib.has_xcb_xf86vidmode_get_permissions_unchecked());
        assert!(lib.has_xcb_xf86vidmode_get_permissions_reply());
    }
}
