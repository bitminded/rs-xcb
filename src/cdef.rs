use std::os::raw::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub type XCBAtom = c_uint;
pub type XCBSetup = c_void;
pub type XCBWindow = c_uint;
pub type XCBTimestamp = c_uint;
pub type XCBColormap = c_uint;
pub type XCBVisualId = c_uint;

// opaque type
#[repr(C)]
pub struct XCBConnection {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XCBVoidCookie {
    sequence: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XCBScreen {
    pub root: c_uint,
    pub default_colormap: c_uint,
    pub white_pixel: c_uint,
    pub black_pixel: c_uint,
    pub current_input_masks: c_uint,
    pub width_in_pixels: c_ushort,
    pub height_in_pixels: c_ushort,
    pub width_in_millimeters: c_ushort,
    pub height_in_millimeters: c_ushort,
    pub min_installed_maps: c_ushort,
    pub max_installed_maps: c_ushort,
    pub root_visual: c_uint,
    pub backing_stores: c_uchar,
    pub save_unders: c_uchar,
    pub root_depth: c_char,
    pub allowed_depths_len: c_char,
}

#[repr(C)]
pub struct XCBScreenIterator {
    pub data: *mut XCBScreen,
    pub rem: c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct XCBGenericEvent {
    pub response_type: c_uchar,
    pub pad0: c_uchar,
    pub sequence: c_ushort,
    pub pad: [c_uint; 7],
    pub full_sequence: c_uint,
}

#[repr(C)]
pub struct XCBEnterNotifyEvent {
    pub response_type: c_uchar,
    pub detail: c_uchar,
    pub sequence: c_ushort,
    pub time: XCBTimestamp,
    pub root: XCBWindow,
    pub event: XCBWindow,
    pub child: XCBWindow,
    pub root_x: c_short,
    pub root_y: c_short,
    pub event_x: c_short,
    pub event_y: c_short,
    pub state: c_ushort,
    pub mode: c_uchar,
    pub same_screen_focus: c_uchar,
}

#[repr(C)]
pub struct XCBDestroyNotifyEvent {
    pub response_type: c_uchar,
    pub pad0: c_uchar,
    pub sequence: c_ushort,
    pub event: XCBWindow,
    pub window: XCBWindow,
}

#[repr(C)]
#[derive(Debug)]
pub struct XCBClientMessageEvent {
    pub response_type: c_uchar,
    pub format: c_uchar,
    pub sequence: c_ushort,
    pub window: XCBWindow,
    pub data_type: XCBAtom,
    // Note: Although the xcb headers say that the correct
    //       type for this field is xcb_client_message_data_t,
    //       it doesn't seem to be correct. So for now we are going to
    //       use a generic byte array.
    // pub data: XCBClientMessageData
    pub data: [c_uchar; 20],
}

/// TODO: Clear up some misunderstandings about this data structure.
///       Since the data field of a client_message can be 8, 16 or 32-bit format,
///       what is the point of having all 3 of these present in this structure?
///       Shouldn't all 3 fields contain the exact same byte data?
#[repr(C)]
#[derive(Debug)]
pub struct XCBClientMessageData {
    pub data8: [c_uchar; 20],
    pub data16: [c_ushort; 10],
    pub data32: [c_uint; 5],
}

#[repr(C)]
pub struct XCBInternAtomCookie {
    pub sequence: c_uint,
}

#[repr(C)]
pub struct XCBGenericError {
    pub response_type: c_uchar,
    pub error_code: c_uchar,
    pub sequence: c_ushort,
    pub resource_id: c_uint,
    pub minor_code: c_ushort,
    pub major_code: c_uchar,
    pub pad0: c_uchar,
    pub pad: [c_uint; 5],
    pub full_sequence: c_uint,
}

#[repr(C)]
pub struct XCBInternAtomReply {
    pub response_type: c_uchar,
    pub pad0: c_uchar,
    pub sequence: c_ushort,
    pub length: c_uint,
    pub atom: XCBAtom,
}

#[repr(C)]
pub struct XCBGetAtomNameCookie {
    pub sequence: c_uint,
}

#[repr(C)]
pub struct XCBGetAtomNameReply {
    pub response_type: c_uchar,
    pub pad0: c_uchar,
    pub sequence: c_ushort,
    pub length: c_uint,
    pub name_len: c_ushort,
    pub pad1: [c_uchar; 22],
}

#[link(name = "xcb")]
extern "system" {
    pub fn xcb_connect(displayname: *const c_char, screenp: *mut c_int) -> *mut XCBConnection;
    pub fn xcb_disconnect(connection: *mut XCBConnection);
    pub fn xcb_generate_id(connection: *mut XCBConnection) -> c_uint;
    pub fn xcb_create_window(
        connection: *mut XCBConnection,
        depth: c_uchar,
        window_id: c_uint,
        parent_id: c_uint,
        x: c_short,
        y: c_short,
        width: c_ushort,
        height: c_ushort,
        border_width: c_ushort,
        class: c_ushort,
        visual: c_uint,
        value_mask: c_uint,
        value_list: *const c_uint,
    ) -> XCBVoidCookie;
    pub fn xcb_get_setup(connection: *mut XCBConnection) -> *const XCBSetup;
    pub fn xcb_setup_roots_iterator(setup: *const XCBSetup) -> XCBScreenIterator;
    pub fn xcb_screen_next(i: *mut XCBScreenIterator);
    pub fn xcb_map_window(connection: *mut XCBConnection, window: XCBWindow) -> XCBVoidCookie;
    pub fn xcb_flush(connection: *mut XCBConnection) -> c_int;
    pub fn xcb_poll_for_event(connection: *mut XCBConnection) -> *mut XCBGenericEvent;
    pub fn xcb_wait_for_event(connection: *mut XCBConnection) -> *mut XCBGenericEvent;
    pub fn xcb_destroy_window(connection: *mut XCBConnection, window: XCBWindow) -> XCBVoidCookie;
    pub fn xcb_intern_atom(
        connection: *mut XCBConnection,
        only_if_exists: c_uchar,
        name_len: c_ushort,
        name: *const c_char,
    ) -> XCBInternAtomCookie;
    pub fn xcb_intern_atom_reply(
        connection: *mut XCBConnection,
        cookie: XCBInternAtomCookie,
        e: *mut *mut XCBGenericError,
    ) -> *mut XCBInternAtomReply;
    pub fn xcb_change_property(
        connection: *mut XCBConnection,
        mode: c_uchar,
        window: XCBWindow,
        property: XCBAtom,
        property_type: XCBAtom,
        format: c_uchar,
        data_len: c_uint,
        data: *const c_void,
    ) -> XCBVoidCookie;
    pub fn xcb_get_atom_name(connection: *mut XCBConnection, atom: XCBAtom)
        -> XCBGetAtomNameCookie;
    pub fn xcb_get_atom_name_reply(
        connection: *mut XCBConnection,
        cookie: XCBGetAtomNameCookie,
        e: *mut *mut XCBGenericError,
    ) -> *mut XCBGetAtomNameReply;
    pub fn xcb_create_colormap(
        connection: *mut XCBConnection,
        alloc: c_uchar,
        mid: XCBColormap,
        window: XCBWindow,
        visual: XCBVisualId,
    ) -> XCBVoidCookie;
}
