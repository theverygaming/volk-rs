#[repr(C)]
#[derive(Clone, Copy)]
pub struct complex<T> {
    pub r: T,
    pub i: T,
}
