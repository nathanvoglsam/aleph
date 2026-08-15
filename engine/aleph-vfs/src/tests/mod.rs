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

use std::io;
use std::io::Read;

use camino::Utf8PathBuf;
use crossbeam::channel::unbounded;

use crate::async_io::IoQueue;
use crate::async_io::top_level_handle_cache::TopLevelHandleCache;
use crate::directory_layer::DirectoryLayer;
use crate::file::AsyncReadResponse;
use crate::{IRouterExt, LayerDesc, Router};

#[test]
pub fn empty_vfs_finds_no_files() {
    let router = Router::new([]).unwrap();

    match router.open("this_file_does_not_exist") {
        Ok(_) => {
            panic!("How did we open a file that doesn't exist?");
        }
        Err(e) => {
            assert!(matches!(e.kind(), io::ErrorKind::NotFound));
        }
    }
}

#[test]
pub fn empty_path_invalid_filename() {
    let router = Router::new([]).unwrap();

    match router.open("") {
        Ok(_) => {
            panic!("Shouldn't be able to open the empty path.");
        }
        Err(e) => {
            assert!(matches!(e.kind(), io::ErrorKind::InvalidFilename));
        }
    }
}

#[test]
pub fn empty_vfs_no_dots_in_path() {
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_a")),
    }];

    let router = Router::new(layers).unwrap();

    let err = router.open(".").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    let err = router.open("./.").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    let err = router.open("../src/uhoh.txt").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    // The vfs itself only checks the root component for '.' and '..' components. Because we've
    // mounted nothing then there's no layer to check anything after the first segment.
    //
    // This means we'll actually fail finding a layer at 'nothing.txt' instead, the rest of the
    // path is ignored.
    let err = router.open("nothing.txt/../../../../here").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotFound));
}

#[test]
pub fn root_is_implicitly_a_directory() {
    let router = Router::new([]).unwrap();

    match router.open("/") {
        Ok(_) => {
            panic!("Shouldn't be able to open the root.");
        }
        Err(e) => {
            assert!(matches!(e.kind(), io::ErrorKind::IsADirectory));
        }
    }
}

#[test]
pub fn single_directory_mount() {
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_a")),
    }];

    let router = Router::new(layers).unwrap();

    let err = router.open("/package_a").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::IsADirectory));

    let err = router.open("/package_b").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotFound));

    let file = router.open("/package_a/file.txt").unwrap();
    let mut reader = file.reader();

    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    assert_eq!(string, "Hello, World!");

    let err = router.open("package_b/file.text").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotFound));

    let err = router.open_async("/package_a/file.txt").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::Unsupported));
}

#[test]
pub fn single_directory_mount_bad_root_not_found() {
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_no_exist")),
    }];

    let err = Router::new(layers).err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotFound));
}

#[test]
pub fn single_directory_mount_bad_root_not_a_directory() {
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_a/file.txt")),
    }];

    let err = Router::new(layers).err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotADirectory));
}

#[test]
pub fn directory_mount_dots_in_path() {
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_a")),
    }];

    let router = Router::new(layers).unwrap();

    let err = router.open("package_a/.").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    let err = router.open("package_a/./.").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    let err = router.open("package_a/../src/uhoh.txt").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));

    let err = router
        .open("package_a/nothing.txt/to.png/see/../../../../here")
        .err()
        .unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::InvalidFilename));
}

#[test]
pub fn multi_directory_mount() {
    let layers = [
        LayerDesc {
            mount_name: "package_a",
            layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_a")),
        },
        LayerDesc {
            mount_name: "package_b",
            layer: DirectoryLayer::new(Utf8PathBuf::from("./test-data/package_b")),
        },
    ];

    let router = Router::new(layers).unwrap();

    let err = router.open("/package_a").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::IsADirectory));

    let err = router.open("/package_b").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::IsADirectory));

    let err = router.open("/package_b/folder").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::IsADirectory));

    let err = router.open("/package_b/folder/no.txt").err().unwrap();
    assert!(matches!(err.kind(), io::ErrorKind::NotFound));

    let file = router.open("/package_a/file.txt").unwrap();
    let mut reader = file.reader();
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    assert_eq!(string, "Hello, World!");

    let file = router.open("/package_b/file.txt").unwrap();
    let mut reader = file.reader();
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    assert_eq!(string, "Hello, from Package B!");

    let file = router.open("/package_b/folder/file.txt").unwrap();
    let mut reader = file.reader();
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    assert_eq!(string, "Hello, Subdir!");
}

#[test]
pub fn async_read_test() {
    let queue = IoQueue::new(TopLevelHandleCache::new(2));
    let layers = [LayerDesc {
        mount_name: "package_a",
        layer: DirectoryLayer::new_with_io_queue(Utf8PathBuf::from("./test-data/package_a"), queue),
    }];

    let router = Router::new(layers).unwrap();

    let file = router.open("/package_a/file.txt").unwrap();
    let mut reader = file.reader();

    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    assert_eq!(string, "Hello, World!");

    let (sender, receiver) = unbounded();

    let file = router.open_async("/package_a/file.txt").unwrap();
    file.load(sender, 21).unwrap();

    let result = receiver.recv().unwrap();
    match result {
        AsyncReadResponse::LoadSuccess { data, cookie, .. } => {
            let data = String::from_utf8(data).unwrap();
            assert_eq!(data, "Hello, World!");
            assert_eq!(cookie, 21);
        }
        _ => panic!("Unexpected response"),
    }
}
