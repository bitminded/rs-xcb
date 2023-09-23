extern crate xcb;

use xcb::{
    xcb_connect, xcb_create_window, xcb_disconnect, xcb_flush, xcb_generate_id, xcb_get_setup,
    xcb_map_window, xcb_screen, xcb_setup_roots_iterator,
};

fn main() {
    let connection = match xcb_connect(None, None) {
        Ok(connection) => connection,
        Err(_) => panic!("Couldn't establish connection to X11 Server"),
    };

    let setup = xcb_get_setup(&connection);
    let iter = xcb_setup_roots_iterator(&setup);
    let screen = xcb_screen(&iter);
    let window_id = xcb_generate_id(&connection);
    xcb_create_window(
        &connection,
        xcb::constants::XCB_COPY_FROM_PARENT as u8,
        window_id,
        screen.root,
        0,
        0,
        150,
        150,
        10,
        xcb::constants::XCB_WINDOW_CLASS_INPUT_OUTPUT,
        screen.root_visual,
        0,
        None,
    );
    xcb_map_window(&connection, window_id);
    xcb_flush(&connection);
    loop {}
    xcb_disconnect(connection);
}
