void start_window_draw() {
    __asm__ volatile (
        "int $0x40;"
        :: "a"(12), "b"(1)
    );
}

void end_window_draw() {
    __asm__ volatile (
        "int $0x40;"
        :: "a"(12), "b"(2)
    );
}

void exit0() {
    __asm__ volatile (
        "int $0x40;"
        :: "a"(-1)
    );
}

void define_window(unsigned ebx, unsigned ecx, unsigned edx, unsigned edi) {
    __asm__ volatile (
        "int $0x40;"
        :: "a"(0), "b"(ebx), "c"(ecx), "d"(edx), "D"(edi)
        : "memory"
    );
}

void display_message(unsigned ebx, unsigned ecx, unsigned edx, unsigned edi, unsigned esi) {
    __asm__ volatile (
        "int $0x40;"
        :: "a"(4), "b"(ebx), "c"(ecx), "d"(edx), "D"(edi), "S"(esi)
        : "memory"
    );
}

unsigned wait_event() {
    unsigned res;
    __asm__ volatile (
        "int $0x40;"
        : "=r"(res) : "a"(10)
    );
    return res;
}

unsigned pressed_key() {
    unsigned res;
    __asm__ volatile (
        "int $0x40;"
        : "=r"(res) : "a"(2)
    );
    return res;
}
