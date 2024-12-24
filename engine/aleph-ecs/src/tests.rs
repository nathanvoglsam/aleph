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

use object_system::unsafe_impl_iobject;

use crate::World;

#[derive(Clone, Default, PartialEq, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}
unsafe_impl_iobject!(Position, "01922977-e48a-7f00-83a5-d78e50e77567");

#[derive(Clone, PartialEq, Debug)]
struct BigComponent {
    pub data: [u8; 64],
}
unsafe_impl_iobject!(BigComponent, "019395de-d02d-7702-89c1-6bdeb67bb7ce");

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
unsafe_impl_iobject!(Scale, "01922978-21d5-7c83-aa72-025c9daae51c");

impl Scale {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, PartialEq, Debug)]
struct Mesh {
    pub a: usize,
}
unsafe_impl_iobject!(Mesh, "01922978-231e-7381-b173-6ecff48c4774");

impl Mesh {
    pub fn new(a: usize) -> Self {
        Self { a }
    }
}

#[derive(Default, Debug)]
struct Dropper(Arc<()>);
unsafe_impl_iobject!(Dropper, "01922979-2550-7d31-b91a-6c05bab9bb3b");

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
fn extend_test_vec_large() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let positions: Vec<_> = (0..20_000)
        .enumerate()
        .map(|(i, _)| Position::new(1.0 * i as f32, 2.0 * i as f32))
        .collect();

    let scales: Vec<_> = (0..20_000)
        .enumerate()
        .map(|(i, _)| Scale::new(3.0 * i as f32, 5.0 * i as f32))
        .collect();

    let ids = world.extend((positions.clone(), scales.clone()));

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for ((id, position), scale) in ids.iter().zip(positions).zip(scales) {
        let (stored_position, stored_scale) = world.query_one::<(&Position, &Scale)>(*id).unwrap();
        assert_eq!(position, stored_position.clone());
        assert_eq!(scale, stored_scale.clone());
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
}

#[test]
fn extend_test_vec_larger() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<BigComponent>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let components: Vec<_> = (0..20_000)
        .map(|_| BigComponent { data: [56; 64] })
        .collect();

    let ids = world.extend((components.clone(),));

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for (id, component) in ids.iter().zip(components) {
        let stored_component = world.query_one::<&BigComponent>(*id).unwrap();
        assert_eq!(&component, stored_component);
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
}

#[test]
fn extend_test_vec_larger_extend_one() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<BigComponent>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let components: Vec<_> = (0..20_000)
        .map(|_| BigComponent { data: [56; 64] })
        .collect();

    let mut ids = Vec::new();
    for component in components.iter() {
        ids.push(world.extend_one((component.clone(),)));
    }

    assert_eq!(ids.len(), 20_000);
    assert_eq!(world.len(), 20_000);

    let mut iter_count = 20000u32;
    for (id, component) in ids.iter().zip(components) {
        let stored_component = world.query_one::<&BigComponent>(*id).unwrap();
        assert_eq!(&component, stored_component);
        iter_count -= 1;
    }
    assert_eq!(iter_count, 0);
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
fn extend_test_one() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let mut ids = Vec::new();
    ids.push(world.extend_one((Position::new(1.0, 2.0), Scale::new(5.0, 6.0))));
    ids.push(world.extend_one((Position::new(3.0, 4.0), Scale::new(7.0, 8.0))));

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
#[should_panic]
fn extend_test_non_matched_sizes() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let _ids = world.extend((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Scale::new(5.0, 6.0)],
    ));
}

#[test]
#[should_panic]
fn extend_test_duplicate_components() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let _ids = world.extend((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Position::new(5.0, 6.0), Position::new(7.0, 8.0)],
    ));
}

#[test]
#[should_panic]
fn extend_test_duplicate_components_2() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let _id = world.extend_one((Position::new(1.0, 2.0), Position::new(5.0, 6.0)));
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

    let counter = Arc::new(());

    let ids = world.extend((
        vec![Position::new(1.0, 2.0), Position::new(3.0, 4.0)],
        vec![Dropper(counter.clone()), Dropper(counter.clone())],
    ));

    assert_eq!(Arc::strong_count(&counter), 3);
    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);

    let comp = world.query_one::<&Dropper>(ids[0]).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);

    let comp = world.query_one::<&Dropper>(ids[1]).unwrap();
    assert_eq!(Arc::strong_count(&comp.0), 3);

    assert!(world.remove_entity(ids[0]));

    assert_eq!(Arc::strong_count(&counter), 2);

    drop(world);

    assert_eq!(Arc::strong_count(&counter), 1);
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
fn is_world_send_and_sync() {
    fn the_send_sync_filter<T: Send + Sync>(_v: T) {
        let _do_nothing = ();
    }
    let world = World::new(Default::default()).unwrap();
    let _nothing = the_send_sync_filter::<World>(world);
}
