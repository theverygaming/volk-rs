use num_complex::Complex;
use volk_sys::std_complex;

pub fn volk_16i_32fc_dot_prod_32fc(input: &[core::ffi::c_short], output: &mut Complex<f32>, taps: &mut [Complex<f32>]) {
    assert!(input.len() == taps.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_16i_32fc_dot_prod_32fc.unwrap_unchecked()(
            output as *mut Complex<f32> as *mut std_complex<f32>,
            input.as_ptr(),
            taps.as_mut_ptr() as *mut std_complex<f32>,
            taps.len() as core::ffi::c_uint,
        );
    }
}

// TODO: volk_16i_branch_4_state_8
// TODO: volk_16ic_convert_32fc
// TODO: volk_16ic_deinterleave_16i_x2
// TODO: volk_16ic_deinterleave_real_16i
// TODO: volk_16ic_deinterleave_real_8i
// TODO: volk_16ic_magnitude_16i
// TODO: volk_16i_convert_8i
// TODO: volk_16ic_s32f_deinterleave_32f_x2
// TODO: volk_16ic_s32f_deinterleave_real_32f
// TODO: volk_16ic_s32f_magnitude_32f
// TODO: volk_16ic_x2_dot_prod_16ic
// TODO: volk_16ic_x2_multiply_16ic
// TODO: volk_16i_max_star_16i
// TODO: volk_16i_max_star_horizontal_16i
// TODO: volk_16i_permute_and_scalar_add

pub fn volk_16i_s32f_convert_32f(input: &[i16], output: &mut [f32], scalar: f32) {
    assert!(output.len() == input.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_16i_s32f_convert_32f.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr(), scalar, output.len() as u32);
    }
}

pub fn volk_16i_s32f_convert_32f_u8(input: &[u8], output: &mut [f32], scalar: f32) {
    assert!(input.len() % 2 == 0, "invalid length");
    assert!(output.len() == (input.len() / 2), "mismatched lengths");

    unsafe {
        volk_sys::volk_16i_s32f_convert_32f.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr() as *mut i16, scalar, output.len() as u32);
    }
}

// TODO: volk_16i_x4_quad_max_star_16i
// TODO: volk_16i_x5_add_quad_16i_x4

pub fn volk_16u_byteswap(vector: &mut [u16]) {
    unsafe {
        volk_sys::volk_16u_byteswap.unwrap_unchecked()(vector.as_mut_ptr(), vector.len() as u32);
    }
}

pub fn volk_16u_byteswap_u8(vector: &mut [u8]) {
    assert!(vector.len() % 2 == 0, "invalid length");

    unsafe {
        volk_sys::volk_16u_byteswap.unwrap_unchecked()(vector.as_mut_ptr() as *mut u16, (vector.len() / 2) as u32);
    }
}

// TODO: volk_32f_64f_add_64f
// TODO: volk_32f_64f_multiply_64f
// TODO: volk_32f_8u_polarbutterfly_32f

pub enum GenericFloatVector<'a> {
    F32(&'a[f32]),
    FC32(&'a[Complex<f32>]),
}

pub fn volk_32f_accumulator_s32f(input: GenericFloatVector, output: &mut f32) {
    unsafe {
        match input {
            GenericFloatVector::F32(inp) => {
                volk_sys::volk_32f_accumulator_s32f.unwrap_unchecked()(
                    output,
                    inp.as_ptr() as *const f32,
                    inp.len() as core::ffi::c_uint,
                );
            },
            GenericFloatVector::FC32(inp) => {
                volk_sys::volk_32f_accumulator_s32f.unwrap_unchecked()(
                    output,
                    inp.as_ptr() as *const f32,
                    (inp.len() as core::ffi::c_uint) * 2,
                );
            },
        }
    }
}

// TODO: volk_32f_acos_32f
// TODO: volk_32f_asin_32f
// TODO: volk_32f_atan_32f
// TODO: volk_32f_binary_slicer_32i
// TODO: volk_32f_binary_slicer_8i
// TODO: volk_32fc_32f_add_32fc

pub fn volk_32fc_32f_dot_prod_32fc(input: &[Complex<f32>], output: &mut Complex<f32>, taps: &[f32]) {
    assert!(input.len() == taps.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32fc_32f_dot_prod_32fc.unwrap_unchecked()(
            output as *mut Complex<f32> as *mut std_complex<f32>,
            input.as_ptr() as *const std_complex<f32>,
            taps.as_ptr(),
            input.len() as core::ffi::c_uint,
        );
    }
}

pub fn volk_32fc_32f_multiply_32fc(input: &[Complex<f32>], output: &mut [Complex<f32>], input_f: &[f32]) {
    assert!(input.len() == output.len(), "mismatched lengths");
    assert!(input.len() == input_f.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32fc_32f_multiply_32fc.unwrap_unchecked()(
            output.as_mut_ptr() as *mut std_complex<f32>,
            input.as_ptr() as *const std_complex<f32>,
            input_f.as_ptr(),
            input.len() as core::ffi::c_uint,
        );
    }
}

// TODO: volk_32fc_accumulator_s32fc
// TODO: volk_32fc_conjugate_32fc
// TODO: volk_32fc_convert_16ic
// TODO: volk_32fc_deinterleave_32f_x2
// TODO: volk_32fc_deinterleave_64f_x2
// TODO: volk_32fc_deinterleave_imag_32f
// TODO: volk_32fc_deinterleave_real_32f
// TODO: volk_32fc_deinterleave_real_64f
// TODO: volk_32fc_index_max_16u
// TODO: volk_32fc_index_max_32u
// TODO: volk_32fc_index_min_16u
// TODO: volk_32fc_index_min_32u

pub fn volk_32fc_magnitude_32f(magnitude_vector: &mut [f32], complex_vector: &[Complex<f32>]) {
    assert!(magnitude_vector.len() == complex_vector.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32fc_magnitude_32f.unwrap_unchecked()(
            magnitude_vector.as_mut_ptr(),
            complex_vector.as_ptr() as *const std_complex<f32>,
            magnitude_vector.len() as core::ffi::c_uint,
        );
    }
}

// TODO: volk_32fc_magnitude_squared_32f
// TODO: volk_32f_convert_64f
// TODO: volk_32f_cos_32f
// TODO: volk_32fc_s32f_atan2_32f
// TODO: volk_32fc_s32fc_multiply_32fc

#[deprecated(since="0.0.2", note="replaced by `volk_32fc_s32fc_x2_rotator2_32fc`")]
pub fn volk_32fc_s32fc_x2_rotator_32fc(input: &[Complex<f32>], output: &mut [Complex<f32>], phase_inc: Complex<f32>, phase: &mut Complex<f32>) {
    volk_32fc_s32fc_x2_rotator2_32fc(input, output, &phase_inc, phase);
}

pub fn volk_32fc_s32fc_x2_rotator2_32fc(input: &[Complex<f32>], output: &mut [Complex<f32>], phase_inc: &Complex<f32>, phase: &mut Complex<f32>) {
    assert!(input.len() == output.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32fc_s32fc_x2_rotator2_32fc.unwrap_unchecked()(
            output.as_mut_ptr() as *mut std_complex<f32>,
            input.as_ptr() as *const std_complex<f32>,
            phase_inc as *const Complex<f32> as *const std_complex<f32>,
            phase as *mut Complex<f32> as *mut std_complex<f32>,
            input.len() as core::ffi::c_uint,
        );
    }
}

// TODO: volk_32fc_s32f_deinterleave_real_16i
// TODO: volk_32fc_s32f_magnitude_16i
// TODO: volk_32fc_s32f_power_32fc
// TODO: volk_32fc_s32f_power_spectrum_32f
// TODO: volk_32fc_s32f_x2_power_spectral_density_32f
// TODO: volk_32fc_x2_add_32fc
// TODO: volk_32fc_x2_conjugate_dot_prod_32fc
// TODO: volk_32fc_x2_divide_32fc
// TODO: volk_32fc_x2_dot_prod_32fc
// TODO: volk_32fc_x2_multiply_32fc
// TODO: volk_32fc_x2_multiply_conjugate_32fc
// TODO: volk_32fc_x2_s32fc_multiply_conjugate_add_32fc
// TODO: volk_32fc_x2_s32f_square_dist_scalar_mult_32f
// TODO: volk_32fc_x2_square_dist_32f
// TODO: volk_32f_exp_32f
// TODO: volk_32f_expfast_32f
// TODO: volk_32f_index_max_16u

pub fn volk_32f_index_max_32u(input: &[f32], output: &mut u32) {
    unsafe {
        volk_sys::volk_32f_index_max_32u.unwrap_unchecked()(
            output,
            input.as_ptr(),
            input.len() as u32
        );
    }
}

// TODO: volk_32f_index_min_16u
// TODO: volk_32f_index_min_32u
// TODO: volk_32f_invsqrt_32f
// TODO: volk_32f_log2_32f
// TODO: volk_32f_s32f_32f_fm_detect_32f
// TODO: volk_32f_s32f_add_32f
// TODO: volk_32f_s32f_calc_spectral_noise_floor_32f

pub fn volk_32f_s32f_convert_16i(input: &[f32], output: &mut [i16], scalar: f32) {
    assert!(output.len() == input.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32f_s32f_convert_16i.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr(), scalar, output.len() as u32);
    }
}

pub fn volk_32f_s32f_convert_16i_u8(input: &[f32], output: &mut [u8], scalar: f32) {
    assert!(output.len() % 2 == 0, "invalid length");
    assert!(input.len() == (output.len() / 2), "mismatched lengths");

    unsafe {
        volk_sys::volk_32f_s32f_convert_16i.unwrap_unchecked()(output.as_mut_ptr() as *mut i16, input.as_ptr(), scalar, input.len() as u32);
    }
}

pub fn volk_32f_s32f_convert_32i(input: &[f32], output: &mut [i32], scalar: f32) {
    assert!(output.len() == input.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32f_s32f_convert_32i.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr(), scalar, output.len() as u32);
    }
}

pub fn volk_32f_s32f_convert_32i_u8(input: &[f32], output: &mut [u8], scalar: f32) {
    assert!(output.len() % 4 == 0, "invalid length");
    assert!(input.len() == (output.len() / 4), "mismatched lengths");

    unsafe {
        volk_sys::volk_32f_s32f_convert_32i.unwrap_unchecked()(output.as_mut_ptr() as *mut i32, input.as_ptr(), scalar, input.len() as u32);
    }
}

// TODO: volk_32f_s32f_convert_8i
// TODO: volk_32f_s32f_multiply_32f
// TODO: volk_32f_s32f_normalize
// TODO: volk_32f_s32f_power_32f
// TODO: volk_32f_s32f_s32f_mod_range_32f
// TODO: volk_32f_s32f_stddev_32f
// TODO: volk_32f_sin_32f
// TODO: volk_32f_sqrt_32f
// TODO: volk_32f_stddev_and_mean_32f_x2
// TODO: volk_32f_tan_32f
// TODO: volk_32f_tanh_32f
// TODO: volk_32f_x2_add_32f
// TODO: volk_32f_x2_divide_32f
// TODO: volk_32f_x2_dot_prod_16i
// TODO: volk_32f_x2_dot_prod_32f
// TODO: volk_32f_x2_interleave_32fc
// TODO: volk_32f_x2_max_32f
// TODO: volk_32f_x2_min_32f
// TODO: volk_32f_x2_multiply_32f
// TODO: volk_32f_x2_pow_32f
// TODO: volk_32f_x2_s32f_interleave_16ic
// TODO: volk_32f_x2_subtract_32f
// TODO: volk_32f_x3_sum_of_poly_32f

pub fn volk_32i_s32f_convert_32f(input: &[i32], output: &mut [f32], scalar: f32) {
    assert!(output.len() == input.len(), "mismatched lengths");

    unsafe {
        volk_sys::volk_32i_s32f_convert_32f.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr(), scalar, output.len() as u32);
    }
}

pub fn volk_32i_s32f_convert_32f_u8(input: &[u8], output: &mut [f32], scalar: f32) {
    assert!(input.len() % 4 == 0, "invalid length");
    assert!(output.len() == (input.len() / 4), "mismatched lengths");

    unsafe {
        volk_sys::volk_32i_s32f_convert_32f.unwrap_unchecked()(output.as_mut_ptr(), input.as_ptr() as *mut i32, scalar, output.len() as u32);
    }
}

// TODO: volk_32i_x2_and_32i
// TODO: volk_32i_x2_or_32i

pub fn volk_32u_byteswap(vector: &mut [u32]) {
    unsafe {
        volk_sys::volk_32u_byteswap.unwrap_unchecked()(vector.as_mut_ptr(), vector.len() as u32);
    }
}

pub fn volk_32u_byteswap_u8(vector: &mut [u8]) {
    assert!(vector.len() % 4 == 0, "invalid length");

    unsafe {
        volk_sys::volk_32u_byteswap.unwrap_unchecked()(vector.as_mut_ptr() as *mut u32, (vector.len() / 4) as u32);
    }
}

// TODO: volk_32u_popcnt
// TODO: volk_32u_reverse_32u
// TODO: volk_64f_convert_32f
// TODO: volk_64f_x2_add_64f
// TODO: volk_64f_x2_max_64f
// TODO: volk_64f_x2_min_64f
// TODO: volk_64f_x2_multiply_64f

pub fn volk_64u_byteswap(vector: &mut [u64]) {
    unsafe {
        volk_sys::volk_64u_byteswap.unwrap_unchecked()(vector.as_mut_ptr(), vector.len() as u32);
    }
}

pub fn volk_64u_byteswap_u8(vector: &mut [u8]) {
    assert!(vector.len() % 8 == 0, "invalid length");

    unsafe {
        volk_sys::volk_64u_byteswap.unwrap_unchecked()(vector.as_mut_ptr() as *mut u64, (vector.len() / 8) as u32);
    }
}

// TODO: volk_64u_popcnt
// TODO: volk_8ic_deinterleave_16i_x2
// TODO: volk_8ic_deinterleave_real_16i
// TODO: volk_8ic_deinterleave_real_8i
// TODO: volk_8i_convert_16i
// TODO: volk_8ic_s32f_deinterleave_32f_x2
// TODO: volk_8ic_s32f_deinterleave_real_32f
// TODO: volk_8ic_x2_s32f_multiply_conjugate_32fc
// TODO: volk_8i_s32f_convert_32f
// TODO: volk_8u_x3_encodepolar_8u
// TODO: volk_8u_x4_conv_k7_r2_8u
