// Include the generated bindings
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern crate libc;
use std::ffi::CString;
use std::ptr;

extern "C" {
    fn compute_end_effector_pose();
}

fn main() {
    // Path to your URDF file
    //let urdf_path = "/models/baxter_simple.urdf";
    //let c_message = CString::new(urdf_path).expect("CString::new failed");

    // Call the C++ function to compute the end-effector pose
    unsafe {
        compute_end_effector_pose();
    }

    // The output will be printed in the C++ function itself
}

