use std::borrow::{Borrow, Cow};

pub trait ShallowClone<'b, U> {
  fn shallow_clone(&'b self) -> U;
}

impl<'a, 'b: 'a, B> ShallowClone<'b, Cow<'a, B>> for Cow<'b, B>
where
  B: ToOwned + ?Sized,
  <B as ToOwned>::Owned: Borrow<B>,
{
  fn shallow_clone(&'b self) -> Cow<'a, B> {
    match self {
      Cow::Borrowed(x) => Cow::Borrowed(&x),
      Cow::Owned(x) => Cow::Borrowed(x.borrow()),
    }
  }
}
