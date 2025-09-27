/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use godot::builtin::{GString, NodePath, StringName};
use godot::meta::wrapped;

use crate::framework::{expect_debug_panic_or_release_ok, itest};

#[itest]
fn node_path_default() {
    let name = NodePath::default();
    let back = GString::from(&name);

    assert_eq!(back, GString::new());
}

#[itest]
fn node_path_conversion() {
    let string = GString::from("some string");
    let name = NodePath::from(&string);
    let back = GString::from(&name);

    assert_eq!(string, back);
}

#[itest]
fn node_path_to_gstring() {
    let node_path = NodePath::from("path/to/node");
    assert_eq!(node_path.to_gstring(), GString::from("path/to/node"));
}

#[itest]
fn node_path_to_string_name() {
    let node_path = NodePath::from("test string");
    assert_eq!(node_path.to_string_name(), StringName::from("test string"));
}

#[itest]
fn node_path_equality() {
    let string = NodePath::from("some string");
    let second = NodePath::from("some string");
    let different = NodePath::from("some");

    assert_eq!(string, second);
    assert_ne!(string, different);
}

#[itest]
fn node_path_clone() {
    let first = NodePath::from("some string");
    #[allow(clippy::redundant_clone)]
    let cloned = first.clone();

    assert_eq!(first, cloned);
}

#[itest]
fn node_path_hash() {
    let set: HashSet<NodePath> = [
        "string_1",
        "SECOND string! :D",
        "emoji time: 😎",
        r#"got/!()%)=!"/]}¡[$½{¥¡}@£symbol characters"#,
        "some garbageTƉ馧쟻�韂󥢛ꮛ૎ཾ̶D@/8ݚ򹾴-䌗򤷨񄣷8",
    ]
    .into_iter()
    .map(NodePath::from)
    .collect();
    assert_eq!(set.len(), 5);
}

#[itest]
fn node_path_with_null() {
    // Godot always ignores bytes after a null byte.
    let cases: &[(&str, &str)] = &[
        (
            "some random string",
            "some random string\0 with a null byte",
        ),
        ("", "\0"),
    ];

    for (left, right) in cases.iter() {
        let left = NodePath::from(*left);
        let right = NodePath::from(*right);

        assert_eq!(left, right);
    }
}

#[itest]
#[cfg(since_api = "4.3")]
#[allow(clippy::reversed_empty_ranges)]
fn node_path_subpath() {
    let path = NodePath::from("path/to/Node:with:props");
    let parts = path.get_name_count() + path.get_subname_count();

    assert_eq!(path.subpath(0..1), "path".into());
    assert_eq!(path.subpath(1..2), "to".into());
    assert_eq!(path.subpath(2..3), "Node".into());
    assert_eq!(path.subpath(3..4), ":with".into());
    assert_eq!(path.subpath(4..5), ":props".into());

    assert_eq!(path.subpath(wrapped(1..-1)), "to/Node:with".into());
    assert_eq!(
        path.subpath(wrapped(1..parts as i32 - 1)),
        "to/Node:with".into()
    );
    assert_eq!(path.subpath(wrapped(0..-2)), "path/to/Node".into());
    assert_eq!(path.subpath(wrapped(-3..-1)), "Node:with".into());
    assert_eq!(path.subpath(wrapped(-2..)), ":with:props".into());
    assert_eq!(path.subpath(wrapped(-1..)), ":props".into());
}

#[itest]
fn node_path_get_name() {
    let path = NodePath::from("../RigidBody2D/Sprite2D");
    assert_eq!(path.get_name(0), "..".into());
    assert_eq!(path.get_name(1), "RigidBody2D".into());
    assert_eq!(path.get_name(2), "Sprite2D".into());

    expect_debug_panic_or_release_ok("NodePath::get_name() out of bounds", || {
        assert_eq!(path.get_name(3), "".into());
    })
}

#[itest]
fn node_path_get_subname() {
    let path = NodePath::from("Sprite2D:texture:resource_name");
    assert_eq!(path.get_subname(0), "texture".into());
    assert_eq!(path.get_subname(1), "resource_name".into());

    expect_debug_panic_or_release_ok("NodePath::get_subname() out of bounds", || {
        assert_eq!(path.get_subname(2), "".into());
    })
}

crate::generate_string_fmt_conversion_tests!(
    builtin: NodePath,
    tests: [
        node_path_display,
        node_path_standard_pad,
        str_to_godot_strings_trait,
        string_to_godot_strings_trait,
    ]
);
