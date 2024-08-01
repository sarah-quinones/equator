pub mod by_val;
pub mod debug;
pub mod sized;

#[repr(transparent)]
pub struct Wrapper<T: ?Sized>(pub T);
