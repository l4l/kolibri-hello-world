#[link(name = "syscalls")]
extern "C" {
    pub fn start_window_draw();

    pub fn end_window_draw();

    #[link_name = "exit0"]
    pub fn exit() -> !;

    pub fn define_window(ebx: u32, ecx: u32, edx: u32, edi: u32);
    pub fn display_message(ebx: u32, ecx: u32, edx: u32, edi: u32, esi: u32);

    pub fn wait_event() -> u32;

    pub fn pressed_key() -> u32;
}
