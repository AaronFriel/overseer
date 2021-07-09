use nonzero_ext::NonZeroAble;

pub trait Handle<T>
where
  T: NonZeroAble,
{
  fn from_index(value: usize) -> Self;

  fn to_index(&self) -> usize;

  fn number(&self) -> T;
}

#[macro_export]
macro_rules! make_handle {
  ($struct_name:ident, $size:ty) => {
    paste::paste! {
      mod [<$struct_name:snake _impl>] {
        use serde::{Deserialize, Serialize};
        use serde_diff::SerdeDiff;
        use nonzero_ext::NonZeroAble;
        use $crate::handle::Handle;

        #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
        #[derive(Serialize, Deserialize, SerdeDiff)]
        #[serde_diff(opaque)]
        pub struct $struct_name(<$size as NonZeroAble>::NonZero);

        impl Handle<$size> for $struct_name {
          #[inline]
          fn from_index(value: usize) -> Self {
            $struct_name(unsafe { ((value + 1) as $size).into_nonzero_unchecked() })
          }
          #[inline]
          fn to_index(&self) -> usize {
            self.number() as usize - 1
          }
          #[inline]
          fn number(&self) -> $size {
            self.0.into()
          }
        }
      }
      pub use [<$struct_name:snake _impl>]::$struct_name;
    }
  };
}
