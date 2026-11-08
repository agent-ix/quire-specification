// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-154 "Map types" (TC-234 I35, I36): a domain package map type declares
//! its key type, its value type and its entry-count bound, and its key type is
//! a scalar key type. The published `package_map_types` cases of
//! `map-type-vectors.json` are run through the rule as FR-154 states it.

use ix_trace_rs::trace;
use serde_json::{Value, json};

const VECTORS: &str = include_str!("../proposals/checked-package-v2/map-type-vectors.json");

/// FR-154's native value type names.
const NATIVES: [&str; 14] = [
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
    "Clock",
];

/// FR-154's scalar key natives: every native but `Rational`, `Decimal` and `Clock`.
const KEY_NATIVES: [&str; 11] = [
    "Boolean",
    "Integer",
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

/// Members another kind declares, refused on a `map`.
const OTHER_KIND_MEMBERS: [&str; 10] = [
    "fields",
    "identityFields",
    "operations",
    "supertypes",
    "clauses",
    "positions",
    "variants",
    "items",
    "inner",
    "target",
];

/// Kinds that are no type, so no `keys` may name one.
const NON_TYPE_KINDS: [&str; 5] = [
    "event",
    "state_machine",
    "process",
    "persistence_interface",
    "namespace",
];

fn parsed(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn refused(declared: &Value, code: &str, cause: &str, member: &str) -> Value {
    json!({"refused": {
        "code": code,
        "cause": cause,
        "at": declared["identity"],
        "path": format!("TypeDefinition.{member}"),
    }})
}

fn malformed(declared: &Value, member: &str) -> Value {
    refused(
        declared,
        "invalid_model_binding",
        "malformed-declaration",
        member,
    )
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
    Unit,
    Declared(&'a Value),
}

/// The type a well-formed `typeRef` names, following aliases, or `None` when
/// a package identity names no node.
fn resolve<'a>(package: &'a Value, type_ref: &'a str) -> Option<Named<'a>> {
    if let Some(name) = type_ref.strip_prefix("ix://quire/native/") {
        return Some(Named::Native(name));
    }
    if type_ref.starts_with("ix://quire/unit/") {
        return Some(Named::Unit);
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
        Some(name) => NATIVES.contains(&name),
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

/// FR-154's scalar key type: a key native, a unit, a value type bound as
/// `Int[..]`, `Text[..]` or `Rational[..]`, or a variant type with no
/// payload-bearing case.
fn scalar_key(named: &Named<'_>) -> bool {
    match named {
        Named::Native(name) => KEY_NATIVES.contains(name),
        Named::Unit => true,
        Named::Declared(found) => match found["kind"].as_str() {
            Some("value") => matches!(
                found["scalar"].as_str(),
                Some("integer" | "string" | "rational")
            ),
            Some("variant") => found["variants"]
                .as_array()
                .is_some_and(|cases| cases.iter().all(|case| case.get("payloadType").is_none())),
            _ => false,
        },
    }
}

/// A `typeRef` member of a map: the absent, malformed and unresolved
/// refusals, in FR-154's order, or the resolved type.
fn reference<'a>(package: &'a Value, map: &'a Value, member: &str) -> Result<Named<'a>, Value> {
    let type_ref = map
        .get(member)
        .and_then(Value::as_str)
        .filter(|type_ref| well_formed(package, type_ref))
        .ok_or_else(|| malformed(map, member))?;
    let named = resolve(package, type_ref)
        .ok_or_else(|| refused(map, "missing_declaration", "missing-name", member))?;
    if let Named::Declared(found) = &named
        && NON_TYPE_KINDS.contains(&found["kind"].as_str().unwrap_or_default())
    {
        return Err(malformed(map, member));
    }
    Ok(named)
}

/// FR-154 "Map types" over one type definition: `None` for a definition of
/// another kind with no map member, the FR-322 row of a map, or the first
/// refusal in the stated order.
fn read_type(package: &Value, declared: &Value) -> Option<Value> {
    if declared["kind"] != "map" {
        let member = ["keys", "bound"]
            .into_iter()
            .find(|member| declared.get(*member).is_some())?;
        return Some(malformed(declared, member));
    }
    Some(read_map(package, declared).unwrap_or_else(|refusal| refusal))
}

fn read_map(package: &Value, map: &Value) -> Result<Value, Value> {
    let members = map.as_object().expect("a type definition object");
    if let Some(extra) = members
        .keys()
        .find(|member| OTHER_KIND_MEMBERS.contains(&member.as_str()))
    {
        return Err(malformed(map, extra));
    }
    if !scalar_key(&reference(package, map, "keys")?) {
        return Err(refused(map, "ill_typed", "operator-ineligible", "keys"));
    }
    reference(package, map, "values")?;
    let bound = map
        .get("bound")
        .and_then(Value::as_object)
        .filter(|bound| {
            bound
                .keys()
                .all(|member| member == "lower" || member == "upper")
                && bound.get("lower").is_some_and(Value::is_u64)
                && bound.get("upper").is_none_or(Value::is_u64)
        })
        .ok_or_else(|| malformed(map, "bound"))?;
    let lower = bound["lower"].as_u64().unwrap_or_default();
    let upper = bound.get("upper").and_then(Value::as_u64);
    if upper.map_or(lower > 0, |upper| lower > upper) {
        return Err(refused(
            map,
            "invalid_model_binding",
            "unpreserved-model-meaning",
            "bound",
        ));
    }
    // FR-322 "Selected timed package": a native key by its name, any other key
    // and the value by their references.
    let keys = map["keys"].as_str().unwrap_or_default();
    let key = keys.strip_prefix("ix://quire/native/").unwrap_or(keys);
    Ok(json!({"admitted": true, "reads": {
        "rendered": format!("Map<{key}, {}>", map["values"].as_str().unwrap_or_default()),
        "min": lower.to_string(),
        "max": upper.map_or_else(|| "unbounded".to_owned(), |upper| upper.to_string()),
    }}))
}

/// The first refusal over a package's type definitions in order, or the row
/// of the map `identity` when none refuses.
fn read_package(package: &Value, identity: &str) -> Value {
    let types = package["types"].as_array().expect("types");
    types
        .iter()
        .filter_map(|declared| read_type(package, declared))
        .find(|outcome| outcome.get("refused").is_some())
        .unwrap_or_else(|| {
            read_type(package, declared(package, identity).expect("the map")).expect("a map reads")
        })
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

#[trace("TC-234", "FR-154-AC-48", "FR-154-AC-49", "FR-322-AC-396")]
#[test]
fn tc_234_package_map_types_declare_a_scalar_key_and_a_bound() {
    let vectors = parsed(VECTORS);
    let section = &vectors["package_map_types"];
    for case in section["cases"].as_array().expect("cases") {
        let mut package = section["base"].clone();
        for patch in case["patch"].as_array().expect("patch") {
            apply(&mut package, patch);
        }
        assert_eq!(
            read_package(&package, "ix://acme/maps/M"),
            case["expected"],
            "{}",
            case["id"]
        );
    }
}

/// Both published timed documents and the orders document declare `ClockMap`
/// as `Map<Text, Clock>` with no maximum, read from its `keys` and `bound`.
#[trace("TC-234", "FR-154-AC-27", "FR-154-AC-48")]
#[test]
fn tc_234_published_map_types_declare_keys_and_bound() {
    for document in [
        include_str!("../proposals/checked-package-v2/domain-package-acme-timed-slots.json"),
        include_str!("../proposals/checked-package-v2/domain-package-acme-timed-clauses.json"),
        include_str!("../proposals/checked-package-v2/domain-package-acme-orders.json"),
    ] {
        let mut document = parsed(document);
        document["identity"] = document["package"]["identity"].clone();
        let clock_map = declared(&document, "ix://acme/orders/ClockMap").expect("ClockMap");
        assert_eq!(
            read_map(&document, clock_map),
            Ok(json!({"admitted": true, "reads": {
                "rendered": "Map<Text, ix://quire/native/Clock>",
                "min": "0",
                "max": "unbounded",
            }}))
        );
    }
}
