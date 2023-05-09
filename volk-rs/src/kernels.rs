use crate::{types::*, vec::AlignedVec};
use volk_sys::std_complex;

pub fn v16i_32fc_dot_prod_32fc(result: &mut complex<f32>, input: &mut AlignedVec<core::ffi::c_short>, taps: &mut AlignedVec<complex<f32>>) {
    assert!(input.len() == taps.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_16i_32fc_dot_prod_32fc.unwrap_unchecked()(
            result as *mut complex<f32> as *mut std_complex<f32>,
            input.as_mut() as *mut core::ffi::c_short,
            taps.as_mut() as *mut complex<f32> as *mut std_complex<f32>,
            taps.len() as core::ffi::c_uint,
        );
    }
}
