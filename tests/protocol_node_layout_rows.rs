// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! STD-650: the schema decides the layout of every protocol control
//! application. A protocol node's body is `ProtocolControlApplication`, so a
//! layout or enumeration violation of FR-464/FR-465/FR-466 is refused at the
//! schema stage as `malformed_wire`, and the first error path of the strict
//! validator is the locus. The 26 layout vectors STD-609 held back are the rows
//! of `protocol-node-layout-rows.json`; each edits the STD-609 base.

use ix_trace_rs::trace;
use serde_json::Value;

const ROWS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-node-layout-rows.json");
const BASE: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-base.json");
const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");

fn parse(text: &str) -> Value {
    serde_json::from_str(text).expect("authored fixture is valid JSON")
}

/// Applies the RFC 6902 `add`, `remove` and `replace` operations a row uses.
fn apply(base: &Value, edit: &[Value]) -> Value {
    let mut package = base.clone();
    for op in edit {
        let path = op["path"].as_str().expect("path");
        let (parent, last) = path.rsplit_once('/').expect("absolute pointer");
        let target = package
            .pointer_mut(parent)
            .unwrap_or_else(|| panic!("{path}: parent exists"));
        match (op["op"].as_str().expect("op"), target) {
            ("remove", Value::Object(map)) => {
                map.remove(last)
                    .unwrap_or_else(|| panic!("{path}: member exists"));
            }
            ("remove", Value::Array(items)) => {
                items.remove(last.parse::<usize>().expect("index"));
            }
            ("replace", Value::Object(map)) => {
                assert!(map.contains_key(last), "{path}: replaced member exists");
                map.insert(last.to_owned(), op["value"].clone());
            }
            ("replace", Value::Array(items)) => {
                items[last.parse::<usize>().expect("index")] = op["value"].clone();
            }
            ("add", Value::Object(map)) => {
                map.insert(last.to_owned(), op["value"].clone());
            }
            ("add", Value::Array(items)) => {
                let at = if last == "-" {
                    items.len()
                } else {
                    last.parse().expect("index")
                };
                items.insert(at, op["value"].clone());
            }
            (other, _) => panic!("{path}: unsupported {other}"),
        }
    }
    package
}

/// FR-464-AC-1, FR-464-AC-5, FR-466-AC-1, FR-466-AC-3, FR-466-AC-5: the base is
/// schema-valid with the application rule wired, and each of the 26 layout
/// vectors is refused by the schema at the recorded first error path.
#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1", "FR-464-AC-5")]
#[test]
fn layout_vectors_are_refused_by_the_schema_at_their_first_error_path() {
    let rows = parse(ROWS);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema is valid");
    assert!(validator.is_valid(&base), "the base is schema-valid");
    let rows = rows["rows"].as_array().expect("rows");
    assert_eq!(rows.len(), 26);
    let mut defects = Vec::new();
    for row in rows {
        let id = row["id"].as_str().unwrap();
        assert_eq!(row["expect"]["code"], "malformed_wire", "{id}");
        assert_eq!(row["expect"]["stage"], "schema", "{id}");
        let edited = apply(&base, row["edit"].as_array().unwrap());
        let first = validator
            .iter_errors(&edited)
            .next()
            .map(|e| e.instance_path().to_string());
        if first.as_deref() != row["expect"]["locus"].as_str() {
            defects.push(format!("{id}: first error {first:?}, recorded {}", row["expect"]["locus"]));
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

/// FR-464-AC-1: the `fence` row is refused for its `ordering` alone: the same
/// appended node with the admitted spelling `acquire` is schema-valid.
#[trace("TC-946", "FR-464-AC-1")]
#[test]
fn the_appended_fence_is_refused_for_its_ordering_alone() {
    let rows = parse(ROWS);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema is valid");
    let row = rows["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "L12")
        .expect("row L12");
    let mut edit = row["edit"].as_array().unwrap().clone();
    assert!(!validator.is_valid(&apply(&base, &edit)));
    edit[0]["value"]["body"]["arguments"][0]["value"]["value"] = "acquire".into();
    assert!(validator.is_valid(&apply(&base, &edit)));
}
