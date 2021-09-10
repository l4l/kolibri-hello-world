#![feature(asm)]
#![no_std]

macro_rules! syscall {
    ($fn_no:literal) => {{
        let mut res: i32 = $fn_no;

        asm!("int 0x40", inout("eax") res);
        res
    }};
    ($fn_no:literal, $($reg:tt)*) => {{
        let mut res: i32 = $fn_no;
        asm!("int 0x40",
            $($reg)*,
            inout("eax") res,
        );
        res
    }}
}

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }
    pub fn g(&self) -> u8 {
        self.1
    }
    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn as_rgb_val(self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.0 as u32)
    }
}

#[inline]
pub unsafe fn start_window_draw() {
    syscall!(12, in("ebx") 1);
}

pub struct Dot {
    pub x: u32,
    pub y: u32,
}

#[repr(u32)]
pub enum WindowKind {
    Fixed = 0,
    NoDraw = 1,
    Resizable = 2,
    Themed = 3,
    FixedThemed = 4,
}

pub struct WindowParams<'a> {
    pub color: Color,
    pub kind: WindowKind,
    pub title: Option<&'a cstr_core::CStr>,
}

#[inline]
pub unsafe fn define_window(start: Dot, width: u32, height: u32, params: WindowParams<'_>) {
    const RELATIVE_FLAG: u32 = 0x20;
    syscall!(
        0,
        in("ebx") start.x * 65536 + width,
        in("ecx") start.y * 65536 + height,
        in("edx") params.color.as_rgb_val() |
            (RELATIVE_FLAG | (params.title.is_some() as u32) << 4 | params.kind as u32) << 24,
        in("edi") params.title.map(|s| s.as_ptr()).unwrap_or_else(core::ptr::null)
    );
}

pub struct WindowTextParams<'a> {
    pub color: Color,
    pub text: &'a str,
    pub bg_color: Option<Color>,
}

#[inline]
pub unsafe fn display_message(start: Dot, params: WindowTextParams<'_>) {
    const UTF8_FLAG: u32 = 0b0011_0000 << 24;
    const BG_FLAG: u32 = 0b0100_0000 << 24;
    // XXX: llvm uses esi internally for x86-32
    //      hope we're lucky enough so it won't be overwritten.
    asm!("mov esi, {}", in(reg) params.text.len());
    syscall!(
        4,
        in("ebx") start.x * 65536 + start.y,
        in("ecx") params.color.as_rgb_val() | BG_FLAG * params.bg_color.is_some() as u32 | UTF8_FLAG,
        in("edx") params.text.as_ptr()
    );
}

#[inline]
pub unsafe fn end_window_draw() {
    syscall!(12, in("ebx") 2);
}

pub fn exit() -> ! {
    unsafe { syscall!(-1) };

    unsafe { core::hint::unreachable_unchecked() }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit();
}

#[non_exhaustive]
pub enum Event {
    Redraw,
    KeyPress,
}

#[inline]
pub fn fetch_event() -> Option<Event> {
    match unsafe { syscall!(10) } {
        1 => Some(Event::Redraw),
        2 => Some(Event::KeyPress),
        _ => None,
    }
}

#[inline]
pub fn fetch_key() -> Option<u8> {
    let res = unsafe { syscall!(2) };
    if res == 1 {
        None
    } else {
        Some(((res >> 8) & 0xff) as u8)
    }
}
