use std::os::raw::{c_uchar, c_ushort, c_uint, c_long};

pub const XCB_COPY_FROM_PARENT: c_long = 0;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: c_ushort = 1;

pub const XCB_CW_EVENT_MASK: c_uint = 2048;
pub const XCB_CW_COLORMAP: c_uint = 8192;

// event mask values
pub const XCB_EVENT_MASK_NO_EVENT: c_uint = 0;
pub const XCB_EVENT_MASK_KEY_PRESS: c_uint = 1;
pub const XCB_EVENT_MASK_KEY_RELEASE: c_uint = 2;
pub const XCB_EVENT_MASK_BUTTON_PRESS: c_uint = 4;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: c_uint = 8;
pub const XCB_EVENT_MASK_ENTER_WINDOW: c_uint = 16;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: c_uint = 32;
pub const XCB_EVENT_MASK_POINTER_MOTION: c_uint = 64;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: c_uint = 128;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: c_uint = 256;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: c_uint = 512;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: c_uint = 1024;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: c_uint = 2048;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: c_uint = 4096;
pub const XCB_EVENT_MASK_BUTTON_MOTION: c_uint = 8192;
pub const XCB_EVENT_MASK_KEYMAP_STATE: c_uint = 16384;
pub const XCB_EVENT_MASK_EXPOSURE: c_uint = 32768;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: c_uint = 65536;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: c_uint = 131072;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: c_uint = 262144;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: c_uint = 524288;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: c_uint = 1048576;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: c_uint = 2097152;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: c_uint = 4194304;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: c_uint = 8388608;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: c_uint = 16777216;

pub const XCB_ENTER_NOTIFY: c_uchar = 7;
pub const XCB_EXPOSE: c_uchar = 12;
pub const XCB_VISIBILITY_NOTIFY: c_uchar = 15;
pub const XCB_DESTROY_NOTIFY: c_uchar = 17;
pub const XCB_UNMAP_NOTIFY: c_uchar = 18;
pub const XCB_PROPERTY_NOTIFY: c_uchar = 28;
pub const XCB_CLIENT_MESSAGE: c_uchar = 33;

pub const XCB_COLORMAP_ALLOC_NONE: c_uchar = 0;
pub const XCB_COLORMAP_ALLOC_ALL: c_uchar = 1;

// property change modes
pub const XCB_PROP_MODE_REPLACE: c_uchar = 0;
pub const XCB_PROP_MODE_PREPEND: c_uchar = 1;
pub const XCB_PROP_MODE_APPEND: c_uchar = 2;
