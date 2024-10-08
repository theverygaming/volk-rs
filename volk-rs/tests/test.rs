use num_complex::Complex;
use volk_rs::{self, vec::AlignedVec};

#[test]
fn vector() {
    let _a: AlignedVec<std::vec::Vec<i64>> = AlignedVec::from_elem(vec![5i64; 100], 2000);

    let mut _b: AlignedVec<u64> = AlignedVec::from_elem(12, 500);

    let mut c: AlignedVec<u64> = AlignedVec::from_elem(16, 100);

    let mut n: u64 = 0;

    for i in c.iter_mut() {
        *i = n;
        n = (n + 1) + 1234;
    }

    _b = AlignedVec::from_elem(16, 5000);

    n = 0;
    for i in _b.iter_mut() {
        *i = n;
        n = (n + 1) + 1234;
    }

    n = 0;
    for i in _b.iter() {
        if *i != n {
            panic!("volk vector borked");
        }
        n = (n + 1) + 1234;
    }

    n = 0;
    for i in c.iter() {
        if *i != n {
            panic!("volk vector borked");
        }
        n = (n + 1) + 1234;
    }
}

#[test]
fn vector_swap() {
    let mut a: AlignedVec<u64> = AlignedVec::from_elem(123, 2);
    let mut b: AlignedVec<u64> = AlignedVec::from_elem(124, 2);
    assert!(a[0] == 123 && a[1] == 123);
    assert!(b[0] == 124 && b[1] == 124);

    std::mem::swap(&mut a, &mut b);

    assert!(a[0] == 124 && a[1] == 124);
    assert!(b[0] == 123 && b[1] == 123);
}

#[test]
fn volk_16i_32fc_dot_prod_32fc() {
    let input: AlignedVec<core::ffi::c_short> = AlignedVec::from_elem(1, 500);
    let mut taps: AlignedVec<Complex<f32>> = AlignedVec::from_elem(Complex { re: 5.0, im: 2.0 }, 500);
    let mut result: Complex<f32> = Complex { re: 0.0, im: 0.0 };
    volk_rs::kernels::volk_16i_32fc_dot_prod_32fc(&input, &mut result, &mut taps);
    assert!(result.re != 0.0 && result.im != 0.0, "borked");
}

#[test]
fn v32fc_s32fc_x2_rotator_32fc() {
    let input: AlignedVec<Complex<f32>> = AlignedVec::from_elem(Complex { re: 5.0, im: 2.0 }, 5000);
    let mut result: AlignedVec<Complex<f32>> = AlignedVec::from_elem(Complex { re: 5.0, im: 2.0 }, 5000);
    let phase_inc: Complex<f32> = Complex { re: 0.5, im: 1.0 };
    let mut phase: Complex<f32> = Complex { re: 1.0, im: 0.0 };
    volk_rs::kernels::volk_32fc_s32fc_x2_rotator2_32fc(&input,&mut result, &phase_inc, &mut phase);
}

#[test]
fn volk_32fc_32f_multiply_32fc() {
    let input: AlignedVec<Complex<f32>> = AlignedVec::from_elem(Complex { re: 0.123, im: 0.576 }, 5000);
    let input_f: AlignedVec<f32> = AlignedVec::from_elem(0.5, 5000);
    let mut result: AlignedVec<Complex<f32>> = AlignedVec::from_elem(Complex { re: 0.0, im: 0.0 }, 5000);
    volk_rs::kernels::volk_32fc_32f_multiply_32fc(&input, &mut result, &input_f);
}
