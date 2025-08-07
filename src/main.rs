use std::ffi::c_double;
use std::os::raw::c_int;
//use std::ptr;

//#[repr(C)]
pub enum PinHandle{}

extern "C" {
    fn pin_create_manipulator() -> *mut PinHandle;
    fn pin_destroy(h: *mut PinHandle);
    fn pin_get_nv(h: *mut PinHandle) -> c_int;
    fn pin_set_state(h: *mut PinHandle,
                     q: *const c_double,
                     v: *const c_double,
                     a: *const c_double);
    fn pin_compute_rnea(h: *mut PinHandle,
                        tau_out: *mut c_double);
}

fn main() {
    unsafe {
        // Create sample manipulator model
        let handle = pin_create_manipulator();
        if handle.is_null() {
            panic!("Failed to create Pinocchio handle");
        }

        // Generalized velocity dimension (nv)
        let nv = pin_get_nv(handle) as usize;

        // Allocate q,v,a,tau
        let mut q  = vec![0.0f64; nv];
        let mut v  = vec![0.0f64; nv];
        let mut a  = vec![0.0f64; nv];
        let mut tau = vec![0.0f64; nv];

        // (Optional) set q,v,a
        // here we just leave them zeros => q=neutral, v=a=0
        pin_set_state(handle,
                      q.as_ptr(),
                      v.as_ptr(),
                      a.as_ptr());

        // Compute tau = rnea(...)
        pin_compute_rnea(handle, tau.as_mut_ptr());

        println!("tau = {:?}", tau);

        // Clean up
        pin_destroy(handle);
    }
}

