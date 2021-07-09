use std::borrow::Cow;

pub trait Viewable<'a> {
  type Context;

  fn view_as(&self, context: &Self::Context) -> Self;
}


impl<'a, T> Viewable<'a> for Vec<T>
where
  T: Viewable<'a>,
{
  type Context = <T as Viewable<'a>>::Context;

  fn view_as(&self, context: &Self::Context) -> Self {
    self.iter().map(|x| x.view_as(context)).collect()
  }
}

impl<'a, T> Viewable<'a> for Cow<'a, T>
where
  T: Clone + Viewable<'a>,
{
  type Context = <T as Viewable<'a>>::Context;

  fn view_as(&self, context: &Self::Context) -> Self {
    Cow::Owned((self as &T).view_as(context))
  }
}
