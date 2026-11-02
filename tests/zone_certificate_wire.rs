// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-418 "Canonical bytes" qualification (TC-371): the zone certificate's
//! RFC 8785 byte encoding, number and option encoding, the absence of any
//! digest member, and the worked vectors ZC-10 to ZC-12.

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_canonicalize;
use serde_json::{Value, json};

const SCHEMA: &str = include_str!("../proposals/backend-provider-v1/schema.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const VECTORS: &str =
    include_str!("../proposals/backend-provider-v1/zone-certificate-vectors.json");

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored fixture is valid JSON")
}

fn validator() -> jsonschema::Validator {
    let registry = jsonschema::Registry::new()
        .add(
            "urn:quire:checked-package:v2",
            parsed(CHECKED_PACKAGE_SCHEMA),
        )
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/ZoneCertificate"}))
        .expect("ZoneCertificate definition compiles")
}

fn task_search_validator() -> jsonschema::Validator {
    let registry = jsonschema::Registry::new()
        .add(
            "urn:quire:checked-package:v2",
            parsed(CHECKED_PACKAGE_SCHEMA),
        )
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/TaskSearchEvidence"}))
        .expect("TaskSearchEvidence definition compiles")
}

fn vectors() -> Vec<Value> {
    let file = parsed(VECTORS);
    file["vectors"].as_array().expect("vectors").clone()
}

fn vector(id: &str) -> Value {
    vectors()
        .into_iter()
        .find(|v| v["id"] == id)
        .expect("vector exists")
}

#[trace("TC-371", "FR-418-AC-6")]
#[trace("TC-371", "FR-418-AC-14")]
#[test]
fn tc_371_zone_certificate_bytes_are_the_rfc_8785_encoding_of_the_wire_value() {
    let valid = validator();
    let rows = vectors();
    assert_eq!(rows.len(), 7);
    for row in &rows {
        let id = row["id"].as_str().unwrap();
        let document = &row["document"];
        assert!(valid.is_valid(document), "{id} is a ZoneCertificate");
        let first = jcs_canonicalize(document).expect("canonicalizes");
        assert_eq!(
            String::from_utf8(first.clone()).unwrap(),
            row["canonical_text"].as_str().unwrap(),
            "{id}: published canonical text"
        );
        assert_eq!(
            first.len(),
            row["byte_length"].as_u64().unwrap() as usize,
            "{id}: published byte length"
        );
        let second = jcs_canonicalize(&parsed(&document.to_string())).expect("canonicalizes");
        assert_eq!(
            first, second,
            "{id}: equal values give byte-equal certificates"
        );
        // Members in code-unit order, no insignificant whitespace, no trailing newline.
        assert!(first.first() == Some(&b'{') && first.last() == Some(&b'}'));
        assert!(!first.contains(&b'\n'), "no newline");
        assert!(
            !String::from_utf8(first.clone()).unwrap().contains("\": \""),
            "no padding after a colon"
        );
        let reread = parsed(std::str::from_utf8(&first).unwrap());
        assert_eq!(&reread, document, "{id}: the bytes read back to the value");
    }
    // The member order is the sorted order, whatever order the value was built in.
    let zc10 = vector("ZC-10")["canonical_text"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(
        zc10.starts_with("{\"admitted\":{\"edges\":[0],\"start\":0},\"bounds\":"),
        "admitted first, its members ascending"
    );
    assert!(zc10.ends_with(",\"scale\":\"1\"}"), "scale last");
    assert!(
        zc10.contains("\"initials\":[0],\"nodes\":"),
        "initials before nodes"
    );
}

#[trace("TC-371", "FR-418-AC-11", "FR-418-AC-13")]
#[trace("TC-371", "FR-418-AC-14")]
#[test]
fn tc_371_task_certificate_uses_finite_templates_and_rejects_exact_task_identities() {
    let valid = validator();
    let zc19 = vector("ZC-19")["document"].clone();
    assert!(
        valid.is_valid(&zc19),
        "ZC-19 admits a periodic finite-template certificate"
    );
    let template = &zc19["edges"][0]["transition"];
    assert_eq!(template["type"], "task-transition-template");
    assert!(template.get("job").is_none());
    assert!(template.get("generation").is_none());
    assert!(template.get("demand").is_none());
    assert!(template.get("jitter").is_none());
    assert!(template.get("deadline").is_none());

    let malformed = parsed(VECTORS);
    for case in malformed["malformed_vectors"]
        .as_array()
        .expect("malformed vectors")
    {
        let id = case["id"].as_str().unwrap();
        let mut document = vector(case["base"].as_str().unwrap())["document"].clone();
        let pointer = case["pointer"].as_str().unwrap();
        *document
            .pointer_mut(pointer)
            .expect("mutation pointer exists") = case["value"].clone();
        assert!(
            !valid.is_valid(&document),
            "{id}: malformed finite template is refused"
        );
    }
    for case in malformed["task_reader_vectors"]
        .as_array()
        .expect("task reader vectors")
    {
        let id = case["id"].as_str().unwrap();
        let mut document = vector(case["base"].as_str().unwrap())["document"].clone();
        let pointer = case["pointer"].as_str().unwrap();
        *document
            .pointer_mut(pointer)
            .expect("mutation pointer exists") = case["value"].clone();
        assert!(
            valid.is_valid(&document),
            "{id}: semantic mismatch retains valid wire shape"
        );
        assert_eq!(case["expected"]["code"], "certificate-rejected", "{id}");
        assert_eq!(case["expected"]["rule"], "successor-missing", "{id}");
        assert!(
            case["expected"]["at"].is_object(),
            "{id}: reader refusal has a locus"
        );
        if id == "ZC-28" {
            assert_eq!(case["checked_task_model"], "TC-372/CtlFp");
            assert_eq!(case["base"], "ZC-30");
            assert_eq!(case["expected"]["at"], json!({"node": 0}));
            assert_eq!(case["value"][0]["transition"]["operation"], "Cell::set");
            let base = vector("ZC-30")["document"].clone();
            assert_eq!(base["identity"]["model"], "test/litmus");
            assert_eq!(base["identity"]["ordinal"], 0);
            assert_eq!(base["identity"]["node_id"]["package"], "package");
            let identity_path = base["identity"]["node_id"]["path"].as_array().unwrap();
            assert_eq!(
                identity_path
                    .iter()
                    .filter(|part| part["node"] == 6)
                    .count(),
                18
            );
            assert_eq!(
                base["task_product"]["queue_bounds"],
                json!([
                    {"taskset":"Ctl","task":"t1","arrival":"periodic","interarrival":{"numerator":"4","denominator":"1"},"relative_deadline":{"numerator":"4","denominator":"1"},"max_jitter":{"numerator":"0","denominator":"1"},"work_lower":{"numerator":"1","denominator":"1"},"work_upper":{"numerator":"1","denominator":"1"},"max_jobs":2},
                    {"taskset":"Ctl","task":"t2","arrival":"periodic","interarrival":{"numerator":"6","denominator":"1"},"relative_deadline":{"numerator":"6","denominator":"1"},"max_jitter":{"numerator":"0","denominator":"1"},"work_lower":{"numerator":"2","denominator":"1"},"work_upper":{"numerator":"2","denominator":"1"},"max_jobs":2},
                    {"taskset":"Ctl","task":"t3","arrival":"periodic","interarrival":{"numerator":"12","denominator":"1"},"relative_deadline":{"numerator":"12","denominator":"1"},"max_jitter":{"numerator":"0","denominator":"1"},"work_lower":{"numerator":"5","denominator":"1"},"work_upper":{"numerator":"5","denominator":"1"},"max_jobs":2}
                ])
            );
        }
    }

    let task_identity = json!({
        "type":"task-transition", "action":"dispatch", "taskset":"S", "task":"A",
        "job":0, "slot":0, "generation":0,
        "release":{"type":"transition","operation":"Task::release","receiver":null,"arguments":[]},
        "source_transition":null,
        "demand":{"numerator":"1","denominator":"1"},
        "jitter":{"numerator":"0","denominator":"1"},
        "deadline":{"numerator":"2","denominator":"1"}
    });
    let frontier = json!({
        "type":"task-search-frontier",
        "identity":{"node_id":{"ordinal":0,"package":"example","path":["Example","TaskSet"]},"model":"example","ordinal":0},
        "nodes":[{"state":{"key":zc19["nodes"][0]["state"]["key"]},"depth":0,"zone":[],"expanded":false}],
        "edges":[{"node":0,"transition":task_identity,"target":0}],
        "frontier":[{"node":0,"reason":"depth-limit","open":["successors"]}],
        "limit":{"type":"limit-reached","limit":"max_depth","value":1,"setting":"analysis.max_depth"}
    });
    assert!(
        task_search_validator().is_valid(&frontier),
        "task search frontiers retain exact identities for replay"
    );
}

#[trace("TC-371", "FR-418-AC-7")]
#[test]
fn tc_371_numbers_are_counts_or_integer_strings() {
    let valid = validator();
    let base = vector("ZC-10")["document"].clone();
    // A bound is an integer string, never a JSON number.
    let mut number_bound = base.clone();
    number_bound["nodes"][1]["zone"][0][1]["bound"] = json!(-1);
    assert!(!valid.is_valid(&number_bound));
    // A bare JSON number is not a DbmBound entry either.
    let mut bare = base.clone();
    bare["nodes"][0]["zone"][0][0] = json!(0);
    assert!(!valid.is_valid(&bare));
    // A leading zero or a plus sign is not canonical.
    for spelling in ["01", "+1", "-0", "1.0"] {
        let mut bad = base.clone();
        bad["nodes"][1]["zone"][0][1]["bound"] = json!(spelling);
        assert!(!valid.is_valid(&bad), "{spelling}");
    }
    // The scale is a positive integer string; an index is a safe-range count.
    let mut scale = base.clone();
    scale["scale"] = json!("0");
    assert!(!valid.is_valid(&scale));
    let mut huge = base.clone();
    huge["initials"] = json!([9_007_199_254_740_992_u64]);
    assert!(!valid.is_valid(&huge));
    // A bound beyond the safe range is carried exactly as a string.
    let mut big = base.clone();
    big["nodes"][1]["zone"][0][1]["bound"] = json!("-123456789123456789123456789");
    assert!(valid.is_valid(&big));
    let bytes = jcs_canonicalize(&big).unwrap();
    assert!(
        String::from_utf8(bytes)
            .unwrap()
            .contains("\"bound\":\"-123456789123456789123456789\"")
    );
    // Infinity is the one string spelling of an unbounded entry.
    assert_eq!(base["nodes"][0]["zone"][1][0], json!("infinity"));
}

#[trace("TC-371", "FR-418-AC-8")]
#[test]
fn tc_371_an_absent_option_is_omitted_and_never_null() {
    let valid = validator();
    let zc10 = vector("ZC-10")["document"].clone();
    for member in ["components", "reasons", "divergence"] {
        assert!(zc10.get(member).is_none(), "ZC-10 omits {member}");
        let mut null = zc10.clone();
        null[member] = Value::Null;
        assert!(!valid.is_valid(&null), "{member} null is refused");
    }
    let mut null_refinement = vector("ZC-11")["document"].clone();
    null_refinement["refinement"] = Value::Null;
    assert!(!valid.is_valid(&null_refinement));
    // locked is present, as true, exactly on a locked node.
    let zc12 = vector("ZC-12")["document"].clone();
    assert_eq!(zc12["nodes"][0]["locked"], json!(true));
    assert!(zc12["nodes"][1].get("locked").is_none());
    let mut false_locked = zc12.clone();
    false_locked["nodes"][1]["locked"] = json!(false);
    assert!(!valid.is_valid(&false_locked));
    // taken has one entry per edge; a null entry stands for no Taken.
    let mut with_null = zc10.clone();
    with_null["refinement"]["taken"][1] = Value::Null;
    assert!(valid.is_valid(&with_null));
    assert_eq!(
        zc10["refinement"]["taken"].as_array().unwrap().len(),
        zc10["edges"].as_array().unwrap().len()
    );
}

fn digest_bearing_pointers(value: &Value, at: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if map.contains_key("digest")
                || map.contains_key("hash")
                || map.contains_key("checksum")
            {
                out.push(at.to_owned());
            }
            for (key, child) in map {
                digest_bearing_pointers(child, &format!("{at}/{key}"), out);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                digest_bearing_pointers(child, &format!("{at}/{index}"), out);
            }
        }
        _ => {}
    }
}

#[trace("TC-371", "FR-418-AC-9")]
#[test]
fn tc_371_the_certificate_embeds_no_digest() {
    for row in vectors() {
        let id = row["id"].as_str().unwrap().to_owned();
        let mut found = Vec::new();
        digest_bearing_pointers(&row["document"], "", &mut found);
        assert!(found.is_empty(), "{id}: digest-bearing members {found:?}");
        for member in row["document"].as_object().unwrap().keys() {
            assert!(
                !member.contains("digest")
                    && !member.contains("hash")
                    && !member.contains("checksum"),
                "{id}: {member}"
            );
        }
    }
}

#[trace("TC-371", "FR-418-AC-10")]
#[test]
fn tc_371_refinement_form_lists_every_initial_node_and_a_locked_entry_node_uses_the_reserved_index()
{
    let valid = validator();
    let zc12 = vector("ZC-12")["document"].clone();
    let part = &zc12["refinement"];
    let initials = zc12["initials"].as_array().unwrap();
    assert_eq!(initials.len(), 2);
    assert!(
        part.get("initials").is_none(),
        "initials is a top-level member only"
    );
    assert!(
        zc12.get("initial").is_none(),
        "the wire has one name for the concept"
    );
    let positions = part["positions"].as_array().unwrap().len() as u64;
    let locked = &zc12["nodes"][initials[0].as_u64().unwrap() as usize];
    // admitted starts at a positioned initial node, not at the locked initials[0].
    let start = zc12["admitted"]["start"].as_u64().unwrap();
    assert_eq!(start, initials[1].as_u64().unwrap());
    assert!(zc12["nodes"][start as usize].get("locked").is_none());
    assert!(zc12["nodes"][start as usize]["automaton"].as_u64().unwrap() < positions);
    assert_eq!(locked["locked"], json!(true));
    assert_eq!(
        locked["automaton"].as_u64().unwrap(),
        positions,
        "the reserved index names no position"
    );
    assert_eq!(part["taken"], json!([]));
    // initials is required and nonempty; a refinement without positions is malformed.
    for member in [
        "positions",
        "taken",
        "refinement_node",
        "concrete_package",
        "abstract_package",
    ] {
        let mut bad = zc12.clone();
        bad["refinement"].as_object_mut().unwrap().remove(member);
        assert!(!valid.is_valid(&bad), "{member} is required");
    }
    let mut empty = zc12.clone();
    empty["initials"] = json!([]);
    assert!(!valid.is_valid(&empty));
    let mut no_positions = zc12.clone();
    no_positions["refinement"]["positions"] = json!([]);
    assert!(
        !valid.is_valid(&no_positions),
        "an empty positions is not a refinement part"
    );
    let mut alias = zc12.clone();
    alias["initial"] = json!(0);
    assert!(!valid.is_valid(&alias), "initial is not a member");
    // The part's node-id member is `refinement_node`; the old names are refused.
    for old in ["refinement", "node"] {
        let mut old_name = zc12.clone();
        let id = old_name["refinement"]["refinement_node"].clone();
        old_name["refinement"]
            .as_object_mut()
            .unwrap()
            .remove("refinement_node");
        assert!(!valid.is_valid(&old_name), "refinement_node is required");
        old_name["refinement"][old] = id.clone();
        assert!(
            !valid.is_valid(&old_name),
            "the part's node id is not named {old}"
        );
        let mut extra = zc12.clone();
        extra["refinement"][old] = id;
        assert!(!valid.is_valid(&extra), "{old} is not a member of the part");
    }
    let mut inner = zc12.clone();
    inner["refinement"]["initials"] = json!([0, 1]);
    assert!(!valid.is_valid(&inner), "the part carries no initials");
    // A certificate without initials or admitted is not a ZoneCertificate.
    for member in ["initials", "admitted"] {
        let mut bad = zc12.clone();
        bad.as_object_mut().unwrap().remove(member);
        assert!(!valid.is_valid(&bad), "{member} is required");
    }
}
