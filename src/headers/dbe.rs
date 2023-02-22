// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::*;
use crate::lazy::*;
use std::os::raw::*;

/// The name of the `Dbe` extension.
pub const XCB_DBE_NAME: &[u8] = b"DOUBLE-BUFFER";

/// The name of the `Dbe` extension.
pub const XCB_DBE_NAME_STR: &str = "DOUBLE-BUFFER";


/// The `Dbe::BackBuffer` type.
pub type xcb_dbe_back_buffer_t = u32;

/// An iterator over `Dbe::BackBuffer` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_back_buffer_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dbe_back_buffer_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dbe_back_buffer_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::SwapAction` enum.
///
/// This enum has the following variants:
///
/// - [`Dbe::SwapAction::Undefined`](XCB_DBE_SWAP_ACTION_UNDEFINED)
/// - [`Dbe::SwapAction::Background`](XCB_DBE_SWAP_ACTION_BACKGROUND)
/// - [`Dbe::SwapAction::Untouched`](XCB_DBE_SWAP_ACTION_UNTOUCHED)
/// - [`Dbe::SwapAction::Copied`](XCB_DBE_SWAP_ACTION_COPIED)
pub type xcb_dbe_swap_action_t = u32;
/// The `Dbe::SwapAction::Undefined` enum variant.
///
/// This is a variant of [`xcb_dbe_swap_action_t`].
pub const XCB_DBE_SWAP_ACTION_UNDEFINED: xcb_dbe_swap_action_t = 0;
/// The `Dbe::SwapAction::Background` enum variant.
///
/// This is a variant of [`xcb_dbe_swap_action_t`].
pub const XCB_DBE_SWAP_ACTION_BACKGROUND: xcb_dbe_swap_action_t = 1;
/// The `Dbe::SwapAction::Untouched` enum variant.
///
/// This is a variant of [`xcb_dbe_swap_action_t`].
pub const XCB_DBE_SWAP_ACTION_UNTOUCHED: xcb_dbe_swap_action_t = 2;
/// The `Dbe::SwapAction::Copied` enum variant.
///
/// This is a variant of [`xcb_dbe_swap_action_t`].
pub const XCB_DBE_SWAP_ACTION_COPIED: xcb_dbe_swap_action_t = 3;

/// The `Dbe::SwapInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_swap_info_t {
    pub window: xcb_window_t,
    pub swap_action: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_dbe_swap_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Dbe::SwapInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_swap_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dbe_swap_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dbe_swap_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::BufferAttributes` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_buffer_attributes_t {
    pub window: xcb_window_t,
}

impl Default for xcb_dbe_buffer_attributes_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Dbe::BufferAttributes` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_buffer_attributes_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dbe_buffer_attributes_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dbe_buffer_attributes_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::VisualInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_visual_info_t {
    pub visual_id: xcb_visualid_t,
    pub depth: u8,
    pub perf_level: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_dbe_visual_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Dbe::VisualInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_visual_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dbe_visual_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dbe_visual_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::VisualInfos` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `infos`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_visual_infos_t {
    pub n_infos: u32,
}

impl Default for xcb_dbe_visual_infos_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Dbe::VisualInfos` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_visual_infos_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dbe_visual_infos_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dbe_visual_infos_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::BadBuffer` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_dbe_bad_buffer_error_t`].
pub const XCB_DBE_BAD_BUFFER: u8 = 0i32 as u8;

/// The `Dbe::BadBuffer` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_bad_buffer_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_buffer: xcb_dbe_back_buffer_t,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl Default for xcb_dbe_bad_buffer_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Dbe::QueryVersion` request.
///
/// Pass this cookie to [`xcb_dbe_query_version_reply`] to retrieve the reply.
///
/// [`xcb_dbe_query_version_reply`]: XcbDbe::xcb_dbe_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dbe_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_query_version_request_t`].
pub const XCB_DBE_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Dbe::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u8,
    pub minor_version: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_dbe_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub pad1: [u8; 22],
}

impl Default for xcb_dbe_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::AllocateBackBuffer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_allocate_back_buffer_request_t`].
pub const XCB_DBE_ALLOCATE_BACK_BUFFER: u8 = 1i32 as u8;

/// The `Dbe::AllocateBackBuffer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_allocate_back_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub buffer: xcb_dbe_back_buffer_t,
    pub swap_action: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_dbe_allocate_back_buffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::DeallocateBackBuffer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_deallocate_back_buffer_request_t`].
pub const XCB_DBE_DEALLOCATE_BACK_BUFFER: u8 = 2i32 as u8;

/// The `Dbe::DeallocateBackBuffer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_deallocate_back_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub buffer: xcb_dbe_back_buffer_t,
}

impl Default for xcb_dbe_deallocate_back_buffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::SwapBuffers` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_swap_buffers_request_t`].
pub const XCB_DBE_SWAP_BUFFERS: u8 = 3i32 as u8;

/// The `Dbe::SwapBuffers` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `actions`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_swap_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub n_actions: u32,
}

impl Default for xcb_dbe_swap_buffers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::BeginIdiom` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_begin_idiom_request_t`].
pub const XCB_DBE_BEGIN_IDIOM: u8 = 4i32 as u8;

/// The `Dbe::BeginIdiom` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_begin_idiom_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dbe_begin_idiom_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::EndIdiom` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_end_idiom_request_t`].
pub const XCB_DBE_END_IDIOM: u8 = 5i32 as u8;

/// The `Dbe::EndIdiom` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_end_idiom_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dbe_end_idiom_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Dbe::GetVisualInfo` request.
///
/// Pass this cookie to [`xcb_dbe_get_visual_info_reply`] to retrieve the reply.
///
/// [`xcb_dbe_get_visual_info_reply`]: XcbDbe::xcb_dbe_get_visual_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_visual_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dbe_get_visual_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::GetVisualInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_get_visual_info_request_t`].
pub const XCB_DBE_GET_VISUAL_INFO: u8 = 6i32 as u8;

/// The `Dbe::GetVisualInfo` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `drawables`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_visual_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub n_drawables: u32,
}

impl Default for xcb_dbe_get_visual_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::GetVisualInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `supported_visuals`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_visual_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub n_supported_visuals: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_dbe_get_visual_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Dbe::GetBackBufferAttributes` request.
///
/// Pass this cookie to [`xcb_dbe_get_back_buffer_attributes_reply`] to retrieve the reply.
///
/// [`xcb_dbe_get_back_buffer_attributes_reply`]: XcbDbe::xcb_dbe_get_back_buffer_attributes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_back_buffer_attributes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dbe_get_back_buffer_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Dbe::GetBackBufferAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDbe::xcb_dbe_id()`], then the type of the request is
/// [`xcb_dbe_get_back_buffer_attributes_request_t`].
pub const XCB_DBE_GET_BACK_BUFFER_ATTRIBUTES: u8 = 7i32 as u8;

/// The `Dbe::GetBackBufferAttributes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_back_buffer_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub buffer: xcb_dbe_back_buffer_t,
}

impl Default for xcb_dbe_get_back_buffer_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Dbe::GetBackBufferAttributes` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dbe_get_back_buffer_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub attributes: xcb_dbe_buffer_attributes_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_dbe_get_back_buffer_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_dbe")]
pub(crate) struct XcbDbeDbe {
    xcb_dbe_id: LazySymbol<*mut xcb_extension_t>,
    xcb_dbe_back_buffer_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_dbe_back_buffer_iterator_t, )>,
    xcb_dbe_back_buffer_end: LazySymbol<unsafe extern "C" fn(i: xcb_dbe_back_buffer_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_dbe_swap_info_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_dbe_swap_info_iterator_t, )>,
    xcb_dbe_swap_info_end: LazySymbol<unsafe extern "C" fn(i: xcb_dbe_swap_info_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_dbe_buffer_attributes_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_dbe_buffer_attributes_iterator_t, )>,
    xcb_dbe_buffer_attributes_end: LazySymbol<unsafe extern "C" fn(i: xcb_dbe_buffer_attributes_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_dbe_visual_info_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_dbe_visual_info_iterator_t, )>,
    xcb_dbe_visual_info_end: LazySymbol<unsafe extern "C" fn(i: xcb_dbe_visual_info_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_dbe_visual_infos_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_dbe_visual_infos_infos: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_visual_infos_t, ) -> *mut xcb_dbe_visual_info_t>,
    xcb_dbe_visual_infos_infos_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_visual_infos_t, ) -> c_int>,
    xcb_dbe_visual_infos_infos_iterator: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_visual_infos_t, ) -> xcb_dbe_visual_info_iterator_t>,
    xcb_dbe_visual_infos_next: LazySymbol<unsafe extern "C" fn(i: *mut xcb_dbe_visual_infos_iterator_t, )>,
    xcb_dbe_visual_infos_end: LazySymbol<unsafe extern "C" fn(i: xcb_dbe_visual_infos_iterator_t, ) -> xcb_generic_iterator_t>,
    xcb_dbe_query_version: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, major_version: u8, minor_version: u8, ) -> xcb_dbe_query_version_cookie_t>,
    xcb_dbe_query_version_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, major_version: u8, minor_version: u8, ) -> xcb_dbe_query_version_cookie_t>,
    xcb_dbe_query_version_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_dbe_query_version_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_query_version_reply_t>,
    xcb_dbe_allocate_back_buffer_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, window: xcb_window_t, buffer: xcb_dbe_back_buffer_t, swap_action: u8, ) -> xcb_void_cookie_t>,
    xcb_dbe_allocate_back_buffer: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, window: xcb_window_t, buffer: xcb_dbe_back_buffer_t, swap_action: u8, ) -> xcb_void_cookie_t>,
    xcb_dbe_deallocate_back_buffer_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_deallocate_back_buffer: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_swap_buffers_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_dbe_swap_buffers_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, n_actions: u32, actions: *const xcb_dbe_swap_info_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_swap_buffers: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, n_actions: u32, actions: *const xcb_dbe_swap_info_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_swap_buffers_actions: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_swap_buffers_request_t, ) -> *mut xcb_dbe_swap_info_t>,
    xcb_dbe_swap_buffers_actions_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_swap_buffers_request_t, ) -> c_int>,
    xcb_dbe_swap_buffers_actions_iterator: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_swap_buffers_request_t, ) -> xcb_dbe_swap_info_iterator_t>,
    xcb_dbe_begin_idiom_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_begin_idiom: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_end_idiom_checked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_end_idiom: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, ) -> xcb_void_cookie_t>,
    xcb_dbe_get_visual_info_sizeof: LazySymbol<unsafe extern "C" fn(_buffer: *const c_void, ) -> c_int>,
    xcb_dbe_get_visual_info: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, n_drawables: u32, drawables: *const xcb_drawable_t, ) -> xcb_dbe_get_visual_info_cookie_t>,
    xcb_dbe_get_visual_info_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, n_drawables: u32, drawables: *const xcb_drawable_t, ) -> xcb_dbe_get_visual_info_cookie_t>,
    xcb_dbe_get_visual_info_supported_visuals_length: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_get_visual_info_reply_t, ) -> c_int>,
    xcb_dbe_get_visual_info_supported_visuals_iterator: LazySymbol<unsafe extern "C" fn(r: *const xcb_dbe_get_visual_info_reply_t, ) -> xcb_dbe_visual_infos_iterator_t>,
    xcb_dbe_get_visual_info_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_dbe_get_visual_info_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_get_visual_info_reply_t>,
    xcb_dbe_get_back_buffer_attributes: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_dbe_get_back_buffer_attributes_cookie_t>,
    xcb_dbe_get_back_buffer_attributes_unchecked: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_dbe_get_back_buffer_attributes_cookie_t>,
    xcb_dbe_get_back_buffer_attributes_reply: LazySymbol<unsafe extern "C" fn(c: *mut xcb_connection_t, cookie: xcb_dbe_get_back_buffer_attributes_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_get_back_buffer_attributes_reply_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.dbe.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self.dbe.$f.exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_dbe")]
impl XcbDbe {
/// The libxcb identifier of the `Dbe` extension.
    #[inline]
    pub fn xcb_dbe_id(&self, ) -> *mut xcb_extension_t { unsafe { sym!(self, xcb_dbe_id) } }

    /// Returns `true` iff the symbol `xcb_dbe_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_id(&self) -> bool { has_sym!(self, xcb_dbe_id) }

/// Advances a `xcb_dbe_back_buffer_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dbe_back_buffer_next(&self, i: *mut xcb_dbe_back_buffer_iterator_t, ) { sym!(self, xcb_dbe_back_buffer_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_back_buffer_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_back_buffer_next(&self) -> bool { has_sym!(self, xcb_dbe_back_buffer_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dbe_back_buffer_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dbe_back_buffer_end(&self, i: xcb_dbe_back_buffer_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_dbe_back_buffer_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_back_buffer_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_back_buffer_end(&self) -> bool { has_sym!(self, xcb_dbe_back_buffer_end) }

/// Advances a `xcb_dbe_swap_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dbe_swap_info_next(&self, i: *mut xcb_dbe_swap_info_iterator_t, ) { sym!(self, xcb_dbe_swap_info_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_info_next(&self) -> bool { has_sym!(self, xcb_dbe_swap_info_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dbe_swap_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dbe_swap_info_end(&self, i: xcb_dbe_swap_info_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_dbe_swap_info_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_info_end(&self) -> bool { has_sym!(self, xcb_dbe_swap_info_end) }

/// Advances a `xcb_dbe_buffer_attributes_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dbe_buffer_attributes_next(&self, i: *mut xcb_dbe_buffer_attributes_iterator_t, ) { sym!(self, xcb_dbe_buffer_attributes_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_buffer_attributes_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_buffer_attributes_next(&self) -> bool { has_sym!(self, xcb_dbe_buffer_attributes_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dbe_buffer_attributes_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dbe_buffer_attributes_end(&self, i: xcb_dbe_buffer_attributes_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_dbe_buffer_attributes_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_buffer_attributes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_buffer_attributes_end(&self) -> bool { has_sym!(self, xcb_dbe_buffer_attributes_end) }

/// Advances a `xcb_dbe_visual_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dbe_visual_info_next(&self, i: *mut xcb_dbe_visual_info_iterator_t, ) { sym!(self, xcb_dbe_visual_info_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_info_next(&self) -> bool { has_sym!(self, xcb_dbe_visual_info_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dbe_visual_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dbe_visual_info_end(&self, i: xcb_dbe_visual_info_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_dbe_visual_info_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_info_end(&self) -> bool { has_sym!(self, xcb_dbe_visual_info_end) }

/// Computes the size of a `xcb_dbe_visual_infos_t` object.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_dbe_visual_infos_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_sizeof(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_sizeof) }

/// Returns a pointer to the `infos` field of a `xcb_dbe_visual_infos_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_infos(&self, r: *const xcb_dbe_visual_infos_t, ) -> *mut xcb_dbe_visual_info_t { sym!(self, xcb_dbe_visual_infos_infos)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_infos` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_infos(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_infos) }

/// Returns the number of elements of the `infos` field of a `xcb_dbe_visual_infos_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_infos_length(&self, r: *const xcb_dbe_visual_infos_t, ) -> c_int { sym!(self, xcb_dbe_visual_infos_infos_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_infos_length(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_infos_length) }

/// Returns an iterator over the elements of the
/// `infos` field of a `xcb_dbe_visual_infos_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_infos_iterator(&self, r: *const xcb_dbe_visual_infos_t, ) -> xcb_dbe_visual_info_iterator_t { sym!(self, xcb_dbe_visual_infos_infos_iterator)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_infos_iterator(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_infos_iterator) }

/// Advances a `xcb_dbe_visual_infos_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_next(&self, i: *mut xcb_dbe_visual_infos_iterator_t, ) { sym!(self, xcb_dbe_visual_infos_next)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_next(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_next) }

/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dbe_visual_infos_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dbe_visual_infos_end(&self, i: xcb_dbe_visual_infos_iterator_t, ) -> xcb_generic_iterator_t { sym!(self, xcb_dbe_visual_infos_end)(i, ) }

    /// Returns `true` iff the symbol `xcb_dbe_visual_infos_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_visual_infos_end(&self) -> bool { has_sym!(self, xcb_dbe_visual_infos_end) }

/// Sends a `Dbe::QueryVersion` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_query_version_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_query_version_reply`]: Self::xcb_dbe_query_version_reply
    #[inline]
    pub unsafe fn xcb_dbe_query_version(&self, c: *mut xcb_connection_t, major_version: u8, minor_version: u8, ) -> xcb_dbe_query_version_cookie_t { sym!(self, xcb_dbe_query_version)(c, major_version, minor_version, ) }

    /// Returns `true` iff the symbol `xcb_dbe_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_query_version(&self) -> bool { has_sym!(self, xcb_dbe_query_version) }

/// Sends a `Dbe::QueryVersion` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_query_version_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_query_version_reply`]: Self::xcb_dbe_query_version_reply
    #[inline]
    pub unsafe fn xcb_dbe_query_version_unchecked(&self, c: *mut xcb_connection_t, major_version: u8, minor_version: u8, ) -> xcb_dbe_query_version_cookie_t { sym!(self, xcb_dbe_query_version_unchecked)(c, major_version, minor_version, ) }

    /// Returns `true` iff the symbol `xcb_dbe_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_query_version_unchecked(&self) -> bool { has_sym!(self, xcb_dbe_query_version_unchecked) }

/// Waits for the reply to a `Dbe::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_dbe_query_version_reply(&self, c: *mut xcb_connection_t, cookie: xcb_dbe_query_version_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_query_version_reply_t { sym!(self, xcb_dbe_query_version_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_dbe_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_query_version_reply(&self) -> bool { has_sym!(self, xcb_dbe_query_version_reply) }

/// Sends a `Dbe::AllocateBackBuffer` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dbe_allocate_back_buffer_checked(&self, c: *mut xcb_connection_t, window: xcb_window_t, buffer: xcb_dbe_back_buffer_t, swap_action: u8, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_allocate_back_buffer_checked)(c, window, buffer, swap_action, ) }

    /// Returns `true` iff the symbol `xcb_dbe_allocate_back_buffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_allocate_back_buffer_checked(&self) -> bool { has_sym!(self, xcb_dbe_allocate_back_buffer_checked) }

/// Sends a `Dbe::AllocateBackBuffer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dbe_allocate_back_buffer(&self, c: *mut xcb_connection_t, window: xcb_window_t, buffer: xcb_dbe_back_buffer_t, swap_action: u8, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_allocate_back_buffer)(c, window, buffer, swap_action, ) }

    /// Returns `true` iff the symbol `xcb_dbe_allocate_back_buffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_allocate_back_buffer(&self) -> bool { has_sym!(self, xcb_dbe_allocate_back_buffer) }

/// Sends a `Dbe::DeallocateBackBuffer` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dbe_deallocate_back_buffer_checked(&self, c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_deallocate_back_buffer_checked)(c, buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_deallocate_back_buffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_deallocate_back_buffer_checked(&self) -> bool { has_sym!(self, xcb_dbe_deallocate_back_buffer_checked) }

/// Sends a `Dbe::DeallocateBackBuffer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dbe_deallocate_back_buffer(&self, c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_deallocate_back_buffer)(c, buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_deallocate_back_buffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_deallocate_back_buffer(&self) -> bool { has_sym!(self, xcb_dbe_deallocate_back_buffer) }

/// Computes the size of a `xcb_dbe_swap_buffers_request_t` object.
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_dbe_swap_buffers_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers_sizeof(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers_sizeof) }

/// Sends a `Dbe::SwapBuffers` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers_checked(&self, c: *mut xcb_connection_t, n_actions: u32, actions: *const xcb_dbe_swap_info_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_swap_buffers_checked)(c, n_actions, actions, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers_checked(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers_checked) }

/// Sends a `Dbe::SwapBuffers` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers(&self, c: *mut xcb_connection_t, n_actions: u32, actions: *const xcb_dbe_swap_info_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_swap_buffers)(c, n_actions, actions, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers) }

/// Returns a pointer to the `actions` field of a `xcb_dbe_swap_buffers_request_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers_actions(&self, r: *const xcb_dbe_swap_buffers_request_t, ) -> *mut xcb_dbe_swap_info_t { sym!(self, xcb_dbe_swap_buffers_actions)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers_actions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers_actions(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers_actions) }

/// Returns the number of elements of the `actions` field of a `xcb_dbe_swap_buffers_request_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers_actions_length(&self, r: *const xcb_dbe_swap_buffers_request_t, ) -> c_int { sym!(self, xcb_dbe_swap_buffers_actions_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers_actions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers_actions_length(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers_actions_length) }

/// Returns an iterator over the elements of the
/// `actions` field of a `xcb_dbe_swap_buffers_request_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_swap_buffers_actions_iterator(&self, r: *const xcb_dbe_swap_buffers_request_t, ) -> xcb_dbe_swap_info_iterator_t { sym!(self, xcb_dbe_swap_buffers_actions_iterator)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_swap_buffers_actions_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_swap_buffers_actions_iterator(&self) -> bool { has_sym!(self, xcb_dbe_swap_buffers_actions_iterator) }

/// Sends a `Dbe::BeginIdiom` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dbe_begin_idiom_checked(&self, c: *mut xcb_connection_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_begin_idiom_checked)(c, ) }

    /// Returns `true` iff the symbol `xcb_dbe_begin_idiom_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_begin_idiom_checked(&self) -> bool { has_sym!(self, xcb_dbe_begin_idiom_checked) }

/// Sends a `Dbe::BeginIdiom` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dbe_begin_idiom(&self, c: *mut xcb_connection_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_begin_idiom)(c, ) }

    /// Returns `true` iff the symbol `xcb_dbe_begin_idiom` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_begin_idiom(&self) -> bool { has_sym!(self, xcb_dbe_begin_idiom) }

/// Sends a `Dbe::EndIdiom` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_request_check`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dbe_end_idiom_checked(&self, c: *mut xcb_connection_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_end_idiom_checked)(c, ) }

    /// Returns `true` iff the symbol `xcb_dbe_end_idiom_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_end_idiom_checked(&self) -> bool { has_sym!(self, xcb_dbe_end_idiom_checked) }

/// Sends a `Dbe::EndIdiom` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dbe_end_idiom(&self, c: *mut xcb_connection_t, ) -> xcb_void_cookie_t { sym!(self, xcb_dbe_end_idiom)(c, ) }

    /// Returns `true` iff the symbol `xcb_dbe_end_idiom` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_end_idiom(&self) -> bool { has_sym!(self, xcb_dbe_end_idiom) }

/// Computes the size of a `xcb_dbe_get_visual_info_request_t` object.
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info_sizeof(&self, _buffer: *const c_void, ) -> c_int { sym!(self, xcb_dbe_get_visual_info_sizeof)(_buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info_sizeof(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info_sizeof) }

/// Sends a `Dbe::GetVisualInfo` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_get_visual_info_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_get_visual_info_reply`]: Self::xcb_dbe_get_visual_info_reply
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info(&self, c: *mut xcb_connection_t, n_drawables: u32, drawables: *const xcb_drawable_t, ) -> xcb_dbe_get_visual_info_cookie_t { sym!(self, xcb_dbe_get_visual_info)(c, n_drawables, drawables, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info) }

/// Sends a `Dbe::GetVisualInfo` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_get_visual_info_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_get_visual_info_reply`]: Self::xcb_dbe_get_visual_info_reply
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info_unchecked(&self, c: *mut xcb_connection_t, n_drawables: u32, drawables: *const xcb_drawable_t, ) -> xcb_dbe_get_visual_info_cookie_t { sym!(self, xcb_dbe_get_visual_info_unchecked)(c, n_drawables, drawables, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info_unchecked(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info_unchecked) }

/// Returns the number of elements of the `supported_visuals` field of a `xcb_dbe_get_visual_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info_supported_visuals_length(&self, r: *const xcb_dbe_get_visual_info_reply_t, ) -> c_int { sym!(self, xcb_dbe_get_visual_info_supported_visuals_length)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info_supported_visuals_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info_supported_visuals_length(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info_supported_visuals_length) }

/// Returns an iterator over the elements of the
/// `supported_visuals` field of a `xcb_dbe_get_visual_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info_supported_visuals_iterator(&self, r: *const xcb_dbe_get_visual_info_reply_t, ) -> xcb_dbe_visual_infos_iterator_t { sym!(self, xcb_dbe_get_visual_info_supported_visuals_iterator)(r, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info_supported_visuals_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info_supported_visuals_iterator(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info_supported_visuals_iterator) }

/// Waits for the reply to a `Dbe::GetVisualInfo` request.
    #[inline]
    pub unsafe fn xcb_dbe_get_visual_info_reply(&self, c: *mut xcb_connection_t, cookie: xcb_dbe_get_visual_info_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_get_visual_info_reply_t { sym!(self, xcb_dbe_get_visual_info_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_visual_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_visual_info_reply(&self) -> bool { has_sym!(self, xcb_dbe_get_visual_info_reply) }

/// Sends a `Dbe::GetBackBufferAttributes` request (checked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_get_back_buffer_attributes_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_get_back_buffer_attributes_reply`]: Self::xcb_dbe_get_back_buffer_attributes_reply
    #[inline]
    pub unsafe fn xcb_dbe_get_back_buffer_attributes(&self, c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_dbe_get_back_buffer_attributes_cookie_t { sym!(self, xcb_dbe_get_back_buffer_attributes)(c, buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_back_buffer_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_back_buffer_attributes(&self) -> bool { has_sym!(self, xcb_dbe_get_back_buffer_attributes) }

/// Sends a `Dbe::GetBackBufferAttributes` request (unchecked).
///
/// This request generates a reply. You must either discard it with
/// [`discard_reply`] or retrieve it with [`xcb_dbe_get_back_buffer_attributes_reply`].
///
/// [`discard_reply`]: crate::Xcb::xcb_discard_reply
/// [`xcb_dbe_get_back_buffer_attributes_reply`]: Self::xcb_dbe_get_back_buffer_attributes_reply
    #[inline]
    pub unsafe fn xcb_dbe_get_back_buffer_attributes_unchecked(&self, c: *mut xcb_connection_t, buffer: xcb_dbe_back_buffer_t, ) -> xcb_dbe_get_back_buffer_attributes_cookie_t { sym!(self, xcb_dbe_get_back_buffer_attributes_unchecked)(c, buffer, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_back_buffer_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_back_buffer_attributes_unchecked(&self) -> bool { has_sym!(self, xcb_dbe_get_back_buffer_attributes_unchecked) }

/// Waits for the reply to a `Dbe::GetBackBufferAttributes` request.
    #[inline]
    pub unsafe fn xcb_dbe_get_back_buffer_attributes_reply(&self, c: *mut xcb_connection_t, cookie: xcb_dbe_get_back_buffer_attributes_cookie_t, e: *mut *mut xcb_generic_error_t, ) -> *mut xcb_dbe_get_back_buffer_attributes_reply_t { sym!(self, xcb_dbe_get_back_buffer_attributes_reply)(c, cookie, e, ) }

    /// Returns `true` iff the symbol `xcb_dbe_get_back_buffer_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dbe_get_back_buffer_attributes_reply(&self) -> bool { has_sym!(self, xcb_dbe_get_back_buffer_attributes_reply) }
}

#[cfg(feature = "xcb_dbe")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDbe::load().unwrap() };
        assert!(lib.has_xcb_dbe_id());
        assert!(lib.has_xcb_dbe_back_buffer_next());
        assert!(lib.has_xcb_dbe_back_buffer_end());
        assert!(lib.has_xcb_dbe_swap_info_next());
        assert!(lib.has_xcb_dbe_swap_info_end());
        assert!(lib.has_xcb_dbe_buffer_attributes_next());
        assert!(lib.has_xcb_dbe_buffer_attributes_end());
        assert!(lib.has_xcb_dbe_visual_info_next());
        assert!(lib.has_xcb_dbe_visual_info_end());
        assert!(lib.has_xcb_dbe_visual_infos_sizeof());
        assert!(lib.has_xcb_dbe_visual_infos_infos());
        assert!(lib.has_xcb_dbe_visual_infos_infos_length());
        assert!(lib.has_xcb_dbe_visual_infos_infos_iterator());
        assert!(lib.has_xcb_dbe_visual_infos_next());
        assert!(lib.has_xcb_dbe_visual_infos_end());
        assert!(lib.has_xcb_dbe_query_version());
        assert!(lib.has_xcb_dbe_query_version_unchecked());
        assert!(lib.has_xcb_dbe_query_version_reply());
        assert!(lib.has_xcb_dbe_allocate_back_buffer_checked());
        assert!(lib.has_xcb_dbe_allocate_back_buffer());
        assert!(lib.has_xcb_dbe_deallocate_back_buffer_checked());
        assert!(lib.has_xcb_dbe_deallocate_back_buffer());
        assert!(lib.has_xcb_dbe_swap_buffers_sizeof());
        assert!(lib.has_xcb_dbe_swap_buffers_checked());
        assert!(lib.has_xcb_dbe_swap_buffers());
        assert!(lib.has_xcb_dbe_swap_buffers_actions());
        assert!(lib.has_xcb_dbe_swap_buffers_actions_length());
        assert!(lib.has_xcb_dbe_swap_buffers_actions_iterator());
        assert!(lib.has_xcb_dbe_begin_idiom_checked());
        assert!(lib.has_xcb_dbe_begin_idiom());
        assert!(lib.has_xcb_dbe_end_idiom_checked());
        assert!(lib.has_xcb_dbe_end_idiom());
        assert!(lib.has_xcb_dbe_get_visual_info_sizeof());
        assert!(lib.has_xcb_dbe_get_visual_info());
        assert!(lib.has_xcb_dbe_get_visual_info_unchecked());
        assert!(lib.has_xcb_dbe_get_visual_info_supported_visuals_length());
        assert!(lib.has_xcb_dbe_get_visual_info_supported_visuals_iterator());
        assert!(lib.has_xcb_dbe_get_visual_info_reply());
        assert!(lib.has_xcb_dbe_get_back_buffer_attributes());
        assert!(lib.has_xcb_dbe_get_back_buffer_attributes_unchecked());
        assert!(lib.has_xcb_dbe_get_back_buffer_attributes_reply());
    }
}
