#![no_std]
#![no_main]

use cstr_core::{cstr, CStr};

use hw_kolibri::{Color, Dot, Event, WindowKind, WindowParams, WindowTextParams};

const HEADER: &CStr = cstr!("Hey Kolibri");
const MSG: &str = "Hello from Rust!";

#[inline(always)] // for some reason function removed otherwise
fn draw_window() {
    unsafe {
        hw_kolibri::start_window_draw();
        hw_kolibri::define_window(
            Dot { x: 50, y: 50 },
            300,
            400,
            WindowParams {
                color: Color::rgb(0xff, 0xff, 0xff),
                kind: WindowKind::Themed,
                title: Some(HEADER),
            },
        );
        hw_kolibri::display_message(
            Dot { x: 0, y: 10 },
            WindowTextParams {
                color: Color::rgb(0x66, 0x22, 0x22),
                text: MSG,
                bg_color: None,
            },
        );
        hw_kolibri::end_window_draw();
    }
}

#[no_mangle]
fn kol_main() -> ! {
    draw_window();

    while let Some(ev) = hw_kolibri::fetch_event() {
        match ev {
            Event::Redraw => draw_window(),
            Event::KeyPress => drop(hw_kolibri::fetch_key()),
            _ => break,
        }
    }

    hw_kolibri::exit();
}
