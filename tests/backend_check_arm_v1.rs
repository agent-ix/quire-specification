// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-483 to FR-485 check-arm vectors (TC-972 to TC-974).
//!
//! Node ids in the vectors are written `$node:<label>`; the harness derives
//! each `NodeId` from its label, so no digest is frozen in the vector file.
//! Each case's outcome stage is validated against the FR-331 schema, its
//! negotiation disposition is recomputed from FR-290's rules, and its arm
//! outcome is checked against FR-483's outcome mapping. Value-validity cases
//! carry a small model that the harness evaluates natively, so a refutation is
//! replayed and a proof is recomputed rather than trusted. The evaluator-dependent
//! contract acceptance criteria are provider obligations the TCs state.

use ix_trace_rs::trace;
use serde_json::{Value, json};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap};
use sha2::{Digest, Sha256};

const SCHEMA: &str = include_str!("../proposals/backend-provider-v1/schema.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const VECTORS: &str = include_str!("../proposals/backend-check-arm-v1/vectors.json");
const TC_972: &str = include_str!("../spec/test-cases/TC-972-check-arm-outcome-contract.md");
const TC_973: &str = include_str!("../spec/test-cases/TC-973-value-validity-check-input.md");
const TC_974: &str = include_str!("../spec/test-cases/TC-974-operation-contract-check-input.md");
const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

fn node_id(label: &str) -> Value {
    let mut hasher = Sha256::new();
    hasher.update(NODE_DOMAIN.as_bytes());
    hasher.update([0]);
    hasher.update(label.as_bytes());
    let digest: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

fn rekey(value: &mut Value) {
    match value {
        Value::String(text) => {
            if let Some(label) = text.strip_prefix("$node:") {
                *value = node_id(label);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rekey),
        Value::Object(map) => map.values_mut().for_each(rekey),
        _ => {}
    }
}

fn cases() -> Vec<Value> {
    parsed(VECTORS)["cases"].as_array().expect("cases").clone()
}

fn outcome_validator() -> jsonschema::Validator {
    let registry = jsonschema::Registry::new()
        .add("urn:quire:checked-package:v2", parsed(CHECKED_PACKAGE_SCHEMA))
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/OutcomeStage"}))
        .expect("outcome stage compiles")
}

const BOUNDABLE: [&str; 4] = ["collection", "population", "integer", "recursive"];

fn advertises(descriptor: &Value, kind: &str, mode: &str) -> bool {
    descriptor["advertises"]
        .as_array()
        .expect("advertises")
        .iter()
        .any(|pair| pair["kind"] == kind && pair["mode"] == mode)
}

/// FR-290's candidate set, domain-kind check and advertised-mode table for one item.
fn negotiate(case: &Value) -> (String, Option<String>) {
    let item = &case["item"];
    let kind = item["kind"].as_str().expect("kind");
    let candidates: Vec<&Value> = case["registry"]
        .as_array()
        .expect("registry")
        .iter()
        .filter(|d| {
            (advertises(d, kind, "bounded") || advertises(d, kind, "unbounded"))
                && item.get("backend").is_none_or(|b| *b == d["backend"])
        })
        .collect();
    let [candidate] = candidates.as_slice() else {
        return ("unsupported".into(), Some("unsupported-requested-capability".into()));
    };
    let extent = &item["extent"];
    let bounded = extent["classification"] == "bounded";
    let unbounded_mode = advertises(candidate, kind, "unbounded");
    let texts = |key: &str| -> Vec<&str> {
        extent[key].as_array().expect("kind list").iter().map(|k| k.as_str().expect("kind text")).collect()
    };
    let compared: Vec<&str> = if bounded {
        texts("bound_kinds")
    } else if unbounded_mode {
        vec![]
    } else {
        texts("domain_kinds").into_iter().filter(|k| BOUNDABLE.contains(k)).collect()
    };
    let domains: Vec<&str> = candidate["domains"].as_array().map_or(vec![], |d| {
        d.iter().map(|k| k.as_str().expect("domain")).collect()
    });
    if compared.iter().any(|k| !domains.contains(k)) {
        return ("unsupported".into(), Some("unsupported-requested-capability".into()));
    }
    if bounded || unbounded_mode {
        return ("supported".into(), None);
    }
    if extent["finite_bound_available"] == true {
        ("requires-bound".into(), None)
    } else {
        ("unsupported".into(), Some("unbounded-extent".into()))
    }
}

const REQUIRED: &[&str] = &[
    "VV-01",
    "VV-02",
    "VV-03",
    "VV-04",
    "VV-05",
    "VV-06",
    "VV-07",
    "VV-08",
    "VV-09",
    "VV-10",
    "VV-11",
    "VV-12",
    "VV-13",
    "VV-14",
    "VV-15",
    "VV-16",
    "SH-01",
    "SH-02",
    "SH-03",
    "SH-04",
    "SH-05",
    "SH-06",
    "SH-07",
    "SH-08",
    "SH-09",
    "SH-10",
    "SH-11",
    "SH-12",
    "SH-13",
    "SH-14",
    "SH-15",
    "SH-16",
    "SH-17",
    "SH-18",
    "SH-19",
    "SH-20",
    "OC-01",
    "OC-02",
    "OC-03",
    "OC-04",
    "OC-05",
    "OC-06",
    "OC-07",
    "OC-08",
    "OC-09",
    "OC-10",
    "OC-11",
    "OC-12",
    "OC-13",
    "OC-14",
    "OC-15",
    "OC-16",
    "OC-17",
    "OC-18",
    "OC-19",
    "OC-20",
    "OC-21",
    "OC-22",
    "OC-23",
    "OC-24",
    "VV-17",
];

const REQUIRED_ACS: &[&str] = &[
    "FR-483-AC-1", "FR-483-AC-2", "FR-483-AC-3", "FR-483-AC-4", "FR-483-AC-5", "FR-483-AC-6",
    "FR-483-AC-7", "FR-483-AC-8", "FR-483-AC-9", "FR-484-AC-1", "FR-484-AC-2", "FR-484-AC-3",
    "FR-484-AC-4", "FR-484-AC-5", "FR-484-AC-6", "FR-484-AC-7", "FR-484-AC-8", "FR-484-AC-9",
    "FR-485-AC-1", "FR-485-AC-2", "FR-485-AC-3", "FR-485-AC-4", "FR-485-AC-5", "FR-485-AC-6",
    "FR-485-AC-7", "FR-485-AC-8", "FR-485-AC-9", "FR-485-AC-10", "FR-485-AC-11", "FR-485-AC-12",
    "FR-485-AC-13", "FR-485-AC-14",
];


fn composed(case: &Value) -> Value {
    let mut document = case["outcome"].clone();
    rekey(&mut document);
    document
}

fn id(case: &Value) -> String {
    case["id"].as_str().expect("id").to_owned()
}

fn result_of(document: &Value) -> Option<&Value> {
    document["results"].as_array().and_then(|r| r.first())
}

const VV_FORMS: [&str; 3] = ["clause", "case-clause", "scalar-operation"];
const OC_FORMS: [&str; 5] = ["invariant", "clause", "frame", "step-relation", "simulation"];

fn role_of(form: &str) -> &'static str {
    match form {
        "scalar-operation" => "expression",
        "simulation" => "generated",
        "frame" => "anchor",
        _ => "claim",
    }
}

/// FR-483 "Check input" gate: what the arm itself refuses before evaluating.
fn gate(case: &Value) -> Option<(&'static str, &'static str, Option<&'static str>)> {
    let item = &case["item"];
    let form = item["form"].as_str().expect("form");
    let forms: &[&str] = if item["kind"] == "value-validity" { &VV_FORMS } else { &OC_FORMS };
    let occurrence_role = item["occurrence"]["role"].as_str().expect("role");
    if !forms.contains(&form) || occurrence_role != role_of(form) {
        return Some(("declined", "invalid-input", None));
    }
    if item["missing_node"] == true || item["unbound_element"] == true {
        return Some(("declined", "incomplete-input", None));
    }
    if item["kind"] == "operation-contract" {
        if form == "simulation" && item["refinement_step_simulable"] == false {
            return Some(("unsupported", "", Some("no-contract-level-reading")));
        }
        if form == "frame" && item["bound"] != true {
            return Some(("unsupported", "", Some("no-contract-level-reading")));
        }
        let has_engine = case["registry"][0]["implementation_engine"] == true;
        if item["bound"] == true && !has_engine {
            return Some(("unsupported", "", Some("no-implementation-engine")));
        }
    }
    None
}

fn check_limit(case: &Value) -> Option<u64> {
    case["item"]["check_limit"].as_u64()
}

/// The descriptor FR-290 selects for the item: its published `bounds.max_cases` is the default.
fn default_limit(case: &Value) -> Option<u64> {
    let item = &case["item"];
    let kind = item["kind"].as_str()?;
    case["registry"].as_array()?.iter().find(|d| {
        (advertises(d, kind, "bounded") || advertises(d, kind, "unbounded"))
            && item.get("backend").is_none_or(|b| *b == d["backend"])
    })?["bounds"]["max_cases"]
        .as_u64()
}

#[trace("TC-972", "FR-483-AC-6")]
#[trace("TC-972", "FR-483-AC-7")]
#[trace("TC-972", "FR-483-AC-8")]
#[trace("TC-973", "FR-484-AC-8")]
#[trace("TC-974", "FR-485-AC-12")]
#[test]
fn check_arm_negotiation_dispositions_follow_fr_290() {
    for case in cases() {
        let (disposition, cause) = negotiate(&case);
        assert_eq!(disposition, case["negotiation"]["disposition"], "{} disposition", case["id"]);
        assert_eq!(cause.as_deref(), case["negotiation"]["cause"].as_str(), "{} cause", case["id"]);
    }
}

#[trace("TC-972", "FR-483-AC-1")]
#[trace("TC-972", "FR-483-AC-3")]
#[trace("TC-972", "FR-483-AC-5")]
#[trace("TC-972", "FR-483-AC-9")]
#[trace("TC-974", "FR-485-AC-1")]
#[trace("TC-974", "FR-485-AC-7")]
#[trace("TC-974", "FR-485-AC-9")]
#[trace("TC-974", "FR-485-AC-10")]
#[trace("TC-974", "FR-485-AC-11")]
#[trace("TC-974", "FR-485-AC-13")]
#[trace("TC-974", "FR-485-AC-14")]
#[test]
fn check_arm_outcomes_map_to_the_fr_331_results() {
    let validator = outcome_validator();
    for case in cases() {
        let id = id(&case);
        let document = composed(&case);
        let errors: Vec<String> = validator.iter_errors(&document).map(|e| e.to_string()).collect();
        assert_eq!(errors, Vec::<String>::new(), "{id} outcome stage is admitted");
        assert_eq!(document["dispositions"][0]["disposition"], case["negotiation"]["disposition"], "{id} disposition");
        let results = document["results"].as_array().expect("results");
        assert_eq!(!results.is_empty(), case["arm"].is_object() || case["independence"].is_array(), "{id} result iff arm ran");
        let counterexamples = document["counterexamples"].as_array().expect("counterexamples");
        if let Some(independent) = case["independence"].as_array() {
            // FR-483-AC-9: every item has its own terminal record and result.
            assert_eq!(document["accounting"].as_array().unwrap().len(), independent.len(), "{id} one record per item");
            for expected in independent {
                let index = expected["request_index"].as_u64().unwrap();
                let entry = results.iter().find(|r| r["request_index"] == index).expect("own result");
                assert_eq!(entry["kind"], expected["kind"], "{id} item {index} kind");
                assert_eq!(entry["result"], expected["result"], "{id} item {index} result");
            }
            let indexes: BTreeSet<u64> = results.iter().map(|r| r["request_index"].as_u64().unwrap()).collect();
            assert_eq!(indexes.len(), results.len(), "{id} no shared result");
            continue;
        }
        let Some(arm) = case["arm"].as_object() else { continue };
        let result = &results[0];
        assert_eq!(result["result"], arm["result"], "{id} result");
        assert_eq!(result["kind"], case["item"]["kind"], "{id} kind");
        let mapped: &[&str] = match arm["outcome"].as_str() {
            Some("supported") => &["proved"],
            Some("violation") => &["refuted"],
            Some("unknown") => &["inconclusive", "incomplete"],
            Some("unsupported") => &["unsupported"],
            None => &["declined"],
            other => panic!("{id}: unknown arm outcome {other:?}"),
        };
        let kind = result["result"].as_str().unwrap();
        assert!(mapped.contains(&kind), "{id} mapping");
        assert!(!["tested", "measured", "failed"].contains(&kind), "{id} is no arm result");
        // The arm's own refusals follow the gate.
        match gate(&case) {
            Some((expected, cause, reason)) => {
                assert_eq!(kind, expected, "{id} gate result");
                if expected == "declined" {
                    assert_eq!(result["cause"]["type"], cause, "{id} declined cause");
                } else {
                    assert_eq!(result["diagnostic"]["detail"]["reason"], reason.unwrap(), "{id} unsupported reason");
                }
            }
            None => {
                assert!(kind != "declined", "{id} nothing refuses it");
                // Reverse implication: the gate's reasons are produced by the gate alone.
                let reason = result["diagnostic"]["detail"]["reason"].as_str();
                assert!(
                    !matches!(reason, Some("no-contract-level-reading" | "no-implementation-engine")),
                    "{id} carries a gate reason that the gate does not derive"
                );
            }
        }
        // FR-483 replay-before-refuted: the replay is computed natively from the case's model, never declared.
        let status = replay_status(&case, &document);
        let carries_counterexample = !counterexamples.is_empty() || !case["candidate_ce"].is_null();
        if case["item"]["kind"] == "operation-contract" && carries_counterexample {
            assert!(!case["replay_model"].is_null(), "{id} records or receives a counterexample, so it carries a replay_model");
        }
        // Both directions: the recorded cause is exactly what the computed replay says.
        match result["cause"]["type"].as_str() {
            Some("replay-refused") => assert_eq!(status, Some(Replay::Refused), "{id} replay-refused"),
            Some("replay-parity") => assert_eq!(status, Some(Replay::NotReproduced), "{id} replay-parity"),
            _ => {}
        }
        if kind == "refuted" {
            assert_eq!(status, Some(Replay::Reproduced), "{id} refuted only after a replay that reproduced");
            let entry = &counterexamples[result["counterexample"].as_u64().unwrap() as usize];
            let expected = if case["item"]["kind"] == "value-validity" { "assignments" } else { "contract-counterexample" };
            assert_eq!(entry["counterexample"]["type"], expected, "{id} counterexample type");
            assert!(entry.get("transcript").is_none(), "{id} has no transcript");
        } else {
            assert!(counterexamples.is_empty(), "{id} records no counterexample");
            match status {
                Some(Replay::NotReproduced) => {
                    assert_eq!(result["cause"]["type"], "replay-parity", "{id} unreproduced candidate");
                    assert_eq!(kind, "inconclusive", "{id}");
                }
                Some(Replay::Refused) => {
                    assert_eq!(result["cause"]["type"], "replay-refused", "{id} unreplayable candidate");
                    assert_eq!(kind, "inconclusive", "{id}");
                }
                _ => {}
            }
        }
        if kind == "proved" {
            assert!(!result["basis"].is_null(), "{id} proved carries a basis");
        }
    }
}

#[trace("TC-972", "FR-483-AC-2")]
#[trace("TC-972", "FR-483-AC-4")]
#[test]
fn proof_basis_certification_and_limits_follow_fr_483() {
    for case in cases() {
        let id = id(&case);
        let document = composed(&case);
        let Some(result) = result_of(&document) else { continue };
        if case["independence"].is_array() {
            continue;
        }
        if result["result"] == "proved" {
            let depth = if case["item"]["kind"] == "value-validity" { 0 } else { 1 };
            let (basis, certification) = (&result["basis"], result["certification"].as_str().unwrap());
            match case["method"].as_str().expect("a proved case names its method") {
                "enumeration" => {
                    assert_eq!((basis["type"].as_str(), certification), (Some("exhaustive"), "uncertified"), "{id}");
                }
                "kani" => {
                    assert_eq!((basis["type"].as_str(), certification), (Some("bounded-proof"), "certified"), "{id}");
                    assert!(basis["checks"].as_u64().unwrap() > 0, "{id} positive SUCCESS count");
                }
                "plugin" => assert_eq!(certification, "trusted", "{id}"),
                m @ ("smt-bounded-certified" | "smt-bounded-uncertified") => {
                    assert_eq!(basis["type"], "bounded-complete", "{id}");
                    assert_eq!(basis["depth"], depth, "{id} depth");
                    assert_eq!(certification, if m.ends_with("uncertified") { "uncertified" } else { "certified" }, "{id}");
                }
                "smt-unbounded-uncertified" => {
                    assert_eq!(basis["type"], "inductive", "{id}");
                    assert_eq!(basis["depth"], depth, "{id} depth");
                    assert_eq!(certification, "uncertified", "{id}");
                }
                other => panic!("{id}: unknown method {other}"),
            }
            assert!(result.get("cause").is_none(), "{id}");
        }
        if result["result"] == "incomplete" && result["cause"]["type"] == "limit-reached" {
            let effective = check_limit(&case).or_else(|| default_limit(&case));
            assert_eq!(result["cause"]["limit"], "max_cases", "{id}");
            assert_eq!(result["cause"]["value"].as_u64(), effective, "{id} limit is the item's, else the default");
            let setting = if case["item"]["kind"] == "value-validity" { "value_check.max_cases" } else { "contract_check.max_cases" };
            assert_eq!(result["cause"]["setting"], setting, "{id} setting follows FR-461");
        }
    }
    // The item's own max_cases overrides the default (SH-11 proves 256 cases under a default of 50).
    let by_id = |wanted: &str| cases().into_iter().find(|c| c["id"] == wanted).expect("case");
    assert!(check_limit(&by_id("SH-11")).unwrap() > default_limit(&by_id("SH-11")).unwrap());
    // check.limits.max_cases is a wire member: a count of at least one, on the two kinds only.
    let registry = jsonschema::Registry::new()
        .add("urn:quire:checked-package:v2", parsed(CHECKED_PACKAGE_SCHEMA))
        .unwrap()
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .unwrap()
        .prepare()
        .unwrap();
    let check = jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/Check"}))
        .unwrap();
    assert!(check.is_valid(&json!({"limits": {"max_cases": 1000}})));
    for bad in [json!({"limits": {"max_cases": 0}}), json!({"limits": {"max_cases": "9"}}), json!({"limits": {"max_cases": -1}}), json!({"limits": {}}), json!({"limits": {"max_cases": 5, "other": 1}})] {
        assert!(!check.is_valid(&bad), "refused before consumption: {bad}");
    }
}

#[trace("TC-972", "FR-483-AC-8")]
#[test]
fn population_domain_is_decided_only_by_a_candidate_that_lists_it() {
    let listed = cases().into_iter().find(|c| c["id"] == "SH-17").unwrap();
    let lacking = cases().into_iter().find(|c| c["id"] == "SH-16").unwrap();
    assert_eq!(negotiate(&listed).0, "supported");
    assert_eq!(negotiate(&lacking).0, "unsupported");
    let mut for_both = lacking.clone();
    for_both["registry"] = listed["registry"].clone();
    assert_eq!(negotiate(&for_both).0, "supported");
}

// ---- a native evaluator for the value-validity models ----

#[derive(Debug, Clone, PartialEq)]
enum Fail {
    Undefined(Option<String>),
    NotEvaluable,
    Exhausted,
}

struct Ctx<'a> {
    spent: &'a Cell<u64>,
    limit: u64,
    functions: &'a Value,
    reached: RefCell<BTreeSet<String>>,
    undefined: RefCell<BTreeSet<String>>,
}

fn eval(e: &Value, vars: &HashMap<String, i128>, cx: &Ctx) -> Result<i128, Fail> {
    match e {
        Value::Number(n) => Ok(i128::from(n.as_i64().expect("int"))),
        Value::String(name) => Ok(*vars.get(name).expect("variable")),
        Value::Array(items) => {
            let op = items[0].as_str().expect("op");
            let arg = |i: usize| eval(&items[i], vars, cx);
            let b = |v: bool| Ok(i128::from(v));
            match op {
                "+" => Ok(arg(1)? + arg(2)?),
                "-" => Ok(arg(1)? - arg(2)?),
                "*" => Ok(arg(1)? * arg(2)?),
                "/" => {
                    let (a, d) = (arg(1)?, arg(2)?);
                    if d == 0 { Err(Fail::Undefined(None)) } else { Ok(a.div_euclid(d)) }
                }
                ">" => b(arg(1)? > arg(2)?),
                "<" => b(arg(1)? < arg(2)?),
                "<=" => b(arg(1)? <= arg(2)?),
                ">=" => b(arg(1)? >= arg(2)?),
                "=" => b(arg(1)? == arg(2)?),
                "!=" => b(arg(1)? != arg(2)?),
                "and" => {
                    if arg(1)? == 0 { Ok(0) } else { b(arg(2)? != 0) }
                }
                "if" => {
                    if arg(1)? != 0 { arg(2) } else { arg(3) }
                }
                "call" => {
                    let name = items[1].as_str().unwrap();
                    let Some(function) = cx.functions.get(name) else { return Err(Fail::NotEvaluable) };
                    // One evaluation of a function body is one case (FR-483).
                    cx.spent.set(cx.spent.get() + 1);
                    if cx.spent.get() > cx.limit {
                        return Err(Fail::Exhausted);
                    }
                    let params = function["params"].as_array().unwrap();
                    let mut scope = HashMap::new();
                    for (i, p) in params.iter().enumerate() {
                        scope.insert(p.as_str().unwrap().to_owned(), arg(2 + i)?);
                    }
                    eval(&function["body"], &scope, cx)
                }
                "narrow" => {
                    let (lo, hi) = (i128::from(items[1].as_i64().unwrap()), i128::from(items[2].as_i64().unwrap()));
                    let value = arg(3)?;
                    if (lo..=hi).contains(&value) {
                        Ok(value)
                    } else {
                        // The operand is checked against the target range (FR-484-AC-6).
                        if let Some(label) = items[3].get(1).and_then(Value::as_str).filter(|_| items[3][0] == "@") {
                            cx.undefined.borrow_mut().insert(label.to_owned());
                        }
                        Err(Fail::Undefined(None))
                    }
                }
                "@" => {
                    let label = items[1].as_str().unwrap().to_owned();
                    cx.reached.borrow_mut().insert(label.clone());
                    match eval(&items[2], vars, cx) {
                        Err(Fail::Undefined(None)) => {
                            cx.undefined.borrow_mut().insert(label.clone());
                            Err(Fail::Undefined(Some(label)))
                        }
                        other => other,
                    }
                }
                other => panic!("unknown op {other}"),
            }
        }
        other => panic!("bad expression {other}"),
    }
}

#[derive(Debug, PartialEq)]
enum Verdict {
    Proved,
    Refuted(HashMap<String, i128>, bool),
    Vacuous,
    NotEvaluable,
    Incomplete(u64),
}

fn violates(case: &Value, vars: &HashMap<String, i128>, spent: &Cell<u64>, limit: u64) -> Result<(bool, bool, bool), Fail> {
    // (violated, undefined, reached)
    let model = &case["model"];
    let functions = model.get("functions").unwrap_or(&Value::Null);
    let cx = Ctx { spent, limit, functions, reached: RefCell::new(BTreeSet::new()), undefined: RefCell::new(BTreeSet::new()) };
    if let Some(goal) = model.get("goal") {
        return match eval(goal, vars, &cx) {
            Ok(v) => Ok((v == 0, false, true)),
            Err(Fail::Undefined(_)) => Ok((true, true, true)),
            Err(other) => Err(other),
        };
    }
    let target = model["occurrence"].as_str().unwrap();
    if let Err(stop @ (Fail::NotEvaluable | Fail::Exhausted)) = eval(&model["body"], vars, &cx) {
        return Err(stop);
    }
    let reached = cx.reached.borrow().contains(target);
    let undefined = cx.undefined.borrow().contains(target);
    Ok((undefined, undefined, reached))
}

fn assignments(model: &Value) -> Vec<HashMap<String, i128>> {
    let mut all = vec![HashMap::new()];
    for v in model["variables"].as_array().unwrap() {
        let (lo, hi) = (i128::from(v["lo"].as_i64().unwrap()), i128::from(v["hi"].as_i64().unwrap()));
        all = all
            .into_iter()
            .flat_map(|m| (lo..=hi).map(move |x| { let mut m = m.clone(); m.insert(v["name"].as_str().unwrap().to_owned(), x); m }))
            .collect();
    }
    all
}

fn decide(case: &Value) -> Verdict {
    // There is no unlimited fallback: the descriptor always publishes a default.
    let limit = check_limit(case).or_else(|| default_limit(case)).expect("a max_cases for every item");
    let spent = Cell::new(0);
    let mut any_reached = case["model"].get("goal").is_some();
    for vars in &assignments(&case["model"]) {
        spent.set(spent.get() + 1);
        if spent.get() > limit {
            return Verdict::Incomplete(limit);
        }
        match violates(case, vars, &spent, limit) {
            Err(Fail::Exhausted) => return Verdict::Incomplete(limit),
            Err(_) => return Verdict::NotEvaluable,
            Ok((true, undefined, _)) => return Verdict::Refuted(vars.clone(), undefined),
            Ok((false, _, reached)) => any_reached |= reached,
        }
    }
    if any_reached { Verdict::Proved } else { Verdict::Vacuous }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Replay {
    Reproduced,
    NotReproduced,
    Refused,
}

fn record_fields(value: &Value, suffix: &str) -> HashMap<String, i128> {
    value["fields"]
        .as_array()
        .map(|fields| {
            fields
                .iter()
                .map(|f| (format!("{}{suffix}", f["name"].as_str().unwrap()), f["value"]["value"].as_str().unwrap().parse().unwrap()))
                .collect()
        })
        .unwrap_or_default()
}

fn argument_env(arguments: &Value, names: &Value) -> HashMap<String, i128> {
    let mut env = HashMap::new();
    for (name, label) in names.as_object().into_iter().flatten() {
        let node = node_id(label.as_str().unwrap());
        let found = arguments.as_array().unwrap().iter().find(|a| a["parameter"] == node).expect("argument");
        env.insert(name.clone(), found["value"]["value"].as_str().unwrap().parse().unwrap());
    }
    env
}

/// FR-485 "Replay", over the vector's small model: an unbound counterexample is evaluated on the recorded
/// values; a bound one re-executes the implementation on the recorded concrete inputs, requires the recorded
/// concrete outcome to be one the implementation produces, compares the abstraction images, then evaluates.
fn replay_contract(model: &Value, ce: &Value, reader_has_engine: bool) -> Replay {
    let bound = ce["bound"] == true;
    if bound && !reader_has_engine {
        return Replay::Refused;
    }
    let executions = ce["executions"].as_array().unwrap();
    let spent = Cell::new(0);
    let cx = Ctx { spent: &spent, limit: u64::MAX, functions: &Value::Null, reached: RefCell::new(BTreeSet::new()), undefined: RefCell::new(BTreeSet::new()) };
    let names = &model["args"];
    let mut frame_hit = false;
    let mut env: HashMap<String, i128> = HashMap::new();
    for (index, execution) in executions.iter().enumerate() {
        let image_pre = record_fields(&execution["pre_state"], "");
        let image_post = record_fields(&execution["post_state"], "");
        if bound {
            let concrete = &execution["concrete"];
            let (pre, post) = (record_fields(&concrete["pre_state"], ""), record_fields(&concrete["post_state"], ""));
            let mut inputs = pre.clone();
            inputs.extend(argument_env(&concrete["arguments"], names));
            let produced = model["outcomes"].as_array().unwrap().iter().any(|outcome| {
                let mut next = pre.clone();
                for (field, expr) in outcome.as_object().unwrap() {
                    next.insert(field.clone(), eval(expr, &inputs, &cx).expect("effect evaluates"));
                }
                next == post
            });
            if !produced {
                return Replay::NotReproduced;
            }
            for (concrete_state, image) in [(&pre, &image_pre), (&post, &image_post)] {
                for (field, location) in model["abstraction"].as_object().unwrap() {
                    if image.get(field) != concrete_state.get(location.as_str().unwrap()) {
                        return Replay::NotReproduced;
                    }
                }
            }
            if model["frame"] == true {
                let inside: BTreeSet<&str> = model["modifies"].as_array().unwrap().iter().map(|m| model["abstraction"][m.as_str().unwrap()].as_str().unwrap()).collect();
                frame_hit |= pre.iter().any(|(location, value)| !inside.contains(location.as_str()) && post.get(location) != Some(value));
            }
        }
        if index == 0 {
            env.extend(image_pre);
        }
        let suffix = ["'", "''"][index.min(1)];
        env.extend(image_post.into_iter().map(|(k, v)| (format!("{k}{suffix}"), v)));
        env.extend(argument_env(&execution["arguments"], names));
    }
    if model["frame"] == true {
        return if frame_hit { Replay::Reproduced } else { Replay::NotReproduced };
    }
    for premise in model["premises"].as_array().unwrap() {
        if eval(premise, &env, &cx) != Ok(1) {
            return Replay::NotReproduced;
        }
    }
    let violated = match eval(&model["obligation"], &env, &cx) {
        Err(Fail::Undefined(_)) => true,
        Ok(value) => value == 0 && model["undefined_only"] != true,
        Err(_) => false,
    };
    if violated { Replay::Reproduced } else { Replay::NotReproduced }
}

/// The native replay of a case's recorded or candidate counterexample, derived from its model.
fn replay_status(case: &Value, document: &Value) -> Option<Replay> {
    let recorded = document["counterexamples"].as_array().and_then(|c| c.first()).map(|e| e["counterexample"].clone());
    let from = |reproduced: bool| if reproduced { Replay::Reproduced } else { Replay::NotReproduced };
    if case["item"]["kind"] == "value-validity" {
        if case["model"].is_null() {
            return case["replay"]["reproduced"].as_bool().map(from);
        }
        return replayed(case, document).map(from);
    }
    let model = &case["replay_model"];
    if model.is_null() {
        return None;
    }
    let candidate = recorded.or_else(|| {
        let mut ce = case["candidate_ce"].clone();
        rekey(&mut ce);
        (!ce.is_null()).then_some(ce)
    })?;
    let engine = case["reader"]["implementation_engine"] != false;
    Some(replay_contract(model, &candidate, engine))
}

fn replayed(case: &Value, document: &Value) -> Option<bool> {
    // The assignment a candidate counterexample names, replayed natively.
    let values = if let Some(entry) = document["counterexamples"].as_array().and_then(|c| c.first()) {
        entry["counterexample"]["values"].clone()
    } else {
        Value::Null
    };
    let mut vars = HashMap::new();
    if values.is_array() {
        for variable in case["model"]["variables"].as_array()? {
            let node = node_id(&format!("vv.{}", variable["name"].as_str()?));
            let found = values.as_array()?.iter().find(|v| v["parameter"] == node)?;
            vars.insert(variable["name"].as_str()?.to_owned(), found["value"]["value"].as_str()?.parse().ok()?);
        }
    } else {
        for (name, value) in case["candidate"].as_object()? {
            vars.insert(name.clone(), i128::from(value.as_i64()?));
        }
        // A single-variable candidate written as {"x": n} names the variable `x`.
    }
    let spent = Cell::new(0);
    violates(case, &vars, &spent, u64::MAX).ok().map(|(violated, _, _)| violated)
}

#[trace("TC-973", "FR-484-AC-1")]
#[trace("TC-973", "FR-484-AC-2")]
#[trace("TC-973", "FR-484-AC-4")]
#[trace("TC-973", "FR-484-AC-5")]
#[trace("TC-973", "FR-484-AC-6")]
#[trace("TC-973", "FR-484-AC-7")]
#[trace("TC-973", "FR-484-AC-9")]
#[test]
fn value_validity_vectors_agree_with_the_native_evaluation() {
    let mut modelled = 0;
    let mut recursion = false;
    for case in cases() {
        let id = id(&case);
        if case["item"]["kind"] != "value-validity" {
            continue;
        }
        let document = composed(&case);
        let result = result_of(&document);
        // Claims no form accepts are declined by the gate, with no evaluation.
        if gate(&case).is_some() {
            assert_eq!(result.unwrap()["result"], "declined", "{id}");
            continue;
        }
        if case["model"].is_null() || result.is_none() {
            continue;
        }
        modelled += 1;
        let result = result.unwrap();
        let verdict = decide(&case);
        let expected = match (&verdict, result["result"].as_str().unwrap(), result["cause"]["type"].as_str()) {
            (Verdict::Proved, "proved", _) => true,
            (Verdict::Vacuous, "inconclusive", Some("kani_vacuous_proof")) => true,
            (Verdict::NotEvaluable, "unsupported", _) => true,
            (Verdict::Incomplete(limit), "incomplete", Some("limit-reached")) => {
                recursion |= case["model"]["functions"]["loop"].is_object();
                result["cause"]["value"] == *limit
            }
            (Verdict::Refuted(..), "refuted", cause) => {
                // The recorded assignment is replayed and must reproduce the violation.
                assert_eq!(replayed(&case, &document), Some(true), "{id} replay reproduces");
                let undefined = matches!(verdict, Verdict::Refuted(_, true));
                assert_eq!(cause == Some("undefined-evaluation"), undefined, "{id} undefined-evaluation iff undefined");
                true
            }
            _ => false,
        };
        // SH-13: the engine claims a violation at a candidate that the replay does not reproduce.
        if result["cause"]["type"] == "replay-parity" {
            assert_eq!(replayed(&case, &document), Some(false), "{id} the candidate does not reproduce");
            assert_eq!(decide(&case), Verdict::Proved, "{id} the claim in fact holds");
            continue;
        }
        assert!(expected, "{id}: evaluator says {verdict:?}, vector says {result}");
    }
    assert!(recursion, "a recursive call stopped by max_cases is exercised");
    assert!(modelled >= 10, "the evaluator ran on the modelled value-validity cases");
}

#[trace("TC-974", "FR-485-AC-9")]
#[test]
fn contract_counterexamples_have_the_fr_485_shape() {
    let mut seen = BTreeSet::new();
    for case in cases() {
        let id = id(&case);
        if case["item"]["kind"] != "operation-contract" {
            continue;
        }
        let document = composed(&case);
        let Some(entry) = document["counterexamples"].as_array().and_then(|c| c.first()) else { continue };
        let ce = &entry["counterexample"];
        let obligation = ce["obligation"].as_str().unwrap();
        seen.insert(obligation.to_owned());
        assert_eq!(Some(obligation), case["cex_obligation"].as_str(), "{id} names its obligation");
        assert_eq!(ce["bound"], case["item"]["bound"], "{id} bound mirrors the item");
        let executions = ce["executions"].as_array().unwrap();
        for execution in executions {
            assert_eq!(!execution["concrete"].is_null(), ce["bound"] == true, "{id} concrete iff bound");
        }
        match obligation {
            "initial-state" | "simulation-initial" => assert!(executions[0]["operation"].is_null(), "{id}"),
            "relation" => assert_eq!(executions.len(), 2, "{id} two executions"),
            _ => assert_eq!(executions.len(), 1, "{id}"),
        }
        let ordinal = case["item"]["occurrence"]["ordinal"].as_u64().unwrap();
        if obligation.starts_with("simulation") {
            // FR-322-AC-88..91: ordinal 0 is the initial rule, k+1 is step row k.
            assert_eq!(obligation == "simulation-initial", ordinal == 0, "{id} ordinal");
            assert_eq!(ce["violated"]["occurrence"]["ordinal"], ordinal, "{id} violated locus");
        }
        if obligation == "frame" {
            // The write outside modifies is visible in the concrete values alone.
            let e = &executions[0];
            assert_eq!(e["pre_state"], e["post_state"], "{id} images agree");
            assert_ne!(e["concrete"]["pre_state"], e["concrete"]["post_state"], "{id} concrete differs");
        }
    }
    for expected in ["initial-state", "preservation", "definedness", "postcondition", "frame", "relation", "simulation-initial", "simulation-step"] {
        assert!(seen.contains(expected), "a vector exercises {expected}");
    }
}

#[test]
fn schema_refuses_a_malformed_contract_counterexample() {
    let validator = outcome_validator();
    let case = cases().into_iter().find(|c| c["id"] == "OC-10").unwrap();
    let good = composed(&case);
    assert!(validator.is_valid(&good));
    let mut unbound_with_concrete = good.clone();
    unbound_with_concrete["counterexamples"][0]["counterexample"]["bound"] = json!(false);
    assert!(!validator.is_valid(&unbound_with_concrete), "unbound forbids concrete");
    let mut bound_without = good.clone();
    bound_without["counterexamples"][0]["counterexample"]["executions"][0].as_object_mut().unwrap().remove("concrete");
    assert!(!validator.is_valid(&bound_without), "bound requires concrete");
    let mut no_bound = good;
    no_bound["counterexamples"][0]["counterexample"].as_object_mut().unwrap().remove("bound");
    assert!(!validator.is_valid(&no_bound), "bound is required");
}

#[test]
fn the_vector_file_is_complete_and_freezes_no_digest() {
    // A pinned case list: deleting a vector fails.
    let ids: Vec<String> = cases().iter().map(id).collect();
    assert_eq!(ids, REQUIRED.to_vec(), "the case list is exactly the pinned list");
    assert_eq!(parsed(VECTORS)["required_cases"], json!(REQUIRED));
    let covered: BTreeSet<String> = cases().iter().flat_map(|c| c["acs"].as_array().unwrap().iter().map(|a| a.as_str().unwrap().to_owned()).collect::<Vec<_>>()).collect();
    for &ac in REQUIRED_ACS {
        assert!(covered.contains(ac), "{ac} has a vector");
    }
    // Every case that settles a replayed refutation or a replay-parity cause is present.
    for must in ["OC-19", "SH-13"] {
        assert!(ids.iter().any(|i| i == must));
    }
    // The evaluator-dependent criteria are stated as the provider's own obligation in the TC that owns them.
    for ac in parsed(VECTORS)["provider_obligations"].as_array().unwrap() {
        let ac = ac.as_str().unwrap();
        let text = if ac.starts_with("FR-484") { TC_973 } else { TC_974 };
        assert!(text.contains("provider's own tests SHALL cover"), "{ac} TC states the obligation");
        assert!(text.contains(ac), "{ac} is named in its TC");
    }
    let _ = TC_972;
    assert!(!VECTORS.contains("\"digest\": \""), "no digest is written in the vectors");
    assert_eq!(node_id("a"), node_id("a"));
    assert_ne!(node_id("a"), node_id("b"));
    let case = cases().into_iter().find(|c| c["id"] == "VV-02").unwrap();
    assert_eq!(composed(&case)["counterexamples"][0]["counterexample"]["values"][0]["parameter"], node_id("vv.x"));
}

#[test]
fn descriptors_for_these_kinds_must_publish_a_positive_max_cases() {
    let registry = jsonschema::Registry::new()
        .add("urn:quire:checked-package:v2", parsed(CHECKED_PACKAGE_SCHEMA))
        .unwrap()
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .unwrap()
        .prepare()
        .unwrap();
    let validator = jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/BackendDescriptor"}))
        .unwrap();
    let build = node_id("build-inputs")["digest"].clone();
    let descriptor = |kind: &str, bounds: &Value| {
        let mut d = json!({
            "backend": "checker",
            "tool": {"name": "checker", "version": "1.0.0", "build": {"domain": "quire.build-inputs/v1", "algorithm": "sha256", "digest": build}},
            "advertises": [{"kind": kind, "mode": "bounded"}],
            "domains": ["integer"]
        });
        if !bounds.is_null() {
            d["bounds"] = bounds.clone();
        }
        d
    };
    for entry in parsed(VECTORS)["descriptor_cases"].as_array().unwrap() {
        for kind in ["value-validity", "operation-contract"] {
            let d = descriptor(kind, &entry["bounds"]);
            assert_eq!(validator.is_valid(&d), entry["valid"] == true, "{kind}: {}", entry["name"]);
        }
    }
    for case in cases() {
        for d in case["registry"].as_array().unwrap() {
            assert!(d["bounds"]["max_cases"].as_u64().is_some_and(|n| n > 0), "{} {}", case["id"], d["backend"]);
        }
    }
}
