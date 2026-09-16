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
}

impl MigrationRefusalCode {
    const ALL: [Self; 5] = [
        Self::InputMissing,
        Self::InputStale,
        Self::InputAmbiguous,
        Self::TargetIncompatible,
        Self::ByteOnlyReconstruction,
    ];

    fn code(self) -> &'static str {
        match self {
            Self::InputMissing => "migration_input_missing",
            Self::InputStale => "migration_input_stale",
            Self::InputAmbiguous => "migration_input_ambiguous",
            Self::TargetIncompatible => "migration_target_incompatible",
            Self::ByteOnlyReconstruction => "migration_byte_only_reconstruction",
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
/// absent), ambiguous, stale, then target incompatibility.
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
            ("all-families-relinked-identical", "relinked-identical"),
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
        let source = migration_package(base["source_package"].as_str().expect("source path"));
        let target = migration_package(base["target_package"].as_str().expect("target path"));
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
            | MigrationRefusalCode::TargetIncompatible => {
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
