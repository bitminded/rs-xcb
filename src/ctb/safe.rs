use crate::types;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::error::Error;

pub type XCBColormap = types::XCBColormap;
pub type XCBWindow = types::XCBWindow;
pub type XCBVisualId = types::XCBVisualId;
pub type XCBVoidCookie = super::XCBVoidCookie;

pub struct XCBConnection
{
    raw: *mut super::XCBConnection
}

pub struct XCBSetup
{
    /// DO NOT FREE THIS
    raw: *mut types::XCBSetup
}

pub fn xcb_connect(displayname: Option<&str>, screen: Option<&mut i32>) -> Result<XCBConnection, Box<dyn Error>>
{
    let cstr_displayname;
    let ptr_displayname;
    match displayname
    {
        Some(value) =>
        {
            match CString::new(value)
            {
                Err(e) =>
                {
                    return Err(Box::new(e));
                },
                Ok(value) =>
                {
                    cstr_displayname = value;
                    ptr_displayname = cstr_displayname.as_ptr();
                }
            }
        },
        None =>
        {
            ptr_displayname = null::<std::os::raw::c_char>();
        }
    }

    let ptr_screen;
    match screen
    {
        None =>
        {
            ptr_screen = null_mut::<std::os::raw::c_int>();
        },
        Some(value) =>
        {
            ptr_screen = value as *mut std::os::raw::c_int;
        }
    }

    let connection = unsafe
    {
        super::xcb_connect(ptr_displayname, ptr_screen)
    };

    Ok(
        XCBConnection
        {
            raw: connection
        }
    )
}

pub fn xcb_disconnect(connection: XCBConnection)
{
    unsafe
    {
        super::xcb_disconnect(connection.raw)
    }
}

pub fn xcb_generate_id(connection: &XCBConnection) -> u32
{
    unsafe
    {
        super::xcb_generate_id(connection.raw)
    }
}

pub fn xcb_create_window(connection: &XCBConnection,
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
    value_list: Option<&[u32]>) -> super::XCBVoidCookie
{
    let value_list_ptr: *const u32;
    match value_list
    {
        None => {
            value_list_ptr = null();
        },
        Some(value) => {
            let value_list_vec = Vec::from(value);
            value_list_ptr = value_list_vec.as_ptr();
            std::mem::forget(value_list_vec);
        }
    }

    unsafe
    {
        super::xcb_create_window(connection.raw,
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
            value_list_ptr)
    }
}

pub fn xcb_get_setup(connection: &XCBConnection) -> XCBSetup
{
    let xcb_setup_ptr = unsafe 
    {
        super::xcb_get_setup(connection.raw)
    };

    XCBSetup
    {
        raw: xcb_setup_ptr as *mut types::XCBSetup
    }
}

pub fn xcb_setup_roots_iterator(setup: &XCBSetup) -> super::XCBScreenIterator
{
    unsafe
    {
        super::xcb_setup_roots_iterator(setup.raw)
    }
}

pub fn xcb_map_window(connection: &XCBConnection, window: types::XCBWindow) -> super::XCBVoidCookie
{
    unsafe
    {
        super::xcb_map_window(connection.raw, window)
    }
}

pub fn xcb_flush(connection: &XCBConnection) -> i32
{
    unsafe
    {
        super::xcb_flush(connection.raw)
    }
}

pub fn xcb_poll_for_event(connection: &XCBConnection) -> Option<Box<super::XCBGenericEvent>>
{
    unsafe
    {
        let event = super::xcb_poll_for_event(connection.raw);
        if event.is_null()
        {
            return None;
        }
        else
        {
            return Some(Box::from_raw(event));
        }
    }
}

pub fn xcb_wait_for_event(connection: &XCBConnection) -> Option<Box<super::XCBGenericEvent>>
{
    unsafe
    {
        let event = super::xcb_wait_for_event(connection.raw);
        if event.is_null()
        {
            return None;
        }
        else
        {
            return Some(Box::from_raw(event));
        }
    }
}

pub fn xcb_destroy_window(connection: &XCBConnection, window: types::XCBWindow) -> super::XCBVoidCookie
{
    unsafe
    {
        super::xcb_destroy_window(connection.raw, window)
    }
}

pub fn xcb_intern_atom(connection: &XCBConnection, only_if_exists: bool, name: &str) -> Result<super::XCBInternAtomCookie, std::ffi::NulError>
{
        match CString::new(name)
        {
            Err(err) =>
            {
                return Err(err);
            },
            Ok(cstr) =>
            {
                let cookie = unsafe
                {
                    super::xcb_intern_atom(connection.raw,
                        if only_if_exists {1} else {0},
                        name.chars().count() as std::os::raw::c_ushort,
                        cstr.as_ptr())
                };
                return Ok(cookie);
            }
        }
}

pub fn xcb_intern_atom_reply(connection: &XCBConnection,
    cookie: super::XCBInternAtomCookie,
    e: Option<*mut *mut super::XCBGenericError>) -> Box<super::XCBInternAtomReply>
{
    let e_ptr = match e
    {
        None =>
        {
            std::ptr::null_mut::<*mut super::XCBGenericError>()
        },
        Some(value) =>
        {
            value
        }
    };

    unsafe
    {
        Box::from_raw(super::xcb_intern_atom_reply(connection.raw, cookie, e_ptr))
    }
}

pub fn xcb_change_property(connection: &XCBConnection,
    mode: std::os::raw::c_uchar,
    window: types::XCBWindow,
    property: super::XCBAtom,
    property_type: super::XCBAtom,
    format: std::os::raw::c_uchar,
    data_len: std::os::raw::c_uint,
    data: *const std::os::raw::c_void) -> super::XCBVoidCookie
{
    unsafe
    {
        super::xcb_change_property(connection.raw, mode, window, property, property_type, format, data_len, data)
    }
}

type XCBGetAtomNameCookie = super::XCBGetAtomNameCookie;

pub fn xcb_get_atom_name(connection: &XCBConnection,
    atom: super::XCBAtom) -> XCBGetAtomNameCookie
{
    unsafe
    {
        super::xcb_get_atom_name(connection.raw, atom)
    }
}

type XCBGetAtomNameReply = super::XCBGetAtomNameReply;

pub fn xcb_get_atom_name_reply(connection: &XCBConnection,
    cookie: XCBGetAtomNameCookie,
    e: Option<*mut *mut super::XCBGenericError>) -> *mut XCBGetAtomNameReply
{
    unsafe
    {
        match e
        {
            None =>
            {
                return super::xcb_get_atom_name_reply(connection.raw, cookie, null_mut::<*mut super::XCBGenericError>());
            },
            Some(e) =>
            {
                return super::xcb_get_atom_name_reply(connection.raw, cookie, e);
            }
        }
    }
}

pub fn xcb_create_colormap(connection: &XCBConnection, alloc: u8, mid: XCBColormap, window: XCBWindow, visual: XCBVisualId) -> XCBVoidCookie
{
    unsafe
    {
        super::xcb_create_colormap(connection.raw, alloc, mid, window, visual)
    }
}