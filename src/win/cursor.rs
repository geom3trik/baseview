
use std::os::raw::c_char;

use winapi::shared::windef::HCURSOR;

use winapi::um::winuser::{LoadCursorW, IDC_HAND, IDC_HAND};

use crate::MouseCursor;

use std::ptr::null_mut;

/*
fn create_empty_cursor(display: *mut x11::xlib::Display) -> Option<u32> {
    let data = 0;
    let pixmap = unsafe {
        let screen = x11::xlib::XDefaultScreen(display);
        let window = x11::xlib::XRootWindow(display, screen);
        x11::xlib::XCreateBitmapFromData(display, window, &data, 1, 1)
    };

    if pixmap == 0 {
        return None;
    }

    unsafe {
        // We don't care about this color, since it only fills bytes
        // in the pixmap which are not 0 in the mask.
        let mut color: x11::xlib::XColor = std::mem::zeroed();

        let cursor = x11::xlib::XCreatePixmapCursor(
            display,
            pixmap,
            pixmap,
            &mut color as *mut _,
            &mut color as *mut _,
            0,
            0,
        );
        x11::xlib::XFreePixmap(display, pixmap);

        Some(cursor as u32)
    }
}

fn load_cursor(display: *mut x11::xlib::Display, name: &[u8]) -> Option<u32> {
    let xcursor = unsafe {
        x11::xcursor::XcursorLibraryLoadCursor(display, name.as_ptr() as *const c_char)
    };

    if xcursor == 0 {
        None
    } else {
        Some(xcursor as u32)
    }
}

fn load_first_existing_cursor(display: *mut x11::xlib::Display, names: &[&[u8]]) -> Option<u32> {
    names.iter()
        .map(|name| load_cursor(display, name))
        .find(|xcursor| xcursor.is_some())
        .unwrap_or(None)
}

*/
pub(super) fn get_cursor(cursor: MouseCursor) -> HCURSOR {
    //let load = |name: &[u8]| load_cursor(display, name);
    //let loadn = |names: &[&[u8]]| load_first_existing_cursor(display, names);

    let cursor = match cursor {
        MouseCursor::Default => None, // catch this in the fallback case below

        MouseCursor::Hand => LoadCursorW(null_mut(), IDC_HAND),
        MouseCursor::HandGrabbing => None,
        MouseCursor::Help => LoadCursorW(null_mut(), IDC_HELP),

        MouseCursor::Hidden => None,

        MouseCursor::Text => LoadCursorW(null_mut(), IDC_IBEAM),
        MouseCursor::VerticalText => None,

        MouseCursor::Working => LoadCursorW(null_mut(), IDC_WAIT),
        MouseCursor::PtrWorking => None,

        MouseCursor::NotAllowed => LoadCursorW(null_mut(), IDC_NO),
        MouseCursor::PtrNotAllowed => None,

        MouseCursor::ZoomIn => None,
        MouseCursor::ZoomOut => None,

        MouseCursor::Alias => None,
        MouseCursor::Copy => None,
        MouseCursor::Move => LoadCursorW(null_mut(), IDC_SIZEALL),
        MouseCursor::AllScroll => None,
        MouseCursor::Cell => None,
        MouseCursor::Crosshair => LoadCursorW(null_mut(), IDC_CROSS),

        MouseCursor::EResize => LoadCursorW(null_mut(), IDC_SIZEWE),
        MouseCursor::NResize => LoadCursorW(null_mut(), IDC_SIZENS),
        MouseCursor::NeResize => LoadCursorW(null_mut(), IDC_SIZENESW),
        MouseCursor::NwResize => LoadCursorW(null_mut(), IDC_SIZENWSE),
        MouseCursor::SResize => LoadCursorW(null_mut(), IDC_SIZENS),
        MouseCursor::SeResize => LoadCursorW(null_mut(), IDC_SIZENWSE),
        MouseCursor::SwResize => LoadCursorW(null_mut(), IDC_SIZENESW),
        MouseCursor::WResize => LoadCursorW(null_mut(), IDC_SIZEWE),
        MouseCursor::EwResize => LoadCursorW(null_mut(), IDC_SIZEWE),
        MouseCursor::NsResize => LoadCursorW(null_mut(), IDC_SIZENS),
        MouseCursor::NwseResize => LoadCursorW(null_mut(), IDC_SIZENWSE),
        MouseCursor::NeswResize => LoadCursorW(null_mut(), IDC_SIZENESW),
        MouseCursor::ColResize => LoadCursorW(null_mut(), IDC_SIZEWE),
        MouseCursor::RowResize => LoadCursorW(null_mut(), IDC_SIZENS),
    };

    cursor
        .or(LoadCursorW(null_mut(), IDC_HAND))
        .unwrap_or(0)
}