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

use std::sync::Arc;

use crate::register_component;
use crate::world::World;
use crate::world::query::{Has, Read, Write};

#[derive(Clone, Default, PartialEq, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}
register_component!(Position);

#[derive(Clone, PartialEq, Debug)]
struct BigComponent {
    pub data: [u8; 64],
}
register_component!(BigComponent);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
struct Scale {
    pub x: f32,
    pub y: f32,
}
register_component!(Scale);

impl Scale {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, PartialEq, Debug)]
struct Mesh {
    pub a: usize,
}
register_component!(Mesh);

impl Mesh {
    pub fn new(a: usize) -> Self {
        Self { a }
    }
}

#[derive(Default, Debug)]
struct Dropper(Arc<()>);
register_component!(Dropper);

#[test]
fn extend_test_vec() {
    let mut world = World::new();

    let ids = world.bulk_insert((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.get_component_ref::<Position>(ids[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.get_component_ref::<Position>(ids[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids[0]));
    assert!(!world.has_component::<Mesh>(ids[1]));
}

#[test]
fn extend_test_vec_large() {
    let mut world = World::new();

    let positions: Vec<_> = (0..20_000)
        .enumerate()
        .map(|(i, _)| Position::new(1.0 * i as f32, 2.0 * i as f32))
        .collect();

    let scales: Vec<_> = (0..20_000)
        .enumerate()
        .map(|(i, _)| Scale::new(3.0 * i as f32, 5.0 * i as f32))
        .collect();

    let ids = world.bulk_insert((positions.clone(), scales.clone()));

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for ((id, position), scale) in ids.iter().zip(positions).zip(scales) {
        let (stored_position, stored_scale) = world
            .query_one::<(Read<Position>, Read<Scale>)>(*id)
            .unwrap();
        assert_eq!(position, stored_position.clone());
        assert_eq!(scale, stored_scale.clone());
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
}

#[test]
fn extend_test_vec_larger() {
    let mut world = World::new();

    let components: Vec<_> = (0..20_000)
        .map(|_| BigComponent { data: [56; 64] })
        .collect();

    let ids = world.bulk_insert(components.clone());

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for (id, component) in ids.iter().zip(components) {
        let stored_component = world.get_component_ref::<BigComponent>(*id).unwrap();
        assert_eq!(&component, stored_component);
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
}

#[test]
fn extend_test_vec_larger_extend_one() {
    let mut world = World::new();

    let components: Vec<_> = (0..20_000)
        .map(|_| BigComponent { data: [56; 64] })
        .collect();

    let mut ids = Vec::new();
    for component in components.iter() {
        ids.push(world.insert(component.clone()));
    }

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for (id, component) in ids.iter().zip(components) {
        let stored_component = world.get_component_ref::<BigComponent>(*id).unwrap();
        assert_eq!(&component, stored_component);
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
}

#[test]
fn extend_test_array() {
    let mut world = World::new();

    let ids = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.get_component_ref::<Position>(ids[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.get_component_ref::<Position>(ids[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids[0]));
    assert!(!world.has_component::<Mesh>(ids[1]));
}

#[test]
fn extend_test_one() {
    let mut world = World::new();

    let mut ids = Vec::new();
    ids.push(world.insert((Position::new(1.0, 2.0), Scale::new(5.0, 6.0))));
    ids.push(world.insert((Position::new(3.0, 4.0), Scale::new(7.0, 8.0))));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.get_component_ref::<Position>(ids[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.get_component_ref::<Position>(ids[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids[0]));
    assert!(!world.has_component::<Mesh>(ids[1]));
}

#[test]
#[should_panic]
fn extend_test_non_matched_sizes() {
    let mut world = World::new();

    let _ids = world.bulk_insert((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Scale::new(5.0, 6.0)],
    ));
}

#[test]
#[should_panic]
fn extend_test_duplicate_components() {
    let mut world = World::new();

    let _ids = world.bulk_insert((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Position::new(5.0, 6.0), Position::new(7.0, 8.0)],
    ));
}

#[test]
#[should_panic]
fn extend_test_duplicate_components_2() {
    let mut world = World::new();

    let _id = world.insert((Position::new(1.0, 2.0), Position::new(5.0, 6.0)));
}

#[test]
fn remove_entity_array() {
    let mut world = World::new();

    let ids_a = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    assert_eq!(ids_a.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.get_component_ref::<Position>(ids_a[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.get_component_ref::<Position>(ids_a[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids_a[0]).unwrap(),
        &Scale::new(5.0, 6.0)
    );
    assert_eq!(
        world.get_component_ref::<Scale>(ids_a[1]).unwrap(),
        &Scale::new(7.0, 8.0)
    );

    assert!(!world.has_component::<Mesh>(ids_a[0]));
    assert!(!world.has_component::<Mesh>(ids_a[1]));

    assert!(world.remove_entity(ids_a[0]).is_some());
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_a[0]).is_some());
    assert_eq!(world.len(), 1);

    assert!(world.remove_entity(ids_a[1]).is_some());
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_a[1]).is_some());
    assert_eq!(world.len(), 0);

    let ids_b = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Mesh::new(9), Mesh::new(10)],
    ));

    assert_eq!(ids_b.len(), 2);
    assert_eq!(world.len(), 2);

    assert_eq!(
        world.get_component_ref::<Position>(ids_b[0]).unwrap(),
        &Position::new(1.0, 2.0)
    );
    assert_eq!(
        world.get_component_ref::<Position>(ids_b[1]).unwrap(),
        &Position::new(3.0, 4.0)
    );
    assert_eq!(
        world.get_component_ref::<Mesh>(ids_b[0]).unwrap(),
        &Mesh::new(9)
    );
    assert_eq!(
        world.get_component_ref::<Mesh>(ids_b[1]).unwrap(),
        &Mesh::new(10)
    );

    assert!(!world.has_component::<Scale>(ids_b[0]));
    assert!(!world.has_component::<Scale>(ids_b[1]));

    assert!(world.remove_entity(ids_b[1]).is_some());
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_b[1]).is_some());
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_a[1]).is_some());
    assert_eq!(world.len(), 1);

    assert!(world.remove_entity(ids_b[0]).is_some());
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_b[0]).is_some());
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_a[0]).is_some());
    assert_eq!(world.len(), 0);
}

#[test]
fn add_remove_component_test() {
    let mut world = World::new();

    let id0 = world.spawn_entity();
    let id1 = world.spawn_entity();

    assert_eq!(world.has_component::<Position>(id0), false);
    assert_eq!(world.has_component::<Position>(id1), false);
    assert_eq!(world.has_component::<Scale>(id0), false);
    assert_eq!(world.has_component::<Scale>(id1), false);

    world.add_component(id0, Position::new(1.0, 2.0)).unwrap();
    world.add_component(id1, Position::new(3.0, 4.0)).unwrap();

    assert_eq!(
        world.get_component_ref::<Position>(id0),
        Some(&Position::new(1.0, 2.0))
    );
    assert_eq!(
        world.get_component_ref::<Position>(id1),
        Some(&Position::new(3.0, 4.0))
    );
    assert_eq!(world.has_component::<Scale>(id0), false);
    assert_eq!(world.has_component::<Scale>(id1), false);

    world.add_component(id0, Scale::new(5.0, 6.0)).unwrap();

    assert_eq!(
        world.get_component_ref::<Position>(id0),
        Some(&Position::new(1.0, 2.0))
    );
    assert_eq!(
        world.get_component_ref::<Position>(id1),
        Some(&Position::new(3.0, 4.0))
    );
    assert_eq!(
        world.get_component_ref::<Scale>(id0),
        Some(&Scale::new(5.0, 6.0))
    );
    assert_eq!(world.get_component_ref::<Scale>(id1), None);

    let removed = world.remove_component::<Position>(id1).unwrap();
    assert_eq!(removed, Position::new(3.0, 4.0));

    assert_eq!(
        world.get_component_ref::<Position>(id0),
        Some(&Position::new(1.0, 2.0))
    );
    assert_eq!(world.get_component_ref::<Position>(id1), None);
    assert_eq!(
        world.get_component_ref::<Scale>(id0),
        Some(&Scale::new(5.0, 6.0))
    );
    assert_eq!(world.get_component_ref::<Scale>(id1), None);

    assert_eq!(world.len(), 2);
}

#[test]
fn drop_test() {
    let mut world = World::new();

    // Counter. Use an arc so we can snoop on when components are dropped via Dropper.
    let counter = Arc::new(());

    let id0 = world.spawn_entity();
    let id1 = world.spawn_entity();

    // Add an unrelated component. We test adding dropper to a entity with no components and one
    // with components.
    world.add_component(id0, Position::new(1.0, 2.0)).unwrap();

    world.add_component(id0, Dropper(counter.clone())).unwrap();
    world.add_component(id1, Dropper(counter.clone())).unwrap();

    // Check that we have exactly 3 live handles to the counter and that all the entities agree
    assert_eq!(Arc::strong_count(&counter), 3);
    let comp = world.get_component_ref::<Dropper>(id0).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);
    let comp = world.get_component_ref::<Dropper>(id1).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);

    // Add another component to force it to move between archetypes. The ref count should not
    // increment and _absolutely not_ be decremented
    world.add_component(id0, Scale::new(5.0, 6.0)).unwrap();

    // Check that we have exactly 3 live handles to the counter and that all the entities agree
    assert_eq!(Arc::strong_count(&counter), 3);
    let comp = world.get_component_ref::<Dropper>(id0).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);
    let comp = world.get_component_ref::<Dropper>(id1).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);

    // Overwrite the dropper component on id1. The ref count should be the same after this call
    // because we drop the old component to replace it with the one we construct here
    world.add_component(id1, Dropper(counter.clone())).unwrap();

    // Check that we have exactly 3 live handles to the counter and that all the entities agree
    assert_eq!(Arc::strong_count(&counter), 3);
    let comp = world.get_component_ref::<Dropper>(id0).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);
    let comp = world.get_component_ref::<Dropper>(id1).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);

    // Kill one of the entities. This _should_ decrement the ref count
    assert!(world.remove_entity(id0).is_some());
    assert_eq!(Arc::strong_count(&counter), 2);

    // Kill the world. This should drop the other live entity, decrementing the ref count
    drop(world);
    assert_eq!(Arc::strong_count(&counter), 1);
}

#[test]
fn query_test() {
    let mut world = World::new();

    let scale_ids = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
    ));

    let mesh_ids = world.bulk_insert((
        [Position::new(1.5, 2.5), Position::new(3.5, 4.5)],
        [Mesh::new(5), Mesh::new(6)],
    ));

    for (id, (_pos, _scale)) in world.query::<(Read<Position>, Read<Scale>)>() {
        assert!(scale_ids.contains(&id));
    }

    for (id, (_pos, _mesh)) in world.query_mut::<(Read<Position>, Write<Mesh>)>() {
        assert!(mesh_ids.contains(&id));
    }

    {
        let mut query = world.query::<(Read<Position>, Read<Scale>)>();

        let first = query.next().unwrap().0;
        assert!(scale_ids.contains(&first));

        let second = query.next().unwrap().0;
        assert!(scale_ids.contains(&second));

        assert!(query.next().is_none());
        assert!(query.next().is_none());
    }

    {
        let mut query = world.query_mut::<(Read<Position>, Write<Mesh>)>();

        let first = query.next().unwrap().0;
        assert!(mesh_ids.contains(&first));

        let second = query.next().unwrap().0;
        assert!(mesh_ids.contains(&second));

        assert!(query.next().is_none());
        assert!(query.next().is_none());
    }
}

#[test]
fn query_one_test() {
    let mut world = World::new();

    let id = world.spawn_entity();
    world
        .add_component(id, Position { x: 1.0, y: 2.0 })
        .ok()
        .unwrap();

    // It's valid for read-only queries to match the same component multiple times because we are
    // allowed to make aliasing shared references. This isn't useful, but we don't have to validate
    // at runtime like mutable queries do.
    let (pos_a, pos_b) = world
        .query_one::<(Read<Position>, Read<Position>)>(id)
        .unwrap();

    assert_eq!(pos_a, &Position { x: 1.0, y: 2.0 });
    assert_eq!(pos_b, &Position { x: 1.0, y: 2.0 });
    assert!(std::ptr::addr_eq(pos_a, pos_b));
}

#[test]
fn query_one_mut_test() {
    let mut world = World::new();

    let id = world.spawn_entity();
    world
        .add_component(id, Position { x: 1.0, y: 2.0 })
        .ok()
        .unwrap();

    let pos = world.query_one_mut::<Write<Position>>(id).unwrap();

    assert_eq!(pos, &Position { x: 1.0, y: 2.0 });
    *pos = Position { x: 7.0, y: 3.0 };

    let pos = world.query_one_mut::<Write<Position>>(id).unwrap();
    assert_eq!(pos, &Position { x: 7.0, y: 3.0 });
}

#[test]
#[should_panic]
fn query_one_mut_validation_test() {
    let mut world = World::new();

    let id = world.spawn_entity();

    let _ = world.query_one_mut::<(Write<Position>, Write<Position>)>(id);
}

#[test]
fn is_world_send_and_sync() {
    fn the_send_sync_filter<T: Send + Sync>(_v: T) {
        let _do_nothing = ();
    }
    let world = World::new();
    let _nothing = the_send_sync_filter::<World>(world);
}

#[test]
fn remove_matching_test() {
    let mut world = World::new();

    // Counter. Use an arc so we can snoop on when components are dropped via Dropper.
    let counter = Arc::new(());

    let ps_ids = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Scale::new(5.0, 6.0), Scale::new(7.0, 8.0)],
        [Dropper(counter.clone()), Dropper(counter.clone())],
    ));

    let pm_ids = world.bulk_insert((
        [Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        [Mesh::new(9), Mesh::new(10)],
        [Dropper(counter.clone()), Dropper(counter.clone())],
    ));

    assert_eq!(Arc::strong_count(&counter), 5);
    assert_eq!(ps_ids.len(), 2);
    assert_eq!(pm_ids.len(), 2);
    assert_eq!(world.len(), 4);

    for &id in &ps_ids {
        assert!(world.is_live(id));
    }

    for &id in &pm_ids {
        assert!(world.is_live(id));
    }

    world.remove_matching::<Has<Scale>>();

    assert_eq!(world.len(), 2);
    assert_eq!(Arc::strong_count(&counter), 3);

    for &id in &ps_ids {
        assert!(!world.is_live(id));
    }

    for &id in &pm_ids {
        assert!(world.is_live(id));
    }

    drop(world);

    assert_eq!(Arc::strong_count(&counter), 1);
}
