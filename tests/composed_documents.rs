// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-154 composed documents (TC-945): a document that is both an artifact type
//! and carries `object:` is classified by `type` alone and declares no model
//! node. The oracle below is the rule as FR-154 states it.

use ix_trace_rs::trace;
use serde_json::Value;
use std::collections::BTreeSet;

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/composed-document-vectors.json");

fn vectors() -> Value {
    serde_json::from_str(VECTORS).expect("authored fixture is valid JSON")
}

fn text<'a>(value: &'a Value, member: &str) -> &'a str {
    value[member]
        .as_str()
        .unwrap_or_else(|| panic!("authored member {member} is a string"))
}

fn array<'a>(value: &'a Value, member: &str) -> &'a [Value] {
    value[member]
        .as_array()
        .unwrap_or_else(|| panic!("authored member {member} is an array"))
}

fn object_types(v: &Value) -> BTreeSet<&str> {
    array(v, "object_types")
        .iter()
        .map(|t| t.as_str().expect("object type is a string"))
        .collect()
}

/// `^[A-Za-z][A-Za-z0-9_]*$`
fn is_object_id(id: &str) -> bool {
    let mut chars = id.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The lift: one node per document whose `type` is an object type. The
/// `object:` member and the `id` shape are never consulted.
fn lifted_identity(document: &Value, package: &str, types: &BTreeSet<&str>) -> Option<String> {
    types
        .contains(text(document, "type"))
        .then(|| format!("ix://{package}/{}", text(document, "id")))
}

fn name_segment<'a>(identity: &'a str, package: &str) -> &'a str {
    identity
        .strip_prefix(&format!("ix://{package}/"))
        .expect("identity is rooted at the package")
}

#[test]
#[trace("TC-945", "FR-154-AC-14")]
fn a_composed_document_declares_no_node_and_an_object_type_declares_one() {
    let v = vectors();
    let package = text(&v, "package_identity");
    let types = object_types(&v);
    for document in array(&v, "documents") {
        let case = text(document, "case");
        let lifted = lifted_identity(document, package, &types);
        let declares = document["declares_node"].as_bool().expect("flag");
        assert_eq!(lifted.is_some(), declares, "{case}");
        if declares {
            assert_eq!(lifted.as_deref(), Some(text(document, "node_identity")), "{case}");
            assert!(is_object_id(name_segment(&lifted.unwrap(), package)), "{case}");
        }
    }
}

#[test]
#[trace("TC-945", "FR-154-AC-14")]
fn no_document_of_an_artifact_type_is_lifted_and_no_lifted_name_has_a_hyphen() {
    let v = vectors();
    let package = text(&v, "package_identity");
    let types = object_types(&v);
    let docs = array(&v, "documents");
    let all: Vec<String> =
        docs.iter().filter_map(|d| lifted_identity(d, package, &types)).collect();
    for document in docs.iter().filter(|d| !types.contains(text(d, "type"))) {
        let id = format!("ix://{package}/{}", text(document, "id"));
        assert!(!all.contains(&id), "{}", text(document, "case"));
    }
    assert!(!all.iter().any(|i| name_segment(i, package).contains('-')));
}

#[test]
#[trace("TC-945", "FR-154-AC-14")]
fn a_reference_to_a_composed_document_is_a_missing_declaration() {
    let v = vectors();
    for case in array(&v, "reference_cases") {
        let nodes: BTreeSet<&str> = array(case, "package_nodes")
            .iter()
            .map(|n| n.as_str().expect("node is a string"))
            .collect();
        let refusal = (!nodes.contains(text(case, "reference")))
            .then(|| ("missing_declaration", "missing-name"));
        let expected = &case["expected"];
        match refusal {
            Some((code, cause)) => {
                assert_eq!(text(expected, "code"), code, "{}", text(case, "case"));
                assert_eq!(text(expected, "cause"), cause, "{}", text(case, "case"));
            }
            None => assert!(expected.is_null(), "{}", text(case, "case")),
        }
    }
}

#[test]
#[trace("TC-945", "FR-154-AC-14")]
fn a_node_named_by_an_artifact_type_id_is_malformed() {
    let v = vectors();
    let package = text(&v, "package_identity");
    for case in array(&v, "node_cases") {
        let name = name_segment(text(case, "node_identity"), package);
        let expected = &case["expected"];
        if is_object_id(name) {
            assert!(expected.is_null(), "{}", text(case, "case"));
        } else {
            assert_eq!(text(expected, "code"), "invalid_model_binding");
            assert_eq!(text(expected, "cause"), "malformed-declaration");
            assert_eq!(text(expected, "member_path"), "TypeDefinition.identity");
        }
    }
}
