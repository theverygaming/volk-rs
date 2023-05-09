#[repr(C)]
#[derive(Clone)]
pub struct complex<T> {
    pub r: T,
    pub i: T,
}
