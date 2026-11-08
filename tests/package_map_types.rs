// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-154 "Map types" (TC-234 I35, I36): a domain package map type declares
//! its key type, its value type and its entry-count bound, and its key type is
//! a scalar key type. The published `package_map_types` cases of
//! `map-type-vectors.json` are run through the rule as FR-154 states it.

use ix_trace_rs::trace;
use serde_json::{Value, json};

const VECTORS: &str = include_str!("../proposals/checked-package-v2/map-type-vectors.json");

/// FR-154's native value type names other than `Clock`.
const SCALAR_NATIVES: [&str; 13] = [
    "Boolean",
    "Integer",
    "Rational",
    "Decimal",
    "Float32",
    "Float64",
    "Text",
    "String",
    "UUID",
    "Timestamp",
    "Bytes",
    "Duration",
    "JsonObject",
];

fn parsed(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn refused(map: &Value, code: &str, cause: &str, member: &str) -> Value {
    json!({"refused": {
        "code": code,
        "cause": cause,
        "at": map["identity"],
        "path": format!("TypeDefinition.{member}"),
    }})
}

fn declared<'a>(package: &'a Value, identity: &str) -> Option<&'a Value> {
    package["types"]
        .as_array()?
        .iter()
        .find(|declared| declared["identity"] == identity)
}

/// A reference to a type, after its alias chain.
enum Named<'a> {
    Native(&'a str),
    Unit(&'a str),
    Declared(&'a Value),
}

/// The type a well-formed `typeRef` names, following aliases, or `None` when
/// a package identity names no node.
fn resolve<'a>(package: &'a Value, type_ref: &'a str) -> Option<Named<'a>> {
    if let Some(name) = type_ref.strip_prefix("ix://quire/native/") {
        return Some(Named::Native(name));
    }
    if let Some(name) = type_ref.strip_prefix("ix://quire/unit/") {
        return Some(Named::Unit(name));
    }
    let found = declared(package, type_ref)?;
    if found["kind"] == "alias" {
        return resolve(package, found["target"].as_str()?);
    }
    Some(Named::Declared(found))
}

/// FR-154's reference form: a native name, a non-empty unit name or a package
/// type identity.
fn well_formed(package: &Value, type_ref: &str) -> bool {
    let prefix = format!("ix://{}/", package["identity"].as_str().unwrap_or_default());
    match type_ref.strip_prefix("ix://quire/native/") {
        Some(name) => name == "Clock" || SCALAR_NATIVES.contains(&name),
        None => {
            type_ref
                .strip_prefix("ix://quire/unit/")
                .is_some_and(|name| !name.is_empty())
                || type_ref
                    .strip_prefix(&prefix)
                    .is_some_and(|name| !name.is_empty())
        }
    }
}

/// FR-154's scalar key type: a native other than `Clock`, a unit, a bounded
/// value type or a variant type with no payload-bearing case.
fn scalar_key(named: &Named<'_>) -> bool {
    match named {
        Named::Native(name) => SCALAR_NATIVES.contains(name),
        Named::Unit(_) => true,
        Named::Declared(found) => match found["kind"].as_str() {
            Some("value") => true,
            Some("variant") => found["variants"]
                .as_array()
                .is_some_and(|cases| cases.iter().all(|case| case.get("payloadType").is_none())),
            _ => false,
        },
    }
}

/// The FR-322 rendering of a resolved type, as the vectors write it.
fn rendered(named: &Named<'_>) -> String {
    match named {
        Named::Native(name) => format!("ix://quire/native/{name}"),
        Named::Unit(name) => format!("Quantity<{name}>"),
        Named::Declared(found) if found["kind"] == "value" => format!(
            "Int[{}, {}]",
            found["min"].as_str().unwrap_or_default(),
            found["max"].as_str().unwrap_or_default()
        ),
        Named::Declared(found) => found["identity"].as_str().unwrap_or_default().to_owned(),
    }
}

/// FR-154 "Map types" over one map type definition: the type it reads as, or
/// the first refusal in the stated order.
fn read_map(package: &Value, map: &Value) -> Value {
    let Some(keys) = map.get("keys") else {
        return refused(
            map,
            "invalid_model_binding",
            "malformed-declaration",
            "keys",
        );
    };
    let Some(keys) = keys.as_str().filter(|keys| well_formed(package, keys)) else {
        return refused(
            map,
            "invalid_model_binding",
            "malformed-declaration",
            "keys",
        );
    };
    let Some(key) = resolve(package, keys) else {
        return refused(map, "missing_declaration", "missing-name", "keys");
    };
    if !scalar_key(&key) {
        return refused(map, "ill_typed", "operator-ineligible", "keys");
    }
    let bound = map.get("bound").and_then(Value::as_object).filter(|bound| {
        bound
            .keys()
            .all(|member| member == "lower" || member == "upper")
            && bound.get("lower").is_some_and(Value::is_u64)
            && bound.get("upper").is_none_or(Value::is_u64)
    });
    let Some(bound) = bound else {
        return refused(
            map,
            "invalid_model_binding",
            "malformed-declaration",
            "bound",
        );
    };
    let lower = bound["lower"].as_u64().unwrap_or_default();
    let upper = bound.get("upper").and_then(Value::as_u64);
    if upper.is_some_and(|upper| lower > upper) {
        return refused(
            map,
            "invalid_model_binding",
            "unpreserved-model-meaning",
            "bound",
        );
    }
    let value =
        resolve(package, map["values"].as_str().unwrap_or_default()).expect("values resolves");
    let suffix = match upper {
        Some(upper) => format!("[{lower}, {upper}]"),
        None if lower == 0 => String::new(),
        None => format!("[{lower}, unbounded]"),
    };
    json!({"admitted": true, "reads": format!("Map<{}, {}>{suffix}", rendered(&key), rendered(&value))})
}

/// Applies the RFC 6902 `add`, `remove` and `replace` operations the cases use.
fn apply(document: &mut Value, patch: &Value) {
    let path = patch["path"].as_str().expect("patch path");
    let (parent, member) = path.rsplit_once('/').expect("patch path has a member");
    let target = document.pointer_mut(parent).expect("patch parent exists");
    let map = target
        .as_object_mut()
        .expect("the cases patch object members");
    match patch["op"].as_str().expect("patch op") {
        "add" | "replace" => {
            map.insert(member.to_owned(), patch["value"].clone());
        }
        "remove" => {
            map.remove(member).expect("removed member exists");
        }
        other => panic!("unsupported patch {other} at {path}"),
    }
}

/// Every map type definition of a package, read in document order.
fn read_maps(package: &Value) -> Vec<Value> {
    package["types"]
        .as_array()
        .expect("types")
        .iter()
        .filter(|declared| declared["kind"] == "map")
        .map(|map| read_map(package, map))
        .collect()
}

#[trace("TC-234", "FR-154-AC-48", "FR-154-AC-49")]
#[test]
fn tc_234_package_map_types_declare_a_scalar_key_and_a_bound() {
    let vectors = parsed(VECTORS);
    let section = &vectors["package_map_types"];
    let base = &section["base"];
    // The base's other map type is admitted, so a case's outcome is the map under test's.
    let others = read_maps(base);
    assert!(
        others.iter().all(|outcome| outcome["admitted"] == true),
        "{others:?}"
    );
    let cases = section["cases"].as_array().expect("cases");
    assert_eq!(cases.len(), 27);
    for (at, case) in cases.iter().enumerate() {
        assert_eq!(case["id"], format!("PM-{:02}", at + 1));
        let mut package = base.clone();
        for patch in case["patch"].as_array().expect("patch") {
            apply(&mut package, patch);
        }
        let map = declared(&package, "ix://acme/maps/M").expect("the map under test");
        assert_eq!(read_map(&package, map), case["expected"], "{}", case["id"]);
        let criterion = if case["expected"]["admitted"] == true {
            "FR-154-AC-48"
        } else {
            "FR-154-AC-49"
        };
        assert_eq!(case["criterion"], criterion, "{}", case["id"]);
    }
}

/// QSL FR-230-AC-19 declares `m: Map<Text, Timer>[0, 2]`, and both published
/// timed documents declare `ClockMap` as `Map<Text, Clock>`, with no maximum:
/// every published map type reads with its declared key and bound.
#[trace("TC-234", "FR-154-AC-27", "FR-154-AC-48")]
#[test]
fn tc_234_published_map_types_declare_keys_and_bound() {
    let vectors = parsed(VECTORS);
    assert_eq!(
        read_map(
            &vectors["package_map_types"]["base"],
            &vectors["package_map_types"]["base"]["types"][13]
        ),
        json!({"admitted": true, "reads": "Map<ix://quire/native/Text, ix://acme/maps/Timer>[0, 2]"})
    );
    for document in [
        include_str!("../proposals/checked-package-v2/domain-package-acme-timed-slots.json"),
        include_str!("../proposals/checked-package-v2/domain-package-acme-timed-clauses.json"),
        include_str!("../proposals/checked-package-v2/domain-package-acme-orders.json"),
    ] {
        let mut document = parsed(document);
        document["identity"] = document["package"]["identity"].clone();
        assert_eq!(
            read_maps(&document),
            [
                json!({"admitted": true, "reads": "Map<ix://quire/native/Text, ix://quire/native/Clock>"})
            ]
        );
    }
}
