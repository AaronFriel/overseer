// use super::$object_name as Obj;
// use $crate::util::{RefCounted, WeakCounted};

// use std::{borrow::Cow, collections::{BTreeMap}, rc::Rc};

// use lazy_static::__Deref;
// use overseer_substrate_core::{make_handle, make_handle_two, util::{Handle,
// ShallowClone}};

// /// A trait for recursively replacing references
// trait Resolve<T, C> {
//   /// Resolve handle-based references into reference counted references
//   fn resolve(self, context: C) -> T;
// }

// #[derive(Clone, PartialEq, Eq, Debug)]
// pub struct Object<'a> {
//   pub name: Cow<'a, str>,
// }

// pub struct Game<'a, S, R> {
//   pub objects: BTreeMap<ObjectHandle, S>,

//   pub players: Vec<Player<'a, R>>,
// }

// pub struct Player<'a, R> {
//   pub name: Cow<'a, str>,
//   pub owned_objects: Vec<R>,
// }

// type SourceObject = Object<'static>;

// struct DeserializedGame(Game<'static, SourceObject, ObjectHandle>);

// impl std::ops::Deref for DeserializedGame {
//   type Target = Game<'static, SourceObject, ObjectHandle>;

//   fn deref(&self) -> &Self::Target {
//     &self.0
//   }
// }

// type ResolvedObject<'a> = Rc<Cow<'a, Object<'a>>>;

// struct ResolvedGame<'a>(Game<'a, ResolvedObject<'a>, ResolvedObject<'a>>);

// impl<'a> std::ops::Deref for ResolvedGame<'a> {
//   type Target = Game<'a, ResolvedObject<'a>, ResolvedObject<'a>>;

//   fn deref(&self) -> &Self::Target {
//     &self.0
//   }
// }

// // make_handle!(ObjectHandle, u8);

// // impl Resolve<ResolvedGame<'static>, ()> for DeserializedGame {
// //   fn resolve(self, _: ()) -> ResolvedGame<'static> {
// //     let DeserializedGame(Game { objects, players }) = self;

// //     let objects: BTreeMap<ObjectHandle, ResolvedObject<'static>> =
// objects.into_iter().map(|(h, x)| (h, Rc::new(Cow::Owned(x)))).collect(); //
// let players = { //       let objects_slice = &objects;
// //       let players: Vec<Player<'static, ResolvedObject<'static>>> = players
// //         .into_iter()
// //         .map(|x| x.resolve(objects_slice))
// //         .collect();
// //       players
// //     };

// //     ResolvedGame(Game { objects, players })
// //   }
// // }

// // impl<'b> Resolve<Player<'static, ResolvedObject<'static>>, &'b
// BTreeMap<ObjectHandle, ResolvedObject<'static>>> //   for Player<'static,
// ObjectHandle> // {
// //   fn resolve(
// //     self,
// //     context: &'b BTreeMap<ObjectHandle, ResolvedObject<'static>>,
// //   ) -> Player<'static, ResolvedObject<'static>> {
// //     let owned_objects: Vec<_> = self
// //       .owned_objects
// //       .iter()
// //       .map(|x| context.get(x).unwrap().clone())
// //       .collect();

// //     Player {
// //       name: self.name.clone(),
// //       owned_objects,
// //     }
// //   }
// // // }

// // #[test]
// // fn foo() {
// //   let mut objects = BTreeMap::new();
// //   objects.insert(ObjectHandle::from_index(0), Object {
// //     name: Cow::Borrowed("foo"),
// //   });
// //   let x = DeserializedGame(Game {
// //     objects,
// //     players: vec![Player {
// //       name: Cow::Borrowed("bar"),
// //       owned_objects: vec![ObjectHandle::from_index(0)],
// //     }],
// //   });

// //   let y = x.resolve(());

// //   let z: Vec<_> = y.objects.iter().map(|(_,x)|
// x.shallow_clone()).collect();

// //   let y_left = y.players[0].owned_objects[0].deref();
// //   let y_right =
// y.objects.get(&ObjectHandle::from_index(0)).unwrap().deref(); //   assert_eq!
// (y_left, y_right);

// //   assert_eq!(y_left, &z[0]);
// // }

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// // #[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff)]
// pub struct FooObject {
//   name: Cow<'static, str>,
// }

// // make_handle_two!(FooObject, FooPool, FooHandle, u32);

// // fn foo() {
// //   let x = FooPool::new();
// // }

// // #[test]
// // fn test_pool() {
// //   let x = FooPool::new();
// // }
