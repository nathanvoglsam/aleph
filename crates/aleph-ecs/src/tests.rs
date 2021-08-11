use crate::World;

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

#[derive(Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct Scale {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct Mesh {
    a: usize,
}

#[test]
fn extend_test_vec() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        vec![Position::default(), Position::default()],
        vec![Scale::default(), Scale::default()],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);
}

#[test]
fn extend_test_array() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids = world.extend((
        [Position::default(), Position::default()],
        [Scale::default(), Scale::default()],
    ));

    assert_eq!(ids.len(), 2);
    assert_eq!(world.len(), 2);
}

#[test]
fn remove_entity_array() {
    let mut world = World::new(Default::default()).unwrap();

    world.register::<Position>();
    world.register::<Scale>();
    world.register::<Mesh>();

    let ids_a = world.extend((
        [Position::default(), Position::default()],
        [Scale::default(), Scale::default()],
    ));

    assert_eq!(ids_a.len(), 2);
    assert_eq!(world.len(), 2);

    assert!(world.remove_entity(ids_a[0]));
    assert_eq!(world.len(), 1);

    assert!(!world.remove_entity(ids_a[0]));
    assert_eq!(world.len(), 1);

    assert!(world.remove_entity(ids_a[1]));
    assert_eq!(world.len(), 0);

    assert!(!world.remove_entity(ids_a[1]));
    assert_eq!(world.len(), 0);

    let ids_b = world.extend((
        [Position::default(), Position::default()],
        [Mesh::default(), Mesh::default()],
    ));

    assert_eq!(ids_b.len(), 2);
    assert_eq!(world.len(), 2);

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

    assert!(world.remove_component::<Scale>(ids[1]));

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
