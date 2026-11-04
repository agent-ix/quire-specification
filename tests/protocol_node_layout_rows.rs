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

/// FR-464-AC-1, FR-464-AC-5, FR-464-AC-24, FR-466-AC-1, FR-466-AC-3,
/// FR-466-AC-5: the base is schema-valid with the application rule wired, and
/// each of the 26 layout vectors is refused by the schema at the recorded first
/// error path. The rows cover an enumerated value outside its set (AC-1), a
/// `choice` with one case (AC-3) and a `repeat` with `exhausted` but no
/// `maximum` (AC-5).
#[trace(
    "TC-946",
    "TC-947",
    "TC-948",
    "FR-464-AC-1",
    "FR-464-AC-5",
    "FR-464-AC-24",
    "FR-466-AC-1",
    "FR-466-AC-3",
    "FR-466-AC-5"
)]
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

fn text(value: &str) -> Value {
    serde_json::json!({
        "term": "literal",
        "type": {"ordinal": 0, "package": "example", "path": ["scalar_type", "text"]},
        "value": value,
        "value_kind": "text"
    })
}

/// FR-464-AC-24, FR-466-AC-1: every spelling an enumeration admits stays
/// schema-valid, so tightening the rule past the enumerations fails here. Each
/// case edits the base at one path: a scalar `replace` or the members of a
/// list of spellings.
#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-24", "FR-466-AC-1")]
#[test]
fn every_admitted_enumerated_spelling_is_schema_valid() {
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema is valid");
    let node = "/semantic_graph/nodes";
    // (path, spellings, true when the path holds a list of spelling literals)
    let cases: [(String, &[&str], bool); 8] = [
        (format!("{node}/62/body/arguments/4/value/value"), &["unknown", "at-most-once", "at-least-once", "exactly-once-premise"], false),
        (format!("{node}/63/body/arguments/2/value/value"), &["reject", "block"], false),
        (format!("{node}/66/body/arguments/2/value/members"), &["relaxed", "acquire", "release", "acq_rel", "seq_cst"], true),
        (format!("{node}/70/body/arguments/0/value/members"), &["sc", "tso", "ra"], true),
        (format!("{node}/70/body/arguments/4/value/members"), &["continue", "cancel"], true),
        (format!("{node}/70/body/arguments/2/value/members/0/value/value"), &["all", "any"], false),
        (format!("{node}/80/body/arguments/0/value/value"), &["any"], false),
        (format!("{node}/82/body/arguments/10/value/members"), &["adversarial"], true),
    ];
    let mut defects = Vec::new();
    for (path, spellings, list) in &cases {
        for spelling in *spellings {
            let value = if *list {
                Value::Array(vec![text(spelling)])
            } else {
                Value::from(*spelling)
            };
            let edit = [serde_json::json!({"op": "replace", "path": path, "value": value})];
            if !validator.is_valid(&apply(&base, &edit)) {
                defects.push(format!("{path}: {spelling} is refused"));
            }
        }
    }
    // `overflow` `loss` pairs with one `loss` spelling; `when` with a condition.
    for spelling in ["head", "tail", "new"] {
        let edit = [
            serde_json::json!({"op": "replace", "path": format!("{node}/63/body/arguments/2/value/value"), "value": "loss"}),
            serde_json::json!({"op": "replace", "path": format!("{node}/63/body/arguments/3/value/members"), "value": [text(spelling)]}),
        ];
        if !validator.is_valid(&apply(&base, &edit)) {
            defects.push(format!("queue overflow loss with {spelling} is refused"));
        }
    }
    let edit = [
        serde_json::json!({"op": "replace", "path": format!("{node}/80/body/arguments/0/value/value"), "value": "when"}),
        serde_json::json!({"op": "replace", "path": format!("{node}/80/body/arguments/1/value/members"), "value": [text("condition")]}),
    ];
    if !validator.is_valid(&apply(&base, &edit)) {
        defects.push("terminal when with a condition is refused".to_owned());
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

/// FR-464-AC-24, FR-466-AC-1: a `fence` with each admitted ordering, and an
/// `event` and an `await` appended with their layouts, are schema-valid.
#[trace("TC-946", "TC-948", "FR-464-AC-24", "FR-466-AC-1")]
#[test]
fn appended_fence_event_and_await_nodes_are_schema_valid() {
    let rows = parse(ROWS);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema is valid");
    let row = rows["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "L12")
        .expect("row L12");
    let appended = row["edit"][0]["value"].clone();
    let effect = &base["semantic_graph"]["nodes"][67]["body"]["arguments"];
    let term = |index: usize| effect[index]["value"].clone();
    let binding = |name: &str, value: Value| {
        serde_json::json!({"term": "binding", "name": name, "value": value})
    };
    let mut nodes = Vec::new();
    for ordering in ["acquire", "release", "acq_rel", "seq_cst"] {
        let mut node = appended.clone();
        node["body"]["arguments"][0]["value"]["value"] = ordering.into();
        nodes.push((format!("fence {ordering}"), node));
    }
    let mut event = appended.clone();
    event["body"]["operation"]["member"]["operator"] = "event".into();
    event["body"]["arguments"] = Value::Array(vec![
        binding("role", term(0)),
        binding("compensation", term(0)),
        binding("binder", term(0)),
        binding("related", term(1)),
        binding("constraint", term(2)),
    ]);
    nodes.push(("event".to_owned(), event));
    let mut wait = appended.clone();
    wait["body"]["operation"]["member"]["operator"] = "await".into();
    wait["body"]["arguments"] = Value::Array(
        ["after", "clock", "within", "event", "then", "timeout"]
            .iter()
            .map(|name| binding(name, term(0)))
            .collect(),
    );
    nodes.push(("await".to_owned(), wait));
    for (label, node) in nodes {
        let edit = [serde_json::json!({"op": "add", "path": "/semantic_graph/nodes/-", "value": node})];
        assert!(validator.is_valid(&apply(&base, &edit)), "{label} is schema-valid");
    }
}
