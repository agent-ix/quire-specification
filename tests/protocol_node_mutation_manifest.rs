// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! STD-609: the executable mutation manifest of the protocol node vectors
//! (TC-946, TC-947, TC-948). The manifest must follow its schema, name every
//! vector exactly once as a row, a later row or a pending question, edit a
//! base that exists and is schema-valid, carry loci that its own edits reach,
//! keep every edited node's `dependencies` equal to the references of its body
//! (so a graph-stage refusal never masks the defect a row is about), and state
//! property assertions that the base itself satisfies.

use ix_trace_rs::trace;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[path = "support/operand_slots.rs"]
mod operand_slots;

const MANIFEST: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-node-mutation-manifest.json");

const MANIFEST_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/protocol-node-mutation-manifest.schema.json");
const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-node-vectors.json");
const BASE: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-base.json");
const PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
fn parse(text: &str) -> Value {
    operand_slots::index_view(&serde_json::from_str(text).expect("authored fixture is valid JSON"))
}

fn unescape(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

/// Applies the RFC 6902 `add`, `remove` and `replace` operations a row uses.
/// `base` (an index view) with a row's `edit` applied in the published slot
/// form, read back as an index view (FR-322 "Operand index segments").
fn apply(base: &Value, edit: &[Value]) -> Value {
    operand_slots::index_view(&apply_published(&operand_slots::slot_view(base), edit))
}

fn apply_published(base: &Value, edit: &[Value]) -> Value {
    let mut package = base.clone();
    for op in edit {
        let path = op["path"].as_str().expect("path");
        let (parent, last) = path.rsplit_once('/').expect("absolute pointer");
        let last = unescape(last);
        let target = package
            .pointer_mut(parent)
            .unwrap_or_else(|| panic!("{path}: parent exists"));
        match (op["op"].as_str().expect("op"), target) {
            ("remove", Value::Object(map)) => {
                map.remove(&last)
                    .unwrap_or_else(|| panic!("{path}: member exists"));
            }
            ("remove", Value::Array(items)) => {
                items.remove(last.parse::<usize>().expect("index"));
            }
            ("replace", Value::Object(map)) => {
                assert!(map.contains_key(&last), "{path}: replaced member exists");
                map.insert(last, op["value"].clone());
            }
            ("add", Value::Object(map)) => {
                map.insert(last, op["value"].clone());
            }
            ("replace", Value::Array(items)) => {
                items[last.parse::<usize>().expect("index")] = op["value"].clone();
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

fn nodes_of(package: &Value) -> &Vec<Value> {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
}

fn declaration_name(node: &Value) -> String {
    match node.get("declaration") {
        Some(declaration) => declaration["qualified_name"]
            .as_array()
            .expect("qualified name")
            .iter()
            .map(|s| s.as_str().expect("segment"))
            .collect::<Vec<_>>()
            .join("::"),
        None => serde_json::to_string(&node["node_id"]["path"]).expect("path"),
    }
}

fn node_named<'a>(package: &'a Value, name: &str) -> &'a Value {
    nodes_of(package)
        .iter()
        .find(|n| declaration_name(n) == name)
        .unwrap_or_else(|| panic!("no node named {name}"))
}

fn ids_of(list: &Value) -> Vec<(String, String)> {
    list.as_array()
        .expect("array")
        .iter()
        .filter(|r| r.get("sub_case_of").is_none())
        .map(|r| {
            (
                r["test_case"].as_str().unwrap().to_owned(),
                r["id"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Segment {
    Node(u64),
    Text(String),
}

/// FR-322 structural order of a node id: package, then path segments, then ordinal.
fn id_key(id: &Value) -> (String, Vec<Segment>, u64) {
    let path = id["path"]
        .as_array()
        .expect("path")
        .iter()
        .map(|s| match s.get("node") {
            Some(index) => Segment::Node(index.as_u64().expect("index")),
            None => Segment::Text(s.as_str().expect("segment").to_owned()),
        })
        .collect();
    (
        id["package"].as_str().expect("package").to_owned(),
        path,
        id["ordinal"].as_u64().expect("ordinal"),
    )
}

fn collect_targets(value: &Value, out: &mut Vec<Value>) {
    match value {
        Value::Object(map) => {
            if map.get("term").and_then(Value::as_str) == Some("reference") {
                if let Some(target) = map.get("target") {
                    out.push(target.clone());
                }
            }
            for (key, child) in map {
                if key != "target" {
                    collect_targets(child, out);
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|item| collect_targets(item, out)),
        _ => {}
    }
}

/// The unique references of a protocol application body and of its `owner_model` edge,
/// in ascending node-id order: what the node's `dependencies` must list (FR-322-AC-96).
fn expected_dependencies(node: &Value) -> Vec<Value> {
    let mut targets = Vec::new();
    collect_targets(&node["body"], &mut targets);
    if let Some(owner) = node.get("owner_model") {
        targets.push(owner.clone());
    }
    let unique: BTreeMap<String, Value> = targets.into_iter().map(|t| (t.to_string(), t)).collect();
    let mut targets: Vec<Value> = unique.into_values().collect();
    targets.sort_by_key(id_key);
    targets
}

fn dependency_defects(package: &Value) -> Vec<String> {
    nodes_of(package)
        .iter()
        .enumerate()
        .filter(|(_, n)| n["node_tag"] == "protocol" && n["body"]["term"] == "application")
        .filter(|(_, n)| {
            expected_dependencies(n) != n["dependencies"].as_array().expect("dependencies").clone()
        })
        .map(|(i, n)| format!("node {i} {}", declaration_name(n)))
        .collect()
}

/// A graph-stage dependency defect row asserts the mismatch itself; a schema row never reaches the graph.
fn dependencies_are_the_point(expect: &Value) -> bool {
    expect["stage"] == "schema"
        || (expect["stage"] == "graph"
            && expect["locus"]
                .as_str()
                .is_some_and(|l| l.ends_with("/dependencies")))
}

fn rows(manifest: &Value) -> &Vec<Value> {
    manifest["rows"].as_array().expect("rows")
}

fn edit_of(row: &Value) -> Vec<Value> {
    row.get("edit")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn the_manifest_follows_its_schema() {
    let schema =
        jsonschema::validator_for(&parse(MANIFEST_SCHEMA)).expect("manifest schema is valid");
    let manifest = parse(MANIFEST);
    let errors: Vec<String> = schema
        .iter_errors(&manifest)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .take(5)
        .collect();
    assert!(errors.is_empty(), "{errors:?}");
    assert!(
        MANIFEST.ends_with("}\n"),
        "the manifest ends with a newline"
    );
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_vector_is_a_row_a_later_row_or_a_pending_question_exactly_once() {
    let manifest = parse(MANIFEST);
    let vectors = parse(VECTORS);
    let expected: Vec<(String, String)> = ids_of(&vectors["cases"]);
    let mut seen = BTreeSet::new();
    for list in ["rows", "later", "pending"] {
        for entry in ids_of(&manifest[list]) {
            assert!(
                seen.insert(entry.clone()),
                "{entry:?} appears in more than one place"
            );
        }
    }
    let expected: BTreeSet<_> = expected.into_iter().collect();
    assert_eq!(
        seen, expected,
        "manifest partitions the vectors, with the same test case per id"
    );
    let known: BTreeSet<&str> = seen.iter().map(|(_, id)| id.as_str()).collect();
    for entry in manifest["later"].as_array().unwrap() {
        if let Some(parent) = entry.get("sub_case_of").and_then(Value::as_str) {
            assert!(
                known.contains(parent),
                "{}: sub_case_of {parent} is a vector",
                entry["id"]
            );
            assert!(
                !known.contains(entry["id"].as_str().unwrap()),
                "{}: a sub-case is not a vector id",
                entry["id"]
            );
        }
    }
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn a_later_row_class_matches_what_its_vector_expects() {
    let manifest = parse(MANIFEST);
    let vectors = parse(VECTORS);
    let prose: BTreeMap<String, String> = vectors["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                c["id"].as_str().unwrap().to_owned(),
                c["Expected"].as_str().unwrap().to_lowercase(),
            )
        })
        .collect();
    let refusal_words = |e: &str| {
        e.contains("refus")
            || e.contains("reported")
            || e.split('`')
                .skip(1)
                .step_by(2)
                .any(|token| token.contains('_') || token.contains('/'))
    };
    let mut defects = Vec::new();
    for entry in manifest["later"].as_array().unwrap() {
        let Some(expected) = prose.get(entry["id"].as_str().unwrap()) else {
            continue;
        };
        let class = entry["class"].as_str().unwrap();
        let admitted = expected.starts_with("admit");
        if class == "refuse" && admitted && entry["reason"] != "multi_outcome_split" {
            defects.push(format!(
                "{}: refuse, but the vector is admitted",
                entry["id"]
            ));
        }
        if class == "refuse" && !admitted && !refusal_words(expected) {
            defects.push(format!(
                "{}: refuse, but the vector names no refusal",
                entry["id"]
            ));
        }
        if class == "admit" && !expected.contains("admit") {
            defects.push(format!(
                "{}: admit, but the vector is not admitted",
                entry["id"]
            ));
        }
    }
    assert!(defects.is_empty(), "{defects:?}");
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_row_edits_a_base_that_exists_and_is_schema_valid() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    assert_eq!(manifest["base"]["fixture"], "protocol-node-base.json");
    let validator = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("I04 schema is valid");
    assert!(
        validator.is_valid(&base),
        "the base package is schema-valid"
    );
    for row in rows(&manifest) {
        assert_eq!(
            row["base"], manifest["base"]["fixture"],
            "{}: base ref",
            row["id"]
        );
    }
    assert!(
        dependency_defects(&base).is_empty(),
        "the base lists exactly the references of every protocol body"
    );
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn every_edit_reaches_the_node_it_names_and_its_locus_resolves() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    let shipped = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("I04 schema is valid");
    let nodes = nodes_of(&base);
    let mut defects = Vec::new();
    for row in rows(&manifest) {
        let id = row["id"].as_str().unwrap();
        let edit = edit_of(row);
        for op in &edit {
            let path = op["path"].as_str().unwrap();
            if let Some(rest) = path.strip_prefix("/semantic_graph/nodes/") {
                let index: usize = rest.split('/').next().unwrap().parse().expect("node index");
                let named = op["node"].as_str().unwrap_or_default();
                if named != declaration_name(&nodes[index]) {
                    defects.push(format!(
                        "{id}: {path} does not enter the node it names ({named})"
                    ));
                }
            }
        }
        let edited = apply(&base, &edit);
        let expect = &row["expect"];
        let first_error = shipped
            .iter_errors(&edited)
            .next()
            .map(|e| e.instance_path().to_string());
        match expect["decision"].as_str().unwrap() {
            "refuse" if expect["stage"] == "schema" => {
                if first_error.as_deref() != expect["locus"].as_str() {
                    defects.push(format!(
                        "{id}: wire locus {:?} is not the first schema error path {first_error:?}",
                        expect["locus"]
                    ));
                }
            }
            _ => {
                if first_error.is_some() {
                    defects.push(format!(
                        "{id}: the edit does not pass the schema ({first_error:?})"
                    ));
                }
                if !dependencies_are_the_point(expect) {
                    let stale = dependency_defects(&edited);
                    if !stale.is_empty() {
                        defects.push(format!(
                            "{id}: dependencies differ from the body references at {stale:?}"
                        ));
                    }
                }
            }
        }
        if expect["decision"] == "refuse" && expect["stage"] != "schema" {
            let locus = expect["locus"].as_str().unwrap();
            if edited.pointer(locus).is_none() {
                defects.push(format!(
                    "{id}: locus {locus} does not resolve in the edited package"
                ));
            }
        }
        if let Some(pointer) = expect.get("document_pointer").and_then(Value::as_str) {
            let locus = expect["locus"].as_str().unwrap();
            if !pointer.starts_with(locus) || edited.pointer(pointer).is_none() {
                defects.push(format!(
                    "{id}: document_pointer {pointer} is not a field under {locus}"
                ));
            }
        }
        if let Some(step) = expect.get("step_alone") {
            let locus = step["locus"].as_str().unwrap();
            if edited.pointer(locus).is_none() {
                defects.push(format!("{id}: step_alone locus {locus} does not resolve"));
            }
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn locus_basis_records_how_each_locus_was_set() {
    let manifest = parse(MANIFEST);
    let mut defects = Vec::new();
    for row in rows(&manifest) {
        let id = row["id"].as_str().unwrap();
        let basis = row["locus_basis"].as_str().unwrap();
        let expect = &row["expect"];
        if expect["decision"] != "refuse" {
            if basis != "not_applicable" {
                defects.push(format!("{id}: {basis} on a row that asserts no refusal"));
            }
            continue;
        }
        let locus = expect["locus"].as_str().unwrap();
        let is_node = locus
            .strip_prefix("/semantic_graph/nodes/")
            .is_some_and(|rest| rest.parse::<usize>().is_ok());
        let fits = match basis {
            "node" => is_node,
            "dependencies_list" => locus.ends_with("/dependencies"),
            "target" => locus.ends_with("/target"),
            "schema_error_path" => expect["stage"] == "schema",
            "field" => !is_node && !locus.ends_with("/dependencies") && !locus.ends_with("/target"),
            _ => false,
        };
        if !fits || (basis == "schema_error_path") != (expect["stage"] == "schema") {
            defects.push(format!(
                "{id}: locus_basis {basis} does not describe locus {locus}"
            ));
        }
    }
    assert!(defects.is_empty(), "{defects:?}");
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn refusal_rows_carry_a_code_and_admission_rows_nothing_more() {
    let manifest = parse(MANIFEST);
    let mut counts = (0, 0, 0);
    for row in rows(&manifest) {
        match row["class"].as_str().unwrap() {
            "refuse" => {
                assert!(row["expect"]["code"].is_string(), "{}", row["id"]);
                counts.0 += 1;
            }
            "admit" => {
                assert_eq!(row["expect"], serde_json::json!({"decision": "admit"}));
                counts.1 += 1;
            }
            "property" => counts.2 += 1,
            other => panic!("{}: class {other} has no rows in PR 1", row["id"]),
        }
    }
    assert!(
        counts.0 > 0 && counts.1 > 0 && counts.2 > 0,
        "every outcome is executed"
    );
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn a_retaining_refusal_retains_what_the_edited_target_is() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    for row in rows(&manifest) {
        let Some(retains) = row["expect"].get("retains") else {
            continue;
        };
        let edited = apply(&base, &edit_of(row));
        let target = edited
            .pointer(row["expect"]["locus"].as_str().unwrap())
            .expect("locus resolves");
        let node = nodes_of(&edited)
            .iter()
            .find(|n| n["node_id"] == *target)
            .unwrap_or_else(|| panic!("{}: the target is a node of the package", row["id"]));
        if let Some(kind) = retains.get("actual_kind").and_then(Value::as_str) {
            assert_eq!(node["semantic_form"], kind, "{}: actual kind", row["id"]);
        }
        if let Some(key) = retains.get("actual_key").and_then(Value::as_str) {
            assert_eq!(declaration_name(node), key, "{}: actual key", row["id"]);
        }
    }
}

fn parameter_level(package: &Value, binder: &str) -> Option<u64> {
    nodes_of(package).iter().find_map(|n| {
        let path = n["node_id"]["path"].as_array()?;
        let named =
            path.first()? == "value" && path.get(1)? == "parameter" && path.get(4)? == binder;
        if named {
            path.get(7)?.as_str()?.parse().ok()
        } else {
            None
        }
    })
}

fn operator_of(node: &Value) -> &str {
    node["body"]["operation"]["member"]["operator"]
        .as_str()
        .unwrap_or_default()
}

fn argument<'a>(node: &'a Value, name: &str) -> &'a Value {
    node["body"]["arguments"]
        .as_array()
        .and_then(|args| args.iter().find(|a| a["name"] == name))
        .map(|a| &a["value"])
        .unwrap_or_else(|| panic!("{}: no argument {name}", declaration_name(node)))
}

fn has_cycle(edges: &[(String, String)]) -> bool {
    fn visit(
        at: &str,
        edges: &[(String, String)],
        active: &mut Vec<String>,
        done: &mut BTreeSet<String>,
    ) -> bool {
        if active.iter().any(|a| a == at) {
            return true;
        }
        if !done.insert(at.to_owned()) {
            return false;
        }
        active.push(at.to_owned());
        let cyclic = edges
            .iter()
            .filter(|(from, _)| from == at)
            .any(|(_, to)| visit(to, edges, active, done));
        active.pop();
        cyclic
    }
    let mut done = BTreeSet::new();
    edges
        .iter()
        .any(|(from, _)| visit(from, edges, &mut Vec::new(), &mut done))
}

/// Evaluates one property assertion against the base and the edited package; returns a defect if it fails.
fn check_assertion(assertion: &Value, base: &Value, edited: &Value) -> Option<String> {
    let kind = assertion["kind"].as_str().unwrap();
    let ids = |names: &Value| -> Vec<String> {
        names
            .as_array()
            .map(|a| a.iter().map(|n| n.as_str().unwrap().to_owned()).collect())
            .unwrap_or_default()
    };
    match kind {
        "admitted" => None,
        "package_byte_equal" => (serde_json::to_string(base).ok()
            != serde_json::to_string(edited).ok())
        .then(|| "the edited package is not byte-equal to the base".to_owned()),
        "binder_levels" => assertion["levels"]
            .as_array()
            .unwrap()
            .iter()
            .find_map(|l| {
                let binder = l["binder"].as_str().unwrap();
                (parameter_level(edited, binder) != l["level"].as_u64()).then(|| {
                    format!(
                        "binder {binder} level is {:?}",
                        parameter_level(edited, binder)
                    )
                })
            }),
        "runtime_subject_kinds" => {
            assertion["operators"]
                .as_array()
                .unwrap()
                .iter()
                .find_map(|o| {
                    let operator = o["operator"].as_str().unwrap();
                    (!nodes_of(edited).iter().any(|n| operator_of(n) == operator))
                        .then(|| format!("no {operator} node in the package"))
                })
        }
        "causal_edges" => {
            let mut graph = Vec::new();
            for edge in assertion["edges"].as_array().unwrap() {
                let from = node_named(edited, edge["from"]["node"].as_str().unwrap());
                let to = node_named(edited, edge["to"]["node"].as_str().unwrap());
                let ok = match edge["relation"].as_str().unwrap() {
                    "message" => {
                        operator_of(from) == "send"
                            && operator_of(to) == "receive"
                            && argument(from, "channel") == argument(to, "channel")
                            && argument(to, "of")["target"] == from["node_id"]
                    }
                    "effect" => {
                        operator_of(to) == "effect"
                            && argument(to, "of")["target"] == from["node_id"]
                    }
                    "join" => {
                        operator_of(to) == "parallel"
                            && argument(to, "branches")
                                .to_string()
                                .contains(&from["node_id"].to_string())
                    }
                    _ => true,
                };
                if !ok {
                    return Some(format!(
                        "{} edge {} to {} is not in the package",
                        edge["relation"], edge["from"]["node"], edge["to"]["node"]
                    ));
                }
                graph.push((
                    edge["from"]["node"].as_str().unwrap().to_owned(),
                    edge["to"]["node"].as_str().unwrap().to_owned(),
                ));
            }
            (assertion["acyclic"] == true && has_cycle(&graph))
                .then(|| "the causal edges are cyclic".to_owned())
        }
        "activation_relation" => assertion["rows"].as_array().unwrap().iter().find_map(|r| {
            let clause = node_named(edited, r["clause"].as_str().unwrap());
            let activation = argument(clause, "activation")["members"]
                .as_array()
                .unwrap();
            let named = |n: &str| {
                activation
                    .iter()
                    .find(|m| m["name"] == n)
                    .map(|m| m["value"].clone())
            };
            if named("kind").map(|k| k["value"].clone()) != Some(r["activation"].clone()) {
                return Some("activation kind differs".to_owned());
            }
            let clause_node = node_named(edited, r["temporal_clause"].as_str().unwrap());
            let required = argument(clause, "requirements")
                .to_string()
                .contains(&clause_node["node_id"].to_string());
            let trigger_ok = match r.get("trigger").and_then(Value::as_str) {
                Some(t) => named("trigger")
                    .is_some_and(|v| v["target"] == node_named(edited, t)["node_id"]),
                None => named("trigger").is_none(),
            };
            (!required || !trigger_ok).then(|| "trigger or requirement differs".to_owned())
        }),
        "node_ids_unchanged" => {
            let names: Vec<String> = if assertion["scope"] == "all" {
                nodes_of(base).iter().map(declaration_name).collect()
            } else {
                ids(&assertion["nodes"])
            };
            names
                .iter()
                .find(|n| node_named(base, n)["node_id"] != node_named(edited, n)["node_id"])
                .map(|n| format!("{n}: node id changed"))
        }
        "node_id_changed" => {
            let n = assertion["node"].as_str().unwrap();
            let before = nodes_of(base)
                .iter()
                .find(|x| declaration_name(x) == n)
                .map(|x| &x["node_id"]);
            let after = nodes_of(edited)
                .iter()
                .find(|x| declaration_name(x) == n)
                .map(|x| &x["node_id"]);
            (before.is_some() && before == after).then(|| format!("{n}: node id unchanged"))
        }
        "body_changed" | "body_unchanged" => {
            let n = assertion["node"].as_str().unwrap();
            let same = node_named(base, n)["body"] == node_named(edited, n)["body"];
            (same != (kind == "body_unchanged")).then(|| format!("{n}: {kind} fails"))
        }
        "distinct_node_ids" => {
            let found: BTreeSet<String> = ids(&assertion["nodes"])
                .iter()
                .map(|n| node_named(edited, n)["node_id"].to_string())
                .collect();
            (found.len() as u64 != assertion["count"].as_u64().unwrap())
                .then(|| format!("{} distinct node ids", found.len()))
        }
        "member_equals" => {
            let node = node_named(edited, assertion["node"].as_str().unwrap());
            (node.pointer(assertion["member"].as_str().unwrap()) != Some(&assertion["value"]))
                .then(|| "member differs".to_owned())
        }
        other => Some(format!("unknown assertion kind {other}")),
    }
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-1")]
#[test]
fn property_rows_hold_on_the_base_they_describe() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    let mut defects = Vec::new();
    for row in rows(&manifest).iter().filter(|r| r["class"] == "property") {
        let edited = apply(&base, &edit_of(row));
        for assertion in row["expect"]["assertions"].as_array().unwrap() {
            if let Some(defect) = check_assertion(assertion, &base, &edited) {
                defects.push(format!("{}: {defect}", row["id"]));
            }
        }
    }
    assert!(defects.is_empty(), "{defects:?}");
}

/// FR-464-AC-2, FR-466-AC-1: V3, V4, V7 and L19 are refused by the schema
/// first, as `malformed_wire` at the first schema error path, which is the
/// edited node's pointer; the reader cause the prose names is what the step
/// returns alone on the same edit (`step_alone`). M1 (FR-464 "Single listing")
/// refuses at the listed member node, `Orders`, not at the clause.
#[trace("TC-946", "TC-948", "FR-464-AC-2", "FR-464-AC-6", "FR-466-AC-1")]
#[test]
fn schema_first_placement_rows_and_the_single_listing_row_pin_their_loci() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("schema is valid");
    let mut defects = Vec::new();
    for (id, node, step_code) in [
        ("V3", "Example::OrderFlow::role::Orders", "operator-ineligible"),
        ("V4", "Example::OrderFlow::role::Orders", "operator-ineligible"),
        ("V7", "Example::OrderFlow::run::Main::Requested", "operation-member-mismatch"),
        ("L19", "Example::OrderFlow::run::Main::Requested", "operation-member-mismatch"),
    ] {
        let row = rows(&manifest).iter().find(|r| r["id"] == id).expect("row");
        let expect = &row["expect"];
        let edited = apply(&base, &edit_of(row));
        let first = validator
            .iter_errors(&edited)
            .next()
            .map(|e| e.instance_path().to_string());
        let index = nodes_of(&base)
            .iter()
            .position(|n| declaration_name(n) == node)
            .expect("node");
        let pointer = format!("/semantic_graph/nodes/{index}");
        if expect["stage"] != "schema"
            || expect["code"] != "malformed_wire"
            || expect["locus"] != pointer.as_str()
            || first.as_deref() != Some(pointer.as_str())
            || expect["step_alone"]["cause"] != step_code
        {
            defects.push(format!("{id}: schema first error {first:?}, recorded {expect}"));
        }
    }
    let m1 = rows(&manifest).iter().find(|r| r["id"] == "M1").expect("M1");
    let orders = nodes_of(&base)
        .iter()
        .position(|n| declaration_name(n) == "Example::OrderFlow::role::Orders")
        .expect("Orders");
    if m1["expect"]["locus"] != format!("/semantic_graph/nodes/{orders}").as_str() {
        defects.push(format!("M1: locus {}", m1["expect"]["locus"]));
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

/// FR-464-AC-2, FR-464-AC-6: V5 (a nested protocol application) is refused by
/// the schema at the holding node's pointer; C12 refuses at `Main`, the control
/// nothing holds; V16, CP28 and C11 pass the schema, so their refusals are the
/// reader's, not the schema's.
#[trace("TC-946", "TC-948", "FR-464-AC-2", "FR-464-AC-4", "FR-464-AC-6")]
#[test]
fn nested_application_and_unheld_root_rows_pin_their_stage_and_locus() {
    let manifest = parse(MANIFEST);
    let base = parse(BASE);
    let validator = jsonschema::validator_for(&parse(PACKAGE_SCHEMA)).expect("schema is valid");
    let row = |id: &str| rows(&manifest).iter().find(|r| r["id"] == id).expect("row");
    let index_of = |name: &str| {
        nodes_of(&base)
            .iter()
            .position(|n| declaration_name(n) == name)
            .expect("node")
    };
    let mut defects = Vec::new();
    let v5 = row("V5");
    let first = validator
        .iter_errors(&apply(&base, &edit_of(v5)))
        .next()
        .map(|e| e.instance_path().to_string());
    let main = format!("/semantic_graph/nodes/{}", index_of("Example::OrderFlow::run::Main"));
    if v5["expect"]["stage"] != "schema"
        || v5["expect"]["locus"] != main.as_str()
        || first.as_deref() != Some(main.as_str())
    {
        defects.push(format!("V5: schema first error {first:?}, recorded {}", v5["expect"]));
    }
    let c12 = row("C12");
    if c12["expect"]["locus"] != main.as_str() || c12["expect"]["code"] != "invalid_semantic_graph" {
        defects.push(format!("C12: recorded {}", c12["expect"]));
    }
    if !validator.is_valid(&apply(&base, &edit_of(c12))) {
        defects.push("C12: the edit is refused by the schema, so the row is not a reader row".into());
    }
    for id in ["V16", "CP28", "C11"] {
        let r = row(id);
        if r["expect"]["stage"] != "reader" || !validator.is_valid(&apply(&base, &edit_of(r))) {
            defects.push(format!("{id}: not a schema-valid reader-stage row"));
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

const TC946: &str =
    include_str!("../spec/test-cases/TC-946-checked-package-protocol-node-family.md");
const TC948: &str =
    include_str!("../spec/test-cases/TC-948-checked-package-protocol-control-bodies.md");

/// The vectors file and the TC-946/TC-948 tables state the same code, cause and
/// locus kind as the manifest row for the rows IR-1322 amended, so prose and
/// vector drift fails. Each cell is compared by the row's code, its cause (the
/// reader cause, or the `step_alone` cause of a schema-stage row) and a locus
/// phrase that names the manifest locus kind.
#[trace("TC-946", "TC-948", "FR-464-AC-2", "FR-464-AC-4", "FR-464-AC-6")]
#[test]
fn amended_vector_and_test_case_cells_agree_with_the_manifest_rows() {
    let manifest = parse(MANIFEST);
    let vectors = parse(VECTORS);
    let mut cells: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    fn collect(value: &Value, out: &mut BTreeMap<String, Vec<(String, String)>>) {
        match value {
            Value::Object(map) => {
                if let (Some(id), Some(expected)) = (
                    map.get("id").and_then(Value::as_str),
                    map.get("Expected").and_then(Value::as_str),
                ) {
                    out.entry(id.to_owned())
                        .or_default()
                        .push(("vectors".to_owned(), expected.to_owned()));
                }
                map.values().for_each(|v| collect(v, out));
            }
            Value::Array(items) => items.iter().for_each(|v| collect(v, out)),
            _ => {}
        }
    }
    collect(&vectors, &mut cells);
    for (name, text) in [("TC-946", TC946), ("TC-948", TC948)] {
        for line in text.lines() {
            let columns: Vec<&str> = line.split('|').map(str::trim).collect();
            if columns.len() > 4 && columns[0].is_empty() {
                cells
                    .entry(columns[1].to_owned())
                    .or_default()
                    .push((name.to_owned(), columns[3].to_owned()));
            }
        }
    }
    let phrases = [
        ("M1", "listed member node"),
        ("V3", "at the node"),
        ("V4", "at the node"),
        ("V5", "node pointer"),
        ("V7", "at the node"),
        ("C12", "root control `Main`"),
        ("L19", "at the node"),
    ];
    let mut defects = Vec::new();
    for (id, phrase) in phrases {
        let row = rows(&manifest).iter().find(|r| r["id"] == id).expect("row");
        let expect = &row["expect"];
        let code = expect["code"].as_str().unwrap();
        let cause = expect["step_alone"]["cause"]
            .as_str()
            .or_else(|| expect["cause"].as_str());
        let found = cells.get(id).expect("a vector cell");
        assert!(found.iter().any(|(source, _)| source == "vectors"), "{id}: vectors cell");
        for (source, cell) in found {
            let mut wants = vec![format!("`{code}`"), phrase.to_owned()];
            if let Some(cause) = cause {
                wants.push(cause.to_owned());
            }
            for want in wants {
                if !cell.contains(&want) && !(source != "vectors" && want == format!("`{code}`") && cell.contains(code)) {
                    defects.push(format!("{id} ({source}): cell {cell:?} lacks {want:?}"));
                }
            }
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}
