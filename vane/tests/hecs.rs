//! Tests taken from the `hecs` crate. Original source
//! available at https://github.com/Ralith/hecs/blob/master/tests/tests.rs.
//!
//! Adjusted to fit API differences. Some tests have been ommitted or
//! commented out, because they test features not available in this library.

// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use vane::*;

#[test]
fn random_access() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456, true));
    assert_eq!(*world.get::<&str>(e).unwrap(), "abc");
    assert_eq!(*world.get::<i32>(e).unwrap(), 123);
    assert_eq!(*world.get::<&str>(f).unwrap(), "def");
    assert_eq!(*world.get::<i32>(f).unwrap(), 456);
    *world.get_mut::<i32>(f).unwrap() = 42;
    assert_eq!(*world.get::<i32>(f).unwrap(), 42);
}

#[test]
fn despawn() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456));
    assert_eq!(world.iter().count(), 2);
    world.despawn(e).unwrap();
    assert_eq!(world.iter().count(), 1);
    assert!(world.get::<&str>(e).is_err());
    assert!(world.get::<i32>(e).is_err());
    assert_eq!(*world.get::<&str>(f).unwrap(), "def");
    assert_eq!(*world.get::<i32>(f).unwrap(), 456);
}

#[test]
fn query_all() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456));

    let ents = world
        .query::<(&i32, &&str)>()
        .iter()
        .map(|(e, (i, s))| (e, *i, *s))
        .collect::<Vec<_>>();
    assert_eq!(ents.len(), 2);
    assert!(ents.contains(&(e, 123, "abc")));
    assert!(ents.contains(&(f, 456, "def")));

    let ents = world.iter().collect::<Vec<_>>();
    assert_eq!(ents.len(), 2);
    assert!(ents.contains(&e));
    assert!(ents.contains(&f));
}

#[test]
fn query_single_component() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456, true));
    let ents = world
        .query::<&i32>()
        .iter()
        .map(|(e, i)| (e, *i))
        .collect::<Vec<_>>();
    assert_eq!(ents.len(), 2);
    assert!(ents.contains(&(e, 123)));
    assert!(ents.contains(&(f, 456)));
}

#[test]
fn query_missing_component() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    world.spawn_bundle(("def", 456));
    assert!(world.query::<(&bool, &i32)>().iter().next().is_none());
}

#[test]
fn query_sparse_component() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456, true));
    let ents = world
        .query::<&bool>()
        .iter()
        .map(|(e, b)| (e, *b))
        .collect::<Vec<_>>();
    assert_eq!(ents, &[(f, true)]);
}

/*
#[test]
fn query_optional_component() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let f = world.spawn_bundle(("def", 456, true));
    let ents = world
        .query::<(Option<&bool>, &i32)>()
        .iter()
        .map(|(e, (b, &i))| (e, b.copied(), i))
        .collect::<Vec<_>>();
    assert_eq!(ents.len(), 2);
    assert!(ents.contains(&(e, None, 123)));
    assert!(ents.contains(&(f, Some(true), 456)));
}
*/

#[test]
fn build_entity() {
    let mut world = World::new();
    let mut entity = EntityBuilder::new();
    entity.add("abc");
    entity.add(123);
    let e = entity.spawn_into(&mut world);
    entity.add("def");
    entity.add([0u8; 1024]);
    entity.add(456);
    entity.add(789);
    let f = entity.spawn_into(&mut world);
    assert_eq!(*world.get::<&str>(e).unwrap(), "abc");
    assert_eq!(*world.get::<i32>(e).unwrap(), 123);
    assert_eq!(*world.get::<&str>(f).unwrap(), "def");
    assert_eq!(*world.get::<i32>(f).unwrap(), 789);
}

#[test]
fn access_builder_components() {
    let mut world = World::new();
    let mut entity = EntityBuilder::new();

    entity.add("abc");
    entity.add(123);

    assert!(entity.has::<&str>());
    assert!(entity.has::<i32>());
    assert!(!entity.has::<usize>());

    let g = world.spawn_builder(&mut entity);

    assert_eq!(*world.get::<&str>(g).unwrap(), "abc");
    assert_eq!(*world.get::<i32>(g).unwrap(), 123);
}

#[test]
fn build_entity_bundle() {
    let mut world = World::new();
    let mut entity = EntityBuilder::new();
    entity.add(123);
    entity.add("abc");
    let e = entity.spawn_into(&mut world);
    entity.add(456);
    entity.add("def");
    entity.add([0u8; 1024]);
    entity.add(789);
    let f = entity.spawn_into(&mut world);
    assert_eq!(*world.get::<&str>(e).unwrap(), "abc");
    assert_eq!(*world.get::<i32>(e).unwrap(), 123);
    assert_eq!(*world.get::<&str>(f).unwrap(), "def");
    assert_eq!(*world.get::<i32>(f).unwrap(), 789);
}

#[test]
fn dynamic_components() {
    let mut world = World::new();
    let e = world.spawn_bundle((42,));
    world.insert(e, true).unwrap();
    world.insert(e, "abc").unwrap();
    assert_eq!(
        world
            .query::<(&i32, &bool)>()
            .iter()
            .map(|(e, (i, b))| (e, *i, *b))
            .collect::<Vec<_>>(),
        &[(e, 42, true)]
    );
    world.remove::<i32>(e).unwrap();
    assert_eq!(
        world
            .query::<(&i32, &bool)>()
            .iter()
            .map(|(e, (i, b))| (e, *i, *b))
            .collect::<Vec<_>>(),
        &[]
    );
    assert_eq!(
        world
            .query::<(&bool, &&str)>()
            .iter()
            .map(|(e, (b, s))| (e, *b, *s))
            .collect::<Vec<_>>(),
        &[(e, true, "abc")]
    );
}

#[test]
#[should_panic(expected = "query causes borrow conflicts: BorrowError")]
fn illegal_borrow() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    world.spawn_bundle(("def", 456));

    let _ = world.query::<(&mut i32, &i32)>().iter().collect::<Vec<_>>();
}

#[test]
fn disjoint_queries() {
    let mut world = World::new();
    world.spawn_bundle(("abc", true));
    world.spawn_bundle(("def", 456));

    let _a = world.query::<(&mut &str, &bool)>();
    let _b = world.query::<(&mut &str, &i32)>();
}

#[test]
fn shared_borrow() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    world.spawn_bundle(("def", 456));

    world.query::<(&i32, &i32)>();
}

#[test]
#[should_panic(expected = "BorrowConflict(BorrowError)")]
fn illegal_random_access() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    let _borrow = world.get_mut::<i32>(e).unwrap();
    world.get::<i32>(e).unwrap();
}

#[test]
#[cfg_attr(miri, ignore)]
fn spawn_many() {
    let mut world = World::new();
    const N: usize = 100_000;
    for _ in 0..N {
        world.spawn_bundle((42u128,));
    }
    assert_eq!(world.iter().count(), N);
}

/*
#[test]
fn clear() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    world.spawn_bundle(("def", 456, true));
    world.clear();
    assert_eq!(world.iter().count(), 0);
}
*/

#[test]
#[should_panic(expected = "query causes borrow conflicts: BorrowError")]
fn alias() {
    let mut world = World::new();
    world.spawn_bundle(("abc", 123));
    world.spawn_bundle(("def", 456, true));
    let mut q = world.query::<&mut i32>();
    let _a = q.iter().collect::<Vec<_>>();
    let mut q = world.query::<&mut i32>();
    let _b = q.iter().collect::<Vec<_>>();
}

#[test]
fn remove_missing() {
    let mut world = World::new();
    let e = world.spawn_bundle(("abc", 123));
    assert!(world.remove::<bool>(e).is_err());
}

/*
#[test]
fn reserve() {
    let mut world = World::new();
    let a = world.reserve_entity();
    let b = world.reserve_entity();

    assert_eq!(world.iter().count(), 0);

    world.flush();

    let entities = world
        .query::<()>()
        .iter()
        .map(|(e, ())| e)
        .collect::<Vec<_>>();

    assert_eq!(entities.len(), 2);
    assert!(entities.contains(&a));
    assert!(entities.contains(&b));
}

#[test]
fn query_one() {
    let mut world = World::new();
    let a = world.spawn_bundle(("abc", 123));
    let b = world.spawn_bundle(("def", 456));
    let c = world.spawn_bundle(("ghi", 789, true));
    assert_eq!(world.query_one::<&i32>(a).unwrap().get(), Some(&123));
    assert_eq!(world.query_one::<&i32>(b).unwrap().get(), Some(&456));
    assert!(world.query_one::<(&i32, &bool)>(a).unwrap().get().is_none());
    assert_eq!(
        world.query_one::<(&i32, &bool)>(c).unwrap().get(),
        Some((&789, &true))
    );
    world.despawn(a).unwrap();
    assert!(world.query_one::<&i32>(a).is_err());
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic(
        expected = "attempted to allocate entity with duplicate f32 components; each type must occur at most once!"
    )
)]
#[cfg_attr(
    not(debug_assertions),
    should_panic(
        expected = "attempted to allocate entity with duplicate components; each type must occur at most once!"
    )
)]
fn duplicate_components_panic() {
    let mut world = World::new();
    world.reserve::<(f32, i64, f32)>(1);
}
*/
