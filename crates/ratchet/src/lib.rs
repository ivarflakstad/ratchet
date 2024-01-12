#![feature(trait_alias)]
mod compiled_op;
mod device;
mod dtype;
mod gpu;
mod op;
mod shape;
mod storage;
mod strides;
mod symshape;
mod tensor;
mod view;

pub use compiled_op::*;
pub use device::*;
pub use dtype::*;
pub use op::*;
pub use shape::*;
pub use storage::*;
pub use strides::*;
pub use symshape::*;
pub use tensor::*;
pub use view::*;

use smallvec::SmallVec;
pub type RVec<T> = SmallVec<[T; 4]>;
pub type DRVec<T> = SmallVec<[T; 8]>; //Double RVec

//https://github.com/sonos/tract/blob/main/data/src/macros.rs#L2
#[macro_export]
macro_rules! rvec {
    (@one $x:expr) => (1usize);
    ($elem:expr; $n:expr) => ({
        $crate::RVec::from_elem($elem, $n)
    });
    ($($x:expr),*$(,)*) => ({
        let count = 0usize $(+ rvec![@one $x])*;
        #[allow(unused_mut)]
        let mut vec = $crate::RVec::new();
        if count <= vec.inline_size() {
            $(vec.push($x);)*
            vec
        } else {
            $crate::RVec::from_vec(vec![$($x,)*])
        }
    });
}

#[macro_export]
macro_rules! shape {
    ($($x:expr),*$(,)*) => ({
        use $crate::rvec;
        $crate::Shape::new(rvec![$($x,)*])
    });
}
