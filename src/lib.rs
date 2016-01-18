#![allow(dead_code, non_snake_case)]

use std::mem;
extern crate libc;
use libc::{ c_int, c_double, int64_t, int16_t, c_void };

#[repr(C)]
struct RawOsnContext {
    perm: *const int64_t,
    permGradIndex3D: *const int64_t,
}

#[link(name = "opensimplex")]
extern {
    fn open_simplex_noise(seed: int64_t, ctx: *mut *mut RawOsnContext) -> c_int;
    fn open_simplex_noise_free(ctx: *mut RawOsnContext) -> c_void;
    fn open_simplex_noise_init_perm(ctx: *mut RawOsnContext, p: *const int16_t, nelements: c_int) -> c_int;
    fn open_simplex_noise2(ctx: *const RawOsnContext, x: c_double, y: c_double) -> c_double;
    fn open_simplex_noise3(ctx: *const RawOsnContext, x: c_double, y: c_double, z: c_double) -> c_double;
    fn open_simplex_noise4(ctx: *const RawOsnContext, x: c_double, y: c_double, z: c_double, w: c_double) -> c_double;
}

pub struct OsnContext {
    ctx: *mut RawOsnContext,
}

impl Drop for OsnContext {
    fn drop(&mut self) {
        unsafe { open_simplex_noise_free(self.ctx) };
    }
}

impl OsnContext {
    fn new(seed: i64) -> Option<OsnContext> {
        let mut ctx: *mut RawOsnContext = unsafe { mem::uninitialized() };
        let res = unsafe { open_simplex_noise(seed, &mut ctx as *mut *mut RawOsnContext) };
        if res == 0 {
            Some(OsnContext {
                ctx: ctx,
            })
        } else {
            None
        }
    }

    unsafe fn as_c_arg(&self) -> *mut RawOsnContext {
        self.ctx as *mut RawOsnContext
    }

    fn noise2(&self, x: f64, y: f64) -> f64 {
        unsafe { open_simplex_noise2(self.as_c_arg(), x, y) }
    }
    fn noise3(&self, x: f64, y: f64, z: f64) -> f64 {
        unsafe { open_simplex_noise3(self.as_c_arg(), x, y, z) }
    }
    fn noise4(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        unsafe { open_simplex_noise4(self.as_c_arg(), x, y, z, w) }
    }
}

#[test]
fn test_basics() {
    let ctx = OsnContext::new(123).unwrap();
    let ctx2 = OsnContext::new(122).unwrap();

    assert!(ctx.noise2(1.0, 1.0) != ctx2.noise2(1.0, 1.0));
    assert!(ctx.noise2(1.0, 1.0) == ctx.noise2(1.0, 1.0));

    assert!(ctx.noise3(1.0, 1.0, 1.0) != ctx2.noise3(1.0, 1.0, 1.0));
    assert!(ctx.noise3(1.0, 1.0, 1.0) == ctx.noise3(1.0, 1.0, 1.0));

    assert!(ctx.noise4(1.0, 1.0, 1.0, 1.0) != ctx2.noise4(1.0, 1.0, 1.0, 1.0));
    assert!(ctx.noise4(1.0, 1.0, 1.0, 1.0) == ctx.noise4(1.0, 1.0, 1.0, 1.0));
}
