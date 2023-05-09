use volk_rs::{self, types::complex, vec::AlignedVec};

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
fn v16i_32fc_dot_prod_32fc() {
    let mut input: AlignedVec<core::ffi::c_short> = AlignedVec::from_elem(1, 500);
    let mut taps: AlignedVec<complex<f32>> = AlignedVec::from_elem(complex { r: 5.0, i: 2.0 }, 500);
    let mut result: complex<f32> = complex { r: 0.0, i: 0.0 };
    volk_rs::kernels::v16i_32fc_dot_prod_32fc(&mut result, &mut input, &mut taps);
    assert!(result.r != 0.0 && result.i != 0.0, "borked");
}
