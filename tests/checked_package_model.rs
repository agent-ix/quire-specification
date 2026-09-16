// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Complete-model definition lock and effective-declaration vector qualification.

use ix_trace_rs::trace;
use quire_verification::contracts::jcs_sha256;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const COMPLETE_MODEL_LOCK: &str =
    include_str!("../proposals/quire-v1/definitions/complete-model-lock.json");
const COMPLETE_MODEL_LOCK_DIR: &str = "proposals/quire-v1/definitions";
const COMPLETE_VALUE_LOCK_BYTES: &[u8] =
    include_bytes!("../proposals/quire-v1/definitions/complete-value-lock.json");
const MODEL_ROOT_DEFINITION: &str =
    include_str!("../proposals/quire-v1/definitions/model-complete.md");
const MODEL_RULE_MANIFEST: &str =
    include_str!("../proposals/quire-v1/definitions/model-complete-rules.json");
const EFFECTIVE_DECLARATION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-effective-declaration-vectors.json");

/// One compile-time byte source for a complete-model qualification-catalog
/// entry, named by its repository-relative path.
struct ModelArtifact {
    path: &'static str,
    bytes: &'static [u8],
}

const MODEL_ARTIFACTS: [ModelArtifact; 16] = [
    ModelArtifact {
        path: "proposals/quire-v1/definitions/model-complete.md",
        bytes: include_bytes!("../proposals/quire-v1/definitions/model-complete.md"),
    },
    ModelArtifact {
        path: "proposals/quire-v1/definitions/state-graph.md",
        bytes: include_bytes!("../proposals/quire-v1/definitions/state-graph.md"),
    },
    ModelArtifact {
        path: "proposals/quire-v1/definitions/model-complete-rules.json",
        bytes: include_bytes!("../proposals/quire-v1/definitions/model-complete-rules.json"),
    },
    ModelArtifact {
        path: "proposals/checked-package-v2/model-effective-declaration.schema.json",
        bytes: include_bytes!(
            "../proposals/checked-package-v2/model-effective-declaration.schema.json"
        ),
    },
    ModelArtifact {
        path: "proposals/checked-package-v2/model-effective-declaration-vectors.json",
        bytes: include_bytes!(
            "../proposals/checked-package-v2/model-effective-declaration-vectors.json"
        ),
    },
    ModelArtifact {
        path: "spec/assurance/AD-006-model-graph-state-contracts.md",
        bytes: include_bytes!("../spec/assurance/AD-006-model-graph-state-contracts.md"),
    },
    ModelArtifact {
        path: "spec/functional/foundation/FR-043-evaluate-finite-graph-relations.md",
        bytes: include_bytes!(
            "../spec/functional/foundation/FR-043-evaluate-finite-graph-relations.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-152-bind-systems-model-structures.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-152-bind-systems-model-structures.md"
        ),
    },
    ModelArtifact {
        path: "spec/functional/type-model/FR-153-query-closed-model-environments.md",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-153-query-closed-model-environments.md"
        ),
    },
    ModelArtifact {
        path: "spec/objects/foundation/FR-204-declared-object-identity.md",
        bytes: include_bytes!("../spec/objects/foundation/FR-204-declared-object-identity.md"),
    },
    ModelArtifact {
        path: "spec/objects/interfaces/FR-321-model-selection-artifact.md",
        bytes: include_bytes!("../spec/objects/interfaces/FR-321-model-selection-artifact.md"),
    },
    ModelArtifact {
        path: "spec/objects/interfaces/interface-003-model-selection.md",
        bytes: include_bytes!("../spec/objects/interfaces/interface-003-model-selection.md"),
    },
];

const EFFECTIVE_DECLARATION_DOMAIN: &str = "quire.model.effective-declaration/v1";

fn parsed_fixture(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn raw_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn unqualified_jcs_sha256(value: &Value) -> String {
    jcs_sha256(value)
        .expect("authored preimage is canonicalizable")
        .strip_prefix("sha256-jcs:")
        .expect("canonicalizer retains its selected prefix")
        .to_owned()
}

fn str_at<'a>(value: &'a Value, pointer: &str) -> &'a str {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("authored member {pointer} is a string"))
}

/// Resolves a lock-relative artifact path against `base` into a
/// repository-relative path, refusing a path that climbs above the root.
fn repository_path(base: &str, relative: &str) -> String {
    let mut segments = base.split('/').collect::<Vec<_>>();
    for segment in relative.split('/') {
        match segment {
            ".." => {
                segments
                    .pop()
                    .expect("artifact path stays inside the repository");
            }
            "." => {}
            other => segments.push(other),
        }
    }
    segments.join("/")
}

fn model_artifact(base: &str, relative: &str) -> &'static ModelArtifact {
    let path = repository_path(base, relative);
    MODEL_ARTIFACTS
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("{path} has compile-time bytes"))
}

/// The text between the first pair of backticks in `text`.
fn first_code_span(text: &str) -> Option<&str> {
    let (_, rest) = text.split_once('`')?;
    rest.split_once('`').map(|(span, _)| span)
}

/// Expected `(identity, revision)` for each repository-relative catalog path,
/// stated independently of the lock by the root definition's own header and
/// always-selected table and by the rule manifest's entries.
fn expected_catalog_identities() -> BTreeMap<String, (String, String)> {
    let mut expected = BTreeMap::new();
    let header = MODEL_ROOT_DEFINITION
        .lines()
        .find_map(|line| line.strip_prefix("Definition identity: "))
        .expect("root definition states its identity");
    let (identity, revision) = header
        .split_once("; revision: ")
        .expect("root header states identity and revision");
    expected.insert(
        repository_path(COMPLETE_MODEL_LOCK_DIR, "model-complete.md"),
        (
            first_code_span(identity)
                .expect("identity is a code span")
                .to_owned(),
            first_code_span(revision)
                .expect("revision is a code span")
                .to_owned(),
        ),
    );

    let table = MODEL_ROOT_DEFINITION
        .split_once("## Always-selected definitions")
        .expect("root definition has an always-selected section")
        .1
        .lines()
        .skip_while(|line| !line.starts_with("| `"))
        .take_while(|line| line.starts_with("| `"));
    for row in table {
        let cells = row.split('|').map(str::trim).collect::<Vec<_>>();
        let [_, identity, revision, artifact, _] = cells.as_slice() else {
            panic!("always-selected row has three cells: {row}");
        };
        let link = artifact
            .split_once("](")
            .and_then(|(_, rest)| rest.strip_suffix(')'))
            .expect("definition artifact is a relative link");
        expected.insert(
            repository_path(COMPLETE_MODEL_LOCK_DIR, link),
            (
                first_code_span(identity)
                    .expect("identity is a code span")
                    .to_owned(),
                first_code_span(revision)
                    .expect("revision is a code span")
                    .to_owned(),
            ),
        );
    }

    let manifest = parsed_fixture(MODEL_RULE_MANIFEST);
    for rule in manifest["rules"].as_array().expect("rules is an array") {
        expected.insert(
            repository_path(COMPLETE_MODEL_LOCK_DIR, str_at(rule, "/path")),
            (
                str_at(rule, "/identity").to_owned(),
                str_at(rule, "/revision").to_owned(),
            ),
        );
    }
    expected
}

/// Every way the lock's catalog identities disagree with the independently
/// stated expected set: an unexpected path, a relabelled identity or
/// revision, or an expected artifact the catalog omits.
fn catalog_identity_mismatches(
    lock: &Value,
    expected: &BTreeMap<String, (String, String)>,
) -> Vec<String> {
    let mut mismatches = Vec::new();
    let mut seen = BTreeSet::new();
    for entry in lock["qualification_catalog"]
        .as_array()
        .expect("qualification catalog is an array")
    {
        let path = repository_path(COMPLETE_MODEL_LOCK_DIR, str_at(entry, "/artifact_path"));
        let actual = (
            str_at(entry, "/definition/identity"),
            str_at(entry, "/definition/revision/value"),
        );
        match expected.get(&path) {
            None => mismatches.push(format!("{path} is not a stated artifact")),
            Some((identity, revision)) if (identity.as_str(), revision.as_str()) != actual => {
                mismatches.push(format!(
                    "{path} states {identity}@{revision} but the lock has {}@{}",
                    actual.0, actual.1
                ));
            }
            Some(_) => {}
        }
        seen.insert(path);
    }
    mismatches.extend(
        expected
            .keys()
            .filter(|path| !seen.contains(*path))
            .map(|path| format!("{path} is missing from the lock")),
    );
    mismatches
}

fn assert_definition_ref(definition: &Value, bytes: &[u8]) {
    let identity = str_at(definition, "/identity");
    assert_eq!(str_at(definition, "/authority"), "agent-ix");
    assert_eq!(str_at(definition, "/revision/namespace"), "quire-draft");
    assert_eq!(
        str_at(definition, "/digest_domain"),
        "quire.definition.bytes/v1"
    );
    assert_eq!(
        str_at(definition, "/digest"),
        raw_sha256(bytes),
        "{identity} binds exact authored bytes"
    );
}

/// FR-204 key bytes of an identity component: the UTF-8 domain string
/// immediately followed by the 32 digest bytes.
fn identity_key_bytes(component: &Value) -> Vec<u8> {
    let digest = str_at(component, "/digest");
    assert_eq!(digest.len(), 64, "a digest is 32 bytes of lowercase hex");
    let mut bytes = str_at(component, "/domain").as_bytes().to_vec();
    bytes.extend(digest.as_bytes().chunks(2).map(|pair| {
        let pair = std::str::from_utf8(pair).expect("hex digest is ASCII");
        u8::from_str_radix(pair, 16).expect("digest is hexadecimal")
    }));
    bytes
}

fn reference_key(entry: &Value) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    (
        identity_key_bytes(&entry["universe"]),
        identity_key_bytes(&entry["type"]),
        str_at(entry, "/object").as_bytes().to_vec(),
    )
}

#[trace("TC-195", "FR-150-AC-6")]
#[test]
fn tc_195_complete_model_lock_catalogs_every_artifact_byte_for_byte() {
    let lock = parsed_fixture(COMPLETE_MODEL_LOCK);
    assert_eq!(
        lock["lock_version"].as_str(),
        Some("quire.model.definition-lock/v1")
    );
    assert_eq!(
        lock["digest_domain"].as_str(),
        Some("quire.definition.bytes/v1")
    );

    let requires = lock["requires"]
        .as_array()
        .expect("model lock requires an array");
    assert_eq!(requires.len(), 1);
    assert_eq!(requires[0]["role"].as_str(), Some("complete_value_lock"));
    assert_eq!(
        requires[0]["artifact_path"].as_str(),
        Some("complete-value-lock.json")
    );
    assert_eq!(
        str_at(&requires[0], "/definition/identity"),
        "quire.value.definition-lock/v1"
    );
    assert_definition_ref(&requires[0]["definition"], COMPLETE_VALUE_LOCK_BYTES);

    let catalog = lock["qualification_catalog"]
        .as_array()
        .expect("qualification catalog is an array");
    assert_eq!(catalog.len(), MODEL_ARTIFACTS.len());
    let mut paths = BTreeSet::new();
    let mut roles = BTreeSet::new();
    for entry in catalog {
        let relative = str_at(entry, "/artifact_path");
        let artifact = model_artifact(COMPLETE_MODEL_LOCK_DIR, relative);
        assert!(paths.insert(artifact.path), "catalog paths are unique");
        assert!(
            roles.insert(str_at(entry, "/role")),
            "catalog roles are unique"
        );
        assert_definition_ref(&entry["definition"], artifact.bytes);
    }
    assert_eq!(
        catalog_identity_mismatches(&lock, &expected_catalog_identities()),
        Vec::<String>::new(),
        "every catalog identity and revision is the one its documents state"
    );
    let always = lock["package_selection"]["always"]
        .as_array()
        .expect("always selection is an array")
        .iter()
        .map(|role| role.as_str().expect("role is a string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(always, roles, "every catalog role is always selected");
}

#[trace("TC-195", "FR-150-AC-6")]
#[test]
fn tc_195_complete_model_rule_manifest_binds_every_rule_byte_for_byte() {
    let manifest = parsed_fixture(MODEL_RULE_MANIFEST);
    assert_eq!(
        manifest["definition_identity"].as_str(),
        Some("quire.model.complete.rules/v1")
    );
    let lock = parsed_fixture(COMPLETE_MODEL_LOCK);
    let catalog = lock["qualification_catalog"]
        .as_array()
        .expect("qualification catalog is an array");
    let rules = manifest["rules"]
        .as_array()
        .expect("rule manifest contains an array");
    assert!(!rules.is_empty());
    for rule in rules {
        let identity = str_at(rule, "/identity");
        let relative = str_at(rule, "/path");
        let artifact = model_artifact(COMPLETE_MODEL_LOCK_DIR, relative);
        assert_eq!(
            str_at(rule, "/sha256"),
            raw_sha256(artifact.bytes),
            "{identity} binds exact authored bytes"
        );
        let entry = catalog
            .iter()
            .find(|entry| entry["definition"]["identity"].as_str() == Some(identity))
            .unwrap_or_else(|| panic!("{identity} is catalogued by the lock"));
        assert_eq!(str_at(entry, "/artifact_path"), relative);
        assert_eq!(str_at(entry, "/definition/digest"), str_at(rule, "/sha256"));
    }
}

#[trace("TC-195", "FR-150-AC-6")]
#[test]
fn tc_195_effective_declaration_vectors_recompute_their_digests() {
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let vectors = file["vectors"].as_array().expect("vectors is an array");
    assert_eq!(vectors.len(), 16);
    let mut names = BTreeSet::new();
    for vector in vectors {
        let name = str_at(vector, "/name");
        assert!(names.insert(name), "vector names are unique");
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            str_at(vector, "/sha256"),
            "{name} digest is the JCS SHA-256 of its preimage"
        );
    }

    let digests = vectors
        .iter()
        .map(|vector| str_at(vector, "/sha256"))
        .collect::<BTreeSet<_>>();
    for view in ["n01-view", "n02-view"] {
        let vector = vectors
            .iter()
            .find(|vector| vector["name"].as_str() == Some(view))
            .unwrap_or_else(|| panic!("{view} is published"));
        let declarations = vector["preimage"]["declarations"]
            .as_array()
            .expect("view declarations are an array");
        let mut previous: Option<&str> = None;
        for declaration in declarations {
            let effective_id = str_at(declaration, "/effective_id/digest");
            assert_eq!(
                str_at(declaration, "/effective_id/domain"),
                EFFECTIVE_DECLARATION_DOMAIN
            );
            assert_eq!(
                unqualified_jcs_sha256(&declaration["preimage"]),
                effective_id,
                "{view} entry identity is recomputed from its preimage"
            );
            assert!(
                digests.contains(effective_id),
                "{view} entry is a published declaration vector"
            );
            assert!(
                previous < Some(effective_id),
                "{view} declarations ascend strictly by effective identity"
            );
            previous = Some(effective_id);
        }
    }
}

#[trace("TC-198", "FR-144-AC-11")]
#[test]
fn tc_198_reference_key_order_ascends_by_fr_204_key_bytes() {
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let order = file["reference_key_order"]
        .as_array()
        .expect("reference key order is an array");
    assert!(order.len() >= 2);
    let keys = order.iter().map(reference_key).collect::<Vec<_>>();
    assert!(
        keys.windows(2).all(|pair| pair[0] < pair[1]),
        "published keys ascend strictly by universe, type and object bytes"
    );
    let mut reversed = keys.clone();
    reversed.reverse();
    reversed.sort();
    assert_eq!(reversed, keys, "sorting any permutation restores the order");
}

#[trace("TC-195", "FR-150-AC-6")]
#[test]
fn tc_195_complete_model_lock_refuses_relabelled_catalog_identities() {
    let expected = expected_catalog_identities();
    assert_eq!(expected.len(), MODEL_ARTIFACTS.len());
    let lock = parsed_fixture(COMPLETE_MODEL_LOCK);
    assert!(catalog_identity_mismatches(&lock, &expected).is_empty());

    let mut relabelled = lock.clone();
    relabelled["qualification_catalog"][0]["definition"]["identity"] =
        Value::from("quire.model.complete/v2");
    assert_eq!(catalog_identity_mismatches(&relabelled, &expected).len(), 1);

    let mut swapped = lock.clone();
    let first = swapped["qualification_catalog"][5]["definition"]["identity"].clone();
    let second = swapped["qualification_catalog"][6]["definition"]["identity"].clone();
    swapped["qualification_catalog"][5]["definition"]["identity"] = second;
    swapped["qualification_catalog"][6]["definition"]["identity"] = first;
    assert_eq!(catalog_identity_mismatches(&swapped, &expected).len(), 2);

    let mut revised = lock.clone();
    revised["qualification_catalog"][1]["definition"]["revision"]["value"] =
        Value::from("1-draft.2");
    assert_eq!(catalog_identity_mismatches(&revised, &expected).len(), 1);

    let mut omitted = lock;
    omitted["qualification_catalog"]
        .as_array_mut()
        .expect("qualification catalog is an array")
        .pop();
    assert_eq!(catalog_identity_mismatches(&omitted, &expected).len(), 1);
}
