#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const VOLK_VERSION_MAJOR: u32 = 2;
pub const VOLK_VERSION_MINOR: u32 = 5;
pub const VOLK_VERSION_MAINT: u32 = 2;
pub const VOLK_VERSION: u32 = 8514;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_complex<_Tp> {
    pub _M_real: _Tp,
    pub _M_imag: _Tp,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Tp>>,
}
pub type std_complex_value_type<_Tp> = _Tp;
pub type lv_8sc_t = std_complex<i8>;
pub type lv_16sc_t = std_complex<i16>;
pub type lv_32fc_t = std_complex<f32>;
pub type p_16i_32fc_dot_prod_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const ::std::os::raw::c_short,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16i_branch_4_state_8 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_short,
        arg2: *mut ::std::os::raw::c_short,
        arg3: *mut *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_short,
        arg5: *mut ::std::os::raw::c_short,
        arg6: *mut ::std::os::raw::c_short,
    ),
>;
pub type p_16i_convert_8i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i8, arg2: *const i16, arg3: ::std::os::raw::c_uint),
>;
pub type p_16i_max_star_16i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_short,
        arg2: *mut ::std::os::raw::c_short,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_16i_max_star_horizontal_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *mut i16, arg3: ::std::os::raw::c_uint),
>;
pub type p_16i_permute_and_scalar_add = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_short,
        arg2: *mut ::std::os::raw::c_short,
        arg3: *mut ::std::os::raw::c_short,
        arg4: *mut ::std::os::raw::c_short,
        arg5: *mut ::std::os::raw::c_short,
        arg6: *mut ::std::os::raw::c_short,
        arg7: *mut ::std::os::raw::c_short,
        arg8: *mut ::std::os::raw::c_short,
        arg9: ::std::os::raw::c_uint,
    ),
>;
pub type p_16i_s32f_convert_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const i16, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_16i_x4_quad_max_star_16i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_short,
        arg2: *mut ::std::os::raw::c_short,
        arg3: *mut ::std::os::raw::c_short,
        arg4: *mut ::std::os::raw::c_short,
        arg5: *mut ::std::os::raw::c_short,
        arg6: ::std::os::raw::c_uint,
    ),
>;
pub type p_16i_x5_add_quad_16i_x4 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_short,
        arg2: *mut ::std::os::raw::c_short,
        arg3: *mut ::std::os::raw::c_short,
        arg4: *mut ::std::os::raw::c_short,
        arg5: *mut ::std::os::raw::c_short,
        arg6: *mut ::std::os::raw::c_short,
        arg7: *mut ::std::os::raw::c_short,
        arg8: *mut ::std::os::raw::c_short,
        arg9: *mut ::std::os::raw::c_short,
        arg10: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_convert_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_16sc_t,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_deinterleave_16i_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i16,
        arg2: *mut i16,
        arg3: *const lv_16sc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_deinterleave_real_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *const lv_16sc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_16ic_deinterleave_real_8i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i8, arg2: *const lv_16sc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_16ic_magnitude_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *const lv_16sc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_16ic_s32f_deinterleave_32f_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *const lv_16sc_t,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_s32f_deinterleave_real_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_16sc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_s32f_magnitude_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_16sc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_x2_dot_prod_16ic = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_16sc_t,
        arg2: *const lv_16sc_t,
        arg3: *const lv_16sc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16ic_x2_multiply_16ic = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_16sc_t,
        arg2: *const lv_16sc_t,
        arg3: *const lv_16sc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_16u_byteswap =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u16, arg2: ::std::os::raw::c_uint)>;
pub type p_16u_byteswappuppet_16u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u16, arg2: *mut u16, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_64f_add_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f32,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_64f_multiply_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f32,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_8u_polarbutterfly_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ),
>;
pub type p_32f_8u_polarbutterflypuppet_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: ::std::os::raw::c_int,
    ),
>;
pub type p_32f_accumulator_s32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_acos_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_asin_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_atan_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_binary_slicer_32i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *const f32,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_binary_slicer_8i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i8, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_convert_64f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f64, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_cos_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_exp_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_expfast_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_index_max_16u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u16, arg2: *const f32, arg3: u32)>;
pub type p_32f_index_max_32u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: *const f32, arg3: u32)>;
pub type p_32f_index_min_16u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u16, arg2: *const f32, arg3: u32)>;
pub type p_32f_index_min_32u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: *const f32, arg3: u32)>;
pub type p_32f_invsqrt_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_log2_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_null_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_32f_fm_detect_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: f32,
        arg4: *mut f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_s32f_add_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_calc_spectral_noise_floor_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_convert_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_convert_32i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_convert_8i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i8, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_mod_rangepuppet_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_multiply_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_normalize = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_power_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_s32f_s32f_mod_range_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: f32,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_s32f_stddev_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32f_sin_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_sqrt_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_stddev_and_mean_32f_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_tan_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_tanh_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32f_x2_add_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_divide_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_dot_prod_16i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i16,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_dot_prod_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_fm_detectpuppet_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *mut f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_interleave_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_max_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_min_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_multiply_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_pow_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_s32f_interleave_16ic = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_16sc_t,
        arg2: *const f32,
        arg3: *const f32,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x2_subtract_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const f32,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32f_x3_sum_of_poly_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *mut f32,
        arg4: *mut f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_32f_add_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_32f_dot_prod_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_32f_multiply_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_accumulator_s32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_conjugate_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_convert_16ic = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_16sc_t,
        arg2: *const lv_32fc_t,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_deinterleave_32f_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_deinterleave_64f_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_deinterleave_imag_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const lv_32fc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_32fc_deinterleave_real_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const lv_32fc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_32fc_deinterleave_real_64f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f64, arg2: *const lv_32fc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_32fc_index_max_16u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u16, arg2: *mut lv_32fc_t, arg3: u32)>;
pub type p_32fc_index_max_32u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: *mut lv_32fc_t, arg3: u32)>;
pub type p_32fc_index_min_16u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u16, arg2: *const lv_32fc_t, arg3: u32)>;
pub type p_32fc_index_min_32u =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: *const lv_32fc_t, arg3: u32)>;
pub type p_32fc_magnitude_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const lv_32fc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_32fc_magnitude_squared_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const lv_32fc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_32fc_s32f_atan2_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_deinterleave_real_16i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i16,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_magnitude_16i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i16,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_power_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_power_spectral_densitypuppet_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_power_spectrum_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32f_x2_power_spectral_density_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_32fc_t,
        arg3: f32,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32fc_multiply_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32fc_rotatorpuppet_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_s32fc_x2_rotator_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: lv_32fc_t,
        arg4: *mut lv_32fc_t,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_add_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_conjugate_dot_prod_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_divide_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_dot_prod_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_multiply_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_multiply_conjugate_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_s32f_square_dist_scalar_mult_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut lv_32fc_t,
        arg3: *mut lv_32fc_t,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_s32fc_multiply_conjugate_add_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_32fc_t,
        arg3: *const lv_32fc_t,
        arg4: lv_32fc_t,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_32fc_x2_square_dist_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut lv_32fc_t,
        arg3: *mut lv_32fc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32i_s32f_convert_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const i32, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_32i_x2_and_32i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i32,
        arg2: *const i32,
        arg3: *const i32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32i_x2_or_32i = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i32,
        arg2: *const i32,
        arg3: *const i32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_32u_byteswap =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: ::std::os::raw::c_uint)>;
pub type p_32u_byteswappuppet_32u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u32, arg2: *mut u32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32u_popcnt = ::std::option::Option<unsafe extern "C" fn(arg1: *mut u32, arg2: u32)>;
pub type p_32u_popcntpuppet_32u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u32, arg2: *const u32, arg3: ::std::os::raw::c_uint),
>;
pub type p_32u_reverse_32u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u32, arg2: *const u32, arg3: ::std::os::raw::c_uint),
>;
pub type p_64f_convert_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const f64, arg3: ::std::os::raw::c_uint),
>;
pub type p_64f_x2_add_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f64,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_64f_x2_max_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f64,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_64f_x2_min_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f64,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_64f_x2_multiply_64f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *const f64,
        arg3: *const f64,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_64u_byteswap =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut u64, arg2: ::std::os::raw::c_uint)>;
pub type p_64u_byteswappuppet_64u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u64, arg2: *mut u64, arg3: ::std::os::raw::c_uint),
>;
pub type p_64u_popcnt = ::std::option::Option<unsafe extern "C" fn(arg1: *mut u64, arg2: u64)>;
pub type p_64u_popcntpuppet_64u = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut u64, arg2: *const u64, arg3: ::std::os::raw::c_uint),
>;
pub type p_8i_convert_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *const i8, arg3: ::std::os::raw::c_uint),
>;
pub type p_8i_s32f_convert_32f = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut f32, arg2: *const i8, arg3: f32, arg4: ::std::os::raw::c_uint),
>;
pub type p_8ic_deinterleave_16i_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut i16,
        arg2: *mut i16,
        arg3: *const lv_8sc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_8ic_deinterleave_real_16i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i16, arg2: *const lv_8sc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_8ic_deinterleave_real_8i = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut i8, arg2: *const lv_8sc_t, arg3: ::std::os::raw::c_uint),
>;
pub type p_8ic_s32f_deinterleave_32f_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *const lv_8sc_t,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_8ic_s32f_deinterleave_real_32f = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f32,
        arg2: *const lv_8sc_t,
        arg3: f32,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_8ic_x2_multiply_conjugate_16ic = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_16sc_t,
        arg2: *const lv_8sc_t,
        arg3: *const lv_8sc_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type p_8ic_x2_s32f_multiply_conjugate_32fc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut lv_32fc_t,
        arg2: *const lv_8sc_t,
        arg3: *const lv_8sc_t,
        arg4: f32,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_8u_conv_k7_r2puppet_8u = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_8u_x2_encodeframepolar_8u = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_uint,
    ),
>;
pub type p_8u_x3_encodepolar_8u_x2 = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: *const ::std::os::raw::c_uchar,
        arg5: *const ::std::os::raw::c_uchar,
        arg6: ::std::os::raw::c_uint,
    ),
>;
pub type p_8u_x3_encodepolarpuppet_8u = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: *const ::std::os::raw::c_uchar,
        arg5: ::std::os::raw::c_uint,
    ),
>;
pub type p_8u_x4_conv_k7_r2_8u = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: *mut ::std::os::raw::c_uchar,
        arg5: ::std::os::raw::c_uint,
        arg6: ::std::os::raw::c_uint,
        arg7: *mut ::std::os::raw::c_uchar,
    ),
>;
extern "C" {
    #[must_use]
    pub fn volk_malloc(size: usize, alignment: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn volk_free(aptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = "! Prints a list of machines available"]
    pub fn volk_list_machines();
}
extern "C" {
    #[doc = "! Returns the name of the machine this instance will use"]
    pub fn volk_get_machine() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "! Get the machine alignment in bytes"]
    pub fn volk_get_alignment() -> usize;
}
extern "C" {
    #[doc = " Is the pointer on a machine alignment boundary?"]
    #[doc = ""]
    #[doc = " Note: for performance reasons, this function"]
    #[doc = " is not usable until another volk API call is made"]
    #[doc = " which will perform certain initialization tasks."]
    #[doc = ""]
    #[doc = " \\param ptr the pointer to some memory buffer"]
    #[doc = " \\return 1 for alignment boundary, else 0"]
    pub fn volk_is_aligned(ptr: *const ::std::os::raw::c_void) -> bool;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_32fc_dot_prod_32fc: p_16i_32fc_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_32fc_dot_prod_32fc_a: p_16i_32fc_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_32fc_dot_prod_32fc_u: p_16i_32fc_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_branch_4_state_8: p_16i_branch_4_state_8;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_branch_4_state_8_a: p_16i_branch_4_state_8;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_branch_4_state_8_u: p_16i_branch_4_state_8;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_convert_8i: p_16i_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_convert_8i_a: p_16i_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_convert_8i_u: p_16i_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_max_star_16i: p_16i_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_max_star_16i_a: p_16i_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_max_star_16i_u: p_16i_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_max_star_horizontal_16i: p_16i_max_star_horizontal_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_max_star_horizontal_16i_a: p_16i_max_star_horizontal_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_max_star_horizontal_16i_u: p_16i_max_star_horizontal_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_permute_and_scalar_add: p_16i_permute_and_scalar_add;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_permute_and_scalar_add_a: p_16i_permute_and_scalar_add;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_permute_and_scalar_add_u: p_16i_permute_and_scalar_add;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_s32f_convert_32f: p_16i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_s32f_convert_32f_a: p_16i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_s32f_convert_32f_u: p_16i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_x4_quad_max_star_16i: p_16i_x4_quad_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_x4_quad_max_star_16i_a: p_16i_x4_quad_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_x4_quad_max_star_16i_u: p_16i_x4_quad_max_star_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16i_x5_add_quad_16i_x4: p_16i_x5_add_quad_16i_x4;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16i_x5_add_quad_16i_x4_a: p_16i_x5_add_quad_16i_x4;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16i_x5_add_quad_16i_x4_u: p_16i_x5_add_quad_16i_x4;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_convert_32fc: p_16ic_convert_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_convert_32fc_a: p_16ic_convert_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_convert_32fc_u: p_16ic_convert_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_deinterleave_16i_x2: p_16ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_deinterleave_16i_x2_a: p_16ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_deinterleave_16i_x2_u: p_16ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_deinterleave_real_16i: p_16ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_deinterleave_real_16i_a: p_16ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_deinterleave_real_16i_u: p_16ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_deinterleave_real_8i: p_16ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_deinterleave_real_8i_a: p_16ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_deinterleave_real_8i_u: p_16ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_magnitude_16i: p_16ic_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_magnitude_16i_a: p_16ic_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_magnitude_16i_u: p_16ic_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_s32f_deinterleave_32f_x2: p_16ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_s32f_deinterleave_32f_x2_a: p_16ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_s32f_deinterleave_32f_x2_u: p_16ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_s32f_deinterleave_real_32f: p_16ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_s32f_deinterleave_real_32f_a: p_16ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_s32f_deinterleave_real_32f_u: p_16ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_s32f_magnitude_32f: p_16ic_s32f_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_s32f_magnitude_32f_a: p_16ic_s32f_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_s32f_magnitude_32f_u: p_16ic_s32f_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_x2_dot_prod_16ic: p_16ic_x2_dot_prod_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_x2_dot_prod_16ic_a: p_16ic_x2_dot_prod_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_x2_dot_prod_16ic_u: p_16ic_x2_dot_prod_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16ic_x2_multiply_16ic: p_16ic_x2_multiply_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16ic_x2_multiply_16ic_a: p_16ic_x2_multiply_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16ic_x2_multiply_16ic_u: p_16ic_x2_multiply_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16u_byteswap: p_16u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16u_byteswap_a: p_16u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16u_byteswap_u: p_16u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_16u_byteswappuppet_16u: p_16u_byteswappuppet_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_16u_byteswappuppet_16u_a: p_16u_byteswappuppet_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_16u_byteswappuppet_16u_u: p_16u_byteswappuppet_16u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_64f_add_64f: p_32f_64f_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_64f_add_64f_a: p_32f_64f_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_64f_add_64f_u: p_32f_64f_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_64f_multiply_64f: p_32f_64f_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_64f_multiply_64f_a: p_32f_64f_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_64f_multiply_64f_u: p_32f_64f_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_8u_polarbutterfly_32f: p_32f_8u_polarbutterfly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_8u_polarbutterfly_32f_a: p_32f_8u_polarbutterfly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_8u_polarbutterfly_32f_u: p_32f_8u_polarbutterfly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_8u_polarbutterflypuppet_32f: p_32f_8u_polarbutterflypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_8u_polarbutterflypuppet_32f_a: p_32f_8u_polarbutterflypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_8u_polarbutterflypuppet_32f_u: p_32f_8u_polarbutterflypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_accumulator_s32f: p_32f_accumulator_s32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_accumulator_s32f_a: p_32f_accumulator_s32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_accumulator_s32f_u: p_32f_accumulator_s32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_acos_32f: p_32f_acos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_acos_32f_a: p_32f_acos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_acos_32f_u: p_32f_acos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_asin_32f: p_32f_asin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_asin_32f_a: p_32f_asin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_asin_32f_u: p_32f_asin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_atan_32f: p_32f_atan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_atan_32f_a: p_32f_atan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_atan_32f_u: p_32f_atan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_binary_slicer_32i: p_32f_binary_slicer_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_binary_slicer_32i_a: p_32f_binary_slicer_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_binary_slicer_32i_u: p_32f_binary_slicer_32i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_binary_slicer_8i: p_32f_binary_slicer_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_binary_slicer_8i_a: p_32f_binary_slicer_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_binary_slicer_8i_u: p_32f_binary_slicer_8i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_convert_64f: p_32f_convert_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_convert_64f_a: p_32f_convert_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_convert_64f_u: p_32f_convert_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_cos_32f: p_32f_cos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_cos_32f_a: p_32f_cos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_cos_32f_u: p_32f_cos_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_exp_32f: p_32f_exp_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_exp_32f_a: p_32f_exp_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_exp_32f_u: p_32f_exp_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_expfast_32f: p_32f_expfast_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_expfast_32f_a: p_32f_expfast_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_expfast_32f_u: p_32f_expfast_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_index_max_16u: p_32f_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_index_max_16u_a: p_32f_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_index_max_16u_u: p_32f_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_index_max_32u: p_32f_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_index_max_32u_a: p_32f_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_index_max_32u_u: p_32f_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_index_min_16u: p_32f_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_index_min_16u_a: p_32f_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_index_min_16u_u: p_32f_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_index_min_32u: p_32f_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_index_min_32u_a: p_32f_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_index_min_32u_u: p_32f_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_invsqrt_32f: p_32f_invsqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_invsqrt_32f_a: p_32f_invsqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_invsqrt_32f_u: p_32f_invsqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_log2_32f: p_32f_log2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_log2_32f_a: p_32f_log2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_log2_32f_u: p_32f_log2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_null_32f: p_32f_null_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_null_32f_a: p_32f_null_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_null_32f_u: p_32f_null_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_32f_fm_detect_32f: p_32f_s32f_32f_fm_detect_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_32f_fm_detect_32f_a: p_32f_s32f_32f_fm_detect_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_32f_fm_detect_32f_u: p_32f_s32f_32f_fm_detect_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_add_32f: p_32f_s32f_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_add_32f_a: p_32f_s32f_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_add_32f_u: p_32f_s32f_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_calc_spectral_noise_floor_32f:
        p_32f_s32f_calc_spectral_noise_floor_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_calc_spectral_noise_floor_32f_a:
        p_32f_s32f_calc_spectral_noise_floor_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_calc_spectral_noise_floor_32f_u:
        p_32f_s32f_calc_spectral_noise_floor_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_convert_16i: p_32f_s32f_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_convert_16i_a: p_32f_s32f_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_convert_16i_u: p_32f_s32f_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_convert_32i: p_32f_s32f_convert_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_convert_32i_a: p_32f_s32f_convert_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_convert_32i_u: p_32f_s32f_convert_32i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_convert_8i: p_32f_s32f_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_convert_8i_a: p_32f_s32f_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_convert_8i_u: p_32f_s32f_convert_8i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_mod_rangepuppet_32f: p_32f_s32f_mod_rangepuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_mod_rangepuppet_32f_a: p_32f_s32f_mod_rangepuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_mod_rangepuppet_32f_u: p_32f_s32f_mod_rangepuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_multiply_32f: p_32f_s32f_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_multiply_32f_a: p_32f_s32f_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_multiply_32f_u: p_32f_s32f_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_normalize: p_32f_s32f_normalize;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_normalize_a: p_32f_s32f_normalize;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_normalize_u: p_32f_s32f_normalize;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_power_32f: p_32f_s32f_power_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_power_32f_a: p_32f_s32f_power_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_power_32f_u: p_32f_s32f_power_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_s32f_mod_range_32f: p_32f_s32f_s32f_mod_range_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_s32f_mod_range_32f_a: p_32f_s32f_s32f_mod_range_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_s32f_mod_range_32f_u: p_32f_s32f_s32f_mod_range_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_s32f_stddev_32f: p_32f_s32f_stddev_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_s32f_stddev_32f_a: p_32f_s32f_stddev_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_s32f_stddev_32f_u: p_32f_s32f_stddev_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_sin_32f: p_32f_sin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_sin_32f_a: p_32f_sin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_sin_32f_u: p_32f_sin_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_sqrt_32f: p_32f_sqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_sqrt_32f_a: p_32f_sqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_sqrt_32f_u: p_32f_sqrt_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_stddev_and_mean_32f_x2: p_32f_stddev_and_mean_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_stddev_and_mean_32f_x2_a: p_32f_stddev_and_mean_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_stddev_and_mean_32f_x2_u: p_32f_stddev_and_mean_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_tan_32f: p_32f_tan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_tan_32f_a: p_32f_tan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_tan_32f_u: p_32f_tan_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_tanh_32f: p_32f_tanh_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_tanh_32f_a: p_32f_tanh_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_tanh_32f_u: p_32f_tanh_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_add_32f: p_32f_x2_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_add_32f_a: p_32f_x2_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_add_32f_u: p_32f_x2_add_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_divide_32f: p_32f_x2_divide_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_divide_32f_a: p_32f_x2_divide_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_divide_32f_u: p_32f_x2_divide_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_dot_prod_16i: p_32f_x2_dot_prod_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_dot_prod_16i_a: p_32f_x2_dot_prod_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_dot_prod_16i_u: p_32f_x2_dot_prod_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_dot_prod_32f: p_32f_x2_dot_prod_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_dot_prod_32f_a: p_32f_x2_dot_prod_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_dot_prod_32f_u: p_32f_x2_dot_prod_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_fm_detectpuppet_32f: p_32f_x2_fm_detectpuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_fm_detectpuppet_32f_a: p_32f_x2_fm_detectpuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_fm_detectpuppet_32f_u: p_32f_x2_fm_detectpuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_interleave_32fc: p_32f_x2_interleave_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_interleave_32fc_a: p_32f_x2_interleave_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_interleave_32fc_u: p_32f_x2_interleave_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_max_32f: p_32f_x2_max_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_max_32f_a: p_32f_x2_max_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_max_32f_u: p_32f_x2_max_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_min_32f: p_32f_x2_min_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_min_32f_a: p_32f_x2_min_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_min_32f_u: p_32f_x2_min_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_multiply_32f: p_32f_x2_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_multiply_32f_a: p_32f_x2_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_multiply_32f_u: p_32f_x2_multiply_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_pow_32f: p_32f_x2_pow_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_pow_32f_a: p_32f_x2_pow_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_pow_32f_u: p_32f_x2_pow_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_s32f_interleave_16ic: p_32f_x2_s32f_interleave_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_s32f_interleave_16ic_a: p_32f_x2_s32f_interleave_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_s32f_interleave_16ic_u: p_32f_x2_s32f_interleave_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x2_subtract_32f: p_32f_x2_subtract_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x2_subtract_32f_a: p_32f_x2_subtract_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x2_subtract_32f_u: p_32f_x2_subtract_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32f_x3_sum_of_poly_32f: p_32f_x3_sum_of_poly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32f_x3_sum_of_poly_32f_a: p_32f_x3_sum_of_poly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32f_x3_sum_of_poly_32f_u: p_32f_x3_sum_of_poly_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_32f_add_32fc: p_32fc_32f_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_32f_add_32fc_a: p_32fc_32f_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_32f_add_32fc_u: p_32fc_32f_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_32f_dot_prod_32fc: p_32fc_32f_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_32f_dot_prod_32fc_a: p_32fc_32f_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_32f_dot_prod_32fc_u: p_32fc_32f_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_32f_multiply_32fc: p_32fc_32f_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_32f_multiply_32fc_a: p_32fc_32f_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_32f_multiply_32fc_u: p_32fc_32f_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_accumulator_s32fc: p_32fc_accumulator_s32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_accumulator_s32fc_a: p_32fc_accumulator_s32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_accumulator_s32fc_u: p_32fc_accumulator_s32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_conjugate_32fc: p_32fc_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_conjugate_32fc_a: p_32fc_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_conjugate_32fc_u: p_32fc_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_convert_16ic: p_32fc_convert_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_convert_16ic_a: p_32fc_convert_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_convert_16ic_u: p_32fc_convert_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_deinterleave_32f_x2: p_32fc_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_deinterleave_32f_x2_a: p_32fc_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_deinterleave_32f_x2_u: p_32fc_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_deinterleave_64f_x2: p_32fc_deinterleave_64f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_deinterleave_64f_x2_a: p_32fc_deinterleave_64f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_deinterleave_64f_x2_u: p_32fc_deinterleave_64f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_deinterleave_imag_32f: p_32fc_deinterleave_imag_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_deinterleave_imag_32f_a: p_32fc_deinterleave_imag_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_deinterleave_imag_32f_u: p_32fc_deinterleave_imag_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_deinterleave_real_32f: p_32fc_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_deinterleave_real_32f_a: p_32fc_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_deinterleave_real_32f_u: p_32fc_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_deinterleave_real_64f: p_32fc_deinterleave_real_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_deinterleave_real_64f_a: p_32fc_deinterleave_real_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_deinterleave_real_64f_u: p_32fc_deinterleave_real_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_index_max_16u: p_32fc_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_index_max_16u_a: p_32fc_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_index_max_16u_u: p_32fc_index_max_16u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_index_max_32u: p_32fc_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_index_max_32u_a: p_32fc_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_index_max_32u_u: p_32fc_index_max_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_index_min_16u: p_32fc_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_index_min_16u_a: p_32fc_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_index_min_16u_u: p_32fc_index_min_16u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_index_min_32u: p_32fc_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_index_min_32u_a: p_32fc_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_index_min_32u_u: p_32fc_index_min_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_magnitude_32f: p_32fc_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_magnitude_32f_a: p_32fc_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_magnitude_32f_u: p_32fc_magnitude_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_magnitude_squared_32f: p_32fc_magnitude_squared_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_magnitude_squared_32f_a: p_32fc_magnitude_squared_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_magnitude_squared_32f_u: p_32fc_magnitude_squared_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_atan2_32f: p_32fc_s32f_atan2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_atan2_32f_a: p_32fc_s32f_atan2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_atan2_32f_u: p_32fc_s32f_atan2_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_deinterleave_real_16i: p_32fc_s32f_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_deinterleave_real_16i_a: p_32fc_s32f_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_deinterleave_real_16i_u: p_32fc_s32f_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_magnitude_16i: p_32fc_s32f_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_magnitude_16i_a: p_32fc_s32f_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_magnitude_16i_u: p_32fc_s32f_magnitude_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_power_32fc: p_32fc_s32f_power_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_power_32fc_a: p_32fc_s32f_power_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_power_32fc_u: p_32fc_s32f_power_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_power_spectral_densitypuppet_32f:
        p_32fc_s32f_power_spectral_densitypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_power_spectral_densitypuppet_32f_a:
        p_32fc_s32f_power_spectral_densitypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_power_spectral_densitypuppet_32f_u:
        p_32fc_s32f_power_spectral_densitypuppet_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_power_spectrum_32f: p_32fc_s32f_power_spectrum_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_power_spectrum_32f_a: p_32fc_s32f_power_spectrum_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_power_spectrum_32f_u: p_32fc_s32f_power_spectrum_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32f_x2_power_spectral_density_32f:
        p_32fc_s32f_x2_power_spectral_density_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32f_x2_power_spectral_density_32f_a:
        p_32fc_s32f_x2_power_spectral_density_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32f_x2_power_spectral_density_32f_u:
        p_32fc_s32f_x2_power_spectral_density_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32fc_multiply_32fc: p_32fc_s32fc_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32fc_multiply_32fc_a: p_32fc_s32fc_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32fc_multiply_32fc_u: p_32fc_s32fc_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32fc_rotatorpuppet_32fc: p_32fc_s32fc_rotatorpuppet_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32fc_rotatorpuppet_32fc_a: p_32fc_s32fc_rotatorpuppet_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32fc_rotatorpuppet_32fc_u: p_32fc_s32fc_rotatorpuppet_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_s32fc_x2_rotator_32fc: p_32fc_s32fc_x2_rotator_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_s32fc_x2_rotator_32fc_a: p_32fc_s32fc_x2_rotator_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_s32fc_x2_rotator_32fc_u: p_32fc_s32fc_x2_rotator_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_add_32fc: p_32fc_x2_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_add_32fc_a: p_32fc_x2_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_add_32fc_u: p_32fc_x2_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_conjugate_dot_prod_32fc: p_32fc_x2_conjugate_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_conjugate_dot_prod_32fc_a: p_32fc_x2_conjugate_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_conjugate_dot_prod_32fc_u: p_32fc_x2_conjugate_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_divide_32fc: p_32fc_x2_divide_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_divide_32fc_a: p_32fc_x2_divide_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_divide_32fc_u: p_32fc_x2_divide_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_dot_prod_32fc: p_32fc_x2_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_dot_prod_32fc_a: p_32fc_x2_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_dot_prod_32fc_u: p_32fc_x2_dot_prod_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_multiply_32fc: p_32fc_x2_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_multiply_32fc_a: p_32fc_x2_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_multiply_32fc_u: p_32fc_x2_multiply_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_multiply_conjugate_32fc: p_32fc_x2_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_multiply_conjugate_32fc_a: p_32fc_x2_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_multiply_conjugate_32fc_u: p_32fc_x2_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_s32f_square_dist_scalar_mult_32f:
        p_32fc_x2_s32f_square_dist_scalar_mult_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_s32f_square_dist_scalar_mult_32f_a:
        p_32fc_x2_s32f_square_dist_scalar_mult_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_s32f_square_dist_scalar_mult_32f_u:
        p_32fc_x2_s32f_square_dist_scalar_mult_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_s32fc_multiply_conjugate_add_32fc:
        p_32fc_x2_s32fc_multiply_conjugate_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_s32fc_multiply_conjugate_add_32fc_a:
        p_32fc_x2_s32fc_multiply_conjugate_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_s32fc_multiply_conjugate_add_32fc_u:
        p_32fc_x2_s32fc_multiply_conjugate_add_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32fc_x2_square_dist_32f: p_32fc_x2_square_dist_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32fc_x2_square_dist_32f_a: p_32fc_x2_square_dist_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32fc_x2_square_dist_32f_u: p_32fc_x2_square_dist_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32i_s32f_convert_32f: p_32i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32i_s32f_convert_32f_a: p_32i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32i_s32f_convert_32f_u: p_32i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32i_x2_and_32i: p_32i_x2_and_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32i_x2_and_32i_a: p_32i_x2_and_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32i_x2_and_32i_u: p_32i_x2_and_32i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32i_x2_or_32i: p_32i_x2_or_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32i_x2_or_32i_a: p_32i_x2_or_32i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32i_x2_or_32i_u: p_32i_x2_or_32i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32u_byteswap: p_32u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32u_byteswap_a: p_32u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32u_byteswap_u: p_32u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32u_byteswappuppet_32u: p_32u_byteswappuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32u_byteswappuppet_32u_a: p_32u_byteswappuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32u_byteswappuppet_32u_u: p_32u_byteswappuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32u_popcnt: p_32u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32u_popcnt_a: p_32u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32u_popcnt_u: p_32u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32u_popcntpuppet_32u: p_32u_popcntpuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32u_popcntpuppet_32u_a: p_32u_popcntpuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32u_popcntpuppet_32u_u: p_32u_popcntpuppet_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_32u_reverse_32u: p_32u_reverse_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_32u_reverse_32u_a: p_32u_reverse_32u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_32u_reverse_32u_u: p_32u_reverse_32u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64f_convert_32f: p_64f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64f_convert_32f_a: p_64f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64f_convert_32f_u: p_64f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64f_x2_add_64f: p_64f_x2_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64f_x2_add_64f_a: p_64f_x2_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64f_x2_add_64f_u: p_64f_x2_add_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64f_x2_max_64f: p_64f_x2_max_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64f_x2_max_64f_a: p_64f_x2_max_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64f_x2_max_64f_u: p_64f_x2_max_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64f_x2_min_64f: p_64f_x2_min_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64f_x2_min_64f_a: p_64f_x2_min_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64f_x2_min_64f_u: p_64f_x2_min_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64f_x2_multiply_64f: p_64f_x2_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64f_x2_multiply_64f_a: p_64f_x2_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64f_x2_multiply_64f_u: p_64f_x2_multiply_64f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64u_byteswap: p_64u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64u_byteswap_a: p_64u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64u_byteswap_u: p_64u_byteswap;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64u_byteswappuppet_64u: p_64u_byteswappuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64u_byteswappuppet_64u_a: p_64u_byteswappuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64u_byteswappuppet_64u_u: p_64u_byteswappuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64u_popcnt: p_64u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64u_popcnt_a: p_64u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64u_popcnt_u: p_64u_popcnt;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_64u_popcntpuppet_64u: p_64u_popcntpuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_64u_popcntpuppet_64u_a: p_64u_popcntpuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_64u_popcntpuppet_64u_u: p_64u_popcntpuppet_64u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8i_convert_16i: p_8i_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8i_convert_16i_a: p_8i_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8i_convert_16i_u: p_8i_convert_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8i_s32f_convert_32f: p_8i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8i_s32f_convert_32f_a: p_8i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8i_s32f_convert_32f_u: p_8i_s32f_convert_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_deinterleave_16i_x2: p_8ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_deinterleave_16i_x2_a: p_8ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_deinterleave_16i_x2_u: p_8ic_deinterleave_16i_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_deinterleave_real_16i: p_8ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_deinterleave_real_16i_a: p_8ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_deinterleave_real_16i_u: p_8ic_deinterleave_real_16i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_deinterleave_real_8i: p_8ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_deinterleave_real_8i_a: p_8ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_deinterleave_real_8i_u: p_8ic_deinterleave_real_8i;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_s32f_deinterleave_32f_x2: p_8ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_s32f_deinterleave_32f_x2_a: p_8ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_s32f_deinterleave_32f_x2_u: p_8ic_s32f_deinterleave_32f_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_s32f_deinterleave_real_32f: p_8ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_s32f_deinterleave_real_32f_a: p_8ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_s32f_deinterleave_real_32f_u: p_8ic_s32f_deinterleave_real_32f;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_x2_multiply_conjugate_16ic: p_8ic_x2_multiply_conjugate_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_x2_multiply_conjugate_16ic_a: p_8ic_x2_multiply_conjugate_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_x2_multiply_conjugate_16ic_u: p_8ic_x2_multiply_conjugate_16ic;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8ic_x2_s32f_multiply_conjugate_32fc: p_8ic_x2_s32f_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8ic_x2_s32f_multiply_conjugate_32fc_a:
        p_8ic_x2_s32f_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8ic_x2_s32f_multiply_conjugate_32fc_u:
        p_8ic_x2_s32f_multiply_conjugate_32fc;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8u_conv_k7_r2puppet_8u: p_8u_conv_k7_r2puppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8u_conv_k7_r2puppet_8u_a: p_8u_conv_k7_r2puppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8u_conv_k7_r2puppet_8u_u: p_8u_conv_k7_r2puppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8u_x2_encodeframepolar_8u: p_8u_x2_encodeframepolar_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8u_x2_encodeframepolar_8u_a: p_8u_x2_encodeframepolar_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8u_x2_encodeframepolar_8u_u: p_8u_x2_encodeframepolar_8u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8u_x3_encodepolar_8u_x2: p_8u_x3_encodepolar_8u_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8u_x3_encodepolar_8u_x2_a: p_8u_x3_encodepolar_8u_x2;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8u_x3_encodepolar_8u_x2_u: p_8u_x3_encodepolar_8u_x2;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8u_x3_encodepolarpuppet_8u: p_8u_x3_encodepolarpuppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8u_x3_encodepolarpuppet_8u_a: p_8u_x3_encodepolarpuppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8u_x3_encodepolarpuppet_8u_u: p_8u_x3_encodepolarpuppet_8u;
}
extern "C" {
    #[doc = "! A function pointer to the dispatcher implementation"]
    pub static mut volk_8u_x4_conv_k7_r2_8u: p_8u_x4_conv_k7_r2_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest aligned implementation"]
    pub static mut volk_8u_x4_conv_k7_r2_8u_a: p_8u_x4_conv_k7_r2_8u;
}
extern "C" {
    #[doc = "! A function pointer to the fastest unaligned implementation"]
    pub static mut volk_8u_x4_conv_k7_r2_8u_u: p_8u_x4_conv_k7_r2_8u;
}
#[test]
fn __bindgen_test_layout_std_complex_open0_float_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<std_complex<f32>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_complex<f32>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_complex<f32>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_complex<f32>)
        )
    );
}
