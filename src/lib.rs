use std::error::Error;
use std::ffi::CString;
use std::ptr::{null, null_mut};

pub mod cdef;
pub mod constants;

/// A simple wrapper struct for raw pointers that are not meant to be freed by application code.
pub struct DoNotFree<T> {
    data: *mut T,
}

impl<T> std::ops::Deref for DoNotFree<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.data) }
    }
}

pub type XCBAtom = cdef::XCBAtom;
pub type XCBTimestamp = cdef::XCBTimestamp;
pub type XCBColormap = cdef::XCBColormap;
pub type XCBWindow = cdef::XCBWindow;
pub type XCBVisualId = cdef::XCBVisualId;
pub type XCBVoidCookie = cdef::XCBVoidCookie;
pub type XCBInternAtomCookie = cdef::XCBInternAtomCookie;
pub type XCBGenericError = cdef::XCBGenericError;
pub type XCBInternAtomReply = cdef::XCBInternAtomReply;
pub type XCBScreen = cdef::XCBScreen;
pub type XCBGetAtomNameCookie = cdef::XCBGetAtomNameCookie;
pub type XCBGetAtomNameReply = cdef::XCBGetAtomNameReply;
pub type XCBScreenIterator = cdef::XCBScreenIterator;
pub type XCBGenericEvent = cdef::XCBGenericEvent;
pub type XCBKeyReleaseEvent = cdef::XCBKeyReleaseEvent;
pub type XCBExposeEvent = cdef::XCBExposeEvent;
pub type XCBEnterNotifyEvent = cdef::XCBEnterNotifyEvent;
pub type XCBDestroyNotifyEvent = cdef::XCBDestroyNotifyEvent;
pub type XCBClientMessageEvent = cdef::XCBClientMessageEvent;
pub type XCBClientMessageData = cdef::XCBClientMessageData;

pub struct XCBConnection {
    raw: *mut cdef::XCBConnection,
}

pub fn xcb_connect(
    displayname: Option<&str>,
    screen: Option<&mut i32>,
) -> Result<XCBConnection, Box<dyn Error>> {
    let cstr_displayname;
    let ptr_displayname;
    match displayname {
        Some(value) => match CString::new(value) {
            Err(e) => {
                return Err(Box::new(e));
            }
            Ok(value) => {
                cstr_displayname = value;
                ptr_displayname = cstr_displayname.as_ptr();
            }
        },
        None => {
            ptr_displayname = null::<std::os::raw::c_char>();
        }
    }

    let ptr_screen;
    match screen {
        None => {
            ptr_screen = null_mut::<std::os::raw::c_int>();
        }
        Some(value) => {
            ptr_screen = value as *mut std::os::raw::c_int;
        }
    }

    let connection = unsafe { cdef::xcb_connect(ptr_displayname, ptr_screen) };

    Ok(XCBConnection { raw: connection })
}

pub fn xcb_disconnect(connection: XCBConnection) {
    unsafe { cdef::xcb_disconnect(connection.raw) }
}

pub fn xcb_generate_id(connection: &XCBConnection) -> u32 {
    unsafe { cdef::xcb_generate_id(connection.raw) }
}

pub fn xcb_create_window(
    connection: &XCBConnection,
    depth: u8,
    window_id: u32,
    parent_id: u32,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    class: u16,
    visual: u32,
    value_mask: u32,
    value_list: Option<&[u32]>,
) -> XCBVoidCookie {
    let value_list_ptr: *const u32;
    match value_list {
        None => {
            value_list_ptr = null();
        }
        Some(value) => {
            let value_list_vec = Vec::from(value);
            value_list_ptr = value_list_vec.as_ptr();
            std::mem::forget(value_list_vec);
        }
    }

    unsafe {
        cdef::xcb_create_window(
            connection.raw,
            depth,
            window_id,
            parent_id,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list_ptr,
        )
    }
}

/// Accessor for setup data returned by the server when the connection was initialized.
///
/// # Parameters
/// ## connection
/// The connection.
///
/// # Return value
/// The setup data of the connection. It is not to be freed by application code.
pub fn xcb_get_setup(connection: &XCBConnection) -> DoNotFree<cdef::XCBSetup> {
    let xcb_setup = unsafe { cdef::xcb_get_setup(connection.raw) };

    DoNotFree::<cdef::XCBSetup> {
        data: xcb_setup as *mut cdef::XCBSetup,
    }
}

/// Creates an iterator for iterating over the available screens contained in the setup.
///
/// # Parameters
/// ## setup
/// The setup.
///
/// # Return value
/// An iterator.
pub fn xcb_setup_roots_iterator(setup: &DoNotFree<cdef::XCBSetup>) -> XCBScreenIterator {
    unsafe { cdef::xcb_setup_roots_iterator(setup.data) }
}

/// Accessor for a screen iterator's current data.
///
/// # Parameters
/// ## iter
/// The iterator
///
/// # Return value
/// The screen. It is not to be freed by application code.
pub fn xcb_screen(iter: &XCBScreenIterator) -> DoNotFree<XCBScreen> {
    DoNotFree::<XCBScreen> { data: iter.data }
}

/// Advances the given iterator to the next screen.
///
/// # Parameters
/// ## iter
/// The iterator
pub fn xcb_screen_next(iter: &mut XCBScreenIterator) {
    unsafe { cdef::xcb_screen_next(iter as *mut cdef::XCBScreenIterator) }
}

pub fn xcb_map_window(connection: &XCBConnection, window: XCBWindow) -> XCBVoidCookie {
    unsafe { cdef::xcb_map_window(connection.raw, window) }
}

pub fn xcb_flush(connection: &XCBConnection) -> i32 {
    unsafe { cdef::xcb_flush(connection.raw) }
}

pub fn xcb_poll_for_event(connection: &XCBConnection) -> Option<Box<XCBGenericEvent>> {
    unsafe {
        let event = cdef::xcb_poll_for_event(connection.raw);
        if event.is_null() {
            return None;
        } else {
            return Some(Box::from_raw(event));
        }
    }
}

pub fn xcb_wait_for_event(connection: &XCBConnection) -> Option<Box<XCBGenericEvent>> {
    unsafe {
        let event = cdef::xcb_wait_for_event(connection.raw);
        if event.is_null() {
            return None;
        } else {
            return Some(Box::from_raw(event));
        }
    }
}

pub fn xcb_destroy_window(connection: &XCBConnection, window: XCBWindow) -> XCBVoidCookie {
    unsafe { cdef::xcb_destroy_window(connection.raw, window) }
}

pub fn xcb_intern_atom(
    connection: &XCBConnection,
    only_if_exists: bool,
    name: &str,
) -> Result<XCBInternAtomCookie, std::ffi::NulError> {
    match CString::new(name) {
        Err(err) => {
            return Err(err);
        }
        Ok(cstr) => {
            let cookie = unsafe {
                cdef::xcb_intern_atom(
                    connection.raw,
                    if only_if_exists { 1 } else { 0 },
                    name.chars().count() as std::os::raw::c_ushort,
                    cstr.as_ptr(),
                )
            };
            return Ok(cookie);
        }
    }
}

pub fn xcb_intern_atom_reply(
    connection: &XCBConnection,
    cookie: XCBInternAtomCookie,
    e: Option<*mut *mut XCBGenericError>,
) -> Box<XCBInternAtomReply> {
    let e_ptr = match e {
        None => std::ptr::null_mut::<*mut XCBGenericError>(),
        Some(value) => value,
    };

    unsafe { Box::from_raw(cdef::xcb_intern_atom_reply(connection.raw, cookie, e_ptr)) }
}

pub fn xcb_change_property(
    connection: &XCBConnection,
    mode: std::os::raw::c_uchar,
    window: XCBWindow,
    property: XCBAtom,
    property_type: XCBAtom,
    format: std::os::raw::c_uchar,
    data_len: std::os::raw::c_uint,
    data: *const std::os::raw::c_void,
) -> XCBVoidCookie {
    unsafe {
        cdef::xcb_change_property(
            connection.raw,
            mode,
            window,
            property,
            property_type,
            format,
            data_len,
            data,
        )
    }
}

pub fn xcb_get_atom_name(connection: &XCBConnection, atom: XCBAtom) -> XCBGetAtomNameCookie {
    unsafe { cdef::xcb_get_atom_name(connection.raw, atom) }
}

pub fn xcb_get_atom_name_reply(
    connection: &XCBConnection,
    cookie: XCBGetAtomNameCookie,
    e: Option<*mut *mut XCBGenericError>,
) -> *mut XCBGetAtomNameReply {
    unsafe {
        match e {
            None => {
                return cdef::xcb_get_atom_name_reply(
                    connection.raw,
                    cookie,
                    null_mut::<*mut XCBGenericError>(),
                );
            }
            Some(e) => {
                return cdef::xcb_get_atom_name_reply(connection.raw, cookie, e);
            }
        }
    }
}

pub fn xcb_create_colormap(
    connection: &XCBConnection,
    alloc: u8,
    mid: XCBColormap,
    window: XCBWindow,
    visual: XCBVisualId,
) -> XCBVoidCookie {
    unsafe { cdef::xcb_create_colormap(connection.raw, alloc, mid, window, visual) }
}

pub fn xcb_clear_area(
    connection: &XCBConnection,
    exposures: bool,
    window: XCBWindow,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
) -> XCBVoidCookie {
    let exposures = if exposures { 1 } else { 0 };
    unsafe { cdef::xcb_clear_area(connection.raw, exposures, window, x, y, width, height) }
}

pub fn xcb_send_event(
    connection: &XCBConnection,
    propagate: bool,
    destination: XCBWindow,
    event_mask: u32,
    event: Box<XCBGenericEvent>,
) -> XCBVoidCookie {
    let propagate = if propagate { 1 } else { 0 };
    unsafe {
        cdef::xcb_send_event(
            connection.raw,
            propagate,
            destination,
            event_mask,
            Box::into_raw(event),
        )
    }
}
