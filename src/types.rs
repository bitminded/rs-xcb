use std::os::raw::{ c_uint, c_void};

pub type XCBSetup = c_void;
pub type XCBWindow = c_uint;
pub type XCBTimestamp = c_uint;
pub type XCBColormap = c_uint;
pub type XCBVisualId = c_uint;