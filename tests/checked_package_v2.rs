// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! I04 checked-package V2 and complete-value qualification vectors.

use ix_trace_rs::trace;
use num_bigint::BigInt;
use num_integer::Integer;
use quire_verification::contracts::jcs_sha256;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const SCHEMA_V1: &str = include_str!("../proposals/checked-package-v1/schema.json");
const README_V1_BYTES: &[u8] = include_bytes!("../proposals/checked-package-v1/README.md");
const SCHEMA_V1_BYTES: &[u8] = include_bytes!("../proposals/checked-package-v1/schema.json");
const POSITIVE_ALL_FAMILIES_V1_BYTES: &[u8] =
    include_bytes!("../proposals/checked-package-v1/fixtures/positive-all-families.json");
const ADVERSE_V1_BYTES: &[u8] =
    include_bytes!("../proposals/checked-package-v1/fixtures/adverse.json");
const NODE_PREIMAGE_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/node-identity-preimage.schema.json");
const NODE_IDENTITY_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/node-identity-vectors.json");
const MIGRATION_CORRESPONDENCE_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/migration-correspondence.schema.json");
const MIGRATION_CORRESPONDENCE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/migration-correspondence-vectors.json");
const MIGRATION_SOURCE_NOMINAL_V1: &str =
    include_str!("../proposals/checked-package-v2/fixtures/migration-source-nominal-v1.json");
const OPERATION_CATALOG: &str =
    include_str!("../proposals/checked-package-v2/operation-catalog.json");
const POSITIVE_OPERATION_IDENTITIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-operation-identities.json");
const COMPOUND_UNIT_SCHEMA: &str =
    include_str!("../proposals/quire-v1/definitions/value-compound-unit.schema.json");
const COMPOUND_UNIT_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/value-compound-unit-vectors.json");
const VALUE_RULE_MANIFEST: &str =
    include_str!("../proposals/quire-v1/definitions/value-complete-rules.json");
const COMPLETE_VALUE_LOCK: &str =
    include_str!("../proposals/quire-v1/definitions/complete-value-lock.json");
const COMPLETE_VALUE_SELECTION_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/complete-value-selection-vectors.json");

/// Whether a qualification-catalog artifact is a value definition or one of
/// the exact rule documents listed by the rule manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactKind {
    Definition,
    Rule,
}

/// One compile-time byte source for a complete-value qualification-catalog entry.
struct CatalogArtifact {
    kind: ArtifactKind,
    role: &'static str,
    path: &'static str,
    identity: &'static str,
    revision: &'static str,
    bytes: &'static [u8],
}

const CATALOG_ARTIFACTS: [CatalogArtifact; 21] = [
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "edition",
        path: "edition.md",
        identity: "ix:native",
        revision: "1-draft.2",
        bytes: include_bytes!("../proposals/quire-v1/definitions/edition.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "root",
        path: "value-complete.md",
        identity: "quire.value.complete/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-complete.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "accounting",
        path: "value-accounting.md",
        identity: "quire.value.accounting/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-accounting.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "compound_unit",
        path: "value-compound-unit.md",
        identity: "quire.value.compound-unit/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-compound-unit.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "compound_unit_schema",
        path: "value-compound-unit.schema.json",
        identity: "quire.value.compound-unit.schema/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-compound-unit.schema.json"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "compound_unit_vectors",
        path: "value-compound-unit-vectors.json",
        identity: "quire.value.compound-unit.vectors/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-compound-unit-vectors.json"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "rule_manifest",
        path: "value-complete-rules.json",
        identity: "quire.value.complete.rules/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-complete-rules.json"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "text_profile",
        path: "value-text-unicode-17.md",
        identity: "quire.value.text.unicode-17.0.0/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-text-unicode-17.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "ieee_profile",
        path: "value-ieee754-2019-default.md",
        identity: "quire.value.ieee754-2019-default/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-ieee754-2019-default.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "integer_division_euclidean",
        path: "value-integer-division-euclidean.md",
        identity: "quire.value.integer-division.euclidean/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../proposals/quire-v1/definitions/value-integer-division-euclidean.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "integer_division_floor",
        path: "value-integer-division-floor.md",
        identity: "quire.value.integer-division.floor/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/definitions/value-integer-division-floor.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Definition,
        role: "integer_division_truncating",
        path: "value-integer-division-truncating.md",
        identity: "quire.value.integer-division.truncating/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../proposals/quire-v1/definitions/value-integer-division-truncating.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_package_contract",
        path: "../package-contract.md",
        identity: "quire.rule.package-contract/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/package-contract.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_shared_grammar",
        path: "../shared-grammar.md",
        identity: "quire.rule.shared-grammar/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../proposals/quire-v1/shared-grammar.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_ad_005",
        path: "../../../spec/assurance/AD-005-complete-value-expression-system.md",
        identity: "quire.rule.ad-005/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../spec/assurance/AD-005-complete-value-expression-system.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_140",
        path: "../../../spec/functional/type-model/FR-140-evaluate-exact-decimals.md",
        identity: "quire.rule.fr-140/v1",
        revision: "1-draft.1",
        bytes: include_bytes!("../spec/functional/type-model/FR-140-evaluate-exact-decimals.md"),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_141",
        path: "../../../spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md",
        identity: "quire.rule.fr-141/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_142",
        path: "../../../spec/functional/type-model/FR-142-evaluate-quantities-and-units.md",
        identity: "quire.rule.fr-142/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-142-evaluate-quantities-and-units.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_147",
        path: "../../../spec/functional/expressions/FR-147-evaluate-integer-division-domains.md",
        identity: "quire.rule.fr-147/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../spec/functional/expressions/FR-147-evaluate-integer-division-domains.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_148",
        path: "../../../spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md",
        identity: "quire.rule.fr-148/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md"
        ),
    },
    CatalogArtifact {
        kind: ArtifactKind::Rule,
        role: "rule_fr_149",
        path: "../../../spec/functional/type-model/FR-149-apply-complete-equality-matrix.md",
        identity: "quire.rule.fr-149/v1",
        revision: "1-draft.1",
        bytes: include_bytes!(
            "../spec/functional/type-model/FR-149-apply-complete-equality-matrix.md"
        ),
    },
];
const POSITIVE_ALL_FAMILIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-all-families.json");
const POSITIVE_ALL_FAMILIES_V1: &str =
    include_str!("../proposals/checked-package-v1/fixtures/positive-all-families.json");
const POSITIVE_NOMINAL_IDENTITIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-nominal-identities.json");
const MIGRATION_SOURCE_CLAUSES_V1: &str =
    include_str!("../proposals/checked-package-v2/fixtures/migration-source-clauses-v1.json");
const POSITIVE_CLAUSE_OPERATIONS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-clause-operations.json");
const ADVERSE: &str = include_str!("../proposals/checked-package-v2/fixtures/adverse.json");
const REQUIRED_NODE_TAGS: [&str; 13] = [
    "scalar_type",
    "composite_type",
    "bounded_domain",
    "value",
    "expression",
    "function",
    "model",
    "relation",
    "state",
    "temporal",
    "protocol",
    "claim",
    "correspondence",
];
const RECONSTRUCTION_FIELDS: [&str; 6] = [
    "sources",
    "edition",
    "profile_selections",
    "definition_selections",
    "model_selections",
    "dependency_selections",
];

fn parsed_fixture(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn unqualified_jcs_sha256(value: &Value) -> String {
    jcs_sha256(value)
        .expect("authored preimage is canonicalizable")
        .strip_prefix("sha256-jcs:")
        .expect("canonicalizer retains its selected prefix")
        .to_owned()
}

fn digest<'a>(value: &'a Value, pointer: &str) -> &'a str {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .expect("authored node reference has a digest")
}

/// The mathematical integer denoted by a canonical decimal-string member.
/// A noncanonical spelling such as `-0` or `01` denotes nothing, and callers
/// compare the parsed value so no spelling can evade a zero check.
fn integer(value: &Value) -> Option<BigInt> {
    let spelling = value.as_str()?;
    let number = spelling.parse::<BigInt>().ok()?;
    (number.to_string() == spelling).then_some(number)
}

/// Whether a decimal-string member denotes exactly `expected`.
fn integer_is(value: &Value, expected: i32) -> bool {
    integer(value) == Some(BigInt::from(expected))
}

fn rational_is_canonical(value: &Value) -> bool {
    let (Some(numerator), Some(denominator)) =
        (integer(&value["numerator"]), integer(&value["denominator"]))
    else {
        return false;
    };
    if denominator <= BigInt::from(0)
        || (numerator == BigInt::from(0) && denominator != BigInt::from(1))
    {
        return false;
    }
    numerator.gcd(&denominator) == BigInt::from(1)
}

fn terms_are_canonical(terms: &[Value], reference_field: &str) -> bool {
    let mut previous = None;
    for term in terms {
        if integer(&term["exponent"]).is_none_or(|exponent| exponent == BigInt::from(0)) {
            return false;
        }
        let current = term[reference_field]["digest"].as_str();
        if current.is_none() || previous >= current {
            return false;
        }
        previous = current;
    }
    true
}

fn vector_for_digest<'a>(vectors: &'a [Value], wanted: &str) -> Option<&'a Value> {
    vectors
        .iter()
        .find(|vector| vector["sha256"].as_str() == Some(wanted))
}

fn catalog_artifact(path: &str) -> Option<&'static CatalogArtifact> {
    CATALOG_ARTIFACTS
        .iter()
        .find(|artifact| artifact.path == path)
}

fn definition_lock_entry_matches_bytes(entry: &Value) -> bool {
    let Some(artifact) = entry["artifact_path"].as_str().and_then(catalog_artifact) else {
        return false;
    };
    let definition = &entry["definition"];
    let expected_digest = format!("{:x}", Sha256::digest(artifact.bytes));
    entry["role"].as_str() == Some(artifact.role)
        && definition["authority"].as_str() == Some("agent-ix")
        && definition["identity"].as_str() == Some(artifact.identity)
        && definition["revision"]["namespace"].as_str() == Some("quire-draft")
        && definition["revision"]["value"].as_str() == Some(artifact.revision)
        && definition["digest_domain"].as_str() == Some("quire.definition.bytes/v1")
        && definition["digest"].as_str() == Some(expected_digest.as_str())
}

fn unit_chain_reaches_root(preimage: &Value, vectors: &[Value]) -> bool {
    let dimension = &preimage["dimension_node_id"];
    let mut current = preimage;
    let mut visited = BTreeSet::new();
    loop {
        let scale = &current["scale"];
        let offset = &current["offset"];
        if !rational_is_canonical(scale)
            || !rational_is_canonical(offset)
            || integer_is(&scale["numerator"], 0)
            || current["dimension_node_id"] != *dimension
        {
            return false;
        }
        let Some(target) = current["target_unit_node_id"].as_object() else {
            return integer_is(&scale["numerator"], 1)
                && integer_is(&scale["denominator"], 1)
                && integer_is(&offset["numerator"], 0)
                && integer_is(&offset["denominator"], 1);
        };
        let target_digest = target["digest"]
            .as_str()
            .expect("schema-validated target digest is a string");
        if !visited.insert(target_digest) {
            return false;
        }
        let Some(target_vector) = vector_for_digest(vectors, target_digest) else {
            return false;
        };
        current = &target_vector["preimage"];
        if current["version"].as_str() != Some("quire.unit-node/v1") {
            return false;
        }
    }
}

fn semantic_preimage_is_valid(preimage: &Value, vectors: &[Value]) -> bool {
    match preimage["version"].as_str() {
        Some("quire.enum-declaration-node/v1") => {
            let members = preimage["members"]
                .as_array()
                .expect("schema-validated enum members are an array");
            preimage["ordered"].as_bool() == Some(true)
                || members
                    .windows(2)
                    .all(|pair| pair[0].as_str() < pair[1].as_str())
        }
        Some("quire.enum-member-node/v1") => {
            let declaration =
                vector_for_digest(vectors, digest(preimage, "/declaration_node_id/digest"));
            declaration.is_some_and(|vector| {
                vector["preimage"]["members"]
                    .as_array()
                    .is_some_and(|members| members.contains(&preimage["case"]))
            })
        }
        Some("quire.dimension-node/v1") => {
            let terms = preimage["terms"]
                .as_array()
                .expect("schema-validated dimension terms are an array");
            terms_are_canonical(terms, "dimension_node_id")
                && terms.iter().all(|term| {
                    vector_for_digest(vectors, digest(term, "/dimension_node_id/digest"))
                        .is_some_and(|vector| {
                            vector["preimage"]["version"].as_str()
                                == Some("quire.dimension-node/v1")
                                && vector["preimage"]["terms"]
                                    .as_array()
                                    .is_some_and(Vec::is_empty)
                        })
                })
        }
        Some("quire.unit-node/v1") => unit_chain_reaches_root(preimage, vectors),
        _ => false,
    }
}

fn compound_unit_preimage_is_valid(preimage: &Value, node_vectors: &[Value]) -> bool {
    let terms = preimage["terms"]
        .as_array()
        .expect("schema-validated compound-unit terms are an array");
    terms_are_canonical(terms, "unit_node_id")
        && terms.iter().all(|term| {
            vector_for_digest(node_vectors, digest(term, "/unit_node_id/digest")).is_some_and(
                |vector| {
                    vector["preimage"]["version"].as_str() == Some("quire.unit-node/v1")
                        && vector["preimage"]["target_unit_node_id"].is_null()
                },
            )
        })
}

fn owner_joins_lock(preimage: &Value, lock: &Value) -> bool {
    let Some(owner) = preimage["owner"].as_object() else {
        return preimage["version"].as_str() == Some("quire.enum-member-node/v1");
    };
    let authority = owner["authority"].as_str();
    let identity = owner["identity"].as_str();
    match owner["kind"].as_str() {
        Some("source") => lock["sources"].as_array().is_some_and(|sources| {
            sources.iter().any(|source| {
                source["authority"].as_str() == authority && source["identity"].as_str() == identity
            })
        }),
        Some("definition") => {
            let selected = lock["definition_selections"]
                .as_array()
                .is_some_and(|definitions| {
                    definitions.iter().any(|definition| {
                        definition["authority"].as_str() == authority
                            && definition["identity"].as_str() == identity
                    })
                });
            let edition = &lock["edition"]["definition"];
            selected
                || (edition["authority"].as_str() == authority
                    && edition["identity"].as_str() == identity)
        }
        Some("model") => lock["model_selections"].as_array().is_some_and(|models| {
            models.iter().any(|model| {
                model["authority"].as_str() == authority
                    && model["identity"].as_str() == identity
                    && model["export"].as_str() == owner["export"].as_str()
            })
        }),
        _ => false,
    }
}

fn node_dependencies_match(node: &Value, expected: &[&Value]) -> bool {
    let Some(dependencies) = node["dependencies"].as_array() else {
        return false;
    };
    dependencies.len() == expected.len()
        && expected
            .iter()
            .all(|wanted| dependencies.iter().any(|actual| actual == *wanted))
}

fn nominal_node_cross_fields_are_valid(node: &Value) -> bool {
    let preimage = &node["nominal_identity_preimage"];
    match preimage["version"].as_str() {
        Some("quire.enum-declaration-node/v1") => {
            node["semantic_type"] == node["node_id"] && node_dependencies_match(node, &[])
        }
        Some("quire.enum-member-node/v1") => {
            let declaration = &preimage["declaration_node_id"];
            node["semantic_type"] == *declaration
                && node_dependencies_match(node, &[declaration])
                && node["body"]["term"].as_str() == Some("literal")
                && node["body"]["value_kind"].as_str() == Some("enum")
                && node["body"]["value"] == preimage["case"]
        }
        Some("quire.dimension-node/v1") => {
            let Some(terms) = preimage["terms"].as_array() else {
                return false;
            };
            let expected = terms
                .iter()
                .map(|term| &term["dimension_node_id"])
                .collect::<Vec<_>>();
            node["semantic_type"] == node["node_id"] && node_dependencies_match(node, &expected)
        }
        Some("quire.unit-node/v1") => {
            let dimension = &preimage["dimension_node_id"];
            let mut expected = vec![dimension];
            if !preimage["target_unit_node_id"].is_null() {
                expected.push(&preimage["target_unit_node_id"]);
            }
            node["semantic_type"] == *dimension && node_dependencies_match(node, &expected)
        }
        _ => false,
    }
}

fn package_lock_matches_identity_preimage(package: &Value) -> bool {
    let lock = &package["lock"];
    let preimage = &package["identity_preimage"];
    lock["edition"] == preimage["edition"]
        && lock["profile_selections"] == preimage["profile_selections"]
        && lock["definition_selections"] == preimage["definition_selections"]
        && lock["model_selections"] == preimage["model_selections"]
        && lock["required_features"] == preimage["required_features"]
        && lock["dependency_selections"] == preimage["dependency_selections"]
}

fn package_nominal_nodes_are_valid(package: &Value) -> bool {
    if !package_lock_matches_identity_preimage(package)
        || unqualified_jcs_sha256(&package["identity_preimage"])
            != package["package_id"]["digest"].as_str().unwrap_or_default()
    {
        return false;
    }
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nominal package nodes are an array");
    let vectors = nodes
        .iter()
        .map(|node| {
            json!({
                "sha256": node["node_id"]["digest"],
                "preimage": node["nominal_identity_preimage"]
            })
        })
        .collect::<Vec<_>>();
    let projections = package["identity_preimage"]["identity_projection"]
        .as_array()
        .expect("nominal package projections are an array");

    nodes.iter().all(|node| {
        let preimage = &node["nominal_identity_preimage"];
        let node_digest = node["node_id"]["digest"].as_str().unwrap_or_default();
        let Some(projection) = projections
            .iter()
            .find(|projection| projection["node_id"] == node["node_id"])
        else {
            return false;
        };
        let mut projected_node = node.clone();
        projected_node
            .as_object_mut()
            .expect("node is an object")
            .remove("occurrences");
        projected_node == *projection
            && unqualified_jcs_sha256(preimage) == node_digest
            && semantic_preimage_is_valid(preimage, &vectors)
            && nominal_node_cross_fields_are_valid(node)
            && owner_joins_lock(preimage, &package["lock"])
    })
}

fn replace_node_field_in_graph_and_projection(
    package: &mut Value,
    node_index: usize,
    relative_pointer: &str,
    replacement: Value,
) {
    for prefix in [
        "/semantic_graph/nodes",
        "/identity_preimage/identity_projection",
    ] {
        let pointer = format!("{prefix}/{node_index}{relative_pointer}");
        *package
            .pointer_mut(&pointer)
            .expect("authored package-node mutation pointer exists") = replacement.clone();
    }
}

fn refresh_package_id(package: &mut Value) {
    package["package_id"]["digest"] = json!(unqualified_jcs_sha256(&package["identity_preimage"]));
}

fn replace_nominal_preimage_in_graph_and_projection(
    package: &mut Value,
    node_index: usize,
    relative_pointer: &str,
    replacement: Value,
) {
    replace_node_field_in_graph_and_projection(
        package,
        node_index,
        &format!("/nominal_identity_preimage{relative_pointer}"),
        replacement,
    );
}

fn apply_authored_patch(candidate: &mut Value, patch: &Value) {
    match patch["op"].as_str().expect("patch operation is a string") {
        "replace" => {
            let path = patch["path"].as_str().expect("replace path is a string");
            *candidate
                .pointer_mut(path)
                .expect("authored replace pointer exists") = patch["value"].clone();
        }
        "move" => {
            let from = patch["from"].as_str().expect("move source is a string");
            let path = patch["path"].as_str().expect("move target is a string");
            let (source_parent, source_index) = from
                .rsplit_once('/')
                .expect("move source contains an array index");
            let moved = candidate
                .pointer_mut(source_parent)
                .and_then(Value::as_array_mut)
                .expect("move source parent is an array")
                .remove(source_index.parse().expect("move source index is numeric"));
            let (target_parent, target_index) = path
                .rsplit_once('/')
                .expect("move target contains an array index");
            candidate
                .pointer_mut(target_parent)
                .and_then(Value::as_array_mut)
                .expect("move target parent is an array")
                .insert(
                    target_index.parse().expect("move target index is numeric"),
                    moved,
                );
        }
        "remove" => {
            let path = patch["path"].as_str().expect("remove path is a string");
            let (parent, member) = path
                .rsplit_once('/')
                .expect("remove path contains an object member");
            candidate
                .pointer_mut(parent)
                .and_then(Value::as_object_mut)
                .expect("remove parent is an object")
                .remove(member)
                .expect("authored remove member exists");
        }
        operation => panic!("unsupported authored patch operation: {operation}"),
    }
}

/// Exact `(name, refused_by)` inventory of the `semantic` mutations in an
/// authored mutation list; every other kind must not name a refusing check.
fn refusing_check_inventory(mutations: &Value) -> BTreeSet<(&str, &str)> {
    mutations
        .as_array()
        .expect("authored mutations are an array")
        .iter()
        .filter_map(|mutation| {
            let name = mutation["name"]
                .as_str()
                .expect("mutation name is a string");
            if mutation["kind"].as_str() == Some("semantic") {
                let check = mutation["refused_by"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{name} names its refusing check"));
                Some((name, check))
            } else {
                assert!(
                    mutation.get("refused_by").is_none(),
                    "{name} is not a semantic mutation and names no refusing check"
                );
                None
            }
        })
        .collect()
}

/// Require a `semantic` mutation to be refused by exactly the named check:
/// `schema` refuses the closed shape, while `semantic` passes the schema and
/// fails only the published semantic constraint.
fn assert_refused_by_named_check(
    name: &str,
    refused_by: &Value,
    structurally_valid: bool,
    semantically_valid: bool,
) {
    match refused_by.as_str() {
        Some("schema") => assert!(
            !structurally_valid,
            "{name} must be refused by the closed schema"
        ),
        Some("semantic") => {
            assert!(structurally_valid, "{name} must pass the closed schema");
            assert!(
                !semantically_valid,
                "{name} must be refused by the semantic constraint"
            );
        }
        other => panic!("{name} names an unknown refusing check: {other:?}"),
    }
}

/// Exact `(name, kind)` inventory of an authored vector list, read through JSON pointers.
fn named_inventory<'a>(
    entries: &'a Value,
    name_pointer: &str,
    kind_pointer: &str,
) -> BTreeSet<(&'a str, &'a str)> {
    entries
        .as_array()
        .expect("authored inventory is an array")
        .iter()
        .map(|entry| {
            (
                entry
                    .pointer(name_pointer)
                    .and_then(Value::as_str)
                    .expect("inventory name is a string"),
                entry
                    .pointer(kind_pointer)
                    .and_then(Value::as_str)
                    .expect("inventory kind is a string"),
            )
        })
        .collect()
}

/// Ordered string members of an authored array, duplicates retained.
fn string_list<'a>(value: &'a Value, what: &str) -> Vec<&'a str> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{what} is an array"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("{what} member is a string"))
        })
        .collect()
}

fn string_set<'a>(value: &'a Value, what: &str) -> BTreeSet<&'a str> {
    string_list(value, what).into_iter().collect()
}

/// Closed package-selection refusal vocabulary of the complete-value lock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectionRefusal {
    UnknownTrigger,
    DuplicateTrigger,
    UnknownRole,
    DuplicateRole,
    RequiredMissing,
    AlternativeConflict,
    TriggerUnsatisfied,
    UntriggeredProfile,
}

impl SelectionRefusal {
    const ALL: [Self; 8] = [
        Self::UnknownTrigger,
        Self::DuplicateTrigger,
        Self::UnknownRole,
        Self::DuplicateRole,
        Self::RequiredMissing,
        Self::AlternativeConflict,
        Self::TriggerUnsatisfied,
        Self::UntriggeredProfile,
    ];

    fn code(self) -> &'static str {
        match self {
            Self::UnknownTrigger => "selection_unknown_trigger",
            Self::DuplicateTrigger => "selection_duplicate_trigger",
            Self::UnknownRole => "selection_unknown_role",
            Self::DuplicateRole => "selection_duplicate_role",
            Self::RequiredMissing => "selection_required_missing",
            Self::AlternativeConflict => "selection_alternative_conflict",
            Self::TriggerUnsatisfied => "selection_trigger_unsatisfied",
            Self::UntriggeredProfile => "selection_untriggered_profile",
        }
    }
}

/// Per-package selection slots parsed from the complete-value lock.
struct PackageSelectionRules<'a> {
    trigger_vocabulary: BTreeSet<&'a str>,
    always: Vec<&'a str>,
    conditional: Vec<(&'a str, &'a str)>,
    exactly_one: Vec<(&'a str, &'a str, Vec<&'a str>)>,
}

impl<'a> PackageSelectionRules<'a> {
    fn from_lock(lock: &'a Value) -> Self {
        let selection = &lock["package_selection"];
        let text = |value: &'a Value, what: &str| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("{what} is a string"))
        };
        Self {
            trigger_vocabulary: string_set(&lock["trigger_vocabulary"], "trigger vocabulary"),
            always: string_list(&selection["always"], "always roles"),
            conditional: selection["conditional"]
                .as_array()
                .expect("conditional slots are an array")
                .iter()
                .map(|slot| {
                    (
                        text(&slot["trigger"], "conditional trigger"),
                        text(&slot["role"], "conditional role"),
                    )
                })
                .collect(),
            exactly_one: selection["exactly_one"]
                .as_array()
                .expect("exactly-one slots are an array")
                .iter()
                .map(|slot| {
                    (
                        text(&slot["trigger"], "alternative trigger"),
                        text(&slot["selector"], "alternative selector"),
                        string_list(&slot["roles"], "alternative roles"),
                    )
                })
                .collect(),
        }
    }

    /// Every selection-slot role occurrence, in slot order.
    fn slot_roles(&self) -> Vec<&'a str> {
        self.always
            .iter()
            .copied()
            .chain(self.conditional.iter().map(|(_, role)| *role))
            .chain(
                self.exactly_one
                    .iter()
                    .flat_map(|(_, _, roles)| roles.iter().copied()),
            )
            .collect()
    }

    /// Independent package-selection oracle; checks run in the normative order.
    fn check(&self, triggers: &[&str], roles: &[&str]) -> Result<(), SelectionRefusal> {
        if triggers
            .iter()
            .any(|trigger| !self.trigger_vocabulary.contains(trigger))
        {
            return Err(SelectionRefusal::UnknownTrigger);
        }
        let mut named_triggers = BTreeSet::new();
        if !triggers
            .iter()
            .all(|trigger| named_triggers.insert(*trigger))
        {
            return Err(SelectionRefusal::DuplicateTrigger);
        }
        let known = self.slot_roles();
        if roles.iter().any(|role| !known.contains(role)) {
            return Err(SelectionRefusal::UnknownRole);
        }
        let mut selected = BTreeSet::new();
        if !roles.iter().all(|role| selected.insert(*role)) {
            return Err(SelectionRefusal::DuplicateRole);
        }
        if !self.always.iter().all(|role| selected.contains(role)) {
            return Err(SelectionRefusal::RequiredMissing);
        }
        let chosen_alternatives = |alternatives: &[&str]| {
            alternatives
                .iter()
                .filter(|role| selected.contains(*role))
                .count()
        };
        if self
            .exactly_one
            .iter()
            .any(|(_, _, alternatives)| chosen_alternatives(alternatives) > 1)
        {
            return Err(SelectionRefusal::AlternativeConflict);
        }
        let triggered = |trigger: &str| triggers.contains(&trigger);
        let chosen = self
            .conditional
            .iter()
            .map(|(trigger, role)| (triggered(trigger), selected.contains(role)))
            .chain(self.exactly_one.iter().map(|(trigger, _, alternatives)| {
                (triggered(trigger), chosen_alternatives(alternatives) == 1)
            }));
        for pair in chosen {
            match pair {
                (true, false) => return Err(SelectionRefusal::TriggerUnsatisfied),
                (false, true) => return Err(SelectionRefusal::UntriggeredProfile),
                (true, true) | (false, false) => {}
            }
        }
        Ok(())
    }
}

/// Closed V1-to-V2 migration refusal vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MigrationRefusalCode {
    InputMissing,
    InputStale,
    InputAmbiguous,
    TargetIncompatible,
    ByteOnlyReconstruction,
    OperationUnresolved,
}

impl MigrationRefusalCode {
    const ALL: [Self; 6] = [
        Self::InputMissing,
        Self::InputStale,
        Self::InputAmbiguous,
        Self::TargetIncompatible,
        Self::ByteOnlyReconstruction,
        Self::OperationUnresolved,
    ];

    fn code(self) -> &'static str {
        match self {
            Self::InputMissing => "migration_input_missing",
            Self::InputStale => "migration_input_stale",
            Self::InputAmbiguous => "migration_input_ambiguous",
            Self::TargetIncompatible => "migration_target_incompatible",
            Self::ByteOnlyReconstruction => "migration_byte_only_reconstruction",
            Self::OperationUnresolved => "migration_operation_unresolved",
        }
    }

    fn from_code(code: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.code() == code)
    }
}

/// A typed refusal paired with its closed `MigrationRefusalSubject` document.
type MigrationRefusal = (MigrationRefusalCode, Value);

fn source_subject(source: &Value) -> Value {
    json!({"input": "sources", "authority": source["authority"], "identity": source["identity"]})
}

/// Independent migration oracle over a candidate correspondence, the selected
/// V1 source package and the relinked V2 package. Every check of one class
/// runs before any check of the next, in the normative order: byte-only,
/// missing (an absent member, an empty source list or any V1-locked source
/// absent), ambiguous, stale, unresolved operation identity, then target
/// incompatibility.
fn classify_migration(
    correspondence: &Value,
    source: &Value,
    target: &Value,
) -> Result<(), MigrationRefusal> {
    let Some(inputs) = correspondence.get("reconstruction_inputs") else {
        return Err((
            MigrationRefusalCode::ByteOnlyReconstruction,
            json!({"input": "reconstruction_inputs"}),
        ));
    };
    if let Some(field) = RECONSTRUCTION_FIELDS
        .into_iter()
        .find(|field| inputs.get(field).is_none())
    {
        return Err((MigrationRefusalCode::InputMissing, json!({"input": field})));
    }
    let supplied = inputs["sources"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    if supplied.is_empty() {
        return Err((
            MigrationRefusalCode::InputMissing,
            json!({"input": "sources"}),
        ));
    }
    let locked = source["lock"]["sources"]
        .as_array()
        .expect("V1 lock sources are an array");
    let same_subject = |left: &Value, right: &Value| {
        left["authority"] == right["authority"] && left["identity"] == right["identity"]
    };
    if let Some(absent) = locked.iter().find(|wanted| {
        !supplied
            .iter()
            .any(|candidate| same_subject(candidate, wanted))
    }) {
        return Err((MigrationRefusalCode::InputMissing, source_subject(absent)));
    }
    if let Some(duplicate) = supplied.iter().enumerate().find_map(|(index, candidate)| {
        supplied[..index]
            .iter()
            .any(|earlier| same_subject(earlier, candidate))
            .then_some(candidate)
    }) {
        return Err((
            MigrationRefusalCode::InputAmbiguous,
            source_subject(duplicate),
        ));
    }
    if let Some(stale) = supplied
        .iter()
        .find(|candidate| !locked.contains(candidate))
    {
        return Err((MigrationRefusalCode::InputStale, source_subject(stale)));
    }
    if let Some(field) = RECONSTRUCTION_FIELDS[1..]
        .iter()
        .find(|field| inputs[**field] != source["lock"][**field])
    {
        return Err((MigrationRefusalCode::InputStale, json!({"input": field})));
    }
    if let Some(ordinal) = first_unresolved_operation(source, target) {
        return Err((
            MigrationRefusalCode::OperationUnresolved,
            json!({"input": "node_correspondences", "ordinal": ordinal}),
        ));
    }
    if correspondence["target_package_id"] != target["package_id"] {
        return Err((
            MigrationRefusalCode::TargetIncompatible,
            json!({"input": "target_package_id"}),
        ));
    }
    Ok(())
}

/// Total ordinal correspondence whose basis is `relinked-identical` exactly
/// when the source and target node digests are equal.
fn correspondence_is_consistent(correspondence: &Value, source: &Value, target: &Value) -> bool {
    let (Some(rows), Some(source_nodes), Some(target_nodes)) = (
        correspondence["node_correspondences"].as_array(),
        source["semantic_graph"]["nodes"].as_array(),
        target["semantic_graph"]["nodes"].as_array(),
    ) else {
        return false;
    };
    correspondence["source_package_id"] == source["package_id"]
        && correspondence["target_package_id"] == target["package_id"]
        && rows.len() == source_nodes.len()
        && rows.len() == target_nodes.len()
        && rows
            .iter()
            .zip(source_nodes.iter().zip(target_nodes))
            .enumerate()
            .all(|(ordinal, (row, (source_node, target_node)))| {
                let identical = source_node["node_id"] == target_node["node_id"];
                let basis = if identical {
                    "relinked-identical"
                } else {
                    "relinked-rekeyed"
                };
                row["ordinal"] == json!(ordinal)
                    && row["source_node_id"] == source_node["node_id"]
                    && row["target_node_id"] == target_node["node_id"]
                    && row["basis"].as_str() == Some(basis)
            })
}

fn migration_package(path: &str) -> Value {
    let bytes = match path {
        "../checked-package-v1/fixtures/positive-all-families.json" => POSITIVE_ALL_FAMILIES_V1,
        "fixtures/migration-source-nominal-v1.json" => MIGRATION_SOURCE_NOMINAL_V1,
        "fixtures/migration-source-clauses-v1.json" => MIGRATION_SOURCE_CLAUSES_V1,
        "fixtures/positive-clause-operations.json" => POSITIVE_CLAUSE_OPERATIONS,
        "fixtures/positive-all-families.json" => POSITIVE_ALL_FAMILIES,
        "fixtures/positive-nominal-identities.json" => POSITIVE_NOMINAL_IDENTITIES,
        unknown => panic!("migration vector names an unpublished package: {unknown}"),
    };
    parsed_fixture(bytes)
}

fn schema_for_def(schema: &Value, definition: &str) -> jsonschema::Validator {
    jsonschema::validator_for(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": format!("#/$defs/{definition}"),
        "$defs": schema["$defs"]
    }))
    .unwrap_or_else(|error| panic!("{definition} is a valid schema definition: {error}"))
}

/// Every application term of a semantic term, in pre-order through
/// application arguments, aggregate members and binding values.
fn application_terms(term: &Value) -> Vec<&Value> {
    let mut found = Vec::new();
    let mut pending = vec![term];
    while let Some(current) = pending.pop() {
        let children = match current["term"].as_str() {
            Some("application") => {
                found.push(current);
                current["arguments"].as_array()
            }
            Some("aggregate") => current["members"].as_array(),
            Some("binding") => {
                pending.push(&current["value"]);
                None
            }
            _ => None,
        };
        pending.extend(children.into_iter().flatten().rev());
    }
    found
}

/// Exact dependency list of an application-bearing node: every reference
/// target and every operation member declaration of its body, unique and
/// ascending by digest.
fn application_dependencies(term: &Value) -> Vec<Value> {
    let mut targets = BTreeMap::new();
    let mut pending = vec![term];
    while let Some(current) = pending.pop() {
        match current["term"].as_str() {
            Some("reference") => {
                if let Some(digest) = current["target"]["digest"].as_str() {
                    targets.insert(digest, &current["target"]);
                }
            }
            Some("application") => {
                let declaration = &current["operation"]["member"]["declaration"];
                if let Some(digest) = declaration["digest"].as_str() {
                    targets.insert(digest, declaration);
                }
                pending.extend(current["arguments"].as_array().into_iter().flatten());
            }
            Some("aggregate") => {
                pending.extend(current["members"].as_array().into_iter().flatten())
            }
            Some("binding") => pending.push(&current["value"]),
            _ => {}
        }
    }
    targets.into_values().cloned().collect()
}

/// The operation exactly derivable from a V1 application class. V1 carries no
/// law, mode, member or leaf, so only the classes with one catalogued meaning
/// and no such selection derive; every other class (`call`, `unary`, `binary`,
/// `quantify`, `collection`, `query`, `convert`, `reaches`) is unresolved.
fn derived_v1_operation(class: &str) -> Option<Value> {
    let identity = match class {
        "conditional" => "quire.op.control.if",
        "let" => "quire.op.control.let",
        "pre" => "quire.op.state.pre",
        "present" => "quire.op.option.present",
        "value" => "quire.op.option.value",
        "deref" => "quire.op.model.deref",
        "temporal" => "quire.op.temporal.clause",
        "protocol_control" => "quire.op.protocol.control",
        "state_transition" => "quire.op.state.transition",
        "claim" => "quire.op.claim.clause",
        _ => return None,
    };
    Some(json!({"identity": identity, "laws": [], "mode": null, "member": null, "leaves": []}))
}

/// V1 migration: the ordinal of the first target node whose applications do
/// not pair, positionally in pre-order, with V1 applications of the same class
/// whose derived operation equals the target operation exactly.
fn first_unresolved_operation(source: &Value, target: &Value) -> Option<usize> {
    let source_nodes = source["semantic_graph"]["nodes"].as_array()?;
    let target_nodes = target["semantic_graph"]["nodes"].as_array()?;
    target_nodes
        .iter()
        .enumerate()
        .find_map(|(ordinal, target_node)| {
            let target_terms = application_terms(&target_node["body"]);
            let source_terms = source_nodes
                .get(ordinal)
                .map(|node| application_terms(&node["body"]))
                .unwrap_or_default();
            let resolved = target_terms.len() == source_terms.len()
                && target_terms
                    .iter()
                    .zip(&source_terms)
                    .all(|(target_term, source_term)| {
                        target_term["operator"] == source_term["operator"]
                            && source_term["operator"]
                                .as_str()
                                .and_then(derived_v1_operation)
                                .is_some_and(|derived| target_term["operation"] == derived)
                    });
            (!resolved).then_some(ordinal)
        })
}

/// Closed reader refusal causes for an application's operation, in evaluation
/// order. The first eight are `invalid_package` causes and the last is the
/// `ill_typed` cause of `quire.native.diagnostics/v1` revision `1-draft.4`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OperationRefusal {
    UnknownOperation,
    ClassMismatch,
    LawMissing,
    LawMismatch,
    LawUnselected,
    ModeMismatch,
    MemberMismatch,
    ModeTypeMismatch,
    OperatorIneligible,
}

impl OperationRefusal {
    const ALL: [Self; 9] = [
        Self::UnknownOperation,
        Self::ClassMismatch,
        Self::LawMissing,
        Self::LawMismatch,
        Self::LawUnselected,
        Self::ModeMismatch,
        Self::MemberMismatch,
        Self::ModeTypeMismatch,
        Self::OperatorIneligible,
    ];

    fn cause(self) -> &'static str {
        match self {
            Self::UnknownOperation => "unknown-operation",
            Self::ClassMismatch => "operation-class-mismatch",
            Self::LawMissing => "operation-law-missing",
            Self::LawMismatch => "operation-law-mismatch",
            Self::LawUnselected => "operation-law-unselected",
            Self::ModeMismatch => "operation-mode-mismatch",
            Self::MemberMismatch => "operation-member-mismatch",
            Self::ModeTypeMismatch => "operation-mode-type-mismatch",
            Self::OperatorIneligible => "operator-ineligible",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::OperatorIneligible => "ill_typed",
            Self::UnknownOperation
            | Self::ClassMismatch
            | Self::LawMissing
            | Self::LawMismatch
            | Self::LawUnselected
            | Self::ModeMismatch
            | Self::MemberMismatch
            | Self::ModeTypeMismatch => "invalid_package",
        }
    }

    fn from_cause(cause: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.cause() == cause)
    }
}

/// Static meaning of one term: its catalogued family, the package type node
/// naming it when there is one, and for a binder the descriptor of its value.
#[derive(Clone, Debug, PartialEq)]
struct Descriptor {
    family: String,
    ty: Option<Value>,
    bound: Option<Box<Descriptor>>,
}

impl Descriptor {
    fn family(family: &str) -> Self {
        Self {
            family: family.to_owned(),
            ty: None,
            bound: None,
        }
    }
}

/// Declared bounds of an exact numeric type, as read by the FR-149 table.
#[derive(Clone, Debug, PartialEq)]
enum NumericBounds {
    Integer,
    Int(BigInt, BigInt),
    Rational(Option<[BigInt; 4]>),
    Decimal(Option<[BigInt; 4]>),
}

/// Expression semantic form required for an application operator class.
fn expression_form_for_class(class: &str) -> Option<&'static str> {
    Some(match class {
        "call" => "call",
        "unary" => "unary",
        "binary" => "binary",
        "conditional" => "conditional",
        "let" => "let",
        "quantify" => "quantify",
        "collection" => "collection",
        "query" => "query",
        "convert" => "conversion",
        "pre" => "pre_read",
        "present" => "presence_read",
        "value" => "value_read",
        "deref" => "deref",
        "reaches" => "reachability",
        _ => return None,
    })
}

/// Independent reader oracle over the operation catalog, a package lock and
/// its graph. Nothing is read from outside the package and catalog.
struct OperationOracle<'a> {
    catalog: &'a Value,
    lock: &'a Value,
    nodes: &'a [Value],
}

impl<'a> OperationOracle<'a> {
    fn for_package(catalog: &'a Value, package: &'a Value) -> Self {
        Self {
            catalog,
            lock: &package["lock"],
            nodes: package["semantic_graph"]["nodes"]
                .as_array()
                .expect("package nodes are an array"),
        }
    }

    fn node(&self, id: &Value) -> Option<&'a Value> {
        self.nodes.iter().find(|node| node["node_id"] == *id)
    }

    /// Families named by a catalog position: a group's members or the family itself.
    fn expand(&self, name: &str) -> Vec<&'a str> {
        match self.catalog["groups"][name].as_array() {
            Some(members) => members.iter().filter_map(Value::as_str).collect(),
            None => vec![],
        }
        .into_iter()
        .chain(
            string_list(&self.catalog["families"], "catalog families")
                .into_iter()
                .filter(|family| *family == name),
        )
        .collect()
    }

    /// Whether an alias or (non-population) bounded domain continues through its semantic type.
    fn refines(node: &Value) -> bool {
        let forwarding = matches!(
            (node["node_tag"].as_str(), node["semantic_form"].as_str()),
            (Some("composite_type"), Some("alias"))
        ) || (node["node_tag"].as_str() == Some("bounded_domain")
            && node["semantic_form"].as_str() != Some("model_population"));
        forwarding && node["semantic_type"] != node["node_id"]
    }

    /// The alias/domain chain from a type node to its structural type, bounded by the graph size.
    fn chain(&self, id: &Value) -> Option<Vec<&'a Value>> {
        let mut chain = vec![self.node(id)?];
        while let Some(last) = chain.last().copied().filter(|node| Self::refines(node)) {
            if chain.len() > self.nodes.len() {
                return None;
            }
            chain.push(self.node(&last["semantic_type"])?);
        }
        Some(chain)
    }

    fn resolve(&self, id: &Value) -> Option<&'a Value> {
        self.chain(id)?.last().copied()
    }

    fn same_type(&self, left: &Value, right: &Value) -> bool {
        matches!((self.resolve(left), self.resolve(right)), (Some(l), Some(r)) if l["node_id"] == r["node_id"])
    }

    fn type_family(&self, id: &Value) -> Option<&'static str> {
        let node = self.resolve(id)?;
        Some(
            match (node["node_tag"].as_str()?, node["semantic_form"].as_str()?) {
                ("scalar_type", "boolean") => "boolean",
                ("scalar_type", "integer") => "integer",
                ("scalar_type", "rational") => "rational",
                ("scalar_type", "decimal") => "decimal",
                ("scalar_type", "float32") => "float32",
                ("scalar_type", "float64") => "float64",
                ("scalar_type", "text") => "text",
                ("scalar_type", "unit") => "quantity",
                ("scalar_type", "enum") => {
                    if node["nominal_identity_preimage"]["ordered"].as_bool() == Some(true) {
                        "ordered_enum"
                    } else {
                        "enum"
                    }
                }
                ("composite_type", "option") => "option",
                ("composite_type", "record") => "record",
                ("composite_type", "tuple") => "tuple",
                ("composite_type", "sequence") => "sequence",
                ("composite_type", "set") => "set",
                ("composite_type", "bag") => "bag",
                ("composite_type", "ordered_set") => "ordered_set",
                ("composite_type", "reference") => "reference",
                ("bounded_domain", "model_population") => "population",
                _ => return None,
            },
        )
    }

    fn typed(&self, id: &Value) -> Option<Descriptor> {
        Some(Descriptor {
            family: self.type_family(id)?.to_owned(),
            ty: Some(id.clone()),
            bound: None,
        })
    }

    /// Value of the binding named `name` in a node body aggregate.
    fn binding<'n>(node: &'n Value, name: &str) -> Option<&'n Value> {
        node["body"]["members"]
            .as_array()?
            .iter()
            .find(|member| member["term"] == "binding" && member["name"] == name)
            .map(|member| &member["value"])
    }

    fn reference_target(term: &Value) -> Option<&Value> {
        (term["term"] == "reference").then_some(&term["target"])
    }

    /// Inner type of an option, collection or reference type.
    fn inner_type(&self, id: &Value) -> Option<&'a Value> {
        let node = self.resolve(id)?;
        matches!(
            self.type_family(id)?,
            "option" | "sequence" | "set" | "bag" | "ordered_set" | "reference"
        )
        .then(|| Self::reference_target(&node["body"]["members"][0]))
        .flatten()
    }

    /// Mode value pinned by a descriptor's type for a type-pinned mode kind:
    /// the first binding of that kind on the alias/domain chain, else `exact`
    /// for rounding and nothing for a text profile.
    fn pin(&self, descriptor: &Descriptor, kind: &str) -> Option<String> {
        let applies = self.catalog["type_pinned_modes"][kind]
            .as_array()?
            .iter()
            .any(|family| family.as_str() == Some(descriptor.family.as_str()));
        let ty = descriptor.ty.as_ref().filter(|_| applies)?;
        let chain = self.chain(ty)?;
        chain
            .iter()
            .find_map(|node| Self::binding(node, kind)?["value"].as_str())
            .map(str::to_owned)
            .or_else(|| (kind == "rounding").then(|| "exact".to_owned()))
    }

    fn descriptor(&self, term: &Value) -> Option<Descriptor> {
        match term["term"].as_str()? {
            "literal" => Some(Descriptor::family(match term["value_kind"].as_str()? {
                "boolean" => "boolean",
                "integer" => "integer",
                "rational" => "rational",
                "decimal" => "decimal",
                "float32_bits" => "float32",
                "float64_bits" => "float64",
                "text" => "text",
                "enum" => "enum",
                "none" => "option",
                _ => return None,
            })),
            "reference" => {
                let target = self.node(&term["target"])?;
                match target["node_tag"].as_str()? {
                    "function" => Some(Descriptor {
                        family: "function".to_owned(),
                        ty: Some(target["semantic_type"].clone()),
                        bound: None,
                    }),
                    "relation" => Some(Descriptor {
                        family: "population".to_owned(),
                        ty: Some(target["node_id"].clone()),
                        bound: None,
                    }),
                    _ => self.typed(&target["semantic_type"]),
                }
            }
            "application" => self.classify(term, None).ok(),
            "binding" => Some(Descriptor {
                family: "binder".to_owned(),
                ty: None,
                bound: Some(Box::new(self.descriptor(&term["value"])?)),
            }),
            "aggregate" => Some(Descriptor::family("aggregate")),
            _ => None,
        }
    }

    /// Operand descriptor of one argument. A nested application (directly or
    /// as a binder value) is classified first and its refusal is reported in
    /// place of any refusal of the enclosing application that needs its type.
    fn argument_descriptor(&self, term: &Value) -> Result<Option<Descriptor>, OperationRefusal> {
        match term["term"].as_str() {
            Some("application") => self.classify(term, None).map(Some),
            Some("binding") => {
                Ok(self
                    .argument_descriptor(&term["value"])?
                    .map(|bound| Descriptor {
                        family: "binder".to_owned(),
                        ty: None,
                        bound: Some(Box::new(bound)),
                    }))
            }
            _ => Ok(self.descriptor(term)),
        }
    }

    /// Result descriptor of a member selection, resolved against its declaration.
    fn member_descriptor(&self, member: &Value) -> Option<Descriptor> {
        let declaration = &member["declaration"];
        match member["kind"].as_str()? {
            "field" => {
                let record = self.resolve(declaration)?;
                (self.type_family(declaration)? == "record").then_some(())?;
                self.typed(Self::reference_target(Self::binding(
                    record,
                    member["name"].as_str()?,
                )?)?)
            }
            "position" => {
                let tuple = self.resolve(declaration)?;
                (self.type_family(declaration)? == "tuple").then_some(())?;
                let position = usize::try_from(member["position"].as_u64()?).ok()?;
                self.typed(Self::reference_target(
                    tuple["body"]["members"].as_array()?.get(position)?,
                )?)
            }
            "element" => {
                let family = self.type_family(declaration)?;
                self.expand("collection_kind")
                    .contains(&family)
                    .then(|| self.typed(self.inner_type(declaration)?))
                    .flatten()
            }
            "relationship_end" => {
                let relationship = self.node(declaration)?;
                (relationship["node_tag"] == "relation"
                    && relationship["semantic_form"] == "relationship")
                    .then(|| {
                        self.typed(Self::reference_target(Self::binding(
                            relationship,
                            member["name"].as_str()?,
                        )?)?)
                    })
                    .flatten()
            }
            "operation" => {
                let model = self.node(declaration)?;
                (model["node_tag"] == "model" && model["semantic_form"] == "model_type")
                    .then(|| {
                        let function = self.node(Self::reference_target(Self::binding(
                            model,
                            member["name"].as_str()?,
                        )?)?)?;
                        (function["node_tag"] == "function")
                            .then(|| self.typed(&function["semantic_type"]))
                            .flatten()
                    })
                    .flatten()
            }
            "type_argument" => self.typed(declaration),
            _ => None,
        }
    }

    fn result_descriptor(
        &self,
        form: &str,
        operation: &Value,
        arguments: &[Descriptor],
    ) -> Option<Descriptor> {
        let indexed = |prefix: &str| {
            form.strip_prefix(prefix)
                .and_then(|index| index.parse::<usize>().ok())
                .and_then(|index| arguments.get(index))
        };
        if let Some(argument) = indexed("operand:") {
            return Some(argument.clone());
        }
        if let Some(argument) = indexed("kind:") {
            return Some(Descriptor::family(&argument.family));
        }
        if let Some(argument) = indexed("inner:") {
            return self.typed(self.inner_type(argument.ty.as_ref()?)?);
        }
        if let Some(argument) = indexed("return:") {
            return (argument.family == "function")
                .then(|| self.typed(argument.ty.as_ref()?))
                .flatten();
        }
        match form {
            "member" => self.member_descriptor(&operation["member"]),
            "absence" => Some(Descriptor::family(
                if operation["mode"]["value"] == "empty" {
                    "option"
                } else {
                    "reference"
                },
            )),
            family if !self.expand(family).is_empty() && !form.contains(':') => {
                Some(Descriptor::family(family))
            }
            _ => None,
        }
    }

    fn numeric_bounds(&self, id: &Value) -> Option<NumericBounds> {
        let bound = |node: &Value, name: &str| integer(Self::binding(node, name)?.get("value")?);
        for node in self.chain(id)? {
            let bounds = match (node["node_tag"].as_str()?, node["semantic_form"].as_str()?) {
                ("bounded_domain", "integer_range") => {
                    NumericBounds::Int(bound(node, "min")?, bound(node, "max")?)
                }
                ("bounded_domain", "rational_range") => NumericBounds::Rational(Some([
                    bound(node, "numerator_min")?,
                    bound(node, "numerator_max")?,
                    bound(node, "denominator_min")?,
                    bound(node, "denominator_max")?,
                ])),
                ("bounded_domain", "decimal_range") => NumericBounds::Decimal(Some([
                    bound(node, "coefficient_min")?,
                    bound(node, "coefficient_max")?,
                    bound(node, "scale_min")?,
                    bound(node, "scale_max")?,
                ])),
                ("scalar_type", "integer") => NumericBounds::Integer,
                ("scalar_type", "rational") => NumericBounds::Rational(None),
                ("scalar_type", "decimal") => NumericBounds::Decimal(None),
                _ => continue,
            };
            return Some(bounds);
        }
        None
    }

    /// FR-149 equality-conversion table, decided from declared bounds alone.
    fn exact_conversion(&self, source: &Value, target: &Value) -> bool {
        if source == target {
            return true;
        }
        let (Some(from), Some(to)) = (self.numeric_bounds(source), self.numeric_bounds(target))
        else {
            return false;
        };
        exact_bounds_conversion(&from, &to)
    }

    /// Text leaves of a compared type in declaration pre-order, each with the
    /// text profile its type pins; `None` when a leaf is IEEE, a text leaf pins
    /// no profile, or the type is cyclic.
    fn expected_leaves(&self, id: &Value) -> Option<Vec<(Vec<String>, String)>> {
        let mut leaves = Vec::new();
        self.collect_leaves(id, &mut Vec::new(), &mut Vec::new(), &mut leaves)?;
        Some(leaves)
    }

    fn collect_leaves(
        &self,
        id: &Value,
        path: &mut Vec<String>,
        visiting: &mut Vec<Value>,
        leaves: &mut Vec<(Vec<String>, String)>,
    ) -> Option<()> {
        let node = self.resolve(id)?;
        if visiting.contains(&node["node_id"]) {
            return None;
        }
        let family = self.type_family(id)?;
        let children: Vec<(String, &Value)> = match family {
            "text" => {
                leaves.push((path.clone(), self.pin(&self.typed(id)?, "text_profile")?));
                return Some(());
            }
            "float32" | "float64" => return None,
            "record" => node["body"]["members"]
                .as_array()?
                .iter()
                .map(|member| {
                    Some((
                        format!("field:{}", member["name"].as_str()?),
                        Self::reference_target(&member["value"])?,
                    ))
                })
                .collect::<Option<_>>()?,
            "tuple" => node["body"]["members"]
                .as_array()?
                .iter()
                .enumerate()
                .map(|(index, member)| {
                    Some((format!("position:{index}"), Self::reference_target(member)?))
                })
                .collect::<Option<_>>()?,
            "option" | "sequence" | "set" | "bag" | "ordered_set" => {
                vec![("inner".to_owned(), self.inner_type(id)?)]
            }
            _ => return Some(()),
        };
        visiting.push(node["node_id"].clone());
        for (segment, child) in children {
            path.push(segment);
            self.collect_leaves(child, path, visiting, leaves)?;
            path.pop();
        }
        visiting.pop();
        Some(())
    }

    fn law_refs(&self, role: &str) -> &'a [Value] {
        self.catalog["law_roles"][role]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    fn selected(&self, definition: &Value) -> bool {
        self.lock["definition_selections"]
            .as_array()
            .is_some_and(|selections| selections.contains(definition))
    }

    fn mode_value_admitted(&self, mode: &Value, kind: &str) -> bool {
        mode["kind"] == kind
            && self.catalog["modes"][kind]
                .as_array()
                .is_some_and(|values| values.contains(&mode["value"]))
    }

    fn constraint_holds(
        &self,
        constraint: &Value,
        operation: &Value,
        arguments: &[Descriptor],
        rest_start: usize,
    ) -> bool {
        let operand = |slot: usize| {
            constraint["operands"][slot]
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| arguments.get(index))
        };
        let member = || self.member_descriptor(&operation["member"]);
        let in_family = |family: &str| {
            constraint["family"]
                .as_str()
                .is_some_and(|name| self.expand(name).contains(&family))
        };
        let ty = |descriptor: Option<&Descriptor>| descriptor.and_then(|d| d.ty.clone());
        match constraint["kind"].as_str() {
            Some("same_family") => {
                matches!((operand(0), operand(1)), (Some(l), Some(r)) if l.family == r.family)
            }
            Some("same_type") => {
                matches!((ty(operand(0)), ty(operand(1))), (Some(l), Some(r)) if self.same_type(&l, &r))
            }
            Some("same_dimension") => {
                let dimension = |id: Option<Value>| {
                    let unit = self.resolve(&id?)?;
                    let dimension = &unit["nominal_identity_preimage"]["dimension_node_id"];
                    (unit["semantic_form"] == "unit" && dimension.is_object())
                        .then(|| dimension.clone())
                };
                matches!((dimension(ty(operand(0))), dimension(member().and_then(|m| m.ty))), (Some(l), Some(r)) if l == r)
            }
            Some("inner_type") => {
                let (Some(collection), Some(element)) = (operand(0), operand(1)) else {
                    return false;
                };
                let Some(inner) = collection.ty.as_ref().and_then(|id| self.inner_type(id)) else {
                    return false;
                };
                self.type_family(inner) == Some(element.family.as_str())
                    && element
                        .ty
                        .as_ref()
                        .is_none_or(|id| self.same_type(inner, id))
            }
            Some("member_of") => {
                matches!(ty(operand(0)), Some(id) if self.same_type(&id, &operation["member"]["declaration"]))
            }
            Some("member_family") => member().is_some_and(|m| in_family(&m.family)),
            Some("bound_family") => operand(0)
                .and_then(|binder| binder.bound.as_deref())
                .is_some_and(|bound| in_family(&bound.family)),
            Some("bound_is_member") => {
                let bound = operand(0)
                    .and_then(|binder| binder.bound.as_deref())
                    .and_then(|bound| bound.ty.clone());
                matches!((bound, member().and_then(|m| m.ty)), (Some(l), Some(r)) if self.same_type(&l, &r))
            }
            Some("exact_conversion") => {
                matches!((ty(operand(0)), member().and_then(|m| m.ty)), (Some(s), Some(t)) if self.exact_conversion(&s, &t))
            }
            Some("lossy_conversion") => {
                matches!((ty(operand(0)), member().and_then(|m| m.ty)), (Some(s), Some(t)) if !self.exact_conversion(&s, &t))
            }
            Some("uniform_rest") => arguments[rest_start.min(arguments.len())..]
                .windows(2)
                .all(|pair| pair[0].family == pair[1].family),
            _ => false,
        }
    }

    /// Classify one application in the normative order and return its result
    /// descriptor. `root_type` is the semantic type of an expression or
    /// function node whose body root is this application.
    fn classify(
        &self,
        term: &Value,
        root_type: Option<&Value>,
    ) -> Result<Descriptor, OperationRefusal> {
        use OperationRefusal as Refusal;
        let operation = &term["operation"];
        let entry = self.catalog["operations"]
            .as_array()
            .expect("catalog operations are an array")
            .iter()
            .find(|entry| entry["identity"] == operation["identity"])
            .ok_or(Refusal::UnknownOperation)?;
        if entry["operator"] != term["operator"] {
            return Err(Refusal::ClassMismatch);
        }
        let required = string_list(&entry["laws"], "catalog law roles");
        let laws = operation["laws"].as_array().ok_or(Refusal::LawMismatch)?;
        let supplied = laws
            .iter()
            .map(|law| law["role"].as_str().unwrap_or_default())
            .collect::<Vec<_>>();
        if required.iter().any(|role| !supplied.contains(role)) {
            return Err(Refusal::LawMissing);
        }
        let supplied_leaves = operation["leaves"].as_array().ok_or(Refusal::LawMismatch)?;
        if supplied != required
            || !laws.iter().all(|law| {
                self.law_refs(&law["role"].as_str().unwrap_or_default())
                    .contains(&law["definition"])
            })
            || (entry["leaves"].is_null() && !supplied_leaves.is_empty())
        {
            return Err(Refusal::LawMismatch);
        }
        if !laws.iter().all(|law| self.selected(&law["definition"])) {
            return Err(Refusal::LawUnselected);
        }
        let mode_fits = match (entry["mode"].as_str(), &operation["mode"]) {
            (None, Value::Null) => true,
            (Some(kind), mode) => self.mode_value_admitted(mode, kind),
            (None, _) => false,
        };
        if !mode_fits {
            return Err(Refusal::ModeMismatch);
        }
        let member = &operation["member"];
        let member_fits = match (entry["member"].as_str(), member) {
            (None, Value::Null) => true,
            (Some(kind), member) => member["kind"] == kind,
            (None, _) => false,
        };
        if !member_fits {
            return Err(Refusal::MemberMismatch);
        }

        let arguments = term["arguments"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|argument| self.argument_descriptor(argument))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Option<Vec<_>>>();
        let form = entry["result"]
            .as_str()
            .expect("catalog result form is a string");
        let result = arguments
            .as_deref()
            .and_then(|arguments| self.result_descriptor(form, operation, arguments));
        if let (Some(kind), Some(arguments), Some(result)) = (
            entry["mode"].as_str(),
            arguments.as_deref(),
            result.as_ref(),
        ) {
            if self.catalog["type_pinned_modes"].get(kind).is_some() {
                let result_source = match root_type {
                    Some(root) => self.typed(root),
                    None => Some(result.clone()),
                };
                let operand_sources = arguments
                    .iter()
                    .map(|argument| argument.bound.as_deref().unwrap_or(argument).clone());
                let sources = if entry["operator"] == "convert" {
                    vec![result_source]
                } else {
                    operand_sources.map(Some).chain([result_source]).collect()
                };
                if let Some(sources) = sources.into_iter().collect::<Option<Vec<_>>>() {
                    let pins = sources
                        .iter()
                        .filter_map(|source| self.pin(source, kind))
                        .collect::<BTreeSet<_>>();
                    let declared = operation["mode"]["value"].as_str().map(str::to_owned);
                    if pins.len() != 1 || pins.first() != declared.as_ref() {
                        return Err(Refusal::ModeTypeMismatch);
                    }
                }
            }
        }

        let arguments = arguments.ok_or(Refusal::OperatorIneligible)?;
        let positions = string_list(&entry["operands"], "catalog operand positions");
        let rest = entry["rest"].as_str();
        let arity_fits = arguments.len() >= positions.len()
            && (rest.is_some() || arguments.len() == positions.len());
        let families_fit = arguments.iter().enumerate().all(|(index, argument)| {
            positions
                .get(index)
                .copied()
                .or(rest)
                .is_some_and(|position| self.expand(position).contains(&argument.family.as_str()))
        });
        let member_resolves = entry["member"].is_null() || self.member_descriptor(member).is_some();
        let constraints_hold = entry["constraints"]
            .as_array()
            .expect("catalog constraints are an array")
            .iter()
            .all(|constraint| {
                self.constraint_holds(constraint, operation, &arguments, positions.len())
            });
        let result = result.ok_or(Refusal::OperatorIneligible)?;
        let root_fits = root_type.is_none_or(|root| {
            self.type_family(root) == Some(result.family.as_str())
                && result.ty.as_ref().is_none_or(|ty| self.same_type(ty, root))
        });
        let expected_leaves = match entry["leaves"].as_str() {
            None => Some(Vec::new()),
            Some(source) => {
                let compared = match source {
                    "operand:0" => arguments.first().and_then(|a| a.ty.clone()),
                    "inner:0" => arguments
                        .first()
                        .and_then(|a| a.ty.as_ref())
                        .and_then(|id| self.inner_type(id))
                        .cloned(),
                    _ => None,
                };
                compared.and_then(|id| self.expected_leaves(&id))
            }
        };
        let (true, true, true, true, true, Some(expected)) = (
            arity_fits,
            families_fit,
            member_resolves,
            constraints_hold,
            root_fits,
            expected_leaves,
        ) else {
            return Err(Refusal::OperatorIneligible);
        };

        if supplied_leaves.len() < expected.len() {
            return Err(Refusal::LawMissing);
        }
        let text_refs = self.law_refs("text_profile");
        let leaf_shape_fits = supplied_leaves.len() == expected.len()
            && supplied_leaves
                .iter()
                .zip(&expected)
                .all(|(leaf, (path, _))| {
                    leaf["path"].as_array().is_some_and(|segments| {
                        segments
                            .iter()
                            .map(Value::as_str)
                            .eq(path.iter().map(|s| Some(s.as_str())))
                    }) && leaf["laws"].as_array().is_some_and(|laws| {
                        laws.len() == 1
                            && laws[0]["role"] == "text_profile"
                            && text_refs.contains(&laws[0]["definition"])
                    })
                });
        if !leaf_shape_fits {
            return Err(Refusal::LawMismatch);
        }
        if !supplied_leaves
            .iter()
            .all(|leaf| self.selected(&leaf["laws"][0]["definition"]))
        {
            return Err(Refusal::LawUnselected);
        }
        if !supplied_leaves
            .iter()
            .all(|leaf| self.mode_value_admitted(&leaf["mode"], "text_profile"))
        {
            return Err(Refusal::ModeMismatch);
        }
        if !supplied_leaves
            .iter()
            .zip(&expected)
            .all(|(leaf, (_, pin))| leaf["mode"]["value"] == pin.as_str())
        {
            return Err(Refusal::ModeTypeMismatch);
        }
        Ok(result)
    }

    /// Classify an application at the body root of `node` (or nested when it is not the root).
    fn classify_in(&self, node: &Value, term: &Value) -> Result<Descriptor, OperationRefusal> {
        let at_root = std::ptr::eq(term, &node["body"])
            && matches!(node["node_tag"].as_str(), Some("expression" | "function"));
        self.classify(term, at_root.then_some(&node["semantic_type"]))
    }
}

/// FR-149 exact-conversion rows over declared numeric bounds.
fn exact_bounds_conversion(from: &NumericBounds, to: &NumericBounds) -> bool {
    use NumericBounds::{Decimal, Int, Integer, Rational};
    let zero = BigInt::from(0);
    let one = BigInt::from(1);
    let ten_pow = |scale: &BigInt| {
        u32::try_from(scale)
            .ok()
            .map(|scale| BigInt::from(10).pow(scale))
    };
    match (from, to) {
        (Int(..), Integer) => true,
        (Int(lo, hi), Int(lo2, hi2)) => lo2 <= lo && hi <= hi2,
        (Int(lo, hi), Rational(Some([n1, n2, d1, d2]))) => {
            n1 <= lo && hi <= n2 && d1 <= &one && &one <= d2
        }
        (Int(lo, hi), Decimal(Some([c1, c2, s1, _]))) => {
            ten_pow(s1).is_some_and(|factor| c1 <= &(lo * &factor) && &(hi * &factor) <= c2)
        }
        (Rational(Some([n1, n2, d1, d2])), Rational(Some([n3, n4, d3, d4]))) => {
            n3 <= n1 && n2 <= n4 && d3 <= d1 && d2 <= d4
        }
        (Rational(Some([n1, n2, _, d2])), Integer | Int(..) | Decimal(Some(_))) if d2 == &one => {
            exact_bounds_conversion(&Int(n1.clone(), n2.clone()), to)
        }
        (Decimal(Some([c1, c2, _, s2])), Rational(Some([n1, n2, d1, d2]))) => {
            n1 <= c1.min(&zero)
                && c2.max(&zero) <= n2
                && d1 <= &one
                && ten_pow(s2).is_some_and(|factor| &factor <= d2)
        }
        (Decimal(Some([c1, c2, s1, s2])), Decimal(Some([c3, c4, s3, s4]))) => {
            s3 <= s1
                && s2 <= s4
                && ((s3 == s1 && c3 <= c1 && c2 <= c4)
                    || (s3 < s1 && c3 <= c1.min(&zero) && c2.max(&zero) <= c4))
        }
        (Decimal(Some([c1, c2, s1, s2])), Integer | Int(..)) if s1 == &zero && s2 == &zero => {
            exact_bounds_conversion(&Int(c1.clone(), c2.clone()), to)
        }
        _ => false,
    }
}

/// The first refusal of a package's applications: nodes ascending by node-id
/// digest, then each node's applications in pre-order.
fn package_operations_refusal(package: &Value, catalog: &Value) -> Result<(), OperationRefusal> {
    let oracle = OperationOracle::for_package(catalog, package);
    let mut nodes = oracle.nodes.iter().collect::<Vec<_>>();
    nodes.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    nodes.into_iter().try_for_each(|node| {
        application_terms(&node["body"])
            .into_iter()
            .try_for_each(|term| oracle.classify_in(node, term).map(drop))
    })
}

/// Body of an application-node preimage: references to members of the node's
/// own recursion group become `group_reference` ordinals in graph order.
fn preimage_term(term: &Value, group: &[&Value]) -> Value {
    match term["term"].as_str() {
        Some("reference") => match group.iter().position(|id| **id == term["target"]) {
            Some(ordinal) => json!({"term": "group_reference", "ordinal": ordinal}),
            None => term.clone(),
        },
        Some("application") => {
            let mut mapped = term.clone();
            mapped["arguments"] = term["arguments"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|argument| preimage_term(argument, group))
                .collect();
            mapped
        }
        Some("aggregate") => {
            let mut mapped = term.clone();
            mapped["members"] = term["members"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|member| preimage_term(member, group))
                .collect();
            mapped
        }
        Some("binding") => {
            let mut mapped = term.clone();
            mapped["value"] = preimage_term(&term["value"], group);
            mapped
        }
        _ => term.clone(),
    }
}

/// The `quire.application-node/v1` preimage of any node whose body contains
/// an application.
fn application_node_preimage(node: &Value, nodes: &[Value]) -> Option<Value> {
    if application_terms(&node["body"]).is_empty() {
        return None;
    }
    let group = node.get("recursion_group").map_or_else(Vec::new, |label| {
        nodes
            .iter()
            .filter(|member| member.get("recursion_group") == Some(label))
            .map(|member| &member["node_id"])
            .collect()
    });
    let recursion = group.iter().position(|id| **id == node["node_id"]).map_or(
        Value::Null,
        |ordinal| json!({"size": group.len(), "ordinal": ordinal}),
    );
    Some(json!({
        "version": "quire.application-node/v1",
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": node["semantic_type"],
        "declaration": node.get("declaration").cloned().unwrap_or(Value::Null),
        "recursion": recursion,
        "body": preimage_term(&node["body"], &group),
    }))
}

/// Cross-field constraints binding an application-bearing node: its key is
/// the JCS digest of its preimage, an expression whose root is an application
/// has the operator's semantic form, and its dependencies are exactly its
/// reference targets and member declarations.
fn application_node_is_valid(node: &Value, nodes: &[Value]) -> bool {
    let Some(preimage) = application_node_preimage(node, nodes) else {
        return false;
    };
    let body = &node["body"];
    let form_fits = node["node_tag"] != "expression"
        || body["term"] != "application"
        || body["operator"]
            .as_str()
            .and_then(expression_form_for_class)
            == node["semantic_form"].as_str();
    node["node_id"]["digest"].as_str() == Some(unqualified_jcs_sha256(&preimage).as_str())
        && form_fits
        && node["dependencies"]
            .as_array()
            .is_some_and(|actual| *actual == application_dependencies(body))
}

/// Semantic refusal of a package's `declaration` members, in the normative
/// order: every nominal mismatch (node id ascending) precedes any ambiguity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeclarationRefusal {
    NominalMismatch,
    AmbiguousName,
}

impl DeclarationRefusal {
    const ALL: [Self; 2] = [Self::NominalMismatch, Self::AmbiguousName];

    fn cause(self) -> &'static str {
        match self {
            Self::NominalMismatch => "declaration-nominal-mismatch",
            Self::AmbiguousName => "ambiguous-name",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::NominalMismatch => "invalid_package",
            Self::AmbiguousName => "ambiguous_declaration",
        }
    }
}

/// Whether a V2 node declares a library-visible name and so must carry the
/// `declaration` member; every other node must not carry it.
fn node_declares(node: &Value) -> bool {
    match node["node_tag"].as_str() {
        Some("scalar_type" | "composite_type" | "bounded_domain" | "function") => true,
        Some("value") => {
            node["semantic_form"] != "enum_value"
                && node["occurrences"].as_array().is_some_and(|occurrences| {
                    occurrences.iter().any(|o| o["role"] == "declaration")
                })
        }
        _ => false,
    }
}

/// Independent oracle over the `declaration` members of a schema-valid V2
/// package; the refusal names the offending node.
fn package_declaration_refusal(package: &Value) -> Result<(), (DeclarationRefusal, Value)> {
    let mut nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| {
        left["node_id"]["digest"]
            .as_str()
            .cmp(&right["node_id"]["digest"].as_str())
    });
    if let Some(node) = nodes.iter().find(|node| {
        node.get("nominal_identity_preimage")
            .and_then(|preimage| preimage.get("qualified_declaration"))
            .is_some_and(|name| node["declaration"]["qualified_name"] != *name)
    }) {
        return Err((DeclarationRefusal::NominalMismatch, node["node_id"].clone()));
    }
    let mut seen = BTreeSet::new();
    for node in nodes {
        if let Some(name) = node
            .get("declaration")
            .map(|d| d["qualified_name"].to_string())
        {
            if !seen.insert(name) {
                return Err((DeclarationRefusal::AmbiguousName, node["node_id"].clone()));
            }
        }
    }
    Ok(())
}

/// Rekey one application-bearing node in the graph, its projection and the
/// source map to its preimage digest, then refresh the package identity.
fn rekey_application_node(package: &mut Value, node_index: usize) {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let node = &nodes[node_index];
    let old_id = node["node_id"].clone();
    let new_digest = unqualified_jcs_sha256(
        &application_node_preimage(node, nodes).expect("rekeyed node bears an application"),
    );
    replace_node_field_in_graph_and_projection(
        package,
        node_index,
        "/node_id/digest",
        json!(new_digest),
    );
    for entry in package["source_map"]
        .as_array_mut()
        .expect("source map is an array")
        .iter_mut()
        .filter(|entry| entry["node_id"] == old_id)
    {
        entry["node_id"]["digest"] = json!(new_digest);
    }
    refresh_package_id(package);
}
/// Unit node index in `positive-nominal-identities.json`; no other node depends on it.
const NOMINAL_UNIT_NODE: usize = 2;

/// Replace the unit node owner and consistently rekey the node, its projection,
/// its source-map entry and the package identity.
fn rekey_nominal_unit_owner(package: &mut Value, owner: Value) {
    replace_nominal_preimage_in_graph_and_projection(package, NOMINAL_UNIT_NODE, "/owner", owner);
    let old_id = package["semantic_graph"]["nodes"][NOMINAL_UNIT_NODE]["node_id"].clone();
    let new_digest = unqualified_jcs_sha256(
        &package["semantic_graph"]["nodes"][NOMINAL_UNIT_NODE]["nominal_identity_preimage"],
    );
    replace_node_field_in_graph_and_projection(
        package,
        NOMINAL_UNIT_NODE,
        "/node_id/digest",
        json!(new_digest),
    );
    for entry in package["source_map"]
        .as_array_mut()
        .expect("source map is an array")
        .iter_mut()
        .filter(|entry| entry["node_id"] == old_id)
    {
        entry["node_id"]["digest"] = json!(new_digest);
    }
    refresh_package_id(package);
}

/// Apply a selection change to the lock and, for identity-bearing members, the
/// identity preimage, then refresh the package identity.
fn replace_selection_consistently(package: &mut Value, pointer: &str, replacement: Value) {
    let targets: &[&str] = if pointer.starts_with("/sources") {
        &["/lock"]
    } else {
        &["/lock", "/identity_preimage"]
    };
    for prefix in targets {
        *package
            .pointer_mut(&format!("{prefix}{pointer}"))
            .expect("authored selection pointer exists") = replacement.clone();
    }
    refresh_package_id(package);
}

#[trace("TC-233", "FR-322-AC-13")]
#[test]
fn tc_233_all_checked_semantic_node_families_are_admitted_by_the_public_schema() {
    let schema = parsed_fixture(SCHEMA);
    jsonschema::validator_for(&schema)
        .expect("I04 schema is valid")
        .validate(&parsed_fixture(POSITIVE_ALL_FAMILIES))
        .expect("positive I04 all-family vector has the closed public shape");

    let vector = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let projections = vector["identity_preimage"]["identity_projection"]
        .as_array()
        .expect("identity projections are an array");
    let nodes = vector["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array");
    assert_eq!(
        nodes.len(),
        projections.len(),
        "every identity-projected node must be present in the admitted graph"
    );
    let node_tags = nodes
        .iter()
        .map(|node| node["node_tag"].as_str().expect("node tag is a string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        node_tags,
        BTreeSet::from(REQUIRED_NODE_TAGS),
        "the all-family vector must exercise every closed V2 node tag exactly"
    );

    let mapped = vector["source_map"]
        .as_array()
        .expect("source map is an array")
        .iter()
        .map(|entry| {
            (
                entry["node_id"]["digest"]
                    .as_str()
                    .expect("source-map node digest is a string"),
                entry["role"].as_str().expect("source-map role is a string"),
                entry["ordinal"]
                    .as_u64()
                    .expect("source-map ordinal is unsigned"),
            )
        })
        .collect::<BTreeSet<_>>();
    for node in nodes {
        for occurrence in node["occurrences"]
            .as_array()
            .expect("node occurrences are an array")
        {
            assert!(
                mapped.contains(&(
                    node["node_id"]["digest"]
                        .as_str()
                        .expect("node digest is a string"),
                    occurrence["role"]
                        .as_str()
                        .expect("occurrence role is a string"),
                    occurrence["ordinal"]
                        .as_u64()
                        .expect("occurrence ordinal is unsigned"),
                )),
                "every graph occurrence has an exact source-map entry"
            );
        }
    }
}

#[trace("TC-233", "FR-322-AC-14")]
#[test]
fn tc_233_package_identity_is_the_jcs_digest_of_its_non_circular_preimage() {
    let vector = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let qualified_digest =
        jcs_sha256(&vector["identity_preimage"]).expect("identity preimage is canonicalizable");
    let actual = qualified_digest
        .strip_prefix("sha256-jcs:")
        .expect("qualified canonicalizer retains its selected prefix");
    let expected = vector["package_id"]["digest"]
        .as_str()
        .expect("package digest is a string");
    assert_eq!(
        actual, expected,
        "package id must bind exactly its preimage"
    );
}

/// Selection members of the V2 lock that are copied, in order, into the
/// `quire.checked-package-id/v2` identity preimage.
const IDENTITY_SELECTION_MEMBERS: [&str; 6] = [
    "edition",
    "profile_selections",
    "definition_selections",
    "model_selections",
    "required_features",
    "dependency_selections",
];

/// Rebuild the non-circular V2 identity preimage from a package's lock and
/// semantic graph alone, dropping node source occurrences, so that the package
/// id is recomputed from its named members rather than read back.
fn derive_identity_preimage(package: &Value) -> Value {
    let mut preimage = serde_json::Map::new();
    preimage.insert("version".to_owned(), json!("quire.checked-package-id/v2"));
    for member in IDENTITY_SELECTION_MEMBERS {
        preimage.insert(member.to_owned(), package["lock"][member].clone());
    }
    let projection = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array")
        .iter()
        .map(|node| {
            let mut projected = node.clone();
            projected
                .as_object_mut()
                .expect("semantic node is an object")
                .remove("occurrences");
            projected
        })
        .collect();
    preimage.insert("identity_projection".to_owned(), Value::Array(projection));
    Value::Object(preimage)
}

#[trace("TC-233", "FR-322-AC-14")]
#[test]
fn tc_233_package_identity_ignores_excluded_members_and_binds_every_named_member() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    for (fixture, bytes) in [
        ("positive-all-families", POSITIVE_ALL_FAMILIES),
        ("positive-nominal-identities", POSITIVE_NOMINAL_IDENTITIES),
        (
            "positive-operation-identities",
            POSITIVE_OPERATION_IDENTITIES,
        ),
    ] {
        let package = parsed_fixture(bytes);
        assert_eq!(
            derive_identity_preimage(&package),
            package["identity_preimage"],
            "{fixture} identity preimage is exactly its lock selections and occurrence-free graph"
        );
        let package_id = package["package_id"]["digest"]
            .as_str()
            .expect("package digest is a string");
        assert_eq!(
            unqualified_jcs_sha256(&derive_identity_preimage(&package)),
            package_id,
            "{fixture} package id is recomputed from its named members"
        );

        // Each excluded member: its edit pointer and replacement, plus the
        // identity-preimage object into which the member would be smuggled.
        let excluded: [(&str, &str, Value, &str, &str, Value); 5] = [
            (
                "source-map region",
                "/source_map/0/regions/0/end",
                json!(9),
                "/identity_preimage",
                "source_map",
                package["source_map"].clone(),
            ),
            (
                "node source occurrence",
                "/semantic_graph/nodes/0/occurrences/0/ordinal",
                json!(7),
                "/identity_preimage/identity_projection/0",
                "occurrences",
                package["semantic_graph"]["nodes"][0]["occurrences"].clone(),
            ),
            (
                "raw source digest",
                "/lock/sources/0/digest",
                json!("8888888888888888888888888888888888888888888888888888888888888888"),
                "/identity_preimage",
                "sources",
                package["lock"]["sources"].clone(),
            ),
            (
                "capability disposition",
                "/capability_report",
                json!([]),
                "/identity_preimage",
                "capability_report",
                package["capability_report"].clone(),
            ),
            (
                "diagnostic catalog",
                "/diagnostics/catalog/digest",
                json!("8888888888888888888888888888888888888888888888888888888888888888"),
                "/identity_preimage",
                "diagnostics",
                package["diagnostics"].clone(),
            ),
        ];
        for (label, pointer, replacement, injection_parent, member, injected) in excluded {
            let mut candidate = package.clone();
            *candidate
                .pointer_mut(pointer)
                .unwrap_or_else(|| panic!("{fixture} has an excluded {label}")) = replacement;
            assert_ne!(
                candidate, package,
                "{fixture} {label} edit changes the package"
            );
            assert!(
                schema.is_valid(&candidate),
                "{fixture} {label} edit keeps the closed V2 shape"
            );
            assert_eq!(
                derive_identity_preimage(&candidate),
                candidate["identity_preimage"],
                "{fixture} stored identity preimage still matches the package after editing the excluded {label}"
            );
            assert_eq!(
                unqualified_jcs_sha256(&derive_identity_preimage(&candidate)),
                package_id,
                "{fixture} editing the excluded {label} leaves the package id unchanged"
            );

            let mut smuggled = package.clone();
            let previous = smuggled
                .pointer_mut(injection_parent)
                .and_then(Value::as_object_mut)
                .unwrap_or_else(|| panic!("{fixture} has an identity object for {label}"))
                .insert(member.to_owned(), injected);
            assert_eq!(previous, None, "{fixture} identity preimage omits {member}");
            assert!(
                !schema.is_valid(&smuggled),
                "{fixture} closed identity preimage refuses the injected excluded {label}"
            );
            assert_ne!(
                derive_identity_preimage(&smuggled),
                smuggled["identity_preimage"],
                "{fixture} an identity preimage carrying the excluded {label} mismatches its derivation"
            );
        }

        let named: [(&str, &str, Value); 8] = [
            (
                "edition digest",
                "/lock/edition/definition/digest",
                json!("8888888888888888888888888888888888888888888888888888888888888888"),
            ),
            (
                "profile selections",
                "/lock/profile_selections",
                json!([{"role": "profile", "definition": package["lock"]["edition"]["definition"]}]),
            ),
            (
                "definition selections",
                "/lock/definition_selections",
                json!([package["lock"]["edition"]["definition"]]),
            ),
            (
                "model selections",
                "/lock/model_selections",
                json!([{
                    "authority": "agent-ix", "identity": "example-compiled-model",
                    "revision": {"namespace": "git", "value": "1"},
                    "digest_domain": "quire.compiled-model.bytes/v1",
                    "digest": "8888888888888888888888888888888888888888888888888888888888888888",
                    "export": "Example"
                }]),
            ),
            (
                "required features",
                "/lock/required_features",
                json!(["quire.value.complete/v1", "quire.value.extra/v1"]),
            ),
            (
                "dependency selections",
                "/lock/dependency_selections",
                json!([{"role": "dependency", "definition": package["lock"]["edition"]["definition"]}]),
            ),
            (
                "graph node type",
                "/semantic_graph/nodes/0/semantic_type/digest",
                json!("8888888888888888888888888888888888888888888888888888888888888888"),
            ),
            ("graph node order", "/semantic_graph/nodes", {
                let mut nodes = package["semantic_graph"]["nodes"].clone();
                nodes
                    .as_array_mut()
                    .expect("semantic graph nodes are an array")
                    .reverse();
                nodes
            }),
        ];
        for (label, pointer, replacement) in named {
            let mut candidate = package.clone();
            *candidate
                .pointer_mut(pointer)
                .unwrap_or_else(|| panic!("{fixture} has a named {label}")) = replacement;
            assert_ne!(
                unqualified_jcs_sha256(&derive_identity_preimage(&candidate)),
                package_id,
                "{fixture} editing the named preimage member {label} changes the package id"
            );
        }
    }
}

/// Typed package key: a digest compares equal only under the same domain.
fn package_key(package_id: &Value) -> (&str, &str) {
    (
        package_id["domain"]
            .as_str()
            .expect("package id domain is a string"),
        package_id["digest"]
            .as_str()
            .expect("package id digest is a string"),
    )
}

#[trace("TC-233", "FR-201-AC-7")]
#[test]
fn tc_233_equal_digest_bytes_under_v1_and_v2_package_domains_compare_unequal() {
    let v1 = jsonschema::validator_for(&parsed_fixture(SCHEMA_V1)).expect("V1 schema is valid");
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let raw_migration_schema = parsed_fixture(MIGRATION_CORRESPONDENCE_SCHEMA);
    let migration =
        jsonschema::validator_for(&raw_migration_schema).expect("migration schema is valid");
    let v2_package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let v1_package = parsed_fixture(POSITIVE_ALL_FAMILIES_V1);

    let v2_id = v2_package["package_id"].clone();
    let mut v1_domain_same_bytes = v2_id.clone();
    v1_domain_same_bytes["domain"] = json!("quire.package.semantic/v1");
    assert_eq!(
        package_key(&v2_id).1,
        package_key(&v1_domain_same_bytes).1,
        "the control shares exact digest bytes"
    );
    assert_ne!(
        package_key(&v2_id),
        package_key(&v1_domain_same_bytes),
        "equal digest bytes under the V1 and V2 package domains are distinct keys"
    );

    let mut relabelled_v2 = v2_package.clone();
    relabelled_v2["package_id"] = v1_domain_same_bytes.clone();
    assert!(v2.is_valid(&v2_package));
    assert!(
        !v2.is_valid(&relabelled_v2),
        "the V2 schema refuses its own digest bytes under the V1 package domain"
    );
    let mut relabelled_v1 = v1_package.clone();
    relabelled_v1["package_id"]["domain"] = json!("quire.package.semantic/v2");
    assert!(v1.is_valid(&v1_package));
    assert!(
        !v1.is_valid(&relabelled_v1),
        "the frozen V1 schema refuses its own digest bytes under the V2 package domain"
    );

    let vectors = parsed_fixture(MIGRATION_CORRESPONDENCE_VECTORS);
    let receipt = &vectors["positives"][0]["outcome"]["correspondence"];
    assert!(migration.is_valid(receipt));
    for (label, pointer, domain) in [
        (
            "source",
            "/source_package_id/domain",
            "quire.package.semantic/v2",
        ),
        (
            "target",
            "/target_package_id/domain",
            "quire.package.semantic/v1",
        ),
    ] {
        let mut swapped = receipt.clone();
        *swapped
            .pointer_mut(pointer)
            .expect("receipt package domain exists") = json!(domain);
        assert!(
            !migration.is_valid(&swapped),
            "the migration receipt refuses a {label} key relabelled to the other package domain"
        );
    }
}

#[trace("TC-233", "FR-322-AC-13")]
#[test]
fn tc_233_public_schema_refuses_authored_version_domain_and_node_mutations() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    assert_eq!(
        named_inventory(&adverse["structural_mutations"], "/id", "/outcome"),
        BTreeSet::from([
            (
                "unknown-contract-version",
                "refused:unknown_contract_version"
            ),
            ("cross-domain-package-id", "refused:digest_domain_mismatch"),
            (
                "cross-domain-source-digest",
                "refused:digest_domain_mismatch"
            ),
            ("unknown-semantic-node-tag", "refused:unsupported_node_tag"),
            ("wrong-node-family-form", "refused:invalid_semantic_graph"),
        ]),
        "the V2 structural adverse inventory is exact"
    );
    for mutation in adverse["structural_mutations"]
        .as_array()
        .expect("adverse mutations are an array")
    {
        let mut candidate = parsed_fixture(POSITIVE_ALL_FAMILIES);
        *candidate
            .pointer_mut(
                mutation["pointer"]
                    .as_str()
                    .expect("mutation pointer is a string"),
            )
            .expect("mutation pointer exists") = mutation["replacement"].clone();
        assert!(
            !schema.is_valid(&candidate),
            "{} must fail the public schema",
            mutation["id"].as_str().expect("mutation id is a string")
        );
    }
}

#[trace("TC-233", "FR-322-AC-13")]
#[test]
fn tc_233_v1_and_v2_remain_explicitly_incompatible_closed_wires() {
    let v1 = jsonschema::validator_for(&parsed_fixture(SCHEMA_V1)).expect("V1 schema is valid");
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let v1_fixture = parsed_fixture(POSITIVE_ALL_FAMILIES_V1);
    let v2_fixture = parsed_fixture(POSITIVE_ALL_FAMILIES);

    assert!(v1.is_valid(&v1_fixture));
    assert!(v2.is_valid(&v2_fixture));

    let nominal_v1_source = parsed_fixture(MIGRATION_SOURCE_NOMINAL_V1);
    v1.validate(&nominal_v1_source)
        .expect("the V2-directory nominal migration source is a frozen V1 package");
    assert!(
        !v2.is_valid(&nominal_v1_source),
        "the V2 schema refuses the V1 nominal migration source"
    );
    assert_eq!(
        unqualified_jcs_sha256(&nominal_v1_source["identity_preimage"]),
        nominal_v1_source["package_id"]["digest"]
            .as_str()
            .expect("nominal V1 source package digest is a string"),
        "the nominal V1 migration source package id binds its stored V1 preimage"
    );
    assert_eq!(
        nominal_v1_source["package_id"]["domain"].as_str(),
        Some("quire.package.semantic/v1")
    );

    assert!(
        !v1.is_valid(&v2_fixture),
        "V1 refuses the V2 version and graph"
    );
    assert!(
        !v2.is_valid(&v1_fixture),
        "the V2 discriminator rejects unchanged V1 bytes"
    );

    let mut widened_v1 = v1_fixture;
    widened_v1["semantic_graph"]["nodes"][0]["nominal_identity_preimage"] = json!({
        "version": "quire.dimension-node/v1",
        "owner": {"kind": "definition", "authority": "agent-ix", "identity": "example-model"},
        "qualified_declaration": ["Example", "Length"],
        "terms": []
    });
    assert!(
        !v1.is_valid(&widened_v1),
        "the frozen V1 node rejects a silently added V2 member"
    );

    let mut relabelled_v1 = parsed_fixture(POSITIVE_ALL_FAMILIES_V1);
    relabelled_v1["contract_version"] = json!("quire.checked-package/v2");
    relabelled_v1["identity_preimage"]["version"] = json!("quire.checked-package-id/v2");
    relabelled_v1["package_id"]["domain"] = json!("quire.package.semantic/v2");
    relabelled_v1["semantic_graph"]["graph_version"] = json!("quire.checked-semantic-graph/v2");
    for projection in relabelled_v1["identity_preimage"]["identity_projection"]
        .as_array_mut()
        .expect("V1 identity projection is an array")
    {
        projection["schema_version"] = json!("quire.checked-semantic-graph/v2");
    }
    for node in relabelled_v1["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("V1 semantic graph is an array")
    {
        node["schema_version"] = json!("quire.checked-semantic-graph/v2");
    }
    replace_node_field_in_graph_and_projection(
        &mut relabelled_v1,
        0,
        "/semantic_form",
        json!("enum"),
    );
    refresh_package_id(&mut relabelled_v1);
    assert!(
        !v2.is_valid(&relabelled_v1),
        "a consistently relabelled V1 graph cannot synthesize the nominal evidence required by V2"
    );
}

#[trace("TC-233", "FR-322-AC-13")]
#[test]
fn tc_233_frozen_v1_publication_bytes_match_independent_locks() {
    let locked = [
        (
            "README.md",
            README_V1_BYTES,
            "29f992c57f59d913e4bc6e1ee06a2327ed9ad5917f4a820a16785dba0977d5d0",
        ),
        (
            "schema.json",
            SCHEMA_V1_BYTES,
            "94c3d39ea81a57ef8c3bf654c6d585b16c51a4c01c85c88f0691ea15aa4fe9e9",
        ),
        (
            "fixtures/positive-all-families.json",
            POSITIVE_ALL_FAMILIES_V1_BYTES,
            "70dd41b9ec9453350ed9becbc504356cfab3a2e00ba8a300b89c30b8e763cc7d",
        ),
        (
            "fixtures/adverse.json",
            ADVERSE_V1_BYTES,
            "d97b09b4d48074d44fc78665dde09a21a321b6dac9eab0ea2cdafad9796f2f95",
        ),
    ];

    for (path, bytes, expected) in locked {
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            expected,
            "frozen V1 publication bytes changed for {path}"
        );
    }
}

#[trace("TC-233", "FR-322-AC-16")]
#[test]
fn tc_233_v1_to_v2_migration_correspondence_is_typed_total_and_input_bound() {
    let raw_schema = parsed_fixture(MIGRATION_CORRESPONDENCE_SCHEMA);
    let schema =
        jsonschema::validator_for(&raw_schema).expect("migration-correspondence schema is valid");
    let outcome_schema = schema_for_def(&raw_schema, "MigrationOutcome");
    let v1_schema =
        jsonschema::validator_for(&parsed_fixture(SCHEMA_V1)).expect("V1 schema is valid");
    let v2_schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(MIGRATION_CORRESPONDENCE_VECTORS);
    let positives = &vectors["positives"];
    assert_eq!(
        named_inventory(
            positives,
            "/name",
            "/outcome/correspondence/node_correspondences/0/basis"
        ),
        BTreeSet::from([
            ("clauses-relinked-rekeyed", "relinked-identical"),
            ("nominal-relinked-rekeyed", "relinked-rekeyed"),
        ]),
        "the positive migration inventory is exact"
    );

    for positive in positives.as_array().expect("positives are an array") {
        let name = positive["name"]
            .as_str()
            .expect("positive name is a string");
        let outcome = &positive["outcome"];
        outcome_schema
            .validate(outcome)
            .unwrap_or_else(|error| panic!("{name} is a closed relinked outcome: {error}"));
        let correspondence = &outcome["correspondence"];
        schema
            .validate(correspondence)
            .unwrap_or_else(|error| panic!("{name} has the closed correspondence shape: {error}"));
        let source = migration_package(
            positive["source_package"]
                .as_str()
                .expect("source package path is a string"),
        );
        let target = migration_package(
            positive["target_package"]
                .as_str()
                .expect("target package path is a string"),
        );
        v1_schema.validate(&source).unwrap_or_else(|error| {
            panic!("{name} source satisfies the frozen V1 schema: {error}")
        });
        v2_schema
            .validate(&target)
            .unwrap_or_else(|error| panic!("{name} target satisfies the V2 schema: {error}"));
        assert_eq!(
            unqualified_jcs_sha256(&source["identity_preimage"]),
            source["package_id"]["digest"]
                .as_str()
                .expect("source package digest is a string"),
            "{name} source package id binds its V1 preimage"
        );
        for field in RECONSTRUCTION_FIELDS {
            assert_eq!(
                correspondence["reconstruction_inputs"][field], source["lock"][field],
                "{name} reconstruction input {field} is selected by the V1 lock"
            );
        }
        assert_eq!(
            classify_migration(correspondence, &source, &target),
            Ok(()),
            "{name} is an admitted re-link"
        );
        assert!(
            correspondence_is_consistent(correspondence, &source, &target),
            "{name} is total and its basis is identical exactly when digests are equal"
        );

        let rows = correspondence["node_correspondences"]
            .as_array()
            .expect("node correspondences are an array");
        for (ordinal, row) in rows.iter().enumerate() {
            let flipped_basis = match row["basis"].as_str() {
                Some("relinked-identical") => "relinked-rekeyed",
                Some("relinked-rekeyed") => "relinked-identical",
                other => panic!("{name} row {ordinal} has an unknown basis: {other:?}"),
            };
            let mut flipped = correspondence.clone();
            flipped["node_correspondences"][ordinal]["basis"] = json!(flipped_basis);
            assert!(schema.is_valid(&flipped));
            assert!(
                !correspondence_is_consistent(&flipped, &source, &target),
                "{name} row {ordinal} refuses a basis that contradicts the node digests"
            );
        }
    }

    let rekeyed = &positives[1]["outcome"]["correspondence"];
    let rekeyed_target = migration_package("fixtures/positive-nominal-identities.json");
    let target_nodes = rekeyed_target["semantic_graph"]["nodes"]
        .as_array()
        .expect("nominal target nodes are an array");
    assert!(
        target_nodes.iter().all(|node| {
            unqualified_jcs_sha256(&node["nominal_identity_preimage"])
                == node["node_id"]["digest"].as_str().unwrap_or_default()
        }),
        "every rekeyed target key is derived from its nominal preimage"
    );
    assert!(
        rekeyed["node_correspondences"]
            .as_array()
            .expect("rekeyed rows are an array")
            .iter()
            .all(|row| row["source_node_id"] != row["target_node_id"]),
        "the canonical nominal migration rekeys every V1 nominal node"
    );

    let catalog = parsed_fixture(OPERATION_CATALOG);
    let clauses_source = migration_package("fixtures/migration-source-clauses-v1.json");
    let clauses_target = migration_package("fixtures/positive-clause-operations.json");
    let clause_nodes = clauses_target["semantic_graph"]["nodes"]
        .as_array()
        .expect("clause target nodes are an array");
    assert_eq!(
        package_operations_refusal(&clauses_target, &catalog),
        Ok(())
    );
    assert_eq!(package_declaration_refusal(&clauses_target), Ok(()));
    let mut derived_classes = BTreeSet::new();
    for (ordinal, (source_node, target_node)) in clauses_source["semantic_graph"]["nodes"]
        .as_array()
        .expect("clause source nodes are an array")
        .iter()
        .zip(clause_nodes)
        .enumerate()
    {
        let source_terms = application_terms(&source_node["body"]);
        assert_eq!(
            source_terms.is_empty(),
            source_node["node_id"] == target_node["node_id"],
            "clause row {ordinal} is rekeyed exactly when it bears an application"
        );
        if !source_terms.is_empty() {
            assert!(
                application_node_is_valid(target_node, clause_nodes),
                "clause row {ordinal} target is keyed by its application-node preimage"
            );
        }
        derived_classes.extend(
            source_terms
                .iter()
                .filter_map(|term| term["operator"].as_str()),
        );
    }
    assert_eq!(
        derived_classes,
        BTreeSet::from([
            "claim",
            "conditional",
            "protocol_control",
            "state_transition",
            "temporal"
        ]),
        "the clause migration exercises every derivable clause class it publishes"
    );
}

#[trace("TC-233", "FR-322-AC-20")]
#[test]
fn tc_233_v1_derivation_table_is_exactly_the_catalogued_selection_free_operations() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let entries = catalog["operations"]
        .as_array()
        .expect("catalog operations are an array");
    let preimage_schema = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let operators = string_set(
        &preimage_schema["$defs"]["PreimageTerm"]["oneOf"][3]["properties"]["operator"]["enum"],
        "application operator enum",
    );
    for operator in operators {
        let candidates = entries
            .iter()
            .filter(|entry| entry["operator"] == operator)
            .collect::<Vec<_>>();
        let selection_free = candidates.len() == 1
            && ["laws", "leaves"]
                .iter()
                .all(|field| candidates[0][*field].as_array().is_none_or(Vec::is_empty))
            && candidates[0]["mode"].is_null()
            && candidates[0]["member"].is_null();
        let derived = derived_v1_operation(operator);
        assert_eq!(
            derived.is_some(),
            selection_free,
            "V1 class {operator} derives exactly when the catalog gives it one selection-free meaning"
        );
        if let Some(derived) = derived {
            assert_eq!(
                derived["identity"], candidates[0]["identity"],
                "V1 class {operator} derives its only catalogued identity"
            );
        }
    }
}

#[trace("TC-233", "FR-322-AC-16")]
#[test]
fn tc_233_v1_to_v2_migration_refusals_use_the_closed_typed_outcome() {
    let raw_schema = parsed_fixture(MIGRATION_CORRESPONDENCE_SCHEMA);
    let schema =
        jsonschema::validator_for(&raw_schema).expect("migration-correspondence schema is valid");
    let outcome_schema = schema_for_def(&raw_schema, "MigrationOutcome");
    let schema_codes = string_set(
        &raw_schema["$defs"]["MigrationRefusalCode"]["enum"],
        "migration refusal code enum",
    );
    assert_eq!(
        schema_codes,
        MigrationRefusalCode::ALL
            .into_iter()
            .map(MigrationRefusalCode::code)
            .collect::<BTreeSet<_>>(),
        "the schema code enum is exactly the vocabulary handled by the oracle"
    );

    let v1_schema =
        jsonschema::validator_for(&parsed_fixture(SCHEMA_V1)).expect("V1 schema is valid");
    let v2_schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(MIGRATION_CORRESPONDENCE_VECTORS);
    let refusals = &vectors["refusals"];
    assert_eq!(
        named_inventory(refusals, "/name", "/expected_outcome/code"),
        BTreeSet::from([
            ("missing-source-input", "migration_input_missing"),
            ("stale-source-input", "migration_input_stale"),
            ("ambiguous-source-input", "migration_input_ambiguous"),
            (
                "incompatible-target-package",
                "migration_target_incompatible"
            ),
            ("byte-only-v1", "migration_byte_only_reconstruction"),
            (
                "missing-source-precedes-ambiguous-source",
                "migration_input_missing"
            ),
            (
                "missing-source-precedes-stale-source",
                "migration_input_missing"
            ),
            ("missing-edition-input", "migration_input_missing"),
            (
                "missing-profile-selections-input",
                "migration_input_missing"
            ),
            (
                "missing-definition-selections-input",
                "migration_input_missing"
            ),
            ("missing-model-selections-input", "migration_input_missing"),
            (
                "missing-dependency-selections-input",
                "migration_input_missing"
            ),
            ("stale-edition-input", "migration_input_stale"),
            ("stale-profile-selections-input", "migration_input_stale"),
            ("stale-definition-selections-input", "migration_input_stale"),
            ("stale-model-selections-input", "migration_input_stale"),
            ("stale-dependency-selections-input", "migration_input_stale"),
            (
                "operation-class-diverges-from-v1",
                "migration_operation_unresolved"
            ),
            (
                "operation-binary-not-derivable-despite-v1-locked-law",
                "migration_operation_unresolved"
            ),
            (
                "all-families-function-call-not-derivable",
                "migration_operation_unresolved"
            ),
            (
                "stale-source-precedes-operation-unresolved",
                "migration_input_stale"
            ),
            (
                "operation-unresolved-precedes-target",
                "migration_operation_unresolved"
            ),
        ]),
        "the migration refusal inventory is exact"
    );
    assert_eq!(
        refusals
            .as_array()
            .expect("migration refusals are an array")
            .iter()
            .filter(|refusal| refusal["base"].as_str() == Some("nominal-relinked-rekeyed"))
            .map(|refusal| (
                refusal["name"].as_str().expect("refusal name is a string"),
                refusal["expected_outcome"]["subject"].to_string(),
            ))
            .collect::<BTreeSet<_>>(),
        [
            (
                "missing-source-precedes-ambiguous-source",
                json!({"input":"sources","authority":"agent-ix","identity":"example-units"}),
            ),
            (
                "missing-source-precedes-stale-source",
                json!({"input":"sources","authority":"agent-ix","identity":"example-units"}),
            ),
            ("missing-edition-input", json!({"input":"edition"})),
            (
                "missing-profile-selections-input",
                json!({"input":"profile_selections"}),
            ),
            (
                "missing-definition-selections-input",
                json!({"input":"definition_selections"}),
            ),
            (
                "missing-model-selections-input",
                json!({"input":"model_selections"}),
            ),
            (
                "missing-dependency-selections-input",
                json!({"input":"dependency_selections"}),
            ),
            ("stale-edition-input", json!({"input":"edition"})),
            (
                "stale-profile-selections-input",
                json!({"input":"profile_selections"}),
            ),
            (
                "stale-definition-selections-input",
                json!({"input":"definition_selections"}),
            ),
            (
                "stale-model-selections-input",
                json!({"input":"model_selections"}),
            ),
            (
                "stale-dependency-selections-input",
                json!({"input":"dependency_selections"}),
            ),
        ]
        .into_iter()
        .map(|(name, subject)| (name, subject.to_string()))
        .collect::<BTreeSet<_>>(),
        "the two-source nominal base pins every non-source input and ordering subject"
    );

    for refusal in refusals
        .as_array()
        .expect("migration refusals are an array")
    {
        let name = refusal["name"].as_str().expect("refusal name is a string");
        let base = vectors["positives"]
            .as_array()
            .expect("positives are an array")
            .iter()
            .find(|positive| positive["name"] == refusal["base"])
            .expect("refusal base positive exists");
        let package_path = |member: &str| {
            refusal
                .get(member)
                .unwrap_or(&base[member])
                .as_str()
                .unwrap_or_else(|| panic!("{name} names its {member}"))
        };
        let mut source = migration_package(package_path("source_package"));
        let mut target = migration_package(package_path("target_package"));
        if refusal.get("target_package").is_none() {
            assert_eq!(
                first_unresolved_operation(&source, &target),
                None,
                "{name}: every operation of the published base target is derivable from its V1 source"
            );
        }
        for (member, package) in [("source_patch", &mut source), ("target_patch", &mut target)] {
            for patch in refusal
                .get(member)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                apply_authored_patch(package, patch);
            }
        }
        v1_schema
            .validate(&source)
            .unwrap_or_else(|error| panic!("{name} source is a frozen V1 package: {error}"));
        v2_schema
            .validate(&target)
            .unwrap_or_else(|error| panic!("{name} target is a V2 package: {error}"));
        assert_eq!(
            unqualified_jcs_sha256(&source["identity_preimage"]),
            source["package_id"]["digest"].as_str().unwrap_or_default(),
            "{name} source package id binds its preimage"
        );
        assert_eq!(
            derive_identity_preimage(&target),
            target["identity_preimage"],
            "{name} target identity preimage is derived from the target"
        );
        assert_eq!(
            unqualified_jcs_sha256(&target["identity_preimage"]),
            target["package_id"]["digest"].as_str().unwrap_or_default(),
            "{name} target package id binds its preimage"
        );
        assert_eq!(
            package_operations_refusal(&target, &catalog),
            Ok(()),
            "{name} target is itself a readable V2 package, so only migration refuses"
        );
        let target_nodes = target["semantic_graph"]["nodes"]
            .as_array()
            .expect("target nodes are an array");
        assert!(
            target_nodes
                .iter()
                .filter(|node| !application_terms(&node["body"]).is_empty())
                .all(|node| application_node_is_valid(node, target_nodes)),
            "{name} target application nodes are keyed by their preimages"
        );
        let mut candidate = base["outcome"]["correspondence"].clone();
        for patch in refusal["patch"]
            .as_array()
            .expect("migration refusal patch is an array")
        {
            apply_authored_patch(&mut candidate, patch);
        }

        let expected = &refusal["expected_outcome"];
        outcome_schema
            .validate(expected)
            .unwrap_or_else(|error| panic!("{name} expected outcome is closed: {error}"));
        let code = expected["code"]
            .as_str()
            .and_then(MigrationRefusalCode::from_code)
            .unwrap_or_else(|| panic!("{name} code is in the closed vocabulary"));
        assert_eq!(
            classify_migration(&candidate, &source, &target),
            Err((code, expected["subject"].clone())),
            "{name} refuses with its exact typed code and subject"
        );
        let names_a_source = expected["subject"].get("identity").is_some();
        match code {
            MigrationRefusalCode::ByteOnlyReconstruction => {
                assert!(
                    !schema.is_valid(&candidate),
                    "{name} cannot even form a correspondence receipt"
                );
            }
            MigrationRefusalCode::InputMissing => {
                assert_eq!(
                    schema.is_valid(&candidate),
                    names_a_source,
                    "{name}: an absent member or empty source list cannot form a receipt, \
                     while an absent locked source among supplied sources is refused only by re-link"
                );
            }
            MigrationRefusalCode::InputStale
            | MigrationRefusalCode::InputAmbiguous
            | MigrationRefusalCode::TargetIncompatible
            | MigrationRefusalCode::OperationUnresolved => {
                assert!(
                    schema.is_valid(&candidate),
                    "{name} is structurally well formed and refused only by re-link"
                );
            }
        }
    }

    for (label, malformed) in [
        (
            "unknown code",
            json!({"outcome":"refused","code":"migration_input_unknown","subject":{"input":"sources"}}),
        ),
        (
            "relinked outcome carrying a code",
            json!({"outcome":"relinked","code":"migration_input_missing","subject":{"input":"sources"}}),
        ),
        (
            "refusal without a subject",
            json!({"outcome":"refused","code":"migration_input_missing"}),
        ),
        (
            "subject authority without identity",
            json!({"outcome":"refused","code":"migration_input_stale","subject":{"input":"sources","authority":"agent-ix"}}),
        ),
        (
            "unknown subject input",
            json!({"outcome":"refused","code":"migration_input_stale","subject":{"input":"graph_bytes"}}),
        ),
    ] {
        assert!(
            !outcome_schema.is_valid(&malformed),
            "the closed migration outcome refuses a {label}"
        );
    }
}

#[trace("TC-233", "FR-322-AC-15")]
#[test]
fn tc_233_nominal_identity_preimages_have_exact_jcs_identities_and_semantics() {
    let schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-preimage schema is valid");
    let public_schema = parsed_fixture(SCHEMA);
    let integrated_schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": "#/$defs/NominalNodePreimage",
        "$defs": public_schema["$defs"]
    });
    let integrated_schema = jsonschema::validator_for(&integrated_schema)
        .expect("integrated nominal-preimage schema is valid");
    let fixture = parsed_fixture(NODE_IDENTITY_VECTORS);
    let vectors = fixture["vectors"]
        .as_array()
        .expect("node identity vectors are an array");
    let mut names = BTreeSet::new();

    for vector in vectors {
        let name = vector["name"].as_str().expect("vector name is a string");
        assert!(names.insert(name), "vector names are unique");
        schema
            .validate(&vector["preimage"])
            .unwrap_or_else(|error| panic!("{name} has the closed preimage shape: {error}"));
        integrated_schema
            .validate(&vector["preimage"])
            .unwrap_or_else(|error| panic!("{name} is integrated in the I04 schema: {error}"));
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string"),
            "{name} binds its exact RFC 8785 JCS preimage"
        );
        assert!(
            semantic_preimage_is_valid(&vector["preimage"], vectors),
            "{name} satisfies the closed nominal-node semantic constraints"
        );
    }
}

#[trace("TC-233", "FR-322-AC-15")]
#[test]
fn tc_233_nominal_node_mutations_refuse_before_exposing_a_stale_identity() {
    let schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-preimage schema is valid");
    let fixture = parsed_fixture(NODE_IDENTITY_VECTORS);
    let vectors = fixture["vectors"]
        .as_array()
        .expect("node identity vectors are an array");
    let nominal_package = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);
    let nominal_lock = &nominal_package["lock"];

    // The semantic oracle itself, not only the schema, refuses `-0`.
    let preimage_of = |name: &str| {
        vectors
            .iter()
            .find(|vector| vector["name"].as_str() == Some(name))
            .unwrap_or_else(|| panic!("{name} vector exists"))["preimage"]
            .clone()
    };
    let velocity_terms = preimage_of("dimension-velocity")["terms"].clone();
    let velocity_terms = velocity_terms
        .as_array()
        .expect("velocity terms are an array");
    assert!(terms_are_canonical(velocity_terms, "dimension_node_id"));
    let mut negative_zero_exponent = velocity_terms.clone();
    negative_zero_exponent[0]["exponent"] = json!("-0");
    assert!(
        !terms_are_canonical(&negative_zero_exponent, "dimension_node_id"),
        "the term oracle refuses a -0 exponent"
    );
    let centimetre = preimage_of("unit-centimetre");
    assert!(unit_chain_reaches_root(&centimetre, vectors));
    let mut negative_zero_scale = centimetre.clone();
    negative_zero_scale["scale"] = json!({"numerator": "-0", "denominator": "1"});
    assert!(
        !unit_chain_reaches_root(&negative_zero_scale, vectors),
        "the unit oracle refuses a -0 scale on a non-root unit"
    );
    let metre = preimage_of("unit-metre");
    assert!(unit_chain_reaches_root(&metre, vectors));
    let mut negative_zero_offset = metre.clone();
    negative_zero_offset["offset"]["numerator"] = json!("-0");
    assert!(
        !rational_is_canonical(&negative_zero_offset["offset"]),
        "a -0 offset numerator is not a canonical rational"
    );
    assert!(
        !unit_chain_reaches_root(&negative_zero_offset, vectors),
        "the unit oracle refuses a -0 root offset"
    );
    assert_eq!(
        named_inventory(&fixture["invalid_mutations"], "/name", "/kind"),
        BTreeSet::from([
            ("enum-member-stale-case", "stale_key"),
            ("enum-owner-absent-from-lock", "owner_join"),
            ("unordered-enum-member-order", "semantic"),
            ("dimension-zero-exponent", "semantic"),
            ("dimension-duplicate-term", "semantic"),
            ("dimension-unsorted-terms", "semantic"),
            ("root-unit-nonidentity-scale", "semantic"),
            ("nonroot-unit-missing-target", "semantic"),
            ("unit-unreduced-rational", "semantic"),
            ("dimension-negative-zero-exponent", "semantic"),
            ("unit-negative-zero-scale", "semantic"),
            ("unit-negative-zero-offset", "semantic"),
        ]),
        "the nominal-node mutation inventory is exact"
    );
    assert_eq!(
        refusing_check_inventory(&fixture["invalid_mutations"]),
        BTreeSet::from([
            ("unordered-enum-member-order", "semantic"),
            ("dimension-zero-exponent", "semantic"),
            ("dimension-duplicate-term", "semantic"),
            ("dimension-unsorted-terms", "semantic"),
            ("root-unit-nonidentity-scale", "semantic"),
            ("nonroot-unit-missing-target", "semantic"),
            ("unit-unreduced-rational", "semantic"),
            ("dimension-negative-zero-exponent", "schema"),
            ("unit-negative-zero-scale", "schema"),
            ("unit-negative-zero-offset", "schema"),
        ]),
        "every semantic nominal-node mutation names exactly its refusing check"
    );

    for mutation in fixture["invalid_mutations"]
        .as_array()
        .expect("invalid mutations are an array")
    {
        let name = mutation["name"]
            .as_str()
            .expect("mutation name is a string");
        let base_name = mutation["base"].as_str().expect("base name is a string");
        let base = vectors
            .iter()
            .find(|vector| vector["name"].as_str() == Some(base_name))
            .expect("mutation base vector exists");
        let mut candidate = base["preimage"].clone();
        for patch in mutation["patch"]
            .as_array()
            .expect("mutation patch is an array")
        {
            apply_authored_patch(&mut candidate, patch);
        }
        assert_eq!(
            mutation["expected_code"].as_str(),
            Some("invalid_semantic_graph"),
            "{name} fixes its expected reader code"
        );
        let structurally_valid = schema.is_valid(&candidate);
        let semantically_valid = semantic_preimage_is_valid(&candidate, vectors);
        match mutation["kind"]
            .as_str()
            .expect("mutation kind is a string")
        {
            "stale_key" => {
                assert!(
                    structurally_valid && semantically_valid,
                    "{name} remains a valid new body"
                );
                assert_ne!(
                    unqualified_jcs_sha256(&candidate),
                    mutation["retained_sha256"]
                        .as_str()
                        .expect("retained digest is a string"),
                    "{name} must fail exact stale-key recomputation"
                );
            }
            "semantic" => assert_refused_by_named_check(
                name,
                &mutation["refused_by"],
                structurally_valid,
                semantically_valid,
            ),
            "owner_join" => {
                assert!(
                    structurally_valid && semantically_valid,
                    "{name} has valid detached bytes"
                );
                assert!(
                    owner_joins_lock(&base["preimage"], nominal_lock),
                    "{name} base owner joins the nominal package lock"
                );
                assert!(
                    !owner_joins_lock(&candidate, nominal_lock),
                    "{name} owner is refused by the nominal package lock join"
                );
            }
            kind => panic!("unknown authored mutation kind: {kind}"),
        }
    }
}

#[trace("TC-233", "FR-322-AC-15")]
#[test]
fn tc_233_nominal_preimages_are_closed_inside_a_real_package_and_lock() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let package = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);
    schema
        .validate(&package)
        .expect("nominal identity package has the closed I04 wire shape");
    assert!(
        package_nominal_nodes_are_valid(&package),
        "package nodes bind exact preimages, projections, package id and owner lock joins"
    );

    let mut absent_owner = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut absent_owner,
        1,
        "/owner/identity",
        json!("absent-model"),
    );
    assert!(!package_nominal_nodes_are_valid(&absent_owner));

    let mut lock_diverges_from_preimage = package.clone();
    lock_diverges_from_preimage["lock"]["definition_selections"][0]["digest"] =
        json!("5555555555555555555555555555555555555555555555555555555555555555");
    assert!(!package_lock_matches_identity_preimage(
        &lock_diverges_from_preimage
    ));
    assert!(!package_nominal_nodes_are_valid(
        &lock_diverges_from_preimage
    ));

    let mut unreduced = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut unreduced,
        2,
        "/offset",
        json!({"numerator":"2", "denominator":"2"}),
    );
    assert!(!package_nominal_nodes_are_valid(&unreduced));

    let mut signed_denominator = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut signed_denominator,
        2,
        "/scale/denominator",
        json!("-1"),
    );
    assert!(!schema.is_valid(&signed_denominator));

    let mut invalid_root = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut invalid_root,
        2,
        "/scale/numerator",
        json!("2"),
    );
    assert!(!package_nominal_nodes_are_valid(&invalid_root));

    let mut invalid_root_offset = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut invalid_root_offset,
        2,
        "/offset/numerator",
        json!("1"),
    );
    assert!(!package_nominal_nodes_are_valid(&invalid_root_offset));

    let mut stale_node_id = package.clone();
    stale_node_id["semantic_graph"]["nodes"][1]["node_id"]["digest"] =
        json!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert!(!package_nominal_nodes_are_valid(&stale_node_id));

    let mut unordered_enum = package.clone();
    replace_nominal_preimage_in_graph_and_projection(
        &mut unordered_enum,
        1,
        "/ordered",
        json!(false),
    );
    assert!(!package_nominal_nodes_are_valid(&unordered_enum));
}

#[trace("TC-233", "FR-322-AC-15")]
#[test]
fn tc_233_nominal_preimages_join_semantic_fields_and_dependencies() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let package = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);

    let mut changed_enum_body = package.clone();
    replace_node_field_in_graph_and_projection(
        &mut changed_enum_body,
        0,
        "/body/value",
        json!("DONE"),
    );
    refresh_package_id(&mut changed_enum_body);
    assert!(schema.is_valid(&changed_enum_body));
    assert!(!package_nominal_nodes_are_valid(&changed_enum_body));

    let mut changed_enum_type = package.clone();
    let wrong_enum_type = package["semantic_graph"]["nodes"][2]["node_id"].clone();
    replace_node_field_in_graph_and_projection(
        &mut changed_enum_type,
        0,
        "/semantic_type",
        wrong_enum_type,
    );
    refresh_package_id(&mut changed_enum_type);
    assert!(schema.is_valid(&changed_enum_type));
    assert!(!package_nominal_nodes_are_valid(&changed_enum_type));

    let mut missing_enum_dependency = package.clone();
    replace_node_field_in_graph_and_projection(
        &mut missing_enum_dependency,
        0,
        "/dependencies",
        json!([]),
    );
    refresh_package_id(&mut missing_enum_dependency);
    assert!(schema.is_valid(&missing_enum_dependency));
    assert!(!package_nominal_nodes_are_valid(&missing_enum_dependency));

    let mut changed_unit_type = package.clone();
    let wrong_unit_type = package["semantic_graph"]["nodes"][1]["node_id"].clone();
    replace_node_field_in_graph_and_projection(
        &mut changed_unit_type,
        2,
        "/semantic_type",
        wrong_unit_type,
    );
    refresh_package_id(&mut changed_unit_type);
    assert!(schema.is_valid(&changed_unit_type));
    assert!(!package_nominal_nodes_are_valid(&changed_unit_type));

    let mut missing_dimension_dependency = package.clone();
    replace_node_field_in_graph_and_projection(
        &mut missing_dimension_dependency,
        2,
        "/dependencies",
        json!([]),
    );
    refresh_package_id(&mut missing_dimension_dependency);
    assert!(schema.is_valid(&missing_dimension_dependency));
    assert!(!package_nominal_nodes_are_valid(
        &missing_dimension_dependency
    ));

    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let centimetre = vectors["vectors"]
        .as_array()
        .expect("node vectors are an array")
        .iter()
        .find(|vector| vector["name"].as_str() == Some("unit-centimetre"))
        .expect("centimetre vector exists");
    let preimage = &centimetre["preimage"];
    let mut nonroot_unit = json!({
        "node_id": {"domain":"quire.checked-semantic-node/v1", "digest":centimetre["sha256"]},
        "schema_version":"quire.checked-semantic-graph/v2",
        "node_tag":"scalar_type",
        "semantic_form":"unit",
        "semantic_type":preimage["dimension_node_id"],
        "dependencies":[preimage["dimension_node_id"], preimage["target_unit_node_id"]],
        "occurrences":[{"role":"declaration", "ordinal":0}],
        "nominal_identity_preimage":preimage,
        "body":{"term":"aggregate", "members":[]}
    });
    assert!(nominal_node_cross_fields_are_valid(&nonroot_unit));
    nonroot_unit["dependencies"] = json!([preimage["dimension_node_id"]]);
    assert!(!nominal_node_cross_fields_are_valid(&nonroot_unit));
}

#[trace("TC-233", "FR-322-AC-15")]
#[test]
fn tc_233_owner_to_lock_join_refuses_independently_of_lock_preimage_equality() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let model_selection = json!([{
        "authority": "agent-ix",
        "identity": "example-compiled-model",
        "revision": {"namespace": "git", "value": "1"},
        "digest_domain": "quire.compiled-model.bytes/v1",
        "digest": "5555555555555555555555555555555555555555555555555555555555555555",
        "export": "Example"
    }]);
    let model_owner = json!({
        "kind": "model", "authority": "agent-ix",
        "identity": "example-compiled-model", "export": "Example"
    });
    let cases = [
        (
            "source",
            json!({"kind": "source", "authority": "agent-ix", "identity": "example"}),
            None,
            ("/sources/0/identity", json!("other-source")),
        ),
        (
            "definition",
            json!({"kind": "definition", "authority": "agent-ix", "identity": "example-model"}),
            None,
            ("/definition_selections/0/identity", json!("other-model")),
        ),
        (
            "edition",
            json!({"kind": "definition", "authority": "agent-ix", "identity": "quire-edition"}),
            None,
            ("/edition/definition/identity", json!("other-edition")),
        ),
        (
            "model",
            model_owner.clone(),
            Some(model_selection.clone()),
            (
                "/model_selections/0/identity",
                json!("other-compiled-model"),
            ),
        ),
        (
            "model-export",
            model_owner,
            Some(model_selection),
            ("/model_selections/0/export", json!("Other")),
        ),
    ];

    for (name, owner, models, (pointer, replacement)) in cases {
        let mut joined = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);
        if let Some(models) = models {
            replace_selection_consistently(&mut joined, "/model_selections", models);
        }
        rekey_nominal_unit_owner(&mut joined, owner);
        schema
            .validate(&joined)
            .unwrap_or_else(|error| panic!("{name} control package has the I04 shape: {error}"));
        let unit_preimage = |package: &Value| {
            package["semantic_graph"]["nodes"][NOMINAL_UNIT_NODE]["nominal_identity_preimage"]
                .clone()
        };
        assert!(
            owner_joins_lock(&unit_preimage(&joined), &joined["lock"]),
            "{name} control owner joins the lock"
        );
        assert!(
            package_nominal_nodes_are_valid(&joined),
            "{name} control package is fully valid"
        );

        let mut unjoined = joined.clone();
        replace_selection_consistently(&mut unjoined, pointer, replacement);
        assert!(
            schema.is_valid(&unjoined),
            "{name} mutation keeps the I04 shape"
        );
        assert!(
            package_lock_matches_identity_preimage(&unjoined),
            "{name} mutation keeps the lock and identity preimage consistent"
        );
        assert_eq!(
            unqualified_jcs_sha256(&unjoined["identity_preimage"]),
            unjoined["package_id"]["digest"]
                .as_str()
                .expect("package digest is a string"),
            "{name} mutation keeps an exact package identity"
        );
        assert!(
            !owner_joins_lock(&unit_preimage(&unjoined), &unjoined["lock"]),
            "{name} owner no longer joins the lock"
        );
        assert!(
            !package_nominal_nodes_are_valid(&unjoined),
            "{name} package refuses through the owner join alone"
        );
    }
}

#[trace("TC-233", "FR-142-AC-8")]
#[test]
fn tc_233_compound_unit_value_identities_have_exact_vectors_and_non_tautological_mutations() {
    let schema = jsonschema::validator_for(&parsed_fixture(COMPOUND_UNIT_SCHEMA))
        .expect("compound-unit schema is valid");
    let fixture = parsed_fixture(COMPOUND_UNIT_VECTORS);
    let node_fixture = parsed_fixture(NODE_IDENTITY_VECTORS);
    let node_vectors = node_fixture["vectors"]
        .as_array()
        .expect("node vectors are an array");
    let vectors = fixture["vectors"]
        .as_array()
        .expect("compound-unit vectors are an array");
    assert_eq!(
        named_inventory(&fixture["invalid_mutations"], "/name", "/kind"),
        BTreeSet::from([
            ("stale-digest", "stale_key"),
            ("zero-exponent", "semantic"),
            ("duplicate-term", "semantic"),
            ("unsorted-terms", "semantic"),
            ("negative-zero-exponent", "semantic"),
        ]),
        "the compound-unit mutation inventory is exact"
    );
    assert_eq!(
        refusing_check_inventory(&fixture["invalid_mutations"]),
        BTreeSet::from([
            ("zero-exponent", "semantic"),
            ("duplicate-term", "semantic"),
            ("unsorted-terms", "semantic"),
            ("negative-zero-exponent", "schema"),
        ]),
        "every semantic compound-unit mutation names exactly its refusing check"
    );

    for vector in vectors {
        schema
            .validate(&vector["preimage"])
            .expect("compound-unit vector has the closed shape");
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string")
        );
        assert!(compound_unit_preimage_is_valid(
            &vector["preimage"],
            node_vectors
        ));
    }

    for mutation in fixture["invalid_mutations"]
        .as_array()
        .expect("compound-unit mutations are an array")
    {
        let base_name = mutation["base"].as_str().expect("base name is a string");
        let base = vectors
            .iter()
            .find(|vector| vector["name"].as_str() == Some(base_name))
            .expect("mutation base exists");
        let mut candidate = base["preimage"].clone();
        for patch in mutation["patch"].as_array().expect("patch is an array") {
            apply_authored_patch(&mut candidate, patch);
        }
        match mutation["kind"]
            .as_str()
            .expect("mutation kind is a string")
        {
            "stale_key" => {
                assert!(schema.is_valid(&candidate));
                assert!(compound_unit_preimage_is_valid(&candidate, node_vectors));
                assert_ne!(
                    unqualified_jcs_sha256(&candidate),
                    mutation["retained_sha256"]
                        .as_str()
                        .expect("digest is a string")
                );
            }
            "semantic" => assert_refused_by_named_check(
                mutation["name"]
                    .as_str()
                    .expect("mutation name is a string"),
                &mutation["refused_by"],
                schema.is_valid(&candidate),
                compound_unit_preimage_is_valid(&candidate, node_vectors),
            ),
            kind => panic!("unknown compound mutation kind: {kind}"),
        }
    }
}

#[trace("TC-233", "FR-201-AC-6")]
#[test]
fn tc_233_complete_value_rule_manifest_binds_every_selected_rule_byte_for_byte() {
    let manifest = parsed_fixture(VALUE_RULE_MANIFEST);
    assert_eq!(
        manifest["definition_identity"].as_str(),
        Some("quire.value.complete.rules/v1")
    );
    assert_eq!(
        manifest["digest_domain"].as_str(),
        Some("quire.definition.bytes/v1")
    );
    let rules = manifest["rules"]
        .as_array()
        .expect("rule manifest contains an array");
    let rule_artifacts = CATALOG_ARTIFACTS
        .iter()
        .filter(|artifact| artifact.kind == ArtifactKind::Rule)
        .collect::<Vec<_>>();
    assert_eq!(rules.len(), rule_artifacts.len());
    let mut identities = BTreeSet::new();
    let mut paths = BTreeSet::new();

    for rule in rules {
        let identity = rule["identity"]
            .as_str()
            .expect("rule identity is a string");
        let path = rule["path"].as_str().expect("rule path is a string");
        assert!(identities.insert(identity), "rule identities are unique");
        assert!(paths.insert(path), "rule paths are unique");
        let artifact = rule_artifacts
            .iter()
            .find(|artifact| artifact.path == path)
            .expect("every manifest path has compile-time rule bytes");
        assert_eq!(identity, artifact.identity);
        assert_eq!(rule["revision"].as_str(), Some(artifact.revision));
        assert_eq!(
            format!("{:x}", Sha256::digest(artifact.bytes)),
            rule["sha256"].as_str().expect("rule digest is a string"),
            "{identity} binds exact authored bytes"
        );
    }
}

#[trace("TC-233", "FR-201-AC-6")]
#[test]
fn tc_233_complete_value_lock_catalogs_every_artifact_and_separates_package_selection() {
    let lock = parsed_fixture(COMPLETE_VALUE_LOCK);
    assert_eq!(
        lock["lock_version"].as_str(),
        Some("quire.value.definition-lock/v1")
    );
    assert_eq!(lock["revision"].as_str(), Some("1-draft.1"));
    assert_eq!(
        lock["digest_domain"].as_str(),
        Some("quire.definition.bytes/v1")
    );
    assert_eq!(lock["language_edition"].as_str(), Some("1-draft"));

    let catalog = lock["qualification_catalog"]
        .as_array()
        .expect("qualification catalog is an array");
    assert_eq!(
        catalog
            .iter()
            .map(|entry| entry["artifact_path"]
                .as_str()
                .expect("artifact path is a string"))
            .collect::<BTreeSet<_>>(),
        CATALOG_ARTIFACTS
            .iter()
            .map(|artifact| artifact.path)
            .collect::<BTreeSet<_>>(),
        "the catalog closes exactly every required, conditional, alternative and rule artifact"
    );
    assert_eq!(catalog.len(), CATALOG_ARTIFACTS.len());
    let mut roles = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for entry in catalog {
        assert!(
            definition_lock_entry_matches_bytes(entry),
            "catalog entry binds exact authored bytes: {}",
            entry["artifact_path"]
        );
        assert!(
            roles.insert(entry["role"].as_str().expect("role is a string")),
            "catalog roles are unique"
        );
        assert!(
            identities.insert(
                entry["definition"]["identity"]
                    .as_str()
                    .expect("definition identity is a string")
            ),
            "catalog identities are unique"
        );
    }

    let manifest = parsed_fixture(VALUE_RULE_MANIFEST);
    let manifest_rules = manifest["rules"]
        .as_array()
        .expect("rule manifest contains an array")
        .iter()
        .map(|rule| {
            (
                rule["path"].as_str(),
                rule["identity"].as_str(),
                rule["revision"].as_str(),
                rule["sha256"].as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let catalog_rules = catalog
        .iter()
        .filter(|entry| {
            entry["artifact_path"]
                .as_str()
                .and_then(catalog_artifact)
                .is_some_and(|artifact| artifact.kind == ArtifactKind::Rule)
        })
        .map(|entry| {
            (
                entry["artifact_path"].as_str(),
                entry["definition"]["identity"].as_str(),
                entry["definition"]["revision"]["value"].as_str(),
                entry["definition"]["digest"].as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        catalog_rules, manifest_rules,
        "catalog rules are the manifest rules"
    );

    let selection = PackageSelectionRules::from_lock(&lock);
    let expected_triggers = BTreeSet::from(["ieee_operation", "integer_div_rem", "text_bearing"]);
    assert_eq!(selection.trigger_vocabulary, expected_triggers);
    let used_triggers = selection
        .conditional
        .iter()
        .map(|(trigger, _)| *trigger)
        .chain(selection.exactly_one.iter().map(|(trigger, _, _)| *trigger))
        .collect::<Vec<_>>();
    assert_eq!(used_triggers.len(), expected_triggers.len());
    assert_eq!(
        used_triggers.into_iter().collect::<BTreeSet<_>>(),
        expected_triggers,
        "each closed trigger governs exactly one selection slot"
    );
    assert_eq!(
        selection
            .conditional
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            ("ieee_operation", "ieee_profile"),
            ("text_bearing", "text_profile"),
        ])
    );
    assert_eq!(selection.exactly_one.len(), 1);
    let (division_trigger, division_selector, division_roles) = &selection.exactly_one[0];
    assert_eq!(*division_trigger, "integer_div_rem");
    assert_eq!(*division_selector, "integer_division_profile");
    assert_eq!(
        division_roles.iter().copied().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "integer_division_euclidean",
            "integer_division_floor",
            "integer_division_truncating",
        ])
    );

    let mut occupancy = BTreeMap::<&str, usize>::new();
    for role in selection.slot_roles() {
        *occupancy.entry(role).or_default() += 1;
    }
    assert_eq!(
        occupancy,
        roles
            .iter()
            .map(|role| (*role, 1))
            .collect::<BTreeMap<_, _>>(),
        "every catalog entry occupies exactly one selection slot and no slot names an uncatalogued role"
    );
    assert_eq!(
        string_set(&lock["selection_refusal_codes"], "selection refusal codes"),
        SelectionRefusal::ALL
            .into_iter()
            .map(SelectionRefusal::code)
            .collect::<BTreeSet<_>>(),
        "the lock refusal vocabulary is exactly the oracle vocabulary"
    );

    let mut stale = catalog[0].clone();
    stale["definition"]["digest"] =
        json!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert!(!definition_lock_entry_matches_bytes(&stale));

    let mut relabeled = catalog[0].clone();
    relabeled["definition"]["identity"] = json!("ix:other");
    assert!(!definition_lock_entry_matches_bytes(&relabeled));

    let mut wrong_revision = catalog[0].clone();
    wrong_revision["definition"]["revision"]["value"] = json!("1-draft.999");
    assert!(!definition_lock_entry_matches_bytes(&wrong_revision));

    let mut wrong_role = catalog[0].clone();
    wrong_role["role"] = json!("root");
    assert!(!definition_lock_entry_matches_bytes(&wrong_role));

    let alternative = catalog
        .iter()
        .find(|entry| entry["role"].as_str() == Some("integer_division_floor"))
        .expect("floor division profile is catalogued");
    let mut relabeled_alternative = alternative.clone();
    relabeled_alternative["definition"]["identity"] =
        json!("quire.value.integer-division.other/v1");
    assert!(!definition_lock_entry_matches_bytes(&relabeled_alternative));

    let rule = catalog
        .iter()
        .find(|entry| entry["role"].as_str() == Some("rule_fr_147"))
        .expect("FR-147 rule is catalogued");
    let mut stale_rule = rule.clone();
    stale_rule["definition"]["digest"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    assert!(!definition_lock_entry_matches_bytes(&stale_rule));
}

#[trace("TC-233", "FR-201-AC-6")]
#[test]
fn tc_233_package_selection_vectors_admit_exact_conditional_and_alternative_profiles() {
    let lock = parsed_fixture(COMPLETE_VALUE_LOCK);
    let selection = PackageSelectionRules::from_lock(&lock);
    let vectors = parsed_fixture(COMPLETE_VALUE_SELECTION_VECTORS);
    assert_eq!(vectors["lock"]["lock_version"], lock["lock_version"]);
    assert_eq!(vectors["lock"]["revision"], lock["revision"]);
    let always = string_list(&vectors["always_roles"], "always roles");
    assert_eq!(
        always, selection.always,
        "vectors select the lock's always slot"
    );

    assert_eq!(
        vectors["accepted"]
            .as_array()
            .expect("accepted selections are an array")
            .iter()
            .map(|vector| vector["name"].as_str().expect("vector name is a string"))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "no-trigger",
            "text-bearing",
            "ieee-operation",
            "div-rem-euclidean",
            "div-rem-floor",
            "div-rem-truncating",
            "every-trigger",
        ]),
        "the accepted selection inventory is exact"
    );
    assert_eq!(
        named_inventory(&vectors["refused"], "/name", "/expected_code"),
        BTreeSet::from([
            ("two-division-profiles", "selection_alternative_conflict"),
            (
                "division-missing-for-div-rem",
                "selection_trigger_unsatisfied"
            ),
            ("division-without-trigger", "selection_untriggered_profile"),
            ("ieee-without-trigger", "selection_untriggered_profile"),
            (
                "unicode-missing-for-text-bearing",
                "selection_trigger_unsatisfied"
            ),
            ("always-role-missing", "selection_required_missing"),
            ("unknown-trigger", "selection_unknown_trigger"),
            ("duplicate-trigger", "selection_duplicate_trigger"),
            (
                "unknown-trigger-before-duplicate-trigger",
                "selection_unknown_trigger"
            ),
            (
                "duplicate-trigger-before-unknown-role",
                "selection_duplicate_trigger"
            ),
            ("unknown-role", "selection_unknown_role"),
            ("duplicate-role", "selection_duplicate_role"),
        ]),
        "the refused selection inventory is exact"
    );

    let check = |vector: &Value| {
        let omitted = vector
            .get("omitted_always_roles")
            .map(|omitted| string_set(omitted, "omitted always roles"))
            .unwrap_or_default();
        let roles = always
            .iter()
            .copied()
            .filter(|role| !omitted.contains(role))
            .chain(string_list(&vector["additional_roles"], "additional roles"))
            .collect::<Vec<_>>();
        selection.check(&string_list(&vector["triggers"], "triggers"), &roles)
    };

    for vector in vectors["accepted"]
        .as_array()
        .expect("accepted selections are an array")
    {
        assert_eq!(
            check(vector),
            Ok(()),
            "{} selects exactly its triggered profiles",
            vector["name"]
        );
    }
    for vector in vectors["refused"]
        .as_array()
        .expect("refused selections are an array")
    {
        let expected = SelectionRefusal::ALL
            .into_iter()
            .find(|refusal| vector["expected_code"].as_str() == Some(refusal.code()))
            .unwrap_or_else(|| panic!("{} code is in the closed vocabulary", vector["name"]));
        assert_eq!(
            check(vector),
            Err(expected),
            "{} refuses with its exact selection code",
            vector["name"]
        );
    }
}

const NATIVE_DIAGNOSTICS: &str =
    include_str!("../proposals/quire-v1/definitions/native-diagnostics.md");

/// Replace one application-bearing node of a package, recompute its exact
/// dependencies and key it, its projection and the package from the preimage.
fn replace_application_node(package: &mut Value, index: usize, mut node: Value) {
    node["dependencies"] = json!(application_dependencies(&node["body"]));
    let mut projected = node.clone();
    projected
        .as_object_mut()
        .expect("semantic node is an object")
        .remove("occurrences");
    package["semantic_graph"]["nodes"][index] = node;
    package["identity_preimage"]["identity_projection"][index] = projected;
    rekey_application_node(package, index);
}

fn node_index_by_digest(package: &Value, digest: &str) -> Option<usize> {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .position(|node| node["node_id"]["digest"] == digest)
}

fn operation_vector<'a>(vectors: &'a Value, name: &str) -> &'a Value {
    vectors["operation_vectors"]
        .as_array()
        .expect("operation vectors are an array")
        .iter()
        .find(|vector| vector["name"] == name)
        .unwrap_or_else(|| panic!("operation vector {name} is published"))
}

/// Package index of the published node keyed by the named operation vector.
fn operation_vector_node(package: &Value, vectors: &Value, name: &str) -> usize {
    let digest = operation_vector(vectors, name)["sha256"]
        .as_str()
        .expect("vector digest is a string");
    node_index_by_digest(package, digest)
        .unwrap_or_else(|| panic!("operation vector {name} keys a published package node"))
}

/// Causes a native-diagnostics row lists for exactly the given code: the
/// backticked spellings of its cause cell before the first `;`.
fn native_diagnostic_row_causes(code: &str) -> BTreeSet<String> {
    let rows = NATIVE_DIAGNOSTICS
        .lines()
        .filter(|line| {
            line.split('|')
                .nth(1)
                .is_some_and(|cell| cell.trim() == format!("`{code}`"))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        1,
        "native diagnostics has exactly one `{code}` row"
    );
    let cause_cell = rows[0]
        .split('|')
        .nth(2)
        .and_then(|cell| cell.split(';').next())
        .expect("diagnostic row has a cause cell");
    cause_cell
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

#[trace("TC-233", "FR-322-AC-17")]
#[test]
fn tc_233_operation_catalog_is_closed_and_bound_to_both_schemas_and_the_value_lock() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package_schema = parsed_fixture(SCHEMA);
    let preimage_schema = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let lock = parsed_fixture(COMPLETE_VALUE_LOCK);
    let entries = catalog["operations"]
        .as_array()
        .expect("catalog operations are an array");
    let identities = entries
        .iter()
        .map(|entry| entry["identity"].as_str().expect("identity is a string"))
        .collect::<Vec<_>>();
    let identity_set = identities.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        identity_set.len(),
        identities.len(),
        "catalog identities are unique"
    );

    for definition in [
        "OperationIdentity",
        "OperationLaw",
        "OperationMode",
        "OperationMember",
        "LeafSegment",
        "OperationLeaf",
        "Operation",
        "Declaration",
    ] {
        assert_eq!(
            package_schema["$defs"][definition], preimage_schema["$defs"][definition],
            "{definition} is identical in the package and node-preimage schemas"
        );
    }
    let defs = &preimage_schema["$defs"];
    assert_eq!(
        string_set(&defs["OperationIdentity"]["enum"], "schema identities"),
        identity_set,
        "the schemas enumerate exactly the catalogued identities"
    );
    let roles = catalog["law_roles"]
        .as_object()
        .expect("law roles are an object");
    assert_eq!(
        string_set(
            &defs["OperationLaw"]["properties"]["role"]["enum"],
            "schema law roles"
        ),
        roles.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        "the schemas enumerate exactly the catalogued law roles"
    );
    let branch_kinds = |definition: &str| {
        defs[definition]["oneOf"]
            .as_array()
            .expect("oneOf is an array")
            .iter()
            .filter_map(|branch| {
                let kind = branch["properties"]["kind"]["const"].as_str()?;
                Some((kind, branch["properties"]["value"]["enum"].clone()))
            })
            .collect::<BTreeMap<_, _>>()
    };
    let modes = catalog["modes"].as_object().expect("modes are an object");
    assert_eq!(
        branch_kinds("OperationMode"),
        modes
            .iter()
            .map(|(kind, values)| (kind.as_str(), values.clone()))
            .collect::<BTreeMap<_, _>>(),
        "the schemas enumerate exactly the catalogued mode kinds and values"
    );
    let member_kinds = string_set(&catalog["member_kinds"], "member kinds");
    assert_eq!(
        branch_kinds("OperationMember")
            .into_keys()
            .collect::<BTreeSet<_>>(),
        member_kinds,
        "the schemas enumerate exactly the catalogued member kinds"
    );
    assert_eq!(
        string_set(
            &defs["PreimageTerm"]["oneOf"][3]["properties"]["operator"]["enum"],
            "operator classes"
        ),
        entries
            .iter()
            .map(|entry| entry["operator"].as_str().expect("operator is a string"))
            .collect::<BTreeSet<_>>(),
        "every schema operator class has a catalogued operation and no other class does"
    );

    let locked = lock["qualification_catalog"]
        .as_array()
        .expect("lock catalog is an array")
        .iter()
        .map(|entry| &entry["definition"])
        .collect::<Vec<_>>();
    for (role, references) in roles {
        let references = references.as_array().expect("role references are an array");
        assert!(!references.is_empty(), "law role {role} names a definition");
        for reference in references {
            assert!(
                locked.contains(&reference),
                "law role {role} definition {reference} is a complete-value lock definition, digest included"
            );
        }
    }

    let families = string_set(&catalog["families"], "families");
    let groups = catalog["groups"].as_object().expect("groups are an object");
    for (name, members) in groups {
        assert!(
            !families.contains(name.as_str()),
            "group {name} is not also a family"
        );
        assert!(
            string_set(members, "group members").is_subset(&families),
            "group {name} lists only families"
        );
    }
    assert_eq!(
        string_set(&groups["any_term"], "any_term"),
        families,
        "every family, including aggregate, binder and clause terms, is a term"
    );
    let position_known = |name: &str| families.contains(name) || groups.contains_key(name);
    let constraint_kinds = string_set(&catalog["constraint_kinds"], "constraint kinds");
    for (kind, pinned) in catalog["type_pinned_modes"]
        .as_object()
        .expect("type-pinned modes are an object")
    {
        assert!(
            modes.contains_key(kind),
            "pinned mode {kind} is a mode kind"
        );
        assert!(string_set(pinned, "pinned families").is_subset(&families));
    }

    for entry in entries {
        let identity = entry["identity"].as_str().expect("identity is a string");
        let operands = string_list(&entry["operands"], "operand positions");
        assert!(
            operands.iter().all(|position| position_known(position)),
            "{identity} operands"
        );
        assert!(
            entry["rest"].is_null() || entry["rest"].as_str().is_some_and(position_known),
            "{identity} rest position"
        );
        let result = entry["result"].as_str().expect("result is a string");
        let result_fits = match result.split_once(':') {
            Some((form, index)) => {
                ["operand", "kind", "inner", "return"].contains(&form)
                    && index
                        .parse::<usize>()
                        .is_ok_and(|index| index < operands.len())
            }
            None => families.contains(result) || ["member", "absence"].contains(&result),
        };
        assert!(
            result_fits,
            "{identity} result {result} is a closed result form"
        );
        assert!(
            string_set(&entry["laws"], "entry laws")
                .iter()
                .all(|role| roles.contains_key(*role)),
            "{identity} laws are law roles"
        );
        assert!(
            entry["mode"].is_null()
                || entry["mode"]
                    .as_str()
                    .is_some_and(|kind| modes.contains_key(kind)),
            "{identity} mode is a mode kind"
        );
        assert!(
            entry["member"].is_null()
                || entry["member"]
                    .as_str()
                    .is_some_and(|kind| member_kinds.contains(kind)),
            "{identity} member is a member kind"
        );
        assert!(
            [json!(null), json!("operand:0"), json!("inner:0")].contains(&entry["leaves"]),
            "{identity} leaves source is closed"
        );
        for constraint in entry["constraints"]
            .as_array()
            .expect("constraints are an array")
        {
            assert!(
                constraint["kind"]
                    .as_str()
                    .is_some_and(|kind| constraint_kinds.contains(kind)),
                "{identity} constraint kind is closed"
            );
            assert!(
                constraint["operands"]
                    .as_array()
                    .expect("constraint operands are an array")
                    .iter()
                    .all(|index| index
                        .as_u64()
                        .is_some_and(|index| (index as usize) < operands.len())),
                "{identity} constraint operands index declared positions"
            );
            assert!(
                constraint["family"].is_null()
                    || constraint["family"].as_str().is_some_and(position_known),
                "{identity} constraint family is known"
            );
        }
    }

    let entry = |identity: &str| {
        entries
            .iter()
            .find(|entry| entry["identity"] == identity)
            .unwrap_or_else(|| panic!("{identity} is catalogued"))
    };
    for identity in ["quire.op.integer.div", "quire.op.integer.rem"] {
        assert_eq!(entry(identity)["laws"], json!(["integer_division"]));
    }
    assert_eq!(entry("quire.op.integer.mod")["laws"], json!([]));
    for identity in &identities {
        let (family, operation) = identity
            .strip_prefix("quire.op.")
            .and_then(|rest| rest.split_once('.'))
            .expect("identity is quire.op.<family>.<operation>");
        if family == "ieee" {
            assert!(
                string_set(&entry(identity)["laws"], "ieee laws").contains("ieee_profile"),
                "{identity} requires the IEEE profile"
            );
            let last = operation.rsplit('.').next().unwrap_or(operation);
            assert!(
                !["eq", "ne", "lt", "le", "gt", "ge", "negate"].contains(&last),
                "{identity}: IEEE equality, ordering and negation are not catalogued (FR-148)"
            );
        }
        if family == "text" {
            assert_eq!(
                entry(identity)["laws"],
                json!(["text_profile"]),
                "{identity} laws"
            );
            assert_eq!(
                entry(identity)["mode"],
                json!("text_profile"),
                "{identity} mode"
            );
        }
    }
    for identity in [
        "quire.op.ieee.numeric_equal",
        "quire.op.ieee.total_order",
        "quire.op.ieee.bit_identical",
        "quire.op.record.project",
        "quire.op.tuple.project",
        "quire.op.collection.index",
        "quire.op.model.deref",
        "quire.op.model.dispatch_call",
    ] {
        entry(identity);
    }
    assert_eq!(
        entry("quire.op.model.all_instances")["operands"],
        json!(["population"])
    );
    assert_eq!(entry("quire.op.model.lookup")["mode"], json!("absence"));
    assert_eq!(
        entry("quire.op.model.navigate")["member"],
        json!("relationship_end")
    );
    assert_eq!(
        entry("quire.op.model.reaches")["member"],
        json!("relationship_end")
    );
    assert_eq!(entry("quire.op.record.project")["member"], json!("field"));
    assert_eq!(entry("quire.op.tuple.project")["member"], json!("position"));
    assert_eq!(
        entry("quire.op.structural.eq")["leaves"],
        json!("operand:0")
    );
    assert_eq!(
        entry("quire.op.collection.contains")["leaves"],
        json!("inner:0")
    );
}

#[trace("TC-233", "FR-322-AC-18")]
#[test]
fn tc_233_operation_vectors_key_every_application_node_of_the_published_packages() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let application_schema =
        schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "ApplicationNode");
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let operation_vectors = vectors["operation_vectors"]
        .as_array()
        .expect("operation vectors are an array");
    let names = operation_vectors
        .iter()
        .map(|vector| vector["name"].as_str().expect("vector name is a string"))
        .collect::<BTreeSet<_>>();
    let digests = operation_vectors
        .iter()
        .map(|vector| {
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        names.len(),
        operation_vectors.len(),
        "vector names are unique"
    );
    assert_eq!(
        digests.len(),
        operation_vectors.len(),
        "vector identities are distinct"
    );

    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    v2.validate(&package)
        .unwrap_or_else(|error| panic!("operation package has the closed V2 shape: {error}"));
    assert_eq!(
        derive_identity_preimage(&package),
        package["identity_preimage"]
    );
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    assert_eq!(package_declaration_refusal(&package), Ok(()));
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    for node in nodes
        .iter()
        .filter(|node| !application_terms(&node["body"]).is_empty())
    {
        assert!(
            application_node_is_valid(node, nodes),
            "{} is keyed by its application-node preimage",
            node["node_id"]["digest"]
        );
        assert!(
            digests.contains(node["node_id"]["digest"].as_str().unwrap_or_default()),
            "every published application node is a published vector"
        );
    }

    let selected = &package["lock"]["definition_selections"];
    for vector in operation_vectors {
        let name = vector["name"].as_str().expect("vector name is a string");
        let preimage = &vector["preimage"];
        application_schema
            .validate(preimage)
            .unwrap_or_else(|error| panic!("{name} preimage is closed: {error}"));
        assert_eq!(
            unqualified_jcs_sha256(preimage),
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string"),
            "{name} records its JCS SHA-256"
        );
        match node_index_by_digest(&package, vector["sha256"].as_str().unwrap_or_default()) {
            Some(index) => assert_eq!(
                application_node_preimage(&nodes[index], nodes).as_ref(),
                Some(preimage),
                "{name} is exactly the preimage of its package node"
            ),
            None => {
                let laws = &preimage["body"]["operation"]["laws"];
                assert!(
                    laws.as_array().is_some_and(|laws| laws.iter().any(|law| {
                        !selected
                            .as_array()
                            .is_some_and(|selected| selected.contains(&law["definition"]))
                    })),
                    "{name} is outside the package only because its law is unselected there"
                );
                let mut placed = package.clone();
                let index = operation_vector_node(&package, &vectors, "integer-div-floor");
                let mut node = nodes[index].clone();
                node["body"] = preimage["body"].clone();
                replace_application_node(&mut placed, index, node);
                assert_eq!(
                    placed["semantic_graph"]["nodes"][index]["node_id"]["digest"], vector["sha256"],
                    "{name} keys the node it replaces"
                );
                assert_eq!(
                    package_operations_refusal(&placed, &catalog),
                    Err(OperationRefusal::LawUnselected),
                    "{name} placed in the package refuses its unselected law"
                );
            }
        }
    }

    let floor = operation_vector_node(&package, &vectors, "integer-div-floor");
    let euclidean = operation_vector(&vectors, "integer-div-euclidean");
    let mut stale = package.clone();
    stale["semantic_graph"]["nodes"][floor]["body"] = euclidean["preimage"]["body"].clone();
    let stale_nodes = stale["semantic_graph"]["nodes"].as_array().expect("nodes");
    assert!(
        !application_node_is_valid(&stale_nodes[floor], stale_nodes),
        "a Euclidean division retaining the floor key is refused"
    );
    let mut rekeyed = package.clone();
    replace_application_node(&mut rekeyed, floor, stale_nodes[floor].clone());
    assert_ne!(
        rekeyed["package_id"], package["package_id"],
        "the division law binds the package id"
    );

    let edits: [(&str, &str, fn(&mut Value)); 4] = [
        ("integer-add", "contradictory semantic form", |node| {
            node["semantic_form"] = json!("unary")
        }),
        (
            "dispatch-call",
            "dependencies without the member declaration",
            |node| {
                let declaration = node["body"]["operation"]["member"]["declaration"].clone();
                node["dependencies"]
                    .as_array_mut()
                    .expect("dependencies are an array")
                    .retain(|dependency| *dependency != declaration);
            },
        ),
        (
            "integer-add",
            "recursion group on a non-recursive node",
            |node| node["recursion_group"] = json!("stray-group"),
        ),
        ("record-project", "extra dependency", |node| {
            let semantic_type = node["semantic_type"].clone();
            node["dependencies"]
                .as_array_mut()
                .expect("dependencies are an array")
                .push(semantic_type)
        }),
    ];
    for (name, label, edit) in edits {
        let index = operation_vector_node(&package, &vectors, name);
        let mut edited_nodes = nodes.clone();
        edit(&mut edited_nodes[index]);
        assert_ne!(
            edited_nodes[index], nodes[index],
            "{name} {label} edits the node"
        );
        assert!(
            !application_node_is_valid(&edited_nodes[index], &edited_nodes),
            "{name} with a {label} is refused"
        );
        let mut rekeyed_nodes = edited_nodes.clone();
        if let Some(preimage) = application_node_preimage(&rekeyed_nodes[index], &rekeyed_nodes) {
            rekeyed_nodes[index]["node_id"]["digest"] = json!(unqualified_jcs_sha256(&preimage));
        }
        assert!(
            label == "recursion group on a non-recursive node"
                || !application_node_is_valid(&rekeyed_nodes[index], &rekeyed_nodes),
            "{name} with a {label} is refused even under a recomputed key"
        );
    }
    let mut without_operation = package.clone();
    let add = operation_vector_node(&package, &vectors, "integer-add");
    without_operation["semantic_graph"]["nodes"][add]["body"]
        .as_object_mut()
        .expect("application body is an object")
        .remove("operation");
    assert!(
        !v2.is_valid(&without_operation),
        "an application without an operation fails the V2 schema"
    );

    let families = parsed_fixture(POSITIVE_ALL_FAMILIES);
    v2.validate(&families)
        .unwrap_or_else(|error| panic!("all-families package is closed: {error}"));
    assert_eq!(package_operations_refusal(&families, &catalog), Ok(()));
    let family_nodes = families["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let call = family_nodes
        .iter()
        .position(|node| {
            node["node_tag"] == "function" && !application_terms(&node["body"]).is_empty()
        })
        .expect("all-families publishes a function whose body is a call");
    for node in family_nodes
        .iter()
        .filter(|node| !application_terms(&node["body"]).is_empty())
    {
        assert!(
            application_node_is_valid(node, family_nodes),
            "all-families application nodes are keyed"
        );
    }
    let callee = &family_nodes[call]["body"]["arguments"][0]["target"];
    assert!(
        family_nodes
            .iter()
            .any(|node| node["node_id"] == *callee && node["node_tag"] == "function"),
        "the call names a published function node"
    );
    assert_eq!(family_nodes[call]["dependencies"], json!([callee]));
    let mut dispatch = family_nodes[call].clone();
    dispatch["body"]["operation"]["identity"] = json!("quire.op.model.dispatch_call");
    assert_ne!(
        application_node_preimage(&dispatch, family_nodes)
            .map(|preimage| unqualified_jcs_sha256(&preimage)),
        family_nodes[call]["node_id"]["digest"]
            .as_str()
            .map(str::to_owned),
        "swapping call for dispatch_call in the function body changes its node id"
    );
    let mut dispatched = families.clone();
    replace_application_node(&mut dispatched, call, dispatch);
    assert_eq!(
        package_operations_refusal(&dispatched, &catalog),
        Err(OperationRefusal::MemberMismatch),
        "dispatch_call without its operation member is refused"
    );
}

#[trace("TC-233", "FR-322-AC-19")]
#[test]
fn tc_233_operation_mutations_rekey_or_refuse_with_their_recorded_cause() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let mutations = &vectors["operation_mutations"];
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let inventory = refusing_check_inventory(mutations);
    assert_eq!(
        inventory
            .iter()
            .filter(|(_, check)| *check == "schema")
            .count(),
        1,
        "only unknown-operation is refused by the closed schema"
    );
    let causes = mutations
        .as_array()
        .expect("mutations are an array")
        .iter()
        .filter_map(|mutation| mutation["expected_cause"].as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        causes,
        OperationRefusal::ALL
            .into_iter()
            .map(OperationRefusal::cause)
            .collect::<BTreeSet<_>>(),
        "the mutations exercise every operation refusal cause"
    );

    for mutation in mutations.as_array().expect("mutations are an array") {
        let name = mutation["name"]
            .as_str()
            .expect("mutation name is a string");
        let base = mutation["base"]
            .as_str()
            .expect("mutation base is a string");
        let index = operation_vector_node(&package, &vectors, base);
        assert_eq!(
            package["semantic_graph"]["nodes"][index]["node_id"]["digest"],
            mutation["retained_sha256"],
            "{name} retains exactly its base key"
        );
        let mut node = package["semantic_graph"]["nodes"][index].clone();
        for patch in mutation["patch"]
            .as_array()
            .expect("mutation patch is an array")
        {
            apply_authored_patch(&mut node, patch);
        }
        let mut stale = package.clone();
        stale["semantic_graph"]["nodes"][index] = node.clone();
        let stale_nodes = stale["semantic_graph"]["nodes"].as_array().expect("nodes");
        assert!(
            !application_node_is_valid(&stale_nodes[index], stale_nodes),
            "{name} never keeps its stale key"
        );
        let mut candidate = package.clone();
        replace_application_node(&mut candidate, index, node);
        let candidate_node = &candidate["semantic_graph"]["nodes"][index];
        match mutation["kind"].as_str() {
            Some("stale_key") => {
                let counterpart = operation_vector(
                    &vectors,
                    mutation["rekeyed_as"].as_str().expect("rekeyed_as"),
                );
                assert_eq!(
                    candidate_node["node_id"]["digest"], counterpart["sha256"],
                    "{name} rekeys to exactly its counterpart vector"
                );
                assert_ne!(
                    candidate["package_id"], package["package_id"],
                    "{name} changes the package id"
                );
                let unselected = candidate_node["body"]["operation"]["laws"]
                    .as_array()
                    .expect("laws are an array")
                    .iter()
                    .any(|law| {
                        !package["lock"]["definition_selections"]
                            .as_array()
                            .expect("selections are an array")
                            .contains(&law["definition"])
                    });
                assert_eq!(
                    package_operations_refusal(&candidate, &catalog),
                    if unselected {
                        Err(OperationRefusal::LawUnselected)
                    } else {
                        Ok(())
                    },
                    "{name} rekeyed is admitted exactly when its laws are selected"
                );
                assert_eq!(mutation["expected_code"], "invalid_package");
            }
            Some("law_unselected") => {
                assert_eq!(
                    package_operations_refusal(&candidate, &catalog),
                    Err(OperationRefusal::LawUnselected),
                    "{name} refuses while the lock selects only the replaced law"
                );
                let law = candidate_node["body"]["operation"]["laws"][0]["definition"].clone();
                let mut relocked = candidate.clone();
                let mut selections = relocked["lock"]["definition_selections"]
                    .as_array()
                    .expect("selections are an array")
                    .clone();
                selections.push(law);
                replace_selection_consistently(
                    &mut relocked,
                    "/definition_selections",
                    json!(selections),
                );
                assert_eq!(
                    package_operations_refusal(&relocked, &catalog),
                    Ok(()),
                    "{name} is admitted once the lock also selects its law"
                );
                let mut stale_lock = relocked.clone();
                replace_selection_consistently(
                    &mut stale_lock,
                    "/definition_selections",
                    package["lock"]["definition_selections"].clone(),
                );
                assert_eq!(
                    package_operations_refusal(&stale_lock, &catalog),
                    Err(OperationRefusal::LawUnselected),
                    "{name} refuses again under the stale division-law selection"
                );
                assert_eq!(
                    OperationRefusal::from_cause(
                        mutation["expected_cause"].as_str().unwrap_or_default()
                    ),
                    Some(OperationRefusal::LawUnselected)
                );
            }
            Some("semantic") => {
                let expected = OperationRefusal::from_cause(
                    mutation["expected_cause"].as_str().expect("expected cause"),
                )
                .unwrap_or_else(|| panic!("{name} cause is closed"));
                assert_eq!(
                    mutation["expected_code"].as_str(),
                    Some(expected.code()),
                    "{name} records the code of its cause"
                );
                let refusal = package_operations_refusal(&candidate, &catalog);
                assert_refused_by_named_check(
                    name,
                    &mutation["refused_by"],
                    v2.is_valid(&candidate),
                    refusal.is_ok(),
                );
                assert_eq!(
                    refusal,
                    Err(expected),
                    "{name} refuses with exactly its recorded cause"
                );
            }
            other => panic!("{name} has an unknown mutation kind {other:?}"),
        }
    }
}

#[trace("TC-233", "FR-272-AC-7")]
#[test]
fn tc_233_package_refusals_report_the_first_cause_in_node_then_pre_order() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let floor = operation_vector_node(&package, &vectors, "integer-div-floor");
    let float = operation_vector_node(&package, &vectors, "float64-add-nearest-even");
    let digest =
        |index: usize| package["semantic_graph"]["nodes"][index]["node_id"]["digest"].clone();
    let (low, high) = if digest(floor).as_str() < digest(float).as_str() {
        (floor, float)
    } else {
        (float, floor)
    };
    let break_node = |candidate: &mut Value, index: usize, unknown: bool| {
        let body = &mut candidate["semantic_graph"]["nodes"][index]["body"];
        if unknown {
            body["operation"]["identity"] = json!("quire.op.integer.pow");
        } else {
            body["operator"] = json!("unary");
        }
    };
    for (low_unknown, expected) in [
        (true, OperationRefusal::UnknownOperation),
        (false, OperationRefusal::ClassMismatch),
    ] {
        let mut candidate = package.clone();
        break_node(&mut candidate, low, low_unknown);
        break_node(&mut candidate, high, !low_unknown);
        assert_eq!(
            package_operations_refusal(&candidate, &catalog),
            Err(expected),
            "the node with the lower node-id digest is reported first"
        );
    }

    let add = operation_vector_node(&package, &vectors, "integer-add");
    let mul = operation_vector(&vectors, "integer-mul");
    let text =
        operation_vector(&vectors, "text-eq-nfc")["preimage"]["body"]["arguments"][0].clone();
    let floor_law =
        operation_vector(&vectors, "integer-div-floor")["preimage"]["body"]["operation"]["laws"]
            .clone();
    let mut inner = mul["preimage"]["body"].clone();
    inner["operation"]["laws"] = floor_law;
    let nested = |outer_mode: Value| {
        let mut candidate = package.clone();
        let body = &mut candidate["semantic_graph"]["nodes"][add]["body"];
        body["operation"]["mode"] = outer_mode;
        body["arguments"][0] = inner.clone();
        body["arguments"][1] = text.clone();
        candidate
    };
    assert_eq!(
        package_operations_refusal(
            &nested(json!({"kind": "rounding", "value": "exact"})),
            &catalog
        ),
        Err(OperationRefusal::ModeMismatch),
        "an application's own operation causes precede its argument applications"
    );
    assert_eq!(
        package_operations_refusal(&nested(json!(null)), &catalog),
        Err(OperationRefusal::LawMismatch),
        "an argument application's refusal precedes the enclosing operator-ineligible cause"
    );
}

#[trace("TC-233", "FR-272-AC-7")]
#[test]
fn tc_233_operation_and_declaration_causes_pair_with_one_catalogued_code() {
    let diagnostic = schema_for_def(&parsed_fixture(SCHEMA), "Diagnostic");
    let codes = ["invalid_package", "ill_typed", "ambiguous_declaration"];
    let rows = codes
        .iter()
        .map(|code| (*code, native_diagnostic_row_causes(code)))
        .collect::<BTreeMap<_, _>>();
    let pairs = OperationRefusal::ALL
        .into_iter()
        .map(|refusal| (refusal.cause(), refusal.code()))
        .chain(
            DeclarationRefusal::ALL
                .into_iter()
                .map(|refusal| (refusal.cause(), refusal.code())),
        )
        .collect::<Vec<_>>();
    assert_eq!(
        pairs
            .iter()
            .map(|(cause, _)| *cause)
            .collect::<BTreeSet<_>>()
            .len(),
        pairs.len(),
        "no two package refusals share a cause variant"
    );
    for (cause, code) in pairs {
        for (row, listed) in &rows {
            assert_eq!(
                listed.contains(cause),
                row == &code,
                "native diagnostics lists {cause} in the `{row}` row exactly when it is its code"
            );
            let entry = json!({"stage": "package_read", "code": row, "cause_tag": cause, "details": [], "loci": []});
            assert_eq!(
                diagnostic.is_valid(&entry),
                row == &code,
                "the Diagnostic schema pairs {cause} with `{row}` exactly when it is its code"
            );
        }
    }
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    for mutation in vectors["operation_mutations"]
        .as_array()
        .expect("mutations")
    {
        let expected_code = mutation["expected_code"].as_str().expect("expected code");
        let code = mutation["expected_cause"]
            .as_str()
            .map_or(Some("invalid_package"), |cause| {
                OperationRefusal::from_cause(cause).map(OperationRefusal::code)
            });
        assert_eq!(
            Some(expected_code),
            code,
            "{} records its cause's code",
            mutation["name"]
        );
    }
}

/// The first application term of a body in pre-order, for in-place edits.
fn first_application_mut(term: &mut Value) -> Option<&mut Value> {
    if term.get("term").is_some_and(|tag| tag == "application") {
        return Some(term);
    }
    match term {
        Value::Object(members) => members.values_mut().find_map(first_application_mut),
        Value::Array(items) => items.iter_mut().find_map(first_application_mut),
        _ => None,
    }
}

fn published_v2_packages() -> [(&'static str, Value); 4] {
    [
        (
            "operation-identities",
            parsed_fixture(POSITIVE_OPERATION_IDENTITIES),
        ),
        ("all-families", parsed_fixture(POSITIVE_ALL_FAMILIES)),
        (
            "clause-operations",
            parsed_fixture(POSITIVE_CLAUSE_OPERATIONS),
        ),
        (
            "nominal-identities",
            parsed_fixture(POSITIVE_NOMINAL_IDENTITIES),
        ),
    ]
}

#[trace("TC-233", "interface-004-AC-1")]
#[test]
fn tc_233_every_v2_application_names_one_catalogued_operation_the_v1_wire_cannot_carry() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let v1 = jsonschema::validator_for(&parsed_fixture(SCHEMA_V1)).expect("V1 schema is valid");
    let identities = catalog["operations"]
        .as_array()
        .expect("catalog operations are an array")
        .iter()
        .map(|entry| &entry["identity"])
        .collect::<Vec<_>>();
    for (label, package) in published_v2_packages() {
        v2.validate(&package)
            .unwrap_or_else(|error| panic!("{label} is a closed V2 package: {error}"));
        assert_eq!(
            package_operations_refusal(&package, &catalog),
            Ok(()),
            "{label}: every application's operation, laws, mode, member and leaves are admitted"
        );
        for node in package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
        {
            for term in application_terms(&node["body"]) {
                assert_eq!(
                    identities
                        .iter()
                        .filter(|identity| ***identity == term["operation"]["identity"])
                        .count(),
                    1,
                    "{label}: each application names exactly one catalogued identity"
                );
                for member in ["laws", "mode", "member", "leaves"] {
                    assert!(
                        term["operation"].get(member).is_some(),
                        "{label}: the operation states its {member} explicitly"
                    );
                }
            }
        }
    }

    let mut v1_package = parsed_fixture(POSITIVE_ALL_FAMILIES_V1);
    v1.validate(&v1_package)
        .unwrap_or_else(|error| panic!("the V1 all-families package is valid: {error}"));
    let operation = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .find_map(|node| {
            application_terms(&node["body"])
                .first()
                .map(|term| term["operation"].clone())
        })
        .expect("the V2 all-families package has an application");
    let node = v1_package["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("V1 nodes are an array")
        .iter_mut()
        .find_map(|node| first_application_mut(&mut node["body"]))
        .expect("the V1 all-families package has an application");
    node["operation"] = operation;
    assert!(
        !v1.is_valid(&v1_package),
        "the frozen V1 wire refuses an operation member"
    );
}

#[trace("TC-233", "interface-004-AC-2")]
#[test]
fn tc_233_operation_identity_law_and_mode_bind_node_and_package_ids() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let euclidean = operation_vector(&vectors, "integer-div-euclidean")["preimage"]["body"]["operation"]["laws"].clone();
    // Each edit with the reader outcome of its rekeyed package: a new identity
    // is admitted, an unlocked law refuses, and an absence mode that disagrees
    // with the retained result type is ineligible rather than silently retyped.
    let edits: [(&str, &str, Value, Result<(), OperationRefusal>); 3] = [
        (
            "integer-add",
            "/body/operation/identity",
            json!("quire.op.integer.sub"),
            Ok(()),
        ),
        (
            "integer-div-floor",
            "/body/operation/laws",
            euclidean,
            Err(OperationRefusal::LawUnselected),
        ),
        (
            "model-lookup-undefined",
            "/body/operation/mode/value",
            json!("empty"),
            Err(OperationRefusal::OperatorIneligible),
        ),
    ];
    for (name, pointer, replacement, expected) in edits {
        let index = operation_vector_node(&package, &vectors, name);
        let mut node = package["semantic_graph"]["nodes"][index].clone();
        *node
            .pointer_mut(pointer)
            .unwrap_or_else(|| panic!("{name} has {pointer}")) = replacement;
        let mut retained = package.clone();
        retained["semantic_graph"]["nodes"][index] = node.clone();
        let retained_nodes = retained["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes");
        assert!(
            !application_node_is_valid(&retained_nodes[index], retained_nodes),
            "{name} edited at {pointer} cannot retain its old key"
        );
        let mut rekeyed = package.clone();
        replace_application_node(&mut rekeyed, index, node);
        assert_ne!(
            rekeyed["semantic_graph"]["nodes"][index]["node_id"],
            package["semantic_graph"]["nodes"][index]["node_id"],
            "{name} edited at {pointer} changes its node id"
        );
        assert_ne!(
            rekeyed["package_id"], package["package_id"],
            "{name} edited at {pointer} changes the package id"
        );
        assert_eq!(
            package_operations_refusal(&rekeyed, &catalog),
            expected,
            "{name} edited at {pointer} is read with exactly its recorded outcome"
        );
    }
}

#[trace("TC-233", "interface-004-AC-3")]
#[test]
fn tc_233_underivable_relinked_operations_refuse_after_stale_inputs_and_before_target() {
    let vectors = parsed_fixture(MIGRATION_CORRESPONDENCE_VECTORS);
    let base = vectors["positives"]
        .as_array()
        .expect("positives are an array")
        .iter()
        .find(|positive| positive["name"] == "clauses-relinked-rekeyed")
        .expect("the clause migration positive is published");
    let source = migration_package(base["source_package"].as_str().expect("source path"));
    let target = migration_package(base["target_package"].as_str().expect("target path"));
    let correspondence = &base["outcome"]["correspondence"];
    assert_eq!(classify_migration(correspondence, &source, &target), Ok(()));
    let mut stale_inputs = correspondence.clone();
    stale_inputs["reconstruction_inputs"]["sources"][0]["digest"] = json!("3".repeat(64));

    let target_nodes = target["semantic_graph"]["nodes"].as_array().expect("nodes");
    let application_ordinals = (0..target_nodes.len())
        .filter(|ordinal| !application_terms(&target_nodes[*ordinal]["body"]).is_empty())
        .collect::<Vec<_>>();
    assert!(
        !application_ordinals.is_empty(),
        "the clause target re-links applications"
    );
    for ordinal in application_ordinals {
        let mut underivable = target.clone();
        let term =
            first_application_mut(&mut underivable["semantic_graph"]["nodes"][ordinal]["body"])
                .expect("the node bears an application");
        term["operation"]["mode"] = json!({"kind": "rounding", "value": "exact"});
        let unresolved = Err((
            MigrationRefusalCode::OperationUnresolved,
            json!({"input": "node_correspondences", "ordinal": ordinal}),
        ));
        assert_eq!(
            classify_migration(correspondence, &source, &underivable),
            unresolved,
            "ordinal {ordinal}: a selection V1 cannot derive refuses as unresolved, not as a target mismatch"
        );
        let mut foreign_target = correspondence.clone();
        foreign_target["target_package_id"]["digest"] = json!("0".repeat(64));
        assert_eq!(
            classify_migration(&foreign_target, &source, &underivable),
            unresolved,
            "ordinal {ordinal}: the unresolved operation precedes target comparison"
        );
        assert_eq!(
            classify_migration(&stale_inputs, &source, &underivable),
            Err((
                MigrationRefusalCode::InputStale,
                json!({"input": "sources", "authority": "agent-ix", "identity": "example"}),
            )),
            "ordinal {ordinal}: stale inputs precede the unresolved operation"
        );
    }
}

#[trace("TC-233", "FR-322-AC-21")]
#[test]
fn tc_233_declaration_members_name_every_declaring_node_exactly_once() {
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    for (label, package) in published_v2_packages() {
        assert_eq!(
            package_declaration_refusal(&package),
            Ok(()),
            "{label} declarations are admitted"
        );
        for node in package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
        {
            assert_eq!(
                node.get("declaration").is_some(),
                node_declares(node),
                "{label}: {} carries a declaration exactly when it declares",
                node["node_id"]["digest"]
            );
        }
    }

    let families = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let index_of = |package: &Value, tag: &str| {
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| node["node_tag"] == tag)
            .unwrap_or_else(|| panic!("a {tag} node is published"))
    };
    let mut undeclared = families.clone();
    let scalar = index_of(&families, "scalar_type");
    for prefix in [
        "/semantic_graph/nodes",
        "/identity_preimage/identity_projection",
    ] {
        undeclared
            .pointer_mut(&format!("{prefix}/{scalar}"))
            .and_then(Value::as_object_mut)
            .expect("node is an object")
            .remove("declaration");
    }
    assert!(
        !v2.is_valid(&undeclared),
        "a declaring node without its declaration fails the schema"
    );
    let mut stray = families.clone();
    let expression = index_of(&families, "expression");
    for prefix in [
        "/semantic_graph/nodes",
        "/identity_preimage/identity_projection",
    ] {
        stray
            .pointer_mut(&format!("{prefix}/{expression}"))
            .and_then(Value::as_object_mut)
            .expect("node is an object")
            .insert(
                "declaration".to_owned(),
                json!({"qualified_name": ["Example", "stray"]}),
            );
    }
    refresh_package_id(&mut stray);
    assert!(
        !v2.is_valid(&stray),
        "an expression carrying a declaration fails the schema"
    );

    let function = |name: &str| {
        families["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| node["declaration"]["qualified_name"] == json!(["Example", name]))
            .unwrap_or_else(|| panic!("{name} is declared"))
    };
    let call = function("pure_function_ffff");
    let callee = function("pure_function_8080");
    let mut ambiguous = families.clone();
    let mut node = families["semantic_graph"]["nodes"][call].clone();
    node["declaration"] = families["semantic_graph"]["nodes"][callee]["declaration"].clone();
    replace_application_node(&mut ambiguous, call, node);
    assert_ne!(
        ambiguous["semantic_graph"]["nodes"][call]["node_id"],
        families["semantic_graph"]["nodes"][call]["node_id"],
        "the declaration member is part of the application-node preimage"
    );
    assert_ne!(ambiguous["package_id"], families["package_id"]);
    v2.validate(&ambiguous)
        .unwrap_or_else(|error| panic!("a duplicated name is schema-valid: {error}"));
    let later = [call, callee]
        .into_iter()
        .map(|index| ambiguous["semantic_graph"]["nodes"][index]["node_id"].clone())
        .max_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()))
        .expect("two declaring nodes");
    assert_eq!(
        package_declaration_refusal(&ambiguous),
        Err((DeclarationRefusal::AmbiguousName, later)),
        "the second declaring node in node-id order refuses as ambiguous"
    );

    let nominal = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);
    let nominal_nodes = nominal["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let named = |name: &str| {
        nominal_nodes
            .iter()
            .position(|node| node["declaration"]["qualified_name"] == json!(["Example", name]))
            .unwrap_or_else(|| panic!("{name} is declared"))
    };
    for (renamed, replacement) in [("Length", "Other"), ("Length", "Status")] {
        let index = named(renamed);
        let mut mismatched = nominal.clone();
        replace_node_field_in_graph_and_projection(
            &mut mismatched,
            index,
            "/declaration/qualified_name",
            json!(["Example", replacement]),
        );
        refresh_package_id(&mut mismatched);
        assert_ne!(
            mismatched["package_id"], nominal["package_id"],
            "the declaration binds the package id"
        );
        v2.validate(&mismatched).unwrap_or_else(|error| {
            panic!("a renamed nominal declaration is schema-valid: {error}")
        });
        assert_eq!(
            package_declaration_refusal(&mismatched),
            Err((
                DeclarationRefusal::NominalMismatch,
                nominal_nodes[index]["node_id"].clone()
            )),
            "{renamed} declared as {replacement} disagrees with its nominal qualified declaration, \
             which precedes any ambiguity"
        );
    }
}
