#![allow(unused)]

#[repr(C, packed)]
pub struct Env {
    env_ver: i32,
    ver_maj: i32,
    ver_min: i32,
    ver_patch: i32,
    framebuffer_address: usize,
    display_address: usize,
    display_width: usize,
    display_height: usize,
    display_bpp: usize,
    display_pitch: usize,
    display_size: usize,
    timer_ticks: usize,
    ram_available: usize,
    ram_used: usize,
    ram_free: usize,
}

unsafe extern "C" {
    fn _syscall(nr: u32, p1: u32, p2: u32, p3: u32) -> u32;
}

#[inline]
pub unsafe extern "C" fn syscall(nr: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    unsafe {
        _syscall(nr, p1, p2, p3)
    }
}

pub unsafe extern "C" fn get_env() -> Env {
    unsafe {
        let mut env = core::mem::zeroed::<Env>();

        syscall(0, (&mut env as *mut Env).addr() as _, 0, 0);

        env
    }
}

pub unsafe extern "C" fn memory_alloc(size: usize, align: usize) -> usize {
    unsafe {
        let mut addr = 0usize;

        syscall(1, size as _, align as _, (&mut addr as *mut usize).addr() as _);

        addr
    }
}

pub unsafe extern "C" fn memory_free(addr: usize) {
    unsafe {
        syscall(2, addr as _, 0, 0);
    }
}

pub unsafe extern "C" fn get_key() -> usize {
    unsafe {
        syscall(13, 0, 0, 0) as usize
    }
}
