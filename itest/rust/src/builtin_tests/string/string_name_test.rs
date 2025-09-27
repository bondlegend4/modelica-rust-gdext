/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use godot::builtin::{static_sname, Encoding, GString, NodePath, StringName};

use crate::framework::{assert_eq_self, itest};

#[itest]
fn string_name_default() {
    let name = StringName::default();
    let back = GString::from(&name);

    assert_eq!(back, GString::new());
}

#[itest]
fn string_name_conversion() {
    // Note: StringName::from(&str) uses direct FFI constructor from Godot 4.2 onwards.

    let string = GString::from("some string");
    let name = StringName::from(&string);
    let back = GString::from(&name);

    assert_eq!(string, back);
}

#[itest]
fn string_name_node_path_conversion() {
    let string = StringName::from("some string");
    let name = NodePath::from(&string);
    let back = StringName::from(&name);

    assert_eq!(string, back);
}

#[itest]
fn string_name_to_gstring() {
    let string_name = StringName::from("test string");
    assert_eq!(string_name.to_gstring(), GString::from("test string"));
}

#[itest]
fn string_name_to_node_path() {
    let string_name = StringName::from("path/to/node");
    assert_eq!(string_name.to_node_path(), NodePath::from("path/to/node"));
}

#[itest]
fn string_name_equality() {
    let string = StringName::from("some string");
    let second = StringName::from("some string");
    let different = StringName::from("some");

    assert_eq!(string, second);
    assert_ne!(string, different);
}

#[itest]
#[allow(clippy::eq_op)]
fn string_name_transient_ord() {
    // We can't deterministically know the ordering, so this test only ensures consistency between different operators.
    let low = StringName::from("Alpha");
    let high = StringName::from("Beta");

    let mut low = low.transient_ord();
    let mut high = high.transient_ord();

    if low > high {
        std::mem::swap(&mut low, &mut high);
    }

    assert!(low < high);
    assert!(low <= high);
    assert!(high > low); // implied.
    assert!(high >= low);

    // Check PartialEq/Eq relation.
    assert_eq_self!(low);
    assert_eq_self!(high);
    assert!(low != high);
}

#[itest]
fn string_name_clone() {
    let first = StringName::from("some string");
    #[allow(clippy::redundant_clone)]
    let cloned = first.clone();

    assert_eq!(first, cloned);
}

#[itest]
fn string_name_hash() {
    let set: HashSet<StringName> = [
        "string_1",
        "SECOND string! :D",
        "emoji time: 😎",
        r#"got/!()%)=!"/]}¡[$½{¥¡}@£symbol characters"#,
        "some garbageTƉ馧쟻�韂󥢛ꮛ૎ཾ̶D@/8ݚ򹾴-䌗򤷨񄣷8",
    ]
    .into_iter()
    .map(StringName::from)
    .collect();
    assert_eq!(set.len(), 5);
}

#[itest]
fn string_name_length() {
    let string = "hello!";
    let name = StringName::from(string);
    assert_eq!(name.len(), string.len());

    let empty = StringName::default();
    assert_eq!(empty.len(), 0);
}

#[itest]
fn string_name_is_empty() {
    let name = StringName::from("hello!");
    assert!(!name.is_empty());
    let empty = StringName::default();
    assert!(empty.is_empty());
}

#[itest]
fn string_name_from_cstr() {
    use std::ffi::CStr;

    let cases: [(&CStr, &str); 3] = [
        (c"pure ASCII\t[~]", "pure ASCII\t[~]"),
        (c"\xB1", "±"),
        (c"Latin-1 \xA3 \xB1 text \xBE", "Latin-1 £ ± text ¾"),
    ];

    for (bytes, string) in cases.into_iter() {
        let a = StringName::__cstr(bytes);
        let b = StringName::from(string);

        assert_eq!(a, b);
    }
}

#[itest]
fn string_name_static_sname() {
    let a = static_sname!(c"pure ASCII\t[~]").clone();
    let b = StringName::from("pure ASCII\t[~]");

    assert_eq!(a, b);

    let a1 = a.clone();
    let a2 = static_sname!(c"pure ASCII\t[~]").clone();

    assert_eq!(a, a1);
    assert_eq!(a1, a2);

    let a = static_sname!(c"\xB1").clone();
    let b = StringName::from("±");

    assert_eq!(a, b);

    let a = static_sname!(c"Latin-1 \xA3 \xB1 text \xBE").clone();
    let b = StringName::from("Latin-1 £ ± text ¾");

    assert_eq!(a, b);
}

#[itest]
fn string_name_with_null() {
    // Godot always ignores bytes after a null byte.
    let cases: &[(&str, &str)] = &[
        (
            "some random string",
            "some random string\0 with a null byte",
        ),
        ("", "\0"),
    ];

    for (left, right) in cases.iter() {
        let left = StringName::from(*left);
        let right = StringName::from(*right);

        assert_eq!(left, right);
    }
}

// Byte and C-string conversions.
crate::generate_string_bytes_and_cstr_tests!(
    builtin: StringName,
    tests: [
        string_name_from_bytes_ascii,
        string_name_from_cstr_ascii,
        string_name_from_bytes_latin1,
        string_name_from_cstr_latin1,
        string_name_from_bytes_utf8,
        string_name_from_cstr_utf8,
    ]
);

crate::generate_string_fmt_conversion_tests!(
    builtin: StringName,
    tests: [
        string_name_display,
        string_name_standard_pad,
        str_to_godot_strings_trait,
        string_to_godot_strings_trait,
    ]
);
