//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::World;

#[derive(Default, PartialEq, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, PartialEq, Debug)]
struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, PartialEq, Debug)]
struct Mesh {
    pub a: usize,
}

impl Mesh {
    pub fn new(a: usize) -> Self {
        Self { a }
    }
}

#[derive(Default, Debug)]
struct Dropper {
    pub counter: Arc<AtomicU32>,
}

impl Dropper {
    pub fn new(counter: &Arc<AtomicU32>) -> Self {
        counter.fetch_add(1, Ordering::SeqCst);
        Self {
            counter: counter.clone(),
        }
    }
}

impl Drop for Dropper {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

#[test]
fn extend_test_vec() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.query_one::<&Position>(ids[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.query_one::<&Position>(ids[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids[0]));
    assert!(!world.has_component::<Mesh>(ids[1]));
}

#[test]
fn extend_test_array() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.query_one::<&Position>(ids[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.query_one::<&Position>(ids[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids[0]));
    assert!(!world.has_component::<Mesh>(ids[1]));
}

#[test]
fn remove_entity_array() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids_a = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids_a.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.query_one::<&Position>(ids_a[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.query_one::<&Position>(ids_a[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids_a[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.query_one::<&Scale>(ids_a[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids_a[0]));
    assert!(!world.has_component::<Mesh>(ids_a[1]));

    assert!(world.remove_entity(ids_a[0]));
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_a[0]));
    assert_eq!(world.len(), 1);

    assert!(world.remove_entity(ids_a[1]));
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_a[1]));
    assert_eq!(world.len(), 0);

    let ids_b = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Mesh::new(9), Mesh::new(10)],
    ));

    assert_eq!(ids_b.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.query_one::<&Position>(ids_b[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.query_one::<&Position>(ids_b[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(world.query_one::<&Mesh>(ids_b[0]).unwrap(), &Mesh::new(9));
    assert_eq!(world.query_one::<&Mesh>(ids_b[1]).unwrap(), &Mesh::new(10));

    assert!(!world.has_component::<Scale>(ids_b[0]));
    assert!(!world.has_component::<Scale>(ids_b[1]));

    assert!(world.remove_entity(ids_b[1]));
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_b[1]));
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_a[1]));
    assert_eq!(world.len(), 1);

    assert!(world.remove_entity(ids_b[0]));
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_b[0]));
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_a[0]));
    assert_eq!(world.len(), 0);
}

#[test]
fn remove_component_test() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        [Position::default(), Position::default()],
        [Scale::default(), Scale::default()],
    ));

    assert!(world.has_component::<Scale>(ids[1]));

    assert!(world.remove_component::<Scale>(ids[1]));

    assert!(!world.has_component::<Scale>(ids[1]));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);
}

#[test]
fn add_component_test() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        [Position::default(), Position::default()],
        [Scale::default(), Scale::default()],
    ));

    assert!(world.add_component(ids[0], Mesh::default()));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);
}

#[test]
fn drop_test() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();
    world.register::<Dropper>();

    let counter = AtomicU32::new(0);
    let counter = Arc::new(counter);

    let ids = world.extend((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Dropper::new(&counter), Dropper::new(&counter)],
    ));

    assert_eq!(counter.load(Ordering::SeqCst), 2);
    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    let comp = world.query_one::<&Dropper>(ids[0]).unwrap();
    assert_eq!(comp.counter.load(Ordering::SeqCst), 2);

    let comp = world.query_one::<&Dropper>(ids[1]).unwrap();
    assert_eq!(comp.counter.load(Ordering::SeqCst), 2);

    assert!(world.remove_entity(ids[0]));

    assert_eq!(counter.load(Ordering::SeqCst), 1);

    drop(world);

    assert_eq!(counter.load(Ordering::SeqCst), 0);
}

#[test]
fn query_test() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let scale_ids = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    let mesh_ids = world.extend((
        [Position::new(1.5, 2.5), Position::new(3.5, 4.5)],
        [Mesh::new(5), Mesh::new(6)],
    ));

    for (id, (_pos, _scale)) in world.query::<(&Position, &Scale)>() {
        assert!(scale_ids.contains(&id));
    }

    for (id, (_pos, _mesh)) in world.query_mut::<(&Position, &mut Mesh)>() {
        assert!(mesh_ids.contains(&id));
    }

    {
        let mut query = world.query::<(&Position, &Scale)>();

        let first = query.next().unwrap().0;
        assert!(scale_ids.contains(&first));

        let second = query.next().unwrap().0;
        assert!(scale_ids.contains(&second));

        assert!(query.next().is_none());
        assert!(query.next().is_none());
    }

    {
        let mut query = world.query_mut::<(&Position, &mut Mesh)>();

        let first = query.next().unwrap().0;
        assert!(mesh_ids.contains(&first));

        let second = query.next().unwrap().0;
        assert!(mesh_ids.contains(&second));

        assert!(query.next().is_none());
        assert!(query.next().is_none());
    }
}

#[test]
#[should_panic]
fn query_checked_test_colliding_access() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let _scale_ids = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    let _mesh_ids = world.extend((
        [Position::new(1.5, 2.5), Position::new(3.5, 4.5)],
        [Mesh::new(5), Mesh::new(6)],
    ));

    let query_a = world.query_checked::<&mut Position>();
    let query_b = world.query_checked::<(&Position, &mut Mesh)>();

    let iter = query_a.zip(query_b);
    assert_eq!(iter.count(), 2);
}

#[test]
fn query_checked_test_safe_access() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let _scale_ids = world.extend((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    let _mesh_ids = world.extend((
        [Position::new(1.5, 2.5), Position::new(3.5, 4.5)],
        [Mesh::new(5), Mesh::new(6)],
    ));

    // query_b will iterate the same archetype as query_a, but only query_a will ask for access to
    // 'Scale' so both queries can run in parallel.
    let query_a = world.query_checked::<(&Position, &mut Scale)>();
    let query_b = world.query_checked::<&Position>();

    let iter = query_a.zip(query_b);
    assert_eq!(iter.count(), 2);
}

#[test]
fn is_world_send_and_sync() {
    fn the_send_sync_filter<T: Send + Sync>(_v: T) {
        let _do_nothing = ();
    }
    let world = World::new(Default::default()).unwrap();
    let _nothing = the_send_sync_filter::<World>(world);
}
