//! FR-322 "Operand index segments": a node-held id names each operand through
//! an operand slot `{"operand": k}` into its holding node's operand row, and a
//! document-held id through an index segment `{"node": i}`.
//!
//! The qualification oracles reason over node ids in one form, with every
//! operand an index segment, so they read the published vectors through
//! [`index_view`]: each operand slot of a node becomes the index segment of the
//! node its row names, and the rows stay in place (the V2 schema requires
//! them). [`slot_view`] is the inverse a producer writes ("Producing"): it
//! numbers each node's slots by first occurrence and rebuilds its row, so
//! `slot_view(&index_view(published)) == published` for every published graph.
//! A vector patch is written against the published slot form, so an oracle
//! applies it to `slot_view` of its base and reads the result back through
//! `index_view` ([`patched`]).

#![allow(dead_code)]

use serde_json::{Map, Value, json};

fn is_node_id(value: &Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == 3 && object.contains_key("package") && object.contains_key("path") && object.contains_key("ordinal")
    })
}

/// The `{package, node}` of a `PackageNodeRef` or an FR-375 external key: its
/// `node` names another package's graph and keeps its index segments.
fn is_foreign_key(value: &Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == 2 && object.contains_key("package") && object.get("node").is_some_and(is_node_id)
    })
}

fn single(value: &Value, member: &str) -> Option<u64> {
    value.as_object().filter(|object| object.len() == 1).and_then(|object| object.get(member)).and_then(Value::as_u64)
}

/// The slot `k` of an operand slot `{"operand": k}`.
pub fn operand_slot(value: &Value) -> Option<u64> {
    single(value, "operand")
}

/// The graph index `i` of an index segment `{"node": i}`.
pub fn index_segment(value: &Value) -> Option<u64> {
    single(value, "node")
}

/// `value` with every segment `replace` maps rewritten, skipping foreign keys.
fn rewrite(value: &Value, replace: &mut impl FnMut(&Value) -> Option<Value>) -> Value {
    if is_foreign_key(value) {
        return value.clone();
    }
    if let Some(replaced) = replace(value) {
        return replaced;
    }
    match value {
        Value::Object(members) => {
            Value::Object(members.iter().map(|(key, member)| (key.clone(), rewrite(member, replace))).collect())
        }
        Value::Array(items) => Value::Array(items.iter().map(|item| rewrite(item, replace)).collect()),
        other => other.clone(),
    }
}

/// The graph indexes of `node`'s index segments in RFC 8785 order (member
/// names ascending, which serde_json's sorted map already gives for the
/// published ASCII names), each once, in first-occurrence order.
fn first_occurrences(value: &Value, row: &mut Vec<u64>) {
    if is_foreign_key(value) {
        return;
    }
    if let Some(index) = index_segment(value) {
        if !row.contains(&index) {
            row.push(index);
        }
        return;
    }
    match value {
        Value::Object(members) => {
            let mut keys: Vec<&String> = members.keys().collect();
            keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
            for key in keys {
                first_occurrences(&members[key.as_str()], row);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| first_occurrences(item, row)),
        _ => {}
    }
}

/// One node in index form: each operand slot becomes the index segment of the
/// row entry it names; a slot outside its row is left as written.
pub fn node_index_view(node: &Value, row: &Value) -> Value {
    let row = row.as_array().cloned().unwrap_or_default();
    rewrite(node, &mut |segment| {
        let slot = usize::try_from(operand_slot(segment)?).ok()?;
        row.get(slot).map(|index| json!({"node": index}))
    })
}

/// One node in slot form, with its operand row numbered by first occurrence.
pub fn node_slot_view(node: &Value) -> (Value, Value) {
    let mut row = Vec::new();
    first_occurrences(node, &mut row);
    let slotted = rewrite(node, &mut |segment| {
        let index = index_segment(segment)?;
        row.iter().position(|entry| *entry == index).map(|slot| json!({"operand": slot}))
    });
    (slotted, json!(row))
}

/// The row-list member that sits beside a node-list member: `nodes` has
/// `operand_rows`, `<prefix>_nodes` has `<prefix>_operand_rows`.
fn rows_member(nodes_member: &str) -> Option<String> {
    if nodes_member == "nodes" {
        Some("operand_rows".to_owned())
    } else {
        nodes_member.strip_suffix("_nodes").map(|prefix| format!("{prefix}_operand_rows"))
    }
}

fn view(value: &Value, to_index: bool) -> Value {
    match value {
        Value::Object(members) => {
            let mut out: Map<String, Value> = Map::new();
            for (key, member) in members {
                out.insert(key.clone(), view(member, to_index));
            }
            for (key, member) in members {
                let Some(nodes) = member.as_array() else { continue };
                let Some(rows_key) = rows_member(key) else { continue };
                let is_graph = members.contains_key("graph_version");
                if !members.contains_key(&rows_key) && !(is_graph && !to_index) {
                    continue;
                }
                if to_index {
                    let rows = members[&rows_key].as_array().cloned().unwrap_or_default();
                    let converted: Vec<Value> = nodes
                        .iter()
                        .enumerate()
                        .map(|(at, node)| node_index_view(node, rows.get(at).unwrap_or(&Value::Null)))
                        .collect();
                    out.insert(key.clone(), Value::Array(converted));
                } else {
                    let (converted, rows): (Vec<Value>, Vec<Value>) = nodes.iter().map(node_slot_view).unzip();
                    out.insert(key.clone(), Value::Array(converted));
                    out.insert(rows_key, Value::Array(rows));
                }
            }
            for (key, member) in members {
                let row_key = format!("{key}_operand_row");
                if let Some(row) = members.get(&row_key) {
                    if to_index {
                        out.insert(key.clone(), node_index_view(member, row));
                    } else {
                        let (converted, row) = node_slot_view(member);
                        out.insert(key.clone(), converted);
                        out.insert(row_key, row);
                    }
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(|item| view(item, to_index)).collect()),
        other => other.clone(),
    }
}

/// Every node list and node of `value` that carries operand rows, in index
/// form; the rows are kept.
pub fn index_view(value: &Value) -> Value {
    view(value, true)
}

/// Every node list and node of `value` that carries operand rows (and every
/// `semantic_graph`), in slot form with rows numbered by first occurrence.
pub fn slot_view(value: &Value) -> Value {
    view(value, false)
}

/// `base` (an index view) with `patch` applied in the published slot form by
/// `apply`, read back as an index view.
pub fn patched(base: &Value, patch: &[Value], mut apply: impl FnMut(&mut Value, &Value)) -> Value {
    let mut slotted = slot_view(base);
    for operation in patch {
        apply(&mut slotted, operation);
    }
    index_view(&slotted)
}
