use std::borrow::Cow;

pub trait Visible {
  type Context;

  fn is_visible(&self, context: &Self::Context) -> bool;

  fn to_visible(&self, context: &Self::Context) -> Self;

  fn to_hidden(&self, context: &Self::Context) -> Self;
}

pub trait Viewable {
  type Context;

  fn view_as(&self, context: &Self::Context) -> Self;
}

impl<T> Viewable for T
where
  T: Visible,
{
  type Context = <T as Visible>::Context;

  fn view_as(&self, context: &Self::Context) -> Self {
    if self.is_visible(context) {
      self.to_visible(context)
    } else {
      self.to_hidden(context)
    }
  }
}

impl<T> Viewable for Vec<T>
where
  T: Viewable,
{
  type Context = <T as Viewable>::Context;

  fn view_as(&self, context: &Self::Context) -> Self {
    self.iter().map(|x| x.view_as(context)).collect()
  }
}

impl<'a, T> Viewable for Cow<'a, T>
where
  T: Clone + Viewable,
{
  type Context = <T as Viewable>::Context;

  fn view_as(&self, context: &Self::Context) -> Self {
    Cow::Owned((self as &T).view_as(context))
  }
}
