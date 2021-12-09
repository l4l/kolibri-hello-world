#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }

    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, x: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *s.add(i) = x as u8;
    }

    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *mut u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let x = *s1.add(i);
        let y = *s2.add(i);
        if x == y {
            continue;
        }

        let (x, y) = (i32::from(x), i32::from(y));
        return x - y;
    }

    0
}
