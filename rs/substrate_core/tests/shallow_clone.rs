use std::{
  alloc::{GlobalAlloc, Layout, System},
  borrow::Cow,
  sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

use overseer_substrate_core::util::ShallowClone;

// Implementation courtesy of and modified from https://kanejaku.org/posts/2021/01/2021-01-27/
struct CheckAlloc;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CheckAlloc {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    ALLOCATIONS.fetch_add(1, SeqCst);
    ALLOCATED_BYTES.fetch_add(layout.size(), SeqCst);
    System.alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    System.dealloc(ptr, layout);
  }
}

#[global_allocator]
static A: CheckAlloc = CheckAlloc;

fn clear_alloc() {
  ALLOCATIONS.store(0, SeqCst);
  ALLOCATED_BYTES.store(0, SeqCst);
}

#[derive(Eq, PartialEq, Debug)]
struct Measurement {
  allocations: usize,
  allocated_bytes: usize,
}

fn get_alloc() -> Measurement {
  let allocations = ALLOCATIONS.load(SeqCst);
  let allocated_bytes = ALLOCATED_BYTES.load(SeqCst);
  Measurement {
    allocations,
    allocated_bytes,
  }
}

const NO_ALLOC: Measurement = Measurement {
  allocations: 0,
  allocated_bytes: 0,
};

macro_rules! assert_alloc_general {
  ($body:expr, $assertion:ident, $measurement:expr) => {{
    clear_alloc();
    let __temp = $body;
    {
      let measurement = get_alloc();
      $assertion!(measurement, $measurement);
    }
    __temp
  }};
  ($body:expr, $assertion:ident) => {
    clear_alloc();
    $body;
    {
      let measurement = get_alloc();
      $assertion!(measurement, NO_ALLOC);
    }
  };
}

macro_rules! assert_stack_only {
  ($body:expr) => {
    assert_alloc_general!($body, assert_eq, NO_ALLOC)
  };
}

macro_rules! assert_heap_allocates {
  ($body:expr) => {
    assert_alloc_general!($body, assert_ne, NO_ALLOC)
  };
}

#[test]
fn test_assert_alloc() {
  let x = assert_heap_allocates!({
    let mut x = Vec::new();
    x.push(1);
    x
  });
  assert_eq!(x.len(), 1);

  let x = assert_heap_allocates!(Box::new(42));
  assert_eq!(*x, 42);

  let x = assert_stack_only!({
    let x = &[1, 2, 3, 4];
    x
  });
  assert_eq!(x.len(), 4);
}

fn is_owned<B: ToOwned + ?Sized>(x: &Cow<B>) -> bool {
  match x {
    Cow::Owned(_) => true,
    Cow::Borrowed(_) => false,
  }
}

#[test]
fn test_shallow_clone_cow<'a>() {
  // Creating an owned Cow alone does not allocate:
  let v: Vec<u8> = vec![1, 2, 3, 4];
  assert_stack_only!({
    let _: Cow<[u8]> = Cow::Owned(v);
  });

  // Regular clone does allocate:
  let v: Vec<u8> = vec![1, 2, 3, 4];
  assert_heap_allocates!({
    let a: Cow<[u8]> = Cow::Owned(v);
    let b = a.clone();
    assert!(is_owned(&b));
  });

  // Shallow clone does not allocate:
  let v: Vec<u8> = vec![1, 2, 3, 4];
  assert_stack_only!({
    let a: Cow<[u8]> = Cow::Owned(v);
    let b: Cow<[u8]> = a.shallow_clone();
    assert_eq!(is_owned(&b), false);
  });

  // Shallow clone allocates on mutation:
  let v: Vec<u8> = vec![1, 2, 3, 4];
  assert_heap_allocates!({
    let a: Cow<[u8]> = Cow::Owned(v);
    let mut b: Cow<[u8]> = a.shallow_clone();
    assert_eq!(is_owned(&b), false);
    let _: &mut Vec<u8> = b.to_mut(); // no actual mutation, just getting a mutable reference
    assert_eq!(is_owned(&b), true);
  });
}
