// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! I04 checked-package V2 and complete-value qualification vectors.

use ix_trace_rs::trace;
use num_bigint::BigInt;
use num_integer::Integer;
use quire_specification::{
    COMPLETE_VALUE_LOCK, COMPLETE_VALUE_SELECTION_VECTORS, NATIVE_DIAGNOSTICS,
};
use quire_verification_contracts::jcs_sha256;
use quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const NODE_PREIMAGE_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/node-identity-preimage.schema.json");
const NODE_IDENTITY_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/node-identity-vectors.json");
/// The catalog lives only in `quire-verification-contracts`; it is read from there.
const OPERATION_CATALOG: &str = CHECKED_OPERATION_CATALOG_V1;
const POSITIVE_OPERATION_IDENTITIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-operation-identities.json");
const COMPOUND_UNIT_SCHEMA: &str =
    include_str!("../proposals/quire-v1/definitions/value-compound-unit.schema.json");
const COMPOUND_UNIT_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/value-compound-unit-vectors.json");
const POSITIVE_ALL_FAMILIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-all-families.json");
const POSITIVE_TIMED_MODEL_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-timed-model-nodes.json");
const POSITIVE_TICK_TIMED_MODEL_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-tick-timed-model-nodes.json");
const TIMED_SLOT_SELECTOR_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/timed-slot-selector-vectors.json");
const TIMED_MODEL_NODE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/timed-model-node-vectors.json");
const FR322_BODY_STEPS: &str = include_str!("../spec/objects/interfaces/FR-322-checked-package-artifact.md");
const FR464_PROTOCOL_NODES: &str = include_str!("../spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md");
const TC233_BODY_STEP_VECTOR: &str = include_str!("../spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md");
const TC946_PROTOCOL_NODE_FAMILY: &str = include_str!("../spec/test-cases/TC-946-checked-package-protocol-node-family.md");
const TC947_PROTOCOL_NODE_FAMILY: &str = include_str!("../spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md");
const TC948_PROTOCOL_NODE_FAMILY: &str = include_str!("../spec/test-cases/TC-948-checked-package-protocol-control-bodies.md");
const PROTOCOL_NODE_VECTORS: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-vectors.json");
const POSITIVE_NOMINAL_IDENTITIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-nominal-identities.json");
const POSITIVE_CLAUSE_OPERATIONS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-clause-operations.json");
const POSITIVE_STATE_TRANSITION: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-state-transition.json");
const POSITIVE_CONTROL_OPERATIONS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-control-operations.json");
const POSITIVE_EXTENSION_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-extension-nodes.json");
const EXTENSION_DEFINITIONS: &str =
    include_str!("../proposals/checked-package-v2/extension-definitions.json");
const EXTENSION_DEFINITIONS_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/extension-definitions.schema.json");
const EXTENSION_DEFINITION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/extension-definition-vectors.json");
const POSITIVE_UNION_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-union-nodes.json");
const POSITIVE_RECURSIVE_RECORDS: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-recursive-records.json");
const MODEL_MEMBER_TYPE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-member-type-vectors.json");
const ADVERSE: &str = include_str!("../proposals/checked-package-v2/fixtures/adverse.json");
const DEPENDENCY_SELECTION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/dependency-selection-vectors.json");
const DEPENDENCY_CALL_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/dependency-call-vectors.json");
const POSITIVE_DEPENDENCY_LIBRARY: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-dependency-library.json");
const POSITIVE_DEPENDENCY_CALL: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-dependency-call.json");
const REQUIRED_NODE_TAGS: [&str; 15] = [
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
    "extension",
    "hybrid",
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

/// A `KeyOrdinal`: a JSON integer, or a canonical decimal string beyond the
/// safe integer range.
fn key_ordinal(value: &Value) -> Option<BigInt> {
    value.as_u64().map(BigInt::from).or_else(|| integer(value))
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
        Some("quire.dimension-equivalence-node/v1") => {
            let Some(ids) = preimage["dimension_node_ids"].as_array() else {
                return false;
            };
            let digests = ids
                .iter()
                .map(|id| id["digest"].as_str())
                .collect::<Vec<_>>();
            digests.len() == 2
                && digests[0].is_some()
                && digests[0] < digests[1]
                && root_scale(preimage).is_some()
                && digests.iter().all(|wanted| {
                    wanted.is_some_and(|wanted| {
                        vector_for_digest(vectors, wanted).is_some_and(|vector| {
                            vector["preimage"]["version"].as_str()
                                == Some("quire.dimension-node/v1")
                        })
                    })
                })
        }
        _ => false,
    }
}

/// The base classes of the dimension-equivalence vectors: the least digest of each
/// connected component of the pairs whose two nodes are both base dimensions, by node.
fn base_class_representatives(vectors: &[Value]) -> BTreeMap<String, String> {
    let is_base = |digest: &str| {
        vector_for_digest(vectors, digest).is_some_and(|vector| {
            vector["preimage"]["terms"]
                .as_array()
                .is_some_and(Vec::is_empty)
        })
    };
    let mut classes: BTreeMap<String, String> = BTreeMap::new();
    let pairs = vectors
        .iter()
        .filter(|vector| {
            vector["preimage"]["version"].as_str() == Some("quire.dimension-equivalence-node/v1")
        })
        .map(|vector| {
            let ids = vector["preimage"]["dimension_node_ids"]
                .as_array()
                .expect("equivalence ids are an array");
            (
                ids[0]["digest"].as_str().unwrap_or_default().to_string(),
                ids[1]["digest"].as_str().unwrap_or_default().to_string(),
            )
        })
        .filter(|(left, right)| is_base(left) && is_base(right))
        .collect::<Vec<_>>();
    for (left, right) in &pairs {
        for node in [left, right] {
            classes.entry(node.clone()).or_insert_with(|| node.clone());
        }
    }
    loop {
        let mut changed = false;
        for (left, right) in &pairs {
            let least = classes[left].clone().min(classes[right].clone());
            for node in [left, right] {
                if classes[node] != least {
                    classes.insert(node.clone(), least.clone());
                    changed = true;
                }
            }
        }
        if !changed {
            return classes;
        }
    }
}

/// The normalized map of a dimension vector over base class representatives.
fn class_normalized_map(
    digest: &str,
    vectors: &[Value],
    classes: &BTreeMap<String, String>,
) -> BTreeMap<String, BigInt> {
    let mut map = BTreeMap::new();
    let preimage = &vector_for_digest(vectors, digest).expect("dimension vector exists")["preimage"];
    let terms = preimage["terms"].as_array().expect("terms are an array");
    if terms.is_empty() {
        let representative = classes.get(digest).cloned().unwrap_or(digest.to_string());
        map.insert(representative, BigInt::from(1));
        return map;
    }
    for term in terms {
        let inner = class_normalized_map(
            term["dimension_node_id"]["digest"].as_str().unwrap_or_default(),
            vectors,
            classes,
        );
        let exponent = integer(&term["exponent"]).expect("exponent is an integer");
        for (base, power) in inner {
            *map.entry(base).or_insert_with(|| BigInt::from(0)) += power * &exponent;
        }
    }
    map.retain(|_, power| *power != BigInt::from(0));
    map
}

/// An exact rational as a reduced numerator and a positive denominator.
type Ratio = (BigInt, BigInt);

fn ratio_new(numerator: BigInt, denominator: BigInt) -> Ratio {
    let divisor = numerator.gcd(&denominator);
    (numerator / &divisor, denominator / divisor)
}

fn ratio_mul(left: &Ratio, right: &Ratio) -> Ratio {
    ratio_new(&left.0 * &right.0, &left.1 * &right.1)
}

fn ratio_inverse(value: &Ratio) -> Ratio {
    (value.1.clone(), value.0.clone())
}

fn ratio_pow(value: &Ratio, exponent: i32) -> Ratio {
    let base = if exponent < 0 { ratio_inverse(value) } else { value.clone() };
    (0..exponent.unsigned_abs()).fold((BigInt::from(1), BigInt::from(1)), |acc, _| {
        ratio_mul(&acc, &base)
    })
}

/// The `root_scale` of an equivalence preimage: a canonical positive rational.
fn root_scale(preimage: &Value) -> Option<Ratio> {
    let scale = &preimage["root_scale"];
    if !rational_is_canonical(scale) {
        return None;
    }
    let (numerator, denominator) = (integer(&scale["numerator"])?, integer(&scale["denominator"])?);
    (numerator > BigInt::from(0)).then_some((numerator, denominator))
}

/// The size of each base-pair member's canonical root against its component's first
/// member, over the pairs whose two nodes are both base dimensions. A pair states that one
/// root of its first node is `root_scale` roots of its second, so the first node's root is
/// `root_scale` times as large. `None` when the ratios around some cycle do not multiply
/// to exactly one.
fn base_root_sizes(
    pairs: &[(String, String, Ratio)],
    is_base: &dyn Fn(&str) -> bool,
) -> Option<BTreeMap<String, Ratio>> {
    let base_pairs = pairs
        .iter()
        .filter(|(left, right, _)| is_base(left) && is_base(right))
        .collect::<Vec<_>>();
    let mut sizes: BTreeMap<String, Ratio> = BTreeMap::new();
    for (start, _, _) in &base_pairs {
        if sizes.contains_key(start) {
            continue;
        }
        sizes.insert(start.clone(), (BigInt::from(1), BigInt::from(1)));
        let mut changed = true;
        while changed {
            changed = false;
            for (left, right, scale) in &base_pairs {
                match (sizes.get(left).cloned(), sizes.get(right).cloned()) {
                    (Some(known), None) => {
                        sizes.insert(right.clone(), ratio_mul(&known, &ratio_inverse(scale)));
                        changed = true;
                    }
                    (None, Some(known)) => {
                        sizes.insert(left.clone(), ratio_mul(&known, scale));
                        changed = true;
                    }
                    _ => {}
                }
            }
        }
    }
    base_pairs
        .iter()
        .all(|(left, right, scale)| sizes[left] == ratio_mul(scale, &sizes[right]))
        .then_some(sizes)
}

/// The size of a dimension's canonical root over its base dimensions: the product of the
/// sizes of its base terms at their exponents (a base dimension outside every pair is 1).
fn expanded_root_size(
    digest: &str,
    vectors: &[Value],
    sizes: &BTreeMap<String, Ratio>,
) -> Ratio {
    let preimage = &vector_for_digest(vectors, digest).expect("dimension vector exists")["preimage"];
    let terms = preimage["terms"].as_array().expect("terms are an array");
    if terms.is_empty() {
        return sizes
            .get(digest)
            .cloned()
            .unwrap_or((BigInt::from(1), BigInt::from(1)));
    }
    terms.iter().fold((BigInt::from(1), BigInt::from(1)), |acc, term| {
        let inner = expanded_root_size(
            term["dimension_node_id"]["digest"].as_str().unwrap_or_default(),
            vectors,
            sizes,
        );
        let exponent = integer(&term["exponent"])
            .and_then(|exponent| exponent.to_string().parse::<i32>().ok())
            .expect("exponent is a small integer");
        ratio_mul(&acc, &ratio_pow(&inner, exponent))
    })
}

/// FR-142 "Dimension equivalence" over the equivalence vectors: no pair of one node, no
/// repeated pair, a canonical positive `root_scale` on every pair, ratios that multiply to
/// one around every cycle of base pairs, and for a pair that is not two base dimensions
/// equal normalized maps under the base classes and a `root_scale` equal to the ratio the
/// base roots imply.
fn dimension_equivalence_pairs_are_admissible(vectors: &[Value]) -> bool {
    let classes = base_class_representatives(vectors);
    let is_base = |digest: &str| {
        vector_for_digest(vectors, digest).is_some_and(|vector| {
            vector["preimage"]["terms"]
                .as_array()
                .is_some_and(Vec::is_empty)
        })
    };
    let mut seen = BTreeSet::new();
    let mut pairs = Vec::new();
    for vector in vectors.iter().filter(|vector| {
        vector["preimage"]["version"].as_str() == Some("quire.dimension-equivalence-node/v1")
    }) {
        let ids = vector["preimage"]["dimension_node_ids"]
            .as_array()
            .expect("equivalence ids are an array");
        let left = ids[0]["digest"].as_str().unwrap_or_default();
        let right = ids[1]["digest"].as_str().unwrap_or_default();
        let Some(scale) = root_scale(&vector["preimage"]) else {
            return false;
        };
        if left == right || !seen.insert((left.to_string(), right.to_string())) {
            return false;
        }
        pairs.push((left.to_string(), right.to_string(), scale));
    }
    let Some(sizes) = base_root_sizes(&pairs, &is_base) else {
        return false;
    };
    pairs.iter().all(|(left, right, scale)| {
        (is_base(left) && is_base(right))
            || (class_normalized_map(left, vectors, &classes)
                == class_normalized_map(right, vectors, &classes)
                && expanded_root_size(left, vectors, &sizes)
                    == ratio_mul(scale, &expanded_root_size(right, vectors, &sizes)))
    })
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
    let authority = owner.get("authority").and_then(Value::as_str);
    let identity = owner.get("identity").and_then(Value::as_str);
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
            models
                .iter()
                .any(|model| model["identity"].as_str() == identity)
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
        Some("quire.dimension-equivalence-node/v1") => {
            let Some(ids) = preimage["dimension_node_ids"].as_array() else {
                return false;
            };
            let expected = ids.iter().collect::<Vec<_>>();
            node["semantic_type"] == node["node_id"]
                && node.get("declaration").is_none()
                && node_dependencies_match(node, &expected)
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
        || unqualified_jcs_sha256(&package_id_preimage(&package))
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
    nodes.iter().all(|node| {
        let preimage = &node["nominal_identity_preimage"];
        let node_digest = node["node_id"]["digest"].as_str().unwrap_or_default();
        unqualified_jcs_sha256(preimage) == node_digest
            && semantic_preimage_is_valid(preimage, &vectors)
            && nominal_node_cross_fields_are_valid(node)
            && owner_joins_lock(preimage, &package["lock"])
    })
}

fn replace_node_field_in_graph(
    package: &mut Value,
    node_index: usize,
    relative_pointer: &str,
    replacement: Value,
) {
    let pointer = format!("/semantic_graph/nodes/{node_index}{relative_pointer}");
    *package
        .pointer_mut(&pointer)
        .expect("authored package-node mutation pointer exists") = replacement;
}

fn refresh_package_id(package: &mut Value) {
    package["package_id"]["digest"] = json!(unqualified_jcs_sha256(&package_id_preimage(&package)));
}

fn replace_nominal_preimage_in_graph(
    package: &mut Value,
    node_index: usize,
    relative_pointer: &str,
    replacement: Value,
) {
    replace_node_field_in_graph(
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
        "add" => {
            let path = patch["path"].as_str().expect("add path is a string");
            let (parent, member) = path
                .rsplit_once('/')
                .expect("add path contains an object member");
            let previous = candidate
                .pointer_mut(parent)
                .and_then(Value::as_object_mut)
                .expect("add parent is an object")
                .insert(member.to_owned(), patch["value"].clone());
            assert!(previous.is_none(), "authored add member is new");
        }
        operation => panic!("unsupported authored patch operation: {operation}"),
    }
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

/// Nodes with an application-node identity under FR-322: every node whose
/// body contains an application, plus every `value`/`union_value` node,
/// including a nullary aggregate construction.
fn is_application_node(node: &Value) -> bool {
    !application_terms(&node["body"]).is_empty()
        || (node["node_tag"] == "value" && node["semantic_form"] == "union_value")
}

/// Exact dependency list of an application-bearing node: every reference
/// target, every operation member declaration and every `reaches` member
/// `population` node of its body, plus the optional protocol-clause owner
/// model edge, unique and ascending by digest.
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
                let member = &current["operation"]["member"];
                for link in [&member["declaration"], &member["population"]] {
                    if let Some(digest) = link["digest"].as_str() {
                        targets.insert(digest, link);
                    }
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

/// Exact dependencies of a checked application node, including its
/// identity-bound protocol-clause owner edge when present.
fn application_node_dependencies(node: &Value) -> Vec<Value> {
    let mut targets = application_dependencies(&node["body"]);
    if let Some(owner) = node.get("owner_model") {
        targets.push(owner.clone());
    }
    targets.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
    targets.dedup_by(|left, right| left["digest"] == right["digest"]);
    targets
}

/// Closed reader refusal causes for an application's operation, in evaluation
/// order. The first eight are `invalid_package` causes and the last is the
/// `ill_typed` cause of `quire.native.diagnostics/v1`.
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
    /// `missing_declaration`/`missing-name`: a `relationship_end` member names
    /// an end its relationship does not name.
    MissingName,
    /// `ambiguous_declaration`/`ambiguous-name`: a `relationship_end` member
    /// names a role both ends of its relationship carry.
    AmbiguousName,
    /// `invalid_package`/`invalid-value`: a `string`, `uuid` or `timestamp`
    /// literal whose `value` is not the canonical payload spelling.
    InvalidValue,
    /// `ill_typed`/`type-mismatch`: a `union_value` whose case or payload does
    /// not fit its union (FR-440; FR-322 "Tuple value types and
    /// payload-bearing variant types").
    TypeMismatch,
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
            Self::MissingName => "missing-name",
            Self::AmbiguousName => "ambiguous-name",
            Self::InvalidValue => "invalid-value",
            Self::TypeMismatch => "type-mismatch",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::OperatorIneligible | Self::TypeMismatch => "ill_typed",
            Self::MissingName => "missing_declaration",
            Self::AmbiguousName => "ambiguous_declaration",
            Self::UnknownOperation
            | Self::ClassMismatch
            | Self::LawMissing
            | Self::LawMismatch
            | Self::LawUnselected
            | Self::ModeMismatch
            | Self::MemberMismatch
            | Self::ModeTypeMismatch
            | Self::InvalidValue => "invalid_package",
        }
    }

    fn from_cause(cause: &str) -> Option<Self> {
        // `MissingName` and `AmbiguousName` are the relationship-end refusals, outside the nine
        // operation causes whose pairing and mutations the tests enumerate.
        for outside in [
            Self::MissingName,
            Self::AmbiguousName,
            Self::InvalidValue,
            Self::TypeMismatch,
        ] {
            if cause == outside.cause() {
                return Some(outside);
            }
        }
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
        "case" => "case",
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

/// The `model` semantic forms whose declaration carries `operation` members,
/// per FR-322-AC-25: `object_type`, `process` and `state_machine` share
/// object-type contract meaning (FR-151) and declare a full operation
/// contract; `persistence_interface` declares operation signatures only
/// (FR-208's "Operation contracts" section). `systems_interface` is
/// deliberately absent: FR-152's Interface carries ordered feature
/// signatures for structural conformance (FR-151), never a `dispatch_call`
/// binding. Every other, non-listed form resolves no `"operation"` member
/// here and refuses `operator-ineligible`/`ill_typed` at the reference
/// itself; FR-208 separately pins a declaring-node cause for three of
/// them -- `unsupported_construct`/`declaration-form` on a
/// `record_value_type` (FR-208-AC-4) and `invalid_model_binding`/
/// `malformed-declaration` on an `event_type` (FR-208-AC-4) or a `namespace`
/// (FR-208-AC-8) -- but that is FR-208's own refusal at the declaring node,
/// a separate rule this reader does not compute or emit, so it is not
/// asserted here.
const OPERATION_BEARING_MODEL_FORMS: [&str; 4] = [
    "object_type",
    "process",
    "persistence_interface",
    "state_machine",
];

/// One leaf of FR-322's "Structural leaf walk".
#[derive(Clone, Debug, PartialEq, Eq)]
enum ExpectedLeaf {
    /// A text leaf at its path, with the text profile its type pins.
    Text(Vec<String>, String),
    /// A recursion leaf, whose path ends in `recursion:<d>`; it carries
    /// `laws` `[]` and `mode` `null`.
    Recursion(Vec<String>),
}

impl ExpectedLeaf {
    fn path(&self) -> &[String] {
        match self {
            Self::Text(path, _) | Self::Recursion(path) => path,
        }
    }
}

/// Whether a `string`, `uuid` or `timestamp` literal's `value` is its canonical
/// payload spelling (FR-322 "Scalar literal values"): a UUID as 32 lowercase
/// hexadecimal digits grouped `8-4-4-4-12`, a Timestamp as the canonical decimal
/// text of a signed 128-bit nanosecond count, a String as any scalar sequence.
/// Every other literal kind is outside this rule.
fn literal_value_is_canonical(literal: &Value) -> bool {
    let value = literal["value"].as_str();
    match literal["value_kind"].as_str() {
        Some("string") => value.is_some(),
        Some("uuid") => value.is_some_and(|text| {
            text.len() == 36
                && text.bytes().enumerate().all(|(index, byte)| match index {
                    8 | 13 | 18 | 23 => byte == b'-',
                    _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
                })
        }),
        Some("timestamp") => value.is_some_and(|text| {
            text.parse::<i128>()
                .is_ok_and(|count| count.to_string() == text)
        }),
        Some("quantity" | "rational") => value.is_some_and(quantity_magnitude_is_canonical),
        Some("decimal") => value.is_some_and(decimal_literal_is_canonical),
        Some(kind @ ("float32_bits" | "float64_bits")) => value.is_some_and(|text| {
            let digits = if kind == "float32_bits" { 8 } else { 16 };
            text.len() == digits && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        }),
        _ => true,
    }
}

/// Whether a `decimal` literal's `value` is the canonical spelling of its
/// coefficient and scale (FR-322 "Scalar literal values"): an optional `-` before
/// a nonzero magnitude, the integer digits with no leading zero (one `0` when the
/// integer part is zero), and, when present, a point followed by at least one
/// fraction digit.
fn decimal_literal_is_canonical(text: &str) -> bool {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (integer, fraction) = match digits.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (digits, None),
    };
    let all_digits = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
    if !all_digits(integer) || (integer.len() > 1 && integer.starts_with('0')) {
        return false;
    }
    if fraction.is_some_and(|part| !all_digits(part)) {
        return false;
    }
    let zero = integer.bytes().chain(fraction.unwrap_or("").bytes()).all(|b| b == b'0');
    !(negative && zero)
}

/// Whether a `quantity` literal's `value` is the canonical spelling of its
/// exact rational magnitude (FR-322 "Scalar literal values"): the canonical
/// decimal integer when the denominator is 1, otherwise `numerator/denominator`
/// in lowest terms with a denominator above 1 and a sign only on the numerator.
fn quantity_magnitude_is_canonical(text: &str) -> bool {
    fn gcd(a: u128, b: u128) -> u128 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    let integer = |part: &str| part.parse::<i128>().is_ok_and(|n| n.to_string() == part);
    match text.split_once('/') {
        None => integer(text),
        Some((numerator, denominator)) => {
            let (Ok(n), Ok(d)) = (numerator.parse::<i128>(), denominator.parse::<u128>()) else {
                return false;
            };
            integer(numerator)
                && d.to_string() == denominator
                && d > 1
                && n != 0
                && gcd(n.unsigned_abs(), d) == 1
        }
    }
}

/// Independent reader oracle over the operation catalog, a package lock and
/// its graph. Nothing is read from outside the package and catalog, except
/// the admitted dependency packages a reader is supplied for its
/// `dependency_selections`, which a `dependency_reference` callee resolves in.
struct OperationOracle<'a> {
    catalog: &'a Value,
    lock: &'a Value,
    nodes: &'a [Value],
    /// The admitted document of each `model_selections` row.
    documents: Vec<Value>,
    /// The supplied admitted package bound to each `dependency_selections` entry.
    dependencies: &'a [(String, Value)],
}

impl<'a> OperationOracle<'a> {
    fn for_package(catalog: &'a Value, package: &'a Value) -> Self {
        Self::for_package_with(catalog, package, &[])
    }

    fn for_package_with(
        catalog: &'a Value,
        package: &'a Value,
        dependencies: &'a [(String, Value)],
    ) -> Self {
        Self {
            dependencies,
            catalog,
            lock: &package["lock"],
            nodes: package["semantic_graph"]["nodes"]
                .as_array()
                .expect("package nodes are an array"),
            documents: package["lock"]["model_selections"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|row| selected_model_document(package, row["identity"].as_str()?))
                .collect(),
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
                ("scalar_type", "string") => "string",
                ("scalar_type", "uuid") => "uuid",
                ("scalar_type", "timestamp") => "timestamp",
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
                ("composite_type", "union") => "union",
                // FR-322 "Tuple value types and payload-bearing variant types":
                // the family of a model variant type is read from the selected
                // document, `union` when some case carries a payload.
                ("model", "tuple_value_type") => "tuple",
                ("model", "variant_type") => {
                    if self
                        .variant_cases(node)?
                        .iter()
                        .any(|(_, payload)| payload.is_some())
                    {
                        "union"
                    } else {
                        "enum"
                    }
                }
                ("composite_type", "sequence") => "sequence",
                ("composite_type", "set") => "set",
                ("composite_type", "bag") => "bag",
                ("composite_type", "ordered_set") => "ordered_set",
                ("composite_type", "reference") => "reference",
                ("model", "object_type" | "process" | "state_machine" | "systems_interface") => {
                    "object"
                }
                ("bounded_domain", "model_population") => "population",
                _ => return None,
            },
        )
    }

    /// The cases of a `model`/`variant_type` node as the selected document
    /// declares them, in ascending UTF-8 byte order of case name, each with its
    /// `payloadType`.
    fn variant_cases(&self, node: &Value) -> Option<Vec<(String, Option<String>)>> {
        (node["node_tag"] == "model" && node["semantic_form"] == "variant_type").then_some(())?;
        let identity = node["owner"]["identity"].as_str()?;
        let owner = node["owner"]["node"].as_str()?;
        let declared = self
            .documents
            .iter()
            .filter(|document| document["identity"] == identity)
            .filter_map(|document| document["variant_types"].as_array())
            .flatten()
            .find(|declared| declared["node"] == owner)?;
        let mut cases = declared["cases"]
            .as_array()?
            .iter()
            .map(|case| {
                Some((
                    case["name"].as_str()?.to_owned(),
                    case["payloadType"].as_str().map(str::to_owned),
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        cases.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        Some(cases)
    }

    /// The `typeRef` of each position of a `model`/`tuple_value_type` node, in
    /// position order, as the selected document declares them.
    fn tuple_positions(&self, node: &Value) -> Option<Vec<String>> {
        (node["node_tag"] == "model" && node["semantic_form"] == "tuple_value_type")
            .then_some(())?;
        let identity = node["owner"]["identity"].as_str()?;
        let owner = node["owner"]["node"].as_str()?;
        let declared = self
            .documents
            .iter()
            .filter(|document| document["identity"] == identity)
            .filter_map(|document| document["tuple_types"].as_array())
            .flatten()
            .find(|declared| declared["node"] == owner)?;
        let mut positions = declared["positions"]
            .as_array()?
            .iter()
            .map(|position| Some((position["ordinal"].as_u64()?, position["typeRef"].as_str()?)))
            .collect::<Option<Vec<_>>>()?;
        positions.sort_by_key(|(ordinal, _)| *ordinal);
        Some(positions.into_iter().map(|(_, r)| r.to_owned()).collect())
    }

    /// The type node a member `typeRef` of the selected document derives
    /// (FR-322 "Model-owned members" step 4, for a required single-valued
    /// member): a native `Boolean` or `Integer` is its scalar node, a tuple or
    /// variant type of the package is its model declaration node.
    fn type_ref_node(&self, reference: &str) -> Option<&'a Value> {
        match reference.strip_prefix("ix://quire/native/") {
            Some(native @ ("Boolean" | "Integer")) => {
                let form = native.to_lowercase();
                self.nodes.iter().find(|node| {
                    node["node_tag"] == "scalar_type" && node["semantic_form"] == form.as_str()
                })
            }
            Some(_) => None,
            None => self.nodes.iter().find(|node| {
                node["node_tag"] == "model"
                    && matches!(
                        node["semantic_form"].as_str(),
                        Some("tuple_value_type" | "variant_type")
                    )
                    && node["owner"]["node"] == reference
            }),
        }
    }

    /// The members of a union type, in member declaration order, each with the
    /// type nodes of its payload positions: the bindings of a
    /// `composite_type`/`union` body (FR-440), or the cases of a
    /// `model`/`variant_type` node of family `union`, which the selected
    /// document declares in ascending UTF-8 byte order of case name.
    fn union_members(&self, id: &Value) -> Option<Vec<(String, Vec<&'a Value>)>> {
        let node = self.resolve(id)?;
        (self.type_family(id)? == "union").then_some(())?;
        if node["node_tag"] == "model" {
            return self
                .variant_cases(node)?
                .into_iter()
                .map(|(name, payload)| {
                    let types = match payload {
                        None => Vec::new(),
                        Some(reference) => vec![&self.type_ref_node(&reference)?["node_id"]],
                    };
                    Some((name, types))
                })
                .collect();
        }
        node["body"]["members"]
            .as_array()?
            .iter()
            .map(|binding| {
                let payloads = binding["value"]["members"]
                    .as_array()?
                    .iter()
                    .map(Self::reference_target)
                    .collect::<Option<Vec<_>>>()?;
                Some((binding["name"].as_str()?.to_owned(), payloads))
            })
            .collect()
    }

    /// FR-322 "Tuple value types and payload-bearing variant types", "Union
    /// reads over a model union": a `union_value` whose `semantic_type` is a
    /// `model`/`variant_type` node names one case of it with one payload term
    /// of the payload type for a payload case and none for a nullary case;
    /// over a variant type of family `enum` it is refused. A union value of
    /// another type is FR-440's and is not read here.
    fn union_value_refusal(&self, node: &Value) -> Option<OperationRefusal> {
        if node["node_tag"] != "value" || node["semantic_form"] != "union_value" {
            return None;
        }
        self.node(&node["semantic_type"])
            .filter(|declared| {
                declared["node_tag"] == "model" && declared["semantic_form"] == "variant_type"
            })?;
        let fits = || {
            let members = self.union_members(&node["semantic_type"])?;
            let [binding] = node["body"]["members"].as_array()?.as_slice() else {
                return None;
            };
            let (_, payload_types) = members
                .iter()
                .find(|(name, _)| binding["name"] == name.as_str())?;
            let terms = binding["value"]["members"].as_array()?;
            (terms.len() == payload_types.len()
                && terms.iter().zip(payload_types).all(|(term, payload_type)| {
                    self.descriptor(term)
                        .and_then(|descriptor| descriptor.ty)
                        .is_some_and(|ty| self.same_type(&ty, payload_type))
                }))
            .then_some(())
        };
        fits().is_none().then_some(OperationRefusal::TypeMismatch)
    }

    /// FR-322 "Strict reader" enum member join over a `model`/`variant_type`:
    /// an `enum_value` whose `semantic_type` is a variant type names a case
    /// the selected document declares, and the variant type is of family
    /// `enum`, else the node is refused `invalid_semantic_graph`.
    fn enum_value_is_refused(&self, node: &Value) -> bool {
        if node["semantic_form"] != "enum_value" {
            return false;
        }
        let Some(cases) = self
            .node(&node["semantic_type"])
            .and_then(|declared| self.variant_cases(declared))
        else {
            return false;
        };
        cases.iter().any(|(_, payload)| payload.is_some())
            || !cases.iter().any(|(name, _)| node["body"]["value"] == name.as_str())
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
            "literal" => {
                let typed = self.typed(&term["type"])?;
                let admitted: &[&str] = match term["value_kind"].as_str()? {
                    "boolean" => &["boolean"],
                    "integer" => &["integer"],
                    "rational" => &["rational"],
                    "decimal" => &["decimal"],
                    "float32_bits" => &["float32"],
                    "float64_bits" => &["float64"],
                    "text" => &["text"],
                    "string" => &["string"],
                    "uuid" => &["uuid"],
                    "timestamp" => &["timestamp"],
                    "enum" => &["enum", "ordered_enum"],
                    "none" => &["option"],
                    "quantity" => &["quantity"],
                    _ => return None,
                };
                admitted.contains(&typed.family.as_str()).then_some(typed)
            }
            "reference" => {
                let target = self.node(&term["target"])?;
                match target["node_tag"].as_str()? {
                    "function" => Some(Descriptor {
                        family: "function".to_owned(),
                        ty: Some(target["semantic_type"].clone()),
                        bound: None,
                    }),
                    "relation" if target["semantic_form"] == "population" => Some(Descriptor {
                        family: "population".to_owned(),
                        ty: Some(target["node_id"].clone()),
                        bound: None,
                    }),
                    "temporal"
                        if matches!(
                            target["semantic_form"].as_str(),
                            Some("formula" | "fairness")
                        ) =>
                    {
                        self.classify(&target["body"]).ok()
                    }
                    _ => self.typed(&target["semantic_type"]),
                }
            }
            // FR-322: a `dependency_reference` callee has family `function`
            // and the named dependency function's result type, which the
            // importing graph carries under the same id.
            "dependency_reference" => {
                let target = dependency_reference_target(self.dependencies, term)?;
                (target["node_tag"] == "function").then(|| Descriptor {
                    family: "function".to_owned(),
                    ty: Some(target["semantic_type"].clone()),
                    bound: None,
                })
            }
            "application" => self.classify(term).ok(),
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
            Some("application") => self.classify(term).map(Some),
            Some("binding") => {
                Ok(self
                    .argument_descriptor(&term["value"])?
                    .map(|bound| Descriptor {
                        family: "binder".to_owned(),
                        ty: None,
                        bound: Some(Box::new(bound)),
                    }))
            }
            // The literal's family is decided first; the value spelling is read
            // only for a literal whose type its kind admits.
            Some("literal") => match self.descriptor(term) {
                Some(_) if !literal_value_is_canonical(term) => Err(OperationRefusal::InvalidValue),
                descriptor => Ok(descriptor),
            },
            _ => Ok(self.descriptor(term)),
        }
    }

    /// The relationship the selected document declares for a model
    /// declaration node, or `None` when the node is not a model-owned
    /// relationship the document declares.
    fn model_relationship(&self, relationship: &Value) -> Option<&Value> {
        if !(is_model_declaration(relationship) && relationship["node_tag"] == "relation") {
            return None;
        }
        let owner = &relationship["owner"];
        self.documents
            .iter()
            .filter(|document| document["identity"] == owner["identity"])
            .flat_map(|document| document["relationships"].as_array().into_iter().flatten())
            .find(|declared| declared["node"] == owner["node"])
    }

    /// The refusal of a `relationship_end` member whose model-owned
    /// relationship does not name its role exactly once (FR-322 "Model-owned
    /// members" step 3): `missing-name` for none, including an absent inverse
    /// (no name is synthesized), `ambiguous-name` for two.
    fn end_lookup_refusal(&self, member: &Value) -> Option<OperationRefusal> {
        if member["kind"] != "relationship_end" {
            return None;
        }
        let relationship = self
            .node(&member["declaration"])
            .and_then(|relationship| self.model_relationship(relationship))?;
        let name = member["name"].as_str()?;
        match relationship["ends"]
            .as_array()?
            .iter()
            .filter(|end| end["role"] == name)
            .count()
        {
            0 => Some(OperationRefusal::MissingName),
            1 => None,
            _ => Some(OperationRefusal::AmbiguousName),
        }
    }

    /// FR-322 "Population link of a `reaches`": whether a `relationship_end`
    /// member's optional `population` is admissible. An absent key is. A present
    /// one is only on a `reaches` application and names a
    /// `bounded_domain`/`model_population` node over `Set<Reference<T>>` whose
    /// model declaration `T` is the type of the end the member's role names.
    fn population_link_holds(&self, entry: &Value, member: &Value) -> bool {
        let Some(population) = member.get("population") else {
            return true;
        };
        let model_declared = |id: &Value, tag: &str, form: &str| {
            self.node(id)
                .filter(|node| node["node_tag"] == tag && node["semantic_form"] == form)
        };
        let first_target = |node: &'a Value| node["body"]["members"][0]["target"].clone();
        let object_type = || {
            let population = model_declared(population, "bounded_domain", "model_population")?;
            let set = model_declared(&population["semantic_type"], "composite_type", "set")?;
            let reference = model_declared(&first_target(set), "composite_type", "reference")?;
            self.node(&first_target(reference))
        };
        let near_end_type = || {
            let relationship = self.node(&member["declaration"])?;
            let declared = self.model_relationship(relationship)?;
            let name = member["name"].as_str()?;
            declared["ends"]
                .as_array()?
                .iter()
                .find(|end| end["role"] == name)
                .map(|end| end["type"].clone())
        };
        entry["operator"] == "reaches"
            && member["kind"] == "relationship_end"
            && matches!(
                (object_type(), near_end_type()),
                (Some(object), Some(end))
                    if is_model_declaration(object) && object["owner"]["node"] == end
            )
    }

    /// The field of a model object type, typed by FR-322 "Model-owned members"
    /// step 4 from the selected document: a required, single-valued Boolean or
    /// Integer field is that scalar node. `None` when `member` does not name a
    /// model object type; `Some(None)` when the field is absent or has another
    /// presence, multiplicity or type (an optional field is `Option<T>`, which
    /// this oracle does not form), so the projection is ineligible.
    fn model_field_descriptor(&self, member: &Value) -> Option<Option<Descriptor>> {
        let declaration = self.node(&member["declaration"])?;
        if declaration["node_tag"] != "model"
            || declaration["semantic_form"] != "object_type"
            || declaration["owner"]["kind"] != "model"
        {
            return None;
        }
        let owner = declaration["owner"]["node"].as_str()?;
        let field = self
            .documents
            .iter()
            .filter_map(|document| document["object_types"].as_array())
            .flatten()
            .find(|declared| declared["node"] == owner)
            .and_then(|declared| declared["fields"].as_array())
            .and_then(|fields| fields.iter().find(|field| field["name"] == member["name"]));
        Some((|| {
            let field = field?;
            let single = field["multiplicity"]["lower"] == "1" && field["multiplicity"]["upper"] == "1";
            if field["presence"] != "required" || !single {
                return None;
            }
            let form = match field["typeRef"].as_str()? {
                "ix://quire/native/Boolean" => "boolean",
                "ix://quire/native/Integer" => "integer",
                _ => return None,
            };
            let scalar = self
                .nodes
                .iter()
                .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == form)?;
            self.typed(&scalar["node_id"])
        })())
    }

    /// Result descriptor of a member selection, resolved against its declaration.
    fn member_descriptor(&self, member: &Value) -> Option<Descriptor> {
        let declaration = &member["declaration"];
        match member["kind"].as_str()? {
            "field" => {
                if let Some(typed) = self.model_field_descriptor(member) {
                    return typed;
                }
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
                // A `KeyOrdinal` is an integer or a decimal string beyond the safe
                // integer range; one past the last position matches none.
                let position = usize::try_from(key_ordinal(&member["position"])?).ok()?;
                if tuple["node_tag"] == "model" {
                    let positions = self.tuple_positions(tuple)?;
                    return self.typed(&self.type_ref_node(positions.get(position)?)?["node_id"]);
                }
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
                if let Some(declared) = self.model_relationship(relationship) {
                    // A model-owned relationship names no end in its body; its
                    // named ends are the document's: `sourceEnd.role` and, when
                    // declared, `targetEnd.role`. FR-152: traversal from source
                    // to target needs `source-to-target`, `bidirectional` or
                    // `undirected`, and from target to source
                    // `target-to-source`, `bidirectional` or `undirected`.
                    let name = member["name"].as_str()?;
                    let end = declared["ends"]
                        .as_array()?
                        .iter()
                        .find(|end| end["role"] == name)?;
                    let needed = if end["end"] == "source" {
                        "source-to-target"
                    } else {
                        "target-to-source"
                    };
                    return [needed, "bidirectional", "undirected"]
                        .contains(&declared["direction"].as_str()?)
                        .then(|| Descriptor::family("reference"));
                }
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
                (model["node_tag"] == "model"
                    && model["semantic_form"]
                        .as_str()
                        .is_some_and(|form| OPERATION_BEARING_MODEL_FORMS.contains(&form)))
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
        terms: &[Value],
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
            // FR-440: the type every arm body has: every arm body agrees, else
            // there is no such type. `union_arms` fixes that the arms are the
            // union's own.
            "arm_body" => {
                let bodies = terms
                    .get(1..)?
                    .iter()
                    .map(|arm| self.descriptor(&arm["value"]["members"][1]))
                    .collect::<Option<Vec<_>>>()?;
                let first = bodies.first()?;
                bodies
                    .iter()
                    .all(|body| {
                        body.family == first.family
                            && match (&body.ty, &first.ty) {
                                (Some(left), Some(right)) => self.same_type(left, right),
                                (None, None) => true,
                                _ => false,
                            }
                    })
                    .then(|| first.clone())
            }
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

    /// Leaves of a compared type in FR-322 "Structural leaf walk" order: a
    /// text leaf with the text profile its type pins, or a recursion leaf
    /// (`recursion:<d>`, no profile). `None` when a leaf is IEEE, a text leaf
    /// pins no profile, or a cycle reenters a type that is not a record,
    /// tuple or union.
    fn expected_leaves(&self, id: &Value) -> Option<Vec<ExpectedLeaf>> {
        let mut leaves = Vec::new();
        self.collect_leaves(id, &mut Vec::new(), &mut Vec::new(), &mut leaves)?;
        Some(leaves)
    }

    /// The walk steps out of one type: each field, tuple position, union
    /// member payload position or element type, with the path segments that
    /// reach it. Any other type has none.
    fn leaf_children(&self, id: &Value) -> Option<Vec<(Vec<String>, &'a Value)>> {
        let node = self.resolve(id)?;
        let members = || node["body"]["members"].as_array();
        Some(match self.type_family(id)? {
            "record" => members()?
                .iter()
                .map(|member| {
                    Some((
                        vec![format!("field:{}", member["name"].as_str()?)],
                        Self::reference_target(&member["value"])?,
                    ))
                })
                .collect::<Option<_>>()?,
            "tuple" if node["node_tag"] == "model" => self
                .tuple_positions(node)?
                .iter()
                .enumerate()
                .map(|(index, reference)| {
                    Some((
                        vec![format!("position:{index}")],
                        &self.type_ref_node(reference)?["node_id"],
                    ))
                })
                .collect::<Option<_>>()?,
            "tuple" => members()?
                .iter()
                .enumerate()
                .map(|(index, member)| {
                    Some((
                        vec![format!("position:{index}")],
                        Self::reference_target(member)?,
                    ))
                })
                .collect::<Option<_>>()?,
            "union" => {
                let mut children = Vec::new();
                for (name, payloads) in self.union_members(id)? {
                    for (index, payload) in payloads.into_iter().enumerate() {
                        children.push((
                            vec![format!("member:{name}"), format!("position:{index}")],
                            payload,
                        ));
                    }
                }
                children
            }
            "option" | "sequence" | "set" | "bag" | "ordered_set" => {
                vec![(vec!["inner".to_owned()], self.inner_type(id)?)]
            }
            _ => Vec::new(),
        })
    }

    /// Whether some walk step sequence leads from `id` to a text type.
    fn reaches_text(&self, id: &Value) -> bool {
        let mut seen = BTreeSet::new();
        let mut pending = vec![id];
        while let Some(current) = pending.pop() {
            if self.type_family(current) == Some("text") {
                return true;
            }
            let Some(node) = self.resolve(current) else {
                continue;
            };
            if !seen.insert(node["node_id"].to_string()) {
                continue;
            }
            pending.extend(
                self.leaf_children(current)
                    .into_iter()
                    .flatten()
                    .map(|(_, child)| child),
            );
        }
        false
    }

    /// One step of the walk at `path`. `open` holds each type node being
    /// walked on the current path with the path length at which it was
    /// entered.
    fn collect_leaves(
        &self,
        id: &Value,
        path: &mut Vec<String>,
        open: &mut Vec<(Value, usize)>,
        leaves: &mut Vec<ExpectedLeaf>,
    ) -> Option<()> {
        let node = self.resolve(id)?;
        let family = self.type_family(id)?;
        if let Some((_, entered)) = open.iter().find(|(open_id, _)| *open_id == node["node_id"]) {
            if !matches!(family, "record" | "tuple" | "union") {
                return None;
            }
            if self.reaches_text(id) {
                let mut recursion = path.clone();
                recursion.push(format!("recursion:{entered}"));
                leaves.push(ExpectedLeaf::Recursion(recursion));
            }
            return Some(());
        }
        match family {
            "text" => {
                let profile = self.pin(&self.typed(id)?, "text_profile")?;
                leaves.push(ExpectedLeaf::Text(path.clone(), profile));
                return Some(());
            }
            "float32" | "float64" => return None,
            _ => {}
        }
        let children = self.leaf_children(id)?;
        open.push((node["node_id"].clone(), path.len()));
        for (segments, child) in children {
            let depth = path.len();
            path.extend(segments);
            self.collect_leaves(child, path, open, leaves)?;
            path.truncate(depth);
        }
        open.pop();
        Some(())
    }

    /// The catalogued definitions of a law role, each as the package
    /// `DefinitionRef` that names it: its authority and identity.
    fn law_refs(&self, role: &str) -> Vec<Value> {
        self.catalog["law_roles"][role]
            .as_array()
            .into_iter()
            .flatten()
            .map(|reference| {
                json!({"authority": reference["authority"], "identity": reference["identity"]})
            })
            .collect()
    }

    /// Whether a law role names a lock profile selection rather than a
    /// catalogued definition.
    fn is_profile_role(&self, role: &str) -> bool {
        self.catalog["profile_law_roles"]
            .as_array()
            .is_some_and(|roles| roles.iter().any(|candidate| candidate == role))
    }

    /// Whether the lock selects a law: a profile law by an exact
    /// `profile_selections` row of its role, any other law by an exact
    /// `definition_selections` entry.
    fn selected(&self, law: &Value) -> bool {
        let role = law["role"].as_str().unwrap_or_default();
        if self.is_profile_role(role) {
            self.lock["profile_selections"]
                .as_array()
                .is_some_and(|selections| {
                    selections.iter().any(|selection| {
                        selection["role"] == role && selection["definition"] == law["definition"]
                    })
                })
        } else {
            self.lock["definition_selections"]
                .as_array()
                .is_some_and(|selections| selections.contains(&law["definition"]))
        }
    }

    fn mode_value_admitted(&self, mode: &Value, kind: &str) -> bool {
        mode["kind"] == kind
            && self.catalog["modes"][kind]
                .as_array()
                .is_some_and(|values| values.contains(&mode["value"]))
    }

    /// FR-149 class of a conversion between two exact numeric types.
    fn conversion_class(&self, source: &Value, target: &Value) -> Option<ConversionClass> {
        if source == target {
            return Some(ConversionClass::Exact);
        }
        conversion_class(&self.numeric_bounds(source)?, &self.numeric_bounds(target)?)
    }

    /// FR-440's `union_arms`: `arms` are exactly one arm per member of
    /// `union`, in member declaration order, each named by its member, each
    /// binder aggregate holding as many parameter references as the member's
    /// payload arity, each binder's `semantic_type` the payload type there.
    fn union_arms_hold(&self, union: &Value, arms: &[Value]) -> bool {
        let Some(members) = self.union_members(union) else {
            return false;
        };
        members.len() == arms.len()
            && members.iter().zip(arms).all(|((name, payloads), arm)| {
                let arm_members = arm["value"]["members"].as_array().map(Vec::as_slice);
                let binders = arm_members
                    .filter(|parts| parts.len() == 2)
                    .and_then(|parts| parts[0]["members"].as_array());
                arm["term"] == "binding"
                    && arm["name"] == name.as_str()
                    && binders.is_some_and(|binders| {
                        payloads.len() == binders.len()
                            && payloads.iter().zip(binders).all(|(payload_type, binder)| {
                                binder["term"] == "reference"
                                    && self.resolve(&binder["target"]).is_some_and(|parameter| {
                                        self.same_type(payload_type, &parameter["semantic_type"])
                                    })
                            })
                    })
            })
    }

    fn constraint_holds(
        &self,
        constraint: &Value,
        operation: &Value,
        arguments: &[Descriptor],
        terms: &[Value],
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
        let converts_as = |class: ConversionClass| matches!((ty(operand(0)), member().and_then(|m| m.ty)), (Some(s), Some(t)) if self.conversion_class(&s, &t) == Some(class));
        match constraint["kind"].as_str() {
            Some("same_family") => {
                matches!((operand(0), operand(1)), (Some(l), Some(r)) if l.family == r.family)
            }
            Some("same_type") => {
                matches!((ty(operand(0)), ty(operand(1))), (Some(l), Some(r)) if self.same_type(&l, &r))
            }
            // This oracle reads no domain package and recovers no model
            // owner, so it cannot decide `conforming_reference`; no
            // published TC-233 package applies it. FR-322 "Reference
            // conformance" is qualified by TC-281 in `model_member_types.rs`.
            Some("conforming_reference") => {
                panic!("the TC-233 oracle does not decide conforming_reference")
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
            Some("exact_conversion") => converts_as(ConversionClass::Exact),
            Some("range_narrowing") => converts_as(ConversionClass::RangeNarrowing),
            Some("rational_narrowing") => converts_as(ConversionClass::RationalNarrowing),
            Some("scale_reduction") => converts_as(ConversionClass::ScaleReduction),
            Some("promotes_exact") => {
                let promotable = self.expand("rational_promotable");
                constraint["operands"].as_array().is_some_and(|slots| {
                    (0..slots.len()).all(|slot| {
                        operand(slot).is_some_and(|d| promotable.contains(&d.family.as_str()))
                    })
                })
            }
            Some("union_arms") => operand(0)
                .and_then(|scrutinee| scrutinee.ty.as_ref())
                .is_some_and(|union| {
                    self.union_arms_hold(union, terms.get(1..).unwrap_or_default())
                }),
            Some("uniform_rest") => arguments[rest_start.min(arguments.len())..]
                .windows(2)
                .all(|pair| pair[0].family == pair[1].family),
            _ => false,
        }
    }

    /// The declared result of an application: its `result_type` when that
    /// type agrees with the result the catalog derives from the operands. A
    /// clause result and a temporal formula result are declared Boolean.
    fn declared_result(&self, term: &Value, derived: Descriptor) -> Option<Descriptor> {
        let result_type = &term["result_type"];
        let declared_family = self.type_family(result_type)?;
        let fits = if derived.family == "clause" || derived.family == "temporal" {
            declared_family == "boolean"
        } else {
            declared_family == derived.family
                && derived
                    .ty
                    .as_ref()
                    .is_none_or(|ty| self.same_type(ty, result_type))
        };
        fits.then(|| Descriptor {
            family: derived.family,
            ty: Some(result_type.clone()),
            bound: None,
        })
    }

    /// Classify one application in the normative order and return its
    /// declared result descriptor.
    fn classify(&self, term: &Value) -> Result<Descriptor, OperationRefusal> {
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
                let role = law["role"].as_str().unwrap_or_default();
                if self.is_profile_role(role) {
                    matched_profile(&law["definition"]).is_some_and(|(kind, _)| kind == role)
                } else {
                    self.law_refs(role).contains(&law["definition"])
                }
            })
            || (entry["leaves"].is_null() && !supplied_leaves.is_empty())
        {
            return Err(Refusal::LawMismatch);
        }
        if !laws.iter().all(|law| self.selected(law)) {
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
            (Some("profile_operator"), member) => {
                member["kind"] == "profile_operator"
                    && laws
                        .first()
                        .and_then(|law| profile_operators(&law["definition"]))
                        .map(|operators| {
                            if operation["identity"] == "quire.op.state.transition" {
                                state_transition_operators()
                            } else {
                                operators
                            }
                        })
                        .zip(member["operator"].as_str())
                        .is_some_and(|(operators, operator)| {
                            operators.iter().any(|candidate| candidate == operator)
                        })
            }
            (Some(kind), member) => member["kind"] == kind,
            (None, _) => false,
        };
        if !member_fits {
            return Err(Refusal::MemberMismatch);
        }

        let terms = term["arguments"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default();
        let arguments = terms
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
            .and_then(|arguments| self.result_descriptor(form, operation, arguments, terms))
            .and_then(|derived| self.declared_result(term, derived));
        if let (Some(kind), Some(arguments), Some(result)) = (
            entry["mode"].as_str(),
            arguments.as_deref(),
            result.as_ref(),
        ) && self.catalog["type_pinned_modes"].get(kind).is_some()
        {
            let operand_sources = arguments
                .iter()
                .map(|argument| argument.bound.as_deref().unwrap_or(argument));
            let sources = if entry["operator"] == "convert" {
                vec![result]
            } else {
                operand_sources.chain([result]).collect()
            };
            let pins = sources
                .into_iter()
                .filter_map(|source| self.pin(source, kind))
                .collect::<BTreeSet<_>>();
            let declared = operation["mode"]["value"].as_str().map(str::to_owned);
            if pins.len() != 1 || pins.first() != declared.as_ref() {
                return Err(Refusal::ModeTypeMismatch);
            }
        }

        if let Some(refusal) = self.end_lookup_refusal(member) {
            return Err(refusal);
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
        let member_resolves = entry["member"].is_null()
            || entry["member"] == "profile_operator"
            || entry["member"] == "state_clause"
            || entry["member"] == "temporal_interval"
            || entry["member"] == "fairness"
            || self.member_descriptor(member).is_some();
        let population_links = self.population_link_holds(entry, member);
        let constraints_hold = entry["constraints"]
            .as_array()
            .expect("catalog constraints are an array")
            .iter()
            .all(|constraint| {
                self.constraint_holds(constraint, operation, &arguments, terms, positions.len())
            });
        let result = result.ok_or(Refusal::OperatorIneligible)?;
        let expected_leaves = match entry["leaves"].as_str() {
            None => Some(Vec::new()),
            Some("result_inner") => match self.type_family(&term["result_type"]) {
                Some("set" | "bag" | "ordered_set") => self
                    .inner_type(&term["result_type"])
                    .and_then(|inner| self.expected_leaves(inner)),
                _ => Some(Vec::new()),
            },
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
            population_links,
            constraints_hold,
            expected_leaves,
        ) else {
            return Err(Refusal::OperatorIneligible);
        };

        if supplied_leaves.len() < expected.len() {
            return Err(Refusal::LawMissing);
        }
        let text_refs = self.law_refs("text_profile");
        let leaf_shape_fits = supplied_leaves.len() == expected.len()
            && supplied_leaves.iter().zip(&expected).all(|(leaf, want)| {
                let path_fits = leaf["path"].as_array().is_some_and(|segments| {
                    segments
                        .iter()
                        .map(Value::as_str)
                        .eq(want.path().iter().map(|s| Some(s.as_str())))
                });
                let laws_fit = leaf["laws"].as_array().is_some_and(|laws| match want {
                    ExpectedLeaf::Text(..) => {
                        laws.len() == 1
                            && laws[0]["role"] == "text_profile"
                            && text_refs.contains(&laws[0]["definition"])
                    }
                    ExpectedLeaf::Recursion(_) => laws.is_empty(),
                });
                path_fits && laws_fit
            });
        if !leaf_shape_fits {
            return Err(Refusal::LawMismatch);
        }
        let text_leaves = || {
            supplied_leaves
                .iter()
                .zip(&expected)
                .filter_map(|(leaf, want)| match want {
                    ExpectedLeaf::Text(_, pin) => Some((leaf, pin)),
                    ExpectedLeaf::Recursion(_) => None,
                })
        };
        if !text_leaves().all(|(leaf, _)| self.selected(&leaf["laws"][0])) {
            return Err(Refusal::LawUnselected);
        }
        if !supplied_leaves
            .iter()
            .zip(&expected)
            .all(|(leaf, want)| match want {
                ExpectedLeaf::Text(..) => self.mode_value_admitted(&leaf["mode"], "text_profile"),
                ExpectedLeaf::Recursion(_) => leaf["mode"].is_null(),
            })
        {
            return Err(Refusal::ModeMismatch);
        }
        if !text_leaves().all(|(leaf, pin)| leaf["mode"]["value"] == pin.as_str()) {
            return Err(Refusal::ModeTypeMismatch);
        }
        Ok(result)
    }

    /// Classify an application of `node`; an expression or function whose
    /// body root is the application declares exactly its result type.
    fn classify_in(&self, node: &Value, term: &Value) -> Result<Descriptor, OperationRefusal> {
        let result = self.classify(term)?;
        let at_root = std::ptr::eq(term, &node["body"])
            && matches!(node["node_tag"].as_str(), Some("expression" | "function"));
        if at_root && node["semantic_type"] != term["result_type"] {
            return Err(OperationRefusal::OperatorIneligible);
        }
        Ok(result)
    }
}

/// FR-149 class of a conversion between exact numeric types: exact, a checked
/// range or rational narrowing obligation, or a scale reduction that rounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConversionClass {
    Exact,
    RangeNarrowing,
    RationalNarrowing,
    ScaleReduction,
}

/// FR-149 conversion class decided from declared bounds alone; `None` when no
/// exact-numeric conversion applies.
fn conversion_class(from: &NumericBounds, to: &NumericBounds) -> Option<ConversionClass> {
    use NumericBounds::{Decimal, Int, Integer, Rational};
    if exact_bounds_conversion(from, to) {
        return Some(ConversionClass::Exact);
    }
    let one = BigInt::from(1);
    let zero = BigInt::from(0);
    match (from, to) {
        (Decimal(Some([_, _, _, s2])), Decimal(Some([_, _, _, s4]))) if s2 > s4 => {
            Some(ConversionClass::ScaleReduction)
        }
        (Decimal(None) | Rational(None), Decimal(_)) => Some(ConversionClass::ScaleReduction),
        (Rational(Some([_, _, _, d2])), Decimal(_)) if d2 > &one => {
            Some(ConversionClass::ScaleReduction)
        }
        (Rational(_), Rational(_)) => Some(ConversionClass::RationalNarrowing),
        (Integer | Int(..), _)
        | (Decimal(_), Rational(_) | Decimal(_))
        | (Rational(Some(_)), Integer | Int(..) | Decimal(_)) => {
            Some(ConversionClass::RangeNarrowing)
        }
        (Decimal(Some([_, _, _, s2])), Integer | Int(..)) if s2 == &zero => {
            Some(ConversionClass::RangeNarrowing)
        }
        _ => None,
    }
}

/// Bytes of the profile definitions and the rule texts that define their
/// operator sets.
const TEMPORAL_PROFILES: [&[u8]; 3] = [
    include_bytes!("../proposals/quire-v1/definitions/temporal-event-position.md"),
    include_bytes!("../proposals/quire-v1/definitions/temporal-fixed-sample.md"),
    include_bytes!("../proposals/quire-v1/definitions/temporal-timestamped-window.md"),
];
const PROTOCOL_FINITE: &[u8] =
    include_bytes!("../proposals/quire-v1/definitions/protocol-finite.md");
const TEMPORAL_COMMON: &str = include_str!("../proposals/quire-v1/definitions/temporal-common.md");

/// Items of an English list `a, b and c` between `start` and `end` in a text
/// whose line breaks are read as spaces, spelled with underscores.
fn listed_items(text: &str, start: &str, end: &str) -> Vec<String> {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let (_, after) = flat
        .split_once(start)
        .unwrap_or_else(|| panic!("rule text lists `{start}`"));
    let (list, _) = after
        .split_once(end)
        .unwrap_or_else(|| panic!("rule list after `{start}` ends with `{end}`"));
    list.split(", ")
        .flat_map(|item| item.split(" and "))
        .map(|item| item.trim().replace(' ', "_"))
        .collect()
}

/// The profile law role and definition text of a temporal or protocol
/// profile `DefinitionRef` whose authority and identity name a published
/// profile definition; `None` for any other reference.
fn matched_profile(definition: &Value) -> Option<(&'static str, &'static str)> {
    let matches = |bytes: &'static [u8]| {
        let text = std::str::from_utf8(bytes).ok()?;
        let line = text
            .lines()
            .find(|line| line.starts_with("Profile identity: "))?;
        let identity = line.split('`').nth(1)?;
        (definition["authority"] == "agent-ix" && definition["identity"] == identity)
            .then_some(text)
    };
    TEMPORAL_PROFILES
        .into_iter()
        .find_map(matches)
        .map(|text| ("temporal_profile", text))
        .or_else(|| matches(PROTOCOL_FINITE).map(|text| ("protocol_profile", text)))
}

/// Operators a selected temporal or protocol profile defines, read from the
/// profile definition the law names: a temporal profile requiring the bounded
/// facet defines its eight bounded operators, and the finite protocol profile
/// the FR-464 protocol operator catalog.
fn profile_operators(definition: &Value) -> Option<Vec<String>> {
    match matched_profile(definition)? {
        ("temporal_profile", text) => text
            .contains("| `quire.temporal.bounded-facet/v1` |")
            .then(|| listed_items(TEMPORAL_COMMON, "bounded operators: ", ".")),
        (_, text) => text
            .contains("Profile identity: `quire.protocol.finite-global/v1`.")
            .then(protocol_operators),
    }
}

/// The finite-global profile's operators are the FR-464 protocol-node catalog.
fn protocol_operators() -> Vec<String> {
    let mut rows = FR464_PROTOCOL_NODES
        .lines()
        .skip_while(|line| !line.starts_with("| Operator |"))
        .skip(2)
        .take_while(|line| line.starts_with('|'));
    let mut operators = Vec::new();
    for row in &mut rows {
        let Some(cell) = row.split('|').nth(1) else {
            continue;
        };
        operators.extend(cell.split('`').skip(1).step_by(2).map(str::to_owned));
    }
    operators
}

/// The finite-global profile's one `state`/`transition` operator, read from
/// FR-464's "State operator" table.
fn state_transition_operators() -> Vec<String> {
    FR464_PROTOCOL_NODES
        .lines()
        .skip_while(|line| !line.starts_with("| State operator |"))
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .filter_map(|row| row.split('|').nth(1))
        .flat_map(|cell| cell.split('`').skip(1).step_by(2).map(str::to_owned))
        .collect()
}

#[trace("TC-946", "FR-464-AC-1")]
#[test]
fn protocol_node_vectors_match_the_fr464_operator_catalog() {
    let vectors = parsed_fixture(PROTOCOL_NODE_VECTORS);
    let expected = protocol_operators();
    let actual = vectors["operators"]
        .as_array()
        .expect("protocol operator vectors are an array")
        .iter()
        .map(|entry| {
            let operator = entry["operator"].as_str().expect("operator is text");
            let semantic_form = entry["semantic_form"]
                .as_str()
                .expect("semantic form is text");
            let arguments = entry["arguments"]
                .as_array()
                .expect("argument layout is an array");
            assert!(!semantic_form.is_empty());
            assert!(!arguments.is_empty());
            operator.to_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 20);

    for schema_text in [SCHEMA, NODE_PREIMAGE_SCHEMA] {
        let schema = parsed_fixture(schema_text);
        let rules = schema["$defs"]["ProtocolControlApplication"]["allOf"]
            .as_array()
            .expect("protocol application argument rules are an array");
        assert_eq!(rules.len(), actual.len());
        for vector in vectors["operators"].as_array().unwrap() {
            let operator = vector["operator"].as_str().unwrap();
            let rule = rules
                .iter()
                .find(|rule| {
                    rule["if"]["properties"]["operation"]["properties"]["member"]["properties"]["operator"]["const"]
                        == operator
                })
                .unwrap_or_else(|| panic!("schema has no argument rule for {operator}"));
            let arguments = rule["then"]["properties"]["arguments"]["prefixItems"]
                .as_array()
                .expect("schema argument layout is an array");
            let names = arguments
                .iter()
                .map(|argument| argument["properties"]["name"]["const"].as_str().unwrap())
                .collect::<Vec<_>>();
            let expected_names = vector["arguments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|name| name.as_str().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(names, expected_names, "{operator} schema argument layout");
            assert_eq!(rule["then"]["properties"]["arguments"]["minItems"], names.len());
            assert_eq!(rule["then"]["properties"]["arguments"]["maxItems"], names.len());
        }
    }
}

#[trace("TC-946", "TC-947", "TC-948", "FR-464-AC-15", "FR-464-AC-16")]
#[test]
fn protocol_node_case_vectors_publish_the_qspec_examples() {
    let vectors = parsed_fixture(PROTOCOL_NODE_VECTORS);
    let cases = vectors["cases"]
        .as_array()
        .expect("QSpec protocol cases are an array");
    let counts = ["TC-946", "TC-947", "TC-948"].map(|test_case| {
        cases
            .iter()
            .filter(|case| case["test_case"] == test_case)
            .count()
    });
    assert_eq!(counts, [127, 88, 167]);
    // Every vector id a QSpec table row names is published as a case: a row
    // the table adds without a vector (TC-946 K10, K14) fails here.
    for (test_case, table) in [
        ("TC-946", TC946_PROTOCOL_NODE_FAMILY),
        ("TC-947", TC947_PROTOCOL_NODE_FAMILY),
        ("TC-948", TC948_PROTOCOL_NODE_FAMILY),
    ] {
        for row in table.lines() {
            let Some(id) = row
                .strip_prefix("| ")
                .and_then(|rest| rest.split(" | ").next())
                .filter(|id| {
                    id.starts_with(|c: char| c.is_ascii_uppercase())
                        && id.ends_with(|c: char| c.is_ascii_digit())
                        && id.chars().all(|c| c.is_ascii_alphanumeric())
                })
            else {
                continue;
            };
            let cells = row.trim_end().trim_end_matches('|').split(" | ").collect::<Vec<_>>();
            let vector = cases
                .iter()
                .find(|case| case["test_case"] == test_case && case["id"] == id);
            let Some(vector) = vector else {
                panic!("{test_case} row {id} has no protocol node vector");
            };
            // A vector's text is its table row's text (rows a vector splits
            // into lettered cases keep their own ids and are not compared).
            if cells.len() == 3 {
                assert_eq!(vector["Expected"], cells[2].trim(), "{test_case} {id} Expected");
                let edit = ["Edit", "Edit to the base", "Encoded"]
                    .iter()
                    .find_map(|field| vector[*field].as_str());
                assert_eq!(edit, Some(cells[1].trim()), "{test_case} {id} edit text");
            }
        }
    }
    for case in cases {
        assert!(case["section"].as_str().is_some_and(|section| !section.is_empty()));
        assert!(case["id"].as_str().is_some_and(|id| !id.is_empty()));
        assert!(case["Expected"].as_str().is_some_and(|expected| !expected.is_empty()));
        assert!(
            ["Edit", "Edit to the base", "Encoded"]
                .iter()
                .any(|field| case[*field].as_str().is_some_and(|edit| !edit.is_empty()))
        );
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
    package_operations_refusal_with(package, catalog, &[])
}

fn package_operations_refusal_with(
    package: &Value,
    catalog: &Value,
    dependencies: &[(String, Value)],
) -> Result<(), OperationRefusal> {
    let oracle = OperationOracle::for_package_with(catalog, package, dependencies);
    let mut nodes = oracle.nodes.iter().collect::<Vec<_>>();
    nodes.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    nodes.into_iter().try_for_each(|node| {
        if let Some(refusal) = oracle.union_value_refusal(node) {
            return Err(refusal);
        }
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

/// The `quire.application-node/v1` preimage of every FR-322 application node;
/// protocol clauses add their identity-bound `owner_model` field.
fn application_node_preimage(node: &Value, nodes: &[Value]) -> Option<Value> {
    if !is_application_node(node) {
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
    let mut preimage = json!({
        "version": "quire.application-node/v1",
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": node["semantic_type"],
        "declaration": node.get("declaration").cloned().unwrap_or(Value::Null),
        "recursion": recursion,
        "body": preimage_term(&node["body"], &group),
    });
    if let Some(owner_model) = node.get("owner_model") {
        preimage["owner_model"] = owner_model.clone();
    }
    Some(preimage)
}

/// Graph-shape constraints of an application-bearing node: an expression
/// whose root is an application has the operator's semantic form, and its
/// dependencies are exactly its body references and member declarations,
/// plus the optional protocol-clause owner-model edge.
fn application_node_shape_is_valid(node: &Value) -> bool {
    application_node_shape_locus(node).is_none()
}

/// The node member that makes an application-bearing node's shape invalid, as
/// a pointer relative to the node: `/dependencies` when the list differs from
/// the body's references, `/semantic_form` when the form does not fit the
/// operator, `""` when the node is no application-bearing node. `None` when
/// the shape is valid.
fn application_node_shape_locus(node: &Value) -> Option<&'static str> {
    let body = &node["body"];
    if !is_application_node(node) {
        return Some("");
    }
    let form_fits = node["node_tag"] != "expression"
        || body["term"] != "application"
        || body["operator"]
            .as_str()
            .and_then(expression_form_for_class)
            == node["semantic_form"].as_str();
    if !form_fits {
        return Some("/semantic_form");
    }
    let dependencies_fit = node["dependencies"]
        .as_array()
        .is_some_and(|actual| *actual == application_node_dependencies(node));
    (!dependencies_fit).then_some("/dependencies")
}

/// Whether an application-bearing node retains the JCS digest of its preimage.
fn application_node_key_is_current(node: &Value, nodes: &[Value]) -> bool {
    application_node_preimage(node, nodes).is_some_and(|preimage| {
        node["node_id"]["digest"].as_str() == Some(unqualified_jcs_sha256(&preimage).as_str())
    })
}

/// The `quire.structural-node/v1` preimage (QSL FR-092, FR-094) of a node that
/// is neither nominal nor application-bearing: the node's own members, its
/// wire `owner` exactly when it carries one, and a `null` `semantic_type`
/// when the node is its own type.
fn structural_node_preimage(node: &Value) -> Option<Value> {
    if node.get("nominal_identity_preimage").is_some()
        || node.get("recursion_group").is_some()
        || is_application_node(node)
    {
        return None;
    }
    let declaration = node.get("declaration").cloned().unwrap_or(Value::Null);
    let semantic_type = if node["semantic_type"] == node["node_id"] {
        Value::Null
    } else {
        node["semantic_type"].clone()
    };
    let mut preimage = json!({
        "version": "quire.structural-node/v1",
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": semantic_type,
        "declaration": declaration,
        "recursion": null,
        "body": node["body"],
    });
    if let Some(owner) = node.get("owner") {
        preimage["owner"] = owner.clone();
    }
    Some(preimage)
}

/// Whether a structural node retains the JCS digest of its preimage.
fn structural_node_key_is_current(node: &Value) -> bool {
    structural_node_preimage(node).is_some_and(|preimage| {
        node["node_id"]["digest"].as_str() == Some(unqualified_jcs_sha256(&preimage).as_str())
    })
}

/// The undeclared builtin and anonymous type shapes a reader re-derives from
/// the node's own body: boolean and integer scalars, the reference, option,
/// collection and model-derived composites, and the integer, collection-bound
/// and population ranges.
fn is_derived_shape(node: &Value) -> bool {
    node.get("declaration").is_none()
        && node.get("recursion_group").is_none()
        && matches!(
            (node["node_tag"].as_str(), node["semantic_form"].as_str()),
            (Some("scalar_type"), Some("boolean" | "integer"))
                | (
                    Some("composite_type"),
                    Some("reference" | "option" | "set" | "bag" | "sequence" | "ordered_set")
                )
                | (
                    Some("bounded_domain"),
                    Some("integer_range" | "collection_bounds" | "model_population")
                )
                | (Some("scalar_type"), Some("compound_unit"))
        )
}

/// A derived-shape node is current when its key is its preimage digest and a
/// scalar or composite node is its own `semantic_type`, which the preimage
/// records as `null`.
fn derived_shape_key_is_current(node: &Value) -> bool {
    let self_typed = node["node_tag"] == "bounded_domain"
        || node["semantic_form"] == "compound_unit"
        || node["semantic_type"] == node["node_id"];
    self_typed && structural_node_key_is_current(node)
}

/// Semantic checks owned by the derived-shape reader after key freshness.
/// These wire checks cover model-reference targets, population bounds and
/// compound-unit term order. QSL lowering owns richer domain checks.
fn derived_shape_semantic_refusal(nodes: &[Value]) -> Option<&Value> {
    let find = |id: &Value| nodes.iter().find(|node| node["node_id"] == *id);
    nodes.iter().find(
        |node| match (node["node_tag"].as_str(), node["semantic_form"].as_str()) {
            (Some("composite_type"), Some("reference")) => {
                let Some(target) = node["body"]["members"]
                    .as_array()
                    .and_then(|members| members.first())
                    .and_then(|member| member["target"].as_object())
                    .map(|target| Value::Object(target.clone()))
                    .and_then(|target| find(&target))
                else {
                    return true;
                };
                // A derived Reference<O> names a model declaration. A record,
                // scalar, function or value is not a declaration target.
                !is_model_reference_target(target)
            }
            (Some("bounded_domain"), Some("model_population")) => {
                let set_shaped = node["semantic_type"]
                    .as_object()
                    .and_then(|id| find(&Value::Object(id.clone())))
                    .is_some_and(|semantic_type| {
                        semantic_type["node_tag"] == "composite_type"
                            && semantic_type["semantic_form"] == "set"
                            && semantic_type["body"]["members"]
                                .as_array()
                                .and_then(|members| members.first())
                                .and_then(|member| member["target"].as_object())
                                .and_then(|target| find(&Value::Object(target.clone())))
                                .is_some_and(|element| {
                                    element["node_tag"] == "composite_type"
                                        && element["semantic_form"] == "reference"
                                })
                    });
                let Some(members) = node["body"]["members"].as_array() else {
                    return true;
                };
                // FR-322 "Derived type forms and forwarding": a population with
                // no `max` member is the unbounded population; a bounded one
                // carries exactly one canonical nonnegative integer `max`.
                let valid_max = |member: &Value| {
                    member["name"] == "max"
                        && member["term"] == "binding"
                        && member["value"]["term"] == "literal"
                        && member["value"]["value_kind"] == "integer"
                        && integer(&member["value"]["value"])
                            .is_some_and(|value| value >= BigInt::from(0))
                };
                let valid_body = match members.as_slice() {
                    [] => true,
                    [member] => valid_max(member),
                    _ => false,
                };
                !set_shaped || !valid_body
            }
            (Some("scalar_type"), Some("compound_unit")) => {
                if node["semantic_type"] != node["node_id"] {
                    return true;
                }
                let Some(terms) = node["body"]["members"].as_array() else {
                    return true;
                };
                let mut previous = None;
                terms.iter().any(|term| {
                    let Some(members) = term["members"].as_array() else {
                        return true;
                    };
                    let unit = members.iter().find(|member| member["name"] == "unit");
                    let exponent = members.iter().find(|member| member["name"] == "exponent");
                    let Some(unit_target) = unit
                        .and_then(|member| member["value"]["target"].as_object())
                        .map(|target| Value::Object(target.clone()))
                        .and_then(|target| find(&target))
                    else {
                        return true;
                    };
                    let Some(unit_digest) = unit_target["node_id"]["digest"].as_str() else {
                        return true;
                    };
                    let nominal_root =
                        unit_target
                            .get("nominal_identity_preimage")
                            .is_some_and(|preimage| {
                                preimage["version"] == "quire.unit-node/v1"
                                    && preimage["target_unit_node_id"].is_null()
                                    && unit_target["body"]
                                        == json!({"term": "aggregate", "members": []})
                                    && unit_target["semantic_type"] == preimage["dimension_node_id"]
                            });
                    let valid_root = unit_target["node_tag"] == "scalar_type"
                        && unit_target["semantic_form"] == "unit"
                        && (unit_target["semantic_type"] == unit_target["node_id"] || nominal_root);
                    let valid_exponent = exponent.is_some_and(|member| {
                        member["value"]["term"] == "literal"
                            && member["value"]["value_kind"] == "integer"
                            && integer(&member["value"]["value"])
                                .is_some_and(|value| value != BigInt::from(0))
                    });
                    let ordered = previous.is_none_or(|last: &str| last < unit_digest);
                    previous = Some(unit_digest);
                    !valid_root || !valid_exponent || !ordered
                })
            }
            _ => false,
        },
    )
}

/// A model declaration node (QSL FR-094): no `declaration`, the form of an
/// object type, a process, a state machine, a persistence interface, a
/// systems interface, a relationship or a population.
fn is_model_declaration(node: &Value) -> bool {
    node.get("declaration").is_none()
        && node.get("nominal_identity_preimage").is_none()
        && matches!(
            (node["node_tag"].as_str(), node["semantic_form"].as_str()),
            (
                Some("model"),
                Some(
                    "object_type"
                        | "process"
                        | "state_machine"
                        | "persistence_interface"
                        | "systems_interface"
                        | "tuple_value_type"
                        | "variant_type"
                )
            ) | (Some("relation"), Some("relationship" | "population"))
        )
}

/// A model declaration node is current only when it is its own type, carries
/// no `recursion_group`, has the empty aggregate body and retains the digest
/// of its closed `ModelDeclarationNode` preimage.
fn model_declaration_key_is_current(node: &Value) -> bool {
    node["semantic_type"] == node["node_id"]
        && node.get("recursion_group").is_none()
        && node["body"] == json!({"term": "aggregate", "members": []})
        && structural_node_key_is_current(node)
}

/// A node QSL FR-092 keys by `quire.structural-node/v1` with no input but the
/// node itself: no nominal preimage, no application in its body, and not a
/// `correspondence`/`abstraction_relation` node, which FR-451 keys by its own
/// `quire.abstraction-relation-node/v1` preimage. Model declaration nodes and
/// clause functions are included: their key is built from their wire `owner`.
fn is_rekeyed_structural_node(node: &Value) -> bool {
    let abstraction_relation =
        node["node_tag"] == "correspondence" && node["semantic_form"] == "abstraction_relation";
    node.get("nominal_identity_preimage").is_none()
        && !is_application_node(node)
        && !abstraction_relation
}

/// The members of `label`'s recursion group, in graph order: a member's
/// ordinal is its position here (FR-322 reads ordinals from graph order).
fn recursion_group_members<'a>(nodes: &'a [Value], label: &str) -> Vec<&'a Value> {
    nodes
        .iter()
        .filter(|node| node["recursion_group"].as_str() == Some(label))
        .collect()
}

/// QSL FR-092 "An in-group node's preimage", without the `group` member: the
/// group-local `quire.structural-node/v1` preimage of the structural member
/// at `member`, in which every position naming a member of the group, a body
/// `reference` or the `semantic_type`, becomes `{term: "group_reference",
/// ordinal}`. `None` when a declared member has no declaring source.
fn group_local_structural_preimage(group: &[&Value], member: usize) -> Option<Value> {
    let ordinal_of = |id: &Value| group.iter().position(|node| node["node_id"] == *id);
    fn in_group(term: &Value, ordinal_of: &dyn Fn(&Value) -> Option<usize>) -> Value {
        match term {
            Value::Object(members) if members.get("term") == Some(&json!("reference")) => {
                match ordinal_of(&members["target"]) {
                    Some(ordinal) => json!({"term": "group_reference", "ordinal": ordinal}),
                    None => term.clone(),
                }
            }
            Value::Object(members) => Value::Object(
                members
                    .iter()
                    .map(|(name, value)| (name.clone(), in_group(value, ordinal_of)))
                    .collect(),
            ),
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .map(|item| in_group(item, ordinal_of))
                    .collect(),
            ),
            other => other.clone(),
        }
    }
    let node = group.get(member)?;
    let semantic_type = if node["semantic_type"] == node["node_id"] {
        Value::Null
    } else {
        match ordinal_of(&node["semantic_type"]) {
            Some(ordinal) => json!({"term": "group_reference", "ordinal": ordinal}),
            None => node["semantic_type"].clone(),
        }
    };
    let mut preimage = json!({
        "version": "quire.structural-node/v1",
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": semantic_type,
        "declaration": node.get("declaration").cloned().unwrap_or(Value::Null),
        "recursion": {"size": group.len(), "ordinal": member},
        "body": in_group(&node["body"], &ordinal_of),
    });
    if let Some(owner) = node.get("owner") {
        preimage["owner"] = owner.clone();
    }
    Some(preimage)
}

/// QSL FR-092 group digest: the SHA-256 of the JCS array whose `k`-th element
/// is the digest of the group-local preimage of ordinal `k`, structural or
/// application (an in-group application node's group-local preimage is its
/// FR-322 preimage).
fn recursion_group_digest(nodes: &[Value], group: &[&Value]) -> Option<String> {
    let local_keys = group
        .iter()
        .enumerate()
        .map(|(member, node)| {
            if !is_application_node(node) {
                group_local_structural_preimage(group, member)
            } else {
                application_node_preimage(node, nodes)
            }
            .map(|preimage| unqualified_jcs_sha256(&preimage))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(unqualified_jcs_sha256(&json!(local_keys)))
}

/// Key of the structural group member at `member` under the group digest
/// `group_digest`: its group-local preimage with `recursion.group` set.
fn group_member_key(group: &[&Value], member: usize, group_digest: &str) -> Option<String> {
    let mut preimage = group_local_structural_preimage(group, member)?;
    preimage["recursion"]["group"] = json!(group_digest);
    Some(unqualified_jcs_sha256(&preimage))
}

/// An in-group structural node is current against its retained
/// `recursion_group` when its key is the digest of its group-local preimage
/// with that label as `recursion.group`. Whether the label is the group's
/// digest is the reader's next check.
fn group_member_key_is_current(nodes: &[Value], node: &Value) -> bool {
    let Some(label) = node["recursion_group"].as_str() else {
        return false;
    };
    let group = recursion_group_members(nodes, label);
    group
        .iter()
        .position(|member| member["node_id"] == node["node_id"])
        .and_then(|member| group_member_key(&group, member, label))
        .is_some_and(|key| node["node_id"]["digest"].as_str() == Some(key.as_str()))
}

/// The first node of the derived-shape stage whose retained key is stale, in
/// its two passes: each re-keyed structural node, grouped or not, against its
/// own preimage (a derived shape also self-typed), ascending by node-id
/// digest; then each recursion group whose retained label is not its
/// recomputed group digest, reported at its lowest node-id digest member.
fn stale_derived_shape_node(nodes: &[Value]) -> Option<&Value> {
    let mut keyed = nodes
        .iter()
        .filter(|node| is_rekeyed_structural_node(node))
        .collect::<Vec<_>>();
    keyed.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    let stale_member = keyed.into_iter().find(|node| {
        if node.get("recursion_group").is_some() {
            !group_member_key_is_current(nodes, node)
        } else if is_model_declaration(node) {
            !model_declaration_key_is_current(node)
        } else if is_derived_shape(node) {
            !derived_shape_key_is_current(node)
        } else {
            !structural_node_key_is_current(node)
        }
    });
    stale_member.or_else(|| {
        nodes
            .iter()
            .filter_map(|node| node["recursion_group"].as_str())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter_map(|label| {
                let group = recursion_group_members(nodes, label);
                if recursion_group_digest(nodes, &group).is_some_and(|digest| digest == label) {
                    return None;
                }
                group
                    .into_iter()
                    .min_by_key(|node| node["node_id"]["digest"].as_str())
            })
            .min_by_key(|node| node["node_id"]["digest"].as_str())
    })
}

/// The declarations of the admitted document of the `model_selections` row
/// selecting `identity`, by FR-322 "Model-owned members" step 1: the
/// caller-supplied document whose RFC 8785 digest is the row's. The document
/// is the package's own `model_bindings.domain_package` when it carries one
/// (already in projection form), else the published `acme/orders` semantic IR
/// document, whose declarations are the `domain_package` projection the
/// vectors derive from it.
fn selected_model_document(package: &Value, identity: &str) -> Option<Value> {
    let row = package["lock"]["model_selections"]
        .as_array()?
        .iter()
        .find(|row| row["identity"] == identity)?;
    let selects = |document: &Value| {
        row["digest_domain"] == "sha256-jcs"
            && row["digest"].as_str() == Some(unqualified_jcs_sha256(document).as_str())
    };
    if let Some(own) = package.pointer("/model_bindings/domain_package")
        && own["identity"] == identity
        && selects(own)
    {
        return Some(own.clone());
    }
    let shapes = parsed_fixture(SHAPES_DOCUMENT);
    if shapes["package"]["identity"] == identity && selects(&shapes) {
        return Some(parsed_fixture(MODEL_TUPLE_UNION_VECTORS)["domain_package"].clone());
    }
    let document = parsed_fixture(DOMAIN_DOCUMENT);
    (document["package"]["identity"] == identity && selects(&document))
        .then(|| parsed_fixture(MODEL_MEMBER_TYPE_VECTORS)["domain_package"].clone())
}

/// A clause function's clause kind: the text literal of the one `clause`
/// binding of its body (QSL FR-094): `invariant`, `precondition` or `body`.
/// `None` when the binding is absent, repeated or not a text literal.
fn clause_kind(node: &Value) -> Option<&str> {
    let mut bindings = node["body"]["members"]
        .as_array()?
        .iter()
        .filter(|member| member["term"] == "binding" && member["name"] == "clause");
    let binding = bindings.next().filter(|_| bindings.next().is_none())?;
    (binding["value"]["term"] == "literal" && binding["value"]["value_kind"] == "text")
        .then(|| binding["value"]["value"].as_str())
        .flatten()
}

/// Whether `document` declares the model-owned node `owner_node` as the kind
/// the wire node stands for: on a model declaration node, an object type
/// (`object_type`, no `interfaceFeatures`), an interface (`systems_interface`),
/// a process, a state machine, a persistence interface or a relationship; on a
/// clause function, an operation member of an object type, or an object type
/// itself for an invariant clause.
fn document_declares_owner(document: &Value, node: &Value, owner_node: &str) -> bool {
    let object_types = document["object_types"].as_array().into_iter().flatten();
    let declares_in = |list: &str| {
        document[list]
            .as_array()
            .into_iter()
            .flatten()
            .any(|declared| declared["node"] == owner_node)
    };
    // A population is declared by its FCD record, keyed by `identity`.
    let declares_population = || {
        document["populations"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|population| population["identity"] == owner_node)
    };
    let declares_object_type = |interface: bool| {
        object_types.clone().any(|object| {
            object["node"] == owner_node && object["interfaceFeatures"].is_null() != interface
        })
    };
    match (node["node_tag"].as_str(), node["semantic_form"].as_str()) {
        (Some("model"), Some("object_type")) => declares_object_type(false),
        (Some("model"), Some("systems_interface")) => declares_object_type(true),
        (Some("model"), Some("process")) => declares_in("processes"),
        (Some("model"), Some("state_machine")) => declares_in("state_machines"),
        (Some("model"), Some("persistence_interface")) => declares_in("persistence_interfaces"),
        (Some("model"), Some("tuple_value_type")) => declares_in("tuple_types"),
        (Some("model"), Some("variant_type")) => declares_in("variant_types"),
        (Some("relation"), Some("relationship")) => document["relationships"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|relationship| relationship["node"] == owner_node),
        (Some("relation"), Some("population")) => declares_population(),
        (Some("function"), _) => match clause_kind(node) {
            // An invariant clause is owned by its object type.
            Some("invariant") => object_types.clone().any(|object| {
                object["node"] == owner_node && object["interfaceFeatures"].is_null()
            }),
            // A precondition or body clause is owned by an operation member
            // of an object type, the operation's context.
            Some("precondition" | "body") => object_types.clone().any(|object| {
                object["operations"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|operation| {
                        operation["name"].as_str().is_some_and(|name| {
                            owner_node
                                == format!("{}/{name}", object["node"].as_str().unwrap_or_default())
                        })
                    })
            }),
            _ => false,
        },
        _ => false,
    }
}

/// A derived `Reference<O>` target (QSL FR-322): a declared model form,
/// whether source-declared in this package or resolved through a `ModelOwner`.
fn is_model_reference_target(node: &Value) -> bool {
    node.get("nominal_identity_preimage").is_none()
        && matches!(
            (node["node_tag"].as_str(), node["semantic_form"].as_str()),
            (
                Some("model"),
                Some("object_type" | "process" | "state_machine" | "systems_interface")
            ) | (Some("relation"), Some("relationship"))
        )
        && ((node.get("declaration").is_some() && node["owner"]["kind"] == "source")
            || (node.get("declaration").is_none() && node["owner"]["kind"] == "model"))
}

/// FR-322's derived-shape key stage, owner join: every owner-bearing node,
/// ascending by retained node-id digest. A `SourceOwner` joins a
/// `lock.sources` entry (`missing-selection` at the node) and agrees with the
/// source of every region of the node's `declaration` occurrence
/// (`invalid-value` at that source-map entry); a `ModelOwner` joins a
/// `model_selections` row whose admitted document declares its `node`
/// (`missing-selection` at the node).
fn owner_join_refusal(package: &Value, nodes: &[Value]) -> Result<(), ReadRefusal> {
    let mut owned = nodes
        .iter()
        .filter(|node| node.get("owner").is_some())
        .collect::<Vec<_>>();
    owned.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    for node in owned {
        let owner = &node["owner"];
        let unjoined = ReadRefusal::OwnerSelection(node["node_id"].clone());
        match owner["kind"].as_str() {
            Some("source") => {
                let same_source = |source: &Value| {
                    source["authority"] == owner["authority"]
                        && source["identity"] == owner["identity"]
                };
                if !package["lock"]["sources"]
                    .as_array()
                    .is_some_and(|sources| sources.iter().any(same_source))
                {
                    return Err(unjoined);
                }
                let moved = package["source_map"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .find(|(_, entry)| {
                        entry["node_id"] == node["node_id"]
                            && entry["role"] == "declaration"
                            && entry["regions"].as_array().is_some_and(|regions| {
                                regions.iter().any(|region| !same_source(&region["source"]))
                            })
                    });
                if let Some((index, _)) = moved {
                    return Err(ReadRefusal::InvalidOwnerValue(format!(
                        "/source_map/{index}"
                    )));
                }
            }
            Some("model") => {
                let declared = owner["identity"]
                    .as_str()
                    .zip(owner["node"].as_str())
                    .is_some_and(|(identity, owner_node)| {
                        selected_model_document(package, identity).is_some_and(|document| {
                            document_declares_owner(&document, node, owner_node)
                        })
                    });
                if !declared {
                    return Err(unjoined);
                }
            }
            Some("model_root") => {
                let joined = package["lock"]["model_selections"]
                    .as_array()
                    .is_some_and(|rows| {
                        rows.iter().any(|row| {
                            row["identity"] == owner["identity"] && row["digest"] == owner["digest"]
                        })
                    });
                if !joined {
                    return Err(unjoined);
                }
            }
            _ => return Err(unjoined),
        }
    }
    Ok(())
}

/// Both cross-field constraints binding an application-bearing node.
fn application_node_is_valid(node: &Value, nodes: &[Value]) -> bool {
    application_node_shape_is_valid(node) && application_node_key_is_current(node, nodes)
}

/// First semantic refusal of a schema-valid V2 package in the normative
/// reader order: graph shape, then a stale application-node key, then a stale
/// derived-shape or in-group structural key, then declaration refusals, then
/// operation refusals. Nodes are visited ascending by retained node-id digest.
#[derive(Clone, Debug, PartialEq)]
enum ReadRefusal {
    GraphShape(Value),
    /// `invalid_model_binding`/`malformed-declaration` at `package_read`: a
    /// `populations` entry of the selected document that is no FCD population.
    /// The locus is the selection row (`/lock/model_selections/<i>`).
    PopulationRecord(String),
    /// `missing_declaration`/`missing-selection` at a node whose `owner` the
    /// lock does not select or whose selected document does not declare it.
    OwnerSelection(Value),
    /// `invalid_package`/`invalid-value` at a `/source_map/{i}` entry whose
    /// region names a source other than its declared node's `SourceOwner`.
    InvalidOwnerValue(String),
    StaleNodeKey(Value),
    /// `invalid_semantic_graph` at an `enum_value` node over a
    /// `model`/`variant_type` whose case the selected document does not
    /// declare or whose variant type is of family `union`.
    InvalidSemanticGraph(Value),
    Declaration(DeclarationRefusal, Value),
    Frame(FrameRefusal, Value),
    State(StateRefusal, Value),
    /// `invalid_semantic_graph` at an `abstraction_relation` node whose
    /// identity step (FR-451, step 1) fails.
    Abstraction(Value),
    /// A `dependency_selections` supply or `dependency_reference` refusal
    /// (FR-322): its code, cause and JSON-pointer locus.
    Dependency(&'static str, &'static str, String),
    Operation(OperationRefusal),
}

impl ReadRefusal {
    /// Cause and code of the `stale-node-key` refusal.
    const STALE_NODE_KEY: (&'static str, &'static str) = ("stale-node-key", "invalid_package");
}

/// Code catalogued for a package-read cause, when the cause is one.
fn package_cause_code(cause: &str) -> Option<&'static str> {
    OperationRefusal::from_cause(cause)
        .map(OperationRefusal::code)
        .or_else(|| {
            DeclarationRefusal::ALL
                .into_iter()
                .find(|refusal| refusal.cause() == cause)
                .map(DeclarationRefusal::code)
        })
        .or_else(|| {
            FrameRefusal::ALL
                .into_iter()
                .find(|refusal| refusal.cause() == cause)
                .map(FrameRefusal::code)
        })
        .or_else(|| {
            (cause == ReadRefusal::STALE_NODE_KEY.0).then_some(ReadRefusal::STALE_NODE_KEY.1)
        })
}

fn read_package_semantics(package: &Value, catalog: &Value) -> Result<(), ReadRefusal> {
    read_package_semantics_with(package, catalog, &[])
}

/// `read_package_semantics` for a reader supplied the admitted packages of a
/// package's `dependency_selections`, as `(identity, package)` pairs.
fn read_package_semantics_with(
    package: &Value,
    catalog: &Value,
    supplied: &[(String, Value)],
) -> Result<(), ReadRefusal> {
    let dependencies = dependency_supply_refusal(package, supplied)?;
    // FR-322 "Model-owned members" step 1 reads the selected document before
    // graph admission; an entry that is no FCD population refuses it.
    if package
        .pointer("/model_bindings/domain_package/populations")
        .and_then(Value::as_array)
        .is_some_and(|populations| {
            populations.iter().any(|population| {
                !population["identity"].is_string() || population.get("node").is_some()
            })
        })
    {
        let row = package["lock"]["model_selections"]
            .as_array()
            .and_then(|rows| rows.iter().position(|row| row["identity"] == "acme/orders"))
            .expect("the selection row of the read document");
        return Err(ReadRefusal::PopulationRecord(format!(
            "/lock/model_selections/{row}"
        )));
    }
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let mut ordered = nodes
        .iter()
        .filter(|node| is_application_node(node))
        .collect::<Vec<_>>();
    ordered.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    if let Some(node) = ordered
        .iter()
        .find(|node| !application_node_shape_is_valid(node))
    {
        return Err(ReadRefusal::GraphShape(node["node_id"].clone()));
    }
    if let Some(node) = ordered
        .iter()
        .find(|node| !application_node_key_is_current(node, nodes))
    {
        return Err(ReadRefusal::StaleNodeKey(node["node_id"].clone()));
    }
    owner_join_refusal(package, nodes)?;
    if let Some(node) = stale_derived_shape_node(nodes) {
        return Err(ReadRefusal::StaleNodeKey(node["node_id"].clone()));
    }
    if let Some(node) = derived_shape_semantic_refusal(nodes) {
        return Err(ReadRefusal::GraphShape(node["node_id"].clone()));
    }
    let oracle = OperationOracle::for_package_with(catalog, package, &dependencies);
    if let Some(node) = nodes.iter().find(|node| oracle.enum_value_is_refused(node)) {
        return Err(ReadRefusal::InvalidSemanticGraph(node["node_id"].clone()));
    }
    package_declaration_refusal(package)
        .map_err(|(refusal, node)| ReadRefusal::Declaration(refusal, node))?;
    package_frame_refusal(package)
        .map_err(|(refusal, locus)| ReadRefusal::Frame(refusal, locus))?;
    package_state_refusal(package)
        .map_err(|(refusal, locus)| ReadRefusal::State(refusal, locus))?;
    abstraction_identity_refusal(nodes).map_err(ReadRefusal::Abstraction)?;
    dependency_reference_refusal(package, &dependencies)?;
    package_operations_refusal_with(package, catalog, &dependencies).map_err(ReadRefusal::Operation)
}

/// FR-322 supply order: each `dependency_selections` entry in lock order is
/// supplied (`missing_import`/`missing-selection`) and then binds, its
/// supplied package recomputing the entry's `package_id`
/// (`stale_dependency`/`content-mismatch`), each at the entry's pointer.
/// Returns the bound packages, in lock order.
fn dependency_supply_refusal(
    package: &Value,
    supplied: &[(String, Value)],
) -> Result<Vec<(String, Value)>, ReadRefusal> {
    let mut bound = Vec::new();
    for (index, entry) in package["lock"]["dependency_selections"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        let locus = format!("/lock/dependency_selections/{index}");
        let Some((identity, dependency)) = supplied
            .iter()
            .find(|(identity, _)| entry["identity"] == identity.as_str())
        else {
            return Err(ReadRefusal::Dependency(
                "missing_import",
                "missing-selection",
                locus,
            ));
        };
        if entry["package_id"]["digest"] != unqualified_jcs_sha256(&package_id_preimage(&dependency))
        {
            return Err(ReadRefusal::Dependency(
                "stale_dependency",
                "content-mismatch",
                locus,
            ));
        }
        bound.push((identity.clone(), dependency.clone()));
    }
    Ok(bound)
}

/// The node a `dependency_reference` names in the bound dependency package
/// whose `package_id` is the term's `package`, when there is one.
fn dependency_reference_target<'a>(
    dependencies: &'a [(String, Value)],
    term: &Value,
) -> Option<&'a Value> {
    dependencies
        .iter()
        .find(|(_, dependency)| dependency["package_id"] == term["package"])?
        .1["semantic_graph"]["nodes"]
        .as_array()?
        .iter()
        .find(|node| node["node_id"] == term["node"])
}

/// Whether a dependency `function`'s parameter and result types are
/// package-independent (FR-322): no node in the transitive `dependencies`
/// closure of its signature type nodes -- each `parameters` binding
/// target's `semantic_type` and its own `semantic_type` -- carries a
/// `declaration` or a `ModelOwner`.
fn dependency_signature_is_package_independent(function: &Value, nodes: &[Value]) -> bool {
    let find = |id: &Value| nodes.iter().find(|node| node["node_id"] == *id);
    let parameters = function["body"]["members"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|member| member["term"] == "binding" && member["name"] == "parameters");
    let mut pending = vec![function["semantic_type"].clone()];
    for parameter in parameters
        .and_then(|binding| binding["value"]["members"].as_array())
        .into_iter()
        .flatten()
    {
        match find(&parameter["target"]) {
            Some(node) => pending.push(node["semantic_type"].clone()),
            None => return false,
        }
    }
    let mut seen = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.to_string()) {
            continue;
        }
        let Some(node) = find(&id) else {
            return false;
        };
        if node.get("declaration").is_some() || node["owner"]["kind"] == "model" {
            return false;
        }
        pending.extend(
            node["dependencies"]
                .as_array()
                .into_iter()
                .flatten()
                .cloned(),
        );
    }
    true
}

/// FR-322's `dependency_reference` check over every node, ascending by
/// node-id digest, at each term's place in its node's pre-order term walk:
/// a `package` absent from the direct `dependency_selections`
/// (`missing_declaration`/`missing-selection` at the term's `package`), then a
/// `node` carrying no `declaration` in that dependency
/// (`missing_declaration`/`missing-name` at its `node`), then a target that
/// is no package-independent `function` or a term that is not the callee of a
/// `quire.op.function.call` (`ill_typed`/`operator-ineligible` at the term).
fn dependency_reference_refusal(
    package: &Value,
    dependencies: &[(String, Value)],
) -> Result<(), ReadRefusal> {
    fn walk<'t>(
        term: &'t Value,
        pointer: String,
        callee: bool,
        out: &mut Vec<(String, &'t Value, bool)>,
    ) {
        match term["term"].as_str() {
            Some("dependency_reference") => out.push((pointer, term, callee)),
            Some("application") => {
                let call = term["operation"]["identity"] == "quire.op.function.call";
                for (index, argument) in term["arguments"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .enumerate()
                {
                    walk(
                        argument,
                        format!("{pointer}/arguments/{index}"),
                        call && index == 0,
                        out,
                    );
                }
            }
            Some("aggregate") => {
                for (index, member) in term["members"].as_array().into_iter().flatten().enumerate()
                {
                    walk(member, format!("{pointer}/members/{index}"), false, out);
                }
            }
            Some("binding") => walk(&term["value"], format!("{pointer}/value"), false, out),
            _ => {}
        }
    }
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let mut ordered = nodes.iter().enumerate().collect::<Vec<_>>();
    ordered.sort_by_key(|(_, node)| node["node_id"]["digest"].as_str().unwrap_or_default());
    let selections = package["lock"]["dependency_selections"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    for (index, node) in ordered {
        let mut terms = Vec::new();
        walk(
            &node["body"],
            format!("/semantic_graph/nodes/{index}/body"),
            false,
            &mut terms,
        );
        for (pointer, term, callee) in terms {
            if !selections
                .iter()
                .any(|entry| entry["package_id"] == term["package"])
            {
                return Err(ReadRefusal::Dependency(
                    "missing_declaration",
                    "missing-selection",
                    format!("{pointer}/package"),
                ));
            }
            let library = dependencies
                .iter()
                .find(|(_, dependency)| dependency["package_id"] == term["package"])
                .map(|(_, dependency)| {
                    dependency["semantic_graph"]["nodes"]
                        .as_array()
                        .map(Vec::as_slice)
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            let Some(target) = dependency_reference_target(dependencies, term)
                .filter(|target| target.get("declaration").is_some())
            else {
                return Err(ReadRefusal::Dependency(
                    "missing_declaration",
                    "missing-name",
                    format!("{pointer}/node"),
                ));
            };
            if !callee
                || target["node_tag"] != "function"
                || !dependency_signature_is_package_independent(target, library)
            {
                return Err(ReadRefusal::Dependency(
                    "ill_typed",
                    "operator-ineligible",
                    pointer,
                ));
            }
        }
    }
    Ok(())
}

/// FR-451's abstraction step 1 (Identity) over every
/// `correspondence`/`abstraction_relation` node, ascending by node-id digest:
/// the node's key is the digest of `{version, body}`, its `semantic_type` is
/// its own id and its `dependencies` are exactly the unique digest-ascending
/// `type`, `population` and `context` targets of its body.
fn abstraction_identity_refusal(nodes: &[Value]) -> Result<(), Value> {
    let mut relations = nodes
        .iter()
        .filter(|node| {
            node["node_tag"] == "correspondence" && node["semantic_form"] == "abstraction_relation"
        })
        .collect::<Vec<_>>();
    relations.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    for node in relations {
        let body = &node["body"];
        let key = unqualified_jcs_sha256(&json!({
            "version": "quire.abstraction-relation-node/v1",
            "body": body,
        }));
        let mut expected = [
            ("objects", "type"),
            ("populations", "population"),
            ("frames", "context"),
        ]
        .into_iter()
        .flat_map(|(list, member)| {
            body[list]
                .as_array()
                .into_iter()
                .flatten()
                .map(move |entry| entry[member].clone())
        })
        .collect::<Vec<_>>();
        expected.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
        expected.dedup();
        let holds = node["node_id"]["digest"].as_str() == Some(key.as_str())
            && node["semantic_type"] == node["node_id"]
            && node["dependencies"].as_array() == Some(&expected);
        if !holds {
            return Err(node["node_id"].clone());
        }
    }
    Ok(())
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

/// Whether a V2 node is a named, source-declared declaration and so must
/// carry the `declaration` member: a declarable tag with a declaration
/// occurrence. Anonymous and builtin nodes, and every other tag, must not.
fn node_declares(node: &Value) -> bool {
    let declarable = match node["node_tag"].as_str() {
        Some(
            "scalar_type" | "composite_type" | "bounded_domain" | "function" | "model" | "protocol"
            | "claim" | "extension" | "hybrid",
        ) => true,
        Some("value") => node["semantic_form"] != "enum_value",
        _ => false,
    };
    declarable
        && node["occurrences"]
            .as_array()
            .is_some_and(|occurrences| occurrences.iter().any(|o| o["role"] == "declaration"))
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
        // FR-322 "declaration": a name is unique within its namespace, and
        // dimensions, units and every other declared type are three namespaces.
        let namespace = match (node["node_tag"].as_str(), node["semantic_form"].as_str()) {
            (Some("scalar_type"), Some("dimension")) => "dimension",
            (Some("scalar_type"), Some("unit")) => "unit",
            _ => "type",
        };
        if let Some(name) = node
            .get("declaration")
            .filter(|declaration| !declaration.is_null())
            .map(|d| format!("{namespace}:{}", d["qualified_name"]))
            && !seen.insert(name)
        {
            return Err((DeclarationRefusal::AmbiguousName, node["node_id"].clone()));
        }
    }
    Ok(())
}

/// The frame node's own `semantic_type` and its body's three named members,
/// in the fixed order FR-340 states: `semantic_type`, then `modifies`,
/// `creates`, `deletes`. This is a spec fact about the shape, never an
/// authored array position, and it is the first tie-break between two
/// simultaneous defects in different members.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FrameGroup {
    SemanticType,
    Modifies,
    Creates,
    Deletes,
    DeletesScoped,
}

impl FrameGroup {
    const MEMBERS: [Self; 3] = [Self::Modifies, Self::Creates, Self::Deletes];

    fn key(self) -> &'static str {
        match self {
            Self::SemanticType => "semantic_type",
            Self::Modifies => "modifies",
            Self::Creates => "creates",
            Self::Deletes => "deletes",
            Self::DeletesScoped => "deletes_scoped",
        }
    }
}

/// The declaration a frame entry names, read from where it stands: a
/// `modifies` entry is a relationship or a field (its `kind`), and a
/// `creates`/`deletes` entry names an object type or process whose
/// instances the operation creates or deletes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FrameEntryKind {
    Relationship,
    Field,
    Instance,
}

/// Whether the declaring node an entry names carries the meaning its kind
/// needs, read from the node's tag and form alone. A relationship entry
/// names a `relation`/`relationship` node; a field entry names the
/// `model`/`object_type`, `model`/`record_value_type`, `model`/`process` or
/// `model`/`state_machine` node declaring the field; a `creates`/`deletes` entry names a `model`/`object_type` or
/// `model`/`process` node. Every other node, including a `value` literal and
/// every systems-model form, has no frame meaning.
fn frame_entry_admits(kind: FrameEntryKind, node: &Value) -> bool {
    matches!(
        (
            kind,
            node["node_tag"].as_str(),
            node["semantic_form"].as_str()
        ),
        (
            FrameEntryKind::Relationship,
            Some("relation"),
            Some("relationship")
        ) | (
            FrameEntryKind::Field,
            Some("model"),
            Some("object_type" | "record_value_type" | "process" | "state_machine")
        ) | (
            FrameEntryKind::Instance,
            Some("model"),
            Some("object_type" | "process")
        )
    )
}

/// The entries of one frame member as (declaring node digest, field name,
/// kind); a relationship or `creates`/`deletes` entry has the empty name.
fn frame_entries(
    frame_node: &Value,
    group: FrameGroup,
) -> Vec<(String, String, FrameEntryKind, u8)> {
    frame_node["body"][group.key()]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| match group {
            FrameGroup::Modifies => {
                let digest = entry["declaration"]["digest"].as_str()?.to_owned();
                match entry["kind"].as_str()? {
                    // The scope enters the order key, `every-object` first.
                    "field" => Some((
                        digest,
                        entry["name"].as_str()?.to_owned(),
                        FrameEntryKind::Field,
                        u8::from(entry["scope"] == "receiver"),
                    )),
                    "relationship" => {
                        Some((digest, String::new(), FrameEntryKind::Relationship, 0))
                    }
                    _ => None,
                }
            }
            _ => Some((
                entry["digest"].as_str()?.to_owned(),
                String::new(),
                FrameEntryKind::Instance,
                0,
            )),
        })
        .collect()
}

fn node_by_digest<'a>(nodes: &'a [Value], digest: &str) -> Option<&'a Value> {
    nodes
        .iter()
        .find(|node| node["node_id"]["digest"].as_str() == Some(digest))
}

/// Why one frame entry, or the frame's `semantic_type`, fails to join: the
/// target names no declared dependency of the frame node (`missing-name`,
/// shared with an entry naming no node of the package at all), or it names a
/// node of a meaning the position cannot carry (`malformed-declaration`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FrameJoinCause {
    MissingName,
    MalformedDeclaration,
}

/// One defect of a `state`/`frame` node. The variant order below, not
/// iteration or array position, is what makes `MeaningJoin` outrank
/// `CanonicalOrder`: `derive(Ord)` compares the declared variants first, so
/// any meaning-join defect anywhere in the node sorts before any
/// canonical-order defect anywhere in its body. Within one kind, ties break
/// by position (`semantic_type` < `modifies` < `creates` < `deletes`), then
/// by the offending entry's declaring node digest and then its field name,
/// never by its position in the array.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FrameDefect {
    MeaningJoin {
        group: FrameGroup,
        digest: String,
        name: String,
        cause: FrameJoinCause,
    },
    /// Two field entries of one declaration and name with different scopes
    /// (`conflicting-binding`): a meaning-join defect that ranks after every
    /// other one and before any canonical-order defect, by its place here.
    ConflictingBinding {
        digest: String,
        name: String,
    },
    CanonicalOrder {
        group: FrameGroup,
    },
}

/// FR-322's "Model-owned members" steps 2 and 3, reused by the full reader
/// so `frame_body_defects` (FR-340) and `anchor_refusal`/`clause_refusal`
/// (FR-341/FR-342) resolve a member's NAME against a real model, not just
/// its declaring node's tag and form. Sourced from a package's own
/// `model_bindings` test field when present: a domain package (the same
/// `object_types`/`fields`/`operations` shape TC-280's standalone oracle
/// reads) plus the model declaration nodes mapping a checked-package node
/// digest to the domain declaration it recovers. The published schema
/// carries no `model_bindings` key, so every published fixture has no
/// `ModelIndex` and keeps its existing (name-blind) coverage exactly.
struct ModelIndex<'a> {
    domain_package: &'a Value,
    declaration_nodes: &'a [Value],
}

fn model_index(package: &Value) -> Option<ModelIndex<'_>> {
    let bindings = package.get("model_bindings")?;
    Some(ModelIndex {
        domain_package: &bindings["domain_package"],
        declaration_nodes: bindings["model_declaration_nodes"].as_array()?,
    })
}

/// The domain declaration node identity (FR-322 step 2's `owner.node`) a
/// checked-package node `digest` recovers, by the fixture's own stated key
/// (already recomputed and asserted correct by the TC-280 oracle this
/// fixture data comes from).
fn mm_owner_node<'a>(declaration_nodes: &'a [Value], digest: &str) -> Option<&'a str> {
    declaration_nodes
        .iter()
        .find(|node| node["sha256"] == digest)
        .and_then(|node| node["preimage"]["owner"]["node"].as_str())
}

/// FR-322 step 3 (own members only; no inheritance or redefinition) for one
/// object type's, process's or state machine's `list` ("fields" or "operations") member named `name`.
fn mm_declared_member<'a>(
    domain_package: &'a Value,
    node: &str,
    list: &str,
    name: &str,
) -> Option<&'a Value> {
    ["object_types", "processes", "state_machines"]
        .into_iter()
        .filter_map(|kind| domain_package[kind].as_array())
        .flatten()
        .find(|declared| declared["node"] == node)?[list]
        .as_array()?
        .iter()
        .find(|member| member["name"] == name)
}

/// Every defect of one frame node against a schema-valid package's other
/// nodes and the frame node's own `dependencies`. Entries are visited in a
/// fixed (group, then array order) sweep only to collect defects; which one
/// is reported is decided afterward by sorting, so the result never depends
/// on where in its array an authored entry sits.
fn frame_body_defects(
    frame_node: &Value,
    nodes: &[Value],
    model: Option<&ModelIndex<'_>>,
) -> Vec<FrameDefect> {
    let dependencies = frame_node["dependencies"]
        .as_array()
        .map(|deps| {
            deps.iter()
                .filter_map(|dependency| dependency["digest"].as_str())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut defects = Vec::new();
    let semantic_type = frame_node["semantic_type"]["digest"]
        .as_str()
        .unwrap_or_default();
    let declaring_type = node_by_digest(nodes, semantic_type);
    if !declaring_type.is_some_and(|node| {
        node["node_tag"] == "model"
            && matches!(
                node["semantic_form"].as_str(),
                Some("object_type" | "process" | "state_machine")
            )
    }) {
        defects.push(FrameDefect::MeaningJoin {
            group: FrameGroup::SemanticType,
            digest: semantic_type.to_owned(),
            name: String::new(),
            cause: if declaring_type.is_some() {
                FrameJoinCause::MalformedDeclaration
            } else {
                FrameJoinCause::MissingName
            },
        });
    }
    // FR-340 "Frames of a process or a state machine": the receiver of such an
    // operation is a run or a machine context, so the frame holds no
    // receiver-scoped member.
    if declaring_type.is_some_and(|node| {
        node["node_tag"] == "model"
            && matches!(
                node["semantic_form"].as_str(),
                Some("process" | "state_machine")
            )
    }) {
        for entry in frame_node["body"]["modifies"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|entry| entry["kind"] == "field" && entry["scope"] == "receiver")
        {
            defects.push(FrameDefect::MeaningJoin {
                group: FrameGroup::Modifies,
                digest: entry["declaration"]["digest"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                name: entry["name"].as_str().unwrap_or_default().to_owned(),
                cause: FrameJoinCause::MalformedDeclaration,
            });
        }
        if frame_node["body"]["deletes_scoped"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|entry| entry["kind"] == "receiver")
        {
            defects.push(FrameDefect::MeaningJoin {
                group: FrameGroup::DeletesScoped,
                digest: frame_node["node_id"]["digest"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                name: String::new(),
                cause: FrameJoinCause::MalformedDeclaration,
            });
        }
    }
    for group in FrameGroup::MEMBERS {
        let entries = frame_entries(frame_node, group);
        let keys = entries
            .iter()
            .map(|(digest, name, _, scope)| (digest.clone(), name.clone(), *scope))
            .collect::<Vec<_>>();
        let mut scopes = BTreeMap::<(String, String), BTreeSet<u8>>::new();
        for (digest, name, kind, scope) in &entries {
            if *kind == FrameEntryKind::Field {
                scopes
                    .entry((digest.clone(), name.clone()))
                    .or_default()
                    .insert(*scope);
            }
        }
        for ((digest, name), kinds) in scopes {
            if kinds.len() > 1 {
                defects.push(FrameDefect::ConflictingBinding { digest, name });
            }
        }
        let mut ascending = keys.clone();
        ascending.sort();
        if ascending != keys {
            defects.push(FrameDefect::CanonicalOrder { group });
        }
        for (digest, name, kind, _) in entries {
            let resolved = node_by_digest(nodes, &digest);
            let is_dependency = dependencies.contains(digest.as_str());
            let meaning_ok = resolved.is_some_and(|node| frame_entry_admits(kind, node));
            if is_dependency && meaning_ok {
                // FR-340's field entries name a declaring node AND a field
                // name (FR-340-AC-13); resolve the name too when a model is
                // available, so an entry naming a real object type but no
                // field of that name is still a meaning-join defect
                // (`missing-name`), not silently admitted.
                let name_ok = kind != FrameEntryKind::Field
                    || model.is_none_or(|model| {
                        mm_owner_node(model.declaration_nodes, &digest).is_some_and(|node| {
                            mm_declared_member(model.domain_package, node, "fields", &name)
                                .is_some()
                        })
                    });
                if name_ok {
                    continue;
                }
                defects.push(FrameDefect::MeaningJoin {
                    group,
                    digest,
                    name,
                    cause: FrameJoinCause::MissingName,
                });
                continue;
            }
            let cause = if resolved.is_some() && is_dependency {
                FrameJoinCause::MalformedDeclaration
            } else {
                FrameJoinCause::MissingName
            };
            defects.push(FrameDefect::MeaningJoin {
                group,
                digest,
                name,
                cause,
            });
        }
    }
    // `deletes_scoped` is ordered `receiver` first, then `parameter` entries by
    // name as UTF-8 bytes (FR-340-AC-17).
    let scoped = frame_node["body"]["deletes_scoped"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|entry| {
            (
                u8::from(entry["kind"] == "parameter"),
                entry["name"].as_str().unwrap_or_default().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    let mut ordered = scoped.clone();
    ordered.sort();
    if ordered != scoped {
        defects.push(FrameDefect::CanonicalOrder {
            group: FrameGroup::DeletesScoped,
        });
    }
    defects
}

/// The one defect a real reader reports for a defective frame body: the
/// least element of its (order-independent) defect set. `frame_body_defects`
/// collects in group/array order, but `reported_frame_defect` sorts before
/// picking, so the outcome is the same regardless of authoring order.
fn reported_frame_defect(
    frame_node: &Value,
    nodes: &[Value],
    model: Option<&ModelIndex<'_>>,
) -> Option<FrameDefect> {
    let mut defects = frame_body_defects(frame_node, nodes, model);
    defects.sort();
    defects.into_iter().next()
}

/// Semantic refusal of one frame body, carrying the wire cause/code FR-340
/// pins for it. `CanonicalOrder` follows FR-322-AC-10's `invalid_semantic_graph`
/// pattern for noncanonical member order and is not a `Diagnostic` cause pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FrameRefusal {
    MissingName,
    MalformedDeclaration,
    ConflictingBinding,
    CanonicalOrder,
}

impl FrameRefusal {
    /// The two wire-paired causes; `CanonicalOrder` is deliberately excluded,
    /// as it is not a `Diagnostic.cause_tag`/`code` pair.
    const ALL: [Self; 3] = [
        Self::MissingName,
        Self::MalformedDeclaration,
        Self::ConflictingBinding,
    ];

    fn cause(self) -> &'static str {
        match self {
            Self::MissingName => "missing-name",
            Self::MalformedDeclaration => "malformed-declaration",
            Self::ConflictingBinding => "conflicting-binding",
            // `invalid_semantic_graph` is one of FR-322's bare v2 refusal
            // codes: of the sixteen codes the v2 refusal vocabulary names,
            // only `invalid_package`, `ill_typed`, `ambiguous_declaration`,
            // `missing_declaration` and `invalid_model_binding` carry a
            // cause tag. `CanonicalOrder` has no cause to return,
            // `.cause()` is never called on it (excluded from `ALL`), and
            // this arm exists only so the match stays exhaustive as the
            // enum evolves.
            Self::CanonicalOrder => {
                unreachable!("CanonicalOrder carries no cause tag; use .code() instead")
            }
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::MissingName => "missing_declaration",
            Self::MalformedDeclaration | Self::ConflictingBinding => "invalid_model_binding",
            Self::CanonicalOrder => "invalid_semantic_graph",
        }
    }
}

impl From<FrameDefect> for FrameRefusal {
    fn from(defect: FrameDefect) -> Self {
        match defect {
            FrameDefect::MeaningJoin {
                cause: FrameJoinCause::MissingName,
                ..
            } => Self::MissingName,
            FrameDefect::MeaningJoin {
                cause: FrameJoinCause::MalformedDeclaration,
                ..
            } => Self::MalformedDeclaration,
            FrameDefect::ConflictingBinding { .. } => Self::ConflictingBinding,
            FrameDefect::CanonicalOrder { .. } => Self::CanonicalOrder,
        }
    }
}

/// Independent oracle over every `state`/`frame` node of a schema-valid V2
/// package; the refusal names the offending node (or, for a meaning-join
/// defect, the offending entry) so the locus never depends on array
/// position. Frame nodes are visited in ascending node-id digest order,
/// matching the package's other order-independent refusals.
fn package_frame_refusal(package: &Value) -> Result<(), (FrameRefusal, Value)> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let model = model_index(package);
    let mut frames = nodes
        .iter()
        .filter(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .collect::<Vec<_>>();
    frames.sort_by(|left, right| {
        left["node_id"]["digest"]
            .as_str()
            .cmp(&right["node_id"]["digest"].as_str())
    });
    for frame in frames {
        if let Some(defect) = reported_frame_defect(frame, nodes, model.as_ref()) {
            let locus = match &defect {
                FrameDefect::MeaningJoin { digest, .. }
                | FrameDefect::ConflictingBinding { digest, .. } => json!({
                    "domain": "quire.checked-semantic-node/v1",
                    "digest": digest,
                }),
                FrameDefect::CanonicalOrder { .. } => frame["node_id"].clone(),
            };
            return Err((FrameRefusal::from(defect), locus));
        }
    }
    Ok(())
}

/// Semantic refusal of a `state`/`operation_anchor` or `state`/`state_clause`
/// node (FR-341, FR-342), with the code and cause each pairs with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateRefusal {
    MissingName,
    MalformedDeclaration,
    OperatorIneligible,
    AmbiguousDeclaration,
}

impl StateRefusal {
    fn cause(self) -> &'static str {
        match self {
            Self::MissingName => "missing-name",
            Self::MalformedDeclaration => "malformed-declaration",
            Self::OperatorIneligible => "operator-ineligible",
            Self::AmbiguousDeclaration => "ambiguous-name",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::MissingName => "missing_declaration",
            Self::MalformedDeclaration => "invalid_model_binding",
            Self::OperatorIneligible => "ill_typed",
            Self::AmbiguousDeclaration => "ambiguous_declaration",
        }
    }
}

/// Join one `reference` a state node's body holds to a declared dependency
/// of that node with an admitted (`node_tag`, `semantic_form`): a target
/// that names no node of the package, or no declared dependency, refuses
/// `missing-name`; a declared dependency of another meaning refuses
/// `malformed-declaration`. The locus is the target.
fn join_state_reference<'a>(
    node: &Value,
    target: &Value,
    nodes: &'a [Value],
    admitted: &[(&str, &str)],
) -> Result<&'a Value, (StateRefusal, Value)> {
    let digest = target["digest"].as_str().unwrap_or_default();
    let is_dependency = node["dependencies"]
        .as_array()
        .is_some_and(|deps| deps.iter().any(|dep| dep["digest"] == digest));
    let resolved = node_by_digest(nodes, digest)
        .filter(|_| is_dependency)
        .ok_or((StateRefusal::MissingName, target.clone()))?;
    admitted
        .iter()
        .any(|(tag, form)| resolved["node_tag"] == *tag && resolved["semantic_form"] == *form)
        .then_some(resolved)
        .ok_or((StateRefusal::MalformedDeclaration, target.clone()))
}

/// The `model` nodes an anchor's `context` and an invariant's anchor may be:
/// an object type, a process or a state machine (FR-342, FR-341).
const ANCHOR_CONTEXTS: [(&str, &str); 3] = [
    ("model", "object_type"),
    ("model", "process"),
    ("model", "state_machine"),
];

/// The named binding's value in an `operation_anchor` body.
fn anchor_binding<'a>(anchor: &'a Value, name: &str) -> &'a Value {
    anchor["body"]["members"]
        .as_array()
        .and_then(|members| members.iter().find(|member| member["name"] == name))
        .map_or(&Value::Null, |member| &member["value"])
}

/// FR-342's joins over one schema-valid `state`/`operation_anchor` node:
/// `context`, then `frame`, then the two `semantic_type` agreements, then
/// (step 4, when a model is available) the operation name itself.
fn anchor_refusal(
    anchor: &Value,
    nodes: &[Value],
    model: Option<&ModelIndex<'_>>,
) -> Result<(), (StateRefusal, Value)> {
    let context = &anchor_binding(anchor, "context")["target"];
    join_state_reference(anchor, context, nodes, &ANCHOR_CONTEXTS)?;
    let frame = join_state_reference(
        anchor,
        &anchor_binding(anchor, "frame")["target"],
        nodes,
        &[("state", "frame")],
    )?;
    if anchor["semantic_type"] != *context || frame["semantic_type"] != *context {
        return Err((
            StateRefusal::MalformedDeclaration,
            anchor["node_id"].clone(),
        ));
    }
    if let Some(model) = model {
        let operation = anchor_binding(anchor, "operation")["value"]
            .as_str()
            .unwrap_or_default();
        let owner_node = mm_owner_node(
            model.declaration_nodes,
            context["digest"].as_str().unwrap_or_default(),
        );
        let resolves = owner_node.is_some_and(|node| {
            mm_declared_member(model.domain_package, node, "operations", operation).is_some()
        });
        if !resolves {
            return Err((StateRefusal::MissingName, anchor["node_id"].clone()));
        }
        // Check 5 (FR-340-AC-17): every `parameter` entry of the frame's
        // `deletes_scoped` names a reference parameter of the resolved
        // operation; the locus is the frame node.
        let reference_parameter = |name: &str| {
            owner_node
                .and_then(|node| {
                    mm_declared_member(model.domain_package, node, "operations", operation)
                        .filter(|_| !node.is_empty())
                })
                .and_then(|record| record["parameters"].as_array())
                .into_iter()
                .flatten()
                .any(|parameter| {
                    parameter["name"] == name
                        && ["object_types", "processes"]
                            .into_iter()
                            .filter_map(|list| model.domain_package[list].as_array())
                            .flatten()
                            .any(|declared| declared["node"] == parameter["typeRef"])
                })
        };
        for entry in frame["body"]["deletes_scoped"]
            .as_array()
            .into_iter()
            .flatten()
        {
            if entry["kind"] == "parameter"
                && !reference_parameter(entry["name"].as_str().unwrap_or_default())
            {
                return Err((StateRefusal::MissingName, frame["node_id"].clone()));
            }
        }
    }
    Ok(())
}

/// FR-341's joins over one schema-valid `state`/`state_clause` node: the
/// anchor, then each parameter reference in order, then the `self`
/// parameter's type and, for an invariant, the parameter count; then (when
/// a model is available) the full precondition/postcondition signature
/// count beyond `self` (FR-341 step 3).
fn clause_refusal(
    clause: &Value,
    nodes: &[Value],
    model: Option<&ModelIndex<'_>>,
) -> Result<(), (StateRefusal, Value)> {
    let body = &clause["body"];
    let kind = body["operation"]["member"]["clause"]
        .as_str()
        .unwrap_or_default();
    // FR-341: a state constraint is anchored and bound as an invariant.
    let context_anchored = matches!(kind, "invariant" | "constraint");
    let admitted_anchor: &[(&str, &str)] = if context_anchored {
        &ANCHOR_CONTEXTS
    } else {
        &[("state", "operation_anchor")]
    };
    let anchor = join_state_reference(
        clause,
        &body["arguments"][1]["target"],
        nodes,
        admitted_anchor,
    )?;
    let context = if context_anchored {
        anchor["node_id"].clone()
    } else {
        anchor_binding(anchor, "context")["target"].clone()
    };
    let parameters = body["arguments"][0]["members"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut joined = Vec::new();
    for parameter in parameters {
        joined.push(join_state_reference(
            clause,
            &parameter["target"],
            nodes,
            &[("value", "parameter")],
        )?);
    }
    let ineligible = (StateRefusal::OperatorIneligible, clause["node_id"].clone());
    let self_type = joined
        .first()
        .and_then(|parameter| node_by_digest(nodes, parameter["semantic_type"]["digest"].as_str()?))
        .ok_or_else(|| ineligible.clone())?;
    if self_type["node_tag"] != "composite_type"
        || self_type["semantic_form"] != "reference"
        || self_type["body"]["members"][0]["target"] != context
    {
        return Err(ineligible);
    }
    if context_anchored && joined.len() != 1 {
        return Err(ineligible);
    }
    if !context_anchored && let Some(model) = model {
        let operation_name = anchor_binding(anchor, "operation")["value"]
            .as_str()
            .unwrap_or_default();
        let owner_node = mm_owner_node(
            model.declaration_nodes,
            context["digest"].as_str().unwrap_or_default(),
        );
        if let Some(operation) = owner_node.and_then(|node| {
            mm_declared_member(model.domain_package, node, "operations", operation_name)
        }) {
            let has_result = kind == "postcondition" && !operation["result"].is_null();
            let parameter_count = operation["parameters"].as_array().map_or(0, Vec::len);
            let expected = 1 + usize::from(has_result) + parameter_count;
            if joined.len() != expected {
                return Err(ineligible);
            }
        }
    }
    Ok(())
}

/// Independent oracle for the state step of the reader order: a
/// `quire.op.state.clause` application at a body root stands only on a
/// `state`/`state_clause` node (a nested one is a malformed wire, refused
/// before this step); then `operation_anchor` nodes and then `state_clause` nodes,
/// each in ascending node-id digest order, report their first defect.
fn package_state_refusal(package: &Value) -> Result<(), (StateRefusal, Value)> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let model = model_index(package);
    let mut ordered = nodes.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    for node in &ordered {
        let root_clause = node["node_tag"] == "state" && node["semantic_form"] == "state_clause";
        let misplaced =
            node["body"]["operation"]["identity"] == "quire.op.state.clause" && !root_clause;
        if misplaced {
            return Err((StateRefusal::OperatorIneligible, node["node_id"].clone()));
        }
    }
    let anchors = ordered
        .iter()
        .filter(|node| node["node_tag"] == "state" && node["semantic_form"] == "operation_anchor")
        .copied()
        .collect::<Vec<_>>();
    for anchor in &anchors {
        anchor_refusal(anchor, nodes, model.as_ref())?;
    }
    anchor_uniqueness_refusal(&anchors)?;
    for node in ordered
        .iter()
        .filter(|node| node["node_tag"] == "state" && node["semantic_form"] == "state_clause")
    {
        clause_refusal(node, nodes, model.as_ref())?;
    }
    Ok(())
}

/// FR-342: one `operation_anchor` node stands for one (declaring object type,
/// operation name); no two anchors share a `context`/`operation` pair. Two
/// anchors of one context may reference one `frame` node. Checked only once
/// every anchor's own joins already hold (`anchor_refusal`), in ascending
/// anchor node-id digest order; the second anchor to claim a pair is the
/// offending node.
fn anchor_uniqueness_refusal(anchors: &[&Value]) -> Result<(), (StateRefusal, Value)> {
    let mut pairs = BTreeSet::new();
    for anchor in anchors {
        let context = anchor_binding(anchor, "context")["target"]["digest"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let operation = anchor_binding(anchor, "operation")["value"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if !pairs.insert((context, operation)) {
            return Err((
                StateRefusal::AmbiguousDeclaration,
                anchor["node_id"].clone(),
            ));
        }
    }
    Ok(())
}

/// A fixed 64-hex-character digest built by repeating one byte pattern, for
/// hand-authored frame vectors below. Not derived from any production code.
fn repeated_digest(byte: &str) -> String {
    byte.repeat(32)
}

/// Digest of the one node of `package` with this tag and form, narrowed by the
/// last segment of its declared name when several share a form.
fn published_digest(package: &Value, tag: &str, form: &str, name: Option<&str>) -> String {
    let hits = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .filter(|node| node["node_tag"] == tag && node["semantic_form"] == form)
        .filter(|node| {
            name.is_none_or(|name| {
                node["declaration"]["qualified_name"]
                    .as_array()
                    .and_then(|path| path.last())
                    .is_some_and(|last| last == name)
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(
        hits.len(),
        1,
        "{tag}/{form} {name:?} names exactly one node"
    );
    hits[0]["node_id"]["digest"]
        .as_str()
        .expect("node digest is a string")
        .to_owned()
}

/// Digest of a node the all-families fixture publishes, by its role there.
fn all_families_digest(role: &str) -> String {
    let package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let (tag, form, name) = match role {
        "relationship" => ("relation", "relationship", None),
        "account" => ("model", "object_type", Some("Account")),
        "process" => ("model", "process", None),
        "address" => ("model", "record_value_type", None),
        "event" => ("model", "event_type", None),
        "literal" => ("value", "literal", Some("enabled")),
        "frame" => ("state", "frame", None),
        other => panic!("unknown all-families role {other}"),
    };
    published_digest(&package, tag, form, name)
}

/// Digest of a node the clause-operations fixture publishes, by its role there.
fn clause_digest(role: &str) -> String {
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    if role == "boolean_literal" {
        return package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["node_tag"] == "value" && node["body"]["value_kind"] == "boolean")
            .expect("a boolean literal is published")["node_id"]["digest"]
            .as_str()
            .expect("digest")
            .to_owned();
    }
    if role == "parameter" {
        // The clause's `over` binder; the finish binder is the other parameter.
        return package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| {
                node["node_tag"] == "value"
                    && node["semantic_form"] == "parameter"
                    && node["body"]["members"][0]["value"]["value"] == "self"
            })
            .expect("the self binder is published")["node_id"]["digest"]
            .as_str()
            .expect("digest")
            .to_owned();
    }
    if role == "reference" {
        // The anchors' context type; the protocol clause's `over` type references the Order owner.
        let context = clause_digest("object_type");
        return package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| {
                node["node_tag"] == "composite_type"
                    && node["semantic_form"] == "reference"
                    && node["body"]["members"][0]["target"]["digest"] == context.as_str()
            })
            .expect("the context reference type is published")["node_id"]["digest"]
            .as_str()
            .expect("digest")
            .to_owned();
    }
    if role == "object_type" {
        // The anchors' context; the selected acme/orders Order node is the clause's owner.
        return published_digest(&package, "model", "object_type", Some("ConfigVersion"));
    }
    let (tag, form) = match role {
        "parameter" => ("value", "parameter"),
        "frame" => ("state", "frame"),
        "anchor" => ("state", "operation_anchor"),
        "reference" => ("composite_type", "reference"),
        "boolean" => ("scalar_type", "boolean"),
        other => panic!("unknown clause-operations role {other}"),
    };
    published_digest(&package, tag, form, None)
}

fn node_ref_digest(digest: &str) -> Value {
    json!({"domain": "quire.checked-semantic-node/v1", "digest": digest})
}

/// A `modifies` entry granting every link of the relationship `digest` names.
fn relationship_entry(digest: &str) -> Value {
    json!({"kind": "relationship", "declaration": node_ref_digest(digest)})
}

/// A `modifies` entry granting writes to field `name` of the declaring node
/// `digest` names.
fn field_entry(digest: &str, name: &str) -> Value {
    json!({
        "kind": "field",
        "declaration": node_ref_digest(digest),
        "name": name,
        "scope": "every-object",
    })
}

/// A hand-authored `state`/`frame` node: real `node_id` shape, the
/// all-families fixture's `Account` object type (`91...91`) as its
/// `semantic_type`, a `generated` occurrence, and an explicit `dependencies`
/// list and body, built directly from digest strings so every vector below
/// is independent of any shared comparator or generator function.
fn authored_frame_node(
    dependencies: &[&str],
    modifies: &[Value],
    creates: &[&str],
    deletes: &[&str],
) -> Value {
    json!({
        "node_id": node_ref_digest(&repeated_digest("93")),
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "state",
        "semantic_form": "frame",
        "semantic_type": node_ref_digest(&all_families_digest("account")),
        "dependencies": dependencies.iter().map(|d| node_ref_digest(d)).collect::<Vec<_>>(),
        "occurrences": [{"role": "generated", "ordinal": 0}],
        "body": {
            "term": "frame",
            "modifies": modifies,
            "creates": creates.iter().map(|d| node_ref_digest(d)).collect::<Vec<_>>(),
            "deletes": deletes.iter().map(|d| node_ref_digest(d)).collect::<Vec<_>>(),
            "deletes_scoped": [],
        },
    })
}

/// A meaning-join defect with the empty field name of a relationship,
/// `creates`/`deletes` or `semantic_type` position.
fn join_defect(group: FrameGroup, digest: &str, cause: FrameJoinCause) -> FrameDefect {
    FrameDefect::MeaningJoin {
        group,
        digest: digest.to_owned(),
        name: String::new(),
        cause,
    }
}

/// FR-340-AC-1: each of the three frame members may independently be empty;
/// an all-empty frame (nothing modified, created or deleted) and the
/// published fixture's fully-populated frame are both defect-free.
#[trace("TC-233", "FR-340-AC-1")]
#[test]
fn tc_233_frame_body_members_may_each_independently_be_empty() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let empty = authored_frame_node(&[], &[], &[], &[]);
    assert_eq!(frame_body_defects(&empty, &nodes, None), Vec::new());
    let published = nodes
        .iter()
        .find(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("the published fixture carries a frame node");
    assert_eq!(
        frame_body_defects(published, &nodes, None),
        Vec::new(),
        "the published all-families frame is defect-free"
    );
}

/// FR-340-AC-2/AC-3: a relationship entry must join a declared `relation`/
/// `relationship` node, a field entry a declared `model`/`object_type` or
/// `model`/`record_value_type` node, and a `creates`/`deletes` entry a
/// declared `model`/`object_type` or `model`/`process` node. An entry naming
/// no node of the package, or naming one that is not a declared dependency,
/// refuses `missing_declaration`/`missing-name`; an entry naming a declared
/// dependency of the wrong meaning refuses `invalid_model_binding`/
/// `malformed-declaration`. A literal has no frame meaning at all.
#[trace("TC-233", "FR-340-AC-2")]
#[trace("TC-233", "FR-340-AC-3")]
#[test]
fn tc_233_frame_entries_join_a_declared_node_of_the_required_meaning() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let relationship = all_families_digest("relationship");
    let object_type_node = all_families_digest("account");
    let process_node = all_families_digest("process");
    let literal = all_families_digest("literal");
    let absent = repeated_digest("ff");

    // A modifies entry naming no node of the package at all.
    let absent_entry = authored_frame_node(&[&absent], &[relationship_entry(&absent)], &[], &[]);
    assert_eq!(
        frame_body_defects(&absent_entry, &nodes, None),
        vec![join_defect(
            FrameGroup::Modifies,
            &absent,
            FrameJoinCause::MissingName
        )]
    );

    // A field entry whose declaring node is real but not a declared dependency.
    let undeclared =
        authored_frame_node(&[], &[field_entry(&object_type_node, "balance")], &[], &[]);
    assert_eq!(
        frame_body_defects(&undeclared, &nodes, None),
        vec![FrameDefect::MeaningJoin {
            group: FrameGroup::Modifies,
            digest: object_type_node.clone(),
            name: "balance".to_owned(),
            cause: FrameJoinCause::MissingName,
        }]
    );

    // A relationship entry naming a declared literal: no frame meaning.
    let wrong_meaning = authored_frame_node(&[&literal], &[relationship_entry(&literal)], &[], &[]);
    assert_eq!(
        frame_body_defects(&wrong_meaning, &nodes, None),
        vec![join_defect(
            FrameGroup::Modifies,
            &literal,
            FrameJoinCause::MalformedDeclaration
        )]
    );

    // A field entry on a process is admitted (FR-340-AC-19), a field entry on an
    // event type is not, and a relationship entry naming an object type: each
    // names a real declaration of the wrong meaning for its kind.
    let process_field = authored_frame_node(
        &[&process_node],
        &[field_entry(&process_node, "stage")],
        &[],
        &[],
    );
    assert_eq!(frame_body_defects(&process_field, &nodes, None), Vec::new());
    let event_node = all_families_digest("event");
    let event_field = authored_frame_node(
        &[&event_node],
        &[field_entry(&event_node, "stage")],
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&event_field, &nodes, None),
        vec![FrameDefect::MeaningJoin {
            group: FrameGroup::Modifies,
            digest: event_node.clone(),
            name: "stage".to_owned(),
            cause: FrameJoinCause::MalformedDeclaration,
        }]
    );
    let object_as_relationship = authored_frame_node(
        &[&object_type_node],
        &[relationship_entry(&object_type_node)],
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&object_as_relationship, &nodes, None),
        vec![join_defect(
            FrameGroup::Modifies,
            &object_type_node,
            FrameJoinCause::MalformedDeclaration
        )]
    );

    // A creates entry naming a relationship, not an object type or process.
    let wrong_group = authored_frame_node(&[&relationship], &[], &[&relationship], &[]);
    assert_eq!(
        frame_body_defects(&wrong_group, &nodes, None),
        vec![join_defect(
            FrameGroup::Creates,
            &relationship,
            FrameJoinCause::MalformedDeclaration
        )]
    );

    // A frame whose semantic_type is none of an object type, a process and a
    // state machine.
    let mut wrong_type = authored_frame_node(&[], &[], &[], &[]);
    wrong_type["semantic_type"] = node_ref_digest(&event_node);
    assert_eq!(
        frame_body_defects(&wrong_type, &nodes, None),
        vec![join_defect(
            FrameGroup::SemanticType,
            &event_node,
            FrameJoinCause::MalformedDeclaration
        )]
    );
    let mut process_type = authored_frame_node(&[], &[], &[], &[]);
    process_type["semantic_type"] = node_ref_digest(&process_node);
    assert_eq!(frame_body_defects(&process_type, &nodes, None), Vec::new());

    // A state machine declares fields but has no object universe, so it is a
    // field entry's declaring node and never a `creates` or `deletes` entry.
    let mut machine_nodes = nodes.clone();
    for node in &mut machine_nodes {
        if node["node_id"]["digest"] == process_node.as_str() {
            node["semantic_form"] = json!("state_machine");
        }
    }
    let machine_field = authored_frame_node(
        &[&process_node],
        &[field_entry(&process_node, "stage")],
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&machine_field, &machine_nodes, None),
        Vec::new()
    );
    let machine_creates = authored_frame_node(&[&process_node], &[], &[&process_node], &[]);
    assert_eq!(
        frame_body_defects(&machine_creates, &machine_nodes, None),
        vec![join_defect(
            FrameGroup::Creates,
            &process_node,
            FrameJoinCause::MalformedDeclaration
        )]
    );
    let machine_deletes = authored_frame_node(&[&process_node], &[], &[], &[&process_node]);
    assert_eq!(
        frame_body_defects(&machine_deletes, &machine_nodes, None),
        vec![join_defect(
            FrameGroup::Deletes,
            &process_node,
            FrameJoinCause::MalformedDeclaration
        )]
    );

    // A fully compatible frame is defect-free.
    let mut valid_entries = [
        (relationship.clone(), relationship_entry(&relationship)),
        (
            object_type_node.clone(),
            field_entry(&object_type_node, "balance"),
        ),
    ];
    valid_entries.sort_by(|a, b| a.0.cmp(&b.0));
    let valid = authored_frame_node(
        &[&relationship, &object_type_node, &process_node],
        &valid_entries.map(|(_, entry)| entry),
        &[&object_type_node],
        &[&process_node],
    );
    assert_eq!(frame_body_defects(&valid, &nodes, None), Vec::new());
}

/// FR-340-AC-10: a field of an object type whose own type names a record
/// value type, and a field declared on that record value type itself, are
/// distinct field entries -- one names the object type's node, the other
/// the record value type's node. `modifies` admits both in the same frame
/// body with no defect. `Account.address` (on the published fixture's
/// `Account` object type, typed `Address`) and `Address.street` (on its
/// `Address` `record_value_type` node) are one such pair.
#[trace("TC-233", "FR-340-AC-10")]
#[test]
fn tc_233_modifies_admits_a_record_typed_field_and_a_field_of_that_record() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let account = all_families_digest("account");
    let address = all_families_digest("address");
    let mut entries = [
        (account.clone(), field_entry(&account, "address")),
        (address.clone(), field_entry(&address, "street")),
    ];
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let both = authored_frame_node(
        &[&account, &address],
        &entries.map(|(_, entry)| entry),
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&both, &nodes, None),
        Vec::new(),
        "a modifies entry naming Account.address and one naming Address.street are distinct field entries and both are admitted"
    );
}

/// FR-340-AC-11 (#106 Q3, option A): a `modifies` entry naming a
/// relationship grants every link of that relationship, observed from each
/// end. A hypothetical owning-end-only reader would need two things this
/// wire encoding never supplies: a second `modifies` entry to reach the
/// relationship's far end, and a wire field on the relationship naming
/// which end declares it. Neither exists -- proving the single admitted
/// entry cannot mean anything narrower than the whole relationship.
#[trace("TC-233", "FR-340-AC-11")]
#[test]
fn tc_233_modifies_relationship_grant_is_not_scoped_to_an_owning_end() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let relationship = all_families_digest("relationship");

    // A single `modifies` entry naming the relationship is, by itself, a
    // complete and defect-free grant -- there is no separate "far end"
    // entry an owning-end-only reading would still be waiting on.
    let single_entry = authored_frame_node(
        &[&relationship],
        &[relationship_entry(&relationship)],
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&single_entry, &nodes, None),
        Vec::new(),
        "one modifies entry naming a relationship is a complete grant with nothing left to reach the far end"
    );

    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");

    // An owning-end-only reading implies a second, end-scoped entry would
    // be meaningful. This vocabulary has none: the only way to author
    // "reach the relationship again" is to repeat its one node key, and
    // the schema's own uniqueItems constraint on `modifies` refuses that
    // outright. There is no authorable second entry to reach the far end.
    let mut duplicated = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let frame_index = duplicated["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("a frame node is published");
    duplicated["semantic_graph"]["nodes"][frame_index]["body"]["modifies"] = json!([
        relationship_entry(&relationship),
        relationship_entry(&relationship)
    ]);
    assert!(
        !schema.is_valid(&duplicated),
        "repeating a relationship's node key to author a second, end-scoped modifies entry is refused by uniqueItems -- no second entry can ever reach a 'far end'"
    );

    // Nor does the relationship's own node carry anything a reader could
    // resolve an owning end from: interface_004's Declaration section
    // names the node tags that carry `declaration: {qualified_name}`, and
    // `relation` is not one of them. Marking one with a `declaration`, as
    // an owning-end-only reading would need to record which end declares
    // it, is not an admissible package at all.
    let mut owner_marked = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let relationship_index = owner_marked["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "relation" && node["semantic_form"] == "relationship")
        .expect("a relationship node is published");
    owner_marked["semantic_graph"]["nodes"][relationship_index]
        .as_object_mut()
        .expect("relationship node is an object")
        .insert(
            "declaration".to_string(),
            json!({"qualified_name": ["Example", "Account", "transactions"]}),
        );
    assert!(
        !schema.is_valid(&owner_marked),
        "a relation/relationship node cannot carry a declaration naming an owning end; the wire encoding has nothing to scope a modifies grant by"
    );
}

/// FR-340-AC-4: each member array is sorted ascending by entry digest and
/// duplicate-free. Duplicates are refused by the schema's `uniqueItems`
/// before this oracle runs; a misordered array is refused here, following
/// FR-322-AC-10's `invalid_semantic_graph` pattern for noncanonical order.
#[trace("TC-233", "FR-340-AC-4")]
#[test]
fn tc_233_frame_member_arrays_are_sorted_ascending_by_entry_digest() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let object_type_node = all_families_digest("account");
    let process_node = all_families_digest("process");
    let (low, high) = if object_type_node < process_node {
        (&object_type_node, &process_node)
    } else {
        (&process_node, &object_type_node)
    };
    // Descending order: both entries are individually valid `deletes`
    // targets, so the only defect is the array's own order.
    let misordered =
        authored_frame_node(&[&object_type_node, &process_node], &[], &[], &[high, low]);
    assert_eq!(
        frame_body_defects(&misordered, &nodes, None),
        vec![FrameDefect::CanonicalOrder {
            group: FrameGroup::Deletes,
        }]
    );
    let ordered = authored_frame_node(&[&object_type_node, &process_node], &[], &[], &[low, high]);
    assert_eq!(frame_body_defects(&ordered, &nodes, None), Vec::new());

    // Two field entries of one declaring node order by field name, as UTF-8
    // bytes, after the declaring node digest.
    let misnamed = authored_frame_node(
        &[&object_type_node],
        &[
            field_entry(&object_type_node, "total"),
            field_entry(&object_type_node, "balance"),
        ],
        &[],
        &[],
    );
    assert_eq!(
        frame_body_defects(&misnamed, &nodes, None),
        vec![FrameDefect::CanonicalOrder {
            group: FrameGroup::Modifies,
        }]
    );
    let named = authored_frame_node(
        &[&object_type_node],
        &[
            field_entry(&object_type_node, "balance"),
            field_entry(&object_type_node, "total"),
        ],
        &[],
        &[],
    );
    assert_eq!(frame_body_defects(&named, &nodes, None), Vec::new());

    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let mut duplicated = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let frame_index = duplicated["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("a frame node is published");
    let first_modifies =
        duplicated["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][0].clone();
    duplicated["semantic_graph"]["nodes"][frame_index]["body"]["modifies"]
        .as_array_mut()
        .expect("modifies is an array")
        .push(first_modifies);
    assert!(
        !schema.is_valid(&duplicated),
        "a duplicated modifies entry fails the schema's uniqueItems before any oracle runs"
    );
}

/// FR-340-AC-5: the frame body's shape validates only as a `state`/`frame`
/// node's body. A `value`/`literal` node's body carrying the frame shape,
/// and a `state`/`frame` node's body carrying an ordinary `SemanticTerm`
/// shape, both fail the public schema.
#[trace("TC-233", "FR-340-AC-5")]
#[test]
fn tc_233_frame_body_shape_is_scoped_to_state_frame_nodes_only() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");

    let mut leaked = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let literal_index = leaked["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "value" && node["semantic_form"] == "literal")
        .expect("a value literal node is published");
    leaked["semantic_graph"]["nodes"][literal_index]["body"] =
        json!({"term": "frame", "modifies": [], "creates": [], "deletes": []});
    assert!(
        !schema.is_valid(&leaked),
        "the frame body shape must not validate on a value node"
    );

    let mut wrong_shape = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let frame_index = wrong_shape["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("a frame node is published");
    wrong_shape["semantic_graph"]["nodes"][frame_index]["body"] =
        json!({"term": "aggregate", "members": []});
    assert!(
        !schema.is_valid(&wrong_shape),
        "a frame node's body must not accept an ordinary SemanticTerm shape"
    );
}

/// FR-340-AC-6: when a frame body has more than one simultaneous defect, the
/// reported diagnostic is fully determined by defect kind (a meaning-join
/// defect always outranks a canonical-order defect) and then by member
/// group (`modifies` < `creates` < `deletes`), never by which group happens
/// to carry which defect. Vector A puts the canonical-order defect in
/// `modifies` and the meaning-join defect in `deletes`; vector B swaps
/// which group carries which defect (the meaning-join defect now in
/// `modifies`, the canonical-order defect now in `deletes`). A reader that
/// incorrectly reported "whichever defect its scan reaches first" would
/// pass vector B (its scan reaches the meaning-join defect first anyway)
/// while failing to notice it is wrong on vector A; this test requires the
/// correct, kind-first precedence on both.
#[trace("TC-233", "FR-340-AC-6")]
#[test]
fn tc_233_a_frame_meaning_join_defect_outranks_a_canonical_order_defect() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let relationship = all_families_digest("relationship");
    let object_type_node = all_families_digest("account");
    let process_node = all_families_digest("process");

    // Vector A: modifies is misordered (canonical-order defect); deletes
    // names a relationship, wrong meaning for deletes (meaning-join defect).
    let mut descending = [
        (
            object_type_node.clone(),
            field_entry(&object_type_node, "balance"),
        ),
        (relationship.clone(), relationship_entry(&relationship)),
    ];
    descending.sort_by(|a, b| b.0.cmp(&a.0));
    let vector_a = authored_frame_node(
        &[&relationship, &object_type_node],
        &descending.map(|(_, entry)| entry),
        &[&object_type_node],
        &[&relationship],
    );
    assert_eq!(
        reported_frame_defect(&vector_a, &nodes, None),
        Some(join_defect(
            FrameGroup::Deletes,
            &relationship,
            FrameJoinCause::MalformedDeclaration
        )),
        "vector A: the meaning-join defect in deletes outranks the canonical-order defect in modifies"
    );

    // Vector B: swap which group carries which defect. modifies now names an
    // object_type, wrong meaning for modifies (meaning-join defect); deletes
    // is now the misordered array (canonical-order defect).
    let vector_b = authored_frame_node(
        &[&object_type_node, &process_node],
        &[relationship_entry(&object_type_node)],
        &[],
        &if process_node > object_type_node {
            [&process_node, &object_type_node]
        } else {
            [&object_type_node, &process_node]
        },
    );
    assert_eq!(
        reported_frame_defect(&vector_b, &nodes, None),
        Some(join_defect(
            FrameGroup::Modifies,
            &object_type_node,
            FrameJoinCause::MalformedDeclaration
        )),
        "vector B: the meaning-join defect in modifies still outranks the canonical-order defect in deletes"
    );
}

/// FR-340-AC-7: the frame body's stated Outputs (checked `modifies`,
/// `creates`, `deletes` and `deletes_scoped` membership, no other member) are
/// exactly what the schema admits; a body missing a stated member, or a field
/// entry with no `scope`, fails the schema, and `package_frame_refusal`
/// reports every published fixture's frame as defect-free through the same
/// reader path `read_package_semantics` uses.
#[trace("TC-233", "FR-340-AC-7")]
#[test]
fn tc_233_frame_body_outputs_are_exactly_the_schemas_required_members() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    assert!(
        schema.is_valid(&parsed_fixture(POSITIVE_ALL_FAMILIES)),
        "the published all-families fixture carries all four frame members and a scope on every field entry"
    );
    for missing in ["modifies", "creates", "deletes", "deletes_scoped"] {
        let mut candidate = parsed_fixture(POSITIVE_ALL_FAMILIES);
        let frame_index = candidate["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
            .expect("a frame node is published");
        candidate["semantic_graph"]["nodes"][frame_index]["body"]
            .as_object_mut()
            .expect("frame body is an object")
            .remove(missing);
        assert!(
            !schema.is_valid(&candidate),
            "a frame body missing `{missing}` must fail the schema"
        );
    }
    let mut unscoped = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let frame_index = unscoped["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("a frame node is published");
    unscoped["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][1]
        .as_object_mut()
        .expect("the published field entry is an object")
        .remove("scope");
    assert!(
        !schema.is_valid(&unscoped),
        "a field entry with no scope must fail the schema"
    );
    for (label, package) in published_v2_packages() {
        assert_eq!(
            package_frame_refusal(&package),
            Ok(()),
            "{label}: every published frame is admitted by the same oracle read_package_semantics uses"
        );
    }
}

/// FR-340-AC-2/AC-3/AC-4/AC-6/AC-8, through the full normative reader: every
/// published `frame_mutations` vector is spliced into the all-families
/// fixture's published frame node (its `dependencies` and
/// `modifies`/`creates`/`deletes` replaced wholesale) and read with
/// `read_package_semantics`, the same entry point a real reader uses. This
/// is what closes the untested refusal path a mutation of
/// `package_frame_refusal` (making it always return `Ok(())`) would
/// otherwise pass unnoticed: every non-canonical-order vector here asserts
/// `ReadRefusal::Frame` is the variant actually returned, its `code()` and
/// (where the vector states one) its `cause()`, and the exact locus —not
/// merely that the standalone oracle functions would refuse it in
/// isolation. The `deletes-misordered` vector's expectation is asserted
/// only by the `invalid_semantic_graph` code string, never by matching the
/// internal `FrameDefect::CanonicalOrder`/`FrameRefusal::CanonicalOrder`
/// variants, and its `expected_cause` is `null` because `CanonicalOrder`
/// carries no cause tag. The two `precedence-*` vectors are what makes a
/// swap of `FrameGroup`'s declared variant order, or of `FrameDefect::
/// MeaningJoin`'s declared field order, fail a test: no prior vector had
/// two meaning-join defects at once. The fifteen `systems-*-in-*` vectors
/// (FR-340-AC-8) splice each of the five published systems-model nodes into
/// each of `modifies` (as a field entry's declaring node), `creates` and
/// `deletes` in turn: they are what makes adding `| "systems_part"` to
/// `frame_entry_admits`'s field arm, or admitting every `systems_*` form as
/// an instance entry, fail a test. The `cross-frame-lower-digest-
/// wins` vector (FR-340-AC-9) is the only one with a `second_frame` field:
/// it mutates the published frame node (digest `93...93`) into a
/// missing-name defect and the loop below inserts a second, brand-new
/// frame node at a lower digest (`01...01`) with a canonical-order defect,
/// so `package_frame_refusal`'s ascending node-id-digest scan reaches the
/// lower-digest node first. No prior vector published two frame nodes at
/// once, so this is what makes a mutant that ranks meaning-join defects
/// first across the whole package, instead of visiting frame nodes in
/// ascending digest order, fail a test: such a mutant would report the
/// `93...93` node's `missing_declaration` instead of the `01...01` node's
/// `invalid_semantic_graph`.
#[trace("TC-233", "FR-340-AC-2")]
#[trace("TC-233", "FR-340-AC-3")]
#[trace("TC-233", "FR-340-AC-4")]
#[trace("TC-233", "FR-340-AC-6")]
#[trace("TC-233", "FR-340-AC-8")]
#[trace("TC-233", "FR-340-AC-9")]
#[trace("TC-233", "FR-340-AC-16")]
#[trace("TC-233", "FR-340-AC-17")]
#[test]
fn tc_233_frame_mutations_refuse_through_the_full_reader_with_their_recorded_cause() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let mutations = vectors["frame_mutations"]
        .as_array()
        .expect("frame_mutations is an array");
    assert_eq!(
        mutations.len(),
        37,
        "frame_mutations covers all thirty-seven authored vectors"
    );

    let causes = mutations
        .iter()
        .filter_map(|mutation| mutation["expected_cause"].as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        causes,
        FrameRefusal::ALL
            .into_iter()
            .map(FrameRefusal::cause)
            .collect::<BTreeSet<_>>(),
        "the mutations exercise every frame refusal cause FrameRefusal::ALL pairs with a code"
    );
    assert!(
        mutations
            .iter()
            .any(|mutation| mutation["expected_cause"].is_null()
                && mutation["expected_code"] == "invalid_semantic_graph"),
        "at least one vector exercises the cause-less canonical-order refusal"
    );

    let package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let frame_index = nodes
        .iter()
        .position(|node| node["node_tag"] == "state" && node["semantic_form"] == "frame")
        .expect("a frame node is published");

    let digest_ref =
        |digest: &str| json!({"domain": "quire.checked-semantic-node/v1", "digest": digest});
    let digests = |mutation: &Value, key: &str| {
        mutation[key]
            .as_array()
            .unwrap_or_else(|| panic!("{key} is an array"))
            .iter()
            .map(|entry| {
                digest_ref(
                    entry
                        .as_str()
                        .unwrap_or_else(|| panic!("{key} entry is a string")),
                )
            })
            .collect::<Vec<_>>()
    };

    for mutation in mutations {
        let name = mutation["name"]
            .as_str()
            .expect("mutation name is a string");
        let mut candidate = package.clone();
        let frame = &mut candidate["semantic_graph"]["nodes"][frame_index];
        frame["dependencies"] = json!(digests(mutation, "dependencies"));
        // A `modifies` entry is authored in its wire shape with a bare
        // digest `declaration`, substituted here as a `NodeRef`.
        frame["body"]["modifies"] = mutation["modifies"]
            .as_array()
            .expect("modifies is an array")
            .iter()
            .map(|entry| {
                let mut entry = entry.clone();
                entry["declaration"] = digest_ref(
                    entry["declaration"]
                        .as_str()
                        .expect("modifies declaration is a digest string"),
                );
                entry
            })
            .collect();
        frame["body"]["creates"] = json!(digests(mutation, "creates"));
        frame["body"]["deletes"] = json!(digests(mutation, "deletes"));
        if let Some(scoped) = mutation.get("deletes_scoped") {
            frame["body"]["deletes_scoped"] = scoped.clone();
        }

        // A vector's `second_frame`, when present, is a complete
        // `state`/`frame` node object -- every member a real
        // semantic-graph node requires -- and is inserted into the graph
        // exactly as authored, so the vendored vector file alone (not this
        // test's Rust) fixes what a second frame node looks like.
        if let Some(second_frame) = mutation.get("second_frame") {
            candidate["semantic_graph"]["nodes"]
                .as_array_mut()
                .expect("nodes is an array")
                .push(second_frame.clone());
        }

        let expected_code = mutation["expected_code"]
            .as_str()
            .expect("expected_code is a string");
        let expected_locus = digest_ref(
            mutation["expected_locus_digest"]
                .as_str()
                .expect("expected_locus_digest is a string"),
        );
        // The edited frame is rekeyed so the read reaches the frame step.
        let renames = rekey_stale_nodes(&mut candidate);
        let expected_locus = renamed_locus(&renames, &expected_locus);
        match read_package_semantics(&candidate, &catalog) {
            Err(ReadRefusal::Frame(refusal, locus)) => {
                assert_eq!(refusal.code(), expected_code, "{name}: code");
                assert_eq!(locus, expected_locus, "{name}: locus");
                if let Some(expected_cause) = mutation["expected_cause"].as_str() {
                    assert_eq!(refusal.cause(), expected_cause, "{name}: cause");
                }
            }
            other => panic!("{name}: expected a frame refusal, got {other:?}"),
        }
    }
}

/// FR-340-AC-6, tightened from "the same kind of diagnostic" to "the same
/// diagnostic": authoring the identical pair of meaning-join defects in two
/// different collection orders sorts to the identical minimum element, not
/// merely an element of the same variant. This is a direct
/// comparator-invariance check on the `FrameDefect` collection itself,
/// distinct from the group-swap check above: two real defects inside one
/// authored frame body are already canonically ordered by digest within
/// their own member array (AC-4), so a re-authored member array can never
/// exercise collection-order independence — only a direct `Vec<FrameDefect>`
/// built in two different orders can.
#[trace("TC-233", "FR-340-AC-6")]
#[test]
fn tc_233_frame_defect_ordering_does_not_depend_on_collection_order() {
    let mut digests = [
        all_families_digest("account"),
        all_families_digest("process"),
    ];
    digests.sort();
    let [lower, higher] = digests;
    let first = join_defect(
        FrameGroup::Modifies,
        &lower,
        FrameJoinCause::MalformedDeclaration,
    );
    let second = join_defect(
        FrameGroup::Modifies,
        &higher,
        FrameJoinCause::MalformedDeclaration,
    );
    let expected = vec![first.clone(), second.clone()];

    // Two field entries of one declaring node tie on the digest and break
    // by field name.
    let named = |name: &str| FrameDefect::MeaningJoin {
        group: FrameGroup::Modifies,
        digest: higher.clone(),
        name: name.to_owned(),
        cause: FrameJoinCause::MissingName,
    };
    let mut by_name = vec![named("total"), named("balance")];
    by_name.sort();
    assert_eq!(by_name, vec![named("balance"), named("total")]);

    // Each vector starts in a different collection order; sorting either one
    // must reach the identical canonical (lower-digest-first) order, not
    // merely reach whatever order the other one reaches -- comparing the two
    // sorted vectors to each other, rather than to this fixed `expected`,
    // would hold trivially for any total order and could never catch a
    // reversed or dropped sort.
    let mut forward = vec![first.clone(), second.clone()];
    forward.sort();
    assert_eq!(
        forward, expected,
        "collecting the lower digest first still sorts ascending by digest"
    );
    let mut backward = vec![second, first];
    backward.sort();
    assert_eq!(
        backward, expected,
        "collecting the higher digest first still sorts to the same ascending-by-digest order"
    );
}

/// Rekey one application-bearing node in the graph and the source map to its
/// preimage digest, then refresh the package identity.
fn rekey_application_node(package: &mut Value, node_index: usize) {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let node = &nodes[node_index];
    let old_id = node["node_id"].clone();
    let new_digest = unqualified_jcs_sha256(
        &application_node_preimage(node, nodes).expect("rekeyed node bears an application"),
    );
    replace_node_field_in_graph(
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
/// Rekey every ungrouped application or structural node whose retained key is
/// stale, and every node naming it, until every such key is current; rewrite
/// each renamed digest wherever the package names it, then rebuild the
/// identity preimage and `package_id`. Returns each original digest's final
/// digest, so a test that edits a node to reach a later reader stage names
/// the edited node by its current key.
fn rekey_stale_nodes(package: &mut Value) -> BTreeMap<String, String> {
    let mut renames = BTreeMap::<String, String>::new();
    let limit = package["semantic_graph"]["nodes"]
        .as_array()
        .map_or(0, Vec::len)
        .pow(2);
    for _ in 0..=limit {
        let nodes = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("package nodes are an array");
        let stale = nodes.iter().find_map(|node| {
            if node.get("recursion_group").is_some() {
                return None;
            }
            let derived = if !is_application_node(node) {
                is_rekeyed_structural_node(node)
                    .then(|| structural_node_preimage(node))
                    .flatten()
            } else {
                application_node_preimage(node, nodes)
            }
            .map(|preimage| unqualified_jcs_sha256(&preimage))?;
            let retained = node["node_id"]["digest"].as_str()?;
            (retained != derived).then(|| (retained.to_owned(), derived))
        });
        let Some((old, new)) = stale else {
            package["identity_preimage"] = derive_identity_preimage(package);
            refresh_package_id(package);
            return renames;
        };
        let text = serde_json::to_string(package)
            .expect("package serializes")
            .replace(&old, &new);
        *package = serde_json::from_str(&text).expect("rekeyed package parses");
        // A renamed dependency keeps each `dependencies` list digest-ascending.
        for node in package["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("package nodes are an array")
        {
            if let Some(dependencies) = node["dependencies"].as_array_mut()
                && dependencies
                    .iter()
                    .any(|dependency| dependency["digest"] == new.as_str())
            {
                dependencies
                    .sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
            }
        }
        for value in renames.values_mut().filter(|value| **value == old) {
            value.clone_from(&new);
        }
        renames.entry(old).or_insert(new);
    }
    panic!("rekeying does not settle within {limit} renames");
}

/// Append a structural node to a fixture, bind its current key and recompute
/// the package id. The caller supplies the complete wire body and cross-fields;
/// this helper is used only by reader qualification vectors.
fn append_rekeyed_structural_node(package: &mut Value, mut node: Value) {
    let self_typed = node["semantic_type"] == node["node_id"]
        || matches!(
            node["semantic_form"].as_str(),
            Some("compound_unit" | "unit")
        );
    if self_typed {
        node["semantic_type"] = node["node_id"].clone();
    }
    let preimage = structural_node_preimage(&node).expect("fixture node is structural");
    let digest = unqualified_jcs_sha256(&preimage);
    node["node_id"]["digest"] = json!(digest);
    if self_typed {
        node["semantic_type"] = node["node_id"].clone();
    }
    package["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("fixture nodes are an array")
        .push(node);
    package["identity_preimage"] = derive_identity_preimage(package);
    refresh_package_id(package);
}

/// `locus` under `renames`: the node id its digest names after rekeying.
fn renamed_locus(renames: &BTreeMap<String, String>, locus: &Value) -> Value {
    let mut renamed = locus.clone();
    if let Some(new) = locus["digest"]
        .as_str()
        .and_then(|digest| renames.get(digest))
    {
        renamed["digest"] = json!(new);
    }
    renamed
}

/// Unit node index in `positive-nominal-identities.json`; no other node depends on it.
const NOMINAL_UNIT_NODE: usize = 2;

/// Replace the unit node owner and consistently rekey the node, its source-map
/// entry and the package identity.
fn rekey_nominal_unit_owner(package: &mut Value, owner: Value) {
    replace_nominal_preimage_in_graph(package, NOMINAL_UNIT_NODE, "/owner", owner);
    let old_id = package["semantic_graph"]["nodes"][NOMINAL_UNIT_NODE]["node_id"].clone();
    let new_digest = unqualified_jcs_sha256(
        &package["semantic_graph"]["nodes"][NOMINAL_UNIT_NODE]["nominal_identity_preimage"],
    );
    replace_node_field_in_graph(
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
    assert_eq!(
        vector["identity_preimage"].get("identity_projection"),
        None,
        "the graph is shipped once: the identity preimage carries no projection"
    );
    let nodes = vector["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array");
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

/// FR-340-AC-8: `ModelNode.semantic_form` names all five of FR-208's
/// systems meanings (FR-152), not only the ten business-model meanings
/// FR-340 already covers, so a systems-model declaration is never
/// unrepresentable on the wire. This vector exercises `systems_part`; the
/// published schema and the package-identity tests already admit and
/// rehash it as an ordinary declared `model` node.
#[trace("TC-233", "FR-340-AC-8")]
#[test]
fn tc_233_schema_names_every_fr208_systems_meaning_as_a_model_semantic_form() {
    let schema = parsed_fixture(SCHEMA);
    let systems_forms =
        schema["$defs"]["ModelNode"]["allOf"][1]["properties"]["semantic_form"]["enum"]
            .as_array()
            .expect("ModelNode semantic_form is an enum")
            .iter()
            .filter_map(Value::as_str)
            .filter(|form| form.starts_with("systems_"))
            .collect::<BTreeSet<_>>();
    assert_eq!(
        systems_forms,
        BTreeSet::from([
            "systems_interface",
            "systems_part",
            "systems_port",
            "systems_connection",
            "systems_allocation",
        ]),
        "the wire names exactly FR-152's five systems kinds (interface, part, port, \
         connection, allocation)"
    );

    let vector = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let nodes = vector["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array");
    assert!(
        nodes
            .iter()
            .any(|node| node["node_tag"] == "model" && node["semantic_form"] == "systems_part"),
        "the all-family vector declares a systems_part model node"
    );
}

#[trace("TC-233", "FR-322-AC-14")]
#[test]
fn tc_233_package_identity_is_the_jcs_digest_of_its_non_circular_preimage() {
    let vector = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let qualified_digest =
        jcs_sha256(&package_id_preimage(&vector)).expect("package-id preimage is canonicalizable");
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

/// Rebuild the shipped V2 identity preimage from a package's lock alone: the
/// version and the selection members, with no node.
fn derive_identity_preimage(package: &Value) -> Value {
    let mut preimage = serde_json::Map::new();
    preimage.insert("version".to_owned(), json!("quire.checked-package-id/v2"));
    for member in IDENTITY_SELECTION_MEMBERS {
        preimage.insert(member.to_owned(), package["lock"][member].clone());
    }
    Value::Object(preimage)
}

/// The FR-322 identity projection a reader derives from the semantic graph:
/// every node in graph order with its source occurrences removed.
fn occurrence_free_projection(package: &Value) -> Value {
    Value::Array(
        package["semantic_graph"]["nodes"]
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
            .collect(),
    )
}

/// The package-id preimage of a package as shipped: its `identity_preimage`
/// with the identity projection derived from its semantic graph.
fn package_id_preimage(package: &Value) -> Value {
    let mut preimage = package["identity_preimage"].clone();
    preimage
        .as_object_mut()
        .expect("identity preimage is an object")
        .insert(
            "identity_projection".to_owned(),
            occurrence_free_projection(package),
        );
    preimage
}

/// The package-id preimage rebuilt from a package's lock and semantic graph
/// alone, so that the package id is recomputed from its named members rather
/// than read back.
fn derive_package_id_preimage(package: &Value) -> Value {
    let mut preimage = derive_identity_preimage(package);
    preimage
        .as_object_mut()
        .expect("identity preimage is an object")
        .insert(
            "identity_projection".to_owned(),
            occurrence_free_projection(package),
        );
    preimage
}

#[trace("TC-233", "FR-322-AC-14")]
#[test]
fn tc_233_package_identity_ignores_excluded_members_and_binds_every_named_member() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    for (fixture, package) in positive_fixtures() {
        assert_eq!(
            derive_identity_preimage(&package),
            package["identity_preimage"],
            "{fixture} identity preimage is exactly its lock selections and occurrence-free graph"
        );
        let package_id = package["package_id"]["digest"]
            .as_str()
            .expect("package digest is a string");
        assert_eq!(
            unqualified_jcs_sha256(&derive_package_id_preimage(&package)),
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
                "/identity_preimage",
                "identity_projection",
                package["semantic_graph"]["nodes"].clone(),
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
                "/diagnostics/catalog/identity",
                json!("another-catalog"),
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
                unqualified_jcs_sha256(&derive_package_id_preimage(&candidate)),
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
                "edition identity",
                "/lock/edition/definition/identity",
                json!("another-edition"),
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
                    "identity": "test/orders",
                    "digest_domain": "sha256-jcs",
                    "digest": "8888888888888888888888888888888888888888888888888888888888888888"
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
                json!([{
                    "identity": "test/geometry",
                    "package_id": {
                        "domain": "quire.package.semantic/v2", "algorithm": "sha256",
                        "digest": "8888888888888888888888888888888888888888888888888888888888888888"
                    }
                }]),
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
                unqualified_jcs_sha256(&derive_package_id_preimage(&candidate)),
                package_id,
                "{fixture} editing the named preimage member {label} changes the package id"
            );
        }
    }
}

#[trace("TC-233", "FR-322-AC-13")]
#[test]
fn tc_233_public_schema_refuses_authored_version_domain_and_node_mutations() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
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

/// The published `positive-all-families` package with `entries` as its
/// `dependency_selections`, in both the lock and the identity preimage.
fn with_dependency_selections(entries: &Value) -> Value {
    let mut package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    package["lock"]["dependency_selections"] = entries.clone();
    package["identity_preimage"]["dependency_selections"] = entries.clone();
    package
}

#[test]
#[trace("TC-233", "FR-322-AC-76")]
fn tc_233_closure_vectors_bind_real_dependency_identities() {
    let vectors = parsed_fixture(DEPENDENCY_SELECTION_VECTORS);
    let closure_vectors = vectors["closure_vectors"]
        .as_array()
        .expect("closure vectors are an array");
    assert_eq!(closure_vectors.len(), 2);

    for vector in closure_vectors {
        let direct = vector["direct_dependencies"]
            .as_array()
            .expect("direct dependencies are an array");
        let locks = vector["dependency_locks"]
            .as_array()
            .expect("dependency locks are an array");
        assert_eq!(direct.len(), locks.len());
        for (entry, lock) in direct.iter().zip(locks) {
            assert_eq!(entry["identity"], lock["identity"]);
            let base = match lock["base"].as_str().expect("base path is a string") {
                "fixtures/positive-all-families.json" => parsed_fixture(POSITIVE_ALL_FAMILIES),
                "fixtures/positive-clause-operations.json" => {
                    parsed_fixture(POSITIVE_CLAUSE_OPERATIONS)
                }
                other => panic!("unknown closure vector base: {other}"),
            };
            let selections = lock["dependency_selections"]
                .as_array()
                .expect("dependency selections are an array");
            for selected in selections {
                assert_eq!(
                    selected["package_id"]["digest"],
                    unqualified_jcs_sha256(&package_id_preimage(&base)),
                    "the selected leaf id is the real base package id"
                );
            }
            let mut preimage = package_id_preimage(&base);
            preimage["dependency_selections"] = lock["dependency_selections"].clone();
            assert_eq!(
                entry["package_id"]["digest"],
                unqualified_jcs_sha256(&preimage),
                "a direct dependency id binds its own direct lock entries"
            );
        }
    }

    let diamond = &closure_vectors[0];
    assert_eq!(diamond["id"], "diamond-conflict-across-dependency-locks");
    assert_eq!(
        diamond["dependency_locks"][0]["dependency_selections"][0]["identity"],
        diamond["dependency_locks"][1]["dependency_selections"][0]["identity"]
    );
    assert_ne!(
        diamond["dependency_locks"][0]["dependency_selections"][0]["package_id"],
        diamond["dependency_locks"][1]["dependency_selections"][0]["package_id"]
    );
    assert_eq!(
        diamond["loci"],
        json!([
            "/lock/dependency_selections/0/lock/dependency_selections/0",
            "/lock/dependency_selections/1/lock/dependency_selections/0"
        ])
    );

    let reference = &closure_vectors[1];
    assert_eq!(reference["id"], "transitive-only-dependency-reference");
    let referenced_id = &reference["dependency_reference"]["package"];
    assert!(
        reference["dependency_locks"]
            .as_array()
            .expect("dependency locks are an array")
            .iter()
            .flat_map(|lock| lock["dependency_selections"]
                .as_array()
                .expect("selections are an array"))
            .any(|selected| &selected["package_id"] == referenced_id)
    );
    assert!(
        reference["direct_dependencies"]
            .as_array()
            .expect("direct dependencies are an array")
            .iter()
            .all(|direct| &direct["package_id"] != referenced_id)
    );
}

#[test]
#[trace("TC-233", "FR-322-AC-35")]
fn tc_233_dependency_selections_carry_each_dependency_package_identity() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(DEPENDENCY_SELECTION_VECTORS);
    assert_eq!(
        vectors["base"], "fixtures/positive-all-families.json",
        "the dependency vectors extend the all-families package"
    );
    let entries = &vectors["dependency_selections"];
    let identities: Vec<&str> = entries
        .as_array()
        .expect("dependency selections are an array")
        .iter()
        .map(|entry| entry["identity"].as_str().expect("identity is a string"))
        .collect();
    assert!(
        identities.len() > 1 && identities.windows(2).all(|pair| pair[0] < pair[1]),
        "the published selections are one per library identity, ascending"
    );

    // The published selections are placeholders no admitted package has.
    let admitted_ids: Vec<Value> = positive_fixtures()
        .iter()
        .map(|(_, package)| package["package_id"]["digest"].clone())
        .collect();
    for entry in entries.as_array().expect("dependency selections are an array") {
        assert!(
            !admitted_ids.contains(&entry["package_id"]["digest"]),
            "{} names an admitted fixture, not a placeholder",
            entry["identity"]
        );
    }

    let mut package = with_dependency_selections(entries);
    package["package_id"]["digest"] = vectors["package_id"].clone();
    let errors: Vec<String> = schema
        .iter_errors(&package)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect();
    assert!(
        errors.is_empty(),
        "a package with dependency selections is admitted: {errors:?}"
    );
    assert_eq!(
        derive_identity_preimage(&package),
        package["identity_preimage"],
        "dependency selections enter the identity preimage exactly as locked"
    );
    let package_id = unqualified_jcs_sha256(&derive_package_id_preimage(&package));
    assert_eq!(
        package_id,
        vectors["package_id"]
            .as_str()
            .expect("package id is a string"),
        "the recorded package id binds the dependency selections"
    );
    assert_ne!(
        package_id,
        parsed_fixture(POSITIVE_ALL_FAMILIES)["package_id"]["digest"],
        "adding dependency selections changes the package id"
    );
    let mut repinned = package.clone();
    for member in ["lock", "identity_preimage"] {
        repinned[member]["dependency_selections"][0]["package_id"]["digest"] =
            json!("5555555555555555555555555555555555555555555555555555555555555555");
    }
    assert_ne!(
        unqualified_jcs_sha256(&derive_package_id_preimage(&repinned)),
        package_id,
        "a dependency's package id enters the importing package id"
    );

    let mutations = &vectors["entry_mutations"];
    for mutation in mutations.as_array().expect("entry mutations are an array") {
        let id = mutation["id"].as_str().expect("mutation id is a string");
        let mut entries = entries.clone();
        entries[0] = mutation["entry"].clone();
        for member in ["lock", "identity_preimage"] {
            let mut candidate = package.clone();
            candidate[member]["dependency_selections"] = entries.clone();
            let entry_path = format!("/{member}/dependency_selections/0");
            let errors: Vec<String> = schema
                .iter_errors(&candidate)
                .map(|error| error.instance_path().to_string())
                .collect();
            assert!(
                !errors.is_empty(),
                "{id} in {member}/dependency_selections must fail the public schema"
            );
            assert!(
                errors.iter().all(|path| path.starts_with(&entry_path)),
                "{id} in {member} fails the schema only at the mutated entry: {errors:?}"
            );
        }
    }

    // A model selection binds by `{identity, digest_domain, digest}` alone;
    // a `ModelRef` carrying any further member, such as `version`, fails.
    let models = &vectors["model_selections"];
    let mut with_models = package.clone();
    for member in ["lock", "identity_preimage"] {
        with_models[member]["model_selections"] = models.clone();
    }
    let errors: Vec<String> = schema
        .iter_errors(&with_models)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect();
    assert!(
        errors.is_empty(),
        "a package with the published model selection is admitted: {errors:?}"
    );
    for mutation in vectors["model_entry_mutations"]
        .as_array()
        .expect("model entry mutations are an array")
    {
        let id = mutation["id"].as_str().expect("mutation id is a string");
        let mut entries = models.clone();
        entries[0] = mutation["entry"].clone();
        for member in ["lock", "identity_preimage"] {
            let mut candidate = with_models.clone();
            candidate[member]["model_selections"] = entries.clone();
            let entry_path = format!("/{member}/model_selections/0");
            let errors: Vec<String> = schema
                .iter_errors(&candidate)
                .map(|error| error.instance_path().to_string())
                .collect();
            assert!(
                !errors.is_empty(),
                "{id} in {member}/model_selections must fail the public schema"
            );
            assert!(
                errors.iter().all(|path| path.starts_with(&entry_path)),
                "{id} in {member} fails the schema only at the mutated entry: {errors:?}"
            );
        }
    }

    let order_vectors = &vectors["order_vectors"];
    for vector in order_vectors
        .as_array()
        .expect("order vectors are an array")
    {
        let id = vector["id"].as_str().expect("order vector id is a string");
        let selections = &vector["dependency_selections"];
        let candidate = with_dependency_selections(selections);
        assert!(
            schema.is_valid(&candidate),
            "{id} is admitted by the schema; the order rule, not the schema, decides it"
        );
        let (outcome, loci) = match dependency_order_refusal(selections) {
            None => ("admitted".to_owned(), Vec::new()),
            Some((cause, loci)) => (format!("refused:invalid_package/{cause}"), loci),
        };
        assert_eq!(outcome, vector["outcome"], "{id} order outcome");
        assert_eq!(json!(loci), vector["loci"], "{id} order loci");
    }
}

/// FR-322-AC-36: a reference into a dependency package is the
/// `dependency_reference` term in the wire body and in the node-identity
/// preimage alike, so the dependency's `package_id` and node id both enter
/// the referencing node's id.
#[trace("TC-233", "FR-322-AC-36")]
#[test]
fn tc_233_a_dependency_reference_enters_the_referencing_node_id() {
    let wire_term = schema_for_def(&parsed_fixture(SCHEMA), "SemanticTerm");
    let preimage_term = schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "PreimageTerm");
    let application = schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "ApplicationNode");
    let reference = json!({
        "term": "dependency_reference",
        "package": {
            "domain": "quire.package.semantic/v2",
            "algorithm": "sha256",
            "digest": "3333333333333333333333333333333333333333333333333333333333333333"
        },
        "node": {
            "domain": "quire.checked-semantic-node/v1",
            "digest": "5353535353535353535353535353535353535353535353535353535353535353"
        }
    });
    assert!(
        wire_term.is_valid(&reference),
        "the V2 schema admits the term"
    );
    assert!(
        preimage_term.is_valid(&reference),
        "the preimage schema admits the term"
    );

    let mut bare = reference.clone();
    bare["package"] = json!("3333333333333333333333333333333333333333333333333333333333333333");
    let mut other_domain = reference.clone();
    other_domain["package"]["domain"] = json!("quire.source.bytes/v1");
    let mut no_node = reference.clone();
    no_node
        .as_object_mut()
        .expect("the term is an object")
        .remove("node");
    let mut extra = reference.clone();
    extra["target"] = reference["node"].clone();
    for (name, mutation) in [
        ("bare-digest package", bare),
        ("another digest domain", other_domain),
        ("missing node", no_node),
        ("extra member", extra),
    ] {
        assert!(!wire_term.is_valid(&mutation), "{name} fails the V2 schema");
        assert!(
            !preimage_term.is_valid(&mutation),
            "{name} fails the preimage schema"
        );
    }

    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let call = vectors["operation_vectors"]
        .as_array()
        .expect("operation vectors are an array")
        .iter()
        .find(|vector| vector["name"] == "function-call")
        .expect("the function-call vector is published");
    let mut preimage = call["preimage"].clone();
    preimage["body"]["arguments"][0] = reference.clone();
    assert!(
        wire_term.is_valid(&preimage["body"]),
        "a call whose callee is a dependency reference is a V2 application term"
    );
    assert!(
        application.is_valid(&preimage),
        "and a valid application-node preimage"
    );
    let id = unqualified_jcs_sha256(&preimage);
    assert_ne!(
        id,
        call["sha256"].as_str().expect("vector digest is a string"),
        "a dependency callee keys differently from a local one"
    );
    let mut repackaged = preimage.clone();
    repackaged["body"]["arguments"][0]["package"]["digest"] =
        json!("5555555555555555555555555555555555555555555555555555555555555555");
    let mut renoded = preimage.clone();
    renoded["body"]["arguments"][0]["node"]["digest"] =
        json!("5454545454545454545454545454545454545454545454545454545454545454");
    let repackaged_id = unqualified_jcs_sha256(&repackaged);
    let renoded_id = unqualified_jcs_sha256(&renoded);
    assert_ne!(
        repackaged_id, id,
        "the dependency package_id enters the node id"
    );
    assert_ne!(renoded_id, id, "the referenced node id enters the node id");
    assert_ne!(renoded_id, repackaged_id, "the two changes key apart");
}

/// The outcome and locus of a dependency-call read, in the vectors' spelling.
fn dependency_call_outcome(package: &Value, result: Result<(), ReadRefusal>) -> (String, Value) {
    match result {
        Ok(()) => ("admitted".to_owned(), Value::Null),
        Err(ReadRefusal::Dependency(code, cause, locus)) => {
            (format!("refused:{code}/{cause}"), json!(locus))
        }
        Err(ReadRefusal::GraphShape(id)) => {
            let index = node_index_where(package, |node| node["node_id"] == id);
            // The locus member is the one the reader's shape check rejects,
            // not a fixed spelling, so a graph-shape refusal for another
            // reason does not match a dependency-list vector.
            let member = application_node_shape_locus(&package["semantic_graph"]["nodes"][index])
                .map_or_else(
                    || "/unclassified".to_owned(),
                    |member| member.to_owned(),
                );
            (
                "refused:invalid_semantic_graph".to_owned(),
                json!(format!("/semantic_graph/nodes/{index}{member}")),
            )
        }
        Err(other) => (format!("refused:{other:?}"), Value::Null),
    }
}

/// FR-322-AC-37 and FR-322-AC-85: the published dependency library is an
/// admitted package whose recorded `package_id` recomputes; the importer
/// selects it by that id, calls its `Int[0, 9] -> Boolean` function through a
/// `dependency_reference` callee with the Boolean node as `result_type` and no
/// `dependencies` entry for the target, and is admitted when the library is
/// supplied. Each published mutation, applied and rekeyed, keeps the V2
/// schema and is refused with exactly its recorded code, cause and locus.
#[trace("TC-233", "FR-322-AC-37", "FR-322-AC-85")]
#[test]
fn tc_233_a_dependency_call_is_admitted_and_its_mutations_refuse_at_their_loci() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(DEPENDENCY_CALL_VECTORS);
    assert_eq!(
        vectors["library"]["package"],
        "fixtures/positive-dependency-library.json"
    );
    assert_eq!(vectors["base"], "fixtures/positive-dependency-call.json");
    let library = parsed_fixture(POSITIVE_DEPENDENCY_LIBRARY);
    let base = parsed_fixture(POSITIVE_DEPENDENCY_CALL);
    let identity = vectors["library"]["identity"]
        .as_str()
        .expect("library identity")
        .to_owned();

    assert!(v2.is_valid(&library), "the library is a V2 package");
    assert_eq!(
        derive_identity_preimage(&library),
        library["identity_preimage"]
    );
    assert_eq!(
        library["package_id"]["digest"],
        unqualified_jcs_sha256(&package_id_preimage(&library)),
        "the library's recorded package_id recomputes"
    );
    assert_eq!(read_package_semantics(&library, &catalog), Ok(()));

    let library_nodes = library["semantic_graph"]["nodes"]
        .as_array()
        .expect("library nodes");
    let function = &library_nodes[declared_index(&library, &["Lib", "is_small"])];
    assert_eq!(
        vectors["library"]["function"],
        function["declaration"]["qualified_name"]
    );
    assert!(dependency_signature_is_package_independent(
        function,
        library_nodes
    ));

    assert!(v2.is_valid(&base), "the importer is a V2 package");
    assert_eq!(derive_identity_preimage(&base), base["identity_preimage"]);
    assert_eq!(
        base["package_id"]["digest"],
        unqualified_jcs_sha256(&package_id_preimage(&base)),
        "the importer's recorded package_id recomputes"
    );
    assert_eq!(
        base["lock"]["dependency_selections"],
        json!([{"identity": identity, "package_id": library["package_id"]}]),
        "the importer selects the library by its recomputed package_id"
    );
    let call = &base["semantic_graph"]["nodes"][declared_index(&base, &["App", "check"])];
    assert_eq!(vectors["call"], call["declaration"]["qualified_name"]);
    let callee = &call["body"]["arguments"][0];
    assert_eq!(callee["term"], "dependency_reference");
    assert_eq!(callee["package"], library["package_id"]);
    assert_eq!(callee["node"], function["node_id"]);
    assert_eq!(call["body"]["result_type"], function["semantic_type"]);
    assert_eq!(
        call["dependencies"],
        json!([]),
        "the target is no dependency"
    );
    assert!(
        base["semantic_graph"]["nodes"]
            .as_array()
            .expect("importer nodes")
            .iter()
            .any(|node| node["node_id"] == function["semantic_type"]),
        "the importer carries the result type node under the library's id"
    );
    let supplied = vec![(identity.clone(), library.clone())];
    assert_eq!(
        read_package_semantics_with(&base, &catalog, &supplied),
        Ok(()),
        "the dependency call is admitted with its library supplied"
    );

    let mutations = vectors["mutations"].as_array().expect("mutations");
    assert_eq!(mutations.len(), 11);
    for mutation in mutations {
        let id = mutation["id"].as_str().expect("mutation id");
        let mut candidate = base.clone();
        for patch in mutation["patch"].as_array().expect("patch") {
            apply_authored_patch(&mut candidate, patch);
        }
        rekey_stale_nodes(&mut candidate);
        let errors: Vec<String> = v2
            .iter_errors(&candidate)
            .map(|error| format!("{error} at {}", error.instance_path()))
            .collect();
        assert!(errors.is_empty(), "{id} keeps the V2 schema: {errors:?}");
        let supply = if mutation["supplied"] == false {
            &[][..]
        } else {
            &supplied[..]
        };
        let (outcome, locus) = dependency_call_outcome(
            &candidate,
            read_package_semantics_with(&candidate, &catalog, supply),
        );
        assert_eq!(outcome, mutation["outcome"], "{id} outcome");
        assert_eq!(locus, mutation["locus"], "{id} locus");
    }
}

/// FR-322's `dependency_selections` order rule: the first entry whose
/// identity repeats an earlier one refuses `conflicting-definition` at both
/// entries; otherwise the first entry not strictly after its predecessor in
/// UTF-8 byte order refuses `invalid-value` at its pointer.
fn dependency_order_refusal(selections: &Value) -> Option<(&'static str, Vec<String>)> {
    let identities: Vec<&[u8]> = selections
        .as_array()
        .expect("dependency selections are an array")
        .iter()
        .map(|entry| {
            entry["identity"]
                .as_str()
                .expect("identity is a string")
                .as_bytes()
        })
        .collect();
    let pointer = |index: usize| format!("/lock/dependency_selections/{index}");
    for (index, identity) in identities.iter().enumerate() {
        if let Some(earlier) = identities[..index].iter().position(|seen| seen == identity) {
            return Some((
                "conflicting-definition",
                vec![pointer(earlier), pointer(index)],
            ));
        }
    }
    identities
        .windows(2)
        .position(|pair| pair[0] >= pair[1])
        .map(|index| ("invalid-value", vec![pointer(index + 1)]))
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
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string"),
            "{name} binds its exact RFC 8785 JCS preimage"
        );
        // A structural vector (the model root) is no nominal preimage: the
        // closed node-preimage schema above and the JCS digest bind it.
        if vector["preimage"]["version"] == "quire.structural-node/v1" {
            continue;
        }
        integrated_schema
            .validate(&vector["preimage"])
            .unwrap_or_else(|error| panic!("{name} is integrated in the I04 schema: {error}"));
        assert!(
            semantic_preimage_is_valid(&vector["preimage"], vectors),
            "{name} satisfies the closed nominal-node semantic constraints"
        );
    }
}

#[trace("TC-233", "FR-322-AC-15", "FR-322-AC-139")]
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
        // FR-322-AC-139: wire precedes graph, so a mutation that the closed schema refuses is
        // `malformed_wire`; every other mutation is a graph defect.
        let wire_refused = mutation["refused_by"].as_str() == Some("schema");
        assert_eq!(
            mutation["expected_code"].as_str(),
            Some(if wire_refused {
                "malformed_wire"
            } else {
                "invalid_semantic_graph"
            }),
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
        "package nodes bind exact preimages, package id and owner lock joins"
    );

    let mut absent_owner = package.clone();
    replace_nominal_preimage_in_graph(
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
    replace_nominal_preimage_in_graph(
        &mut unreduced,
        2,
        "/offset",
        json!({"numerator":"2", "denominator":"2"}),
    );
    assert!(!package_nominal_nodes_are_valid(&unreduced));

    let mut signed_denominator = package.clone();
    replace_nominal_preimage_in_graph(
        &mut signed_denominator,
        2,
        "/scale/denominator",
        json!("-1"),
    );
    assert!(!schema.is_valid(&signed_denominator));

    let mut invalid_root = package.clone();
    replace_nominal_preimage_in_graph(
        &mut invalid_root,
        2,
        "/scale/numerator",
        json!("2"),
    );
    assert!(!package_nominal_nodes_are_valid(&invalid_root));

    let mut invalid_root_offset = package.clone();
    replace_nominal_preimage_in_graph(
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
    replace_nominal_preimage_in_graph(
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
    replace_node_field_in_graph(
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
    replace_node_field_in_graph(
        &mut changed_enum_type,
        0,
        "/semantic_type",
        wrong_enum_type,
    );
    refresh_package_id(&mut changed_enum_type);
    assert!(schema.is_valid(&changed_enum_type));
    assert!(!package_nominal_nodes_are_valid(&changed_enum_type));

    let mut missing_enum_dependency = package.clone();
    replace_node_field_in_graph(
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
    replace_node_field_in_graph(
        &mut changed_unit_type,
        2,
        "/semantic_type",
        wrong_unit_type,
    );
    refresh_package_id(&mut changed_unit_type);
    assert!(schema.is_valid(&changed_unit_type));
    assert!(!package_nominal_nodes_are_valid(&changed_unit_type));

    let mut missing_dimension_dependency = package.clone();
    replace_node_field_in_graph(
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
        "identity": "test/orders",
        "digest_domain": "sha256-jcs",
        "digest": "5555555555555555555555555555555555555555555555555555555555555555"
    }]);
    let model_owner = json!({
        "kind": "model", "identity": "test/orders", "node": "ix://test/orders/Example"
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
            model_owner,
            Some(model_selection),
            ("/model_selections/0/identity", json!("test/other")),
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
            unqualified_jcs_sha256(&package_id_preimage(&unjoined)),
            unjoined["package_id"]["digest"]
                .as_str()
                .expect("package digest is a string"),
            "{name} mutation keeps an exact package identity"
        );
        assert!(
            !owner_joins_lock(&unit_preimage(&unjoined), &unjoined["lock"]),
            "{name} owner does not join the lock"
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

#[trace("TC-233", "FR-201-AC-7")]
#[test]
fn tc_233_definition_library_exposes_the_exact_authored_bytes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for (exposed, path) in [
        (
            COMPLETE_VALUE_LOCK,
            "proposals/quire-v1/definitions/complete-value-lock.json",
        ),
        (
            COMPLETE_VALUE_SELECTION_VECTORS,
            "proposals/quire-v1/definitions/complete-value-selection-vectors.json",
        ),
        (
            NATIVE_DIAGNOSTICS,
            "proposals/quire-v1/definitions/native-diagnostics.md",
        ),
    ] {
        let on_disk = std::fs::read(root.join(path)).expect("authored file reads");
        assert_eq!(exposed.as_bytes(), on_disk.as_slice(), "{path}");
    }
}

#[trace("TC-233", "FR-201-AC-9")]
#[test]
fn tc_233_complete_value_lock_catalogs_every_artifact_and_separates_package_selection() {
    let lock = parsed_fixture(COMPLETE_VALUE_LOCK);
    assert_eq!(
        lock["lock_version"].as_str(),
        Some("quire.value.definition-lock/v1")
    );
    assert_eq!(lock["language_edition"].as_str(), Some("1-draft"));

    let catalog = lock["qualification_catalog"]
        .as_array()
        .expect("qualification catalog is an array");
    let mut roles = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for entry in catalog {
        assert_eq!(entry["definition"]["authority"].as_str(), Some("agent-ix"));
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
}

#[trace("TC-233", "FR-201-AC-9")]
#[test]
fn tc_233_package_selection_vectors_admit_exact_conditional_and_alternative_profiles() {
    let lock = parsed_fixture(COMPLETE_VALUE_LOCK);
    let selection = PackageSelectionRules::from_lock(&lock);
    let vectors = parsed_fixture(COMPLETE_VALUE_SELECTION_VECTORS);
    assert_eq!(vectors["lock"]["lock_version"], lock["lock_version"]);
    let always = string_list(&vectors["always_roles"], "always roles");
    assert_eq!(
        always, selection.always,
        "vectors select the lock's always slot"
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

/// Replace one application-bearing node of a package, recompute its exact
/// dependencies and key it and the package from the preimage.
fn replace_application_node(package: &mut Value, index: usize, mut node: Value) {
    node["dependencies"] = json!(application_dependencies(&node["body"]));
    package["semantic_graph"]["nodes"][index] = node;
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

const TYPED_DIAGNOSTIC_CAUSE: &str =
    include_str!("../spec/objects/foundation/FR-272-typed-diagnostic-cause.md");

/// Causes a native-diagnostics row lists for exactly the given code: the
/// backticked spellings of its cause cell before the first `;`.
fn native_diagnostic_row_causes(code: &str) -> BTreeSet<String> {
    diagnostic_row_causes(NATIVE_DIAGNOSTICS, code)
}

/// The one table row of `document` whose first cell is exactly the given
/// code.
fn diagnostic_row<'a>(document: &'a str, code: &str) -> &'a str {
    let rows = document
        .lines()
        .filter(|line| {
            line.split('|')
                .nth(1)
                .is_some_and(|cell| cell.trim() == format!("`{code}`"))
        })
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "the document has exactly one `{code}` row");
    rows[0]
}

/// The backticked spellings of `text`.
fn backticked(text: &str) -> BTreeSet<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// Causes the one table row of `document` whose first cell is exactly the
/// given code lists: the backticked spellings of its second cell before the
/// first `;`.
fn diagnostic_row_causes(document: &str, code: &str) -> BTreeSet<String> {
    let cause_cell = diagnostic_row(document, code)
        .split('|')
        .nth(2)
        .and_then(|cell| cell.split(';').next())
        .expect("diagnostic row has a cause cell");
    backticked(cause_cell)
}

/// FR-272's canonical cause payload names: the backticked spellings of the
/// sentence that introduces them.
fn canonical_payload_names() -> BTreeSet<String> {
    let intro = "Cause payload fields use these exact ordered names when applicable:";
    let start = TYPED_DIAGNOSTIC_CAUSE
        .find(intro)
        .expect("FR-272 states its canonical payload names")
        + intro.len();
    let list = &TYPED_DIAGNOSTIC_CAUSE[start..];
    let end = list.find(". ").expect("the canonical name list ends");
    backticked(&list[..end])
}

/// Payload keys the one `code` row of `document` names: every backticked
/// name written `key`: after its cause list (the rest of the second cell after
/// its first `;`, and every later cell), whether or not it is canonical.
fn diagnostic_row_payload_keys(document: &str, code: &str) -> BTreeSet<String> {
    payload_keys_in(&diagnostic_row_payload_text(document, code))
}

/// The payload text of the `code` row: the rest of its second cell after the
/// first `;`, and every later cell.
fn diagnostic_row_payload_text(document: &str, code: &str) -> String {
    let cells = diagnostic_row(document, code)
        .split('|')
        .skip(2)
        .collect::<Vec<_>>();
    let mut payload = cells[0]
        .split_once(';')
        .map(|(_, rest)| rest.to_owned())
        .unwrap_or_default();
    for cell in &cells[1..] {
        payload.push_str(cell);
    }
    payload
}

/// Every backticked name in `payload` written `key`:.
fn payload_keys_in(payload: &str) -> BTreeSet<String> {
    let pieces = payload.split('`').collect::<Vec<_>>();
    pieces
        .iter()
        .enumerate()
        .skip(1)
        .step_by(2)
        .filter(|(index, _)| {
            pieces
                .get(index + 1)
                .is_some_and(|after| after.starts_with(':'))
        })
        .map(|(_, name)| (*name).to_owned())
        .collect()
}

/// The payload keys the `code` row names for one `cause`: the stretch of its
/// payload text from `or `cause`,` (as in "For `cause`, ...") to the next
/// cause's; a row of one cause has the whole payload text.
fn diagnostic_row_cause_payload_keys(
    document: &str,
    code: &str,
    cause: &str,
    causes: &[&str],
) -> BTreeSet<String> {
    let payload = diagnostic_row_payload_text(document, code);
    if causes.len() == 1 {
        return payload_keys_in(&payload);
    }
    let start_of = |name: &str| payload.find(&format!("or `{name}`,"));
    let start = start_of(cause).unwrap_or_else(|| panic!("`{code}` payload introduces `{cause}`"));
    let end = causes
        .iter()
        .filter_map(|other| start_of(other))
        .filter(|at| *at > start)
        .min()
        .unwrap_or(payload.len());
    payload_keys_in(&payload[start..end])
}

/// `text` with every run of whitespace, line breaks included, made one space.
fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A requirement's normative text: everything before its acceptance
/// criteria, so a criterion restating a rule cannot stand in for the rule.
fn before_criteria(document: &str) -> &str {
    document
        .split("## Acceptance Criteria")
        .next()
        .unwrap_or(document)
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
    let profile_roles = string_set(&catalog["profile_law_roles"], "profile law roles");
    assert!(
        profile_roles.iter().all(|role| !roles.contains_key(*role)),
        "a profile law role names no fixed catalog definition: the package lock selects it"
    );
    assert_eq!(
        string_set(
            &defs["OperationLaw"]["properties"]["role"]["enum"],
            "schema law roles"
        ),
        roles
            .keys()
            .map(String::as_str)
            .chain(profile_roles.iter().copied())
            .collect::<BTreeSet<_>>(),
        "the schemas enumerate exactly the catalogued law roles and profile law roles"
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
        .map(|entry| {
            (
                &entry["definition"]["authority"],
                &entry["definition"]["identity"],
            )
        })
        .collect::<Vec<_>>();
    for (role, references) in roles {
        let references = references.as_array().expect("role references are an array");
        assert!(!references.is_empty(), "law role {role} names a definition");
        for reference in references {
            assert!(
                locked.contains(&(&reference["authority"], &reference["identity"])),
                "law role {role} definition {reference} is a complete-value lock definition"
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
        families
            .iter()
            .copied()
            .filter(|family| *family != "object")
            .collect::<BTreeSet<_>>(),
        "every family except object, including aggregate, binder and clause terms, is a term; \
         an object term fits only a field_owner position (FR-322)"
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

    let leaf_sources = string_set(&catalog["leaf_sources"], "leaf sources");
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
            None => {
                families.contains(result) || ["member", "absence", "arm_body"].contains(&result)
            }
        };
        assert!(
            result_fits,
            "{identity} result {result} is a closed result form"
        );
        assert!(
            string_set(&entry["laws"], "entry laws")
                .iter()
                .all(|role| roles.contains_key(*role) || profile_roles.contains(role)),
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
            entry["leaves"].is_null()
                || entry["leaves"]
                    .as_str()
                    .is_some_and(|source| leaf_sources.contains(source)),
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
                !["eq", "ne", "lt", "le", "gt", "ge"].contains(&last),
                "{identity}: IEEE equality and ordering are the numeric_equal, total_order and bit_identical identities (FR-148)"
            );
        }
        if family == "text" && operation.starts_with("to_") {
            let expected_laws = if matches!(operation, "to_float32" | "to_float64") {
                json!(["text_profile", "ieee_profile"])
            } else {
                json!(["text_profile"])
            };
            assert_eq!(entry(identity)["laws"], expected_laws, "{identity} laws");
            assert_eq!(entry(identity)["member"], "type_argument", "{identity} member");
            assert_eq!(entry(identity)["constraints"][0]["kind"], "member_family", "{identity} target family constraint");
            assert_ne!(entry(identity)["mode"], json!("text_profile"), "{identity} takes no text_profile mode");
        } else if family == "text" {
            assert_eq!(entry(identity)["laws"], json!(["text_profile"]), "{identity} laws");
            assert_eq!(entry(identity)["mode"], json!("text_profile"), "{identity} mode");
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
    assert_eq!(
        entry("quire.op.temporal.clause")["laws"],
        json!(["temporal_profile"]),
        "quire.op.temporal.clause law"
    );
    assert_eq!(
        entry("quire.op.temporal.clause")["member"],
        Value::Null,
        "a temporal clause names no operator: its formula's operators are their own identities (FR-370)"
    );
    for operator in [
        "holds",
        "true",
        "false",
        "not",
        "and",
        "or",
        "implies",
        "eventually",
        "always",
        "once",
        "historically",
        "until",
        "release",
        "since",
        "triggered",
    ] {
        let identity = format!("quire.op.temporal.{operator}");
        assert_eq!(
            entry(&identity)["operator"],
            json!("temporal_formula"),
            "{identity} class"
        );
        assert_eq!(
            entry(&identity)["laws"],
            json!([]),
            "{identity} carries no law"
        );
    }
    assert_eq!(
        entry("quire.op.temporal.fair")["member"],
        json!("fairness"),
        "a fairness constraint carries the fairness member"
    );
    for (identity, role) in [
        ("quire.op.claim.clause", "temporal_profile"),
        ("quire.op.protocol.control", "protocol_profile"),
        ("quire.op.state.transition", "protocol_profile"),
    ] {
        assert_eq!(entry(identity)["laws"], json!([role]), "{identity} law");
        assert_eq!(
            entry(identity)["member"],
            json!("profile_operator"),
            "{identity} names the exact profile operator it applies"
        );
    }
    assert_eq!(
        entry("quire.op.rational.div")["operands"],
        json!(["rational_promotable", "rational_promotable"])
    );
    assert_eq!(
        string_set(&groups["rational_promotable"], "rational_promotable"),
        BTreeSet::from(["integer", "rational"])
    );
    assert!(
        entry("quire.op.rational.div")["constraints"]
            .as_array()
            .is_some_and(|constraints| constraints
                .iter()
                .any(|constraint| constraint["kind"] == "promotes_exact")),
        "rational division states its exact integer promotion"
    );
    for (identity, kind) in [
        ("quire.op.numeric.narrow", "range_narrowing"),
        ("quire.op.rational.narrow", "rational_narrowing"),
    ] {
        assert!(
            entry(identity)["constraints"]
                .as_array()
                .is_some_and(|constraints| constraints
                    .iter()
                    .any(|constraint| constraint["kind"] == kind)),
            "{identity} is a checked {kind} obligation"
        );
        assert!(entry(identity)["mode"].is_null(), "{identity} never rounds");
    }
    for identity in [
        "quire.op.collection.set",
        "quire.op.collection.bag",
        "quire.op.collection.ordered_set",
        "quire.op.collection.map",
        "quire.op.collection.flat_map",
        "quire.op.collection.flatten",
        "quire.op.collection.convert",
    ] {
        assert_eq!(
            entry(identity)["leaves"],
            json!("result_inner"),
            "{identity} carries the element text leaves of its keyed result"
        );
    }
    for identity in [
        "quire.op.quantity.convert",
        "quire.op.collection.sum.quantity",
    ] {
        assert_eq!(
            entry(identity)["mode"],
            json!("rounding"),
            "{identity} mode"
        );
    }
    assert!(
        string_set(&catalog["type_pinned_modes"]["rounding"], "rounding pins").contains("quantity"),
        "a quantity rounding mode agrees with the rounding its type pins"
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
    for node in nodes.iter().filter(|node| is_application_node(node)) {
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

    // A named operation vector, a mutation label and the node edit it applies.
    type GraphShapeEdit = (&'static str, &'static str, fn(&mut Value));
    let edits: [GraphShapeEdit; 4] = [
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
        .position(|node| node["node_tag"] == "function" && is_application_node(node))
        .expect("all-families publishes a function whose body is a call");
    for node in family_nodes.iter().filter(|node| is_application_node(node)) {
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

/// A minimal package exercising `member_descriptor`'s `"operation"` arm
/// alone: a `model` node of `form` binding `run` to a `function` node whose
/// result is a boolean scalar type, with nothing else in the graph.
fn operation_member_package(form: &str) -> Value {
    json!({
        "lock": {},
        "semantic_graph": {
            "nodes": [
                {
                    "node_id": "model-1",
                    "node_tag": "model",
                    "semantic_form": form,
                    "body": {
                        "term": "aggregate",
                        "members": [
                            {
                                "term": "binding",
                                "name": "run",
                                "value": {"term": "reference", "target": "fn-1"}
                            }
                        ]
                    }
                },
                {
                    "node_id": "fn-1",
                    "node_tag": "function",
                    "semantic_type": "bool-1"
                },
                {
                    "node_id": "bool-1",
                    "node_tag": "scalar_type",
                    "semantic_form": "boolean"
                }
            ]
        }
    })
}

/// FR-322-AC-25: an `operation` member resolves only against a declaring
/// `model` node of `semantic_form` `object_type`, `process`,
/// `state_machine` or `persistence_interface`. `systems_interface` is
/// exercised alongside the business-model refusals to pin its deliberate
/// exclusion from the dispatch set (FR-151, FR-152).
#[trace("TC-233", "FR-322-AC-25")]
#[test]
fn tc_233_operation_member_resolves_only_against_an_operation_bearing_model_form() {
    let catalog = json!({});
    let member = json!({"kind": "operation", "declaration": "model-1", "name": "run"});
    let accepted_forms = [
        "object_type",
        "process",
        "persistence_interface",
        "state_machine",
    ];
    assert_eq!(
        OPERATION_BEARING_MODEL_FORMS, accepted_forms,
        "the operation-bearing forms are exactly FR-322-AC-25's object type, process, \
         persistence interface and state machine"
    );
    for form in accepted_forms {
        let package = operation_member_package(form);
        let oracle = OperationOracle::for_package(&catalog, &package);
        assert!(
            oracle.member_descriptor(&member).is_some(),
            "a dispatched operation resolves against a declaring {form}"
        );
    }
    let refused_forms = [
        "record_value_type",
        "event_type",
        "namespace",
        "systems_interface",
    ];
    for form in refused_forms {
        assert!(
            !OPERATION_BEARING_MODEL_FORMS.contains(&form),
            "{form} is not one of the operation-bearing forms"
        );
        let package = operation_member_package(form);
        let oracle = OperationOracle::for_package(&catalog, &package);
        assert!(
            oracle.member_descriptor(&member).is_none(),
            "a dispatched operation does not resolve against a declaring {form}"
        );
    }
}

/// FR-322-AC-25's refusal exercised through the full operations reader
/// (`package_operations_refusal`, not `member_descriptor` alone): a
/// `systems_interface` and a `record_value_type` declaring node each refuse
/// a real `dispatch_call` application as `operator-ineligible`/`ill_typed`.
///
/// `dispatch_call`'s catalog result form is itself `member`, so its
/// `result` is `self.member_descriptor(&operation["member"])` -- the exact
/// call `member_resolves` also makes. A failing `member_descriptor` already
/// exhausts `result` at the earlier `result.ok_or(Refusal::OperatorIneligible)?`
/// before the arity/family/member/constraint/leaves let-else is ever
/// reached, so no `dispatch_call` scenario -- `systems_interface` included --
/// can tell `member_resolves` apart from a mutant that hard-codes it `true`.
/// `quire.op.model.reaches` is not coupled that way: its catalog result form
/// is `boolean`, computed independently of its `relationship_end` member, so
/// an unresolved relationship-end declaration reaches the let-else and is
/// refused by `member_resolves` alone, dying if that term is mutated.
#[trace("TC-233", "FR-322-AC-25")]
#[test]
fn tc_233_operator_ineligible_member_refusals_refuse_through_the_full_operations_reader() {
    let catalog = parsed_fixture(OPERATION_CATALOG);

    for form in ["systems_interface", "record_value_type"] {
        let mut package = operation_member_package(form);
        let nodes = package["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("package nodes are an array");
        nodes.push(json!({"node_id": "reftype-1", "node_tag": "composite_type", "semantic_form": "reference"}));
        nodes.push(
            json!({"node_id": "refarg-1", "node_tag": "expression", "semantic_type": "reftype-1"}),
        );
        nodes.push(json!({
            "node_id": "app-1",
            "node_tag": "function",
            "semantic_type": "bool-1",
            "body": {
                "term": "application",
                "operator": "call",
                "operation": {
                    "identity": "quire.op.model.dispatch_call",
                    "laws": [],
                    "leaves": [],
                    "mode": null,
                    "member": {"kind": "operation", "declaration": "model-1", "name": "run"}
                },
                "arguments": [
                    {"term": "reference", "target": "refarg-1"}
                ],
                "result_type": "bool-1"
            }
        }));
        assert_eq!(
            package_operations_refusal(&package, &catalog),
            Err(OperationRefusal::OperatorIneligible),
            "a {form} declaring node refuses a dispatch_call as operator-ineligible"
        );
    }
    assert_eq!(
        OperationRefusal::OperatorIneligible.code(),
        "ill_typed",
        "operator-ineligible carries the ill_typed code"
    );

    let reaches_package = json!({
        "lock": {},
        "semantic_graph": {
            "nodes": [
                {"node_id": "bool-1", "node_tag": "scalar_type", "semantic_form": "boolean"},
                {"node_id": "reftype-1", "node_tag": "composite_type", "semantic_form": "reference"},
                {"node_id": "target-1", "node_tag": "expression", "semantic_type": "reftype-1"},
                {
                    "node_id": "app-1",
                    "node_tag": "function",
                    "semantic_type": "bool-1",
                    "body": {
                        "term": "application",
                        "operator": "reaches",
                        "operation": {
                            "identity": "quire.op.model.reaches",
                            "laws": [],
                            "leaves": [],
                            "mode": null,
                            "member": {
                                "kind": "relationship_end",
                                "declaration": "target-1",
                                "name": "end"
                            }
                        },
                        "arguments": [
                            {"term": "reference", "target": "target-1"},
                            {"term": "reference", "target": "target-1"}
                        ],
                        "result_type": "bool-1"
                    }
                }
            ]
        }
    });
    assert_eq!(
        package_operations_refusal(&reaches_package, &catalog),
        Err(OperationRefusal::OperatorIneligible),
        "an unresolved relationship-end member refuses a reaches call as operator-ineligible"
    );
}

#[trace("TC-233", "FR-322-AC-19")]
#[test]
fn tc_233_operation_mutations_rekey_or_refuse_with_their_recorded_cause() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let mutations = &vectors["operation_mutations"];
    let mut package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let renames = BTreeMap::<String, String>::new();
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
            .chain([ReadRefusal::STALE_NODE_KEY.0])
            .collect::<BTreeSet<_>>(),
        "the mutations exercise every operation refusal cause and stale-node-key"
    );

    for mutation in mutations.as_array().expect("mutations are an array") {
        let name = mutation["name"]
            .as_str()
            .expect("mutation name is a string");
        let base = mutation["base"]
            .as_str()
            .expect("mutation base is a string");
        let base_digest = operation_vector(&vectors, base)["sha256"]
            .as_str()
            .expect("base vector digest is a string");
        assert_eq!(
            Some(base_digest),
            mutation["retained_sha256"].as_str(),
            "{name} records exactly its base vector key"
        );
        let current_base_digest = renames.get(base_digest).map_or(base_digest, String::as_str);
        let index = node_index_by_digest(&package, current_base_digest)
            .unwrap_or_else(|| panic!("{name} base vector remains in the modelized package"));
        assert_eq!(
            package["semantic_graph"]["nodes"][index]["node_id"]["digest"], current_base_digest,
            "{name} starts from the current key of its base vector"
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
        let mut current_shape = node.clone();
        current_shape["dependencies"] = json!(application_dependencies(&node["body"]));
        let retained_id = package["semantic_graph"]["nodes"][index]["node_id"].clone();
        let shape_valid = application_node_shape_is_valid(&current_shape);
        assert!(
            shape_valid || mutation["kind"] != "stale_key",
            "{name}: a stale-key mutation keeps a well-formed graph"
        );
        let mut retained = package.clone();
        retained["semantic_graph"]["nodes"][index] = current_shape;
        assert_eq!(
            read_package_semantics(&retained, &catalog),
            Err(if shape_valid {
                ReadRefusal::StaleNodeKey(retained_id)
            } else {
                ReadRefusal::GraphShape(retained_id)
            }),
            "{name} retaining its key refuses stale-node-key after graph shape and before any declaration or operation cause"
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
                assert_eq!(
                    (
                        mutation["expected_cause"].as_str(),
                        mutation["expected_code"].as_str()
                    ),
                    (
                        Some(ReadRefusal::STALE_NODE_KEY.0),
                        Some(ReadRefusal::STALE_NODE_KEY.1)
                    ),
                    "{name} records the stale-node-key cause and its invalid_package code"
                );
                assert_eq!(
                    read_package_semantics(&candidate, &catalog),
                    package_operations_refusal(&candidate, &catalog)
                        .map_err(ReadRefusal::Operation),
                    "{name} rekeyed is no longer stale"
                );
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
                if let Some(rekeyed_sha256) = mutation["rekeyed_sha256"].as_str() {
                    let nodes = candidate["semantic_graph"]["nodes"]
                        .as_array()
                        .expect("candidate nodes are an array");
                    let preimage = application_node_preimage(candidate_node, nodes)
                        .unwrap_or_else(|| panic!("{name} has an application-node preimage"));
                    assert_eq!(
                        preimage, mutation["rekeyed_preimage"],
                        "{name} rekeys from the exact expected application-node preimage"
                    );
                    assert_eq!(
                        unqualified_jcs_sha256(&preimage),
                        rekeyed_sha256,
                        "{name} expected preimage has its recorded JCS digest"
                    );
                    assert_eq!(
                        candidate_node["node_id"]["digest"], rekeyed_sha256,
                        "{name} rekeys to exactly its expected identity digest"
                    );
                    assert_ne!(
                        candidate["package_id"], package["package_id"],
                        "{name} changes the package id"
                    );
                }
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
        .chain(
            FrameRefusal::ALL
                .into_iter()
                .map(|refusal| (refusal.cause(), refusal.code())),
        )
        .chain([ReadRefusal::STALE_NODE_KEY])
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
        let code = package_cause_code(
            mutation["expected_cause"]
                .as_str()
                .expect("every operation mutation records its cause"),
        );
        assert_eq!(
            Some(expected_code),
            code,
            "{} records its cause's code",
            mutation["name"]
        );
    }
}

fn published_v2_packages() -> [(&'static str, Value); 7] {
    let operation = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    [
        ("operation-identities", operation),
        ("all-families", parsed_fixture(POSITIVE_ALL_FAMILIES)),
        (
            "clause-operations",
            parsed_fixture(POSITIVE_CLAUSE_OPERATIONS),
        ),
        (
            "control-operations",
            parsed_fixture(POSITIVE_CONTROL_OPERATIONS),
        ),
        (
            "nominal-identities",
            parsed_fixture(POSITIVE_NOMINAL_IDENTITIES),
        ),
        (
            "recursive-records",
            parsed_fixture(POSITIVE_RECURSIVE_RECORDS),
        ),
        (
            "extension-nodes",
            parsed_fixture(POSITIVE_EXTENSION_NODES),
        ),
    ]
}

#[trace("TC-233", "interface_004-AC-1")]
#[test]
fn tc_233_every_v2_application_names_exactly_one_catalogued_operation_identity() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
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
}

#[trace("TC-233", "interface_004-AC-2")]
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
    for prefix in ["/semantic_graph/nodes"] {
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
    for prefix in ["/semantic_graph/nodes"] {
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
    let operations = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let anonymous = operations["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| {
            node["node_tag"] == "scalar_type"
                && node["occurrences"]
                    .as_array()
                    .is_some_and(|occurrences| occurrences.iter().all(|o| o["role"] == "type"))
        })
        .expect("the operation package uses a builtin scalar only as a type occurrence");
    let mut named_builtin = operations.clone();
    for prefix in ["/semantic_graph/nodes"] {
        named_builtin
            .pointer_mut(&format!("{prefix}/{anonymous}"))
            .and_then(Value::as_object_mut)
            .expect("node is an object")
            .insert(
                "declaration".to_owned(),
                json!({"qualified_name": ["Example", "integer"]}),
            );
    }
    refresh_package_id(&mut named_builtin);
    assert!(
        !v2.is_valid(&named_builtin),
        "a builtin or anonymous type occurrence carrying an invented declaration fails the schema"
    );

    let function = |name: &str| {
        families["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| node["declaration"]["qualified_name"] == json!(["Example", name]))
            .unwrap_or_else(|| panic!("{name} is declared"))
    };
    let call = function("run");
    let callee = function("helper");
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
        replace_node_field_in_graph(
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

/// FR-322-AC-81: a name is unique within its namespace. A unit, a dimension
/// and a record that carry one name coexist; two units of one name refuse
/// `ambiguous_declaration`/`ambiguous-name` at the later node.
#[trace("TC-233", "FR-322-AC-81")]
#[test]
fn tc_233_declaration_names_are_unique_per_namespace() {
    let node = |id: &str, tag: &str, form: &str| {
        json!({
            "node_id": walk_id(id),
            "node_tag": tag,
            "semantic_form": form,
            "declaration": {"qualified_name": ["ms"]}
        })
    };
    let package = |nodes: Vec<Value>| json!({"semantic_graph": {"nodes": nodes}});
    let coexisting = package(vec![
        node("a", "scalar_type", "unit"),
        node("b", "composite_type", "record"),
        node("c", "scalar_type", "dimension"),
    ]);
    assert_eq!(package_declaration_refusal(&coexisting), Ok(()));
    let twice = package(vec![
        node("a", "scalar_type", "unit"),
        node("b", "scalar_type", "unit"),
    ]);
    assert_eq!(
        package_declaration_refusal(&twice),
        Err((DeclarationRefusal::AmbiguousName, walk_id("b")))
    );
    let two_types = package(vec![
        node("a", "composite_type", "record"),
        node("b", "scalar_type", "enum"),
    ]);
    assert_eq!(
        package_declaration_refusal(&two_types),
        Err((DeclarationRefusal::AmbiguousName, walk_id("b")))
    );
}

/// Index of the node whose root application names `identity`.
fn node_with_operation(package: &Value, identity: &str) -> usize {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .position(|node| node["body"]["operation"]["identity"] == identity)
        .unwrap_or_else(|| panic!("a node applies {identity}"))
}

#[trace("TC-233", "FR-322-AC-22")]
#[test]
fn tc_233_clause_applications_bind_a_selected_profile_law_and_one_profile_member() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    let temporal = node_with_operation(&package, "quire.op.temporal.clause");
    let protocol = node_with_operation(&package, "quire.op.protocol.control");
    let operation = |index: usize| format!("/semantic_graph/nodes/{index}/body/operation");
    let refusal = |edit: &dyn Fn(&mut Value)| {
        let mut candidate = package.clone();
        edit(&mut candidate);
        package_operations_refusal(&candidate, &catalog)
    };

    assert_eq!(
        refusal(&|candidate| {
            *candidate
                .pointer_mut(&format!("{}/laws", operation(temporal)))
                .expect("laws") = json!([]);
        }),
        Err(OperationRefusal::LawMissing),
        "a clause without its profile law refuses"
    );
    let protocol_law = package
        .pointer(&format!("{}/laws/0/definition", operation(protocol)))
        .expect("protocol law")
        .clone();
    assert_eq!(
        refusal(&|candidate| {
            *candidate
                .pointer_mut(&format!("{}/laws/0/definition", operation(temporal)))
                .expect("temporal law") = protocol_law.clone();
        }),
        Err(OperationRefusal::LawMismatch),
        "a protocol definition cannot stand in the temporal_profile role"
    );
    assert_eq!(
        refusal(&|candidate| {
            for prefix in ["/lock", "/identity_preimage"] {
                candidate
                    .pointer_mut(&format!("{prefix}/profile_selections"))
                    .and_then(Value::as_array_mut)
                    .expect("profile selections")
                    .retain(|row| row["role"] != "temporal_profile");
            }
            refresh_package_id(candidate);
        }),
        Err(OperationRefusal::LawUnselected),
        "a profile law the package lock does not select refuses"
    );
    assert_eq!(
        package.pointer(&format!("{}/member", operation(temporal))),
        Some(&Value::Null),
        "a temporal clause carries no member; its operators are quire.op.temporal.* applications"
    );
    assert_eq!(
        refusal(&|candidate| {
            *candidate
                .pointer_mut(&format!("{}/member", operation(temporal)))
                .expect("member") = json!({"kind": "profile_operator", "operator": "eventually"});
        }),
        Err(OperationRefusal::MemberMismatch),
        "a temporal clause naming a profile operator refuses"
    );
    for member in [
        json!({"kind": "profile_operator", "operator": "no_such_operator"}),
        json!(null),
    ] {
        assert_eq!(
            refusal(&|candidate| {
                *candidate
                    .pointer_mut(&format!("{}/member", operation(protocol)))
                    .expect("member") = member.clone();
            }),
            Err(OperationRefusal::MemberMismatch),
            "clause member {member} is not an operator of the selected protocol profile"
        );
    }
}

#[trace("TC-233", "FR-322-AC-23")]
#[test]
fn tc_233_application_result_types_and_literal_types_are_checked_not_inferred() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let div = operation_vector_node(&package, &vectors, "rational-div-integer-operands");
    let id = |role: &str| {
        let (tag, form) = match role {
            "rational" => ("scalar_type", "rational"),
            "text" => ("scalar_type", "text"),
            "range" => ("bounded_domain", "integer_range"),
            other => panic!("unknown operation-identities role {other}"),
        };
        node_ref_digest(&published_digest(&package, tag, form, None))
    };
    let with = |edit: &dyn Fn(&mut Value)| {
        let mut candidate = package.clone();
        edit(&mut candidate);
        package_operations_refusal(&candidate, &catalog)
    };
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    assert_eq!(
        with(&|candidate| {
            candidate["semantic_graph"]["nodes"][div]["body"]["result_type"] = id("rational");
        }),
        Err(OperationRefusal::OperatorIneligible),
        "a root result_type differing from the node semantic_type refuses"
    );
    assert_eq!(
        with(&|candidate| {
            let node = &mut candidate["semantic_graph"]["nodes"][div];
            node["semantic_type"] = id("text");
            node["body"]["result_type"] = id("text");
        }),
        Err(OperationRefusal::OperatorIneligible),
        "a result_type the operation cannot produce refuses"
    );
    let literal = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| {
            node["node_tag"] == "value"
                && node["semantic_form"] == "literal"
                && node["body"]["value"] == "3"
                && node["body"]["type"] == id("range")
        })
        .expect("literal 3 is published");
    assert_eq!(
        package["semantic_graph"]["nodes"][literal]["body"]["type"],
        id("range"),
        "the literal carries its declared type"
    );
    let inline = |ty: &str| {
        let mut literal_term = package["semantic_graph"]["nodes"][literal]["body"].clone();
        literal_term["type"] = id(ty);
        with(&|candidate| {
            candidate["semantic_graph"]["nodes"][div]["body"]["arguments"][0] =
                literal_term.clone();
        })
    };
    assert_eq!(
        inline("range"),
        Ok(()),
        "an inline literal of its declared type is admitted"
    );
    assert_eq!(
        inline("text"),
        Err(OperationRefusal::OperatorIneligible),
        "an inline literal whose type disagrees with its value kind refuses"
    );
}

#[trace("TC-233", "FR-322-AC-24")]
#[test]
fn tc_233_numeric_conversions_classify_and_rational_division_promotes_exactly() {
    let int = |a: i32| BigInt::from(a);
    let four = |a: i32, b: i32, c: i32, d: i32| Some([int(a), int(b), int(c), int(d)]);
    use NumericBounds::{Decimal, Int, Integer, Rational};
    for (from, to, expected) in [
        (Int(int(-9), int(9)), Integer, ConversionClass::Exact),
        (
            Int(int(-9), int(9)),
            Rational(four(-9, 9, 1, 1)),
            ConversionClass::Exact,
        ),
        (
            Int(int(0), int(300)),
            Decimal(four(0, 20000, 2, 2)),
            ConversionClass::RangeNarrowing,
        ),
        (
            Integer,
            Int(int(0), int(9)),
            ConversionClass::RangeNarrowing,
        ),
        (
            Decimal(four(0, 99999, 3, 3)),
            Decimal(four(0, 9999, 2, 2)),
            ConversionClass::ScaleReduction,
        ),
        (
            Rational(four(-9, 9, 1, 9)),
            Rational(four(-9, 9, 1, 3)),
            ConversionClass::RationalNarrowing,
        ),
        (
            Rational(None),
            Rational(four(-9, 9, 1, 9)),
            ConversionClass::RationalNarrowing,
        ),
    ] {
        assert_eq!(
            conversion_class(&from, &to),
            Some(expected),
            "{from:?} -> {to:?}"
        );
    }

    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let div = operation_vector_node(&package, &vectors, "rational-div-integer-operands");
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    let entry = catalog["operations"]
        .as_array()
        .expect("catalog operations")
        .iter()
        .find(|entry| entry["identity"] == "quire.op.rational.div")
        .expect("rational.div is catalogued");
    assert!(
        entry.to_string().contains("promotes_exact"),
        "rational.div carries the promotes_exact constraint"
    );
    let decimal_literal = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .find(|node| node["body"]["value_kind"] == "decimal" && node["body"]["value"] == "1.25")
        .expect("the decimal literal 1.25 is published");
    let decimal = decimal_literal["node_id"].clone();
    let mut candidate = package.clone();
    candidate["semantic_graph"]["nodes"][div]["body"]["arguments"][1]["target"] = decimal;
    assert_eq!(
        package_operations_refusal(&candidate, &catalog),
        Err(OperationRefusal::OperatorIneligible),
        "a decimal operand is not exactly promotable to the rational result"
    );
}

#[trace("TC-233", "FR-272-AC-7")]
#[test]
fn tc_233_a_stale_node_key_precedes_declaration_refusals() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let families = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let nodes = families["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let declared = |name: &str| {
        nodes
            .iter()
            .position(|node| node["declaration"]["qualified_name"] == json!(["Example", name]))
            .unwrap_or_else(|| panic!("{name} is declared"))
    };
    let (call, callee) = (declared("run"), declared("helper"));
    let mut ambiguous = families.clone();
    let mut node = nodes[call].clone();
    node["declaration"] = nodes[callee]["declaration"].clone();
    replace_application_node(&mut ambiguous, call, node);
    assert!(matches!(
        read_package_semantics(&ambiguous, &catalog),
        Err(ReadRefusal::Declaration(
            DeclarationRefusal::AmbiguousName,
            _
        ))
    ));
    let other = nodes
        .iter()
        .position(|node| node["node_id"] != nodes[call]["node_id"] && is_application_node(node))
        .expect("a second application-bearing node is published");
    assert_ne!(other, call);
    ambiguous["semantic_graph"]["nodes"][other]["semantic_type"]["digest"] = json!("a0".repeat(32));
    assert_eq!(
        read_package_semantics(&ambiguous, &catalog),
        Err(ReadRefusal::StaleNodeKey(nodes[other]["node_id"].clone())),
        "a stale application-node key is reported before the declaration ambiguity"
    );
}

/// Every published V2 fixture, including the clause and control fixtures the
/// operation catalog exercises, carries only current application-node keys:
/// the full normative reader (which checks node-key currency before any
/// declaration or operation refusal) admits each one outright. A digest edit
/// to any application node's retained key would surface here as
/// `ReadRefusal::StaleNodeKey` rather than passing silently, unlike a check
/// that calls only `package_operations_refusal`, which never inspects node
/// keys at all.
#[trace("TC-233", "FR-272-AC-7")]
#[test]
fn tc_233_every_published_package_carries_only_current_application_node_keys() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    for (label, package) in published_v2_packages() {
        assert_eq!(
            read_package_semantics(&package, &catalog),
            Ok(()),
            "{label}: the full reader admits it, so every application node key is current"
        );
    }
}

/// The `unimplemented_capability` cause set is closed: the native-diagnostics
/// row and the FR-272 row each list exactly its three causes and name the
/// `backend`, `construct`, `construct_name`, `location` and `capability`
/// payload keys.
#[trace("TC-233", "FR-272-AC-25")]
#[test]
fn tc_233_unimplemented_capability_cause_set_is_closed() {
    let causes = ["backend-not-supplied", "untranslatable-entry", "unimplemented-meaning"]
        .iter()
        .map(|cause| (*cause).to_owned())
        .collect::<BTreeSet<_>>();
    for (document, label) in [
        (NATIVE_DIAGNOSTICS, "native-diagnostics"),
        (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
    ] {
        assert_eq!(
            diagnostic_row_causes(document, "unimplemented_capability"),
            causes,
            "{label} `unimplemented_capability` row lists exactly its causes"
        );
        let keys = diagnostic_row_payload_keys(document, "unimplemented_capability");
        for key in ["backend", "construct", "construct_name", "location", "capability"] {
            assert!(
                keys.contains(key),
                "{label} `unimplemented_capability` row names `{key}`"
            );
        }
    }
}

/// The `invalid_quire_context` cause set is closed: the native-diagnostics row
/// and the FR-272 row each list exactly `unsupported-render-form` and name the
/// `view` and `form` payload keys.
#[trace("TC-233", "FR-272-AC-24")]
#[test]
fn tc_233_invalid_quire_context_cause_set_is_closed() {
    let causes = BTreeSet::from(["unsupported-render-form".to_owned()]);
    for (document, label) in [
        (NATIVE_DIAGNOSTICS, "native-diagnostics"),
        (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
    ] {
        assert_eq!(
            diagnostic_row_causes(document, "invalid_quire_context"),
            causes,
            "{label} `invalid_quire_context` row lists exactly its causes"
        );
        let keys = diagnostic_row_payload_keys(document, "invalid_quire_context");
        assert!(
            keys.contains("view") && keys.contains("form"),
            "{label} `invalid_quire_context` row names `view` and `form`"
        );
    }
}

/// The `stage_limit_exceeded` and `duplicate_selection` cause sets are closed:
/// the native-diagnostics row and the FR-272 row of each code list exactly
/// these causes, so neither document can add, drop or rename a cause without
/// the other and this list. No `stage_limit_exceeded` cause names a depth, and
/// both of its rows name the `setting` that raises the limit.
#[trace("TC-233", "FR-272-AC-10")]
#[test]
fn tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed() {
    let expected: [(&str, &[&str]); 2] = [
        (
            "stage_limit_exceeded",
            &[
                "input-bytes-exceeded",
                "token-count-exceeded",
                "node-count-exceeded",
                "edge-count-exceeded",
                "occurrence-count-exceeded",
                "diagnostic-count-exceeded",
                "work-budget-exceeded",
                "rational-bits-exceeded",
            ],
        ),
        ("duplicate_selection", &["duplicate-identity"]),
    ];
    for (code, causes) in expected {
        let causes = causes
            .iter()
            .map(|cause| (*cause).to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            diagnostic_row_causes(NATIVE_DIAGNOSTICS, code),
            causes,
            "native-diagnostics `{code}` row lists exactly its causes"
        );
        assert_eq!(
            diagnostic_row_causes(TYPED_DIAGNOSTIC_CAUSE, code),
            causes,
            "FR-272 `{code}` row lists exactly its causes"
        );
    }
    for (document, label) in [
        (NATIVE_DIAGNOSTICS, "native-diagnostics"),
        (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
    ] {
        assert!(
            ["kind", "bound", "actual", "setting"]
                .iter()
                .all(
                    |key| diagnostic_row_payload_keys(document, "stage_limit_exceeded")
                        .contains(*key)
                ),
            "{label} `stage_limit_exceeded` row names `kind`, `bound`, `actual` and `setting`"
        );
    }
}

/// The `invalid_result` cause set is closed: the native-diagnostics row, the
/// FR-272 row and FR-331's "Result entries" table list exactly these causes,
/// and each catalog row names the pointer and the expected and received
/// certification it retains (FR-272-AC-23).
#[trace("TC-233", "FR-272-AC-23")]
#[test]
fn tc_233_invalid_result_cause_set_is_closed() {
    let causes = [
        "measured-with-truth",
        "missing-member",
        "unknown-member",
        "wrong-value-kind",
        "invalid-value",
        "certification-mismatch",
        "unknown-cause",
    ]
    .iter()
    .map(|cause| (*cause).to_owned())
    .collect::<BTreeSet<_>>();
    for (document, label) in [
        (NATIVE_DIAGNOSTICS, "native-diagnostics"),
        (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
    ] {
        assert_eq!(
            diagnostic_row_causes(document, "invalid_result"),
            causes,
            "{label} `invalid_result` row lists exactly its causes"
        );
        let keys = diagnostic_row_payload_keys(document, "invalid_result");
        for key in ["document_pointer", "expected", "actual"] {
            assert!(
                keys.contains(key),
                "{label} `invalid_result` row names `{key}`"
            );
        }
    }
    assert_eq!(
        result_entry_refusal_causes(),
        causes,
        "FR-331 \"Result entries\" table lists exactly the invalid_result causes"
    );
}

/// The first cell of each row of FR-331's "Result entries" refusal table.
fn result_entry_refusal_causes() -> BTreeSet<String> {
    let header = "| Cause | Defect | Pointer and payload |";
    let table = &BACKEND_PROVIDER_ENVELOPE[BACKEND_PROVIDER_ENVELOPE
        .find(header)
        .expect("FR-331 has its result-entry refusal table")
        + header.len()..];
    table
        .lines()
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .flat_map(|line| backticked(line.split('|').nth(1).unwrap_or_default()))
        .collect()
}

const BACKEND_PROVIDER_ENVELOPE: &str =
    include_str!("../spec/objects/interfaces/FR-331-backend-provider-envelope.md");

/// The `resource_exhausted`, `cancelled` and `conflicting_declaration` rows of
/// native-diagnostics and FR-272 list exactly the same closed causes, and each
/// cause names exactly its payload keys in both, every key a canonical FR-272
/// payload name (FR-272-AC-16, FR-272-AC-17).
#[trace("TC-233", "FR-272-AC-16", "FR-272-AC-17")]
#[test]
fn tc_233_resource_exhausted_and_cancelled_rows_agree() {
    type CauseKeys = (&'static str, &'static [&'static str]);
    let expected: [(&str, &[CauseKeys]); 3] = [
        (
            "conflicting_declaration",
            &[("duplicate-declaration", &["subject", "related"])],
        ),
        (
            "resource_exhausted",
            &[
                (
                    "insufficient-next-charge",
                    &[
                        "stage",
                        "accounting_contract",
                        "limit_kind",
                        "limit",
                        "consumed",
                        "next_charge",
                        "charge_point",
                    ],
                ),
                ("allocation-failed", &["requested"]),
                ("reservation-size-overflow", &["upstream"]),
                (
                    "intake-limit-exceeded",
                    &["kind", "bound", "actual", "setting"],
                ),
                (
                    "input-bytes-exceeded",
                    &["kind", "bound", "actual", "setting"],
                ),
            ],
        ),
        (
            "cancelled",
            &[(
                "caller-cancelled",
                &[
                    "stage",
                    "accounting_contract",
                    "consumed",
                    "charge_point",
                    "related",
                ],
            )],
        ),
    ];
    diagnostic_row(DIAGNOSTIC_CODE_VOCABULARY, "conflicting_declaration");
    let canonical = canonical_payload_names();
    for (code, per_cause) in expected {
        let names = per_cause.iter().map(|(c, _)| *c).collect::<Vec<_>>();
        let causes = names
            .iter()
            .map(|c| (*c).to_owned())
            .collect::<BTreeSet<_>>();
        for (document, label) in [
            (NATIVE_DIAGNOSTICS, "native-diagnostics"),
            (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
        ] {
            assert_eq!(
                diagnostic_row_causes(document, code),
                causes,
                "{label} `{code}` row lists exactly its causes"
            );
            for (cause, keys) in per_cause {
                let keys = keys
                    .iter()
                    .map(|k| (*k).to_owned())
                    .collect::<BTreeSet<_>>();
                assert_eq!(
                    diagnostic_row_cause_payload_keys(document, code, cause, &names),
                    keys,
                    "{label} `{code}` row names exactly the payload keys of `{cause}`"
                );
                assert!(
                    keys.is_subset(&canonical),
                    "`{cause}` payload keys are canonical FR-272 payload names"
                );
            }
        }
    }
}

/// The `runtime_invariant` rows of native-diagnostics and FR-272 list exactly
/// `established-invariant-broken` and name `kind`, `bound` and `required` for
/// the `identity-byte-bound-reached` invariant (FR-272-AC-18); the `stage`
/// rule names a label per limits type in the `resource_exhausted` and
/// `cancelled` rows and in value-accounting (FR-272-AC-19).
#[trace("TC-233", "FR-272-AC-18", "FR-272-AC-19")]
#[test]
fn tc_233_runtime_invariant_payload_and_stage_rule() {
    let canonical = canonical_payload_names();
    for (document, label) in [
        (NATIVE_DIAGNOSTICS, "native-diagnostics"),
        (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
    ] {
        assert_eq!(
            diagnostic_row_causes(document, "runtime_invariant"),
            BTreeSet::from(["established-invariant-broken".to_owned()]),
            "{label} `runtime_invariant` row lists exactly its cause"
        );
        let keys = diagnostic_row_payload_keys(document, "runtime_invariant");
        let expected = ["kind", "bound", "required"]
            .iter()
            .map(|k| (*k).to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            keys, expected,
            "{label} row names exactly the identity payload keys"
        );
        assert!(
            keys.is_subset(&canonical),
            "the keys are canonical payload names"
        );
        let invariant_text = diagnostic_row_payload_text(document, "runtime_invariant");
        for kind in ["`canonical-bytes`", "`input-bytes`", "`object-bytes`"] {
            assert!(
                invariant_text.contains(kind),
                "{label} lists the encoder limit kind {kind}"
            );
        }
        assert!(
            diagnostic_row_payload_text(document, "runtime_invariant")
                .contains("`identity-byte-bound-reached`"),
            "{label} ties the keys to the `identity-byte-bound-reached` invariant"
        );
        for code in ["resource_exhausted", "cancelled"] {
            let payload = diagnostic_row_payload_text(document, code);
            assert!(
                payload.contains("limits type assigns"),
                "{label} `{code}` row takes `stage` from the limits type"
            );
        }
        let payload = diagnostic_row_payload_text(document, "resource_exhausted");
        for label_value in ["`model-normalization`", "`population-admission`"] {
            assert!(payload.contains(label_value), "{label} names {label_value}");
        }
    }
    let accounting = one_line(VALUE_ACCOUNTING);
    for label_value in ["`model-normalization`", "`population-admission`"] {
        assert!(
            accounting.contains(&format!("{label_value} for every"))
                || accounting.contains(&format!("and {label_value} for every")),
            "value-accounting assigns {label_value} to a limits type"
        );
    }
}

/// The eleven value-refusal and source-identity codes with their closed causes
/// and payload keys (FR-272-AC-11).
const VALUE_REFUSAL_ROWS: [(&str, &[&str], &[&str]); 11] = [
    (
        "invalid_source_identity",
        &["blank-label", "empty-path"],
        &["label"],
    ),
    (
        "inexact_decimal",
        &["nonzero-discarded-digit"],
        &["expected"],
    ),
    ("decimal_out_of_domain", &["outside-domain"], &["expected"]),
    (
        "division_out_of_domain",
        &["quotient-outside-domain", "remainder-outside-domain"],
        &["expected"],
    ),
    ("modulo_out_of_domain", &["outside-domain"], &["expected"]),
    (
        "text_length_out_of_domain",
        &["outside-domain"],
        &["expected"],
    ),
    ("integer_out_of_domain", &["outside-domain"], &["expected"]),
    ("rational_out_of_domain", &["outside-domain"], &["expected"]),
    (
        "ieee_not_exact",
        &["rounding-required"],
        &["expected", "flags"],
    ),
    (
        "ieee_nan_payload_not_representable",
        &["payload-exceeds-target"],
        &["expected", "actual"],
    ),
    (
        "ieee_rational_out_of_domain",
        &["outside-domain"],
        &["expected"],
    ),
];

fn owned_set(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

/// The eleven value-refusal and source-identity cause sets and payload keys are
/// closed: the native-diagnostics row and the FR-272 row of each code list
/// exactly these causes and name exactly these payload keys, and every key is
/// one of FR-272's canonical payload names, so neither document can add, drop
/// or rename a cause or key without the other and this list.
#[trace("TC-233", "FR-272-AC-11")]
#[test]
fn tc_233_revision_1_draft_8_cause_sets_and_payload_keys_are_closed() {
    let canonical = canonical_payload_names();
    for (code, causes, keys) in VALUE_REFUSAL_ROWS {
        let causes = owned_set(causes);
        let keys = owned_set(keys);
        assert!(
            keys.is_subset(&canonical),
            "every `{code}` payload key is an FR-272 canonical payload name"
        );
        for (document, name) in [
            (NATIVE_DIAGNOSTICS, "native-diagnostics"),
            (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
        ] {
            assert_eq!(
                diagnostic_row_causes(document, code),
                causes,
                "{name} `{code}` row lists exactly its causes"
            );
            assert_eq!(
                diagnostic_row_payload_keys(document, code),
                keys,
                "{name} `{code}` row names exactly its payload keys"
            );
        }
    }
}

const DIAGNOSTIC_CODE_VOCABULARY: &str =
    include_str!("../spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md");

/// FR-271's values table has exactly one row for each code FR-271-AC-6 names.
#[trace("TC-233", "FR-271-AC-6")]
#[test]
fn tc_233_code_vocabulary_follows_the_catalog() {
    for (code, _, _) in VALUE_REFUSAL_ROWS {
        diagnostic_row(DIAGNOSTIC_CODE_VOCABULARY, code);
    }
}

/// The FR-322 reader codes that FR-271-AC-7 and FR-272-AC-22 catalogue.
const READER_CODES: [&str; 8] = [
    "unknown_contract_version",
    "malformed_wire",
    "duplicate_member",
    "unknown_member",
    "digest_domain_mismatch",
    "unknown_required_capability",
    "invalid_semantic_graph",
    "unsupported_node_tag",
];

/// FR-271-AC-7: the values table has exactly one row for each reader code.
#[trace("TC-233", "FR-271-AC-7")]
#[test]
fn tc_233_reader_codes_have_one_vocabulary_row_each() {
    for code in READER_CODES {
        diagnostic_row(DIAGNOSTIC_CODE_VOCABULARY, code);
    }
}

/// FR-272-AC-22: the native-diagnostics row and the FR-272 row of each reader
/// code list no cause variant and name `locus` and `document_pointer`;
/// `unknown_contract_version` also names `contract_version`; `locus` and
/// `contract_version` are canonical payload names.
#[trace("TC-233", "FR-272-AC-22")]
#[test]
fn tc_233_reader_code_rows_list_no_cause_and_name_their_payload() {
    let payload_names = canonical_payload_names();
    assert!(payload_names.contains("locus"), "`locus` is canonical");
    assert!(
        payload_names.contains("contract_version"),
        "`contract_version` is canonical"
    );
    for code in READER_CODES {
        for (document, name) in [
            (NATIVE_DIAGNOSTICS, "native-diagnostics"),
            (TYPED_DIAGNOSTIC_CAUSE, "FR-272"),
        ] {
            assert!(
                diagnostic_row_causes(document, code).is_empty(),
                "{name} `{code}` row lists no cause variant"
            );
            let row = diagnostic_row(document, code);
            assert!(
                row.contains("`locus`") && row.contains("`document_pointer`"),
                "{name} `{code}` row names `locus` and `document_pointer`"
            );
            assert_eq!(
                row.contains("`contract_version`"),
                code == "unknown_contract_version",
                "{name} `{code}` row names `contract_version` only for unknown_contract_version"
            );
        }
    }
}

const EXACT_NUMERIC_DEFINEDNESS: &str =
    include_str!("../spec/functional/foundation/FR-044-check-composed-numeric-definedness.md");
const COLLECTION_QUERIES: &str =
    include_str!("../spec/functional/expressions/FR-145-convert-and-query-collections.md");
const IEEE_PROFILES: &str =
    include_str!("../spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md");
const VALUE_ACCOUNTING: &str =
    include_str!("../proposals/quire-v1/definitions/value-accounting.md");

/// FR-044-AC-10: FR-044, `quire.value.accounting/v1` and the catalog each
/// state the evaluation-time integer and `Rational[..]` result-domain
/// refusals and the `sum` carve-out, and FR-145 keeps the `sum` seed or
/// prefix a located undefined outcome, so one input has one outcome.
#[trace("TC-233", "FR-044-AC-10")]
#[test]
fn tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined() {
    let integer = "refused { code: integer_out_of_domain, cause: outside-domain }";
    let rational = "refused { code: rational_out_of_domain, cause: outside-domain }";
    for (document, name) in [
        (before_criteria(EXACT_NUMERIC_DEFINEDNESS), "FR-044"),
        (VALUE_ACCOUNTING, "value-accounting"),
    ] {
        let text = one_line(document);
        assert!(
            text.contains(integer) && text.contains(rational),
            "{name} states both evaluation-time result-domain refusals"
        );
        assert!(
            text.contains("other than a `sum` seed or prefix"),
            "{name} carves the `sum` seed or prefix out of the refusal"
        );
    }
    assert!(
        one_line(before_criteria(EXACT_NUMERIC_DEFINEDNESS))
            .contains("FR-145 makes that a located undefined outcome, not a refusal"),
        "FR-044 names FR-145's located undefined outcome for a `sum` prefix"
    );
    assert!(
        one_line(VALUE_ACCOUNTING).contains(
            "ends the sum with FR-145's located undefined outcome at that summand or addition, in place of the family's membership refusal"
        ),
        "the sum schedule replaces the family's membership refusal with FR-145's outcome"
    );
    assert!(
        one_line(NATIVE_DIAGNOSTICS).contains(
            "A `sum` seed or running total outside `N`'s domain is none of these refusals: FR-145 makes it a located undefined outcome"
        ),
        "the catalog excludes a `sum` running total from its value refusals"
    );
    assert!(
        one_line(before_criteria(COLLECTION_QUERIES))
            .contains("a non-member seed or prefix is a located undefined outcome"),
        "FR-145 keeps a non-member `sum` seed or prefix undefined"
    );
}

const EXACT_DECIMALS: &str =
    include_str!("../spec/functional/type-model/FR-140-evaluate-exact-decimals.md");

/// FR-140-AC-5: the decimal family and the quantity family's decimal and
/// integer targets carry the same `sum` carve-out as FR-044: FR-140's
/// normative text and each membership sentence of `quire.value.accounting/v1`
/// leave a `sum` seed or running total to FR-145's located undefined outcome,
/// so one input has one outcome in every family.
#[trace("TC-233", "FR-140-AC-5")]
#[test]
fn tc_233_decimal_and_unit_domain_refusals_leave_sum_prefixes_undefined() {
    let decimals = one_line(before_criteria(EXACT_DECIMALS));
    assert!(
        decimals.contains(
            "a nonmember, other than a `sum` seed or running total, refuses as `refused { code: decimal_out_of_domain, cause: outside-domain }`"
        ),
        "FR-140 carves the `sum` seed or running total out of `decimal_out_of_domain`"
    );
    assert!(
        decimals.contains("FR-145 makes that a located undefined outcome, not a refusal"),
        "FR-140 names FR-145's located undefined outcome for a `sum` running total"
    );
    let accounting = one_line(VALUE_ACCOUNTING);
    for (sentence, family) in [
        (
            "FR-140 membership, which charges nothing, refuses, other than a `sum` seed or running total,",
            "a quantity's decimal target",
        ),
        (
            "refuses (`integer_out_of_domain`, `outside-domain`), other than a `sum` seed or running total,",
            "a quantity's integer target",
        ),
        (
            "A nonmember result, other than a `sum` seed or running total, is refused after `decimal.rounding`",
            "decimal arithmetic",
        ),
    ] {
        assert!(
            accounting.contains(sentence),
            "value-accounting carves the `sum` running total out of {family}'s membership refusal"
        );
    }
}

/// The two source labels in their catalog order.
const SOURCE_LABELS: [&str; 2] = ["authority", "identity"];

/// FR-272-AC-12: both `invalid_source_identity` rows name the two labels in
/// order, decide `blank-label` at the first blank label, and report
/// `empty-path` only when every label is non-blank.
#[trace("TC-233", "FR-272-AC-12")]
#[test]
fn tc_233_invalid_source_identity_checks_labels_before_the_path() {
    for (document, name, first_blank, path_last) in [
        (
            NATIVE_DIAGNOSTICS,
            "native-diagnostics",
            "the first blank label decides `blank-label`",
            "`empty-path` is reported only when every label is non-blank",
        ),
        (
            TYPED_DIAGNOSTIC_CAUSE,
            "FR-272",
            "the first blank label wins",
            "`empty-path` only when every label is non-blank",
        ),
    ] {
        let row = diagnostic_row(document, "invalid_source_identity");
        let labels = row
            .split('`')
            .skip(1)
            .step_by(2)
            .filter(|name| SOURCE_LABELS.contains(name))
            .collect::<Vec<_>>();
        assert_eq!(
            labels, SOURCE_LABELS,
            "{name} names the two labels once each, in order"
        );
        assert!(
            row.contains("The label check runs first"),
            "{name} checks the labels before the path"
        );
        assert!(
            row.contains(first_blank),
            "{name}: the first blank label decides"
        );
        assert!(row.contains(path_last), "{name}: `empty-path` comes last");
    }
}

/// FR-272-AC-13: the catalog's payload value spellings: the four domain
/// forms, the two widths, the flag order (FR-148's own order) and the two
/// labels (the `invalid_source_identity` row's order).
#[trace("TC-233", "FR-272-AC-13")]
#[test]
fn tc_233_payload_values_have_one_spelling() {
    let catalog = one_line(NATIVE_DIAGNOSTICS);
    let intro = "Each payload value is written as one exact ASCII string:";
    let start = catalog
        .find(intro)
        .expect("the catalog states its payload value spellings")
        + intro.len();
    let rules = &catalog[start..];
    let rules = &rules[..rules
        .find("Other retained host/source codes")
        .expect("the spelling list ends before the host/source code paragraph")];
    let item = |lead: &str| -> String {
        let begin = rules
            .find(lead)
            .unwrap_or_else(|| panic!("the spelling list has a `{lead}` item"));
        let rest = &rules[begin..];
        rest[..rest.find(";").unwrap_or(rest.len())].to_owned()
    };
    let domain = rules
        .find("- a domain is")
        .map(|begin| &rules[begin..])
        .and_then(|rest| rest.find("- an IEEE width").map(|end| &rest[..end]))
        .expect("the spelling list has a domain item");
    for form in [
        "`Int[lo, hi]`",
        "`Decimal[lo, hi; smin, smax]`",
        "`Rational[lo, hi; dmin, dmax]`",
        "`Text[min, max; profile]`",
    ] {
        assert!(domain.contains(form), "the domain item writes {form}");
    }
    for rule in [
        "each bound is written as FR-038 writes an integer",
        "bounds within a group are separated by `, `, groups by `; `",
        "no rounding mode is written",
    ] {
        assert!(domain.contains(rule), "the domain item states: {rule}");
    }
    assert_eq!(
        backticked(&item("- an IEEE width is")),
        owned_set(&["binary32", "binary64"]),
        "an IEEE width is exactly `binary32` or `binary64`"
    );
    let ieee = one_line(IEEE_PROFILES);
    let drawn = "Each operation returns a fresh closed flag set drawn from";
    let flags_start = ieee.find(drawn).expect("FR-148 names its flag set") + drawn.len();
    let ieee_flags = &ieee[flags_start..];
    let ieee_flags = &ieee_flags[..ieee_flags.find(';').expect("FR-148's flag list ends")];
    let ordered = |text: &str| -> Vec<String> {
        text.split('`')
            .skip(1)
            .step_by(2)
            .map(str::to_owned)
            .collect()
    };
    let flag_item = rules
        .find("- a flag set is")
        .map(|begin| &rules[begin..])
        .and_then(|rest| rest.find("- a label is").map(|end| &rest[..end]))
        .expect("the spelling list has a flag-set item");
    let flag_order =
        ordered(&flag_item[..flag_item.find("joined by").expect("the flag item joins")]);
    assert_eq!(
        flag_order,
        ordered(ieee_flags),
        "the flag order is FR-148's flag order"
    );
    assert!(
        flag_item.contains("joined by `,` with no space"),
        "flags are joined by `,` with no space"
    );
    assert_eq!(
        ordered(&item("- a label is")),
        SOURCE_LABELS,
        "a label is exactly one of the two labels, in the row's order"
    );
}

/// Index of the node whose digest is `digest` in a published package.
fn node_index(package: &Value, digest: &str) -> usize {
    node_index_by_digest(package, digest).unwrap_or_else(|| panic!("{digest} is published"))
}

/// The clause fixture's `state`/`state_clause` node of `clause` kind.
fn clause_node_index(package: &Value, clause: &str) -> usize {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| {
            node["semantic_form"] == "state_clause"
                && node["body"]["operation"]["member"]["clause"] == clause
        })
        .unwrap_or_else(|| panic!("the clause fixture carries a {clause} clause"))
}

/// Replace one clause node's body through `edit`, rekeying the node, its
/// dependencies and package id as a producer would.
fn edited_clause(package: &Value, clause: &str, edit: impl FnOnce(&mut Value)) -> Value {
    let mut candidate = package.clone();
    let index = clause_node_index(&candidate, clause);
    let mut node = candidate["semantic_graph"]["nodes"][index].clone();
    edit(&mut node["body"]);
    replace_application_node(&mut candidate, index, node);
    candidate
}

/// FR-341-AC-1/AC-2, FR-341-AC-8, FR-342-AC-1, FR-340-AC-12/AC-14/AC-15:
/// the node-form body rules the published schema carries. A
/// `state`/`state_clause` body is a `quire.op.state.clause` application
/// over a parameter aggregate of references, an anchor reference and a
/// condition, carrying the `state_clause` member, and its occurrences are
/// `claim`; an `operation_anchor` body is the three bindings in order and
/// its occurrences are `anchor`; a `transition` body is a
/// `quire.op.state.transition` application; a frame's occurrences are
/// `generated`. `ModelNode.semantic_form` is the closed set FR-340-AC-15
/// names, so no `model` node names a member declaration form.
#[trace("TC-233", "FR-341-AC-1")]
#[trace("TC-233", "FR-341-AC-2")]
#[trace("TC-233", "FR-341-AC-8")]
#[trace("TC-233", "FR-342-AC-1")]
#[trace("TC-233", "FR-340-AC-12")]
#[trace("TC-233", "FR-340-AC-14")]
#[trace("TC-233", "FR-340-AC-15")]
#[trace("TC-233", "FR-341-AC-10")]
#[test]
fn tc_233_state_node_forms_fix_their_body_roots_and_occurrence_roles() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    schema
        .validate(&package)
        .unwrap_or_else(|error| panic!("the clause package has the closed V2 shape: {error}"));
    let invariant = clause_node_index(&package, "invariant");
    let anchor = node_index(&package, &clause_digest("anchor"));
    let frame = node_index(&package, &clause_digest("frame"));
    let refused = |pointer: &str, value: Value, why: &str| {
        let mut candidate = package.clone();
        *candidate
            .pointer_mut(pointer)
            .unwrap_or_else(|| panic!("{pointer} exists")) = value;
        assert!(!schema.is_valid(&candidate), "{why}");
    };
    let node = |index: usize| format!("/semantic_graph/nodes/{index}");
    let temporal = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .find(|node| node["semantic_form"] == "temporal_clause")
        .expect("a temporal clause is published")["body"]
        .clone();

    refused(
        &format!("{}/body", node(invariant)),
        temporal.clone(),
        "a state_clause body root is a state.clause application, never another clause",
    );
    refused(
        &format!("{}/body/operation/identity", node(invariant)),
        json!("quire.op.state.transition"),
        "a state_clause body names quire.op.state.clause",
    );
    refused(
        &format!("{}/body/operator", node(invariant)),
        json!("state_transition"),
        "a state_clause body has the state_clause operator class",
    );
    refused(
        &format!("{}/body/operation/member", node(invariant)),
        json!({"kind": "profile_operator", "operator": "always"}),
        "a state_clause body carries the state_clause member",
    );
    refused(
        &format!("{}/body/operation/member/clause", node(invariant)),
        json!("transition"),
        "a clause kind is invariant, precondition or postcondition",
    );
    refused(
        &format!("{}/body/arguments/0", node(invariant)),
        json!({"term": "reference", "target": node_ref_digest(&clause_digest("parameter"))}),
        "argument 0 is the parameter aggregate",
    );
    refused(
        &format!("{}/body/arguments/0/members", node(invariant)),
        json!([]),
        "the parameter aggregate holds self at least",
    );
    refused(
        &format!("{}/body/arguments/1", node(invariant)),
        json!({"term": "aggregate", "members": []}),
        "argument 1 is the anchor reference",
    );
    refused(
        &format!("{}/occurrences/0/role", node(invariant)),
        json!("declaration"),
        "a state_clause occurrence is a claim",
    );
    refused(
        &format!("{}/body/members/0/name", node(anchor)),
        json!("frame"),
        "an anchor's first binding is context",
    );
    refused(
        &format!("{}/body/members/1/value/value_kind", node(anchor)),
        json!("integer"),
        "an anchor's operation is a text literal",
    );
    refused(
        &format!("{}/body/members/2", node(anchor)),
        Value::Null,
        "an anchor carries exactly three bindings",
    );
    refused(
        &format!("{}/occurrences/0/role", node(anchor)),
        json!("claim"),
        "an operation_anchor occurrence is an anchor",
    );
    refused(
        &format!("{}/occurrences/0/role", node(frame)),
        json!("declaration"),
        "a frame occurrence is generated",
    );
    let parameter = node_index(&package, &clause_digest("parameter"));
    refused(
        &format!("{}/body/members/0/name", node(parameter)),
        json!("level"),
        "a parameter's first binding is name",
    );
    refused(
        &format!("{}/body/members/0/value/value_kind", node(parameter)),
        json!("integer"),
        "a parameter's name is a text literal",
    );
    refused(
        &format!("{}/body/members/1/value/value_kind", node(parameter)),
        json!("text"),
        "a parameter's level is an integer literal",
    );
    refused(
        &format!("{}/body/members", node(parameter)),
        json!([
            {"term": "binding", "name": "name", "value": {"term": "literal", "value_kind": "text", "value": "self"}},
            {"term": "binding", "name": "level", "value": {"term": "literal", "value_kind": "integer", "value": "0"}},
            {"term": "binding", "name": "extra", "value": {"term": "literal", "value_kind": "integer", "value": "1"}}
        ]),
        "a parameter carries exactly two bindings",
    );
    refused(
        &format!("{}/occurrences/0/role", node(parameter)),
        json!("declaration"),
        "a parameter occurrence is expression",
    );
    let transition_package = parsed_fixture(POSITIVE_STATE_TRANSITION);
    schema
        .validate(&transition_package)
        .unwrap_or_else(|error| panic!("the transition package has the closed V2 shape: {error}"));
    let transition = transition_package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["semantic_form"] == "transition")
        .expect("the transition package carries a transition");
    let mut candidate = transition_package.clone();
    *candidate
        .pointer_mut(&format!("{}/body", node(transition)))
        .expect("transition body exists") = temporal;
    assert!(
        !schema.is_valid(&candidate),
        "a transition body root is a state.transition application"
    );
    refused(
        &format!(
            "{}/semantic_form",
            node(node_index(&package, &clause_digest("object_type")))
        ),
        json!("field_declaration"),
        "no model node is a field declaration",
    );
    refused(
        &format!(
            "{}/semantic_form",
            node(node_index(&package, &clause_digest("object_type")))
        ),
        json!("operation_declaration"),
        "no model node is an operation declaration",
    );
    refused(
        &format!(
            "{}/semantic_form",
            node(node_index(&package, &clause_digest("object_type")))
        ),
        json!("clause_member_declaration"),
        "no model node is a clause member declaration",
    );
}

/// FR-341-AC-3/AC-4/AC-6/AC-7 and FR-342-AC-2/AC-3/AC-5 through the full
/// reader: the published clause package's anchor and clauses have no
/// defect, and each authored defect refuses at the state step with its
/// code, cause and locus, in the stated order.
#[trace("TC-233", "FR-341-AC-3")]
#[trace("TC-233", "FR-341-AC-4")]
#[trace("TC-233", "FR-341-AC-6")]
#[trace("TC-233", "FR-341-AC-7")]
#[trace("TC-233", "FR-342-AC-2")]
#[trace("TC-233", "FR-342-AC-3")]
#[trace("TC-233", "FR-342-AC-5")]
#[test]
fn tc_233_state_step_joins_anchors_then_clauses_with_their_recorded_cause() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert_eq!(package_state_refusal(&package), Ok(()));
    let reference =
        |b: &str| json!({"term": "reference", "target": node_ref_digest(&clause_digest(b))});
    let expect = |candidate: &Value, refusal: StateRefusal, locus: Value, why: &str| {
        // Edited nodes are rekeyed so the read reaches the state step.
        let mut candidate = candidate.clone();
        let renames = rekey_stale_nodes(&mut candidate);
        let locus = renamed_locus(&renames, &locus);
        match read_package_semantics(&candidate, &catalog) {
            Err(ReadRefusal::State(actual, actual_locus)) => {
                assert_eq!(actual, refusal, "{why}: refusal");
                assert_eq!(actual_locus, locus, "{why}: locus");
            }
            other => panic!("{why}: expected a state refusal, got {other:?}"),
        }
    };
    let target = |b: &str| node_ref_digest(&clause_digest(b));

    // Clause anchors (FR-341-AC-3).
    let wrong = edited_clause(&package, "invariant", |body| {
        body["arguments"][1] = reference("anchor")
    });
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("anchor"),
        "an invariant anchors at an object type",
    );
    let wrong = edited_clause(&package, "precondition", |body| {
        body["arguments"][1] = reference("object_type")
    });
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("object_type"),
        "a precondition anchors at an operation_anchor",
    );
    let mut undeclared = package.clone();
    let index = clause_node_index(&undeclared, "invariant");
    undeclared["semantic_graph"]["nodes"][index]["dependencies"]
        .as_array_mut()
        .expect("dependencies")
        .retain(|dep| dep["digest"] != clause_digest("object_type"));
    undeclared["identity_preimage"] = derive_identity_preimage(&undeclared);
    assert!(
        read_package_semantics(&undeclared, &catalog)
            .is_err_and(|refusal| matches!(refusal, ReadRefusal::GraphShape(_))),
        "a body reference missing from dependencies is a graph-shape refusal first"
    );

    // Parameters (FR-341-AC-4).
    let wrong = edited_clause(&package, "invariant", |body| {
        body["arguments"][0]["members"][0] = reference("boolean_literal")
    });
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("boolean_literal"),
        "a parameter reference names a value/parameter node",
    );
    let wrong = edited_clause(&package, "invariant", |body| {
        body["arguments"][0]["members"]
            .as_array_mut()
            .expect("members")
            .push(reference("parameter"));
    });
    let clause_id = |candidate: &Value| {
        candidate["semantic_graph"]["nodes"][clause_node_index(candidate, "invariant")]["node_id"]
            .clone()
    };
    expect(
        &wrong,
        StateRefusal::OperatorIneligible,
        clause_id(&wrong),
        "an invariant binds self alone",
    );
    let mut retyped = package.clone();
    let self_index = node_index(&retyped, &clause_digest("parameter"));
    retyped["semantic_graph"]["nodes"][self_index]["semantic_type"] = target("boolean");
    retyped["identity_preimage"] = derive_identity_preimage(&retyped);
    expect(
        &retyped,
        StateRefusal::OperatorIneligible,
        clause_id(&retyped),
        "self has type Reference<context>",
    );

    // Misplaced clause application (FR-341-AC-6): a state.clause
    // application at the body root of a node other than state_clause
    // refuses at that node; one nested inside another clause's condition
    // is outside the body grammar and is a malformed wire.
    let clause_body = package["semantic_graph"]["nodes"][clause_node_index(&package, "invariant")]
        ["body"]
        .clone();
    let mut wrong = package.clone();
    let mut function =
        wrong["semantic_graph"]["nodes"][clause_node_index(&package, "invariant")].clone();
    function["node_tag"] = json!("function");
    function["semantic_form"] = json!("function");
    function["body"] = clause_body.clone();
    let index = wrong["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .len();
    wrong["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes")
        .push(function.clone());
    replace_application_node(&mut wrong, index, function);
    let misplaced_id = wrong["semantic_graph"]["nodes"][index]["node_id"].clone();
    expect(
        &wrong,
        StateRefusal::OperatorIneligible,
        misplaced_id,
        "a state.clause application at a function's body root refuses at that node",
    );
    let nested = edited_clause(&package, "precondition", |body| {
        body["arguments"][2] = clause_body
    });
    let precondition = clause_node_index(&nested, "precondition");
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    assert!(
        !schema.is_valid(&nested),
        "a nested state.clause application is outside the body grammar"
    );
    assert_eq!(
        case_placement_refusal(&nested),
        Err(TemporalRefusal::MalformedWire(format!(
            "/semantic_graph/nodes/{precondition}/body/arguments/2"
        ))),
        "a nested state.clause application is a malformed wire, not operator-ineligible"
    );

    // Anchors (FR-342-AC-2/AC-3) and the anchors-before-clauses order (FR-342-AC-5).
    let anchor = node_index(&package, &clause_digest("anchor"));
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][anchor]["body"]["members"][0]["value"] = reference("frame");
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("frame"),
        "an anchor's context is an object type",
    );
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][anchor]["body"]["members"][2]["value"] =
        reference("object_type");
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("object_type"),
        "an anchor's frame is a frame node",
    );
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][anchor]["dependencies"] = json!([target("object_type")]);
    expect(
        &wrong,
        StateRefusal::MissingName,
        target("frame"),
        "an anchor's frame is a declared dependency",
    );
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][anchor]["semantic_type"] = target("reference");
    expect(
        &wrong,
        StateRefusal::MalformedDeclaration,
        target("anchor"),
        "an anchor's semantic type is its context",
    );
    let frame = node_index(&package, &clause_digest("frame"));
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][frame]["semantic_type"] = target("reference");
    rekey_stale_nodes(&mut wrong);
    assert!(
        matches!(
            read_package_semantics(&wrong, &catalog),
            Err(ReadRefusal::Frame(..))
        ),
        "a frame whose semantic type is not an object type refuses at the frame step, before the anchor"
    );
    let mut both = edited_clause(&package, "invariant", |body| {
        body["arguments"][1] = reference("anchor")
    });
    let anchor = node_index(&both, &clause_digest("anchor"));
    both["semantic_graph"]["nodes"][anchor]["semantic_type"] = target("reference");
    expect(
        &both,
        StateRefusal::MalformedDeclaration,
        target("anchor"),
        "the anchor defect is reported before the clause defect",
    );

    // Every state refusal pairs with the code and cause FR-341 and FR-342 state.
    for (refusal, code, cause) in [
        (
            StateRefusal::MissingName,
            "missing_declaration",
            "missing-name",
        ),
        (
            StateRefusal::MalformedDeclaration,
            "invalid_model_binding",
            "malformed-declaration",
        ),
        (
            StateRefusal::OperatorIneligible,
            "ill_typed",
            "operator-ineligible",
        ),
    ] {
        assert_eq!((refusal.code(), refusal.cause()), (code, cause));
    }
}

/// FR-341-AC-13 (TC-233 CN-01 to CN-03): a state constraint is a
/// `state`/`state_clause` node whose kind member's `clause` is `constraint`;
/// it validates against the V2 schema, is anchored and bound as an
/// invariant, and differs in node id from the invariant with its body.
#[trace("TC-233", "FR-341-AC-13")]
#[test]
fn tc_233_a_state_constraint_is_checked_as_an_invariant() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let reference =
        |b: &str| json!({"term": "reference", "target": node_ref_digest(&clause_digest(b))});
    let to_constraint = |body: &mut Value| {
        body["operation"]["member"]["clause"] = json!("constraint");
    };
    let read = |candidate: &Value| {
        let mut candidate = candidate.clone();
        let renames = rekey_stale_nodes(&mut candidate);
        (
            read_package_semantics(&candidate, &catalog),
            renames,
            candidate,
        )
    };

    // CN-01: admitted, with another node id than the invariant's.
    let constrained = edited_clause(&package, "invariant", to_constraint);
    let (admitted, _, rekeyed) = read(&constrained);
    assert!(
        schema.is_valid(&rekeyed),
        "a constraint clause validates against the V2 schema"
    );
    assert!(admitted.is_ok(), "a constraint is admitted: {admitted:?}");
    let invariant_id = package["semantic_graph"]["nodes"][clause_node_index(&package, "invariant")]
        ["node_id"]
        .clone();
    let constraint_id = rekeyed["semantic_graph"]["nodes"]
        [clause_node_index(&rekeyed, "constraint")]["node_id"]
        .clone();
    assert_ne!(
        invariant_id, constraint_id,
        "the kind member is part of the node's identity"
    );

    // CN-02: a constraint anchors at its type's node, never an operation anchor.
    let wrong = edited_clause(&package, "invariant", |body| {
        to_constraint(body);
        body["arguments"][1] = reference("anchor");
    });
    match read(&wrong) {
        (Err(ReadRefusal::State(refusal, locus)), renames, _) => {
            assert_eq!(refusal, StateRefusal::MalformedDeclaration);
            assert_eq!(
                locus,
                renamed_locus(&renames, &node_ref_digest(&clause_digest("anchor")))
            );
        }
        other => panic!(
            "a constraint anchored at an operation anchor refuses: {:?}",
            other.0
        ),
    }

    // CN-03: a constraint binds `self` alone.
    let wrong = edited_clause(&package, "invariant", |body| {
        to_constraint(body);
        body["arguments"][0]["members"]
            .as_array_mut()
            .expect("members")
            .push(reference("parameter"));
    });
    match read(&wrong) {
        (Err(ReadRefusal::State(refusal, locus)), _, rekeyed) => {
            assert_eq!(refusal, StateRefusal::OperatorIneligible);
            assert_eq!(
                locus,
                rekeyed["semantic_graph"]["nodes"][clause_node_index(&rekeyed, "constraint")]["node_id"]
            );
        }
        other => panic!(
            "a constraint binding a second parameter refuses: {:?}",
            other.0
        ),
    }
}
/// A minimal test-only domain package and model-declaration-node mapping,
/// keyed to the published clause package's own node digests (FR-322's
/// "Model-owned members" steps 2/3 shape, the same one TC-280's standalone
/// oracle reads). Not a domain package any published V2 fixture selects;
/// authored only so the full reader has a real model to resolve a frame
/// field entry's name (FR-340-AC-13) and an anchor's operation name and a
/// clause's signature (FR-342-AC-4, FR-341-AC-5) against, instead of the
/// declaring node's tag and form alone.
fn clause_model_bindings(operation_parameters: usize) -> Value {
    let parameters = (0..operation_parameters)
        .map(|i| {
            json!({
                "name": format!("p{i}"),
                "typeRef": "ix://quire/native/Integer",
                "multiplicity": {"lower": "1", "upper": "1", "ordered": false, "unique": true},
            })
        })
        .collect::<Vec<_>>();
    json!({
        "domain_package": {
            "identity": "test/model-bindings",
            "version": "1.0.0",
            "object_types": [{
                "node": "test/config-version",
                "interfaceFeatures": null,
                "supertypes": [],
                "fields": [{
                    "name": "versionNumber",
                    "typeRef": "ix://quire/native/Integer",
                    "presence": "required",
                    "multiplicity": {"lower": "1", "upper": "1", "ordered": false, "unique": true},
                    "redefines": null,
                }],
                "operations": [{
                    "name": "attemptUpdate",
                    "parameters": parameters,
                    "result": null,
                    "redefines": null,
                }],
            }],
            "value_types": [],
            "relationships": [],
        },
        "model_declaration_nodes": [{
            "name": "ConfigVersion",
            "preimage": {
                "owner": {
                    "node": "test/config-version",
                    "identity": "test/model-bindings",
                    "version": "1.0.0",
                },
            },
            "sha256": clause_digest("object_type"),
        }],
    })
}

/// FR-340-AC-13 and FR-342-AC-4/FR-341-AC-5 through the full reader (SR-725
/// FND-001/FND-002): with a real model attached, a frame field entry's name,
/// an anchor's operation name and a precondition's signature are resolved,
/// not just their declaring node's tag and form; without the model attached,
/// the very same defective packages are admitted, showing the coverage is
/// genuinely new (mutation evidence for the checks this test adds).
#[trace("TC-233", "FR-340-AC-13")]
#[trace("TC-233", "FR-342-AC-4")]
#[trace("TC-233", "FR-341-AC-5")]
#[test]
fn tc_233_the_full_reader_resolves_frame_anchor_and_clause_member_names() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let target = |b: &str| node_ref_digest(&clause_digest(b));

    let mut with_model = package.clone();
    with_model["model_bindings"] = clause_model_bindings(0);
    assert_eq!(
        read_package_semantics(&with_model, &catalog),
        Ok(()),
        "attaching a model that agrees with the published package changes nothing"
    );

    // FND-001: an undeclared frame field entry name.
    let frame_index = node_index(&with_model, &clause_digest("frame"));
    let mut bad_field = with_model.clone();
    bad_field["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][0]["name"] =
        json!("no_such_field");
    rekey_stale_nodes(&mut bad_field);
    assert_eq!(
        read_package_semantics(&bad_field, &catalog),
        Err(ReadRefusal::Frame(
            FrameRefusal::MissingName,
            target("object_type")
        )),
        "a frame field entry naming no field of its declaring object type refuses missing-name"
    );
    let mut bad_field_no_model = package.clone();
    let frame_index = node_index(&bad_field_no_model, &clause_digest("frame"));
    bad_field_no_model["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][0]["name"] =
        json!("no_such_field");
    rekey_stale_nodes(&mut bad_field_no_model);
    assert_eq!(
        read_package_semantics(&bad_field_no_model, &catalog),
        Ok(()),
        "the same entry is silently admitted without a model to resolve its name against"
    );

    // FND-002 (FR-342 step 4): an anchor naming a nonexistent operation.
    let anchor_index = node_index(&with_model, &clause_digest("anchor"));
    let mut bad_operation = with_model.clone();
    bad_operation["semantic_graph"]["nodes"][anchor_index]["body"]["members"][1]["value"]["value"] =
        json!("doesNotExist");
    let renames = rekey_stale_nodes(&mut bad_operation);
    assert_eq!(
        read_package_semantics(&bad_operation, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::MissingName,
            renamed_locus(&renames, &target("anchor"))
        )),
        "an anchor naming an operation the context does not declare refuses missing-name"
    );
    let mut bad_operation_no_model = package.clone();
    let anchor_index = node_index(&bad_operation_no_model, &clause_digest("anchor"));
    bad_operation_no_model["semantic_graph"]["nodes"][anchor_index]["body"]["members"][1]["value"]
        ["value"] = json!("doesNotExist");
    rekey_stale_nodes(&mut bad_operation_no_model);
    assert_eq!(
        read_package_semantics(&bad_operation_no_model, &catalog),
        Ok(()),
        "the same anchor is silently admitted without a model to resolve its operation against"
    );

    // FND-002 (FR-341 step 3): a precondition missing a parameter the
    // resolved operation declares.
    let mut missing_parameter = package.clone();
    missing_parameter["model_bindings"] = clause_model_bindings(1);
    let precondition_id = missing_parameter["semantic_graph"]["nodes"]
        [clause_node_index(&missing_parameter, "precondition")]["node_id"]
        .clone();
    assert_eq!(
        read_package_semantics(&missing_parameter, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::OperatorIneligible,
            precondition_id
        )),
        "a precondition binding fewer values than self plus the operation's own parameters \
         refuses operator-ineligible"
    );
}

/// FR-342-AC-6: a duplicate anchor for the same (declaring object type,
/// operation name) pair is refused; two anchors of one context whose declared
/// operations have equal frames reference one frame node and are admitted.
#[trace("TC-233", "FR-342-AC-6")]
#[test]
fn tc_233_duplicate_anchors_are_refused_and_a_shared_frame_is_admitted() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));

    let anchor_index = node_index(&package, &clause_digest("anchor"));
    let mut duplicated = package.clone();
    // The duplicate is the same content, so its current key is the anchor's.
    let duplicate_anchor = duplicated["semantic_graph"]["nodes"][anchor_index].clone();
    let anchor_id = duplicate_anchor["node_id"].clone();
    duplicated["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes")
        .push(duplicate_anchor);
    assert_eq!(
        read_package_semantics(&duplicated, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::AmbiguousDeclaration,
            anchor_id.clone()
        )),
        "a second anchor for the same (context, operation) pair is refused, not admitted"
    );

    // A second operation of the same context, declared by the model, with an
    // anchor that references the same frame node: admitted.
    let mut shared_frame = package.clone();
    let mut bindings = clause_model_bindings(0);
    let operations = bindings["domain_package"]["object_types"][0]["operations"]
        .as_array_mut()
        .expect("operations");
    let mut other = operations[0].clone();
    other["name"] = json!("otherAttempt");
    operations.push(other);
    shared_frame["model_bindings"] = bindings;
    let mut second_anchor = shared_frame["semantic_graph"]["nodes"][anchor_index].clone();
    second_anchor["node_id"] = node_ref_digest(&repeated_digest("b7"));
    second_anchor["body"]["members"][1]["value"]["value"] = json!("otherAttempt");
    shared_frame["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes")
        .push(second_anchor);
    rekey_stale_nodes(&mut shared_frame);
    assert_eq!(
        read_package_semantics(&shared_frame, &catalog),
        Ok(()),
        "two anchors of one context that reference one frame node are admitted"
    );
}

/// FR-341-AC-7's second half: the state step reports its defect before any
/// operation refusal (FR-322), not only before other state defects.
#[trace("TC-233", "FR-341-AC-7")]
#[test]
fn tc_233_state_step_refusal_precedes_an_operation_step_refusal() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);

    let if_index = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "expression" && node["semantic_form"] == "conditional")
        .expect("a conditional expression is published");
    let mut with_operation_defect = package.clone();
    let mut if_node = with_operation_defect["semantic_graph"]["nodes"][if_index].clone();
    if_node["body"]["operation"]["identity"] = json!("quire.op.control.iff");
    replace_application_node(&mut with_operation_defect, if_index, if_node);
    assert!(
        package_operations_refusal(&with_operation_defect, &catalog).is_err(),
        "an unknown operation identity is, on its own, an operation-step defect"
    );
    assert!(
        matches!(
            read_package_semantics(&with_operation_defect, &catalog),
            Err(ReadRefusal::Operation(_))
        ),
        "with no state defect, the operation defect is what the full reader reports"
    );

    let both = edited_clause(&with_operation_defect, "precondition", |body| {
        body["arguments"][1] =
            json!({"term": "reference", "target": node_ref_digest(&clause_digest("object_type"))});
    });
    assert_eq!(
        read_package_semantics(&both, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::MalformedDeclaration,
            node_ref_digest(&clause_digest("object_type"))
        )),
        "a package carrying both a state defect and an operation defect reports the state \
         defect, matching FR-341-AC-7's 'before any operation refusal'"
    );
}

/// FR-341-AC-9: the catalog entry of `quire.op.state.clause`, bound to both
/// schemas, and every published state clause admitted by the operation step.
#[trace("TC-233", "FR-341-AC-9")]
#[test]
fn tc_233_state_clause_is_a_catalogued_operation_the_schemas_enumerate() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let entry = catalog["operations"]
        .as_array()
        .expect("catalog operations are an array")
        .iter()
        .find(|entry| entry["identity"] == "quire.op.state.clause")
        .expect("quire.op.state.clause is catalogued");
    assert_eq!(entry["operator"], "state_clause");
    assert_eq!(entry["operands"], json!(["aggregate", "object", "boolean"]));
    assert_eq!(entry["rest"], Value::Null);
    assert_eq!(entry["result"], "clause");
    assert_eq!(entry["laws"], json!([]));
    assert_eq!(entry["mode"], Value::Null);
    assert_eq!(entry["member"], "state_clause");
    for schema in [parsed_fixture(SCHEMA), parsed_fixture(NODE_PREIMAGE_SCHEMA)] {
        assert!(
            string_set(&schema["$defs"]["OperationIdentity"]["enum"], "identities")
                .contains("quire.op.state.clause")
        );
    }
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));
}

#[trace("TC-279", "FR-322-AC-28")]
#[test]
fn tc_279_model_owner_preimages_carry_package_identity_and_node_only() {
    let schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-identity preimage schema is valid");
    let file = parsed_fixture(NODE_IDENTITY_VECTORS);
    let owned = file["vectors"]
        .as_array()
        .expect("vectors are an array")
        .iter()
        .filter(|vector| vector["preimage"]["owner"]["kind"] == "model")
        .collect::<Vec<_>>();
    assert!(
        !owned.is_empty(),
        "the vectors publish ModelOwner preimages"
    );
    for vector in owned {
        let name = vector["name"].as_str().expect("vector name is a string");
        let preimage = &vector["preimage"];
        assert_eq!(
            preimage["owner"]
                .as_object()
                .expect("owner is an object")
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["identity", "kind", "node"]),
            "{name} owner names the domain package identity and node only"
        );
        assert!(schema.is_valid(preimage), "{name} validates");
        assert_eq!(
            unqualified_jcs_sha256(preimage),
            vector["sha256"]
                .as_str()
                .expect("vector digest is a string"),
            "{name} recomputes its recorded identity"
        );

        let mut versioned = preimage.clone();
        versioned["owner"]["version"] = json!("1.0.0");
        assert!(
            !schema.is_valid(&versioned),
            "{name} with a version label is refused by the closed schema"
        );

        let mut other_package = preimage.clone();
        other_package["owner"]["identity"] = json!("acme/other");
        assert!(schema.is_valid(&other_package));
        assert_ne!(
            unqualified_jcs_sha256(&other_package),
            unqualified_jcs_sha256(preimage),
            "{name} identity depends on the owning domain package"
        );
    }
}

/// The node in `package` whose `node_tag` and `semantic_form` are given.
fn union_fixture_node<'a>(package: &'a Value, tag: &str, form: &str) -> &'a Value {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array")
        .iter()
        .find(|node| node["node_tag"] == tag && node["semantic_form"] == form)
        .unwrap_or_else(|| panic!("the union fixture carries a {tag}/{form} node"))
}

#[trace("TC-395", "FR-440-AC-1")]
#[trace("TC-395", "FR-440-AC-2")]
#[trace("TC-395", "FR-440-AC-3")]
#[test]
fn tc_395_union_nodes_fixture_is_a_closed_v2_package_with_member_ordered_arms() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    schema
        .validate(&package)
        .unwrap_or_else(|error| panic!("the union fixture is a closed V2 package: {error}"));
    assert_eq!(
        derive_identity_preimage(&package),
        package["identity_preimage"],
        "the union fixture's identity preimage is its selections and occurrence-free graph"
    );
    assert_eq!(
        unqualified_jcs_sha256(&package_id_preimage(&package)),
        package["package_id"]["digest"].as_str().unwrap_or_default(),
        "the union fixture's package id is recomputed from its preimage"
    );
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array");
    for node in nodes.iter().filter(|node| is_application_node(node)) {
        assert!(
            application_node_is_valid(node, nodes),
            "{} keys its application preimage and lists exactly its references",
            node["semantic_form"]
        );
    }

    let union = union_fixture_node(&package, "composite_type", "union");
    let members = union["body"]["members"]
        .as_array()
        .expect("union body members are an array");
    let shape = members
        .iter()
        .map(|binding| {
            (
                binding["name"].as_str().unwrap_or_default(),
                binding["value"]["members"].as_array().map_or(0, Vec::len),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(shape, [("Circle", 1), ("Rect", 2), ("Empty", 0)]);

    let case = union_fixture_node(&package, "expression", "case");
    let arms = case["body"]["arguments"]
        .as_array()
        .expect("case arguments are an array")
        .iter()
        .skip(1)
        .map(|arm| {
            (
                arm["name"].as_str().unwrap_or_default(),
                arm["value"]["members"][0]["members"]
                    .as_array()
                    .map_or(0, Vec::len),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        arms, shape,
        "a case's arms follow the union's member declaration order and payload arities"
    );
}

#[trace("TC-395")]
#[test]
fn tc_395_value_and_nullary_union_values_use_application_node_keys() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    let indexes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("semantic graph nodes are an array")
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            (node["node_tag"] == "value" && node["semantic_form"] == "union_value").then_some(index)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        indexes.len(),
        2,
        "the authentic fixture has payload and nullary union values"
    );

    let mut current = package.clone();
    for &index in &indexes {
        rekey_application_node(&mut current, index);
    }
    assert_eq!(
        read_package_semantics(&current, &catalog),
        Ok(()),
        "payload and nullary union values are admitted after graph rekey"
    );
    for &index in &indexes {
        let current_digest = current["semantic_graph"]["nodes"][index]["node_id"]["digest"]
            .as_str()
            .expect("current node key is a digest");
        let stale_digest =
            match current["semantic_graph"]["nodes"][index]["body"]["members"][0]["name"]
                .as_str()
                .expect("union member name is a string")
            {
                "Rect" => "e50bf8e0e17f7cf14ce559bdfe07c5a6f7baafb0820b0b2fdc2db02ed34c385f",
                "Empty" => "d09135f93f7eaef2b22d256039978e7dcff7133bc85710b2bdceba5777bb9a60",
                member => panic!("unexpected union member {member}"),
            };
        assert_ne!(
            current_digest, stale_digest,
            "the prior fixture key is stale under FR-322"
        );
        let mut stale = serde_json::from_str::<Value>(
            &serde_json::to_string(&current)
                .expect("current package serializes")
                .replace(current_digest, stale_digest),
        )
        .expect("stale package parses");
        refresh_package_id(&mut stale);
        let expected = stale["semantic_graph"]["nodes"][index]["node_id"].clone();
        assert_eq!(
            read_package_semantics(&stale, &catalog),
            Err(ReadRefusal::StaleNodeKey(expected)),
            "a stale payload or nullary union value key is refused with stale-node-key"
        );
    }
}

#[trace("TC-395", "FR-440-AC-1")]
#[trace("TC-395", "FR-440-AC-2")]
#[trace("TC-395", "FR-440-AC-4")]
#[test]
fn tc_395_malformed_union_bodies_fail_the_published_schema() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    let index = |tag: &str, form: &str| {
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("semantic graph nodes are an array")
            .iter()
            .position(|node| node["node_tag"] == tag && node["semantic_form"] == form)
            .expect("the fixture carries the node")
    };
    let union = index("composite_type", "union");
    let value = index("value", "union_value");
    let case = index("expression", "case");
    let mutations: [(&str, String, Value); 4] = [
        (
            "union member payload that is not an aggregate",
            format!("/semantic_graph/nodes/{union}/body/members/0/value"),
            json!({"term": "aggregate", "members": [{"term": "aggregate", "members": []}]}),
        ),
        (
            "union value with two member bindings",
            format!("/semantic_graph/nodes/{value}/body/members/1"),
            package["semantic_graph"]["nodes"][value]["body"]["members"][0].clone(),
        ),
        (
            "case arm without its binder aggregate",
            format!("/semantic_graph/nodes/{case}/body/arguments/1/value/members/0"),
            json!({"term": "reference", "target": package["semantic_graph"]["nodes"][case]["body"]["arguments"][0]["target"].clone()}),
        ),
        (
            "case application naming another operation",
            format!("/semantic_graph/nodes/{case}/body/operation/identity"),
            json!("quire.op.control.if"),
        ),
    ];
    for (label, pointer, replacement) in mutations {
        let mut mutated = package.clone();
        let (parent, last) = pointer.rsplit_once('/').expect("pointer has a parent");
        let target = mutated
            .pointer_mut(parent)
            .unwrap_or_else(|| panic!("{label}: {parent} exists"));
        match target {
            Value::Array(items) => {
                let at = last.parse::<usize>().expect("array index");
                if at < items.len() {
                    items[at] = replacement;
                } else {
                    items.push(replacement);
                }
            }
            Value::Object(map) => {
                map.insert(last.to_owned(), replacement);
            }
            _ => panic!("{label}: {parent} is a container"),
        }
        assert!(
            !schema.is_valid(&mutated),
            "{label} fails the published V2 schema"
        );
    }
}

#[trace("TC-396", "FR-441-AC-1")]
#[trace("TC-396", "FR-441-AC-2")]
#[trace("TC-396", "FR-441-AC-3")]
#[trace("TC-396", "FR-441-AC-4")]
#[test]
fn tc_396_union_member_keys_recompute_and_differ_across_unions() {
    let schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-preimage schema is valid");
    let fixture = parsed_fixture(NODE_IDENTITY_VECTORS);
    let vectors = fixture["union_member_vectors"]
        .as_array()
        .expect("union member vectors are an array");
    let mut digests = BTreeSet::new();
    for vector in vectors {
        let name = vector["name"].as_str().expect("vector name is a string");
        schema
            .validate(&vector["preimage"])
            .unwrap_or_else(|error| panic!("{name} has the closed preimage shape: {error}"));
        let digest = unqualified_jcs_sha256(&vector["preimage"]);
        assert_eq!(
            digest,
            vector["sha256"].as_str().unwrap_or_default(),
            "{name} binds its exact RFC 8785 JCS preimage"
        );
        assert!(digests.insert(digest), "{name} has its own member key");
    }
    assert_eq!(digests.len(), 5, "three composite-union members and two model-union members");
    let model_union = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let shape = &model_union["declaration_nodes"][3];
    assert_eq!(shape["name"], "Shape");
    for name in ["union-member-model-shape-both", "union-member-model-shape-empty"] {
        let vector = vectors
            .iter()
            .find(|vector| vector["name"] == name)
            .unwrap_or_else(|| panic!("{name} is published"));
        assert_eq!(
            vector["preimage"]["declaration_node_id"]["digest"], shape["sha256"],
            "{name}: the member of a model union is keyed by the variant type's node, payload or not"
        );
    }
    let mut renamed = vectors[0]["preimage"].clone();
    renamed["member"] = json!("circle");
    assert_ne!(
        unqualified_jcs_sha256(&renamed),
        vectors[0]["sha256"].as_str().unwrap_or_default(),
        "a member identifier is compared byte for byte"
    );
}

/// A Leaf term of FR-322's body grammar. `group_reference` is a Leaf only in
/// a node-identity preimage.
fn is_leaf_term(term: &Value, preimage: bool) -> bool {
    match term["term"].as_str() {
        Some("literal" | "reference" | "dependency_reference") => true,
        Some("group_reference") => preimage,
        _ => false,
    }
}

/// A Group term: an `aggregate` whose members are each a Leaf or a `binding`
/// of a Leaf.
fn is_group_term(term: &Value, preimage: bool) -> bool {
    term["term"] == "aggregate"
        && term["members"].as_array().is_some_and(|members| {
            members.iter().all(|member| {
                is_leaf_term(member, preimage)
                    || (member["term"] == "binding" && is_leaf_term(&member["value"], preimage))
            })
        })
}

/// A Tuple term: an `aggregate` whose members are each a Leaf or a Group.
fn is_tuple_term(term: &Value, preimage: bool) -> bool {
    term["term"] == "aggregate"
        && term["members"].as_array().is_some_and(|members| {
            members
                .iter()
                .all(|member| is_leaf_term(member, preimage) || is_group_term(member, preimage))
        })
}

/// A Member term: a Leaf, a Group, or a `binding` of a Leaf, a Group or a
/// Tuple.
fn is_member_term(term: &Value, preimage: bool) -> bool {
    let value = &term["value"];
    is_leaf_term(term, preimage)
        || is_group_term(term, preimage)
        || (term["term"] == "binding"
            && (is_leaf_term(value, preimage)
                || is_group_term(value, preimage)
                || is_tuple_term(value, preimage)))
}

/// A Body term: a Leaf, an `application` of Members, an `aggregate` of
/// Members, a `frame` body or an `extension_form` body whose clause values
/// are each a `reference`. No production names itself or a higher
/// stratum, so the walk is a fixed number of levels deep.
fn is_body_term(term: &Value, preimage: bool) -> bool {
    let all_members = |items: &Value| {
        items
            .as_array()
            .is_some_and(|items| items.iter().all(|item| is_member_term(item, preimage)))
    };
    match term["term"].as_str() {
        Some("application") => all_members(&term["arguments"]),
        Some("aggregate") => all_members(&term["members"]),
        Some("frame" | "abstraction_relation") => true,
        Some("extension_form") => term["clauses"].as_array().is_some_and(|clauses| {
            clauses
                .iter()
                .all(|clause| clause["term"] == "binding" && clause["value"]["term"] == "reference")
        }),
        _ => is_leaf_term(term, preimage),
    }
}

/// Every node body of every published V2 package, and every application
/// node-identity preimage, is in the closed five-stratum body grammar.
#[trace("TC-427", "FR-322-AC-39")]
#[test]
fn tc_427_published_bodies_follow_the_five_stratum_body_grammar() {
    for (label, package) in published_v2_packages() {
        for node in package["semantic_graph"]["nodes"]
            .as_array()
            .expect("the graph has nodes")
        {
            assert!(
                is_body_term(&node["body"], false),
                "{label}: node {} has a body in the grammar",
                node["node_id"]["digest"]
            );
        }
    }
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let application = schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "ApplicationNode");
    let mut preimages = 0;
    for vector in vectors["operation_vectors"]
        .as_array()
        .expect("operation vectors are an array")
    {
        let preimage = &vector["preimage"];
        assert!(
            application.is_valid(preimage),
            "an application preimage fits the schema"
        );
        assert!(
            is_body_term(&preimage["body"], true),
            "an application preimage body is in the grammar"
        );
        preimages += 1;
    }
    assert!(
        preimages > 0,
        "the operation vectors carry application preimages"
    );
}

/// JSON nesting depth of a value: a scalar is 0, a container one more than its
/// deepest member.
fn json_depth(value: &Value) -> usize {
    match value {
        Value::Array(items) => 1 + items.iter().map(json_depth).max().unwrap_or(0),
        Value::Object(members) => 1 + members.values().map(json_depth).max().unwrap_or(0),
        _ => 0,
    }
}

/// An `abstraction_relation` body is a Body production of fixed JSON depth:
/// its entries hold only identifiers, node ids and Rust paths, so a filled
/// body is no deeper than an empty one plus its fixed entry shape, and an
/// extra nested level fails the schema.
#[trace("TC-427", "FR-322-AC-39")]
#[test]
fn tc_427_abstraction_relation_bodies_are_fixed_depth_body_productions() {
    let schema = schema_for_def(&parsed_fixture(SCHEMA), "AbstractionRelationBody");
    let node = json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)});
    let body = json!({
        "term": "abstraction_relation",
        "objects": [{
            "type": node,
            "rust_type": ["store", "Config"],
            "fields": [{"name": "versionNumber", "rust_field": "version_number"}]
        }],
        "populations": [{"population": node, "collection": ["store", "configs"]}],
        "frames": [{
            "context": node,
            "operation": "attemptUpdate",
            "function": ["store", "attempt_update"],
            "receiver": "self",
            "parameters": [{"name": "next", "rust_parameter": "next"}]
        }]
    });
    assert!(schema.is_valid(&body), "the filled body fits the schema");
    assert!(is_body_term(&body, false), "the body is a Body production");
    let empty =
        json!({"term": "abstraction_relation", "objects": [], "populations": [], "frames": []});
    assert!(schema.is_valid(&empty));
    assert_eq!(json_depth(&body), 5, "entry shapes fix the depth");
    let mut deeper = body.clone();
    deeper["objects"][0]["fields"][0]["rust_field"] = json!({"nested": "version_number"});
    assert!(
        !schema.is_valid(&deeper),
        "a further nested level fails the schema"
    );
}

/// A composite subterm standing inline where the grammar admits only a Leaf
/// or a Group fails the V2 schema, and the same meaning with that subterm as
/// its own referenced node passes it.
#[trace("TC-427", "FR-322-AC-40")]
#[test]
fn tc_427_nested_inline_terms_fail_the_schema_and_referenced_terms_pass() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("I04 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    let mutations = adverse["body_grammar_mutations"]
        .as_array()
        .expect("body grammar mutations are an array");
    assert_eq!(
        mutations
            .iter()
            .map(|mutation| mutation["outcome"].as_str().expect("outcome is a string"))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["refused:malformed_wire"]),
        "every body grammar defect is a malformed wire"
    );
    for mutation in mutations {
        let id = mutation["id"].as_str().expect("mutation id is a string");
        let pointer = mutation["pointer"].as_str().expect("pointer is a string");
        let node_body = |candidate: &Value| {
            let node = pointer.split('/').take(4).collect::<Vec<_>>().join("/");
            candidate
                .pointer(&format!("{node}/body"))
                .expect("the mutation names a node body")
                .clone()
        };
        let mut nested = parsed_fixture(POSITIVE_ALL_FAMILIES);
        *nested.pointer_mut(pointer).expect("pointer exists") = mutation["replacement"].clone();
        assert!(!schema.is_valid(&nested), "{id} fails the V2 schema");
        assert!(
            !is_body_term(&node_body(&nested), false),
            "{id} is outside the grammar"
        );
        let mut referenced = parsed_fixture(POSITIVE_ALL_FAMILIES);
        *referenced.pointer_mut(pointer).expect("pointer exists") = mutation["flattened"].clone();
        assert!(
            schema.is_valid(&referenced),
            "{id} with a referenced subterm passes"
        );
        assert!(
            is_body_term(&node_body(&referenced), false),
            "{id} referenced form is in the grammar"
        );
    }
}

const SEMANTIC_NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

fn walk_id(name: &str) -> Value {
    json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": name})
}

fn walk_ref(name: &str) -> Value {
    json!({"term": "reference", "target": walk_id(name)})
}

/// A type node named `name` whose body is an aggregate of `members`.
fn walk_type(name: &str, tag: &str, form: &str, members: Vec<Value>) -> Value {
    json!({
        "node_id": walk_id(name),
        "node_tag": tag,
        "semantic_form": form,
        "semantic_type": walk_id(name),
        "body": {"term": "aggregate", "members": members}
    })
}

fn walk_text(name: &str, profile: &str) -> Value {
    walk_type(
        name,
        "scalar_type",
        "text",
        vec![
            json!({"term": "binding", "name": "text_profile", "value": {"term": "literal", "value": profile}}),
        ],
    )
}

fn walk_scalar(name: &str, form: &str) -> Value {
    walk_type(name, "scalar_type", form, vec![])
}

fn walk_record(name: &str, fields: &[(&str, &str)]) -> Value {
    let members = fields
        .iter()
        .map(|(field, ty)| json!({"term": "binding", "name": field, "value": walk_ref(ty)}))
        .collect();
    walk_type(name, "composite_type", "record", members)
}

fn walk_tuple(name: &str, positions: &[&str]) -> Value {
    walk_type(
        name,
        "composite_type",
        "tuple",
        positions.iter().map(|ty| walk_ref(ty)).collect(),
    )
}

fn walk_union(name: &str, members: &[(&str, &[&str])]) -> Value {
    let members = members
        .iter()
        .map(|(member, payload)| {
            json!({
                "term": "binding",
                "name": member,
                "value": {"term": "aggregate", "members": payload.iter().map(|ty| walk_ref(ty)).collect::<Vec<_>>()}
            })
        })
        .collect();
    walk_type(name, "composite_type", "union", members)
}

fn walk_option(name: &str, inner: &str) -> Value {
    walk_type(name, "composite_type", "option", vec![walk_ref(inner)])
}

/// The catalog's text-profile law definition.
fn walk_text_law(catalog: &Value) -> Value {
    walk_catalog_law(catalog, "text_profile")
}

fn walk_catalog_law(catalog: &Value, role: &str) -> Value {
    let law = &catalog["law_roles"][role][0];
    json!({"authority": law["authority"], "identity": law["identity"]})
}

/// A package holding `types`, two operand values `left` and `right` of type
/// `compared` and a Boolean result type, whose lock selects the text law.
fn walk_package(catalog: &Value, mut types: Vec<Value>, compared: &str) -> Value {
    types.push(walk_scalar("Bool", "boolean"));
    for operand in ["left", "right"] {
        types.push(json!({
            "node_id": walk_id(operand),
            "node_tag": "value",
            "semantic_form": "parameter",
            "semantic_type": walk_id(compared),
            "body": {"term": "aggregate", "members": []}
        }));
    }
    json!({
        "lock": {"profile_selections": [], "definition_selections": [walk_text_law(catalog)]},
        "semantic_graph": {"nodes": types}
    })
}

/// `quire.op.structural.eq` over `left` and `right` carrying `leaves`.
fn walk_structural_eq(leaves: &[Value]) -> Value {
    json!({
        "term": "application",
        "operator": "binary",
        "operation": {"identity": "quire.op.structural.eq", "laws": [], "mode": null, "member": null, "leaves": leaves},
        "result_type": walk_id("Bool"),
        "arguments": [walk_ref("left"), walk_ref("right")]
    })
}

fn walk_text_leaf(catalog: &Value, path: &[&str], profile: &str) -> Value {
    json!({
        "path": path,
        "laws": [{"role": "text_profile", "definition": walk_text_law(catalog)}],
        "mode": {"kind": "text_profile", "value": profile}
    })
}

fn walk_recursion_leaf(path: &[&str]) -> Value {
    json!({"path": path, "laws": [], "mode": null})
}

/// `quire.op.structural.eq` over two `compared` values of `types` is admitted
/// with exactly `leaves`, in order: with the last leaf dropped it refuses
/// `operation-law-missing`, and with the leaves reversed it refuses
/// `operation-law-mismatch`.
fn assert_structural_leaves(types: Vec<Value>, compared: &str, leaves: &[Value]) {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = walk_package(&catalog, types, compared);
    let oracle = OperationOracle::for_package(&catalog, &package);
    assert_eq!(
        oracle
            .classify(&walk_structural_eq(leaves))
            .map(|result| result.family),
        Ok("boolean".to_owned()),
        "eq over {compared} is admitted with exactly its walk's leaves"
    );
    if let Some((_, fewer)) = leaves.split_last() {
        assert_eq!(
            oracle.classify(&walk_structural_eq(fewer)).err(),
            Some(OperationRefusal::LawMissing),
            "eq over {compared} without its last leaf refuses"
        );
    }
    if leaves.len() > 1 {
        let reversed = leaves.iter().rev().cloned().collect::<Vec<_>>();
        assert_eq!(
            oracle.classify(&walk_structural_eq(&reversed)).err(),
            Some(OperationRefusal::LawMismatch),
            "eq over {compared} with its leaves out of walk order refuses"
        );
    }
}

/// FR-322's `Label` union: text leaves under `member:<Name>`, `position:i`,
/// in member declaration order, none for the payload-less member.
#[trace("TC-395", "FR-322-AC-45")]
#[trace("TC-395", "FR-440-AC-8")]
#[test]
fn tc_395_union_text_leaves_are_member_then_position_paths_in_member_order() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    assert_structural_leaves(
        vec![
            walk_text("Short", "nfc"),
            walk_scalar("Integer", "integer"),
            walk_union(
                "Label",
                &[
                    ("Named", &["Short"]),
                    ("Tagged", &["Integer", "Short"]),
                    ("Empty", &[]),
                ],
            ),
        ],
        "Label",
        &[
            walk_text_leaf(&catalog, &["member:Named", "position:0"], "nfc"),
            walk_text_leaf(&catalog, &["member:Tagged", "position:1"], "nfc"),
        ],
    );
    for source in [SCHEMA, NODE_PREIMAGE_SCHEMA] {
        let segment = schema_for_def(&parsed_fixture(source), "LeafSegment");
        assert!(segment.is_valid(&json!("member:Named")));
        assert!(!segment.is_valid(&json!("member:")));
    }
}

/// A cycle through a record, tuple or union is a recursion leaf: nothing
/// when no text type is reachable, else `recursion:<d>` with `laws` `[]` and
/// `mode` `null`; never `operator-ineligible`.
#[trace("TC-395", "FR-322-AC-46")]
#[trace("TC-395", "FR-440-AC-8")]
#[test]
fn tc_395_cycles_through_records_tuples_and_unions_are_recursion_leaves() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let text = || walk_text("Short", "nfc");
    let integer = || walk_scalar("Integer", "integer");

    let list = || walk_union("List", &[("Cons", &["Integer", "List"]), ("Nil", &[])]);
    assert_structural_leaves(vec![integer(), list()], "List", &[]);
    assert_structural_leaves(
        vec![
            integer(),
            walk_record("Tree", &[("kids", "Forest")]),
            walk_union("Forest", &[("Grow", &["Integer", "Tree"]), ("Done", &[])]),
        ],
        "Tree",
        &[],
    );

    assert_structural_leaves(
        vec![
            text(),
            walk_record("Node", &[("label", "Short"), ("next", "OptNode")]),
            walk_option("OptNode", "Node"),
        ],
        "Node",
        &[
            walk_text_leaf(&catalog, &["field:label"], "nfc"),
            walk_recursion_leaf(&["field:next", "inner", "recursion:0"]),
        ],
    );
    assert_structural_leaves(
        vec![
            text(),
            walk_record("Cell", &[("item", "Pair")]),
            walk_tuple("Pair", &["Short", "OptCell"]),
            walk_option("OptCell", "Cell"),
        ],
        "Cell",
        &[
            walk_text_leaf(&catalog, &["field:item", "position:0"], "nfc"),
            walk_recursion_leaf(&["field:item", "position:1", "inner", "recursion:0"]),
        ],
    );
    let chain = || walk_union("Chain", &[("Link", &["Short", "Chain"]), ("End", &[])]);
    assert_structural_leaves(
        vec![text(), chain()],
        "Chain",
        &[
            walk_text_leaf(&catalog, &["member:Link", "position:0"], "nfc"),
            walk_recursion_leaf(&["member:Link", "position:1", "recursion:0"]),
        ],
    );
    assert_structural_leaves(
        vec![text(), chain(), walk_option("OptChain", "Chain")],
        "OptChain",
        &[
            walk_text_leaf(&catalog, &["inner", "member:Link", "position:0"], "nfc"),
            walk_recursion_leaf(&["inner", "member:Link", "position:1", "recursion:1"]),
        ],
    );

    let package = walk_package(&catalog, vec![text(), chain()], "Chain");
    let oracle = OperationOracle::for_package(&catalog, &package);
    let mut with_law = walk_recursion_leaf(&["member:Link", "position:1", "recursion:0"]);
    with_law["laws"] = json!([{"role": "text_profile", "definition": walk_text_law(&catalog)}]);
    assert_eq!(
        oracle
            .classify(&walk_structural_eq(&[
                walk_text_leaf(&catalog, &["member:Link", "position:0"], "nfc"),
                with_law,
            ]))
            .err(),
        Some(OperationRefusal::LawMismatch),
        "a recursion leaf carries no law"
    );

    for source in [SCHEMA, NODE_PREIMAGE_SCHEMA] {
        let segment = schema_for_def(&parsed_fixture(source), "LeafSegment");
        for admitted in ["recursion:0", "recursion:12"] {
            assert!(
                segment.is_valid(&json!(admitted)),
                "{admitted} is a segment"
            );
        }
        for refused in ["recursion:", "recursion:01", "recursion:-1"] {
            assert!(
                !segment.is_valid(&json!(refused)),
                "{refused} is no segment"
            );
        }
    }
}

/// The V2 Diagnostic schema admits each #181 cause only with its codes.
#[trace("TC-233", "FR-322-AC-47", "FR-322-AC-78")]
#[test]
fn tc_233_diagnostic_schema_pairs_each_selection_and_type_cause_with_its_codes() {
    let schema = parsed_fixture(SCHEMA);
    let diagnostic = schema_for_def(&schema, "Diagnostic");
    let codes = string_list(
        &schema["$defs"]["Diagnostic"]["properties"]["code"]["enum"],
        "diagnostic codes",
    );
    let pairings: [(&str, &[&str]); 12] = [
        ("type-mismatch", &["ill_typed"]),
        ("non-boolean-root", &["ill_typed"]),
        (
            "missing-selection",
            &["missing_import", "missing_declaration"],
        ),
        (
            "unsupported-selection",
            &["unknown_language", "unknown_edition", "unknown_profile"],
        ),
        (
            "wrong-selection-role",
            &["unknown_language", "unknown_edition", "unknown_profile"],
        ),
        ("unsupported-wire", &["unknown_wire"]),
        ("unsupported-feature", &["unknown_required_feature"]),
        ("missing-timed-model", &["invalid_package"]),
        ("timed-node-reference", &["invalid_package"]),
        ("duplicate-clock-place", &["invalid_package"]),
        ("malformed-clock-place", &["invalid_package"]),
        ("invalid-time-quantity", &["ill_typed"]),
    ];
    for (cause, admitted) in pairings {
        for code in &codes {
            let entry = json!({"stage": "package_read", "code": code, "cause_tag": cause, "details": [], "loci": []});
            assert_eq!(
                diagnostic.is_valid(&entry),
                admitted.contains(code),
                "the Diagnostic schema pairs {cause} with `{code}` exactly when FR-322 lists it"
            );
        }
    }
}

/// Every `code`/`cause` pair FR-481 names, in its node, reader and claim
/// rules, and every one FR-193 names for hybrid claims and trace replay, is
/// a catalogued pair: the native-diagnostics row of the code lists the cause,
/// and when the code is a V2 `Diagnostic` code the schema's `cause_tag` enum
/// holds the cause and the schema admits it with that code and no other.
#[trace("TC-966", "FR-481-AC-4", "FR-322-AC-47")]
#[test]
fn tc_966_every_hybrid_cause_is_catalogued_and_paired() {
    let schema = parsed_fixture(SCHEMA);
    let diagnostic = schema_for_def(&schema, "Diagnostic");
    let codes = string_list(
        &schema["$defs"]["Diagnostic"]["properties"]["code"]["enum"],
        "diagnostic codes",
    );
    let causes = string_list(
        &schema["$defs"]["Diagnostic"]["properties"]["cause_tag"]["enum"],
        "diagnostic causes",
    );
    let texts = [
        include_str!("../spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md"),
        include_str!("../spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md"),
    ];
    let mut pairs = BTreeSet::new();
    for text in texts {
        let spans = text.split('`').collect::<Vec<_>>();
        for window in spans.windows(3) {
            if window[1] == "/"
                && window[0].chars().all(|c| c.is_ascii_lowercase() || c == '_')
                && window[0].contains('_')
                && window[2].chars().all(|c| c.is_ascii_lowercase() || c == '-')
                && !window[2].is_empty()
            {
                pairs.insert((window[0].to_owned(), window[2].to_owned()));
            }
        }
    }
    assert!(
        pairs.contains(&("ill_typed".to_owned(), "non-boolean-root".to_owned())),
        "the scan finds FR-481's Boolean-root refusal"
    );
    let node_tags = string_list(
        &schema["$defs"]["NodeBase"]["properties"]["node_tag"]["enum"],
        "node tags",
    );
    // A row's first cell names one code, or several codes sharing one row.
    let row_causes = |code: &str| {
        NATIVE_DIAGNOSTICS
            .lines()
            .find(|line| {
                line.split('|')
                    .nth(1)
                    .is_some_and(|cell| backticked(cell).contains(code))
            })
            .map(|line| {
                backticked(
                    line.split('|')
                        .nth(2)
                        .and_then(|cell| cell.split(';').next())
                        .expect("cause cell"),
                )
            })
            .unwrap_or_else(|| panic!("native diagnostics has a `{code}` row"))
    };
    for (code, cause) in &pairs {
        if node_tags.contains(&code.as_str()) {
            continue; // a node tag and form such as `scalar_type`/`boolean`
        }
        assert!(
            row_causes(code).contains(cause),
            "native diagnostics lists `{cause}` in the `{code}` row"
        );
        if codes.contains(&code.as_str()) {
            assert!(causes.contains(&cause.as_str()), "`{cause}` is a Diagnostic cause_tag");
            for other in &codes {
                let entry = json!({"stage": "package_read", "code": other, "cause_tag": cause, "details": [], "loci": []});
                if *other == code.as_str() {
                    assert!(diagnostic.is_valid(&entry), "the schema pairs `{cause}` with `{code}`");
                }
            }
        }
    }
}

/// FR-322-AC-78: `unsupported-wire` and `unsupported-feature` appear in the
/// FR-272 row of `unknown_wire` and `unknown_required_feature` respectively and
/// in no other FR-272 row, and `unknown_contract_version` and
/// `unknown_required_capability` list no cause. The schema half of the pairing
/// is checked by the FR-322-AC-47 test above.
#[trace("TC-233", "FR-322-AC-78")]
#[test]
fn tc_233_unsupported_wire_and_feature_causes_appear_in_their_owner_row_only() {
    let codes: Vec<String> = TYPED_DIAGNOSTIC_CAUSE
        .lines()
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .filter(|cell| cell.starts_with('`') && cell.ends_with('`') && cell.len() > 2)
        .map(|cell| cell.trim_matches('`').to_owned())
        .collect();
    for (cause, owner) in [
        ("unsupported-wire", "unknown_wire"),
        ("unsupported-feature", "unknown_required_feature"),
    ] {
        for code in &codes {
            let lists = diagnostic_row_causes(TYPED_DIAGNOSTIC_CAUSE, code).contains(cause);
            assert_eq!(
                lists,
                code == owner,
                "the FR-272 `{code}` row lists {cause} only when it is `{owner}`"
            );
        }
    }
    for reader in ["unknown_contract_version", "unknown_required_capability"] {
        assert!(
            diagnostic_row_causes(TYPED_DIAGNOSTIC_CAUSE, reader).is_empty(),
            "the FR-272 `{reader}` row lists no cause"
        );
    }
}

/// A temporal-step refusal of FR-370 (or the wire or graph check that
/// precedes the step), with its FR-272 code and cause.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TemporalRefusal {
    /// `invalid_package`/`invalid-value`.
    InvalidValue,
    /// `invalid_package`/`operation-member-mismatch`.
    MemberMismatch,
    /// `invalid_semantic_graph`: a rational bound not in lowest terms.
    InvalidSemanticGraph,
    /// `unknown_profile`/`wrong-selection-role`.
    WrongSelectionRole,
    /// `unknown_profile`/`unsupported-selection`.
    UnsupportedSelection,
    /// `missing_declaration`/`missing-name`.
    MissingName,
    /// `ambiguous_declaration`/`ambiguous-name`.
    AmbiguousName,
    /// `missing_declaration`/`missing-selection`.
    MissingSelection,
    /// `invalid_model_binding`/`malformed-declaration`.
    MalformedDeclaration,
    /// `ill_typed`/`operator-ineligible` at a JSON pointer or node id.
    OperatorIneligible(String),
    /// `malformed_wire` at a JSON pointer.
    MalformedWire(String),
}

const FR_250: &str = include_str!("../spec/objects/temporal/FR-250-temporal-semantic-profile.md");
const INFINITE_TRACE: &str = "quire.temporal.infinite-trace/v1";
const TIMED: &str = "quire.temporal.timed/v1";

/// FR-250's five profile identities, read from its Values table.
fn fr_250_profiles() -> Vec<&'static str> {
    FR_250
        .lines()
        .filter(|line| line.starts_with("| `quire.temporal."))
        .filter_map(|line| line.split('`').nth(1))
        .collect()
}

fn exact_rational(value: &Value) -> Option<(BigInt, BigInt)> {
    Some((
        integer(&value["numerator"])?,
        integer(&value["denominator"])?,
    ))
}

/// Strict wire validation of one `temporal_interval` member's `interval`
/// against both V2 schemas, then the graph's lowest-terms rule for a timed
/// bound.
fn interval_wire_refusal(
    member_schemas: &[jsonschema::Validator],
    interval: &Value,
) -> Result<(), TemporalRefusal> {
    let member = json!({"kind": "temporal_interval", "interval": interval});
    if !member_schemas.iter().all(|schema| schema.is_valid(&member)) {
        return Err(TemporalRefusal::InvalidValue);
    }
    if interval.get("lower_end").is_some()
        && [&interval["lower"], &interval["upper"]]
            .into_iter()
            .any(|bound| !rational_is_canonical(bound))
    {
        return Err(TemporalRefusal::InvalidSemanticGraph);
    }
    Ok(())
}

/// FR-370 "Placement and profile fit": whether `profile` admits the shape
/// of `interval`.
fn profile_fits(profile: &str, interval: &Value) -> bool {
    let timed = interval.get("lower_end").is_some();
    match profile {
        INFINITE_TRACE => !timed,
        TIMED => interval.is_null() || timed,
        _ => !interval.is_null() && !timed && !interval["upper"].is_null(),
    }
}

/// FR-370's checks of one `temporal_interval` member of a clause under
/// `profile`, in reader order: wire and graph checks, the profile fit, then
/// the bounds.
fn interval_refusal(
    member_schemas: &[jsonschema::Validator],
    profile: &str,
    interval: &Value,
) -> Result<(), TemporalRefusal> {
    interval_wire_refusal(member_schemas, interval)?;
    if !profile_fits(profile, interval) {
        return Err(TemporalRefusal::MemberMismatch);
    }
    bounds_refusal(interval)
}

/// FR-370's bound check: `lower > upper`, or a timed open end with
/// `lower = upper`, is `invalid-value`.
fn bounds_refusal(interval: &Value) -> Result<(), TemporalRefusal> {
    if interval.is_null() || interval["upper"].is_null() {
        return Ok(());
    }
    let ordering = if interval.get("lower_end").is_some() {
        let ((ln, ld), (un, ud)) = (
            exact_rational(&interval["lower"]).ok_or(TemporalRefusal::InvalidValue)?,
            exact_rational(&interval["upper"]).ok_or(TemporalRefusal::InvalidValue)?,
        );
        let ordering = (ln * &ud).cmp(&(un * &ld));
        let open = interval["lower_end"] == "open" || interval["upper_end"] == "open";
        if ordering.is_eq() && open {
            return Err(TemporalRefusal::InvalidValue);
        }
        ordering
    } else {
        integer(&interval["lower"]).cmp(&integer(&interval["upper"]))
    };
    if ordering.is_gt() {
        return Err(TemporalRefusal::InvalidValue);
    }
    Ok(())
}

fn temporal_member_schemas() -> Vec<jsonschema::Validator> {
    [SCHEMA, NODE_PREIMAGE_SCHEMA]
        .into_iter()
        .map(|source| schema_for_def(&parsed_fixture(source), "OperationMember"))
        .collect()
}

fn timed_interval(lower: (&str, &str), upper: (&str, &str), ends: (&str, &str)) -> Value {
    json!({
        "lower": {"numerator": lower.0, "denominator": lower.1},
        "upper": {"numerator": upper.0, "denominator": upper.1},
        "lower_end": ends.0,
        "upper_end": ends.1
    })
}

#[trace("TC-303", "FR-370-AC-8")]
#[test]
fn tc_303_timed_intervals_carry_exact_rational_bounds_and_open_or_closed_ends() {
    let schemas = temporal_member_schemas();
    let three = ("3", "1");
    let zero = ("0", "1");
    let ends = [
        ("closed", "closed"),
        ("closed", "open"),
        ("open", "closed"),
        ("open", "open"),
    ];
    let variants = ends
        .iter()
        .map(|end| timed_interval(zero, three, *end))
        .collect::<Vec<_>>();
    for interval in &variants {
        assert_eq!(
            interval_refusal(&schemas, TIMED, interval),
            Ok(()),
            "{interval} is admitted"
        );
    }
    assert_eq!(
        variants
            .iter()
            .map(unqualified_jcs_sha256)
            .collect::<BTreeSet<_>>()
            .len(),
        4,
        "the four end variants are distinct members"
    );
    let cases = [
        (timed_interval(("1", "2"), three, ends[0]), Ok(())),
        (
            timed_interval(three, three, ("open", "closed")),
            Err(TemporalRefusal::InvalidValue),
        ),
        (
            timed_interval(three, ("1", "1"), ends[0]),
            Err(TemporalRefusal::InvalidValue),
        ),
        (
            timed_interval(("2", "4"), three, ends[0]),
            Err(TemporalRefusal::InvalidSemanticGraph),
        ),
    ];
    for (interval, outcome) in cases {
        assert_eq!(
            interval_refusal(&schemas, TIMED, &interval),
            outcome,
            "{interval}"
        );
    }
    for profile in fr_250_profiles()
        .into_iter()
        .filter(|profile| *profile != TIMED)
    {
        assert_eq!(
            interval_refusal(&schemas, profile, &variants[0]),
            Err(TemporalRefusal::MemberMismatch),
            "the timed form is not admitted under {profile}"
        );
    }
}

#[trace("TC-303", "FR-370-AC-9")]
#[test]
fn tc_303_negative_interval_bounds_fail_the_schema_under_every_profile() {
    let schemas = temporal_member_schemas();
    let negatives = [
        json!({"lower": "-1", "upper": null}),
        json!({"lower": "0", "upper": "-1"}),
        timed_interval(("-1", "1"), ("3", "1"), ("closed", "closed")),
    ];
    let profiles = fr_250_profiles();
    assert_eq!(profiles.len(), 5, "FR-250 lists five profiles");
    for profile in profiles {
        for interval in &negatives {
            assert_eq!(
                interval_refusal(&schemas, profile, interval),
                Err(TemporalRefusal::InvalidValue),
                "{interval} under {profile} is invalid-value, never operation-member-mismatch"
            );
        }
    }
}

/// FR-370 "Profile check" of a clause law's definition against the lock.
fn profile_check(definition: &Value, lock: &Value) -> Result<(), TemporalRefusal> {
    if fr_250_profiles().contains(&definition["identity"].as_str().unwrap_or_default()) {
        return Ok(());
    }
    let other_role = std::iter::once(&lock["edition"])
        .chain(lock["profile_selections"].as_array().into_iter().flatten())
        .any(|selection| {
            selection["definition"] == *definition && selection["role"] != "temporal_profile"
        });
    Err(if other_role {
        TemporalRefusal::WrongSelectionRole
    } else {
        TemporalRefusal::UnsupportedSelection
    })
}

/// A temporal clause reduced to what FR-370's per-clause checks read.
struct TemporalClauseCase {
    law: Value,
    over_declared: bool,
    intervals: Vec<Value>,
}

/// FR-370's per-clause checks in reader order: wire and graph checks of the
/// intervals, the profile check, the `over` reference, the profile fit of
/// every interval, then each interval's bounds.
fn temporal_clause_refusal(
    clause: &TemporalClauseCase,
    lock: &Value,
) -> Result<(), TemporalRefusal> {
    let schemas = temporal_member_schemas();
    for interval in &clause.intervals {
        interval_wire_refusal(&schemas, interval)?;
    }
    profile_check(&clause.law, lock)?;
    if !clause.over_declared {
        return Err(TemporalRefusal::MissingName);
    }
    let profile = clause.law["identity"].as_str().unwrap_or_default();
    if !clause
        .intervals
        .iter()
        .all(|interval| profile_fits(profile, interval))
    {
        return Err(TemporalRefusal::MemberMismatch);
    }
    clause.intervals.iter().try_for_each(bounds_refusal)
}

#[trace("TC-303", "FR-370-AC-10")]
#[test]
fn tc_303_a_clause_profile_check_runs_before_its_other_checks() {
    let protocol = json!({"authority": "agent-ix", "identity": "quire.protocol.finite-global/v1"});
    let lock = json!({
        "edition": {"role": "edition", "definition": {"authority": "agent-ix", "identity": "quire-edition"}},
        "profile_selections": [{"role": "protocol_profile", "definition": protocol}]
    });
    let law = |identity: &str| json!({"authority": "agent-ix", "identity": identity});
    let integer_interval = json!({"lower": "0", "upper": "3"});
    let known = TemporalClauseCase {
        law: law(TIMED),
        over_declared: true,
        intervals: vec![json!(null)],
    };
    assert_eq!(temporal_clause_refusal(&known, &lock), Ok(()));
    for (definition, refusal) in [
        (protocol.clone(), TemporalRefusal::WrongSelectionRole),
        (
            law("quire.temporal.unknown/v1"),
            TemporalRefusal::UnsupportedSelection,
        ),
    ] {
        let clause = TemporalClauseCase {
            law: definition,
            over_declared: false,
            intervals: vec![
                json!({"lower": "3", "upper": "1"}),
                integer_interval.clone(),
            ],
        };
        assert_eq!(
            temporal_clause_refusal(&clause, &lock),
            Err(refusal),
            "the profile check precedes the over and interval defects"
        );
    }
    let clause = TemporalClauseCase {
        law: law(TIMED),
        over_declared: false,
        intervals: vec![integer_interval],
    };
    assert_eq!(
        temporal_clause_refusal(&clause, &lock),
        Err(TemporalRefusal::MissingName),
        "with a known profile the over defect is reported next"
    );
}

/// FR-370 "Fairness resolution": the effective operations of an object
/// type, own first, then each supertype's that the type does not redefine,
/// each with the type that declares it.
fn effective_operations(domain: &Value, node: &str) -> Vec<(String, String)> {
    let Some(object_type) = domain["object_types"]
        .as_array()
        .and_then(|types| types.iter().find(|candidate| candidate["node"] == node))
    else {
        return Vec::new();
    };
    let own = string_list(&object_type["operations"], "operations");
    let mut effective = own
        .iter()
        .map(|name| ((*name).to_owned(), node.to_owned()))
        .collect::<Vec<_>>();
    for supertype in string_list(&object_type["supertypes"], "supertypes") {
        for inherited in effective_operations(domain, supertype) {
            if !own.contains(&inherited.0.as_str()) && !effective.contains(&inherited) {
                effective.push(inherited);
            }
        }
    }
    effective
}

/// The three steps of FR-370 "Fairness resolution" for one fairness member;
/// `Ok` names the type whose operation it resolves to.
fn fairness_resolution(
    nodes: &[Value],
    selected: &[&str],
    domain: &Value,
    member: &Value,
) -> Result<String, TemporalRefusal> {
    let digest = member["declaration"]["digest"].as_str().unwrap_or_default();
    let declaring = node_by_digest(nodes, digest)
        .filter(|node| node["node_tag"] == "model" && node["semantic_form"] == "object_type");
    if declaring.is_none() {
        return Err(TemporalRefusal::MalformedDeclaration);
    }
    if !selected.contains(&digest) {
        return Err(TemporalRefusal::MissingSelection);
    }
    let matches = effective_operations(domain, digest)
        .into_iter()
        .filter(|(name, _)| member["name"] == name.as_str())
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Err(TemporalRefusal::MissingName),
        [(_, owner)] => Ok(owner.clone()),
        _ => Err(TemporalRefusal::AmbiguousName),
    }
}

#[trace("TC-303", "FR-370-AC-11")]
#[test]
fn tc_303_fairness_members_resolve_among_effective_operations() {
    let model = |name: &str, form: &str| json!({"node_id": walk_id(name), "node_tag": "model", "semantic_form": form});
    let nodes = [
        model("Base", "object_type"),
        model("Derived", "object_type"),
        model("Leaf", "object_type"),
        model("Left", "object_type"),
        model("Right", "object_type"),
        model("Both", "object_type"),
        model("Stray", "object_type"),
        model("Gateway", "systems_interface"),
        json!({"node_id": walk_id("Count"), "node_tag": "value", "semantic_form": "parameter"}),
    ];
    let selected = [
        "Base", "Derived", "Leaf", "Left", "Right", "Both", "Gateway",
    ];
    let domain = json!({"object_types": [
        {"node": "Base", "operations": ["run", "stop"], "supertypes": []},
        {"node": "Derived", "operations": ["run"], "supertypes": ["Base"]},
        {"node": "Leaf", "operations": [], "supertypes": ["Derived"]},
        {"node": "Left", "operations": ["go"], "supertypes": []},
        {"node": "Right", "operations": ["go"], "supertypes": []},
        {"node": "Both", "operations": [], "supertypes": ["Left", "Right"]},
        {"node": "Stray", "operations": ["run"], "supertypes": []}
    ]});
    let member = |declaration: &str, name: &str| json!({"kind": "fairness", "fairness_kind": "weak", "granularity": "whole", "declaration": walk_id(declaration), "name": name});
    let cases = [
        (member("Leaf", "run"), Ok("Derived".to_owned())),
        (member("Leaf", "stop"), Ok("Base".to_owned())),
        (member("Leaf", "halt"), Err(TemporalRefusal::MissingName)),
        (member("Both", "go"), Err(TemporalRefusal::AmbiguousName)),
        (
            member("Stray", "run"),
            Err(TemporalRefusal::MissingSelection),
        ),
        (
            member("Gateway", "run"),
            Err(TemporalRefusal::MalformedDeclaration),
        ),
        (
            member("Count", "run"),
            Err(TemporalRefusal::MalformedDeclaration),
        ),
    ];
    for (member, outcome) in cases {
        assert_eq!(
            fairness_resolution(&nodes, &selected, &domain, &member),
            outcome,
            "{member}"
        );
    }
}

/// Every application within `term` with its JSON pointer, `pointer` naming
/// `term` itself.
fn applications_at<'v>(term: &'v Value, pointer: &str) -> Vec<(String, &'v Value)> {
    let mut found = Vec::new();
    let nested = |key: &str| {
        term[key]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .flat_map(|(index, item)| applications_at(item, &format!("{pointer}/{key}/{index}")))
            .collect::<Vec<_>>()
    };
    match term["term"].as_str() {
        Some("application") => {
            found.push((pointer.to_owned(), term));
            found.extend(nested("arguments"));
        }
        Some("aggregate") => found.extend(nested("members")),
        Some("binding") => {
            found.extend(applications_at(&term["value"], &format!("{pointer}/value")))
        }
        _ => {}
    }
    found
}

/// Every reference target within `term`.
fn reference_targets(term: &Value) -> Vec<&Value> {
    match term["term"].as_str() {
        Some("reference") => vec![&term["target"]],
        Some("application") => term["arguments"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(reference_targets)
            .collect(),
        Some("aggregate") => term["members"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(reference_targets)
            .collect(),
        Some("binding") => reference_targets(&term["value"]),
        _ => Vec::new(),
    }
}

fn node_form<'v>(nodes: &'v [Value], target: &Value) -> Option<(&'v str, &'v str)> {
    let node = nodes.iter().find(|node| node["node_id"] == *target)?;
    Some((node["node_tag"].as_str()?, node["semantic_form"].as_str()?))
}

/// FR-370 placement of references to formula and fairness nodes: a formula
/// node only from a clause's formula argument or a formula application's
/// operand, a fairness node only from a clause's fairness argument.
/// Located at the referencing node.
fn temporal_reference_refusal(nodes: &[Value]) -> Result<(), TemporalRefusal> {
    let mut ordered = nodes.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|node| node["node_id"]["digest"].as_str().unwrap_or_default());
    for node in ordered {
        let body = &node["body"];
        let clause = node["node_tag"] == "temporal" && node["semantic_form"] == "temporal_clause";
        let formula_operator = body["operator"] == "temporal_formula";
        let positioned = match body["arguments"].as_array() {
            Some(arguments) if clause => arguments
                .iter()
                .enumerate()
                .flat_map(|(index, argument)| {
                    reference_targets(argument)
                        .into_iter()
                        .map(move |target| (Some(index), target))
                })
                .collect::<Vec<_>>(),
            _ => reference_targets(body)
                .into_iter()
                .map(|target| (None, target))
                .collect(),
        };
        for (position, target) in positioned {
            let admitted = match node_form(nodes, target) {
                Some(("temporal", "formula")) => formula_operator || position == Some(5),
                Some(("temporal", "fairness")) => position == Some(4),
                _ => true,
            };
            if !admitted {
                return Err(TemporalRefusal::OperatorIneligible(
                    node["node_id"]["digest"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                ));
            }
        }
    }
    Ok(())
}

/// FR-370 placement of diagnostic `details`: strict wire validation reports
/// an application of the formula, fairness or case operator class at its
/// `operator`; the temporal step then reports a reference to a formula,
/// fairness or case node at its `details` entry.
fn details_refusal(package: &Value) -> Result<(), TemporalRefusal> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let details = package["diagnostics"]["entries"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .flat_map(|(e, entry)| {
            entry["details"]
                .as_array()
                .into_iter()
                .flatten()
                .enumerate()
                .map(move |(d, term)| (format!("/diagnostics/entries/{e}/details/{d}"), term))
        })
        .collect::<Vec<_>>();
    for (pointer, term) in &details {
        if let Some((at, _)) =
            applications_at(term, pointer)
                .into_iter()
                .find(|(_, application)| {
                    matches!(
                        application["operator"].as_str(),
                        Some("temporal_formula" | "temporal_fairness" | "case")
                    )
                })
        {
            return Err(TemporalRefusal::OperatorIneligible(format!(
                "{at}/operator"
            )));
        }
    }
    for (pointer, term) in details {
        let refused = reference_targets(term).into_iter().any(|target| {
            matches!(
                node_form(nodes, target),
                Some(("temporal", "formula" | "fairness") | ("expression", "case"))
            )
        });
        if refused {
            return Err(TemporalRefusal::OperatorIneligible(pointer));
        }
    }
    Ok(())
}

#[trace("TC-303", "FR-370-AC-12")]
#[test]
fn tc_303_formula_fairness_and_case_terms_stand_only_where_placed() {
    let node = |name: &str, tag: &str, form: &str, body: Value| json!({"node_id": walk_id(name), "node_tag": tag, "semantic_form": form, "body": body});
    let clause_body = |fairness: &str, formula: &str| {
        json!({"term": "application", "operator": "temporal", "arguments": [
            walk_ref("over"), walk_ref("clock"), walk_ref("activation"), walk_ref("captures"),
            {"term": "aggregate", "members": [walk_ref(fairness)]}, walk_ref(formula)
        ]})
    };
    let formula = node(
        "formula",
        "temporal",
        "formula",
        json!({"term": "application", "operator": "temporal_formula", "arguments": []}),
    );
    let fairness = node(
        "fairness",
        "temporal",
        "fairness",
        json!({"term": "application", "operator": "temporal_fairness", "arguments": []}),
    );
    let clause = node(
        "clause",
        "temporal",
        "temporal_clause",
        clause_body("fairness", "formula"),
    );
    assert_eq!(
        temporal_reference_refusal(&[formula.clone(), fairness.clone(), clause]),
        Ok(())
    );
    let state = node(
        "state",
        "state",
        "clause",
        json!({"term": "application", "operator": "state_clause", "arguments": [walk_ref("formula")]}),
    );
    assert_eq!(
        temporal_reference_refusal(&[formula.clone(), fairness.clone(), state]),
        Err(TemporalRefusal::OperatorIneligible("state".to_owned())),
        "a formula reference from a state clause's condition refuses at the referencing node"
    );
    let misplaced = node(
        "clause",
        "temporal",
        "temporal_clause",
        clause_body("fairness", "fairness"),
    );
    assert_eq!(
        temporal_reference_refusal(&[formula, fairness, misplaced]),
        Err(TemporalRefusal::OperatorIneligible("clause".to_owned())),
        "a fairness reference from a formula argument refuses at the referencing node"
    );

    let union_package = parsed_fixture(POSITIVE_UNION_NODES);
    let mut package = union_package.clone();
    let nodes = package["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    nodes.push(node(
        "formula",
        "temporal",
        "formula",
        json!({"term": "aggregate", "members": []}),
    ));
    nodes.push(node(
        "fairness",
        "temporal",
        "fairness",
        json!({"term": "aggregate", "members": []}),
    ));
    let id_of =
        |tag: &str, form: &str| union_fixture_node(&union_package, tag, form)["node_id"].clone();
    let reference = |target: Value| json!({"term": "reference", "target": target});
    let with_details = |details: Vec<Value>| {
        let mut candidate = package.clone();
        candidate["diagnostics"]["entries"] = json!([{"stage": "package_read", "code": "ill_typed", "cause_tag": "operator-ineligible", "details": [], "loci": []}, {"stage": "package_read", "code": "ill_typed", "cause_tag": "operator-ineligible", "details": details, "loci": []}]);
        candidate
    };
    let admitted = with_details(vec![
        reference(id_of("composite_type", "union")),
        json!({"term": "binding", "name": "value", "value": reference(id_of("value", "union_value"))}),
    ]);
    assert_eq!(
        details_refusal(&admitted),
        Ok(()),
        "details may reference union and union_value nodes"
    );
    for target in [
        walk_id("formula"),
        walk_id("fairness"),
        id_of("expression", "case"),
    ] {
        assert_eq!(
            details_refusal(&with_details(vec![
                reference(id_of("composite_type", "union")),
                reference(target.clone())
            ])),
            Err(TemporalRefusal::OperatorIneligible(
                "/diagnostics/entries/1/details/1".to_owned()
            )),
            "a details reference to {target} refuses at its details entry"
        );
    }
    let diagnostic = schema_for_def(&parsed_fixture(SCHEMA), "Diagnostic");
    let case_body = union_fixture_node(&union_package, "expression", "case")["body"].clone();
    for operator in ["temporal_formula", "temporal_fairness", "case"] {
        let mut application = case_body.clone();
        application["operator"] = json!(operator);
        let candidate = with_details(vec![application]);
        assert!(
            !diagnostic.is_valid(&candidate["diagnostics"]["entries"][1]),
            "a {operator} application in details is outside the body grammar"
        );
        assert_eq!(
            details_refusal(&candidate),
            Err(TemporalRefusal::OperatorIneligible(
                "/diagnostics/entries/1/details/0/operator".to_owned()
            )),
            "a {operator} application in details refuses at its operator, not malformed_wire"
        );
    }
    let mut formula_application = case_body.clone();
    formula_application["operator"] = json!("temporal_formula");
    let mut with_nested_case = formula_application.clone();
    with_nested_case["arguments"] = json!([case_body.clone()]);
    assert_eq!(
        details_refusal(&with_details(vec![with_nested_case])),
        Err(TemporalRefusal::OperatorIneligible(
            "/diagnostics/entries/1/details/0/operator".to_owned()
        )),
        "the outermost application of a details term is reported first"
    );
    let aggregate = json!({"term": "aggregate", "members": [formula_application, case_body]});
    assert_eq!(
        details_refusal(&with_details(vec![aggregate])),
        Err(TemporalRefusal::OperatorIneligible(
            "/diagnostics/entries/1/details/0/members/0/operator".to_owned()
        )),
        "the first member in document order is reported first"
    );
}

#[trace("TC-303", "FR-370-AC-5")]
#[test]
fn tc_303_nested_temporal_applications_are_malformed_wire() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let host = nodes
        .iter()
        .position(|node| node["body"]["term"] == "application" && node["semantic_form"] != "case")
        .expect("the fixture has another application node");
    let case_body = nodes
        .iter()
        .find(|node| node["semantic_form"] == "case")
        .expect("the fixture has a case node")["body"]
        .clone();
    for operator in ["temporal_formula", "temporal_fairness", "temporal"] {
        let mut application = case_body.clone();
        application["operator"] = json!(operator);
        let mut nested = package.clone();
        nested["semantic_graph"]["nodes"][host]["body"]["arguments"][0] = application;
        assert!(
            !schema.is_valid(&nested),
            "a nested {operator} application is outside the body grammar"
        );
        assert_eq!(
            case_placement_refusal(&nested),
            Err(TemporalRefusal::MalformedWire(format!(
                "/semantic_graph/nodes/{host}/body/arguments/0"
            ))),
            "a nested {operator} application is a malformed wire, not operator-ineligible"
        );
    }
}

/// FR-440 "Case placement" and FR-322's body grammar: a nested `case`
/// application refuses at its `operator` and any other nested application
/// is a malformed wire, both at strict wire validation; a `case` at the body
/// root of a node other than `expression`/`case` then refuses at that node.
fn case_placement_refusal(package: &Value) -> Result<(), TemporalRefusal> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    for (index, node) in nodes.iter().enumerate() {
        let root = format!("/semantic_graph/nodes/{index}/body");
        for (pointer, application) in applications_at(&node["body"], &root) {
            if pointer == root {
                continue;
            }
            return Err(if application["operator"] == "case" {
                TemporalRefusal::OperatorIneligible(format!("{pointer}/operator"))
            } else {
                TemporalRefusal::MalformedWire(pointer)
            });
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        let at_root = node["body"]["operation"]["identity"] == "quire.op.control.case";
        let case_node = node["node_tag"] == "expression" && node["semantic_form"] == "case";
        if at_root && !case_node {
            return Err(TemporalRefusal::OperatorIneligible(format!(
                "/semantic_graph/nodes/{index}"
            )));
        }
    }
    Ok(())
}

#[trace("TC-395", "FR-440-AC-7")]
#[test]
fn tc_395_a_case_application_stands_only_at_an_expression_case_body_root() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let case = nodes
        .iter()
        .position(|node| node["semantic_form"] == "case")
        .expect("the fixture has a case node");

    let mut referenced = package.clone();
    referenced["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes")
        .push(json!({
            "node_id": walk_id("area"),
            "node_tag": "function",
            "semantic_form": "function",
            "body": {"term": "reference", "target": nodes[case]["node_id"].clone()}
        }));
    assert_eq!(
        case_placement_refusal(&referenced),
        Ok(()),
        "a referenced expression/case node is admitted"
    );

    let mut rooted = package.clone();
    rooted["semantic_graph"]["nodes"][case]["node_tag"] = json!("function");
    rooted["semantic_graph"]["nodes"][case]["semantic_form"] = json!("function");
    assert_eq!(
        case_placement_refusal(&rooted),
        Err(TemporalRefusal::OperatorIneligible(format!(
            "/semantic_graph/nodes/{case}"
        ))),
        "a case at a function's body root refuses at that node"
    );

    let host = nodes
        .iter()
        .position(|node| node["body"]["term"] == "application" && node["semantic_form"] != "case")
        .expect("the fixture has another application node");
    let mut nested = package.clone();
    nested["semantic_graph"]["nodes"][host]["body"]["arguments"][0] = nodes[case]["body"].clone();
    assert!(
        !schema.is_valid(&nested),
        "a nested case is outside the body grammar"
    );
    assert_eq!(
        case_placement_refusal(&nested),
        Err(TemporalRefusal::OperatorIneligible(format!(
            "/semantic_graph/nodes/{host}/body/arguments/0/operator"
        ))),
        "a nested case refuses at its operator, not malformed_wire"
    );
    let mut other = nested.clone();
    other["semantic_graph"]["nodes"][host]["body"]["arguments"][0]["operator"] = json!("binary");
    assert_eq!(
        case_placement_refusal(&other),
        Err(TemporalRefusal::MalformedWire(format!(
            "/semantic_graph/nodes/{host}/body/arguments/0"
        ))),
        "any other nested application stays a malformed wire"
    );
}

/// Every ungrouped node of every published positive fixture that is neither
/// nominal nor application-bearing carries the key of its own
/// `quire.structural-node/v1` preimage, so no fixture holds a placeholder.
/// The recursive fixture's group members are checked by
/// `tc_233_recursive_record_fixture_carries_derived_group_keys`.
#[trace("TC-233", "FR-322-AC-50")]
#[test]
fn tc_233_positive_fixtures_carry_derived_structural_keys() {
    for (name, text) in [
        ("positive-all-families", POSITIVE_ALL_FAMILIES),
        ("positive-clause-operations", POSITIVE_CLAUSE_OPERATIONS),
        ("positive-control-operations", POSITIVE_CONTROL_OPERATIONS),
        (
            "positive-operation-identities",
            POSITIVE_OPERATION_IDENTITIES,
        ),
        ("positive-union-nodes", POSITIVE_UNION_NODES),
        ("positive-extension-nodes", POSITIVE_EXTENSION_NODES),
        ("positive-recursive-records", POSITIVE_RECURSIVE_RECORDS),
    ] {
        let package = parsed_fixture(text);
        let mut structural = 0;
        for node in package["semantic_graph"]["nodes"]
            .as_array()
            .expect("package nodes are an array")
        {
            if is_rekeyed_structural_node(node) && structural_node_preimage(node).is_some() {
                structural += 1;
                assert!(
                    structural_node_key_is_current(node),
                    "{name}: {} {} keeps a key that is not its preimage digest",
                    node["node_tag"],
                    node["semantic_form"]
                );
            }
        }
        assert!(structural > 0, "{name} publishes structural nodes");
    }
}

/// A tampered derived-shape node (same `node_id`, patched body, recomputed
/// `package_id`) is schema-valid and identity-consistent
/// yet refused `invalid_package`/`stale-node-key` at its own `node_id`, while
/// the untampered package is admitted. The rows cover an ungrouped
/// `integer_range`, `reference` and `set`, the declared record
/// `Example::Point`, and the in-group `collection_bounds`, `sequence` and
/// `option` of the recursive records.
#[trace("TC-233", "FR-322-AC-48", "FR-322-AC-49")]
#[test]
fn tc_233_tampered_derived_shape_nodes_refuse_stale_node_key() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    let mutations = adverse["derived_shape_key_mutations"]
        .as_array()
        .expect("derived-shape mutations are an array");
    let operation = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let bases = BTreeMap::from([
        (
            "positive-all-families",
            parsed_fixture(POSITIVE_ALL_FAMILIES),
        ),
        ("positive-operation-identities", operation),
        (
            "positive-recursive-records",
            parsed_fixture(POSITIVE_RECURSIVE_RECORDS),
        ),
    ]);
    for (name, base) in &bases {
        assert_eq!(read_package_semantics(base, &catalog), Ok(()), "{name}");
    }
    let tampered_forms = mutations
        .iter()
        .map(|mutation| {
            (
                mutation["node"]["semantic_form"].as_str(),
                mutation["base"].as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    for form in [
        (Some("integer_range"), Some("positive-operation-identities")),
        (Some("reference"), Some("positive-operation-identities")),
        (Some("set"), Some("positive-operation-identities")),
        (
            Some("collection_bounds"),
            Some("positive-recursive-records"),
        ),
        (Some("sequence"), Some("positive-recursive-records")),
        (Some("option"), Some("positive-recursive-records")),
        (Some("record"), Some("positive-all-families")),
    ] {
        assert!(tampered_forms.contains(&form), "{form:?} is tampered");
    }
    for mutation in mutations {
        let id = mutation["id"].as_str().expect("mutation id is a string");
        let base = mutation["base"]
            .as_str()
            .and_then(|name| bases.get(name))
            .unwrap_or_else(|| panic!("{id}: names a published base"));
        let index = base["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| {
                node["node_tag"] == mutation["node"]["node_tag"]
                    && node["semantic_form"] == mutation["node"]["semantic_form"]
            })
            .expect("the mutated node is published");
        let mut candidate = base.clone();
        for prefix in ["/semantic_graph/nodes"] {
            let node = candidate
                .pointer_mut(&format!("{prefix}/{index}"))
                .expect("node pointer exists");
            for patch in mutation["patch"].as_array().expect("patch is an array") {
                apply_authored_patch(node, patch);
            }
        }
        refresh_package_id(&mut candidate);
        assert_eq!(
            candidate["semantic_graph"]["nodes"][index]["node_id"],
            base["semantic_graph"]["nodes"][index]["node_id"],
            "{id}: the tampered node keeps its node id"
        );
        assert_eq!(
            derive_identity_preimage(&candidate),
            candidate["identity_preimage"],
            "{id}: the identity preimage and package id agree with the tampered graph"
        );
        assert!(schema.is_valid(&candidate), "{id}: the schema admits it");
        assert_eq!(
            (
                mutation["expected_cause"].as_str(),
                mutation["expected_code"].as_str()
            ),
            (
                Some(ReadRefusal::STALE_NODE_KEY.0),
                Some(ReadRefusal::STALE_NODE_KEY.1)
            ),
            "{id}: records the stale-node-key refusal"
        );
        assert_eq!(
            read_package_semantics(&candidate, &catalog),
            Err(ReadRefusal::StaleNodeKey(
                candidate["semantic_graph"]["nodes"][index]["node_id"].clone()
            )),
            "{id}: refused stale-node-key at the tampered node"
        );
    }
}

/// FR-322-AC-62's three named derived-form mutations.  Each candidate stays
/// schema-valid and changes only the semantic preimage.  Retaining the old
/// key therefore names a stale node; replacing it with the recomputed digest
/// makes the key current; this test does not claim admission of a malformed
/// recomputed form.  Full-package retained-key refusals for `reference` are
/// exercised by `tc_233_tampered_derived_shape_nodes_refuse_stale_node_key`;
/// this test sends the `model_population` and `compound_unit` cases through
/// that reader as well.
#[trace("FR-322-AC-62", "TC-233")]
#[test]
fn tc_322_derived_form_key_mutations() {
    type KeyMutation = (&'static str, &'static str, fn(&mut Value));

    let schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let package_schema =
        jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let cases: [KeyMutation; 3] = [
        (
            "RM-REF-TARGET",
            r#"{"body":{"members":[{"target":{"digest":"6f006fa31e8ceeb4c7e6ffb71a68a7883643c931f90d487d35239d49390c596b","domain":"quire.checked-semantic-node/v1"},"term":"reference"}],"term":"aggregate"},"declaration":null,"node_tag":"composite_type","recursion":null,"semantic_form":"reference","semantic_type":null,"version":"quire.structural-node/v1"}"#,
            |candidate: &mut Value| {
                candidate["body"]["members"][0]["target"]["digest"] =
                    json!("6f006fa31e8ceeb4c7e6ffb71a68a7883643c931f90d487d35239d49390c596c");
                candidate["body"]["members"][0]["target"]["domain"] =
                    json!("quire.checked-semantic-node/v1");
                candidate["body"]["members"][0]["target"]["digest"] =
                    json!("b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5");
            },
        ),
        (
            "RM-POP-MAX",
            r#"{"body":{"members":[{"name":"max","term":"binding","value":{"term":"literal","type":{"digest":"07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32","domain":"quire.checked-semantic-node/v1"},"value":"3","value_kind":"integer"}}],"term":"aggregate"},"declaration":null,"node_tag":"bounded_domain","recursion":null,"semantic_form":"model_population","semantic_type":{"digest":"b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5","domain":"quire.checked-semantic-node/v1"},"version":"quire.structural-node/v1"}"#,
            |candidate: &mut Value| {
                candidate["body"]["members"][0]["value"]["value"] = json!("5");
            },
        ),
        (
            "RM-CU-ORDER",
            r#"{"body":{"members":[{"members":[{"name":"unit","term":"binding","value":{"target":{"digest":"79637623a46d29e884b62c6fa292aeb29d41e4ecc4e800b4d7ee910a3eaf23a4","domain":"quire.checked-semantic-node/v1"},"term":"reference"}},{"name":"exponent","term":"binding","value":{"term":"literal","type":{"digest":"07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32","domain":"quire.checked-semantic-node/v1"},"value":"1","value_kind":"integer"}}],"term":"aggregate"},{"members":[{"name":"unit","term":"binding","value":{"target":{"digest":"d0f0c5d24d6accb2f1d9afff7c6cdda3143df382e50af4bda0704d48c67ed998","domain":"quire.checked-semantic-node/v1"},"term":"reference"}},{"name":"exponent","term":"binding","value":{"term":"literal","type":{"digest":"07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32","domain":"quire.checked-semantic-node/v1"},"value":"-1","value_kind":"integer"}}],"term":"aggregate"}],"term":"aggregate"},"declaration":null,"node_tag":"scalar_type","recursion":null,"semantic_form":"compound_unit","semantic_type":null,"version":"quire.structural-node/v1"}"#,
            |candidate: &mut Value| {
                candidate["body"]["members"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
            },
        ),
    ];
    for (name, source, mutate) in cases {
        let base = parsed_fixture(source);
        let base_key = unqualified_jcs_sha256(&base);
        let wire_node = |preimage: &Value, key: &str| {
            let node_id = json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": key
            });
            json!({
                "node_id": node_id,
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": preimage["node_tag"],
                "semantic_form": preimage["semantic_form"],
                "semantic_type": if preimage["semantic_type"].is_null() {
                    node_id
                } else {
                    preimage["semantic_type"].clone()
                },
                "dependencies": [],
                "occurrences": [{"role": "type", "ordinal": 0}],
                "body": preimage["body"]
            })
        };
        assert!(
            schema.is_valid(&wire_node(&base, &base_key)),
            "{name}: base wire node schema"
        );
        let mut mutated = base.clone();
        mutate(&mut mutated);
        let mutated_key = unqualified_jcs_sha256(&mutated);
        assert!(
            schema.is_valid(&wire_node(&mutated, &mutated_key)),
            "{name}: mutated wire node schema"
        );
        assert_ne!(base_key, mutated_key, "{name}: mutation changes the key");
        assert_eq!(
            base_key,
            unqualified_jcs_sha256(&base),
            "{name}: retained key"
        );
        assert_eq!(
            mutated_key,
            unqualified_jcs_sha256(&mutated),
            "{name}: recomputed key"
        );

        if matches!(name, "RM-POP-MAX" | "RM-CU-ORDER") {
            let mut package = parsed_fixture(POSITIVE_ALL_FAMILIES);
            assert_eq!(
                read_package_semantics(&package, &catalog),
                Ok(()),
                "{name}: reader admits the unmodified package"
            );
            append_rekeyed_structural_node(&mut package, wire_node(&mutated, &mutated_key));
            let node_index = package["semantic_graph"]["nodes"]
                .as_array()
                .expect("reader fixture nodes are an array")
                .len()
                - 1;
            replace_node_field_in_graph(
                &mut package,
                node_index,
                "/node_id/digest",
                json!(base_key),
            );
            package["identity_preimage"] = derive_identity_preimage(&package);
            refresh_package_id(&mut package);

            let retained_node_id =
                package["semantic_graph"]["nodes"][node_index]["node_id"].clone();
            assert!(
                package_schema.is_valid(&package),
                "{name}: retained-key package remains schema-valid"
            );
            assert_eq!(
                read_package_semantics(&package, &catalog),
                Err(ReadRefusal::StaleNodeKey(retained_node_id)),
                "{name}: the package reader refuses the retained key"
            );
        }
    }
}

#[trace("FR-322-AC-62", "TC-233")]
#[test]
fn tc_322_recomputed_derived_forms_refuse_semantic_mutations() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let base = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);

    let mut wrong_reference = base.clone();
    let reference_index = wrong_reference["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .position(|node| {
            node["node_tag"] == "composite_type" && node["semantic_form"] == "reference"
        })
        .unwrap();
    let integer_id = wrong_reference["semantic_graph"]["nodes"][16]["node_id"].clone();
    replace_node_field_in_graph(
        &mut wrong_reference,
        reference_index,
        "/body/members/0/target",
        integer_id.clone(),
    );
    replace_node_field_in_graph(
        &mut wrong_reference,
        reference_index,
        "/dependencies",
        json!([integer_id]),
    );
    rekey_stale_nodes(&mut wrong_reference);
    assert_eq!(
        read_package_semantics(&wrong_reference, &catalog),
        Err(ReadRefusal::GraphShape(
            wrong_reference["semantic_graph"]["nodes"][reference_index]["node_id"].clone()
        )),
        "recomputed reference target is rejected"
    );

    for (label, replacement_index) in [("function target", 11_usize), ("value target", 27)] {
        let mut candidate = base.clone();
        let index = candidate["semantic_graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .position(|node| {
                node["node_tag"] == "composite_type" && node["semantic_form"] == "reference"
            })
            .unwrap();
        let target = candidate["semantic_graph"]["nodes"][replacement_index]["node_id"].clone();
        replace_node_field_in_graph(
            &mut candidate,
            index,
            "/body/members/0/target",
            target.clone(),
        );
        replace_node_field_in_graph(
            &mut candidate,
            index,
            "/dependencies",
            json!([target]),
        );
        rekey_stale_nodes(&mut candidate);
        assert_eq!(
            read_package_semantics(&candidate, &catalog),
            Err(ReadRefusal::GraphShape(
                candidate["semantic_graph"]["nodes"][index]["node_id"].clone()
            )),
            "recomputed {label} is rejected"
        );
    }

    let mut wrong_population = base.clone();
    let mut population_reference = wrong_population["semantic_graph"]["nodes"][58].clone();
    population_reference["body"]["members"][0]["target"] =
        wrong_population["semantic_graph"]["nodes"][57]["node_id"].clone();
    population_reference["dependencies"] =
        json!([wrong_population["semantic_graph"]["nodes"][57]["node_id"]]);
    append_rekeyed_structural_node(&mut wrong_population, population_reference);
    let population_reference_id = wrong_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()["node_id"]
        .clone();
    let mut population_set = wrong_population["semantic_graph"]["nodes"][42].clone();
    population_set["body"]["members"][0]["target"] = population_reference_id.clone();
    population_set["dependencies"] = json!([population_reference_id]);
    append_rekeyed_structural_node(&mut wrong_population, population_set);
    let population_set_id = wrong_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()["node_id"]
        .clone();
    let mut population = wrong_population["semantic_graph"]["nodes"][50].clone();
    population["semantic_form"] = json!("model_population");
    population["semantic_type"] = population_set_id.clone();
    population["body"] = json!({
        "term": "aggregate",
        "members": [{
            "name": "max",
            "term": "binding",
            "value": {
                "term": "literal",
                "type": wrong_population["semantic_graph"]["nodes"][16]["node_id"],
                "value": "-1",
                "value_kind": "integer"
            }
        }]
    });
    population["dependencies"] = json!([
        wrong_population["semantic_graph"]["nodes"][16]["node_id"],
        population_set_id
    ]);
    append_rekeyed_structural_node(&mut wrong_population, population);
    let population_id = wrong_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()["node_id"]
        .clone();
    assert_eq!(
        read_package_semantics(&wrong_population, &catalog),
        Err(ReadRefusal::GraphShape(population_id)),
        "recomputed negative population bound is rejected"
    );

    let mut positive_population = wrong_population.clone();
    let positive_index = positive_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .len()
        - 1;
    positive_population["semantic_graph"]["nodes"][positive_index]["body"]["members"][0]["value"]
        ["value"] = json!("3");
    let positive_preimage =
        structural_node_preimage(&positive_population["semantic_graph"]["nodes"][positive_index])
            .unwrap();
    let positive_digest = unqualified_jcs_sha256(&positive_preimage);
    positive_population["semantic_graph"]["nodes"][positive_index]["node_id"]["digest"] =
        json!(positive_digest);
    positive_population["identity_preimage"] = derive_identity_preimage(&positive_population);
    refresh_package_id(&mut positive_population);
    assert_eq!(
        read_package_semantics(&positive_population, &catalog),
        Ok(()),
        "Set<Reference<Model>> population with nonnegative max is admitted"
    );

    let mut arbitrary_set_population = wrong_population.clone();
    let population_index = arbitrary_set_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .len()
        - 1;
    arbitrary_set_population["semantic_graph"]["nodes"][population_index]["body"]["members"][0]["value"]
        ["value"] = json!("3");
    let integer_set = arbitrary_set_population["semantic_graph"]["nodes"][42].clone();
    let integer_id = arbitrary_set_population["semantic_graph"]["nodes"][16]["node_id"].clone();
    let mut integer_set = integer_set;
    integer_set["body"]["members"][0]["target"] = integer_id.clone();
    integer_set["dependencies"] = json!([integer_id.clone()]);
    append_rekeyed_structural_node(&mut arbitrary_set_population, integer_set);
    let integer_set_id = arbitrary_set_population["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()["node_id"]
        .clone();
    arbitrary_set_population["semantic_graph"]["nodes"][population_index]["semantic_type"] =
        integer_set_id.clone();
    arbitrary_set_population["semantic_graph"]["nodes"][population_index]["dependencies"] =
        json!([integer_id]);
    let population_preimage = structural_node_preimage(
        &arbitrary_set_population["semantic_graph"]["nodes"][population_index],
    )
    .unwrap();
    let population_digest = unqualified_jcs_sha256(&population_preimage);
    arbitrary_set_population["semantic_graph"]["nodes"][population_index]["node_id"]["digest"] =
        json!(population_digest);
    arbitrary_set_population["identity_preimage"] =
        derive_identity_preimage(&arbitrary_set_population);
    refresh_package_id(&mut arbitrary_set_population);
    assert_eq!(
        read_package_semantics(&arbitrary_set_population, &catalog),
        Err(ReadRefusal::GraphShape(
            arbitrary_set_population["semantic_graph"]["nodes"][population_index]["node_id"]
                .clone()
        )),
        "population over an arbitrary set is rejected"
    );

    // Use the published nominal roots and the exact FR-094 U2 body. Nominal
    // roots are targetless and carry their dimension key as semantic_type;
    // they are not self-typed structural unit fixtures.
    let mut actual_u2 = parsed_fixture(POSITIVE_NOMINAL_IDENTITIES);
    let metre = actual_u2["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| {
            node["node_id"]["digest"]
                == "79637623a46d29e884b62c6fa292aeb29d41e4ecc4e800b4d7ee910a3eaf23a4"
        })
        .expect("published unit-metre root")
        .clone();
    let mut second = metre.clone();
    second["node_id"]["digest"] =
        json!("d0f0c5d24d6accb2f1d9afff7c6cdda3143df382e50af4bda0704d48c67ed998");
    second["semantic_type"]["digest"] =
        json!("628777f8df3c2700593d56dc20fb110640214dd09bc6cb961d802fdb9b7f6077");
    second["declaration"]["qualified_name"] = json!(["Example", "second"]);
    second["nominal_identity_preimage"]["qualified_declaration"] = json!(["Example", "second"]);
    second["nominal_identity_preimage"]["dimension_node_id"]["digest"] =
        json!("628777f8df3c2700593d56dc20fb110640214dd09bc6cb961d802fdb9b7f6077");
    second["dependencies"] = json!([second["semantic_type"].clone()]);
    actual_u2["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(second.clone());
    let metre_source_map = actual_u2["source_map"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["node_id"] == metre["node_id"])
        .expect("metre declaration has a source-map row")
        .clone();
    let mut second_source_map = metre_source_map.clone();
    second_source_map["node_id"] = second["node_id"].clone();
    actual_u2["source_map"]
        .as_array_mut()
        .unwrap()
        .push(second_source_map);
    actual_u2["identity_preimage"] = derive_identity_preimage(&actual_u2);
    refresh_package_id(&mut actual_u2);

    let metre_id = metre["node_id"].clone();
    let second_id = json!({"domain": "quire.checked-semantic-node/v1", "digest": "d0f0c5d24d6accb2f1d9afff7c6cdda3143df382e50af4bda0704d48c67ed998"});
    let compound = json!({
        "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0000000000000000000000000000000000000000000000000000000000000000"},
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "scalar_type",
        "semantic_form": "compound_unit",
        "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "0000000000000000000000000000000000000000000000000000000000000000"},
        "dependencies": [metre_id, second_id],
        "occurrences": [{"role": "type", "ordinal": 0}],
        "body": {"term": "aggregate", "members": [
            {"term": "aggregate", "members": [
                {"name": "unit", "term": "binding", "value": {"term": "reference", "target": metre["node_id"]}},
                {"name": "exponent", "term": "binding", "value": {"term": "literal", "type": {"domain": "quire.checked-semantic-node/v1", "digest": "07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32"}, "value": "1", "value_kind": "integer"}}
            ]},
            {"term": "aggregate", "members": [
                {"name": "unit", "term": "binding", "value": {"term": "reference", "target": second_id}},
                {"name": "exponent", "term": "binding", "value": {"term": "literal", "type": {"domain": "quire.checked-semantic-node/v1", "digest": "07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32"}, "value": "-1", "value_kind": "integer"}}
            ]}
        ]},
    });
    append_rekeyed_structural_node(&mut actual_u2, compound);
    let actual_u2_id = actual_u2["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()["node_id"]
        .clone();
    let mut compound_source_map = metre_source_map;
    compound_source_map["node_id"] = actual_u2_id.clone();
    compound_source_map["role"] = json!("type");
    actual_u2["source_map"]
        .as_array_mut()
        .unwrap()
        .push(compound_source_map);
    for node in actual_u2["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .take(2)
    {
        let occurrence = &node["occurrences"][0];
        assert!(
            actual_u2["source_map"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| {
                    entry["node_id"] == node["node_id"]
                        && entry["role"] == occurrence["role"]
                        && entry["ordinal"] == occurrence["ordinal"]
                }),
            "every added nominal U2 occurrence has an exact source-map entry"
        );
    }
    assert_eq!(
        actual_u2_id["digest"], "bcf431d15781d66fcdd2514e251004373eb64a3ef21e37f7b2535f2a871821e7",
        "U2 uses the published vector"
    );
    jsonschema::validator_for(&parsed_fixture(SCHEMA))
        .expect("V2 schema is valid")
        .validate(&actual_u2)
        .unwrap_or_else(|error| {
            panic!("nominal-root U2 package fails the full public schema: {error}")
        });
    assert_eq!(
        read_package_semantics(&actual_u2, &catalog),
        Ok(()),
        "canonical U2 is admitted"
    );

    let mut wrong_compound = actual_u2.clone();
    let compound_index = wrong_compound["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .len()
        - 1;
    wrong_compound["semantic_graph"]["nodes"][compound_index]["body"]["members"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let preimage =
        structural_node_preimage(&wrong_compound["semantic_graph"]["nodes"][compound_index])
            .unwrap();
    let digest = unqualified_jcs_sha256(&preimage);
    wrong_compound["semantic_graph"]["nodes"][compound_index]["node_id"]["digest"] = json!(digest);
    let compound_id = wrong_compound["semantic_graph"]["nodes"][compound_index]["node_id"].clone();
    wrong_compound["semantic_graph"]["nodes"][compound_index]["semantic_type"] =
        compound_id.clone();
    wrong_compound["identity_preimage"] = derive_identity_preimage(&wrong_compound);
    refresh_package_id(&mut wrong_compound);
    let compound_id = wrong_compound["semantic_graph"]["nodes"][compound_index]["node_id"].clone();
    assert_eq!(
        read_package_semantics(&wrong_compound, &catalog),
        Err(ReadRefusal::GraphShape(compound_id)),
        "recomputed noncanonical compound-unit order is rejected"
    );

    let mut non_self_typed = wrong_compound.clone();
    let compound_index = non_self_typed["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .len()
        - 1;
    let integer_type = non_self_typed["semantic_graph"]["nodes"][16]["node_id"].clone();
    non_self_typed["semantic_graph"]["nodes"][compound_index]["semantic_type"] =
        integer_type.clone();
    let preimage =
        structural_node_preimage(&non_self_typed["semantic_graph"]["nodes"][compound_index])
            .unwrap();
    let digest = unqualified_jcs_sha256(&preimage);
    non_self_typed["semantic_graph"]["nodes"][compound_index]["node_id"]["digest"] = json!(digest);
    non_self_typed["identity_preimage"] = derive_identity_preimage(&non_self_typed);
    refresh_package_id(&mut non_self_typed);
    assert_eq!(
        read_package_semantics(&non_self_typed, &catalog),
        Err(ReadRefusal::GraphShape(
            non_self_typed["semantic_graph"]["nodes"][compound_index]["node_id"].clone()
        )),
        "a non-self-typed compound unit is rejected"
    );

    let mut non_root = wrong_compound.clone();
    let non_root_index = non_root["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .len()
        - 1;
    non_root["semantic_graph"]["nodes"][non_root_index]["body"]["members"][0]["members"][0]["value"]
        ["target"] = non_root["semantic_graph"]["nodes"][16]["node_id"].clone();
    let preimage =
        structural_node_preimage(&non_root["semantic_graph"]["nodes"][non_root_index]).unwrap();
    let digest = unqualified_jcs_sha256(&preimage);
    non_root["semantic_graph"]["nodes"][non_root_index]["node_id"]["digest"] = json!(digest);
    let non_root_id = non_root["semantic_graph"]["nodes"][non_root_index]["node_id"].clone();
    non_root["semantic_graph"]["nodes"][non_root_index]["semantic_type"] = non_root_id.clone();
    non_root["identity_preimage"] = derive_identity_preimage(&non_root);
    refresh_package_id(&mut non_root);
    assert_eq!(
        read_package_semantics(&non_root, &catalog),
        Err(ReadRefusal::GraphShape(
            non_root["semantic_graph"]["nodes"][non_root_index]["node_id"].clone()
        )),
        "a compound term naming a non-unit root is rejected"
    );
}

/// The recursive-record fixture (`record List { next?: List; }` through an
/// `Option` field and `record Tree { kids: Sequence<Tree>[0, 3]; }` through a
/// bounded collection), as QSL's emitter writes it, is schema-valid, binds its
/// package id, every recursion-group member's key and group label recompute
/// from the members' places in graph order, and the full reader admits it.
#[trace("TC-233", "FR-322-AC-49", "FR-322-AC-50")]
#[test]
fn tc_233_recursive_record_fixture_carries_derived_group_keys() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_RECURSIVE_RECORDS);
    assert!(
        schema.is_valid(&package),
        "the recursive fixture is a V2 package"
    );
    assert_eq!(
        derive_identity_preimage(&package),
        package["identity_preimage"]
    );
    assert_eq!(
        unqualified_jcs_sha256(&package_id_preimage(&package)),
        package["package_id"]["digest"]
            .as_str()
            .expect("package digest")
    );
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let labels = nodes
        .iter()
        .filter_map(|node| node["recursion_group"].as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(labels.len(), 2, "one group for List and one for Tree");
    for label in labels {
        let positions = nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node["recursion_group"] == label)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        assert!(
            positions.windows(2).all(|pair| pair[1] == pair[0] + 1),
            "a group's members are written together"
        );
        let group = positions
            .iter()
            .map(|&index| &nodes[index])
            .collect::<Vec<_>>();
        // The group label is the digest of the members' group-local keys.
        assert_eq!(
            recursion_group_digest(nodes, &group).as_deref(),
            Some(label)
        );
        for (member, node) in group.iter().enumerate() {
            assert_eq!(
                json!(group_member_key(&group, member, label)),
                node["node_id"]["digest"],
                "{} {} keys from its group preimage",
                node["node_tag"],
                node["semantic_form"]
            );
        }
    }
    for node in nodes
        .iter()
        .filter(|node| node.get("recursion_group").is_none())
    {
        assert!(
            !is_application_node(node) && structural_node_key_is_current(node),
            "{} {} is keyed outside every group",
            node["node_tag"],
            node["semantic_form"]
        );
    }
    assert_eq!(
        read_package_semantics(&package, &parsed_fixture(OPERATION_CATALOG)),
        Ok(())
    );
}

/// A recursion group relabelled with a label that is not its group digest,
/// every member rekeyed under that label so that each member's key is the
/// digest of its own group-local preimage, and every reference rewritten, is
/// refused `invalid_package`/`stale-node-key` at the group's lowest node-id
/// digest member: the label must be the recomputed group digest.
#[trace("TC-233", "FR-322-AC-49")]
#[test]
fn tc_233_forged_recursion_group_label_refuses_stale_node_key() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_RECURSIVE_RECORDS);
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let tree = nodes
        .iter()
        .find(|node| node["declaration"]["qualified_name"] == json!(["Tree"]))
        .expect("Tree is declared");
    let label = tree["recursion_group"]
        .as_str()
        .expect("Tree is in a group");
    let forged = unqualified_jcs_sha256(&json!("forged-group-label"));
    let group = recursion_group_members(nodes, label);
    let renames = group
        .iter()
        .enumerate()
        .map(|(member, node)| {
            let old = node["node_id"]["digest"]
                .as_str()
                .expect("node digest")
                .to_owned();
            let new = group_member_key(&group, member, &forged).expect("structural member keys");
            (old, new)
        })
        .collect::<Vec<_>>();
    let mut text = serde_json::to_string(&package).expect("package serializes");
    text = text.replace(label, &forged);
    for (old, new) in &renames {
        text = text.replace(old.as_str(), new.as_str());
    }
    let mut candidate: Value = serde_json::from_str(&text).expect("forged package parses");
    candidate["identity_preimage"] = derive_identity_preimage(&candidate);
    refresh_package_id(&mut candidate);
    assert!(schema.is_valid(&candidate), "the schema admits it");
    let candidate_nodes = candidate["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let forged_group = recursion_group_members(candidate_nodes, &forged);
    assert_eq!(forged_group.len(), group.len());
    for node in &forged_group {
        assert!(
            group_member_key_is_current(candidate_nodes, node),
            "each forged member's key is its own group-local preimage digest"
        );
    }
    let lowest = forged_group
        .iter()
        .map(|node| node["node_id"].clone())
        .min_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()))
        .expect("the group has members");
    assert_eq!(
        read_package_semantics(&candidate, &catalog),
        Err(ReadRefusal::StaleNodeKey(lowest))
    );
}

const POSITIVE_TWO_OWNERS_A: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-two-owners-a.json");
const POSITIVE_TWO_OWNERS_B: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-two-owners-b.json");

/// Every published positive fixture, by name.
fn positive_fixtures() -> [(&'static str, Value); 12] {
    let operation = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    [
        (
            "positive-nominal-identities",
            parsed_fixture(POSITIVE_NOMINAL_IDENTITIES),
        ),
        (
            "positive-all-families",
            parsed_fixture(POSITIVE_ALL_FAMILIES),
        ),
        (
            "positive-clause-operations",
            parsed_fixture(POSITIVE_CLAUSE_OPERATIONS),
        ),
        (
            "positive-control-operations",
            parsed_fixture(POSITIVE_CONTROL_OPERATIONS),
        ),
        ("positive-operation-identities", operation),
        ("positive-union-nodes", parsed_fixture(POSITIVE_UNION_NODES)),
        (
            "positive-recursive-records",
            parsed_fixture(POSITIVE_RECURSIVE_RECORDS),
        ),
        (
            "positive-state-transition",
            parsed_fixture(POSITIVE_STATE_TRANSITION),
        ),
        (
            "positive-extension-nodes",
            parsed_fixture(POSITIVE_EXTENSION_NODES),
        ),
        (
            "positive-two-owners-a",
            parsed_fixture(POSITIVE_TWO_OWNERS_A),
        ),
        (
            "positive-two-owners-b",
            parsed_fixture(POSITIVE_TWO_OWNERS_B),
        ),
        (
            "positive-dependency-library",
            parsed_fixture(POSITIVE_DEPENDENCY_LIBRARY),
        ),
    ]
}

/// The owner kind FR-322's presence rule requires of a node: a structural
/// node (no nominal preimage, no application at its body root) carries a
/// `SourceOwner` when declared, and a `ModelOwner` when it is a model
/// declaration node or a clause function.
fn required_owner_kind(node: &Value) -> Option<&'static str> {
    let structural =
        node.get("nominal_identity_preimage").is_none() && node["body"]["term"] != "application";
    if !structural {
        None
    } else if node.get("declaration").is_some() {
        Some("source")
    } else if node["node_tag"] == "model" && node["semantic_form"] == "model_root" {
        Some("model_root")
    } else if is_model_declaration(node) || node["node_tag"] == "function" {
        Some("model")
    } else {
        None
    }
}

/// Apply `edit` to the node at `index`, keep the node id, and recompute the
/// package id.
fn edit_node(package: &mut Value, index: usize, edit: impl Fn(&mut Value)) {
    edit(
        package
            .pointer_mut(&format!("/semantic_graph/nodes/{index}"))
            .expect("node pointer exists"),
    );
    refresh_package_id(package);
}

fn source_owner(authority: &str, identity: &str) -> Value {
    json!({"kind": "source", "authority": authority, "identity": identity})
}

fn node_index_where(package: &Value, matches: impl Fn(&Value) -> bool) -> usize {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .position(matches)
        .expect("the package holds the node")
}

fn declared_index(package: &Value, name: &[&str]) -> usize {
    node_index_where(package, |node| {
        node["declaration"]["qualified_name"] == json!(name)
    })
}

/// Move the regions of `index`'s `declaration` occurrence to `source`.
fn move_declaration_regions(package: &mut Value, index: usize, source: &Value) {
    let id = package["semantic_graph"]["nodes"][index]["node_id"].clone();
    for entry in package["source_map"]
        .as_array_mut()
        .expect("source map is an array")
        .iter_mut()
        .filter(|entry| entry["node_id"] == id && entry["role"] == "declaration")
    {
        for region in entry["regions"].as_array_mut().expect("regions") {
            region["source"] = source.clone();
        }
    }
}

fn add_lock_source(package: &mut Value, source: &Value) {
    package["lock"]["sources"]
        .as_array_mut()
        .expect("lock sources")
        .push(source.clone());
}

/// FR-322-AC-51: a node carries `owner` exactly when its preimage does; the
/// full reader admits every positive fixture.
#[trace("TC-233", "FR-322-AC-51", "FR-322-AC-50")]
#[test]
fn tc_233_every_positive_fixture_carries_owners_exactly_where_the_rule_requires() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let mut kinds = BTreeSet::new();
    for (name, package) in positive_fixtures() {
        assert!(schema.is_valid(&package), "{name} is a V2 package");
        let nodes = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes");
        for node in nodes {
            assert_eq!(
                node["owner"]["kind"].as_str(),
                required_owner_kind(node),
                "{name}: {} {} carries an owner exactly when the rule requires one",
                node["node_tag"],
                node["semantic_form"]
            );
            kinds.extend(node["owner"]["kind"].as_str().map(str::to_owned));
        }
        assert_eq!(read_package_semantics(&package, &catalog), Ok(()), "{name}");
    }
    assert_eq!(
        kinds,
        BTreeSet::from([
            "model".to_owned(),
            "model_root".to_owned(),
            "source".to_owned()
        ])
    );
}

/// FR-322-AC-51: each owner mutation fails the V2 schema, which a reader
/// reports `malformed_wire`.
#[trace("TC-233", "FR-322-AC-51")]
#[test]
fn tc_233_owner_mutations_fail_the_schema_as_malformed_wire() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let bases = positive_fixtures().into_iter().collect::<BTreeMap<_, _>>();
    let adverse = parsed_fixture(ADVERSE);
    let mutations = adverse["owner_mutations"]
        .as_array()
        .expect("owner mutations");
    assert_eq!(mutations.len(), 10);
    for mutation in mutations {
        let id = mutation["id"].as_str().expect("id");
        let base = &bases[mutation["base"].as_str().expect("base")];
        assert!(schema.is_valid(base), "{id}: the base is admitted");
        let index = node_index_where(base, |node| {
            node["node_tag"] == mutation["node"]["node_tag"]
                && node["semantic_form"] == mutation["node"]["semantic_form"]
        });
        let mut candidate = base.clone();
        edit_node(&mut candidate, index, |node| {
            for patch in mutation["patch"].as_array().expect("patch") {
                apply_authored_patch(node, patch);
            }
        });
        assert_eq!(mutation["outcome"], "refused:malformed_wire", "{id}");
        assert!(!schema.is_valid(&candidate), "{id}: the schema refuses it");
    }
}

/// FR-322-AC-51: a declared record whose owner moves to a second source, with
/// that source in the lock, the declaration regions moved, the node id kept
/// and the package id recomputed, is refused stale-node-key.
#[trace("TC-233", "FR-322-AC-51")]
#[test]
fn tc_233_a_changed_owner_refuses_stale_node_key_at_that_node() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let mut package = parsed_fixture(POSITIVE_TWO_OWNERS_A);
    let other = parsed_fixture(POSITIVE_TWO_OWNERS_B)["lock"]["sources"][0].clone();
    let point = declared_index(&package, &["Point"]);
    let id = package["semantic_graph"]["nodes"][point]["node_id"].clone();
    add_lock_source(&mut package, &other);
    move_declaration_regions(&mut package, point, &other);
    let owner = source_owner("agent-ix", "example-b");
    edit_node(&mut package, point, |node| node["owner"] = owner.clone());
    assert_eq!(package["semantic_graph"]["nodes"][point]["node_id"], id);
    assert_eq!(
        read_package_semantics(&package, &catalog),
        Err(ReadRefusal::StaleNodeKey(id))
    );
}

/// FR-322-AC-52: an owner no lock entry selects refuses missing-selection at
/// its node, ahead of a stale key at a node of lower node-id digest.
#[trace("TC-233", "FR-322-AC-52")]
#[test]
fn tc_233_an_unjoined_owner_refuses_missing_selection_ahead_of_a_stale_key() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let base = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let nodes = base["semantic_graph"]["nodes"].as_array().expect("nodes");
    let digest = |node: &Value| {
        node["node_id"]["digest"]
            .as_str()
            .unwrap_or_default()
            .to_owned()
    };

    let record = declared_index(&base, &["Example", "Point"]);
    let mut source = base.clone();
    let id = nodes[record]["node_id"].clone();
    edit_node(&mut source, record, |node| {
        node["owner"] = source_owner("agent-ix", "missing");
    });
    assert_eq!(
        read_package_semantics(&source, &catalog),
        Err(ReadRefusal::OwnerSelection(id))
    );

    let relationship = node_index_where(&base, |node| node["semantic_form"] == "relationship");
    let mut model = base.clone();
    let id = nodes[relationship]["node_id"].clone();
    edit_node(&mut model, relationship, |node| {
        node["owner"]["identity"] = json!("acme/billing");
    });
    assert_eq!(
        read_package_semantics(&model, &catalog),
        Err(ReadRefusal::OwnerSelection(id))
    );

    // The unjoined node is the owner-bearing node of highest digest, and a
    // body-tampered, owner-free structural node has a lower digest.
    let owned = (0..nodes.len())
        .filter(|&index| nodes[index].get("owner").is_some())
        .max_by_key(|&index| digest(&nodes[index]))
        .expect("an owner-bearing node");
    let stale = (0..nodes.len())
        .filter(|&index| {
            nodes[index].get("owner").is_none()
                && is_rekeyed_structural_node(&nodes[index])
                && nodes[index]["semantic_form"] != "frame"
        })
        .min_by_key(|&index| digest(&nodes[index]))
        .expect("an owner-free structural node");
    assert!(digest(&nodes[stale]) < digest(&nodes[owned]));
    let mut both = base.clone();
    let id = nodes[owned]["node_id"].clone();
    edit_node(&mut both, stale, |node| {
        node["body"] = json!({"term": "aggregate", "members": [{
            "term": "binding", "name": "tampered",
            "value": {"term": "aggregate", "members": []}
        }]});
    });
    edit_node(&mut both, owned, |node| {
        node["owner"] = source_owner("agent-ix", "missing");
    });
    assert_eq!(
        read_package_semantics(&both, &catalog),
        Err(ReadRefusal::OwnerSelection(id)),
        "the join is reported before the stale key of the lower-digest node"
    );
}

/// FR-322-AC-52: a declared node's owner and its source map agree. Moving the
/// declaration region to another lock source changes no node id and not the
/// package id, and is refused invalid-value at that source-map entry.
#[trace("TC-233", "FR-322-AC-52")]
#[test]
fn tc_233_a_moved_declaration_region_refuses_invalid_value_at_its_source_map_entry() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let mut package = parsed_fixture(POSITIVE_TWO_OWNERS_A);
    let package_id = package["package_id"].clone();
    let other = parsed_fixture(POSITIVE_TWO_OWNERS_B)["lock"]["sources"][0].clone();
    let point = declared_index(&package, &["Point"]);
    let id = package["semantic_graph"]["nodes"][point]["node_id"].clone();
    add_lock_source(&mut package, &other);
    move_declaration_regions(&mut package, point, &other);
    assert_eq!(package["package_id"], package_id);
    assert_eq!(
        derive_identity_preimage(&package),
        package["identity_preimage"]
    );
    let entry = package["source_map"]
        .as_array()
        .expect("source map")
        .iter()
        .position(|entry| entry["node_id"] == id && entry["role"] == "declaration")
        .expect("Point's declaration entry");
    assert_eq!(
        read_package_semantics(&package, &catalog),
        Err(ReadRefusal::InvalidOwnerValue(format!(
            "/source_map/{entry}"
        )))
    );
}

/// FR-322-AC-53: the same declarations under two source owners are two
/// packages; owner-keyed nodes differ, content-keyed nodes do not.
#[trace("TC-233", "FR-322-AC-53")]
#[test]
fn tc_233_two_owners_share_content_keyed_nodes_and_differ_on_owner_keyed_ones() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let a = parsed_fixture(POSITIVE_TWO_OWNERS_A);
    let b = parsed_fixture(POSITIVE_TWO_OWNERS_B);
    for (name, package) in [("a", &a), ("b", &b)] {
        assert!(schema.is_valid(package), "{name} is a V2 package");
        assert_eq!(read_package_semantics(package, &catalog), Ok(()), "{name}");
        assert_eq!(
            derive_identity_preimage(package),
            package["identity_preimage"],
            "{name}"
        );
        let nodes = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes");
        let labels = nodes
            .iter()
            .filter_map(|node| node["recursion_group"].as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(labels.len(), 1, "{name}: List is the only group");
        for label in labels {
            let group = recursion_group_members(nodes, label);
            assert_eq!(
                recursion_group_digest(nodes, &group).as_deref(),
                Some(label),
                "{name}"
            );
            for (member, node) in group.iter().enumerate() {
                assert_eq!(
                    group_member_key(&group, member, label).as_deref(),
                    node["node_id"]["digest"].as_str(),
                    "{name}: in-group key recomputes from the package alone"
                );
            }
        }
    }
    let ids = |package: &Value, pick: &dyn Fn(&Value) -> bool| -> Vec<Value> {
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|node| pick(node))
            .map(|node| node["node_id"].clone())
            .collect()
    };
    let point = |node: &Value| node["declaration"]["qualified_name"] == json!(["Point"]);
    let three = |node: &Value| node["declaration"]["qualified_name"] == json!(["three"]);
    let integer = |node: &Value| node["semantic_form"] == "integer";
    let list = |node: &Value| node.get("recursion_group").is_some();
    assert_ne!(ids(&a, &point), ids(&b, &point), "Point ids differ");
    assert_eq!(
        ids(&a, &integer),
        ids(&b, &integer),
        "the builtin Integer node is shared"
    );
    assert_eq!(
        ids(&a, &three),
        ids(&b, &three),
        "the application-keyed three is shared"
    );
    let (list_a, list_b) = (ids(&a, &list), ids(&b, &list));
    assert_eq!(list_a.len(), 2, "the List record and its Option node");
    assert!(
        list_a
            .iter()
            .zip(&list_b)
            .all(|(left, right)| left != right)
    );
    assert_ne!(
        a["semantic_graph"]["nodes"][0]["recursion_group"],
        b["semantic_graph"]["nodes"][0]["recursion_group"]
    );
    assert_ne!(a["package_id"], b["package_id"]);

    // Package a carrying b's owners, with b's source in the lock and the
    // declaration regions moved to it, is refused: every key names a's owner.
    let mut crossed = a.clone();
    let other = b["lock"]["sources"][0].clone();
    add_lock_source(&mut crossed, &other);
    let owned = (0..crossed["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .len())
        .filter(|&index| {
            crossed["semantic_graph"]["nodes"][index]
                .get("owner")
                .is_some()
        })
        .collect::<Vec<_>>();
    for index in owned {
        move_declaration_regions(&mut crossed, index, &other);
        let owner = source_owner("agent-ix", "example-b");
        edit_node(&mut crossed, index, |node| node["owner"] = owner.clone());
    }
    assert!(matches!(
        read_package_semantics(&crossed, &catalog),
        Err(ReadRefusal::StaleNodeKey(_))
    ));
}

/// FR-322-AC-54: the derived-shape key stage leaves the
/// `correspondence`/`abstraction_relation` node to FR-451, whose identity step
/// refuses an edited body as invalid_semantic_graph, not stale-node-key.
#[trace("TC-233", "FR-322-AC-54")]
#[test]
fn tc_233_the_key_stage_leaves_an_abstraction_relation_node_to_fr_451() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert!(schema.is_valid(&package));
    let index = node_index_where(&package, |node| {
        node["semantic_form"] == "abstraction_relation"
    });
    let node = &package["semantic_graph"]["nodes"][index];
    assert!(node.get("owner").is_none());
    assert!(
        !is_rekeyed_structural_node(node),
        "the key stage does not key it"
    );
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));

    let mut edited = package.clone();
    let id = node["node_id"].clone();
    edit_node(&mut edited, index, |node| {
        node["body"]["frames"][0]["function"][1] = json!("attempt_other");
    });
    assert!(schema.is_valid(&edited));
    assert_eq!(edited["semantic_graph"]["nodes"][index]["node_id"], id);
    assert_eq!(
        read_package_semantics(&edited, &catalog),
        Err(ReadRefusal::Abstraction(id))
    );
}

/// FR-440-AC-4's arm clauses through the full reader: a case whose arms are
/// not exactly one per union member in declaration order (swapped, one
/// dropped, one repeated), whose binder aggregate has another arity than the
/// member's payload, whose binder is typed other than its payload position, or
/// whose later arm body has another type than the first, is
/// operator-ineligible.
#[trace("TC-395", "FR-440-AC-4")]
#[test]
fn tc_395_a_case_must_hold_one_arm_per_union_member_in_order() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_UNION_NODES);
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));
    let case = node_index_where(&package, |node| node["semantic_form"] == "case");
    let scrutinee = package["semantic_graph"]["nodes"][case]["body"]["arguments"][0].clone();
    type Edit = fn(&mut Vec<Value>, &Value);
    let edits: [(&str, Edit); 7] = [
        ("arms swapped", |arms, _| arms.swap(1, 2)),
        ("an arm dropped", |arms, _| {
            arms.pop();
        }),
        ("an arm repeated", |arms, _| arms[2] = arms[1].clone()),
        ("a binder missing", |arms, _| {
            arms[2]["value"]["members"][0]["members"]
                .as_array_mut()
                .expect("binders")
                .pop();
        }),
        ("a binder too many", |arms, _| {
            let binder = arms[1]["value"]["members"][0]["members"][0].clone();
            arms[1]["value"]["members"][0]["members"]
                .as_array_mut()
                .expect("binders")
                .push(binder);
        }),
        ("a binder of another type", |arms, scrutinee| {
            arms[1]["value"]["members"][0]["members"][0] = scrutinee.clone();
        }),
        ("a later arm body of another type", |arms, scrutinee| {
            arms[2]["value"]["members"][1] = scrutinee.clone();
        }),
    ];
    for (label, edit) in edits {
        let mut candidate = package.clone();
        let mut arguments = candidate["semantic_graph"]["nodes"][case]["body"]["arguments"]
            .as_array()
            .expect("case arguments")
            .clone();
        edit(&mut arguments, &scrutinee);
        let arguments = Value::Array(arguments);
        edit_node(&mut candidate, case, |node| {
            node["body"]["arguments"] = arguments.clone();
            node["dependencies"] = json!(application_dependencies(&node["body"]));
        });
        rekey_stale_nodes(&mut candidate);
        assert_eq!(
            read_package_semantics(&candidate, &catalog),
            Err(ReadRefusal::Operation(OperationRefusal::OperatorIneligible)),
            "{label}"
        );
    }
}

/// The adverse entry `id` of `list`, with its base package, the index of the
/// node it names and the node's id.
fn adverse_case(list: &str, id: &str) -> (Value, Value, usize, Value) {
    let adverse = parsed_fixture(ADVERSE);
    let case = adverse[list]
        .as_array()
        .expect("adverse list")
        .iter()
        .find(|case| case["id"] == id)
        .unwrap_or_else(|| panic!("{list} holds {id}"))
        .clone();
    let base = positive_fixtures()
        .into_iter()
        .find(|(name, _)| *name == case["base"].as_str().expect("base"))
        .expect("a published base")
        .1;
    let index = node_index_where(&base, |node| {
        node["node_tag"] == case["node"]["node_tag"]
            && node["semantic_form"] == case["node"]["semantic_form"]
            && case["node"]["owner_node"]
                .as_str()
                .is_none_or(|owner| node["owner"]["node"] == owner)
    });
    let id = base["semantic_graph"]["nodes"][index]["node_id"].clone();
    (case, base, index, id)
}

/// FR-322-AC-55 (owner join, clause kind): the published `clause_owner_cases`.
/// A precondition clause function owned by the operation member `Order/size`
/// is admitted by the full reader; a precondition owned by the object type
/// `Order`, an invariant owned by an operation member, a `postcondition` kind
/// (not a clause function's), a clause with no kind and one with two each
/// refuse `missing_declaration`/`missing-selection`
/// at the clause function's node id.
#[trace("TC-233", "FR-322-AC-55")]
#[test]
fn tc_233_clause_function_owners_follow_their_clause_kind() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["clause_owner_cases"].as_array().expect("cases");
    assert_eq!(cases.len(), 7);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let (case, base, index, node_id) = adverse_case("clause_owner_cases", id);
        let mut candidate = base.clone();
        edit_node(&mut candidate, index, |node| {
            for patch in case["patch"].as_array().expect("patch") {
                apply_authored_patch(node, patch);
            }
        });
        let outcome = read_package_semantics(&candidate, &catalog);
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ => {
                assert_eq!(case["expected_code"], "missing_declaration", "{id}");
                assert_eq!(case["expected_cause"], "missing-selection", "{id}");
                assert_eq!(case["expected_locus"], "node_id", "{id}");
                assert_eq!(outcome, Err(ReadRefusal::OwnerSelection(node_id)), "{id}");
            }
        }
    }
}

/// FR-322-AC-100 (one wire graph): each published `shipped_projection_cases`
/// entry ships an `identity_projection` member inside `identity_preimage`, as
/// the occurrence-free graph or as an empty array; the closed V2 schema
/// refuses it, which a reader reports `malformed_wire` at that member.
#[trace("TC-233", "FR-322-AC-100")]
#[test]
fn tc_233_an_identity_preimage_that_ships_a_projection_is_refused() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["shipped_projection_cases"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 2);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        assert_eq!(case["base"], "positive-all-families", "{id}");
        let base = parsed_fixture(POSITIVE_ALL_FAMILIES);
        assert!(schema.is_valid(&base), "{id}: the base is admitted");
        let shipped = match &case["identity_projection"] {
            Value::String(form) if form == "occurrence-free-graph" => {
                occurrence_free_projection(&base)
            }
            literal @ Value::Array(_) => literal.clone(),
            other => panic!("{id}: unknown shipped projection {other}"),
        };
        let mut candidate = base.clone();
        candidate["identity_preimage"]
            .as_object_mut()
            .expect("identity preimage is an object")
            .insert("identity_projection".to_owned(), shipped);
        // The shipped copy disagrees with the lock-only derivation, while the
        // package id rebuilt from the lock and graph alone still names the
        // package: the copy is never part of its identity.
        assert_ne!(
            candidate["identity_preimage"],
            derive_identity_preimage(&candidate),
            "{id}: a shipped projection is not part of the derived identity preimage"
        );
        assert_eq!(
            unqualified_jcs_sha256(&derive_package_id_preimage(&candidate)),
            base["package_id"]["digest"].as_str().expect("digest"),
            "{id}: the package id is rebuilt from the lock and graph, not the shipped copy"
        );
        assert!(!schema.is_valid(&candidate), "{id}: the schema refuses it");
        assert_eq!(case["expected_code"], "malformed_wire", "{id}");
        // The locus names the shipped member: it exists in the candidate and
        // not in the base, and removing it restores the admitted package.
        let locus = case["expected_locus"].as_str().expect("locus");
        assert!(candidate.pointer(locus).is_some(), "{id}: locus names a member of the candidate");
        assert!(base.pointer(locus).is_none(), "{id}: locus names no member of the base");
        let (parent, member) = locus.rsplit_once('/').expect("locus has a parent");
        let mut repaired = candidate.clone();
        repaired
            .pointer_mut(parent)
            .and_then(Value::as_object_mut)
            .expect("locus parent is an object")
            .remove(member);
        assert_eq!(repaired, base, "{id}: removing the locus member restores the base");
        assert!(schema.is_valid(&repaired), "{id}: the repaired package is admitted");
    }
}

/// FR-322 "Owner member schema": a `state`/`state_clause` node (FR-341) is
/// application-keyed and carries no `owner`; the schema refuses one that does.
#[trace("TC-233", "FR-322-AC-51")]
#[test]
fn tc_233_a_state_clause_node_carries_no_owner() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let clauses = (0..package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .len())
        .filter(|&index| {
            package["semantic_graph"]["nodes"][index]["semantic_form"] == "state_clause"
        })
        .collect::<Vec<_>>();
    assert_eq!(clauses.len(), 2);
    for index in clauses {
        assert!(
            package["semantic_graph"]["nodes"][index]
                .get("owner")
                .is_none()
        );
        let mut owned = package.clone();
        edit_node(&mut owned, index, |node| {
            node["owner"] = json!({"kind": "model", "identity": "acme/orders", "node": "ix://acme/orders/Order"});
        });
        assert!(
            !schema.is_valid(&owned),
            "a state_clause node with an owner is refused"
        );
    }
}

/// The JCS SHA-256 of the selected domain package document, the digest every
/// published model selection of `acme/orders` carries.
const DOMAIN_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-orders.json");

/// FR-322-AC-32: each published package that selects `acme/orders` selects the
/// semantic IR document by its own `sha256-jcs` digest, in the lock and in the
/// identity preimage.
#[trace("TC-233", "FR-322-AC-32")]
#[test]
fn tc_233_published_model_selections_name_the_domain_document_by_its_jcs_digest() {
    let digest = unqualified_jcs_sha256(&parsed_fixture(DOMAIN_DOCUMENT));
    let mut selecting = 0;
    for (name, package) in positive_fixtures() {
        for rows in [
            &package["lock"]["model_selections"],
            &package["identity_preimage"]["model_selections"],
        ] {
            for row in rows.as_array().into_iter().flatten() {
                selecting += 1;
                assert_eq!(row["digest"], digest.as_str(), "{name}");
                assert_eq!(row["digest_domain"], "sha256-jcs", "{name}");
            }
        }
    }
    assert_eq!(
        selecting, 10,
        "five published packages select it, twice each"
    );
}

/// FR-322-AC-82 and FR-322-AC-83: the published `population_reference_cases`.
/// A `relation`/`population` node owned by a population the selected document
/// declares is the operand of `quire.op.model.all_instances` and
/// `quire.op.model.lookup`; two populations key and read differently; a
/// population owned by anything else is refused at the owner join, one without
/// an owner fails the V2 schema, and the node is no `collection_kind` operand.
/// The published fixture's operand 0 is a `Population<Order>[2]` binding, which
/// the full reader admits.
#[trace("TC-233", "FR-322-AC-82")]
#[trace("TC-233", "FR-322-AC-83")]
#[test]
fn tc_233_a_population_reference_names_the_declared_population_it_reads() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let preimage_schema = jsonschema::validator_for(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": "#/$defs/ModelDeclarationNode",
        "$defs": parsed_fixture(NODE_PREIMAGE_SCHEMA)["$defs"],
    }))
    .expect("ModelDeclarationNode is a valid schema definition");
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["population_reference_cases"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 15);
    let mut admitted = BTreeMap::<String, (Value, Value, Value)>::new();
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let mut candidate = positive_fixtures()
            .into_iter()
            .find(|(name, _)| *name == case["base"].as_str().expect("base"))
            .expect("a published base")
            .1;
        let operation = case["operation"].as_str().expect("operation");
        let mut population_id = Value::Null;
        if operation != "published" {
            let mut document = parsed_fixture(MODEL_MEMBER_TYPE_VECTORS)["domain_package"].clone();
            document["populations"] = adverse["selected_document_populations"].clone();
            // Each malformed shape alone: a `node` member and no `identity`
            // (the old shape), a `node` member beside an `identity`, an entry
            // with only its `identity` removed.
            match case["document_populations"].as_str() {
                Some("node-only") => {
                    document["populations"] = json!([
                        {"node": "ix://acme/orders/counters"},
                        {"node": "ix://acme/orders/archive"}
                    ]);
                }
                Some("with-node-member") => {
                    document["populations"][0]["node"] = json!("ix://acme/orders/counters");
                }
                Some("without-identity") => {
                    document["populations"][0]
                        .as_object_mut()
                        .expect("a population")
                        .remove("identity");
                }
                _ => {}
            }
            let selection = json!([{
                "identity": "acme/orders",
                "digest_domain": "sha256-jcs",
                "digest": unqualified_jcs_sha256(&document),
            }]);
            candidate["model_bindings"] = json!({"domain_package": document});
            replace_selection_consistently(&mut candidate, "/model_selections", selection.clone());
            // The selected package changed, so its root joins the new row.
            for root in model_root_positions(&candidate) {
                candidate["semantic_graph"]["nodes"][root]["owner"]["digest"] =
                    selection[0]["digest"].clone();
                rekey_model_root(&mut candidate, root);
            }
            let placeholder =
                json!({"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)});
            let mut node = json!({
                "node_id": placeholder,
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "relation",
                "semantic_form": "population",
                "semantic_type": placeholder,
                "dependencies": [],
                "occurrences": [{"ordinal": 0, "role": "generated"}],
                "body": {"term": "aggregate", "members": []},
            });
            if let Some(owner_node) = case["owner_node"].as_str() {
                node["owner"] =
                    json!({"kind": "model", "identity": "acme/orders", "node": owner_node});
            }
            append_rekeyed_structural_node(&mut candidate, node);
            population_id = candidate["semantic_graph"]["nodes"]
                .as_array()
                .and_then(|nodes| nodes.last())
                .expect("appended")["node_id"]
                .clone();
            let application = node_index_where(&candidate, |node| {
                node["body"]["operation"]["identity"] == format!("quire.op.{operation}").as_str()
            });
            let as_declaration = case["member_declaration"] == true;
            let as_link = case["member_population"] == true;
            edit_node(&mut candidate, application, |node| {
                if as_declaration {
                    node["body"]["operation"]["member"]["declaration"] = population_id.clone();
                } else if as_link {
                    node["body"]["operation"]["member"]["population"] = population_id.clone();
                } else {
                    node["body"]["arguments"][0]["target"] = population_id.clone();
                }
                node["dependencies"] = json!(application_dependencies(&node["body"]));
            });
            if let Some(also) = case["also"].as_str() {
                let second = node_index_where(&candidate, |node| {
                    node["body"]["operation"]["identity"] == format!("quire.op.{also}").as_str()
                });
                edit_node(&mut candidate, second, |node| {
                    node["body"]["arguments"][0]["target"] = population_id.clone();
                    node["dependencies"] = json!(application_dependencies(&node["body"]));
                });
            }
            rekey_stale_nodes(&mut candidate);
        }
        // `model_bindings` is a test-only field outside the closed wire shape.
        let mut wire = candidate.clone();
        wire.as_object_mut()
            .expect("a package")
            .remove("model_bindings");
        if case["expected_code"] == "malformed_wire" {
            assert!(!v2.is_valid(&wire), "{id}: the node needs its owner");
            continue;
        }
        assert!(v2.is_valid(&wire), "{id}: validates against the V2 schema");
        let outcome = read_package_semantics(&candidate, &catalog);
        if case["outcome"] == "admitted" {
            assert_eq!(outcome, Ok(()), "{id}");
            if operation == "published" {
                continue;
            }
            let population = &candidate["semantic_graph"]["nodes"]
                [node_index_where(&candidate, |node| node["node_id"] == population_id)];
            assert_eq!(population["semantic_type"], population["node_id"], "{id}");
            let owner = &population["owner"];
            let preimage = json!({
                "version": "quire.structural-node/v1",
                "node_tag": "relation",
                "semantic_form": "population",
                "semantic_type": null,
                "declaration": null,
                "recursion": null,
                "owner": owner,
                "body": {"term": "aggregate", "members": []},
            });
            assert!(preimage_schema.is_valid(&preimage), "{id}: closed preimage");
            assert_eq!(
                population["node_id"]["digest"],
                unqualified_jcs_sha256(&preimage).as_str(),
                "{id}: the key is the digest of the declaration-key preimage"
            );
            let application =
                &candidate["semantic_graph"]["nodes"][node_index_where(&candidate, |node| {
                    node["body"]["operation"]["identity"]
                        == format!("quire.op.{operation}").as_str()
                })];
            assert!(
                application["dependencies"]
                    .as_array()
                    .is_some_and(|dependencies| dependencies.contains(&population_id)),
                "{id}: the application depends on the population node"
            );
            if let Some(also) = case["also"].as_str() {
                // One package where both operations read the one population.
                let reading = candidate["semantic_graph"]["nodes"]
                    .as_array()
                    .expect("nodes")
                    .iter()
                    .filter(|node| {
                        node["body"]["arguments"][0]["target"] == population_id
                            && [operation, also].iter().any(|name| {
                                node["body"]["operation"]["identity"]
                                    == format!("quire.op.{name}").as_str()
                            })
                    })
                    .count();
                assert_eq!(reading, 2, "{id}: both operations read the one node");
                let population_nodes = candidate["semantic_graph"]["nodes"]
                    .as_array()
                    .expect("nodes")
                    .iter()
                    .filter(|node| node["semantic_form"] == "population")
                    .count();
                assert_eq!(population_nodes, 1, "{id}: one population is one node");
            }
            admitted.insert(
                format!(
                    "{}/{operation}",
                    case["population"].as_str().expect("population")
                ),
                (
                    population_id.clone(),
                    application["node_id"].clone(),
                    candidate["package_id"].clone(),
                ),
            );
        } else {
            let (code, cause) = match outcome {
                Err(ReadRefusal::OwnerSelection(node)) => {
                    assert_eq!(node, population_id, "{id}: refused at the population node");
                    ("missing_declaration", "missing-selection")
                }
                Err(ReadRefusal::Operation(refusal)) => (refusal.code(), refusal.cause()),
                // FR-154 step 1 refuses the document before any graph check.
                Err(ReadRefusal::PopulationRecord(locus)) => {
                    assert_eq!(locus, "/lock/model_selections/0", "{id}: the selection row");
                    ("invalid_model_binding", "malformed-declaration")
                }
                other => panic!("{id}: unexpected {other:?}"),
            };
            assert_eq!(case["expected_code"], code, "{id}");
            assert_eq!(case["expected_cause"], cause, "{id}");
        }
    }
    let counters = &admitted["counters/model.all_instances"];
    let archive = &admitted["archive/model.all_instances"];
    assert_ne!(counters.0, archive.0, "two populations key differently");
    assert_ne!(counters.1, archive.1, "so do the reads of them");
    assert_ne!(counters.2, archive.2, "and the packages that hold them");
}

/// FR-322-AC-56 (relationship ends): the published `relationship_end_cases`.
/// A relationship the document declares without a `targetEnd.role` names one
/// end, its `sourceEnd.role`; a `relationship_end` naming it is admitted by the
/// full reader, and one naming the missing inverse refuses
/// `missing_declaration`/`missing-name`.
#[trace("TC-233", "FR-322-AC-56")]
#[test]
fn tc_233_a_relationship_end_names_only_the_ends_its_relationship_declares() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["relationship_end_cases"].as_array().expect("cases");
    assert_eq!(cases.len(), 4);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let mut candidate = positive_fixtures()
            .into_iter()
            .find(|(name, _)| *name == case["base"].as_str().expect("base"))
            .expect("a published base")
            .1;
        let relationship = node_index_where(&candidate, |node| {
            node["node_tag"] == "relation" && node["semantic_form"] == "relationship"
        });
        edit_node(&mut candidate, relationship, |node| {
            node["owner"]["node"] = case["relationship_owner_node"].clone();
        });
        let applications = (0..candidate["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .len())
            .filter(|&index| {
                candidate["semantic_graph"]["nodes"][index]["body"]["operation"]["member"]["kind"]
                    == "relationship_end"
            })
            .collect::<Vec<_>>();
        assert!(!applications.is_empty(), "{id}");
        for index in applications {
            edit_node(&mut candidate, index, |node| {
                node["body"]["operation"]["member"]["name"] = case["member_name"].clone();
            });
        }
        rekey_stale_nodes(&mut candidate);
        let outcome = read_package_semantics(&candidate, &catalog);
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ => {
                let refusal =
                    OperationRefusal::from_cause(case["expected_cause"].as_str().expect("cause"))
                        .expect("a published operation cause");
                assert_eq!(case["expected_code"], refusal.code(), "{id}");
                assert_eq!(outcome, Err(ReadRefusal::Operation(refusal)), "{id}");
            }
        }
    }
}

/// Append the nodes of `Population<T>[max]` for the model declaration node
/// `object`: its `Reference<T>`, `Set<Reference<T>>` and the
/// `bounded_domain`/`model_population` node, each cloned from a node of the
/// package and rekeyed. Returns the population node's id.
fn append_population_chain(package: &mut Value, object: &Value, max: Option<&str>) -> Value {
    let template = |tag: &str, form: &str| {
        let index = node_index_where(package, |node| {
            node["node_tag"] == tag && node["semantic_form"] == form
        });
        package["semantic_graph"]["nodes"][index].clone()
    };
    let integer = template("scalar_type", "integer")["node_id"].clone();
    let mut reference = template("composite_type", "reference");
    let mut set = template("composite_type", "set");
    let mut population = template("bounded_domain", "integer_range");
    reference["body"]["members"][0]["target"] = object.clone();
    reference["dependencies"] = json!([object]);
    let last_id = |package: &Value| {
        package["semantic_graph"]["nodes"]
            .as_array()
            .and_then(|nodes| nodes.last())
            .expect("package nodes are not empty")["node_id"]
            .clone()
    };
    append_rekeyed_structural_node(package, reference);
    let reference_id = last_id(package);
    set["body"]["members"][0]["target"] = reference_id.clone();
    set["dependencies"] = json!([reference_id]);
    append_rekeyed_structural_node(package, set);
    let set_id = last_id(package);
    population["semantic_form"] = json!("model_population");
    population["semantic_type"] = set_id.clone();
    let members = max.map_or_else(Vec::new, |max| {
        vec![json!({
            "name": "max",
            "term": "binding",
            "value": {
                "term": "literal",
                "type": integer,
                "value": max,
                "value_kind": "integer"
            }
        })]
    });
    population["body"] = json!({"term": "aggregate", "members": members});
    let mut dependencies = vec![set_id];
    if max.is_some() {
        dependencies.push(integer);
    }
    dependencies.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
    population["dependencies"] = json!(dependencies);
    append_rekeyed_structural_node(package, population);
    last_id(package)
}

/// FR-322-AC-74 (unbounded population node): the published
/// `unbounded_population_cases`. A `model_population` node with no `max`
/// member is the unbounded population and is admitted by the full reader, and
/// the bounded node with `max` `3` is unchanged; a `max` that is negative or
/// not an integer, and a node with a second member beside `max`, are still
/// refused. Each node is reached as the `population` of the `Order-order-Order`
/// `reaches` application, which the reader walks as in AC-73.
#[trace("TC-233", "FR-322-AC-74")]
#[test]
fn tc_233_a_model_population_with_no_max_is_the_unbounded_population() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["unbounded_population_cases"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 5);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let mut candidate = positive_fixtures()
            .into_iter()
            .find(|(name, _)| *name == case["base"].as_str().expect("base"))
            .expect("a published base")
            .1;
        let application = node_index_where(&candidate, |node| {
            node["body"]["operation"]["member"]["kind"] == "relationship_end"
                && node["body"]["operation"]["identity"] == "quire.op.model.reaches"
        });
        let order = node_index_where(&candidate, |node| {
            node["node_tag"] == "model" && node["owner"]["node"] == "ix://acme/orders/Order"
        });
        let object = candidate["semantic_graph"]["nodes"][order]["node_id"].clone();
        let link = append_population_chain(&mut candidate, &object, case["max"].as_str());
        if case["extra_member"] == true {
            let index = node_index_where(&candidate, |node| node["node_id"] == link);
            let extra = json!({
                "name": "min",
                "term": "binding",
                "value": {
                    "term": "literal",
                    "type": candidate["semantic_graph"]["nodes"][index]["body"]["members"][0]
                        ["value"]["type"],
                    "value": "0",
                    "value_kind": "integer"
                }
            });
            edit_node(&mut candidate, index, |node| {
                node["body"]["members"]
                    .as_array_mut()
                    .expect("members")
                    .push(extra.clone());
            });
        }
        edit_node(&mut candidate, application, |node| {
            node["body"]["operation"]["member"]["population"] = link.clone();
            node["dependencies"] = json!(application_dependencies(&node["body"]));
        });
        rekey_stale_nodes(&mut candidate);
        assert!(
            v2.is_valid(&candidate),
            "{id}: validates against the V2 schema"
        );
        // The fixture's own `Population<Order>[2]` binding type precedes the
        // appended chain, whose population node is the last one.
        let population = candidate["semantic_graph"]["nodes"]
            .as_array()
            .and_then(|nodes| {
                nodes
                    .iter()
                    .rfind(|node| node["semantic_form"] == "model_population")
            })
            .expect("the appended population node");
        assert_eq!(
            population["body"]["members"].as_array().map(Vec::len),
            Some(usize::from(case["max"].is_string()) + usize::from(case["extra_member"] == true)),
            "{id}: the node carries exactly the members the case names"
        );
        let outcome = read_package_semantics(&candidate, &catalog);
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ => {
                // The derived-shape refusal is `GraphShape` at the node it names.
                assert_eq!(
                    (
                        case["expected_code"].as_str(),
                        case["expected_cause"].as_str()
                    ),
                    (Some("invalid_package"), Some("invalid-derived-shape")),
                    "{id}: the fixture records the derived-shape code and cause"
                );
                assert_eq!(
                    outcome,
                    Err(ReadRefusal::GraphShape(population["node_id"].clone())),
                    "{id}: refused at the population node"
                );
            }
        }
    }
}

/// FR-322-AC-74: the hand-written RFC 8785 preimages of the bounded population
/// PO1 and the unbounded population PO0 recompute to the published keys. PO0
/// is PO1's preimage with the `max` member removed.
#[trace("TC-233", "FR-322-AC-74")]
#[test]
fn tc_233_unbounded_population_preimage_vectors() {
    const PO1: &str = r#"{"body":{"members":[{"name":"max","term":"binding","value":{"term":"literal","type":{"digest":"07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32","domain":"quire.checked-semantic-node/v1"},"value":"3","value_kind":"integer"}}],"term":"aggregate"},"declaration":null,"node_tag":"bounded_domain","recursion":null,"semantic_form":"model_population","semantic_type":{"digest":"b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5","domain":"quire.checked-semantic-node/v1"},"version":"quire.structural-node/v1"}"#;
    const PO_MAX_0: &str = r#"{"body":{"members":[{"name":"max","term":"binding","value":{"term":"literal","type":{"digest":"07f6dca966d22bde13d3bb198f12610e57d8e1e04d0476bbab03f405d2b04e32","domain":"quire.checked-semantic-node/v1"},"value":"0","value_kind":"integer"}}],"term":"aggregate"},"declaration":null,"node_tag":"bounded_domain","recursion":null,"semantic_form":"model_population","semantic_type":{"digest":"b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5","domain":"quire.checked-semantic-node/v1"},"version":"quire.structural-node/v1"}"#;
    const PO0: &str = r#"{"body":{"members":[],"term":"aggregate"},"declaration":null,"node_tag":"bounded_domain","recursion":null,"semantic_form":"model_population","semantic_type":{"digest":"b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5","domain":"quire.checked-semantic-node/v1"},"version":"quire.structural-node/v1"}"#;
    let schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let mut keys = Vec::new();
    for (name, source, key) in [
        (
            "PO1",
            PO1,
            "859d6b01a132a14711a8a4568e24a7ab54e3255b78bbcc62b14e4293dbf71f9a",
        ),
        (
            "PO0",
            PO0,
            "f65a8239f4fbc1b6e75bfeed4a578e1c219a27f155a82ddfb7aeeb22657c3b8b",
        ),
        (
            "PO_MAX_0",
            PO_MAX_0,
            "0ade8ed9baf129e82dee28096d64f5a0deabca6f3aa06874a4a7e76bb9688e34",
        ),
    ] {
        let preimage = parsed_fixture(source);
        assert_eq!(unqualified_jcs_sha256(&preimage), key, "{name}: key");
        let node_id = json!({"domain": "quire.checked-semantic-node/v1", "digest": key});
        let wire = json!({
            "node_id": node_id,
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": preimage["node_tag"],
            "semantic_form": preimage["semantic_form"],
            "semantic_type": preimage["semantic_type"],
            "dependencies": [],
            "occurrences": [{"role": "type", "ordinal": 0}],
            "body": preimage["body"]
        });
        assert!(schema.is_valid(&wire), "{name}: wire node schema");
        keys.push(key);
    }
    assert_ne!(keys[0], keys[1], "bounded and unbounded keys differ");
    assert_ne!(
        keys[1], keys[2],
        "the unbounded population is not the population with max 0"
    );
    assert_ne!(
        keys[1], "b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5",
        "the unbounded population is not its Set<Reference<Order>>"
    );
}

/// FR-322-AC-73 (population link of a `reaches`): the published
/// `reaches_population_cases`. A `quire.op.model.reaches` application whose
/// `relationship_end` member names a `model_population` node over the type of
/// its role's end is admitted, for a `max` of `3` and of `0`; the key is part
/// of the application's key and its dependencies; a `population` that is an
/// `integer_range` node, an absent node or a population over another model
/// declaration refuses `ill_typed`/`operator-ineligible`; a dependencies list
/// without the population node is `invalid_semantic_graph`. The application
/// without the key keeps the member, and the node id, of the published fixture.
#[trace("TC-233", "FR-322-AC-73")]
#[test]
fn tc_233_a_reaches_population_link_names_a_population_over_its_role_end_type() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["reaches_population_cases"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 7);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let mut candidate = positive_fixtures()
            .into_iter()
            .find(|(name, _)| *name == case["base"].as_str().expect("base"))
            .expect("a published base")
            .1;
        let published = candidate.clone();
        let application = node_index_where(&candidate, |node| {
            node["body"]["operation"]["member"]["kind"] == "relationship_end"
                && node["body"]["operation"]["identity"] == "quire.op.model.reaches"
        });
        let population = &case["population"];
        let link = if population.is_null() {
            None
        } else if population["absent"] == true {
            Some(json!({"domain": "quire.checked-semantic-node/v1", "digest": "f".repeat(64)}))
        } else if let Some(owner_node) = population["model_population_over"].as_str() {
            let order = node_index_where(&candidate, |node| {
                node["node_tag"] == "model" && node["owner"]["node"] == "ix://acme/orders/Order"
            });
            let object = if owner_node == "ix://acme/orders/Order" {
                candidate["semantic_graph"]["nodes"][order]["node_id"].clone()
            } else {
                let mut other = candidate["semantic_graph"]["nodes"][order].clone();
                other["owner"]["node"] = json!(owner_node);
                append_rekeyed_structural_node(&mut candidate, other);
                candidate["semantic_graph"]["nodes"]
                    .as_array()
                    .and_then(|nodes| nodes.last())
                    .expect("appended")["node_id"]
                    .clone()
            };
            Some(append_population_chain(
                &mut candidate,
                &object,
                Some(population["max"].as_str().expect("max")),
            ))
        } else {
            let target = node_index_where(&candidate, |node| {
                node["node_tag"] == population["node_tag"]
                    && node["semantic_form"] == population["semantic_form"]
            });
            Some(candidate["semantic_graph"]["nodes"][target]["node_id"].clone())
        };
        if let Some(link) = &link {
            let without_dependency = case["dependencies"] == "without-population";
            edit_node(&mut candidate, application, |node| {
                node["body"]["operation"]["member"]["population"] = link.clone();
                node["dependencies"] = json!(application_dependencies(&node["body"]));
            });
            rekey_stale_nodes(&mut candidate);
            let member =
                &candidate["semantic_graph"]["nodes"][application]["body"]["operation"]["member"];
            assert!(member["population"].is_object(), "{id}");
            assert!(
                v2.is_valid(&candidate),
                "{id}: the member with a population validates against the V2 schema"
            );
            if without_dependency {
                let kept = application_dependencies(
                    &published["semantic_graph"]["nodes"][application]["body"],
                );
                edit_node(&mut candidate, application, |node| {
                    node["dependencies"] = json!(kept);
                });
            }
        } else {
            assert_eq!(
                published["semantic_graph"]["nodes"][application]["body"]["operation"]["member"]
                    .as_object()
                    .map(|member| member.keys().cloned().collect::<BTreeSet<_>>()),
                Some(BTreeSet::from([
                    "declaration".to_owned(),
                    "kind".to_owned(),
                    "name".to_owned()
                ])),
                "{id}: the published member names exactly kind, declaration and name"
            );
        }
        let outcome = read_package_semantics(&candidate, &catalog);
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ if case["expected_code"] == "invalid_semantic_graph" => assert!(
                matches!(outcome, Err(ReadRefusal::GraphShape(_))),
                "{id}: {outcome:?}"
            ),
            _ => {
                let refusal =
                    OperationRefusal::from_cause(case["expected_cause"].as_str().expect("cause"))
                        .expect("a published operation cause");
                assert_eq!(case["expected_code"], refusal.code(), "{id}");
                assert_eq!(outcome, Err(ReadRefusal::Operation(refusal)), "{id}");
            }
        }
    }
}

/// A package of the native scalar types, two operand values of each, and
/// a Boolean result type, whose lock selects the text law.
fn scalar_equality_package(catalog: &Value) -> Value {
    let mut types = vec![
        walk_scalar("String", "string"),
        walk_scalar("UUID", "uuid"),
        walk_scalar("Timestamp", "timestamp"),
        walk_scalar("Integer", "integer"),
        walk_scalar("F32", "float32"),
        walk_scalar("F64", "float64"),
        walk_text("Short", "nfc"),
        walk_scalar("Bool", "boolean"),
    ];
    for (prefix, ty) in [
        ("s", "String"),
        ("u", "UUID"),
        ("t", "Timestamp"),
        ("x", "Short"),
    ] {
        for side in ["1", "2"] {
            types.push(json!({
                "node_id": walk_id(&format!("{prefix}{side}")),
                "node_tag": "value",
                "semantic_form": "parameter",
                "semantic_type": walk_id(ty),
                "body": {"term": "aggregate", "members": []}
            }));
        }
    }
    json!({
        "lock": {"profile_selections": [], "definition_selections": [walk_text_law(catalog)]},
        "semantic_graph": {"nodes": types}
    })
}

fn scalar_application(identity: &str, laws: Value, mode: Value, left: &str, right: &str) -> Value {
    json!({
        "term": "application",
        "operator": "binary",
        "operation": {"identity": identity, "laws": laws, "mode": mode, "member": null, "leaves": []},
        "result_type": walk_id("Bool"),
        "arguments": [walk_ref(left), walk_ref(right)]
    })
}

fn scalar_literal(kind: &str, ty: &str, value: &str) -> Value {
    json!({"term": "literal", "type": walk_id(ty), "value_kind": kind, "value": value})
}

fn collect_value_kind_enums<'a>(schema: &'a Value, found: &mut Vec<&'a Value>) {
    match schema {
        Value::Object(map) => {
            for (key, child) in map {
                if key == "value_kind" && child.get("enum").is_some() {
                    found.push(&child["enum"]);
                } else {
                    collect_value_kind_enums(child, found);
                }
            }
        }
        Value::Array(items) => items
            .iter()
            .for_each(|child| collect_value_kind_enums(child, found)),
        _ => {}
    }
}

/// SE-01 to SE-06 and SE-09: both schemas enumerate the six identities and the
/// three literal kinds, the catalog defines each identity as a plain binary
/// boolean comparison, and each identity over two operands of its family is admitted.
#[trace("FR-322-AC-67", "TC-233")]
#[test]
fn tc_233_scalar_equality_identities_and_literal_kinds_are_enumerated_and_admitted() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = scalar_equality_package(&catalog);
    let oracle = OperationOracle::for_package(&catalog, &package);
    for (family, prefix) in [("string", "s"), ("uuid", "u"), ("timestamp", "t")] {
        for operation in ["eq", "ne"] {
            let identity = format!("quire.op.{family}.{operation}");
            for schema in [parsed_fixture(SCHEMA), parsed_fixture(NODE_PREIMAGE_SCHEMA)] {
                assert!(
                    string_set(&schema["$defs"]["OperationIdentity"]["enum"], "identities")
                        .contains(identity.as_str()),
                    "{identity} is enumerated"
                );
            }
            let entry = catalog["operations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["identity"] == identity.as_str())
                .unwrap_or_else(|| panic!("{identity} is catalogued"));
            assert_eq!(entry["operands"], json!([family, family]));
            assert_eq!(entry["result"], "boolean");
            assert_eq!(entry["laws"], json!([]));
            assert_eq!(entry["mode"], Value::Null);
            assert_eq!(
                oracle
                    .classify(&scalar_application(
                        &identity,
                        json!([]),
                        Value::Null,
                        &format!("{prefix}1"),
                        &format!("{prefix}2"),
                    ))
                    .map(|result| result.family),
                Ok("boolean".to_owned()),
                "{identity} over two {family} operands is admitted"
            );
        }
    }
    for schema in [parsed_fixture(SCHEMA), parsed_fixture(NODE_PREIMAGE_SCHEMA)] {
        let mut enums = vec![];
        collect_value_kind_enums(&schema, &mut enums);
        assert!(!enums.is_empty());
        for kinds in enums {
            let kinds = string_set(kinds, "value kinds");
            for kind in ["string", "uuid", "timestamp"] {
                assert!(kinds.contains(kind), "{kind} is a literal value_kind");
            }
        }
    }
    for (kind, ty, value) in [
        ("string", "String", "a\u{e9}"),
        ("uuid", "UUID", "00112233-4455-6677-8899-aabbccddeeff"),
        ("timestamp", "Timestamp", "-1"),
    ] {
        let literal = scalar_literal(kind, ty, value);
        assert!(
            oracle
                .argument_descriptor(&literal)
                .is_ok_and(|d| d.is_some()),
            "a {kind} literal typed by its own family is admitted"
        );
    }
}

/// FR-322 "Scalar literal values" (FR-322-AC-114): a `rational` literal's
/// `value` is its exact value in the canonical magnitude spelling, and any
/// other spelling of the same value is refused `invalid-value`.
#[trace("FR-322-AC-114", "TC-233")]
#[test]
fn tc_233_rational_literals_spell_one_canonical_magnitude() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let mut package = scalar_equality_package(&catalog);
    package["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(walk_scalar("Ratio", "rational"));
    let oracle = OperationOracle::for_package(&catalog, &package);
    for value in ["0", "1", "-3", "1/2", "-7/2"] {
        assert!(
            oracle
                .argument_descriptor(&scalar_literal("rational", "Ratio", value))
                .is_ok_and(|d| d.is_some_and(|d| d.family == "rational")),
            "a rational literal {value} is admitted"
        );
    }
    for value in ["2/2", "1/1", "02", "+1", "2/4", "1/0", "0/5", "1/-2", "2.5", ""] {
        assert_eq!(
            oracle
                .argument_descriptor(&scalar_literal("rational", "Ratio", value))
                .err(),
            Some(OperationRefusal::InvalidValue),
            "the rational spelling {value:?} is not canonical"
        );
    }
}

/// QL-01 to QL-04 (FR-322-AC-79): a `quantity` literal typed by a unit node is
/// admitted, both schemas enumerate the kind, and a literal typed by another
/// family or carrying a non-canonical magnitude is refused by exactly its step.
#[trace("FR-322-AC-79", "TC-233")]
#[test]
fn tc_233_quantity_literals_are_typed_by_a_unit_node_and_spell_canonical_magnitudes() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let mut package = scalar_equality_package(&catalog);
    package["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(walk_scalar("Ms", "unit"));
    let oracle = OperationOracle::for_package(&catalog, &package);
    for schema in [parsed_fixture(SCHEMA), parsed_fixture(NODE_PREIMAGE_SCHEMA)] {
        let mut enums = vec![];
        collect_value_kind_enums(&schema, &mut enums);
        assert!(!enums.is_empty());
        for kinds in enums {
            assert!(string_set(kinds, "value kinds").contains("quantity"));
        }
    }
    for value in ["3", "0", "-5", "1/1000", "-7/2"] {
        let literal = scalar_literal("quantity", "Ms", value);
        assert!(
            oracle.argument_descriptor(&literal).is_ok_and(|d| d.is_some_and(|d| d.family == "quantity")),
            "QL: a quantity literal {value} typed by a unit node is admitted"
        );
    }
    for value in ["03", "+3", "2/4", "4/2", "1/0", "0/5", "3/1", "1/-2", " 3", "2.5", ""] {
        assert_eq!(
            oracle.argument_descriptor(&scalar_literal("quantity", "Ms", value)).err(),
            Some(OperationRefusal::InvalidValue),
            "QL: the magnitude {value:?} is not canonical"
        );
    }
    for ty in ["Integer", "String", "Bool"] {
        assert!(
            oracle
                .argument_descriptor(&scalar_literal("quantity", ty, "3"))
                .is_ok_and(|d| d.is_none()),
            "QL: a quantity literal typed by {ty} is outside the kind's family"
        );
    }
    assert!(
        oracle
            .argument_descriptor(&scalar_literal("integer", "Ms", "3"))
            .is_ok_and(|d| d.is_none()),
        "QL: an integer literal typed by a unit node is outside the kind's family"
    );
}

/// SE-07, SE-08 and SE-11 to SE-13: a cross-family operand pair, an
/// ordering or arithmetic identity over these families, a stray `text_profile`
/// law and a stray `text_profile` mode are each refused by exactly their step.
#[trace("FR-322-AC-67", "TC-233")]
#[test]
fn tc_233_scalar_equality_refuses_cross_family_laws_modes_ordering_and_arithmetic() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = scalar_equality_package(&catalog);
    let oracle = OperationOracle::for_package(&catalog, &package);
    let refusal = |identity: &str, laws: Value, mode: Value, left: &str, right: &str| {
        oracle
            .classify(&scalar_application(identity, laws, mode, left, right))
            .err()
    };
    for (identity, left, right) in [
        ("quire.op.string.eq", "s1", "x1"),
        ("quire.op.uuid.eq", "u1", "s1"),
        ("quire.op.string.ne", "s1", "u1"),
        ("quire.op.timestamp.eq", "t1", "u1"),
        ("quire.op.integer.lt", "u1", "u2"),
        ("quire.op.integer.add", "u1", "u2"),
        ("quire.op.integer.lt", "t1", "t2"),
        ("quire.op.integer.add", "s1", "s2"),
    ] {
        assert_eq!(
            refusal(identity, json!([]), Value::Null, left, right),
            Some(OperationRefusal::OperatorIneligible),
            "{identity} over {left} and {right} refuses ill_typed/operator-ineligible"
        );
    }
    assert_eq!(
        refusal(
            "quire.op.text.eq",
            json!([{"role": "text_profile", "definition": walk_text_law(&catalog)}]),
            json!({"kind": "text_profile", "value": "nfc"}),
            "s1",
            "s2"
        ),
        Some(OperationRefusal::ModeTypeMismatch),
        "a string operand of quire.op.text.eq pins no text profile and refuses operation-mode-type-mismatch"
    );
    assert_eq!(
        refusal(
            "quire.op.string.eq",
            json!([{"role": "text_profile", "definition": walk_text_law(&catalog)}]),
            Value::Null,
            "s1",
            "s2"
        ),
        Some(OperationRefusal::LawMismatch),
        "a text_profile law on string equality refuses operation-law-mismatch"
    );
    assert_eq!(
        refusal(
            "quire.op.uuid.eq",
            json!([]),
            json!({"kind": "text_profile", "value": "nfc"}),
            "u1",
            "u2"
        ),
        Some(OperationRefusal::ModeMismatch),
        "a text_profile mode on uuid equality refuses operation-mode-mismatch"
    );
    assert_eq!(OperationRefusal::LawMismatch.code(), "invalid_package");
    assert_eq!(OperationRefusal::ModeMismatch.code(), "invalid_package");
    assert_eq!(OperationRefusal::OperatorIneligible.code(), "ill_typed");
}

/// FR-478-AC-3: every text and string numeric conversion identity is
/// catalogued, admitted with its target member, and refuses the wrong member
/// family or a text profile law on a String operand.
#[trace("FR-322-AC-17", "FR-478-AC-3", "TC-233")]
#[test]
fn tc_233_text_number_operations_are_catalogued_and_checked() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let expected_text_identities: Vec<String> = [
        "integer", "decimal", "rational", "float32", "float64", "quantity",
    ]
    .map(|target| format!("quire.op.text.to_{target}"))
    .into();
    let expected_string_identities: Vec<String> = [
        "integer", "decimal", "rational", "float32", "float64", "quantity",
    ]
    .map(|target| format!("quire.op.string.to_{target}"))
    .into();
    for schema in [parsed_fixture(SCHEMA), parsed_fixture(NODE_PREIMAGE_SCHEMA)] {
        let identities: Vec<String> = schema["$defs"]["OperationIdentity"]["enum"]
            .as_array()
            .expect("operation identities array")
            .iter()
            .map(|identity| identity.as_str().expect("an identity").to_owned())
            .collect();
        let text_anchor = identities
            .iter()
            .position(|identity| identity == "quire.op.text.size")
            .expect("text.size is enumerated");
        let string_anchor = identities
            .iter()
            .position(|identity| identity == "quire.op.string.ne")
            .expect("string.ne is enumerated");
        assert_eq!(
            &identities[text_anchor + 1..text_anchor + 7],
            expected_text_identities
        );
        assert_eq!(
            &identities[string_anchor + 1..string_anchor + 7],
            expected_string_identities
        );
    }
    let mut package = scalar_equality_package(&catalog);
    package["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes array")
        .extend([
            walk_scalar("Decimal", "decimal"),
            walk_scalar("Rational", "rational"),
            walk_scalar("Ms", "unit"),
        ]);
    package["lock"]["definition_selections"]
        .as_array_mut()
        .expect("definition selections")
        .push(walk_catalog_law(&catalog, "ieee_profile"));
    let oracle = OperationOracle::for_package(&catalog, &package);
    let targets = [
        ("integer", "Integer", None),
        ("decimal", "Decimal", Some("rounding")),
        ("rational", "Rational", None),
        ("float32", "F32", Some("rounding")),
        ("float64", "F64", Some("rounding")),
        ("quantity", "Ms", Some("rounding")),
    ];
    let application = |identity: &str, source: &str, target: &str, law_roles: &[&str]| {
        let laws = law_roles
            .iter()
            .map(|role| json!({"role": role, "definition": walk_catalog_law(&catalog, role)}))
            .collect::<Vec<_>>();
        json!({
            "term": "application",
            "operator": "convert",
            "operation": {
                "identity": identity,
                "laws": laws,
                "mode": if identity.contains("to_decimal") || identity.contains("to_float") || identity.contains("to_quantity") {
                    json!({"kind": "rounding", "value": "exact"})
                } else {
                    Value::Null
                },
                "member": {"kind": "type_argument", "declaration": walk_id(target)},
                "leaves": []
            },
            "result_type": walk_id(target),
            "arguments": [walk_ref(source)]
        })
    };

    for source in ["text", "string"] {
        let source_node = if source == "text" { "x1" } else { "s1" };
        for (target, target_node, mode) in targets {
            let identity = format!("quire.op.{source}.to_{target}");
            let mut law_roles = Vec::new();
            if source == "text" {
                law_roles.push("text_profile");
            }
            if matches!(target, "float32" | "float64") {
                law_roles.push("ieee_profile");
            }
            let term = application(&identity, source_node, target_node, &law_roles);
            assert_eq!(
                oracle.classify(&term).map(|descriptor| descriptor.family),
                Ok(target.to_owned()),
                "{identity} is admitted"
            );
            assert_eq!(
                mode,
                catalog["operations"]
                    .as_array()
                    .expect("operations array")
                    .iter()
                    .find(|entry| entry["identity"] == identity)
                    .and_then(|entry| entry["mode"].as_str()),
                "{identity} pins its target mode"
            );

            if source == "string" {
                let mut with_text_law = term;
                with_text_law["operation"]["laws"]
                    .as_array_mut()
                    .expect("laws array")
                    .push(json!({
                        "role": "text_profile",
                        "definition": walk_catalog_law(&catalog, "text_profile")
                    }));
                assert_eq!(
                    oracle.classify(&with_text_law),
                    Err(OperationRefusal::LawMismatch),
                    "{identity} refuses a text_profile law"
                );
            }
        }
    }

    let wrong_member = application("quire.op.text.to_integer", "x1", "Bool", &["text_profile"]);
    assert_eq!(
        oracle.classify(&wrong_member),
        Err(OperationRefusal::OperatorIneligible),
        "a boolean type_argument violates the integer member_family"
    );
}

/// SE-10 and SE-14 to SE-18: a literal whose type is not its kind's family
/// refuses `operator-ineligible`, and a non-canonical `uuid`, `timestamp` or
/// `string` value refuses `invalid_package`/`invalid-value`.
#[trace("FR-322-AC-67", "TC-233")]
#[test]
fn tc_233_scalar_literals_need_their_family_and_canonical_value() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = scalar_equality_package(&catalog);
    let oracle = OperationOracle::for_package(&catalog, &package);
    let uuid = "00112233-4455-6677-8899-aabbccddeeff";
    let literal_refusal = |kind: &str, ty: &str, value: &str| {
        oracle
            .argument_descriptor(&scalar_literal(kind, ty, value))
            .map(|descriptor| descriptor.is_some())
    };
    for (kind, ty, value) in [
        ("uuid", "String", uuid),
        ("string", "UUID", "abc"),
        ("string", "Short", "abc"),
        ("timestamp", "Integer", "0"),
        ("timestamp", "UUID", "0"),
        ("uuid", "Timestamp", uuid),
    ] {
        assert_eq!(
            literal_refusal(kind, ty, value),
            Ok(false),
            "a {kind} literal typed {ty} is not admitted by the literal rule"
        );
    }
    // A literal failing both checks reports the family refusal first.
    for (kind, ty, value) in [
        ("uuid", "String", "00112233-4455-6677-8899-AABBCCDDEEFF"),
        ("timestamp", "Integer", "01"),
    ] {
        assert_eq!(
            literal_refusal(kind, ty, value),
            Ok(false),
            "a {kind} literal typed {ty} with a non-canonical value reports the family refusal"
        );
    }
    // A string literal whose value is not a JSON string is not canonical.
    for value in [json!(7), json!(null), json!(["a"])] {
        let literal = json!({"term": "literal", "type": walk_id("String"), "value_kind": "string", "value": value});
        assert_eq!(
            oracle.argument_descriptor(&literal).map(|d| d.is_some()),
            Err(OperationRefusal::InvalidValue),
            "string literal value {value} is not a JSON string"
        );
    }
    for (kind, ty, value) in [
        ("uuid", "UUID", "00112233-4455-6677-8899-AABBCCDDEEFF"),
        ("uuid", "UUID", "00112233445566778899aabbccddeeff"),
        ("uuid", "UUID", "00112233-4455-6677-8899-aabbccddeef"),
        ("timestamp", "Timestamp", "-0"),
        ("timestamp", "Timestamp", "+1"),
        ("timestamp", "Timestamp", "01"),
        ("timestamp", "Timestamp", ""),
        (
            "timestamp",
            "Timestamp",
            "170141183460469231731687303715884105728",
        ),
    ] {
        assert_eq!(
            literal_refusal(kind, ty, value),
            Err(OperationRefusal::InvalidValue),
            "{kind} value {value:?} is not canonical"
        );
    }
    for (kind, ty, value) in [
        ("uuid", "UUID", uuid),
        ("timestamp", "Timestamp", "0"),
        (
            "timestamp",
            "Timestamp",
            "-170141183460469231731687303715884105728",
        ),
        (
            "timestamp",
            "Timestamp",
            "170141183460469231731687303715884105727",
        ),
        ("string", "String", ""),
        ("string", "String", "Caf\u{e9} and cafe\u{301}"),
    ] {
        assert_eq!(
            literal_refusal(kind, ty, value),
            Ok(true),
            "{kind} value {value:?} is canonical"
        );
    }
    assert_eq!(OperationRefusal::InvalidValue.code(), "invalid_package");
    assert_eq!(OperationRefusal::InvalidValue.cause(), "invalid-value");
}

/// FB-01 to FB-04 (FR-322-AC-108): a `float32_bits` literal spells 8 and a
/// `float64_bits` literal 16 lowercase hexadecimal digits with no prefix; any
/// other spelling refuses `invalid-value` after the family check.
#[trace("FR-322-AC-108", "TC-233")]
#[test]
fn tc_233_float_bits_literals_spell_fixed_width_lowercase_hex() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = scalar_equality_package(&catalog);
    let oracle = OperationOracle::for_package(&catalog, &package);
    let literal_refusal = |kind: &str, ty: &str, value: &Value| {
        oracle
            .argument_descriptor(&json!({"term": "literal", "type": walk_id(ty), "value_kind": kind, "value": value}))
            .map(|descriptor| descriptor.is_some())
    };
    for (kind, ty, values) in [
        ("float32_bits", "F32", ["3f000000", "00000000", "80000000", "7fc00000"]),
        ("float64_bits", "F64", ["3fe0000000000000", "0000000000000000", "8000000000000000", "7ff8000000000001"]),
    ] {
        for value in values {
            assert_eq!(literal_refusal(kind, ty, &json!(value)), Ok(true), "FB-01: {kind} {value}");
        }
    }
    for (kind, ty, values) in [
        (
            "float32_bits",
            "F32",
            vec![json!("3F000000"), json!("0x3f000000"), json!("3f00000"), json!("3f0000000"), json!("0.5"), json!(1056964608)],
        ),
        (
            "float64_bits",
            "F64",
            vec![
                json!("3FE0000000000000"),
                json!("0x3fe0000000000000"),
                json!("3fe000000000000"),
                json!("3fe00000000000000"),
                json!("0.5"),
                json!(4602678819172646912u64),
            ],
        ),
    ] {
        for value in values {
            assert_eq!(
                literal_refusal(kind, ty, &value),
                Err(OperationRefusal::InvalidValue),
                "FB-02/FB-03: {kind} value {value} is not canonical"
            );
        }
    }
    for (kind, ty, value) in [("float32_bits", "F64", "3F000000"), ("float64_bits", "String", "0.5")] {
        assert_eq!(
            literal_refusal(kind, ty, &json!(value)),
            Ok(false),
            "FB-04: a {kind} literal typed {ty} reports the family refusal first"
        );
    }
    let mut spelled = 0;
    for name in std::fs::read_dir("proposals/checked-package-v2/fixtures").unwrap() {
        let fixture: Value = serde_json::from_str(&std::fs::read_to_string(name.unwrap().path()).unwrap()).unwrap();
        let mut stack = vec![&fixture];
        while let Some(value) = stack.pop() {
            match value {
                Value::Object(map) => {
                    if map.get("term") == Some(&json!("literal")) && map["value_kind"].as_str().is_some_and(|k| k.starts_with("float")) {
                        assert!(literal_value_is_canonical(value), "fixture float literal {value}");
                        spelled += 1;
                    }
                    stack.extend(map.values());
                }
                Value::Array(items) => stack.extend(items),
                _ => {}
            }
        }
    }
    assert!(spelled > 0, "the fixtures carry float bit literals");
}

/// VT-01 to VT-04 (FR-322-AC-71): a `model`/`variant_type` node is a model
/// declaration node, as an `event_type` node is. With a `ModelOwner` and no
/// `declaration` the V2 schema admits it; with no `owner`, or with a
/// `SourceOwner` and no `declaration`, it fails the schema. The closed
/// `ModelDeclarationNode` preimage admits the form too.
#[trace("TC-940", "FR-322-AC-71")]
#[test]
fn tc_940_variant_type_is_a_model_declaration_node_with_a_model_owner() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let base = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let index = node_index_where(&base, |node| {
        node["node_tag"] == "model" && node["semantic_form"] == "record_value_type"
    });
    let model_owner =
        json!({"kind": "model", "identity": "acme/orders", "node": "ix://acme/orders/Tier"});
    let with = |edit: &dyn Fn(&mut Value)| {
        let mut candidate = base.clone();
        edit_node(&mut candidate, index, |node| {
            node["semantic_form"] = json!("variant_type");
            node.as_object_mut().expect("node").remove("declaration");
            if node.get("occurrences").is_some() {
                node["occurrences"] = json!([{"ordinal": 0, "role": "generated"}]);
            }
            node["semantic_type"] = node["node_id"].clone();
            node["owner"] = model_owner.clone();
            edit(node);
        });
        candidate
    };

    assert!(
        schema.is_valid(&with(&|_| {})),
        "VT-01: a model-owned variant_type node is a V2 node"
    );
    for (name, edit) in [
        (
            "VT-02",
            (&|node: &mut Value| {
                node.as_object_mut().expect("node").remove("owner");
            }) as &dyn Fn(&mut Value),
        ),
        ("VT-03", &|node: &mut Value| {
            node["owner"] =
                json!({"kind": "source", "authority": "agent-ix", "identity": "example"});
        }),
    ] {
        assert!(
            !schema.is_valid(&with(edit)),
            "{name}: the V2 schema refuses it"
        );
    }

    let preimage_schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-identity preimage schema is valid");
    let preimage = json!({
        "version": "quire.structural-node/v1",
        "node_tag": "model",
        "semantic_form": "variant_type",
        "semantic_type": null,
        "declaration": null,
        "recursion": null,
        "owner": model_owner,
        "body": {"term": "aggregate", "members": []}
    });
    assert!(
        preimage_schema.is_valid(&preimage),
        "VT-04: the ModelDeclarationNode preimage admits variant_type"
    );
}

/// ET-01 to ET-04 (FR-322-AC-68): a `model`/`event_type` node is a model
/// declaration node. With a `ModelOwner` and no `declaration` the V2 schema
/// admits it; with no `owner`, with a `SourceOwner` and no `declaration`, or
/// with a `declaration` beside a `ModelOwner` it fails the schema. The closed
/// `ModelDeclarationNode` preimage admits the form too.
#[trace("TC-233", "FR-322-AC-68")]
#[test]
fn tc_233_event_type_is_a_model_declaration_node_with_a_model_owner() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let base = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let index = node_index_where(&base, |node| {
        node["node_tag"] == "model" && node["semantic_form"] == "record_value_type"
    });
    let model_owner =
        json!({"kind": "model", "identity": "acme/orders", "node": "ix://acme/orders/OrderPlaced"});
    let with = |edit: &dyn Fn(&mut Value)| {
        let mut candidate = base.clone();
        edit_node(&mut candidate, index, |node| {
            node["semantic_form"] = json!("event_type");
            node.as_object_mut().expect("node").remove("declaration");
            node["occurrences"] = json!([{"ordinal": 0, "role": "generated"}]);
            node["semantic_type"] = node["node_id"].clone();
            node["owner"] = model_owner.clone();
            edit(node);
        });
        candidate
    };

    let admitted = with(&|_| {});
    assert!(
        schema.is_valid(&admitted),
        "ET-01: a model-owned event_type node is a V2 node"
    );
    for (name, edit) in [
        (
            "ET-02",
            (&|node: &mut Value| {
                node.as_object_mut().expect("node").remove("owner");
            }) as &dyn Fn(&mut Value),
        ),
        ("ET-03", &|node: &mut Value| {
            node["owner"] =
                json!({"kind": "source", "authority": "agent-ix", "identity": "example"});
        }),
        ("ET-04", &|node: &mut Value| {
            node["declaration"] = json!({"qualified_name": ["Example", "OrderPlaced"]});
        }),
    ] {
        assert!(
            !schema.is_valid(&with(edit)),
            "{name}: the V2 schema refuses it"
        );
    }

    let preimage_schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node-identity preimage schema is valid");
    for form in ["record_value_type", "event_type"] {
        let preimage = json!({
            "version": "quire.structural-node/v1",
            "node_tag": "model",
            "semantic_form": form,
            "semantic_type": null,
            "declaration": null,
            "recursion": null,
            "owner": model_owner,
            "body": {"term": "aggregate", "members": []}
        });
        assert!(
            preimage_schema.is_valid(&preimage),
            "the ModelDeclarationNode preimage admits {form}"
        );
    }
}

const MODEL_OWNER_FORM_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-owner-form-vectors.json");

/// Rekey every `correspondence`/`abstraction_relation` node (FR-451), which
/// `rekey_stale_nodes` leaves alone: its key is the digest of
/// `{version, body}` and its `dependencies` are the digest-ascending targets
/// of its body.
fn rekey_abstraction_nodes(package: &mut Value) {
    let count = package["semantic_graph"]["nodes"]
        .as_array()
        .map_or(0, Vec::len);
    for index in 0..count {
        let node = package["semantic_graph"]["nodes"][index].clone();
        if node["node_tag"] != "correspondence" || node["semantic_form"] != "abstraction_relation" {
            continue;
        }
        let body = &node["body"];
        let mut dependencies = [
            ("objects", "type"),
            ("populations", "population"),
            ("frames", "context"),
        ]
        .into_iter()
        .flat_map(|(list, member)| {
            body[list]
                .as_array()
                .into_iter()
                .flatten()
                .map(move |entry| entry[member].clone())
        })
        .collect::<Vec<_>>();
        dependencies.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
        dependencies.dedup();
        package["semantic_graph"]["nodes"][index]["dependencies"] = json!(dependencies);
        let key = unqualified_jcs_sha256(&json!({
            "version": "quire.abstraction-relation-node/v1",
            "body": body,
        }));
        let old = node["node_id"]["digest"]
            .as_str()
            .expect("digest")
            .to_owned();
        let text = serde_json::to_string(package)
            .expect("package serializes")
            .replace(&old, &key);
        *package = serde_json::from_str(&text).expect("rekeyed package parses");
    }
    package["identity_preimage"] = derive_identity_preimage(package);
    refresh_package_id(package);
}

/// The clause-operations fixture with the form of its declaring type's node
/// changed from `object_type` to `form`, and every key that depends on it
/// rekeyed. Returns the package and each original digest's final digest.
fn clause_package_owned_by(form: &str) -> (Value, BTreeMap<String, String>) {
    let mut package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let index = node_index(&package, &clause_digest("object_type"));
    package["semantic_graph"]["nodes"][index]["semantic_form"] = json!(form);
    let renames = rekey_stale_nodes(&mut package);
    rekey_abstraction_nodes(&mut package);
    (package, renames)
}

/// FR-340-AC-12, FR-341-AC-11, FR-342-AC-7 and FR-342-AC-8: the declaring type
/// of the published clause package's operation is an object type, a process or
/// a state machine in turn, and each package is admitted by the full reader;
/// the frame and the anchor are typed by the declaring node, the invariant is
/// anchored at it, and `self` is the reference over it.
#[trace("TC-233", "FR-340-AC-12")]
#[trace("TC-233", "FR-341-AC-11")]
#[trace("TC-233", "FR-342-AC-7")]
#[trace("TC-233", "FR-342-AC-8")]
#[test]
fn tc_233_a_process_or_state_machine_declares_the_anchor_frame_and_clauses_of_its_operations() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    for form in ["object_type", "process", "state_machine"] {
        let (package, renames) = clause_package_owned_by(form);
        assert_eq!(
            read_package_semantics(&package, &catalog),
            Ok(()),
            "a {form} owner is admitted by the full reader"
        );
        let declaring = renamed_locus(&renames, &node_ref_digest(&clause_digest("object_type")));
        let nodes = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes");
        let of_form = |semantic_form: &str| {
            nodes
                .iter()
                .find(|node| node["node_tag"] == "state" && node["semantic_form"] == semantic_form)
                .unwrap_or_else(|| panic!("a {semantic_form} node is published"))
        };
        assert_eq!(
            of_form("frame")["semantic_type"],
            declaring,
            "{form}: frame"
        );
        let anchor = of_form("operation_anchor");
        assert_eq!(anchor["semantic_type"], declaring, "{form}: anchor");
        assert_eq!(anchor_binding(anchor, "context")["target"], declaring);
        let invariant = &nodes[clause_node_index(&package, "invariant")];
        assert_eq!(
            invariant["body"]["arguments"][1]["target"], declaring,
            "{form}: the invariant is anchored at the declaring node"
        );
        let parameter = node_by_digest(
            nodes,
            invariant["body"]["arguments"][0]["members"][0]["target"]["digest"]
                .as_str()
                .expect("self is a reference"),
        )
        .expect("the self parameter is published");
        let self_type = node_by_digest(
            nodes,
            parameter["semantic_type"]["digest"]
                .as_str()
                .expect("self is typed"),
        )
        .expect("the type of self is published");
        assert_eq!(self_type["semantic_form"], "reference");
        assert_eq!(self_type["body"]["members"][0]["target"], declaring);
    }
}

/// FR-342-AC-9 and FR-340-AC-12: a declaring node that is a record value type,
/// an event type, a persistence interface or a systems interface is no context.
/// As a frame's `semantic_type` it refuses at the frame step, and as the
/// `context` of an anchor over a valid process frame it refuses at the anchor
/// step, `invalid_model_binding`/`malformed-declaration` at the target.
#[trace("TC-233", "FR-340-AC-12")]
#[trace("TC-233", "FR-342-AC-9")]
#[test]
fn tc_233_a_declaring_node_that_is_none_of_the_three_is_no_context() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    for form in [
        "record_value_type",
        "event_type",
        "persistence_interface",
        "systems_interface",
    ] {
        // As a frame's `semantic_type`: a node of that form is appended to the
        // all-families package and the published frame is typed by it.
        let mut frame_owned = parsed_fixture(POSITIVE_ALL_FAMILIES);
        let mut declaring = frame_owned["semantic_graph"]["nodes"]
            [node_index(&frame_owned, &all_families_digest("process"))]
        .clone();
        declaring["semantic_form"] = json!(form);
        declaring["declaration"]["qualified_name"] = json!(["Example", "NotAContext"]);
        append_rekeyed_structural_node(&mut frame_owned, declaring);
        let declaring_id = frame_owned["semantic_graph"]["nodes"]
            .as_array()
            .and_then(|nodes| nodes.last())
            .expect("the appended node")["node_id"]
            .clone();
        let frame = node_index_where(&frame_owned, |node| node["semantic_form"] == "frame");
        let frame_node = &mut frame_owned["semantic_graph"]["nodes"][frame];
        frame_node["semantic_type"] = declaring_id.clone();
        frame_node["body"]["modifies"] = json!([]);
        frame_node["body"]["creates"] = json!([]);
        frame_node["body"]["deletes"] = json!([]);
        frame_node["dependencies"] = json!([]);
        rekey_stale_nodes(&mut frame_owned);
        assert_eq!(
            read_package_semantics(&frame_owned, &catalog),
            Err(ReadRefusal::Frame(
                FrameRefusal::MalformedDeclaration,
                declaring_id
            )),
            "a frame typed by a {form} refuses at the frame step"
        );

        let (mut package, _) = clause_package_owned_by("process");
        let mut other = package["semantic_graph"]["nodes"]
            [node_index_where(&package, |node| node["semantic_form"] == "process")]
        .clone();
        other["semantic_form"] = json!(form);
        other["declaration"]["qualified_name"] = json!(["Config", "NotAContext"]);
        append_rekeyed_structural_node(&mut package, other);
        let other_id = package["semantic_graph"]["nodes"]
            .as_array()
            .and_then(|nodes| nodes.last())
            .expect("the appended node")["node_id"]
            .clone();
        let anchor = node_index_where(&package, |node| node["semantic_form"] == "operation_anchor");
        let node = &mut package["semantic_graph"]["nodes"][anchor];
        node["body"]["members"][0]["value"]["target"] = other_id.clone();
        let mut dependencies = vec![
            other_id.clone(),
            node["body"]["members"][2]["value"]["target"].clone(),
        ];
        dependencies.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
        node["dependencies"] = json!(dependencies);
        rekey_stale_nodes(&mut package);
        assert_eq!(
            read_package_semantics(&package, &catalog),
            Err(ReadRefusal::State(
                StateRefusal::MalformedDeclaration,
                other_id
            )),
            "an anchor whose context is a {form} refuses at the anchor step"
        );
    }
}

/// FR-340-AC-19 and FR-340-AC-20 over `receiver_frame_cases`: the frame of an
/// operation of an object type admits the receiver forms, the frame of a
/// process or a state machine refuses each with the locus the vector names
/// (the entry's declaring node for a field entry, the frame node for a
/// `deletes_scoped` entry) and admits the other scoped forms.
#[trace("TC-233", "FR-340-AC-19")]
#[trace("TC-233", "FR-340-AC-20")]
#[test]
fn tc_233_a_process_or_state_machine_frame_holds_no_receiver_scoped_member() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let vectors = parsed_fixture(MODEL_OWNER_FORM_VECTORS);
    let cases = vectors["receiver_frame_cases"].as_array().expect("cases");
    assert!(
        cases.len() >= 10,
        "the vectors cover every owner form and entry"
    );
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let mut package = parsed_fixture(POSITIVE_ALL_FAMILIES);
        let owner_form = case["owner_form"].as_str().expect("owner form");
        let owner_digest = match owner_form {
            "object_type" => all_families_digest("account"),
            _ => all_families_digest("process"),
        };
        if owner_form == "state_machine" {
            let index = node_index(&package, &owner_digest);
            package["semantic_graph"]["nodes"][index]["semantic_form"] = json!("state_machine");
        }
        let frame = node_index_where(&package, |node| node["semantic_form"] == "frame");
        let frame_id = package["semantic_graph"]["nodes"][frame]["node_id"].clone();
        let owner = node_ref_digest(&owner_digest);
        let body = &mut package["semantic_graph"]["nodes"][frame]["body"];
        let entry = &case["entry"];
        body["modifies"] = json!([]);
        body["creates"] = json!([]);
        body["deletes"] = json!([]);
        body["deletes_scoped"] = json!([]);
        match case["member"].as_str().expect("member") {
            "modifies" => {
                body["modifies"] = json!([{
                    "kind": "field",
                    "declaration": owner,
                    "name": "balance",
                    "scope": entry["scope"],
                }]);
            }
            "deletes_scoped" => body["deletes_scoped"] = json!([entry]),
            other => panic!("{name}: unknown member {other}"),
        }
        package["semantic_graph"]["nodes"][frame]["semantic_type"] = owner.clone();
        package["semantic_graph"]["nodes"][frame]["dependencies"] = json!([owner]);
        let renames = rekey_stale_nodes(&mut package);
        let decided = match read_package_semantics(&package, &catalog) {
            Ok(()) => json!({"admitted": true}),
            Err(ReadRefusal::Frame(refusal, locus)) => {
                let named = renamed_locus(&renames, &owner);
                let framed = renamed_locus(&renames, &frame_id);
                let at = if locus == named {
                    "declaration"
                } else if locus["digest"]
                    == package["semantic_graph"]["nodes"][frame]["node_id"]["digest"]
                    || locus == framed
                {
                    "frame"
                } else {
                    "elsewhere"
                };
                json!({"refused": {"code": refusal.code(), "cause": refusal.cause(), "locus": at}})
            }
            other => panic!("{name}: unexpected {other:?}"),
        };
        assert_eq!(decided, case["expected"], "{name}");
    }
}

/// FR-340-AC-17 state step: every `parameter` entry of a frame's
/// `deletes_scoped` names a reference parameter of the operation its anchor
/// resolves, else `missing_declaration`/`missing-name` at the frame node; a
/// `receiver` entry needs no parameter.
#[trace("TC-233", "FR-340-AC-17")]
#[test]
fn tc_233_deletes_scoped_parameter_entries_name_a_reference_parameter() {
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let cases = vectors["deletes_scoped_parameter_cases"]
        .as_array()
        .expect("cases");
    assert!(cases.len() >= 5, "the vectors cover each outcome");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let mut package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
        let mut bindings = clause_model_bindings(0);
        bindings["domain_package"]["object_types"][0]["operations"][0]["parameters"] = json!([{
            "name": case["parameter"]["name"],
            "typeRef": case["parameter"]["typeRef"],
            "multiplicity": {"lower": "1", "upper": "1", "ordered": false, "unique": true},
        }]);
        package["model_bindings"] = bindings;
        let frame = node_index(&package, &clause_digest("frame"));
        package["semantic_graph"]["nodes"][frame]["body"]["deletes_scoped"] =
            case["entries"].clone();
        let renames = rekey_stale_nodes(&mut package);
        let frame_id = renamed_locus(&renames, &node_ref_digest(&clause_digest("frame")));
        // The frame step admits the body; the anchor join then decides the
        // parameter entries. The precondition clause is not read here: the
        // test operation gains a parameter that the published clause does not
        // bind.
        assert_eq!(
            package_frame_refusal(&package),
            Ok(()),
            "{name}: frame step"
        );
        let nodes = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes");
        let anchor = nodes
            .iter()
            .find(|node| node["node_tag"] == "state" && node["semantic_form"] == "operation_anchor")
            .expect("an anchor is published");
        let decided = match anchor_refusal(anchor, nodes, model_index(&package).as_ref()) {
            Ok(()) => json!({"admitted": true}),
            Err((refusal, locus)) => {
                assert_eq!(locus, frame_id, "{name}: locus is the frame node");
                json!({"refused": {"code": refusal.code(), "cause": refusal.cause(), "locus": "frame"}})
            }
        };
        assert_eq!(decided, case["expected"], "{name}");
    }
}

/// FR-322-AC-84: supply is checked entry by entry in lock order, presence
/// then binding, and the first failing entry is the only one reported.
#[trace("TC-233", "FR-322-AC-84")]
#[test]
fn tc_233_supply_refuses_the_first_failing_entry_in_lock_order() {
    let vectors = parsed_fixture(DEPENDENCY_SELECTION_VECTORS);
    for vector in vectors["supply_vectors"].as_array().expect("supply vectors") {
        let id = vector["id"].as_str().expect("id");
        let mut outcome = "admitted".to_owned();
        for (index, entry) in vector["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .enumerate()
        {
            let refusal = match entry["supply"].as_str().expect("supply") {
                "absent" => Some("missing_import/missing-selection"),
                "stale" => Some("stale_dependency/content-mismatch"),
                _ => None,
            };
            if let Some(refusal) = refusal {
                outcome = format!("refused:{refusal}@/lock/dependency_selections/{index}");
                break;
            }
        }
        let expected = match vector["outcome"].as_str().expect("outcome") {
            "admitted" => "admitted".to_owned(),
            refused => format!("{refused}@{}", vector["locus"].as_str().expect("locus")),
        };
        assert_eq!(outcome, expected, "{id}");
    }
}

/// FR-370-AC-13: argument 1 (`over`) of a clause application is always
/// present as a reference; removing it or replacing it by `null` fails the
/// published schema.
#[trace("TC-303", "FR-370-AC-13")]
#[test]
fn tc_303_a_clause_application_has_no_absent_or_null_over_argument() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert!(
        schema.is_valid(&package),
        "the published clause package is admitted"
    );
    let index = node_with_operation(&package, "quire.op.temporal.clause");
    let arguments = format!("/semantic_graph/nodes/{index}/body/arguments");
    let mut removed = package.clone();
    removed
        .pointer_mut(&arguments)
        .and_then(Value::as_array_mut)
        .expect("arguments")
        .remove(0);
    assert!(!schema.is_valid(&removed), "an absent over argument fails");
    let mut nulled = package.clone();
    *nulled.pointer_mut(&format!("{arguments}/0")).expect("over") = Value::Null;
    assert!(!schema.is_valid(&nulled), "a null over argument fails");
    // A non-reference term in argument 1 (the clause's own clock text literal)
    // fails the schema: it is the one change that leaves the argument count and
    // every other argument intact, so only the argument-1 shape rejects it.
    let mut clock = package.clone();
    let literal = clock
        .pointer(&format!("{arguments}/1"))
        .expect("clock")
        .clone();
    *clock.pointer_mut(&format!("{arguments}/0")).expect("over") = literal;
    assert!(
        !schema.is_valid(&clock),
        "a non-reference over argument fails"
    );
}

const REFINEMENT_STEP_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/refinement-step-occurrence-vectors.json");

/// FR-322 "Refinement step occurrences": the refusal pointer over a
/// refinement owner's `source_map` entries, or `None` when admitted.
fn refinement_step_refusal(case: &Value) -> Option<String> {
    let refinement = &case["refinement"];
    let source_map = case["source_map"].as_array().expect("source map");
    let owner = source_map
        .iter()
        .find(|entry| {
            entry["role"] == "declaration"
                && entry["regions"] == json!([refinement["declaration"]])
        })
        .or_else(|| {
            source_map
                .iter()
                .find(|entry| entry["node_id"] == refinement["node_id"])
        })
        .map(|entry| &entry["node_id"]);
    let Some(owner) = owner else {
        return Some("/source_map".to_owned());
    };
    // A hidden field: an abstract field of the entry's object type with no field row.
    let hidden = refinement["objects"]
        .as_array()
        .expect("objects")
        .iter()
        .any(|entry| {
            let rows = entry["fields"].as_array().expect("field rows");
            refinement["abstract_fields"][entry["abstract_type"].as_str().expect("type")]
                .as_array()
                .expect("abstract fields")
                .iter()
                .any(|field| !rows.contains(field))
        });
    let eligible = !hidden && refinement["histories"] == 0 && refinement["any_row"] == false;
    let step_names = refinement["steps"].as_array().expect("steps");
    let Some(step_rows) = case["refinement_step_rows"].as_array() else {
        return Some("/refinement_step_rows".to_owned());
    };
    let mut previous_key: Option<(String, u64)> = None;
    for (index, row) in step_rows.iter().enumerate() {
        if row["node_id"] != *owner {
            return Some(format!("/refinement_step_rows/{index}"));
        }
        let ordinal = row["ordinal"].as_u64().expect("row ordinal");
        let key = (
            row["node_id"]["digest"].as_str().expect("node digest").to_owned(),
            ordinal,
        );
        if previous_key.as_ref().is_some_and(|previous| previous >= &key) {
            return Some(format!("/refinement_step_rows/{index}"));
        }
        previous_key = Some(key);
    }
    if step_rows.len() != step_names.len() {
        return Some("/refinement_step_rows".to_owned());
    }
    for (index, row) in step_rows.iter().enumerate() {
        if usize::try_from(row["ordinal"].as_u64().expect("row ordinal")).ok() != Some(index) {
            return Some(format!("/refinement_step_rows/{index}"));
        }
    }
    // The sidecar is keyed by the canonical step order, independently of the
    // order in which the source rows were written.
    let steps: Vec<&Value> = step_rows.iter().map(|row| &row["region"]).collect();
    let mut seen = std::collections::BTreeSet::new();
    for (index, entry) in source_map.iter().enumerate() {
        if entry["node_id"] != *owner {
            continue;
        }
        let role = entry["role"].as_str().expect("role");
        let ordinal = entry["ordinal"].as_u64().expect("ordinal") as usize;
        let at = format!("/source_map/{index}");
        if !seen.insert((role.to_owned(), ordinal)) {
            return Some(at);
        }
        let required = match (role, ordinal) {
            ("declaration", 0) => Some(&refinement["declaration"]),
            ("generated", 0) if eligible => Some(&refinement["declaration"]),
            ("generated", n) if eligible && (1..=steps.len()).contains(&n) => Some(steps[n - 1]),
            _ => None,
        };
        match required {
            Some(region) if entry["regions"] == json!([region]) => {}
            _ => return Some(at),
        }
    }
    let required = std::iter::once(("declaration".to_owned(), 0)).chain(
        (0..=steps.len())
            .filter(|_| eligible)
            .map(|ordinal| ("generated".to_owned(), ordinal)),
    );
    required
        .into_iter()
        .any(|key| !seen.contains(&key))
        .then(|| "/source_map".to_owned())
}

#[trace("TC-233", "FR-322-AC-88", "FR-322-AC-89", "FR-322-AC-90", "FR-322-AC-91")]
#[test]
fn tc_233_a_refinement_owner_carries_one_generated_occurrence_per_proof_obligation() {
    let vectors = parsed_fixture(REFINEMENT_STEP_VECTORS);
    let cases = vectors["cases"].as_array().expect("cases");
    assert_eq!(cases.len(), 24);
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let refusal = refinement_step_refusal(case);
        match (&case["expected"], refusal) {
            (Value::String(admitted), None) => assert_eq!(admitted, "admitted", "{name}"),
            (expected, Some(at)) => {
                let refused = &expected["refused"];
                assert_eq!(refused["code"], "invalid_package", "{name}");
                assert_eq!(refused["cause"], "invalid-value", "{name}");
                assert_eq!(refused["at"], at.as_str(), "{name}");
            }
            (expected, outcome) => panic!("{name}: expected {expected}, got {outcome:?}"),
        }
    }
}

/// The operator identities FR-322 "Integer literal operands" lists, read from
/// the requirement text.
fn integer_literal_operand_operators() -> Vec<String> {
    let text = include_str!("../spec/objects/interfaces/FR-322-checked-package-artifact.md");
    let start = text
        .find("**Integer literal operands.**")
        .expect("FR-322 states the rule");
    let paragraph = &text[start..start + text[start..].find("\n\n").expect("paragraph end")];
    let mut operators = Vec::new();
    let mut rest = paragraph;
    while let Some(at) = rest.find("`quire.op.integer.") {
        let tail = &rest[at + 1..];
        let end = tail.find('`').expect("closing backtick");
        operators.push(tail[..end].to_owned());
        rest = &tail[end..];
    }
    operators
}

/// FR-322 "Integer literal operands": the integer operators' integer literal
/// operands, typed at the builtin Integer node, are inline `literal` terms in
/// every published producer-output package (`positive_fixtures()`, which omits
/// the reader-admission fixture `positive-integer-literal-references.json`), and each listed operator has a recorded inline
/// application vector.
#[trace("TC-233", "FR-322-AC-92")]
#[test]
fn tc_233_integer_operator_literal_operands_are_inline_in_the_published_packages() {
    let operators = integer_literal_operand_operators();
    assert_eq!(operators.len(), 6, "{operators:?}");
    let mut inline = BTreeMap::<String, usize>::new();
    for (name, package) in positive_fixtures() {
        let nodes = package["semantic_graph"]["nodes"].as_array().expect("nodes");
        let integer = nodes
            .iter()
            .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
            .map(|node| node["node_id"].clone());
        for node in nodes.iter().filter(|node| is_application_node(node)) {
            let operation = node["body"]["operation"]["identity"].as_str().unwrap_or_default();
            if !operators.iter().any(|listed| listed == operation) {
                continue;
            }
            for operand in node["body"]["arguments"].as_array().expect("arguments") {
                if operand["term"] == "literal" {
                    assert_eq!(operand["value_kind"], "integer", "{name}");
                    assert_eq!(Some(&operand["type"]), integer.as_ref(), "{name}");
                    *inline.entry(name.to_owned()).or_default() += 1;
                    if node["declaration"]["qualified_name"] == json!(["three"]) {
                        assert_eq!(node["dependencies"], json!([]), "{name}: no operand node");
                    }
                } else if operand["term"] == "reference" {
                    let target = nodes
                        .iter()
                        .find(|candidate| candidate["node_id"] == operand["target"])
                        .expect("a reference names a node");
                    let integer_literal = target["node_tag"] == "value"
                        && target["semantic_form"] == "literal"
                        && target["body"]["value_kind"] == "integer";
                    assert!(!integer_literal, "{name}: {operation} references an integer literal node");
                }
            }
        }
    }
    assert_eq!(inline["positive-two-owners-a"], 2);
    assert_eq!(inline["positive-two-owners-b"], 2);
    assert_eq!(inline["positive-operation-identities"], 4);
    // The literal nodes for 7 and -2 stay for the operands that reference them.
    let operation_package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    for value in ["7", "-2"] {
        assert!(
            operation_package["semantic_graph"]["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .any(|node| node["semantic_form"] == "literal"
                    && node["body"]["value_kind"] == "integer"
                    && node["body"]["value"] == value),
            "the {value} literal node is present"
        );
    }
    // One recorded inline application per listed operator, and none beyond.
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let application_schema =
        schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "ApplicationNode");
    let cases = vectors["inline_literal_cases"].as_array().expect("cases");
    let recorded: BTreeSet<&str> = cases
        .iter()
        .map(|case| case["operation"].as_str().expect("operation"))
        .collect();
    let listed: BTreeSet<&str> = operators.iter().map(String::as_str).collect();
    assert_eq!(recorded, listed, "one case per listed operator");
    for case in cases {
        let name = case["name"].as_str().expect("name");
        application_schema
            .validate(&case["preimage"])
            .unwrap_or_else(|error| panic!("{name} preimage is closed: {error}"));
        assert_eq!(
            unqualified_jcs_sha256(&case["preimage"]),
            case["sha256"].as_str().expect("digest"),
            "{name}"
        );
        assert_eq!(
            case["preimage"]["body"]["operation"]["identity"], case["operation"],
            "{name}: the case names its preimage's operation"
        );
        // The reference form of the same application: each inline operand
        // replaced by a reference to the published literal node of its value.
        let integer = operation_package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
            .expect("the Integer node")["node_id"]
            .clone();
        let mut by_reference = case["preimage"].clone();
        for operand in by_reference["body"]["arguments"]
            .as_array_mut()
            .expect("arguments")
        {
            assert_eq!(operand["term"], "literal", "{name}: operands are inline");
            assert_eq!(operand["value_kind"], "integer", "{name}");
            assert_eq!(operand["type"], integer, "{name}: typed at the Integer node");
            let node = operation_package["semantic_graph"]["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .find(|node| {
                    node["semantic_form"] == "literal"
                        && node["body"]["value_kind"] == "integer"
                        && node["body"]["value"] == operand["value"]
                })
                .unwrap_or_else(|| panic!("{name}: a published literal node of the value"));
            *operand = json!({"term": "reference", "target": node["node_id"]});
        }
        assert_eq!(
            unqualified_jcs_sha256(&by_reference),
            case["reference_form_sha256"].as_str().expect("digest"),
            "{name}: the reference form's recorded key"
        );
        assert_ne!(case["sha256"], case["reference_form_sha256"], "{name}");
    }
}

const POSITIVE_INTEGER_LITERAL_REFERENCES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-integer-literal-references.json");

/// FR-322 "Integer literal operands": the rule binds the producer. A package
/// whose `quire.op.integer.add` application references literal nodes is
/// admitted by the reader, under its own node identity.
#[trace("TC-233", "FR-322-AC-93")]
#[test]
fn tc_233_a_reader_admits_the_reference_form_of_an_integer_literal_operand() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let references = parsed_fixture(POSITIVE_INTEGER_LITERAL_REFERENCES);
    v2.validate(&references)
        .unwrap_or_else(|error| panic!("the reference form has the closed V2 shape: {error}"));
    assert_eq!(read_package_semantics(&references, &catalog), Ok(()));
    let application = |package: &Value| {
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["declaration"]["qualified_name"] == json!(["three"]))
            .expect("the `three` application")
            .clone()
    };
    let referenced = application(&references);
    assert_eq!(referenced["body"]["operation"]["identity"], "quire.op.integer.add");
    assert!(
        referenced["body"]["arguments"]
            .as_array()
            .expect("arguments")
            .iter()
            .all(|operand| operand["term"] == "reference"),
        "the published operands are references"
    );
    let inline = application(&parsed_fixture(POSITIVE_TWO_OWNERS_A));
    assert_ne!(
        referenced["node_id"], inline["node_id"],
        "the reference form has a different node identity"
    );
    assert_ne!(references["package_id"], parsed_fixture(POSITIVE_TWO_OWNERS_A)["package_id"]);
}

// ---------------------------------------------------------------------------
// FR-469 / TC-951: the `extension`/`extension_form` checked-package node.
// ---------------------------------------------------------------------------

/// An extension definition the reader holds: the node entries of its
/// `typed_node_schema` as (node kind, clause (name, type spelling) list).
#[derive(Clone)]
struct HeldExtension {
    definition: Value,
    nodes: Vec<(String, Vec<(String, String)>)>,
}

fn extension_definition(identity: &str) -> Value {
    json!({"authority": "example", "identity": identity})
}

/// Every entry of `extension-definitions.json`, held as the reader holds a
/// definition: its reference and its `typed_node_schema` node entries.
fn published_extensions() -> Vec<HeldExtension> {
    parsed_fixture(EXTENSION_DEFINITIONS)["definitions"]
        .as_array()
        .expect("published definitions are an array")
        .iter()
        .map(held_entry)
        .collect()
}

/// The published FR-467 extension definition under `{example, identity}`.
fn published_extension(identity: &str) -> HeldExtension {
    let reference = extension_definition(identity);
    published_extensions()
        .into_iter()
        .find(|held| held.definition == reference)
        .unwrap_or_else(|| panic!("extension-definitions.json publishes {identity}"))
}

fn held_entry(entry: &Value) -> HeldExtension {
    HeldExtension {
        definition: entry["reference"].clone(),
        nodes: entry["definition"]["typed_node_schema"]["nodes"]
            .as_array()
            .expect("typed_node_schema nodes")
            .iter()
            .map(|node| {
                let clauses = node["clauses"]
                    .as_array()
                    .expect("node entry clauses")
                    .iter()
                    .map(|clause| {
                        let text = |member: &str| {
                            clause[member].as_str().expect("clause member").to_owned()
                        };
                        (text("name"), text("type"))
                    })
                    .collect();
                (node["node_kind"].as_str().expect("node kind").to_owned(), clauses)
            })
            .collect(),
    }
}

fn held_retention() -> HeldExtension {
    published_extension("retention")
}

fn held_quota() -> HeldExtension {
    published_extension("quota")
}

fn held_policy() -> HeldExtension {
    published_extension("policy")
}

/// A refusal of the extension reader: code, cause, the pointer FR-469 names
/// (relative to the document), the `locus` node id when the code carries one,
/// and the names the refusal states.
#[derive(Debug, PartialEq)]
struct ExtensionRefusal {
    code: &'static str,
    cause: Option<&'static str>,
    pointer: String,
    locus: Option<Value>,
    naming: Vec<String>,
}

fn extension_refusal(
    code: &'static str,
    cause: Option<&'static str>,
    pointer: impl Into<String>,
) -> ExtensionRefusal {
    ExtensionRefusal {
        code,
        cause,
        pointer: pointer.into(),
        locus: None,
        naming: Vec::new(),
    }
}

/// The type a clause value has, spelled as `typed_node_schema` spells a
/// clause type: `Text`, `Int[lo, hi]`, or an extension node's node kind.
fn clause_value_type(nodes: &[Value], subnode: &Value) -> String {
    if subnode["node_tag"] == "extension" {
        return subnode["body"]["node_kind"]
            .as_str()
            .expect("extension node kind")
            .to_owned();
    }
    let Some(typed) = subnode["semantic_type"]["digest"]
        .as_str()
        .and_then(|digest| node_by_digest(nodes, digest))
    else {
        return "unknown".to_owned();
    };
    match (typed["node_tag"].as_str(), typed["semantic_form"].as_str()) {
        (Some("scalar_type"), Some("text")) => "Text".to_owned(),
        (Some("scalar_type"), Some("integer")) => "Int".to_owned(),
        (Some("bounded_domain"), Some("integer_range")) => {
            let bound = |name: &str| {
                typed["body"]["members"]
                    .as_array()
                    .and_then(|members| members.iter().find(|member| member["name"] == name))
                    .and_then(|member| member["value"]["value"].as_str())
                    .unwrap_or("?")
                    .to_owned()
            };
            format!("Int[{}, {}]", bound("min"), bound("max"))
        }
        (_, Some(form)) => form.to_owned(),
        _ => "unknown".to_owned(),
    }
}

/// FR-469's reader joins over every `extension` node, ascending by node-id
/// digest, reporting the first failing join of the first failing node.
fn extension_joins(package: &Value, held: &[HeldExtension]) -> Result<(), ExtensionRefusal> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let mut extensions = nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node["node_tag"] == "extension")
        .collect::<Vec<_>>();
    extensions.sort_by_key(|(_, node)| node["node_id"]["digest"].as_str().unwrap_or_default());
    for (index, node) in extensions {
        let base = format!("/semantic_graph/nodes/{index}/body");
        let body = &node["body"];
        let selected = package["lock"]["definition_selections"]
            .as_array()
            .is_some_and(|selections| selections.contains(&body["definition"]));
        if !selected {
            return Err(extension_refusal(
                "missing_declaration",
                Some("missing-selection"),
                format!("{base}/definition"),
            ));
        }
        let Some(definition) = held
            .iter()
            .find(|candidate| candidate.definition == body["definition"])
        else {
            return Err(ExtensionRefusal {
                locus: Some(node["node_id"].clone()),
                ..extension_refusal(
                    "unknown_required_capability",
                    None,
                    format!("{base}/definition"),
                )
            });
        };
        let Some((_, entry)) = definition
            .nodes
            .iter()
            .find(|(kind, _)| body["node_kind"].as_str() == Some(kind.as_str()))
        else {
            return Err(extension_refusal(
                "missing_declaration",
                Some("missing-name"),
                format!("{base}/node_kind"),
            ));
        };
        let clauses = body["clauses"].as_array().expect("clauses are an array");
        for (position, clause) in clauses.iter().enumerate() {
            if entry.get(position).map(|(name, _)| name.as_str()) != clause["name"].as_str() {
                return Err(extension_refusal(
                    "invalid_package",
                    Some("invalid-value"),
                    format!("{base}/clauses/{position}"),
                ));
            }
        }
        if clauses.len() < entry.len() {
            return Err(ExtensionRefusal {
                naming: vec![entry[clauses.len()].0.clone()],
                ..extension_refusal(
                    "invalid_package",
                    Some("missing-member"),
                    format!("{base}/clauses"),
                )
            });
        }
        for (position, clause) in clauses.iter().enumerate() {
            let pointer = format!("{base}/clauses/{position}/value");
            let Some(subnode) = clause["value"]["target"]["digest"]
                .as_str()
                .and_then(|digest| node_by_digest(nodes, digest))
            else {
                return Err(extension_refusal(
                    "missing_declaration",
                    Some("missing-name"),
                    pointer,
                ));
            };
            let (name, expected) = &entry[position];
            let actual = clause_value_type(nodes, subnode);
            if &actual != expected {
                return Err(ExtensionRefusal {
                    naming: vec![name.clone(), expected.clone(), actual],
                    ..extension_refusal("ill_typed", Some("type-mismatch"), pointer)
                });
            }
        }
    }
    Ok(())
}

/// The strict V2 reader over a package and the extension definitions it
/// holds: the closed schema, the graph-shape check of an extension node's
/// `dependencies`, the key and declaration stages, then FR-469's joins.
fn read_extension_package(
    package: &Value,
    held: &[HeldExtension],
) -> Result<(), ExtensionRefusal> {
    let v2 = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    if !v2.is_valid(package) {
        return Err(extension_refusal("malformed_wire", None, ""));
    }
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    for node in nodes.iter().filter(|node| node["node_tag"] == "extension") {
        let mut targets = body_targets(&node["body"]);
        targets.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
        targets.dedup();
        if node["dependencies"] != json!(targets) {
            return Err(ExtensionRefusal {
                locus: Some(node["node_id"].clone()),
                ..extension_refusal("invalid_semantic_graph", None, "/dependencies")
            });
        }
    }
    let catalog = parsed_fixture(OPERATION_CATALOG);
    match read_package_semantics(package, &catalog) {
        Ok(()) => {}
        Err(ReadRefusal::StaleNodeKey(node)) => {
            return Err(ExtensionRefusal {
                locus: Some(node),
                ..extension_refusal("invalid_package", Some("stale-node-key"), "")
            });
        }
        Err(ReadRefusal::Declaration(DeclarationRefusal::AmbiguousName, node)) => {
            return Err(ExtensionRefusal {
                locus: Some(node),
                ..extension_refusal("ambiguous_declaration", Some("ambiguous-name"), "")
            });
        }
        Err(other) => panic!("unexpected refusal before the extension joins: {other:?}"),
    }
    extension_joins(package, held)
}

fn extension_package() -> Value {
    parsed_fixture(POSITIVE_EXTENSION_NODES)
}

fn extension_node_index(package: &Value, name: &str) -> usize {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .position(|node| {
            node["node_tag"] == "extension" && node["declaration"]["qualified_name"] == json!([name])
        })
        .unwrap_or_else(|| panic!("extension node {name}"))
}

/// The node id of the one `extension` node of a package with exactly one.
fn only_form_node_id(package: &Value) -> Value {
    let forms = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .filter(|node| node["node_tag"] == "extension")
        .collect::<Vec<_>>();
    assert_eq!(forms.len(), 1, "exactly one extension node");
    forms[0]["node_id"].clone()
}

/// Apply `mutate` to the `Audit` form node, then rekey every node whose key
/// is no longer its preimage digest and rebuild the identity preimage.
fn mutate_audit_form(package: &mut Value, mutate: impl FnOnce(&mut Value)) {
    let index = extension_node_index(package, "Audit");
    mutate(&mut package["semantic_graph"]["nodes"][index]);
    rekey_stale_nodes(package);
}

fn node_by_shape<'a>(package: &'a Value, tag: &str, form: &str) -> &'a Value {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .find(|node| node["node_tag"] == tag && node["semantic_form"] == form)
        .unwrap_or_else(|| panic!("a {tag}/{form} node"))
}

/// Every `target` of a `reference` term in a body.
fn body_targets(body: &Value) -> Vec<Value> {
    match body {
        Value::Object(object) if object.get("term") == Some(&json!("reference")) => {
            vec![object["target"].clone()]
        }
        Value::Object(object) => object.values().flat_map(body_targets).collect(),
        Value::Array(items) => items.iter().flat_map(body_targets).collect(),
        _ => Vec::new(),
    }
}

/// Append a structural node with the given `semantic_type` (`None` for a
/// self-typed node), declaration, one occurrence of `role`, and a one-byte
/// `source_map` region at `start`; return its node id.
fn push_structural_node(
    package: &mut Value,
    (tag, form): (&str, &str),
    semantic_type: Option<Value>,
    body: Value,
    declaration: Option<Value>,
    (role, start): (&str, u64),
) -> Value {
    let placeholder = json!({"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)});
    let mut dependencies = body_targets(&body);
    dependencies.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
    dependencies.dedup();
    let mut node = json!({
        "node_id": placeholder,
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": tag,
        "semantic_form": form,
        "semantic_type": semantic_type.unwrap_or_else(|| placeholder.clone()),
        "dependencies": dependencies,
        "occurrences": [{"role": role, "ordinal": 0}],
        "body": body,
    });
    if let Some(declaration) = declaration {
        node["declaration"] = declaration;
        node["owner"] = json!({"kind": "source", "authority": "agent-ix", "identity": "example"});
    }
    append_rekeyed_structural_node(package, node);
    let id = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .last()
        .expect("appended node")["node_id"]
        .clone();
    let source = package["lock"]["sources"][0].clone();
    package["source_map"]
        .as_array_mut()
        .expect("source map is an array")
        .push(json!({
            "node_id": id,
            "role": role,
            "ordinal": 0,
            "regions": [{"source": source, "start": start, "end": start + 1}],
        }));
    id
}

fn literal_node(
    package: &mut Value,
    semantic_type: &Value,
    (value, kind): (&str, &str),
    start: u64,
) -> Value {
    let literal_type = node_by_shape(
        package,
        "scalar_type",
        if kind == "text" { "text" } else { "integer" },
    )["node_id"]
        .clone();
    push_structural_node(
        package,
        ("value", "literal"),
        Some(semantic_type.clone()),
        json!({"term": "literal", "type": literal_type, "value": value, "value_kind": kind}),
        None,
        ("expression", start),
    )
}

fn range_node(package: &mut Value, (low, high): (&str, &str), start: u64) -> Value {
    let integer = node_by_shape(package, "scalar_type", "integer")["node_id"].clone();
    let bound = |name: &str, value: &str| {
        json!({"term": "binding", "name": name, "value": {
            "term": "literal", "type": integer, "value": value, "value_kind": "integer"}})
    };
    push_structural_node(
        package,
        ("bounded_domain", "integer_range"),
        Some(integer.clone()),
        json!({"term": "aggregate", "members": [bound("min", low), bound("max", high)]}),
        None,
        ("type", start),
    )
}

/// Append a selected extension form node and select its definition.
fn push_extension_form(
    package: &mut Value,
    (definition, node_kind, name): (&Value, &str, &str),
    clauses: &[(&str, &Value)],
    start: u64,
) -> Value {
    let selections = package["lock"]["definition_selections"]
        .as_array_mut()
        .expect("definition selections are an array");
    if !selections.contains(definition) {
        selections.push(definition.clone());
        selections.sort_by_key(|selection| selection["identity"].as_str().map(str::to_owned));
    }
    let bindings = clauses
        .iter()
        .map(|(clause, target)| {
            json!({"term": "binding", "name": clause, "value": {"term": "reference", "target": target}})
        })
        .collect::<Vec<_>>();
    push_structural_node(
        package,
        ("extension", "extension_form"),
        None,
        json!({
            "term": "extension_form",
            "definition": definition,
            "node_kind": node_kind,
            "clauses": bindings,
        }),
        Some(json!({"qualified_name": [name]})),
        ("declaration", start),
    )
}

/// The package with a held, selected `quota` form `Q { limit: 5 }` beside
/// `Audit`, and the `Q` node id.
fn extension_package_with_quota() -> (Value, Value) {
    let mut package = extension_package();
    let range = range_node(&mut package, ("1", "100"), 2000);
    let limit = literal_node(&mut package, &range, ("5", "integer"), 2010);
    let quota = push_extension_form(
        &mut package,
        (&extension_definition("quota"), "quota", "Q"),
        &[("limit", &limit)],
        2020,
    );
    (package, quota)
}

const AUDIT_FORM: &str = r#"retention Audit { subject: "audit", days: 30 }"#;

#[trace("TC-951", "FR-469-AC-1")]
#[test]
fn tc_951_the_fixture_form_is_one_extension_node_the_full_reader_admits() {
    let package = extension_package();
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes");
    assert_eq!(
        read_extension_package(&package, &[held_retention()]),
        Ok(()),
        "X01: the full reader admits the package"
    );
    let form = &nodes[extension_node_index(&package, "Audit")];
    only_form_node_id(&package);
    assert_eq!(form["semantic_form"], "extension_form");
    assert_eq!(form["declaration"], json!({"qualified_name": ["Audit"]}));
    assert_eq!(form["owner"]["kind"], "source");
    assert_eq!(form["semantic_type"], form["node_id"], "its own type");
    assert!(
        form.get("recursion_group").is_none() && form.get("nominal_identity_preimage").is_none()
    );
    assert_eq!(form["body"]["term"], "extension_form");
    assert_eq!(form["body"]["definition"], extension_definition("retention"));
    assert_eq!(form["body"]["node_kind"], "retention");
    let clauses = form["body"]["clauses"].as_array().expect("clauses");
    let names = clauses
        .iter()
        .map(|clause| clause["name"].as_str().expect("clause name"))
        .collect::<Vec<_>>();
    assert_eq!(names, ["subject", "days"], "clause order of the production");
    let types = clauses
        .iter()
        .map(|clause| {
            assert_eq!(clause["term"], "binding");
            assert_eq!(clause["value"]["term"], "reference");
            let target = node_by_digest(
                nodes,
                clause["value"]["target"]["digest"]
                    .as_str()
                    .expect("clause target digest"),
            )
            .expect("every clause value is its own node");
            clause_value_type(nodes, target)
        })
        .collect::<Vec<_>>();
    assert_eq!(types, ["Text", "Int[1, 3650]"]);
    let mut targets = body_targets(&form["body"]);
    targets.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
    assert_eq!(
        form["dependencies"],
        json!(targets),
        "digest-ascending clause targets"
    );
    assert_eq!(
        form["occurrences"],
        json!([{"role": "declaration", "ordinal": 0}])
    );
    let entries = package["source_map"]
        .as_array()
        .expect("source map")
        .iter()
        .filter(|entry| entry["node_id"] == form["node_id"])
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 1, "one source_map entry for the one occurrence");
    let regions = entries[0]["regions"].as_array().expect("regions");
    assert_eq!(regions.len(), 1, "exactly the one region of the whole form");
    let span =
        regions[0]["end"].as_u64().expect("end") - regions[0]["start"].as_u64().expect("start");
    assert_eq!(span, AUDIT_FORM.len() as u64, "the region spans the whole form");
}

#[trace("TC-951", "FR-469-AC-2")]
#[test]
fn tc_951_the_node_key_binds_definition_kind_name_owner_and_clause_subnodes_not_the_span() {
    let package = extension_package();
    let index = extension_node_index(&package, "Audit");
    let form = package["semantic_graph"]["nodes"][index].clone();
    let preimage = structural_node_preimage(&form).expect("the form is a structural node");
    assert_eq!(preimage["version"], "quire.structural-node/v1");
    assert_eq!(preimage["semantic_type"], Value::Null);
    assert_eq!(preimage["recursion"], Value::Null);
    let preimage_schema = jsonschema::validator_for(&parsed_fixture(NODE_PREIMAGE_SCHEMA))
        .expect("node preimage schema is valid");
    preimage_schema
        .validate(&preimage)
        .expect("X02: the preimage is the closed ExtensionFormNode");
    let mut stray = preimage.clone();
    stray["extra"] = json!(true);
    assert!(!preimage_schema.is_valid(&stray), "the preimage is closed");
    assert_eq!(
        form["node_id"]["digest"].as_str(),
        Some(unqualified_jcs_sha256(&preimage).as_str()),
        "X02: the recomputed key equals the retained key"
    );

    let held = [held_retention()];
    assert_eq!(read_extension_package(&package, &held), Ok(()));
    let changes: [(&str, Box<dyn Fn(&mut Value)>); 5] = [
        (
            "definition identity",
            Box::new(|package| {
                mutate_audit_form(package, |node| {
                    node["body"]["definition"]["identity"] = json!("retention-v2");
                });
            }),
        ),
        (
            "node kind",
            Box::new(|package| {
                mutate_audit_form(package, |node| node["body"]["node_kind"] = json!("retention-b"));
            }),
        ),
        (
            "declared name",
            Box::new(|package| {
                mutate_audit_form(package, |node| {
                    node["declaration"]["qualified_name"] = json!(["Ledger"]);
                });
            }),
        ),
        (
            "owner identity",
            Box::new(|package| {
                let other = json!({
                    "authority": "agent-ix",
                    "digest": "3333333333333333333333333333333333333333333333333333333333333333",
                    "digest_domain": "quire.source.bytes/v1",
                    "identity": "other",
                });
                package["lock"]["sources"]
                    .as_array_mut()
                    .expect("sources")
                    .push(other.clone());
                let id = only_form_node_id(package);
                for entry in package["source_map"].as_array_mut().expect("source map") {
                    if entry["node_id"] == id {
                        entry["regions"][0]["source"] = other.clone();
                    }
                }
                mutate_audit_form(package, |node| {
                    node["owner"]["identity"] = json!("other");
                });
            }),
        ),
        (
            "days subnode",
            Box::new(|package| {
                let days = package["semantic_graph"]["nodes"]
                    .as_array()
                    .expect("nodes")
                    .iter()
                    .position(|node| node["node_tag"] == "value" && node["body"]["value"] == "30")
                    .expect("the days literal");
                package["semantic_graph"]["nodes"][days]["body"]["value"] = json!("31");
                rekey_stale_nodes(package);
            }),
        ),
    ];
    for (what, change) in changes {
        let mut changed = package.clone();
        change(&mut changed);
        assert_ne!(
            only_form_node_id(&changed),
            form["node_id"],
            "X02: changing the {what} changes the node id"
        );
        assert_ne!(
            changed["package_id"], package["package_id"],
            "X02: changing the {what} changes package_id"
        );
        if what == "owner identity" {
            assert_eq!(read_extension_package(&changed, &held), Ok(()));
        }
    }

    // X03: only the source region moves.
    let mut moved = package.clone();
    for entry in moved["source_map"].as_array_mut().expect("source map") {
        if entry["node_id"] == form["node_id"] {
            for bound in ["start", "end"] {
                entry["regions"][0][bound] =
                    json!(entry["regions"][0][bound].as_u64().expect("bound") + 1);
            }
        }
    }
    assert_eq!(read_extension_package(&moved, &held), Ok(()), "X03");
    assert_eq!(moved["package_id"], package["package_id"], "X03");
    assert_eq!(only_form_node_id(&moved), form["node_id"], "X03");

    // X04: the old key retained after a definition identity change.
    let mut stale = package.clone();
    replace_node_field_in_graph(
        &mut stale,
        index,
        "/body/definition/identity",
        json!("retention-v2"),
    );
    stale["identity_preimage"] = derive_identity_preimage(&stale);
    refresh_package_id(&mut stale);
    assert_eq!(
        read_extension_package(&stale, &held),
        Err(ExtensionRefusal {
            locus: Some(form["node_id"].clone()),
            ..extension_refusal("invalid_package", Some("stale-node-key"), "")
        }),
        "X04"
    );
}

#[trace("TC-951", "FR-469-AC-3")]
#[test]
fn tc_951_a_missing_definition_selection_or_unheld_definition_refuses_before_any_clause_check() {
    let package = extension_package();
    let index = extension_node_index(&package, "Audit");
    let node_id = package["semantic_graph"]["nodes"][index]["node_id"].clone();
    let pointer = format!("/semantic_graph/nodes/{index}/body/definition");
    // X05
    let before = package.clone();
    assert_eq!(
        read_extension_package(&package, &[]),
        Err(ExtensionRefusal {
            locus: Some(node_id),
            ..extension_refusal("unknown_required_capability", None, pointer.clone())
        })
    );
    assert_eq!(package, before, "the lock, source map and node are unchanged");
    // X06
    let mut unselected = package.clone();
    unselected["lock"]["definition_selections"] = json!([]);
    unselected["identity_preimage"] = derive_identity_preimage(&unselected);
    refresh_package_id(&mut unselected);
    for held in [vec![held_retention()], Vec::new()] {
        assert_eq!(
            read_extension_package(&unselected, &held),
            Err(extension_refusal(
                "missing_declaration",
                Some("missing-selection"),
                pointer.clone()
            ))
        );
    }
}

#[trace("TC-951", "FR-469-AC-4")]
#[test]
fn tc_951_node_kind_and_clause_set_joins_refuse_at_the_offending_position() {
    let held = [held_retention()];
    let base = extension_package();
    let index = extension_node_index(&base, "Audit");
    let prefix = format!("/semantic_graph/nodes/{index}/body");
    // X07
    let mut kind = base.clone();
    mutate_audit_form(&mut kind, |node| {
        node["body"]["node_kind"] = json!("retentions");
    });
    assert_eq!(
        read_extension_package(&kind, &held),
        Err(extension_refusal(
            "missing_declaration",
            Some("missing-name"),
            format!("{prefix}/node_kind")
        ))
    );
    // X08: reordered, an extra clause, a repeated clause.
    for (what, order, offender) in [
        ("days, subject", vec![1, 0], 0),
        ("subject, days, owner", vec![0, 1, 2], 2),
        ("subject, subject", vec![0, 0], 1),
    ] {
        let mut changed = base.clone();
        mutate_audit_form(&mut changed, |node| {
            let mut clauses = node["body"]["clauses"].clone();
            let extra = json!({"term": "binding", "name": "owner", "value": clauses[0]["value"]});
            clauses.as_array_mut().expect("clauses").push(extra);
            node["body"]["clauses"] =
                json!(order.iter().map(|i| clauses[*i].clone()).collect::<Vec<_>>());
            let mut targets = body_targets(&node["body"]);
            targets.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
            targets.dedup();
            node["dependencies"] = json!(targets);
        });
        assert_eq!(
            read_extension_package(&changed, &held),
            Err(extension_refusal(
                "invalid_package",
                Some("invalid-value"),
                format!("{prefix}/clauses/{offender}")
            )),
            "X08: {what}"
        );
    }
    // X09
    let mut short = base.clone();
    mutate_audit_form(&mut short, |node| {
        let first = node["body"]["clauses"][0].clone();
        node["dependencies"] = json!([first["value"]["target"].clone()]);
        node["body"]["clauses"] = json!([first]);
    });
    assert_eq!(
        read_extension_package(&short, &held),
        Err(ExtensionRefusal {
            naming: vec!["days".to_owned()],
            ..extension_refusal(
                "invalid_package",
                Some("missing-member"),
                format!("{prefix}/clauses")
            )
        })
    );
}

#[trace("TC-951", "FR-469-AC-5")]
#[test]
fn tc_951_clause_type_joins_refuse_a_mistyped_absent_or_other_kind_subnode() {
    let base = extension_package();
    let index = extension_node_index(&base, "Audit");
    let pointer = format!("/semantic_graph/nodes/{index}/body/clauses/1/value");
    let held = [held_retention()];
    // X10: the days subnode typed Text.
    let mut mistyped = base.clone();
    let text = node_by_shape(&base, "scalar_type", "text")["node_id"].clone();
    let days = base["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_tag"] == "value" && node["body"]["value"] == "30")
        .expect("the days literal");
    let node = &mut mistyped["semantic_graph"]["nodes"][days];
    node["semantic_type"] = text.clone();
    node["body"] = json!({"term": "literal", "type": text, "value": "thirty", "value_kind": "text"});
    rekey_stale_nodes(&mut mistyped);
    assert_eq!(
        read_extension_package(&mistyped, &held),
        Err(ExtensionRefusal {
            naming: vec!["days".to_owned(), "Int[1, 3650]".to_owned(), "Text".to_owned()],
            ..extension_refusal("ill_typed", Some("type-mismatch"), pointer.clone())
        })
    );
    // X11: the days reference re-pointed at a key the graph lacks.
    let absent = json!({"domain": "quire.checked-semantic-node/v1", "digest": "ab".repeat(32)});
    let mut dangling = base.clone();
    mutate_audit_form(&mut dangling, |node| {
        node["body"]["clauses"][1]["value"]["target"] = absent.clone();
        let mut targets = body_targets(&node["body"]);
        targets.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
        node["dependencies"] = json!(targets);
    });
    assert_eq!(
        read_extension_package(&dangling, &held),
        Err(extension_refusal(
            "missing_declaration",
            Some("missing-name"),
            pointer
        ))
    );
    // X12: a `policy` definition, held and selected beside `retention` and
    // `quota`, whose clause `retain` is typed by the node kind `retention`.
    let (with_quota, quota) = extension_package_with_quota();
    let audit = with_quota["semantic_graph"]["nodes"][index]["node_id"].clone();
    let policy = extension_definition("policy");
    let policy_held = [held_retention(), held_quota(), held_policy()];
    let mut wrong_kind = with_quota.clone();
    push_extension_form(&mut wrong_kind, (&policy, "policy", "P"), &[("retain", &quota)], 2030);
    let policy_index = wrong_kind["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .len()
        - 1;
    assert_eq!(
        read_extension_package(&wrong_kind, &policy_held),
        Err(ExtensionRefusal {
            naming: vec!["retain".to_owned(), "retention".to_owned(), "quota".to_owned()],
            ..extension_refusal(
                "ill_typed",
                Some("type-mismatch"),
                format!("/semantic_graph/nodes/{policy_index}/body/clauses/0/value")
            )
        })
    );
    let mut right_kind = with_quota;
    push_extension_form(&mut right_kind, (&policy, "policy", "P"), &[("retain", &audit)], 2030);
    assert_eq!(read_extension_package(&right_kind, &policy_held), Ok(()));
}

#[trace("TC-951", "FR-469-AC-6")]
#[test]
fn tc_951_wire_graph_shape_and_name_refusals_of_the_extension_node() {
    let held = [held_retention()];
    let base = extension_package();
    let index = extension_node_index(&base, "Audit");
    // X13: four schema-level defects.
    let defects: [(&str, Box<dyn Fn(&mut Value)>); 4] = [
        (
            "inline literal clause value",
            Box::new(|node| {
                node["body"]["clauses"][1]["value"] = json!({
                    "term": "literal", "type": node["body"]["clauses"][1]["value"]["target"],
                    "value": "30", "value_kind": "integer"});
            }),
        ),
        (
            "second occurrence",
            Box::new(|node| {
                node["occurrences"]
                    .as_array_mut()
                    .expect("occurrences")
                    .push(json!({"role": "declaration", "ordinal": 1}));
            }),
        ),
        (
            "expression occurrence",
            Box::new(|node| node["occurrences"][0]["role"] = json!("expression")),
        ),
        (
            "another form",
            Box::new(|node| node["semantic_form"] = json!("retention")),
        ),
    ];
    for (what, defect) in defects {
        let mut changed = base.clone();
        defect(&mut changed["semantic_graph"]["nodes"][index]);
        assert_eq!(
            read_extension_package(&changed, &held),
            Err(extension_refusal("malformed_wire", None, "")),
            "X13: {what}"
        );
    }
    // X14
    let mut missing = base.clone();
    let node_id = base["semantic_graph"]["nodes"][index]["node_id"].clone();
    let kept = base["semantic_graph"]["nodes"][index]["dependencies"][0].clone();
    replace_node_field_in_graph(&mut missing, index, "/dependencies", json!([kept]));
    missing["identity_preimage"] = derive_identity_preimage(&missing);
    refresh_package_id(&mut missing);
    assert_eq!(
        read_extension_package(&missing, &held),
        Err(ExtensionRefusal {
            locus: Some(node_id.clone()),
            ..extension_refusal("invalid_semantic_graph", None, "/dependencies")
        }),
        "X14"
    );
    // X15: a declared record `Audit` beside the form.
    let mut both = base.clone();
    let record = push_structural_node(
        &mut both,
        ("composite_type", "record"),
        None,
        json!({"term": "aggregate", "members": []}),
        Some(json!({"qualified_name": ["Audit"]})),
        ("declaration", 3000),
    );
    let later = if record["digest"].as_str() > node_id["digest"].as_str() {
        record
    } else {
        node_id
    };
    assert_eq!(
        read_extension_package(&both, &held),
        Err(ExtensionRefusal {
            locus: Some(later),
            ..extension_refusal("ambiguous_declaration", Some("ambiguous-name"), "")
        }),
        "X15"
    );
}

#[trace("TC-951", "FR-469-AC-7")]
#[test]
fn tc_951_the_node_kind_is_body_data_so_a_second_extension_needs_no_schema_change() {
    let (package, quota) = extension_package_with_quota();
    let node = node_by_digest(
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes"),
        quota["digest"].as_str().expect("digest"),
    )
    .expect("the quota form");
    assert_eq!(node["node_tag"], "extension", "X16: same tag");
    assert_eq!(node["semantic_form"], "extension_form", "X16: same form");
    assert_eq!(node["body"]["node_kind"], "quota");
    assert_eq!(
        read_extension_package(&package, &[held_retention(), held_quota()]),
        Ok(()),
        "X16: admitted under the unchanged V2 schemas"
    );
    for schema_text in [SCHEMA, NODE_PREIMAGE_SCHEMA] {
        assert!(
            !schema_text.contains("\"retention\"") && !schema_text.contains("\"quota\""),
            "the closed tag and form lists name no node kind"
        );
    }
}

#[trace("TC-951", "FR-469-AC-8")]
#[test]
fn tc_951_a_selected_extension_is_carried_by_definition_selections_not_required_features() {
    for (name, package) in [
        ("positive-extension-nodes", extension_package()),
        ("positive-all-families", parsed_fixture(POSITIVE_ALL_FAMILIES)),
    ] {
        let selections = package["lock"]["definition_selections"]
            .as_array()
            .expect("definition selections");
        assert!(
            selections.contains(&extension_definition("retention")),
            "{name}: the extension is selected"
        );
        let features = package["lock"]["required_features"]
            .as_array()
            .expect("required features");
        for selection in selections {
            assert!(
                !features.contains(&selection["identity"]),
                "{name}: required_features lists feature identifiers, never a definition"
            );
        }
    }
    let refusal = read_extension_package(&extension_package(), &[]).expect_err("X05 refuses");
    assert_eq!(
        refusal.code, "unknown_required_capability",
        "X17: an unheld extension is never an unknown required feature"
    );
}

#[trace("TC-951", "FR-469-AC-9")]
#[test]
fn tc_951_published_definitions_meet_their_schema_and_pair_fr_467_members() {
    let published = parsed_fixture(EXTENSION_DEFINITIONS);
    let schema = jsonschema::validator_for(&parsed_fixture(EXTENSION_DEFINITIONS_SCHEMA))
        .expect("extension definitions schema is valid");
    assert!(schema.is_valid(&published), "XD: the published definitions meet their schema");
    let definitions = published["definitions"].as_array().expect("definitions");
    let references = definitions
        .iter()
        .map(|entry| entry["reference"].clone())
        .collect::<Vec<_>>();
    let mut ordered = references.clone();
    ordered.sort_by_key(|reference| {
        (
            reference["authority"].as_str().unwrap_or_default().to_owned(),
            reference["identity"].as_str().unwrap_or_default().to_owned(),
        )
    });
    ordered.dedup();
    assert_eq!(references, ordered, "one entry per reference, in reference order");
    for entry in definitions {
        let definition = &entry["definition"];
        let productions = definition["grammar_schema"]["productions"]
            .as_array()
            .expect("productions");
        let nodes = definition["typed_node_schema"]["nodes"]
            .as_array()
            .expect("node entries");
        assert_eq!(productions.len(), nodes.len(), "one node entry per production");
        for production in productions {
            let node = nodes
                .iter()
                .find(|node| node["keyword"] == production["keyword"])
                .expect("each production has its node entry");
            let names = node["clauses"]
                .as_array()
                .expect("clauses")
                .iter()
                .map(|clause| clause["name"].clone())
                .collect::<Vec<_>>();
            assert_eq!(json!(names), production["clauses"], "clause names in production order");
        }
    }
}

#[trace("TC-951", "FR-469-AC-10")]
#[test]
fn tc_951_withholding_a_published_definition_refuses_its_fixture() {
    let vectors = parsed_fixture(EXTENSION_DEFINITION_VECTORS);
    assert_eq!(vectors["definitions"], "extension-definitions.json");
    let all = published_extensions();
    for vector in vectors["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        let package = match vector["fixture"].as_str() {
            Some("fixtures/positive-extension-nodes.json") => extension_package(),
            Some("fixtures/positive-all-families.json") => parsed_fixture(POSITIVE_ALL_FAMILIES),
            other => panic!("{id}: unexpected fixture {other:?}"),
        };
        let withheld = vector["withheld"].as_array().expect("withheld");
        let selected = package["lock"]["definition_selections"]
            .as_array()
            .expect("definition selections");
        let held = all
            .iter()
            .filter(|held| selected.contains(&held.definition) && !withheld.contains(&held.definition))
            .cloned()
            .collect::<Vec<_>>();
        let expected = &vector["expected"];
        let actual = read_extension_package(&package, &held);
        if expected["result"] == "admitted" {
            assert_eq!(actual, Ok(()), "{id}");
        } else {
            let refusal = actual.expect_err(id);
            assert_eq!(refusal.code, expected["code"], "{id}");
            assert_eq!(json!(refusal.cause), expected["cause"], "{id}");
            assert_eq!(refusal.locus.as_ref(), Some(&expected["locus"]), "{id}");
            assert_eq!(refusal.pointer, expected["document_pointer"], "{id}");
        }
    }
}

const HYBRID_NODE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/hybrid-node-vectors.json");

/// The unique, digest-ascending `reference` targets of a body, at any depth.
fn body_reference_targets(body: &Value) -> Vec<Value> {
    let mut found = Vec::new();
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        match term {
            Value::Object(members) => {
                if members.get("term") == Some(&json!("reference")) {
                    found.push(members["target"].clone());
                }
                pending.extend(members.values());
            }
            Value::Array(items) => pending.extend(items),
            _ => {}
        }
    }
    found.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
    found.dedup();
    found
}

/// Every checked-semantic NodeId-shaped reference in a positive vector must
/// name a node in that vector's complete node document.
fn assert_semantic_node_refs_resolve(nodes: &[Value], label: &str) {
    let ids = nodes
        .iter()
        .map(|node| node["node_id"]["digest"].as_str().expect("node digest"))
        .collect::<BTreeSet<_>>();
    fn visit(value: &Value, path: &str, ids: &BTreeSet<&str>, label: &str) {
        match value {
            Value::Object(fields) => {
                if fields.get("domain") == Some(&json!("quire.checked-semantic-node/v1")) {
                    let digest = fields["digest"].as_str().expect("semantic NodeId digest");
                    assert!(
                        ids.contains(digest),
                        "{label}{path} references absent node {digest}"
                    );
                }
                for (name, child) in fields {
                    visit(child, &format!("{path}/{name}"), ids, label);
                }
            }
            Value::Array(children) => {
                for (index, child) in children.iter().enumerate() {
                    visit(child, &format!("{path}/{index}"), ids, label);
                }
            }
            _ => {}
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        visit(node, &format!("/{index}"), &ids, label);
    }
}

/// The binding of `name` in an aggregate body's members.
fn hybrid_binding<'a>(body: &'a Value, name: &str) -> &'a Value {
    &body["members"]
        .as_array()
        .expect("aggregate members")
        .iter()
        .find(|member| member["name"] == name)
        .unwrap_or_else(|| panic!("binding {name}"))["value"]
}

/// Whether a node's key is the digest of its own preimage: the
/// `quire.application-node/v1` preimage for an application-bearing node,
/// else the `quire.structural-node/v1` preimage.
fn hybrid_vector_key_is_current(node: &Value, nodes: &[Value]) -> bool {
    if is_application_node(node) {
        application_node_key_is_current(node, nodes)
    } else {
        structural_node_key_is_current(node)
    }
}

/// Rekey `node` from its own members, keeping a self-typed node self-typed.
fn rekey_hybrid_vector_node(node: &mut Value, nodes: &[Value]) {
    let self_typed = node["semantic_type"] == node["node_id"];
    node["dependencies"] = json!(body_reference_targets(&node["body"]));
    let preimage = if is_application_node(node) {
        application_node_preimage(node, nodes).expect("application preimage")
    } else {
        structural_node_preimage(node).expect("structural preimage")
    };
    node["node_id"]["digest"] = json!(unqualified_jcs_sha256(&preimage));
    if self_typed {
        node["semantic_type"] = node["node_id"].clone();
    }
}

#[trace("TC-966", "FR-481-AC-1", "FR-481-AC-2", "FR-481-AC-6")]
#[test]
fn tc_966_hybrid_model_nodes_meet_the_schema_and_carry_their_own_keys() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let nodes = vectors["nodes"].as_array().expect("nodes");
    let mut positive_document = nodes.clone();
    positive_document.extend(
        vectors["extra_nodes"]
            .as_array()
            .expect("extra nodes")
            .iter()
            .cloned(),
    );
    positive_document.extend(
        vectors["restored_nodes"]
            .as_array()
            .expect("restored nodes")
            .iter()
            .cloned(),
    );
    assert_semantic_node_refs_resolve(&positive_document, "hybrid positive vectors");
    let roles = &vectors["roles"];
    let node_schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let preimage_schema = schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "HybridNode");
    let ids = nodes
        .iter()
        .map(|node| node["node_id"]["digest"].as_str().expect("digest"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), nodes.len(), "every node id is unique");
    for (index, node) in nodes.iter().enumerate() {
        assert!(
            node_schema.is_valid(node),
            "node {index} meets SemanticNode"
        );
        assert!(
            hybrid_vector_key_is_current(node, nodes),
            "node {index} key is current"
        );
        if node["node_tag"] == "hybrid" {
            let preimage = structural_node_preimage(node).expect("hybrid nodes are structural");
            assert!(
                preimage_schema.is_valid(&preimage),
                "node {index} preimage meets HybridNode"
            );
            assert_eq!(
                node["semantic_type"], node["node_id"],
                "node {index} is self-typed"
            );
            assert_eq!(
                node["dependencies"],
                json!(body_reference_targets(&node["body"])),
                "node {index} dependencies are its body's targets"
            );
        }
    }
    let at = |role: &Value| &nodes[role.as_u64().expect("index") as usize];
    let model = at(&roles["model"]);
    assert_eq!(model["semantic_form"], "hybrid_model");
    assert_eq!(model["declaration"], json!({"qualified_name": ["Braking"]}));
    assert_eq!(model["owner"]["kind"], "source");
    let modes = roles["modes"].as_array().expect("modes");
    for (mode, name) in modes.iter().zip(["Moving", "Stopped"]) {
        let mode = at(mode);
        assert_eq!(
            mode["declaration"],
            json!({"qualified_name": ["Braking", name]})
        );
        assert_eq!(mode["owner"], model["owner"]);
    }
    assert_eq!(
        hybrid_binding(&model["body"], "modes")["members"],
        json!(
            modes
                .iter()
                .map(|mode| json!({"term": "reference", "target": at(mode)["node_id"]}))
                .collect::<Vec<_>>()
        ),
        "the initial mode is the first"
    );
    for transition in roles["transitions"].as_array().expect("transitions") {
        let transition = at(transition);
        assert!(transition.get("declaration").is_none() && transition.get("owner").is_none());
        assert_eq!(transition["occurrences"][0]["role"], "expression");
    }
    // Variables: v, a, t at levels 0, 1, 2, typed rational; t is the clock.
    let rational = nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "rational")
        .expect("rational");
    let entries = hybrid_binding(&model["body"], "variables")["members"]
        .as_array()
        .expect("variables");
    let classes = entries
        .iter()
        .map(|entry| {
            hybrid_binding(entry, "class")["value"]
                .as_str()
                .expect("class")
        })
        .collect::<Vec<_>>();
    assert_eq!(classes, ["hybrid", "hybrid", "clock"]);
    for (level, (entry, variable)) in entries
        .iter()
        .zip(roles["variables"].as_array().expect("variables"))
        .enumerate()
    {
        let variable = at(variable);
        assert_eq!(
            hybrid_binding(entry, "parameter")["target"],
            variable["node_id"]
        );
        assert_eq!(variable["semantic_type"], rational["node_id"]);
        assert_eq!(
            hybrid_binding(&variable["body"], "level")["value"],
            json!(level.to_string())
        );
    }
    // The clock's rate is the rational literal one in every mode.
    for mode in modes {
        let rates = hybrid_binding(&at(mode)["body"], "rates")["members"]
            .as_array()
            .expect("rates");
        assert_eq!(rates.len(), entries.len(), "one rate per variable");
        let clock_rate = nodes
            .iter()
            .find(|node| node["node_id"] == rates[2]["target"])
            .expect("clock rate node");
        assert_eq!(clock_rate["semantic_form"], "literal");
        assert_eq!(clock_rate["body"]["value"], "1");
    }
    // The claim is a temporal clause over a parameter typed by the model node.
    let claim = at(&roles["claim"]);
    let over = at(&roles["over"]);
    assert_eq!(claim["semantic_form"], "temporal_clause");
    assert_eq!(claim["body"]["arguments"][0]["target"], over["node_id"]);
    assert_eq!(over["semantic_type"], model["node_id"]);
    assert_eq!(claim["body"]["arguments"][1]["value"], "model-time");
    assert_eq!(
        claim["body"]["operation"]["laws"],
        json!([{"role": "temporal_profile", "definition": {"authority": "agent-ix", "identity": "quire.temporal.infinite-trace/v1"}}])
    );
    assert_eq!(
        claim["body"]["arguments"][2],
        json!({"term": "aggregate", "members": [{"term": "binding", "name": "kind", "value": {"term": "literal", "type": nodes.iter().find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "text").expect("text")["node_id"], "value_kind": "text", "value": "origin"}}]})
    );
    assert_eq!(
        claim["body"]["arguments"][3]["members"],
        json!([]),
        "no captures"
    );
    assert_eq!(
        claim["body"]["arguments"][4]["members"],
        json!([]),
        "no fairness"
    );
    let node_at = |id: &Value| {
        nodes
            .iter()
            .find(|node| node["node_id"] == *id)
            .expect("referenced node")
    };
    let always = node_at(&claim["body"]["arguments"][5]["target"]);
    assert_eq!(
        always["body"]["operation"]["identity"],
        "quire.op.temporal.always"
    );
    assert_eq!(
        always["body"]["operation"]["member"],
        json!({"kind": "temporal_interval", "interval": {"lower": "0", "upper": "10"}})
    );
    let holds = node_at(&always["body"]["arguments"][0]["target"]);
    assert_eq!(
        holds["body"]["operation"]["identity"],
        "quire.op.temporal.holds"
    );
    let below = node_at(&holds["body"]["arguments"][0]["target"]);
    assert_eq!(
        below["body"]["operation"]["identity"],
        "quire.op.rational.lt"
    );
    let v = at(&roles["variables"][0]);
    assert_eq!(
        below["body"]["arguments"][0]["target"], v["node_id"],
        "holds reads v"
    );
    let thirty = node_at(&below["body"]["arguments"][1]["target"]);
    assert_eq!(thirty["body"]["value"], "30");
    // Moving's rates are -a, 0 and 1; its transition is target 1, guard
    // v = 0 and reset (0, 0).
    let a = at(&roles["variables"][1]);
    let moving = at(&modes[0]);
    let rates = hybrid_binding(&moving["body"], "rates")["members"]
        .as_array()
        .expect("rates");
    let negate = node_at(&rates[0]["target"]);
    assert_eq!(
        negate["body"]["operation"]["identity"],
        "quire.op.rational.negate"
    );
    assert_eq!(negate["body"]["arguments"][0]["target"], a["node_id"]);
    assert_eq!(node_at(&rates[1]["target"])["body"]["value"], "0");
    assert_eq!(node_at(&rates[2]["target"])["body"]["value"], "1");
    let transitions = hybrid_binding(&moving["body"], "transitions")["members"]
        .as_array()
        .expect("transitions");
    assert_eq!(transitions.len(), 1);
    let transition = node_at(&transitions[0]["target"]);
    assert_eq!(hybrid_binding(&transition["body"], "target")["value"], "1");
    let guard = node_at(&hybrid_binding(&transition["body"], "guard")["target"]);
    assert_eq!(
        guard["body"]["operation"]["identity"],
        "quire.op.rational.eq"
    );
    assert_eq!(guard["body"]["arguments"][0]["target"], v["node_id"]);
    assert_eq!(
        node_at(&guard["body"]["arguments"][1]["target"])["body"]["value"],
        "0"
    );
    let resets = hybrid_binding(&transition["body"], "resets")["members"]
        .as_array()
        .expect("resets");
    assert_eq!(resets.len(), 1);
    assert_eq!(hybrid_binding(&resets[0], "variable")["value"], "0");
    assert_eq!(
        node_at(&hybrid_binding(&resets[0], "value")["target"])["body"]["value"],
        "0"
    );
    assert!(
        hybrid_binding(&at(&modes[1])["body"], "transitions")["members"]
            .as_array()
            .expect("none")
            .is_empty()
    );
}

#[trace("TC-966", "FR-481-AC-3")]
#[test]
fn tc_966_a_changed_rate_rekeys_its_mode_the_model_and_the_claim() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let mut nodes = vectors["nodes"].as_array().expect("nodes").clone();
    let extra = vectors["extra_nodes"].as_array().expect("extra nodes");
    let roles = &vectors["roles"];
    let index = |role: &Value| role.as_u64().expect("index") as usize;
    let moving = index(&roles["modes"][0]);
    let model = index(&roles["model"]);
    let over = index(&roles["over"]);
    let claim = index(&roles["claim"]);
    let before = [moving, model, over, claim].map(|at| nodes[at]["node_id"].clone());
    let replacement = extra[index(&roles["replacement_rate_of_v"])]["node_id"].clone();
    assert_eq!(
        nodes[moving]["body"]["members"][1]["value"]["members"][0]["target"],
        nodes[index(&roles["moving_rate_of_v"])]["node_id"]
    );
    for node in extra {
        assert!(
            hybrid_vector_key_is_current(node, &nodes),
            "an added term node holds its own key"
        );
    }
    nodes.extend(extra.iter().cloned());
    nodes[moving]["body"]["members"][1]["value"]["members"][0]["target"] = replacement;
    let snapshot = nodes.clone();
    rekey_hybrid_vector_node(&mut nodes[moving], &snapshot);
    let moving_id = nodes[moving]["node_id"].clone();
    nodes[model]["body"]["members"][1]["value"]["members"][0]["target"] = moving_id;
    let snapshot = nodes.clone();
    rekey_hybrid_vector_node(&mut nodes[model], &snapshot);
    nodes[over]["semantic_type"] = nodes[model]["node_id"].clone();
    let snapshot = nodes.clone();
    rekey_hybrid_vector_node(&mut nodes[over], &snapshot);
    nodes[claim]["body"]["arguments"][0]["target"] = nodes[over]["node_id"].clone();
    let snapshot = nodes.clone();
    rekey_hybrid_vector_node(&mut nodes[claim], &snapshot);
    for (at, old) in [moving, model, over, claim].into_iter().zip(before) {
        assert_ne!(nodes[at]["node_id"], old, "node {at} is rekeyed");
        assert!(
            hybrid_vector_key_is_current(&nodes[at], &nodes),
            "node {at} key is current"
        );
    }
    // A respelled source with the same checked content keeps every key: the
    // keys are a function of the nodes alone, so rekeying unchanged nodes is
    // the identity.
    let original = vectors["nodes"].as_array().expect("nodes").clone();
    for at in [moving, model, over, claim] {
        let mut copy = original[at].clone();
        rekey_hybrid_vector_node(&mut copy, &original);
        assert_eq!(
            copy["node_id"], original[at]["node_id"],
            "node {at} is stable"
        );
    }
}

#[trace("TC-966", "FR-481-AC-5")]
#[test]
fn tc_966_malformed_hybrid_nodes_fail_the_published_schema() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let nodes = vectors["nodes"].as_array().expect("nodes");
    let node_schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let cases = vectors["schema_refusals"]
        .as_array()
        .expect("schema refusals");
    assert_eq!(cases.len(), 8);
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let mut node = nodes[case["node"].as_u64().expect("node") as usize].clone();
        assert!(
            node_schema.is_valid(&node),
            "{name}: the unedited node is valid"
        );
        let pointer = case["pointer"].as_str().expect("pointer");
        let (parent, member) = pointer.rsplit_once('/').expect("member pointer");
        let holder = node.pointer_mut(parent).expect("parent exists");
        match (&case["value"], holder) {
            (Value::Null, Value::Object(members)) => {
                assert!(
                    members.remove(member).is_some(),
                    "{name}: removed member exists"
                );
            }
            (value, Value::Object(members)) => {
                members.insert(member.to_owned(), value.clone());
            }
            (value, Value::Array(items)) => {
                items[member.parse::<usize>().expect("index")] = value.clone();
            }
            (_, other) => panic!("{name}: unexpected parent {other}"),
        }
        assert!(!node_schema.is_valid(&node), "{name}: {}", case["why"]);
    }
}

/// Whether a `decimal` literal's `value` is a member of `Decimal[lo, hi; smin,
/// smax]` by FR-140: the value is normalized, `s* = max(s, smin)` and the value
/// is a member exactly when `s* <= smax` and `v x 10^s*` lies in `[lo, hi]`. The
/// written coefficient and scale never decide it.
fn decimal_literal_is_member(text: &str, lo: i128, hi: i128, smin: u32, smax: u32) -> bool {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (integer, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let mut scale = fraction.len() as u32;
    let Ok(mut coefficient) = format!("{integer}{fraction}").parse::<i128>() else {
        return false;
    };
    while scale > 0 && coefficient % 10 == 0 {
        coefficient /= 10;
        scale -= 1;
    }
    if coefficient == 0 {
        scale = 0;
    }
    if negative {
        coefficient = -coefficient;
    }
    let membership_scale = scale.max(smin);
    if membership_scale > smax {
        return false;
    }
    let widened = 10i128
        .checked_pow(membership_scale - scale)
        .and_then(|factor| coefficient.checked_mul(factor));
    match widened {
        Some(c) => (lo..=hi).contains(&c),
        None => false,
    }
}

/// FR-322-AC-113: the canonical spelling and the FR-140 value membership of a
/// `decimal` literal's `value`.
#[test]
fn decimal_literal_values_are_canonical_spellings() {
    for admitted in [
        "1.25", "0.05", "10", "0.00", "1000", "0.5", "-0.5", "0", "1000.00", "1.250", "5",
    ] {
        assert!(decimal_literal_is_canonical(admitted), "{admitted}");
    }
    for refused in [
        ".5", "1.", "+1.5", "01.5", "-0.0", "-0", "1e2", "1/4", "", "-", "1.2.3",
    ] {
        assert!(!decimal_literal_is_canonical(refused), "{refused}");
    }
}

#[trace("TC-233", "FR-322-AC-118")]
#[test]
fn tc_233_dimension_equivalence_node_is_a_self_typed_pair_of_dimension_keys() {
    let fixture = parsed_fixture(NODE_IDENTITY_VECTORS);
    let vectors = fixture["vectors"]
        .as_array()
        .expect("node identity vectors are an array");
    let equivalence = vectors
        .iter()
        .find(|vector| vector["name"].as_str() == Some("dimension-equivalence-rpc-flow-time"))
        .expect("the equivalence vector exists");
    assert!(dimension_equivalence_pairs_are_admissible(vectors));

    let node_id = json!({"domain": "quire.checked-semantic-node/v1", "digest": equivalence["sha256"]});
    let ids = equivalence["preimage"]["dimension_node_ids"].clone();
    let node = json!({
        "node_id": node_id,
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "scalar_type",
        "semantic_form": "dimension_equivalence",
        "semantic_type": node_id,
        "dependencies": ids,
        "occurrences": [{"role": "anchor", "ordinal": 0}],
        "nominal_identity_preimage": equivalence["preimage"],
        "body": {"term": "aggregate", "members": []}
    });
    let wire = schema_for_def(&parsed_fixture(SCHEMA), "ScalarTypeNode");
    assert!(wire.is_valid(&node), "the wire schema admits the equivalence node");
    let mut wire_declared = node.clone();
    wire_declared["declaration"] = json!({"qualified_name": ["Time"]});
    assert!(!wire.is_valid(&wire_declared), "a nameless node carries no declaration");
    let mut wire_unpreimaged = node.clone();
    wire_unpreimaged.as_object_mut().unwrap().remove("nominal_identity_preimage");
    assert!(!wire.is_valid(&wire_unpreimaged), "the node carries its preimage");
    assert!(nominal_node_cross_fields_are_valid(&node));
    let mut typed_elsewhere = node.clone();
    typed_elsewhere["semantic_type"] = ids[0].clone();
    assert!(!nominal_node_cross_fields_are_valid(&typed_elsewhere));
    let mut one_dependency = node.clone();
    one_dependency["dependencies"] = json!([ids[0]]);
    assert!(!nominal_node_cross_fields_are_valid(&one_dependency));
    let mut declared = node.clone();
    declared["declaration"] = json!({"qualified_name": ["Time"]});
    assert!(!nominal_node_cross_fields_are_valid(&declared));

    // A repeated pair is refused.
    let mut repeated = vectors.to_vec();
    repeated.push(equivalence.clone());
    assert!(!dimension_equivalence_pairs_are_admissible(&repeated));

    // Two derived dimensions are admissible only when their maps are equal under the
    // base classes the base pairs make.
    let base = |digest: &str| {
        json!({"sha256": digest, "preimage": {"version": "quire.dimension-node/v1", "terms": []}})
    };
    let derived = |digest: &str, term: &str| {
        json!({"sha256": digest, "preimage": {"version": "quire.dimension-node/v1", "terms": [
            {"dimension_node_id": {"digest": term}, "exponent": "-1"}]}})
    };
    let pair_scaled = |digest: &str, left: &str, right: &str, numerator: &str, denominator: &str| {
        json!({"sha256": digest, "preimage": {
            "version": "quire.dimension-equivalence-node/v1",
            "dimension_node_ids": [{"digest": left}, {"digest": right}],
            "root_scale": {"numerator": numerator, "denominator": denominator}}})
    };
    let pair = |digest: &str, left: &str, right: &str| pair_scaled(digest, left, right, "1", "1");
    let mut speeds = vec![
        base("a1"),
        base("b1"),
        derived("a2", "a1"),
        derived("b2", "b1"),
        pair("e2", "a2", "b2"),
    ];
    assert!(
        !dimension_equivalence_pairs_are_admissible(&speeds),
        "derived dimensions over unrelated bases differ"
    );
    speeds.push(pair("e1", "a1", "b1"));
    assert!(
        dimension_equivalence_pairs_are_admissible(&speeds),
        "the base pair makes the maps equal"
    );

    // Unequal roots: with 1 root of a1 = 1000 roots of b1, the reciprocal dimensions carry
    // the reciprocal ratio, and any other root ratio contradicts the base roots.
    let mut scaled = vec![
        base("a1"),
        base("b1"),
        derived("a2", "a1"),
        derived("b2", "b1"),
        pair_scaled("e1", "a1", "b1", "1000", "1"),
        pair_scaled("e2", "a2", "b2", "1", "1000"),
    ];
    assert!(
        dimension_equivalence_pairs_are_admissible(&scaled),
        "the derived pair carries the ratio the base roots imply"
    );
    scaled[5] = pair_scaled("e2", "a2", "b2", "1", "1");
    assert!(
        !dimension_equivalence_pairs_are_admissible(&scaled),
        "a derived pair contradicting the base roots is refused"
    );

    // The ratios around a cycle of base pairs multiply to exactly one.
    let cycle = |closing_numerator: &str| {
        vec![
            base("c1"),
            base("c2"),
            base("c3"),
            pair_scaled("f1", "c1", "c2", "1", "1"),
            pair_scaled("f2", "c2", "c3", "1000", "1"),
            pair_scaled("f3", "c1", "c3", closing_numerator, "1"),
        ]
    };
    assert!(dimension_equivalence_pairs_are_admissible(&cycle("1000")));
    assert!(
        !dimension_equivalence_pairs_are_admissible(&cycle("1001")),
        "a cycle whose ratios do not multiply to one is refused"
    );

    // The file vectors close the cycle rpc, flow, clock; changing one ratio breaks it.
    let mut broken = vectors.to_vec();
    for vector in &mut broken {
        if vector["name"].as_str() == Some("dimension-equivalence-flow-clock-time") {
            vector["preimage"]["root_scale"]["numerator"] = json!("1001");
        }
    }
    assert!(
        !dimension_equivalence_pairs_are_admissible(&broken),
        "the vectors' cycle is consistent only with its authored ratio"
    );

    // A zero, negative or unreduced ratio is no root ratio.
    for refused in [("0", "1"), ("-1", "1"), ("2", "2")] {
        let mut bad = vectors.to_vec();
        for vector in &mut bad {
            if vector["name"].as_str() == Some("dimension-equivalence-rpc-flow-time") {
                vector["preimage"]["root_scale"] =
                    json!({"numerator": refused.0, "denominator": refused.1});
            }
        }
        assert!(!dimension_equivalence_pairs_are_admissible(&bad), "{refused:?}");
    }
}

/// FR-322-AC-113: membership is decided by value, not by the written spelling.
#[test]
fn decimal_literal_membership_is_decided_by_value() {
    for member in [
        "1.25", "0.05", "10", "0.00", "1000", "0.5", "1000.00", "1.250",
    ] {
        assert!(decimal_literal_is_member(member, 0, 1000, 0, 2), "{member}");
    }
    for refused in ["1.255", "1001", "1000.01", "-1"] {
        assert!(!decimal_literal_is_member(refused, 0, 1000, 0, 2), "{refused}");
    }
    for member in ["-0.5", "5", "100"] {
        assert!(decimal_literal_is_member(member, -1000, 1000, 1, 2), "{member}");
    }
    assert!(!decimal_literal_is_member("1001", -1000, 1000, 1, 2));
}

#[trace("TC-966", "FR-481-AC-13", "FR-322-AC-13")]
#[test]
fn tc_966_the_all_families_fixture_carries_a_hybrid_model_with_bare_variable_terms() {
    let package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    let node_at = |id: &Value| {
        nodes
            .iter()
            .find(|node| node["node_id"] == *id)
            .expect("referenced node")
    };
    let hybrid = nodes
        .iter()
        .filter(|node| node["node_tag"] == "hybrid")
        .collect::<Vec<_>>();
    let forms = hybrid
        .iter()
        .map(|node| node["semantic_form"].as_str().expect("form"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        forms,
        BTreeSet::from(["hybrid_model", "hybrid_mode", "hybrid_transition"])
    );
    for node in &hybrid {
        assert!(
            structural_node_key_is_current(node),
            "a hybrid node holds its own key"
        );
        assert_eq!(
            node["dependencies"],
            json!(body_reference_targets(&node["body"]))
        );
    }
    let model = hybrid
        .iter()
        .find(|node| node["semantic_form"] == "hybrid_model")
        .expect("model");
    assert_eq!(
        model["declaration"],
        json!({"qualified_name": ["Example", "Tank"]})
    );
    let entries = hybrid_binding(&model["body"], "variables")["members"]
        .as_array()
        .expect("variables");
    let variables = entries
        .iter()
        .map(|entry| &hybrid_binding(entry, "parameter")["target"])
        .collect::<Vec<_>>();
    let classes = entries
        .iter()
        .map(|entry| {
            hybrid_binding(entry, "class")["value"]
                .as_str()
                .expect("class")
        })
        .collect::<Vec<_>>();
    assert_eq!(classes, ["hybrid", "hybrid"]);
    let mode = node_at(&hybrid_binding(&model["body"], "modes")["members"][0]["target"]);
    assert_eq!(
        mode["declaration"],
        json!({"qualified_name": ["Example", "Tank", "Filling"]})
    );
    let rates = hybrid_binding(&mode["body"], "rates")["members"]
        .as_array()
        .expect("rates");
    assert_eq!(
        &rates[0]["target"], variables[1],
        "x' = y is a bare variable reference"
    );
    let one = node_at(&rates[1]["target"]);
    assert_eq!(
        (one["semantic_form"].as_str(), one["body"]["value"].as_str()),
        (Some("literal"), Some("1"))
    );
    assert!(literal_value_is_canonical(&one["body"]));
    let transition = node_at(&hybrid_binding(&mode["body"], "transitions")["members"][0]["target"]);
    let reset = &hybrid_binding(&transition["body"], "resets")["members"][0];
    assert_eq!(
        &hybrid_binding(reset, "value")["target"],
        variables[1],
        "x is reset to y"
    );
    // FR-481 checks 3.2 and 3.3: every invariant and guard is a Boolean term,
    // every rate and reset value a rational term or one of the variables.
    let form_of = |id: &Value| {
        let ty = node_at(&node_at(id)["semantic_type"]);
        (
            ty["node_tag"].as_str().expect("tag").to_owned(),
            ty["semantic_form"].as_str().expect("form").to_owned(),
        )
    };
    let boolean = ("scalar_type".to_owned(), "boolean".to_owned());
    let rational = ("scalar_type".to_owned(), "rational".to_owned());
    let rational_term = |id: &Value| variables.contains(&id) || form_of(id) == rational;
    assert_eq!(
        form_of(&hybrid_binding(&mode["body"], "invariant")["target"]),
        boolean
    );
    assert_eq!(
        form_of(&hybrid_binding(&transition["body"], "guard")["target"]),
        boolean
    );
    for target in [
        &rates[0]["target"],
        &rates[1]["target"],
        &hybrid_binding(reset, "value")["target"],
    ] {
        assert!(
            rational_term(target),
            "a rate or reset value is a rational term"
        );
    }
    for variable in &variables {
        assert_eq!(form_of(variable), rational, "a variable is typed rational");
    }
    let mapped = package["source_map"].as_array().expect("source map");
    for node in &hybrid {
        assert!(
            mapped
                .iter()
                .any(|entry| entry["node_id"] == node["node_id"]),
            "each hybrid node has a source-map entry"
        );
    }
}

#[trace("TC-966", "FR-481-AC-8", "FR-481-AC-9")]
#[test]
fn tc_966_restored_hybrid_forms_meet_the_schema_and_carry_their_own_keys() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let base = vectors["nodes"].as_array().expect("nodes");
    let restored = vectors["restored_nodes"].as_array().expect("restored nodes");
    let roles = &vectors["restored_roles"];
    let all = base.iter().chain(restored).cloned().collect::<Vec<_>>();
    let node_schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let preimage_schema = schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "HybridNode");
    let ids = all
        .iter()
        .map(|node| node["node_id"]["digest"].as_str().expect("digest"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), all.len(), "every node id is unique");
    for (index, node) in restored.iter().enumerate() {
        assert!(node_schema.is_valid(node), "restored node {index} meets SemanticNode");
        assert!(
            hybrid_vector_key_is_current(node, &all),
            "restored node {index} key is current"
        );
        if node["node_tag"] == "hybrid" {
            let preimage = structural_node_preimage(node).expect("hybrid nodes are structural");
            assert!(preimage_schema.is_valid(&preimage), "restored node {index} preimage");
            assert_eq!(node["semantic_type"], node["node_id"], "restored node {index} is self-typed");
            assert_eq!(node["dependencies"], json!(body_reference_targets(&node["body"])));
        }
    }
    let at = |role: &Value| &restored[role.as_u64().expect("index") as usize];
    let by_id = |id: &Value| all.iter().find(|node| node["node_id"] == *id).expect("node");
    let model = at(&roles["model"]);
    assert_eq!(model["declaration"], json!({"qualified_name": ["Braking2"]}));
    let names = model["body"]["members"]
        .as_array()
        .expect("members")
        .iter()
        .map(|member| member["name"].clone())
        .collect::<Vec<_>>();
    assert_eq!(names, [json!("variables"), json!("modes"), json!("initial")]);
    let initial = hybrid_binding(&model["body"], "initial")["members"]
        .as_array()
        .expect("initial entries");
    let modes = initial
        .iter()
        .map(|entry| hybrid_binding(entry, "mode")["value"].clone())
        .collect::<Vec<_>>();
    assert_eq!(modes, [json!("0"), json!("1")], "two initial modes");
    let conditions = roles["initial_conditions"].as_array().expect("conditions");
    for (entry, role) in initial.iter().zip(conditions) {
        assert_eq!(hybrid_binding(entry, "condition")["target"], at(role)["node_id"]);
        assert_eq!(at(role)["body"]["operation"]["identity"], "quire.op.boolean.and");
    }
    // Braking keeps the default initial entry, so its model has no binding.
    let braking = &base[vectors["roles"]["model"].as_u64().expect("model") as usize];
    assert_eq!(braking["body"]["members"].as_array().expect("members").len(), 2);
    let transition = at(&roles["transition"]);
    let resets = hybrid_binding(&transition["body"], "resets")["members"]
        .as_array()
        .expect("resets");
    assert_eq!(resets[0]["members"].as_array().expect("entry").len(), 2, "`=` has no relation");
    assert_eq!(hybrid_binding(&resets[1], "relation")["value"], "ge");
    // Slows: each activation, trigger at level 1, read at level 3, capture at level 6.
    let slows = at(&roles["each_claim"]);
    let arguments = &slows["body"]["arguments"];
    assert_eq!(arguments[1]["value"], "model-time");
    assert_eq!(hybrid_binding(&arguments[2], "kind")["value"], "each");
    let trigger = at(&roles["trigger"]);
    assert_eq!(hybrid_binding(&arguments[2], "trigger")["target"], trigger["node_id"]);
    assert_eq!(trigger["semantic_type"], model["node_id"]);
    assert_eq!(hybrid_binding(&trigger["body"], "level")["value"], "1");
    let read = at(&roles["activation_read"]);
    assert_eq!(hybrid_binding(&read["body"], "level")["value"], "3");
    assert_eq!(hybrid_binding(&arguments[3], "v0")["target"], read["node_id"]);
    let capture = at(&roles["capture"]);
    assert_eq!(hybrid_binding(&capture["body"], "level")["value"], "6");
    let capture_type = at(&roles["capture_type"]);
    assert_eq!(capture["semantic_type"], capture_type["node_id"]);
    assert_eq!(capture_type["semantic_form"], "rational_range");
    assert_eq!(hybrid_binding(&capture_type["body"], "numerator_max")["value"], "20");
    assert_eq!(
        slows["body"]["operation"]["laws"][0]["definition"]["identity"],
        "quire.temporal.timed/v1"
    );
    let fairness = by_id(&arguments[4]["members"][0]["target"]);
    assert_eq!(fairness["node_id"], at(&roles["fairness"])["node_id"]);
    let member = &fairness["body"]["operation"]["member"];
    assert_eq!(member["declaration"], at(&roles["modes"][0])["node_id"]);
    assert_eq!(member["name"], "Stopped");
    let eventually = by_id(&arguments[5]["target"]);
    assert_eq!(
        eventually["body"]["operation"]["member"]["interval"]["upper"],
        json!({"numerator": "21", "denominator": "2"})
    );
    let recent = at(&roles["steps_claim"]);
    assert_eq!(recent["body"]["arguments"][1]["value"], "model-steps");
    let once = by_id(&recent["body"]["arguments"][5]["target"]);
    assert_eq!(once["body"]["operation"]["identity"], "quire.op.temporal.once");
    assert_eq!(
        once["body"]["operation"]["member"]["interval"],
        json!({"lower": "0", "upper": "2"})
    );
}

#[trace("TC-966", "FR-481-AC-11")]
#[test]
fn tc_966_restored_hybrid_shapes_meet_or_fail_the_published_schema() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let restored = vectors["restored_nodes"].as_array().expect("restored nodes");
    let roles = &vectors["restored_roles"];
    let node_schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let at = |role: &Value| restored[role.as_u64().expect("index") as usize].clone();
    let mut empty_initial = at(&roles["model"]);
    empty_initial["body"]["members"][2]["value"]["members"] = json!([]);
    assert!(!node_schema.is_valid(&empty_initial), "an empty initial fails");
    let mut no_condition = at(&roles["model"]);
    no_condition["body"]["members"][2]["value"]["members"][0]["members"]
        .as_array_mut()
        .expect("entry")
        .pop();
    assert!(!node_schema.is_valid(&no_condition), "an entry without condition fails");
    let mut approx = at(&roles["transition"]);
    approx["body"]["members"][2]["value"]["members"][1]["members"][2]["value"]["value"] =
        json!("approx");
    assert!(!node_schema.is_valid(&approx), "an unknown relation fails");
    approx["body"]["members"][2]["value"]["members"][1]["members"][2]["value"]["value"] = json!("eq");
    assert!(!node_schema.is_valid(&approx), "`=` is written with no relation");
    let slows = at(&roles["each_claim"]);
    let eventually_id = slows["body"]["arguments"][5]["target"].clone();
    let mut open = restored
        .iter()
        .find(|node| node["node_id"] == eventually_id)
        .expect("eventually")
        .clone();
    open["body"]["operation"]["member"]["interval"]["upper"] = Value::Null;
    open["body"]["operation"]["member"]["interval"]["upper_end"] = json!("open");
    assert!(node_schema.is_valid(&open), "a timed `[q, *]` meets the schema");
}

#[trace("TC-367", "FR-415-AC-17", "FR-322-AC-120")]
#[test]
fn tc_367_timed_model_node_tag_and_preimage_are_catalogued() {
    let schema = parsed_fixture(SCHEMA);
    let tags = &schema["$defs"]["NodeBase"]["properties"]["node_tag"]["enum"];
    assert!(tags.as_array().unwrap().contains(&json!("timed_model")));
    let forms = schema["$defs"]["TimedModelNode"]["allOf"][1]["properties"]["semantic_form"]["enum"]
        .as_array()
        .unwrap();
    for form in ["timed_model", "time_source", "clock_place", "clock_reset", "clock_guard", "time_invariant", "urgency", "clock_constraint"] {
        assert!(forms.contains(&json!(form)), "{form} is a timed node form");
    }
    let preimages = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let timed = schema_for_def(&preimages, "TimedModelNode");
    let package: Value =
        serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).expect("timed fixture JSON");
    let source = package["semantic_graph"]["nodes"][44].clone();
    let preimage = structural_node_preimage(&source).expect("timed source preimage");
    assert!(timed.is_valid(&preimage), "timed source identity preimage is closed and admissible");
    let mut adverse = preimage;
    adverse["body"]["unexpected"] = json!(true);
    assert!(!timed.is_valid(&adverse), "timed node preimages reject unknown members");
}

#[trace("TC-367", "FR-322-AC-120", "FR-322-AC-121")]
#[test]
fn tc_367_timed_model_fixture_is_closed_self_typed_and_ordered() {
    let package: Value =
        serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).expect("timed fixture JSON");
    let schema = parsed_fixture(SCHEMA);
    let package_validator = jsonschema::validator_for(&schema).expect("checked package schema");
    assert!(
        package_validator.is_valid(&package),
        "all timed forms form a closed checked package: {:?}",
        package_validator
            .iter_errors(&package)
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        package["identity_preimage"],
        derive_identity_preimage(&package)
    );
    assert_eq!(
        package["package_id"]["digest"],
        unqualified_jcs_sha256(&package_id_preimage(&package))
    );
    let tick_package: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES)
        .expect("tick timed fixture JSON");
    assert!(package_validator.is_valid(&tick_package));
    assert_eq!(tick_package["identity_preimage"], derive_identity_preimage(&tick_package));
    assert_eq!(tick_package["package_id"]["digest"], unqualified_jcs_sha256(&package_id_preimage(&tick_package)));
    let tick_nodes = tick_package["semantic_graph"]["nodes"].as_array().unwrap();
    assert_semantic_node_refs_resolve(tick_nodes, "tick timed checked-package fixture");
    let tick_root = &tick_nodes[45];
    assert_eq!(tick_nodes[44]["body"]["members"][0]["value"]["value"], "tick");
    for index in 3..9 {
        assert!(tick_root["body"]["members"][index]["value"]["members"].as_array().unwrap().is_empty());
    }

    let vectors: Value =
        serde_json::from_str(TIMED_MODEL_NODE_VECTORS).expect("timed vectors JSON");
    let indexes = vectors["node_indexes"].as_object().expect("node indexes");
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes");
    assert_semantic_node_refs_resolve(nodes, "timed checked-package fixture");
    for (index, unit) in nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node["node_tag"] == "scalar_type" && node["semantic_form"] == "unit")
    {
        let dimension = &unit["nominal_identity_preimage"]["dimension_node_id"];
        assert_eq!(
            unqualified_jcs_sha256(&unit["nominal_identity_preimage"]),
            unit["node_id"]["digest"],
            "unit node {index} retains its nominal key"
        );
        assert!(
            nominal_node_cross_fields_are_valid(unit),
            "unit node {index} is typed by its dimension and depends on its declared units"
        );
        assert_eq!(
            unit["semantic_type"], *dimension,
            "unit node {index} is typed by its dimension"
        );
        assert!(
            nodes.iter().any(|node| node["node_id"] == *dimension
                && node["node_tag"] == "scalar_type"
                && node["semantic_form"] == "dimension"),
            "unit node {index} points at a declared dimension"
        );
    }
    let timed_schema = schema_for_def(&schema, "TimedModelNode");
    for (name, index) in indexes {
        let node = &nodes[index.as_u64().expect("index") as usize];
        if node["node_tag"] != "timed_model" {
            assert!(
                matches!(name.as_str(), "dimension" | "unit"),
                "only support nodes are not timed forms"
            );
            continue;
        }
        assert_eq!(
            node["node_tag"], "timed_model",
            "{name} has the timed node tag"
        );
        assert_eq!(
            node["semantic_type"], node["node_id"],
            "{name} uses its self type"
        );
        assert!(
            structural_node_key_is_current(node),
            "{name} key hashes its complete body"
        );
        assert_eq!(
            node["dependencies"],
            json!(body_reference_targets(&node["body"])),
            "{name} dependencies match its resolved body references"
        );
        let preimage = structural_node_preimage(node).expect("timed form has structural preimage");
        assert!(
            schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "TimedModelNode")
                .is_valid(&preimage),
            "{name} preimage matches its closed schema"
        );
        assert!(
            timed_schema.is_valid(node),
            "{name} node matches its closed schema"
        );
    }
    let dense_source = nodes[indexes["time_source"].as_u64().unwrap() as usize].clone();
    assert!(
        timed_schema.is_valid(&dense_source),
        "the dense source variant has its exact two-binding layout"
    );
    for refusal in vectors["schema_refusals"]
        .as_array()
        .expect("refusal vectors")
    {
        let index = indexes[refusal["node"].as_str().expect("node role")]
            .as_u64()
            .expect("node index") as usize;
        let mut adverse = package.clone();
        let node = &mut adverse["semantic_graph"]["nodes"][index];
        match refusal["id"].as_str().expect("vector id") {
            "TM-01" => {
                node["body"]["members"].as_array_mut().unwrap().remove(1);
            }
            "TM-02" => {
                node["body"]["members"].as_array_mut().unwrap().push(json!({"term":"binding","name":"extra","value":{"term":"aggregate","members":[]}}));
            }
            "TM-03" => {
                node["body"]["members"][1]["value"]["members"][0]["members"][0]["value"]["value"] =
                    json!("slot");
            }
            "TM-04" => {
                node["body"]["members"].as_array_mut().unwrap().swap(0, 1);
            }
            "TM-05" => {
                node["body"]["members"][1]["value"]["members"] = json!([]);
            }
            other => panic!("unhandled schema refusal vector {other}"),
        }
        assert!(
            !package_validator.is_valid(&adverse),
            "{} mutation is refused",
            refusal["id"]
        );
    }
}

#[trace("TC-367", "FR-322-AC-123", "FR-322-AC-124", "FR-322-AC-125")]
#[test]
fn tc_367_timed_model_authority_bindings_are_explicit_and_joined() {
    let package: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).unwrap();
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let tick_package: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES).unwrap();
    let tick_nodes = tick_package["semantic_graph"]["nodes"].as_array().unwrap();
    let id = |node: &Value| node["node_id"]["digest"].as_str().unwrap().to_owned();
    let target = |node: &Value, member: usize| {
        node["body"]["members"][member]["value"]["target"]["digest"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let root = &nodes[52];
    let dimension_id = target(root, 1);
    let dimension = nodes.iter().find(|node| id(node) == dimension_id).unwrap();
    assert_eq!(dimension["semantic_form"], "dimension");
    assert_eq!(nodes[43]["semantic_type"]["digest"], dimension_id);

    let source = &tick_nodes[44];
    assert_eq!(source["body"]["members"][0]["value"]["value"], "tick");
    assert_eq!(target(source, 2), target(&tick_nodes[45], 0));
    let operation_id = target(source, 3);
    let operation = tick_nodes.iter().find(|node| id(node) == operation_id).unwrap();
    assert_eq!(operation["node_tag"], "state");
    assert_eq!(operation["semantic_form"], "operation_anchor");
    assert_eq!(target(operation, 0), target(source, 2));
    assert_eq!(operation["body"]["members"][1]["value"]["value"], "size");
    let frame_id = target(operation, 2);
    let frame = tick_nodes.iter().find(|node| id(node) == frame_id).unwrap();
    assert_eq!(frame["semantic_form"], "frame");
    assert_eq!(frame["semantic_type"]["digest"], target(source, 2));
    assert!(structural_node_key_is_current(operation));

    let place = &nodes[45];
    let path = place["body"]["members"][1]["value"]["members"].as_array().unwrap();
    let resolved = place["body"]["members"][2]["value"]["members"].as_array().unwrap();
    assert_eq!(path.len(), 2, "the positive path selects a Map value");
    assert_eq!(resolved.len(), path.len());
    assert_eq!(resolved[0]["members"][1]["value"]["value"], "ix://acme/orders/Order");
    assert_eq!(resolved[0]["members"][2]["value"]["value"], "ix://acme/orders/Order/clock");
    assert_eq!(resolved[0]["members"][3]["value"]["value"], "Map<Text, ix://quire/native/Clock>");
    assert_eq!(resolved[0]["members"][4]["value"]["value"], "map");
    assert_eq!(resolved[1]["members"][1]["value"]["value"], "Map<Text, ix://quire/native/Clock>");
    assert_eq!(resolved[1]["members"][3]["value"]["value"], "ix://quire/native/Clock");
}






#[trace("TC-367", "FR-322-AC-126", "FR-322-AC-127", "FR-322-AC-120")]
#[test]
fn tc_367_timed_model_semantic_refusals_are_replayable_packages() {
    let positive: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).unwrap();
    let vectors: Value = serde_json::from_str(TIMED_MODEL_NODE_VECTORS).unwrap();
    let schema = parsed_fixture(SCHEMA);
    let validator = jsonschema::validator_for(&schema).unwrap();
    let rows = vectors["semantic_refusals"].as_array().unwrap();
    assert_eq!(rows.len(), 21);
    let all_ids = ["schema_refusals", "authority_vectors", "semantic_refusals"]
        .into_iter()
        .flat_map(|section| vectors[section].as_array().unwrap())
        .map(|row| row["id"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let expected_count = ["schema_refusals", "authority_vectors", "semantic_refusals"]
        .into_iter()
        .map(|section| vectors[section].as_array().unwrap().len())
        .sum::<usize>();
    assert_eq!(all_ids.len(), expected_count, "all timed vector ids are unique");
    for row in rows {
        let is_tick = row["fixture"] == "tick";
        let mut adverse = if is_tick {
            serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES).unwrap()
        } else {
            positive.clone()
        };
        match row["mutation"].as_str().unwrap() {
            "remove-timed-root" => {
                adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().remove(if is_tick { 45 } else { 52 });
            }
            "dangling-tick-operation" => {
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][3]["value"]["target"]["digest"] =
                    json!("0000000000000000000000000000000000000000000000000000000000000000");
            }
            "function-as-tick-operation" => {
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][3]["value"]["target"] =
                    adverse["semantic_graph"]["nodes"][5]["node_id"].clone();
            }
            "duplicate-clock-place" => {
                let root = if is_tick { 45 } else { 52 };
                let places = adverse["semantic_graph"]["nodes"][root]["body"]["members"][3]["value"]["members"]
                    .as_array_mut().unwrap();
                let duplicate = places[0].clone();
                places.push(duplicate);
            }
            "noncanonical-place-path" => {
                adverse["semantic_graph"]["nodes"][45]["body"]["members"][1]["value"]["members"][0]["members"][1]["value"]["value"] = json!("not-clock");
            }
            "zero-tick-period" => {
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][4]["value"]["value"] = json!("0");
            }
            "missing-clock-resolution-row" => {
                let steps = adverse["semantic_graph"]["nodes"][45]["body"]["members"][1]["value"]["members"]
                    .as_array_mut().unwrap();
                let duplicate = steps[0].clone();
                steps.push(duplicate);
            }
            "mismatched-clock-path-step" => {
                adverse["semantic_graph"]["nodes"][45]["body"]["members"][2]["value"]["members"][0]["members"][0]["value"]["value"] = json!(1);
            }
            "wrong-clock-collection-metadata" => {
                adverse["semantic_graph"]["nodes"][45]["body"]["members"][2]["value"]["members"][0]["members"][4]["value"]["value"] = json!("sequence");
            }
            "undeclared-clock-member" => {
                adverse["semantic_graph"]["nodes"][45]["body"]["members"][2]["value"]["members"][0]["members"][2]["value"]["value"] = json!("ix://acme/orders/Order/unknown");
            }
            "wrong-terminal-clock-type" => {
                adverse["semantic_graph"]["nodes"][45]["body"]["members"][2]["value"]["members"][0]["members"][3]["value"]["value"] = json!("ix://quire/native/Integer");
            }
            "coherent-forged-selected-chain" => {
                let place = &mut adverse["semantic_graph"]["nodes"][45]["body"]["members"];
                place[1]["value"]["members"][1]["members"][1]["value"]["value"] = json!("sequence");
                let rows = &mut place[2]["value"]["members"];
                rows[0]["members"][3]["value"]["value"] = json!("Sequence<Evil>");
                rows[0]["members"][4]["value"]["value"] = json!("sequence");
                rows[1]["members"][1]["value"]["value"] = json!("Evil");
                rows[1]["members"][2]["value"]["value"] = json!("Sequence<Evil>[slot]");
            }
            "tick-model-context-mismatch" => {
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][2]["value"]["target"] =
                    adverse["semantic_graph"]["nodes"][14]["node_id"].clone();
            }
            "unresolved-tick-operation" => {
                adverse["semantic_graph"]["nodes"][46]["body"]["members"][1]["value"]["value"] = json!("missing");
            }
            "tick-source-with-clock-place" => {
                let place = positive["semantic_graph"]["nodes"][45].clone();
                adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(place.clone());
                let root = &mut adverse["semantic_graph"]["nodes"][45];
                root["body"]["members"][3]["value"]["members"].as_array_mut().unwrap().push(json!({"term":"reference", "target":place["node_id"].clone()}));
                root["dependencies"].as_array_mut().unwrap().push(place["node_id"].clone());
                root["dependencies"].as_array_mut().unwrap().sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
            }
            "source-unit-length" | "source-unit-derived-non-time" | "source-unit-derived-time" => {
                let mutation = row["mutation"].as_str().unwrap();
                let name = match mutation {
                    "source-unit-length" => "Length",
                    "source-unit-derived-non-time" => "Area",
                    _ => "Duration",
                };
                let time_id = adverse["semantic_graph"]["nodes"][42]["node_id"].clone();
                let mut dimension = adverse["semantic_graph"]["nodes"][42].clone();
                dimension["nominal_identity_preimage"]["qualified_declaration"] = json!(["Example", name]);
                dimension["declaration"]["qualified_name"] = json!(["Example", name]);
                if name == "Area" || name == "Duration" {
                    let base_id = if name == "Area" {
                        let mut length = dimension.clone();
                        length["nominal_identity_preimage"]["qualified_declaration"] = json!(["Example", "Length"]);
                        length["declaration"]["qualified_name"] = json!(["Example", "Length"]);
                        let id = json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": unqualified_jcs_sha256(&length["nominal_identity_preimage"])});
                        length["node_id"] = id.clone();
                        length["semantic_type"] = id.clone();
                        adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(length);
                        id
                    } else { time_id };
                    dimension["nominal_identity_preimage"]["terms"] = json!([{
                        "dimension_node_id": base_id.clone(),
                        "exponent": if name == "Area" { "2" } else { "1" }
                    }]);
                    dimension["dependencies"] = json!([base_id]);
                }
                let dimension_id = json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": unqualified_jcs_sha256(&dimension["nominal_identity_preimage"])});
                dimension["node_id"] = dimension_id.clone();
                dimension["semantic_type"] = dimension_id.clone();
                adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(dimension);
                let mut unit = adverse["semantic_graph"]["nodes"][43].clone();
                unit["nominal_identity_preimage"]["qualified_declaration"] = json!(["Example", "u"]);
                unit["nominal_identity_preimage"]["dimension_node_id"] = dimension_id.clone();
                let unit_id = json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": unqualified_jcs_sha256(&unit["nominal_identity_preimage"])});
                unit["node_id"] = unit_id.clone();
                unit["semantic_type"] = dimension_id.clone();
                unit["dependencies"] = json!([dimension_id]);
                unit["declaration"]["qualified_name"] = json!(["Example", "u"]);
                adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(unit);
                let source = &mut adverse["semantic_graph"]["nodes"][44];
                let old = source["body"]["members"][1]["value"]["target"].clone();
                source["body"]["members"][1]["value"]["target"] = unit_id.clone();
                let deps = source["dependencies"].as_array_mut().unwrap();
                deps.retain(|dep| dep != &old);
                deps.push(unit_id);
                deps.sort_by(|left, right| left["digest"].as_str().cmp(&right["digest"].as_str()));
            }
            other => panic!("unhandled semantic timed-model vector {other}"),
        }
        rekey_stale_nodes(&mut adverse);
        assert!(validator.is_valid(&adverse), "{} is a schema-valid semantic control", row["id"]);
        assert_eq!(
            adverse["package_id"]["digest"],
            unqualified_jcs_sha256(&package_id_preimage(&adverse)),
            "{} retains the recomputed package id",
            row["id"]
        );
        let expected = match row["mutation"].as_str().unwrap() {
            "remove-timed-root" => ("invalid_package", "missing-timed-model", "/semantic_graph/nodes"),
            "dangling-tick-operation" | "function-as-tick-operation" => (
                "invalid_package", "timed-node-reference", "/semantic_graph/nodes/44/body/members/3/value/target",
            ),
            "duplicate-clock-place" => (
                "invalid_package", "duplicate-clock-place", "/semantic_graph/nodes/52/body/members/3/value/members/1",
            ),
            "noncanonical-place-path" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/45/body/members/1/value/members/0",
            ),
            "zero-tick-period" => (
                "ill_typed", "invalid-time-quantity", "/semantic_graph/nodes/44/body/members/4/value/value",
            ),
            "missing-clock-resolution-row" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/45/body/members/2/value/members",
            ),
            "mismatched-clock-path-step" | "wrong-clock-collection-metadata" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/45/body/members/2/value/members/0",
            ),
            "undeclared-clock-member" | "wrong-terminal-clock-type" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/45/body/members/2/value/members/0",
            ),
            "coherent-forged-selected-chain" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/45/body/members/2/value/members/0",
            ),
            "tick-model-context-mismatch" => (
                "invalid_package", "timed-node-reference", "/semantic_graph/nodes/44/body/members/2/value/target",
            ),
            "unresolved-tick-operation" => (
                "missing_declaration", "missing-name", "/semantic_graph/nodes/46/body/members/1",
            ),
            "tick-source-with-clock-place" => (
                "invalid_package", "clock-without-dense-time", "/semantic_graph/nodes/53/body/members/0/value/target",
            ),
            "source-unit-length" | "source-unit-derived-non-time" | "source-unit-derived-time" => (
                "ill_typed", "invalid-time-quantity", "/semantic_graph/nodes/44/body/members/1/value/target",
            ),
            other => panic!("unhandled expected diagnostic for {other}"),
        };
        assert_eq!(row["expected_code"], expected.0);
        assert_eq!(row["expected_cause"], expected.1);
        assert_eq!(row["locus"], expected.2);
    }
}

#[trace("TC-233", "FR-322-AC-121")]
#[test]
fn tc_233_body_steps_pin_timed_model_between_hybrid_and_protocol() {
    let body_steps = FR322_BODY_STEPS
        .split("**Body steps.**")
        .nth(1)
        .expect("FR-322 Body steps section")
        .split("- derived-shape key stage")
        .next()
        .expect("FR-322 body-step section end");
    let positions = ["hybrid", "timed-model", "protocol"].map(|step| {
        body_steps.find(step).unwrap_or_else(|| panic!("FR-322 body steps contain {step}"))
    });
    assert!(positions[0] < positions[1] && positions[1] < positions[2]);
    assert!(TC233_BODY_STEP_VECTOR.contains("BS-01"));
    assert!(TC233_BODY_STEP_VECTOR.contains("timed-model step's refusal before protocol"));
    assert!(TC233_BODY_STEP_VECTOR.contains("hybrid < timed-model < protocol"));
}

#[trace("TC-367", "FR-322-AC-120")]
#[test]
fn tc_367_timed_model_causes_are_catalogued_and_paired() {
    let schema = parsed_fixture(SCHEMA);
    let diagnostic = schema_for_def(&schema, "Diagnostic");
    for (code, cause) in [
        ("invalid_package", "missing-timed-model"),
        ("invalid_package", "timed-node-reference"),
        ("invalid_package", "duplicate-clock-place"),
        ("invalid_package", "malformed-clock-place"),
        ("invalid_package", "clock-without-dense-time"),
        ("ill_typed", "invalid-time-quantity"),
    ] {
        let entry = json!({"stage":"package_read", "code":code, "cause_tag":cause, "details":[], "loci":[]});
        assert!(diagnostic.is_valid(&entry), "{cause} pairs with {code}");
    }
    let invalid_package = NATIVE_DIAGNOSTICS
        .lines()
        .find(|line| line.split('|').nth(1).is_some_and(|cell| backticked(cell).contains("invalid_package")))
        .expect("native invalid_package row");
    let ill_typed = NATIVE_DIAGNOSTICS
        .lines()
        .find(|line| line.split('|').nth(1).is_some_and(|cell| backticked(cell).contains("ill_typed")))
        .expect("native ill_typed row");
    for cause in ["missing-timed-model", "timed-node-reference", "duplicate-clock-place", "malformed-clock-place", "clock-without-dense-time"] {
        assert!(backticked(invalid_package).contains(cause), "native invalid_package catalogs {cause}");
    }
    assert!(backticked(ill_typed).contains("invalid-time-quantity"));
}

/// FR-341-AC-15, FR-341-AC-17, FR-464-AC-19: the published transition package
/// carries one Boolean model-field `state`/`transition` node whose body is the
/// five-binding `quire.op.state.transition` application. Both keys recompute,
/// the closed schema refuses each body mutation, the operation step admits it
/// and refuses another profile operator, and the identity binds the anchor,
/// subject, pre and post nodes while the source map stays outside it.
#[trace("TC-233", "FR-341-AC-15")]
#[trace("TC-233", "FR-341-AC-17")]
#[trace("TC-233", "FR-464-AC-19")]
#[trace("TC-233", "FR-341-AC-19")]
#[trace("TC-233", "FR-341-AC-18")]
#[test]
fn tc_233_state_transition_package_binds_anchor_subject_pre_and_post() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_STATE_TRANSITION);
    schema
        .validate(&package)
        .unwrap_or_else(|error| panic!("the transition package has the closed V2 shape: {error}"));
    let nodes = package["semantic_graph"]["nodes"].as_array().expect("nodes");
    for node in nodes {
        let current = if is_application_node(node) {
            application_node_key_is_current(node, nodes)
        } else {
            structural_node_key_is_current(node)
        };
        assert!(current, "{} keeps the digest of its preimage", node["semantic_form"]);
    }
    assert_eq!(
        package["package_id"]["digest"].as_str(),
        Some(unqualified_jcs_sha256(&derive_package_id_preimage(&package)).as_str())
    );
    let preimage_schema =
        schema_for_def(&parsed_fixture(NODE_PREIMAGE_SCHEMA), "ApplicationNode");
    for node in nodes.iter().filter(|node| is_application_node(node)) {
        let preimage = application_node_preimage(node, nodes).expect("application preimage");
        assert!(
            preimage_schema.is_valid(&preimage),
            "{} preimage is valid under the preimage schema",
            node["semantic_form"]
        );
    }
    // The operation step admits every application of the package, the
    // subject projection over a dereferenced model object included.
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    let operation_refusal = |candidate: &Value, form: &str| {
        let oracle = OperationOracle::for_package_with(&catalog, candidate, &[]);
        oracle
            .nodes
            .iter()
            .filter(|node| node["semantic_form"] == form)
            .flat_map(|node| {
                application_terms(&node["body"])
                    .into_iter()
                    .map(|term| oracle.classify_in(node, term).map(drop))
                    .collect::<Vec<_>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(drop)
    };

    let transition = node_with_operation(&package, "quire.op.state.transition");
    let body = |index: usize| format!("/semantic_graph/nodes/{index}/body");
    let refused = |pointer: String, value: Value, why: &str| {
        let mut candidate = package.clone();
        *candidate.pointer_mut(&pointer).unwrap_or_else(|| panic!("{pointer} exists")) = value;
        assert!(!schema.is_valid(&candidate), "{why}");
    };
    let arguments = package["semantic_graph"]["nodes"][transition]["body"]["arguments"].clone();
    let names = arguments
        .as_array()
        .expect("arguments")
        .iter()
        .map(|argument| argument["name"].as_str().expect("binding name"))
        .collect::<Vec<_>>();
    assert_eq!(names, ["anchor", "subject", "inputs", "pre", "post"]);
    let with_arguments = |edit: &dyn Fn(&mut Vec<Value>)| {
        let mut edited = arguments.as_array().expect("arguments").clone();
        edit(&mut edited);
        Value::Array(edited)
    };
    refused(
        format!("{}/arguments", body(transition)),
        with_arguments(&|edited| edited.swap(0, 1)),
        "the bindings are anchor, subject, inputs, pre, post in that order",
    );
    refused(
        format!("{}/arguments", body(transition)),
        with_arguments(&|edited| {
            edited.pop();
        }),
        "post is required",
    );
    refused(
        format!("{}/arguments", body(transition)),
        with_arguments(&|edited| edited.push(edited[4].clone())),
        "the body carries exactly five bindings",
    );
    refused(
        format!("{}/arguments/3/value/members", body(transition)),
        json!([arguments[4]["value"], arguments[4]["value"]]),
        "pre holds at most one clause",
    );
    refused(
        format!("{}/arguments/1/value", body(transition)),
        json!({"term": "aggregate", "members": []}),
        "the subject is one reference",
    );
    refused(
        format!("{}/operation/member/operator", body(transition)),
        json!("sequence"),
        "the schema fixes the transition operator",
    );
    refused(
        format!("{}/operation/laws/0/role", body(transition)),
        json!("temporal_profile"),
        "the law is the protocol profile",
    );
    refused(
        format!("{}/operation/mode", body(transition)),
        json!({"kind": "rounding", "value": "nearest-even"}),
        "a transition carries no mode",
    );
    refused(
        format!("/semantic_graph/nodes/{transition}/occurrences/0/role"),
        json!("declaration"),
        "a transition occurrence is a claim",
    );

    let mut other_operator = package.clone();
    other_operator["semantic_graph"]["nodes"][transition]["body"]["operation"]["member"]
        ["operator"] = json!("sequence");
    // XT-06 / XT-19: the published schema refuses the document first; the
    // operation step alone still refuses it as a member mismatch.
    assert!(
        !schema.is_valid(&other_operator),
        "the schema pins the transition body's operator, so a complete reader refuses malformed_wire"
    );
    assert_eq!(
        operation_refusal(&other_operator, "transition"),
        Err(OperationRefusal::MemberMismatch),
        "XT-19: the finite-global profile defines no protocol operator for a state transition"
    );

    let clause_package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let protocol = node_with_operation(&clause_package, "quire.op.protocol.control");
    let mut protocol_transition = clause_package.clone();
    protocol_transition["semantic_graph"]["nodes"][protocol]["body"]["operation"]["member"]
        ["operator"] = json!("transition");
    // XT-07 / XT-20: schema-invalid first (malformed_wire), operation step alone refuses.
    assert!(
        schema.is_valid(&clause_package),
        "control: the unedited clause package is schema-valid, so the failure below is the operator alone"
    );
    assert!(
        !schema.is_valid(&protocol_transition),
        "ProtocolControlApplication enumerates no `transition` operator, so the schema refuses first"
    );
    assert_eq!(
        package_operations_refusal(&protocol_transition, &catalog),
        Err(OperationRefusal::MemberMismatch),
        "XT-20: a protocol control never applies the state operator"
    );

    // XT-18: a lock without the protocol_profile selection leaves the law unselected.
    let mut unselected = package.clone();
    for prefix in ["/lock", "/identity_preimage"] {
        unselected
            .pointer_mut(&format!("{prefix}/profile_selections"))
            .and_then(Value::as_array_mut)
            .expect("profile selections")
            .retain(|row| row["role"] != "protocol_profile");
    }
    refresh_package_id(&mut unselected);
    assert_eq!(
        operation_refusal(&unselected, "transition"),
        Err(OperationRefusal::LawUnselected),
        "a transition needs the protocol profile selected"
    );

    // XT-16: a transition application nested in another term, and another
    // application as a transition node's body, each fail the schema.
    let enabled = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["body"]["operation"]["identity"] == "quire.op.boolean.or")
        .expect("the precondition term");
    refused(
        format!("{}/arguments/0", body(enabled)),
        package["semantic_graph"]["nodes"][transition]["body"].clone(),
        "a transition application is never nested",
    );
    refused(
        body(transition),
        package["semantic_graph"]["nodes"][enabled]["body"].clone(),
        "a transition node's body root is the transition application",
    );

    let ids = |candidate: &Value| {
        candidate["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .map(|node| node["node_id"]["digest"].as_str().expect("id").to_owned())
            .collect::<BTreeSet<_>>()
    };
    let mut regions = package.clone();
    regions["source_map"][0]["regions"][0]["start"] = json!(40);
    assert_eq!(ids(&regions), ids(&package), "a source region moves no node id");
    assert_eq!(
        unqualified_jcs_sha256(&derive_package_id_preimage(&regions)),
        package["package_id"]["digest"].as_str().expect("id"),
        "a source region is outside the package identity"
    );

    let anchor_digest = package["semantic_graph"]["nodes"][transition]["body"]["arguments"][0]
        ["value"]["target"]["digest"]
        .as_str()
        .expect("anchor digest")
        .to_owned();
    let mut renamed = package.clone();
    let anchor_index = node_index(&renamed, &anchor_digest);
    renamed["semantic_graph"]["nodes"][anchor_index]["body"]["members"][1]["value"]["value"] =
        json!("restore");
    let renames = rekey_stale_nodes(&mut renamed);
    let renamed_forms = renames
        .keys()
        .map(|old| {
            let node = &package["semantic_graph"]["nodes"][node_index(&package, old)];
            format!("{}/{}", node["node_tag"], node["semantic_form"])
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        renamed_forms,
        BTreeSet::from([
            "\"state\"/\"operation_anchor\"".to_owned(),
            "\"state\"/\"state_clause\"".to_owned(),
            "\"state\"/\"transition\"".to_owned(),
        ]),
        "the anchor's operation enters the anchor, both clauses and the transition"
    );
    assert_ne!(renamed["package_id"], package["package_id"]);

    // Rewriting `post` to name the precondition changes the transition and the package id.
    let mut rewired = package.clone();
    let pre_target = rewired["semantic_graph"]["nodes"][transition]["body"]["arguments"][3]
        ["value"]["members"][0]
        .clone();
    rewired["semantic_graph"]["nodes"][transition]["body"]["arguments"][4]["value"] = pre_target;
    let renames = rekey_stale_nodes(&mut rewired);
    assert_eq!(renames.len(), 1, "only the transition is rekeyed");
    assert_ne!(rewired["package_id"], package["package_id"]);
}

/// Every published positive package, read from the fixtures directory so that
/// a newly published package cannot escape the selected-root checks.
fn published_positive_packages() -> Vec<(String, Value)> {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("proposals/checked-package-v2/fixtures");
    let mut paths = std::fs::read_dir(&directory)
        .expect("fixtures directory is readable")
        .map(|entry| entry.expect("fixture entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("positive-") && name.ends_with(".json"))
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path).expect("fixture is readable");
            (
                path.file_name().unwrap().to_string_lossy().into_owned(),
                parsed_fixture(&text),
            )
        })
        .collect()
}

/// The `model_root` node a producer derives from one `model_selections` row,
/// computed from the row alone.
fn derived_model_root(selection: &Value) -> Value {
    let owner = json!({
        "kind": "model_root",
        "identity": selection["identity"],
        "digest": selection["digest"],
    });
    let body = json!({"term": "aggregate", "members": []});
    let key = unqualified_jcs_sha256(&json!({
        "version": "quire.structural-node/v1",
        "node_tag": "model",
        "semantic_form": "model_root",
        "semantic_type": null,
        "declaration": null,
        "recursion": null,
        "owner": owner,
        "body": body,
    }));
    let node_id = json!({"domain": "quire.checked-semantic-node/v1", "digest": key});
    json!({
        "node_id": node_id,
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "model",
        "semantic_form": "model_root",
        "semantic_type": node_id,
        "dependencies": [],
        "occurrences": [{"role": "generated", "ordinal": 0}],
        "owner": owner,
        "body": body,
    })
}

fn model_root_positions(package: &Value) -> Vec<usize> {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes are an array")
        .iter()
        .enumerate()
        .filter(|(_, node)| node["node_tag"] == "model" && node["semantic_form"] == "model_root")
        .map(|(position, _)| position)
        .collect()
}

/// The reader-stage model-root checks of FR-322-AC-135, in reader order: a
/// repeated `node_id` (graph shape), the owner join, then completeness.
/// Returns `(code, cause, pointer)`.
fn model_root_reader_refusal(package: &Value) -> Option<(&'static str, &'static str, String)> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes are an array");
    let mut seen = BTreeSet::new();
    for (position, node) in nodes.iter().enumerate() {
        if !seen.insert(node["node_id"].to_string()) {
            return Some((
                "invalid_semantic_graph",
                "",
                format!("/semantic_graph/nodes/{position}/node_id"),
            ));
        }
    }
    let selections = package["lock"]["model_selections"]
        .as_array()
        .expect("model selections are an array");
    let roots = model_root_positions(package);
    let joins = |selection: &Value, owner: &Value| {
        selection["identity"] == owner["identity"] && selection["digest"] == owner["digest"]
    };
    for position in &roots {
        let owner = &nodes[*position]["owner"];
        if !selections.iter().any(|selection| joins(selection, owner)) {
            return Some((
                "missing_declaration",
                "missing-selection",
                format!("/semantic_graph/nodes/{position}/owner"),
            ));
        }
    }
    selections
        .iter()
        .any(|selection| {
            !roots
                .iter()
                .any(|position| joins(selection, &nodes[*position]["owner"]))
        })
        .then(|| {
            (
                "invalid_semantic_graph",
                "",
                "/semantic_graph/nodes".to_owned(),
            )
        })
}

/// Rewrites the root at `position`, rekeys it and the package id, and keeps
/// the source map in step, as a producer rebuilding identities would.
fn rekey_model_root(package: &mut Value, position: usize) {
    let old = package["semantic_graph"]["nodes"][position]["node_id"].clone();
    let preimage = structural_node_preimage(&package["semantic_graph"]["nodes"][position])
        .expect("root has a structural preimage");
    let new = json!({
        "domain": "quire.checked-semantic-node/v1",
        "digest": unqualified_jcs_sha256(&preimage),
    });
    let node = &mut package["semantic_graph"]["nodes"][position];
    node["node_id"] = new.clone();
    node["semantic_type"] = new.clone();
    for entry in package["source_map"].as_array_mut().expect("source map") {
        if entry["node_id"] == old {
            entry["node_id"] = new.clone();
        }
    }
    refresh_package_id(package);
}

const MODEL_ROOT_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-root-vectors.json");

/// Applies the RFC 6902 `add`, `remove` and `replace` operations the model-root
/// vectors use.
fn apply_json_patch(package: &mut Value, patch: &Value) {
    let path = patch["path"].as_str().expect("patch path");
    let (parent, member) = path.rsplit_once('/').expect("patch path has a member");
    let target = package.pointer_mut(parent).expect("patch parent exists");
    match (patch["op"].as_str().expect("patch op"), target) {
        ("add", Value::Array(items)) if member == "-" => items.push(patch["value"].clone()),
        ("add", Value::Object(map)) => {
            map.insert(member.to_owned(), patch["value"].clone());
        }
        ("replace", Value::Array(items)) => {
            items[member.parse::<usize>().expect("index")] = patch["value"].clone();
        }
        ("replace", Value::Object(map)) => {
            *map.get_mut(member).expect("replaced member exists") = patch["value"].clone();
        }
        ("remove", Value::Array(items)) => {
            items.remove(member.parse::<usize>().expect("index"));
        }
        ("remove", Value::Object(map)) => {
            map.remove(member).expect("removed member exists");
        }
        (operation, _) => panic!("unsupported model-root patch {operation} at {path}"),
    }
}

#[trace("TC-310", "FR-322-AC-132", "FR-322-AC-134")]
#[test]
fn tc_310_every_selected_model_package_carries_exactly_one_recomputed_root() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let mut selected = 0;
    let mut root_keys = BTreeMap::new();
    for (name, package) in published_positive_packages() {
        let selections = package["lock"]["model_selections"].as_array().unwrap();
        let roots = model_root_positions(&package);
        assert_eq!(
            roots.len(),
            selections.len(),
            "{name}: one root per model selection, none without one"
        );
        assert!(schema.is_valid(&package), "{name} is schema-valid");
        assert_eq!(
            model_root_reader_refusal(&package),
            None,
            "{name}: the reader admits its roots"
        );
        assert_eq!(
            unqualified_jcs_sha256(&package_id_preimage(&package)),
            package["package_id"]["digest"].as_str().unwrap(),
            "{name}: package_id recomputes with the root"
        );
        let node_count = package["semantic_graph"]["nodes"].as_array().unwrap().len();
        for selection in selections {
            selected += 1;
            let expected = derived_model_root(selection);
            let position = roots
                .iter()
                .copied()
                .find(|position| {
                    package["semantic_graph"]["nodes"][*position]["owner"]["identity"]
                        == selection["identity"]
                })
                .unwrap_or_else(|| panic!("{name}: no root for {}", selection["identity"]));
            let found = &package["semantic_graph"]["nodes"][position];
            assert_eq!(found, &expected, "{name}: root equals the derived root");
            assert_eq!(
                position,
                node_count - 1,
                "{name}: the root is last in graph order"
            );
            assert_eq!(
                package["source_map"].as_array().unwrap().last().unwrap()["node_id"],
                found["node_id"],
                "{name}: the root's entry is last in the source map"
            );
            assert!(
                structural_node_key_is_current(found),
                "{name}: root key is current"
            );
            assert_eq!(
                root_keys
                    .entry((
                        selection["identity"].to_string(),
                        selection["digest"].to_string()
                    ))
                    .or_insert_with(|| found["node_id"].clone()),
                &found["node_id"],
                "{name}: the root key depends on identity and digest alone"
            );
        }
    }
    assert!(selected >= 7, "every selected-model positive is covered");
}

#[trace("TC-310", "FR-322-AC-135", "FR-322-AC-138")]
#[test]
fn tc_310_root_owner_schema_and_reader_stages_run_from_the_published_vectors() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let preimage_schema = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let preimage_validator = schema_for_def(&preimage_schema, "ModelDeclarationNode");
    let vectors = parsed_fixture(MODEL_ROOT_VECTORS);
    assert_eq!(vectors["base"], "fixtures/positive-all-families.json");
    let base = parsed_fixture(POSITIVE_ALL_FAMILIES);
    let mut ids = BTreeSet::new();
    for vector in vectors["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        assert!(ids.insert(id), "vector ids are unique");
        let mut package = base.clone();
        for patch in vector["patch"].as_array().expect("patch") {
            apply_json_patch(&mut package, patch);
        }
        assert_eq!(
            unqualified_jcs_sha256(&package_id_preimage(&package)),
            package["package_id"]["digest"].as_str().unwrap(),
            "{id}: the vector carries the recomputed package_id"
        );
        let schema_valid = schema.is_valid(&package);
        assert_eq!(schema_valid, vector["schema"] == "valid", "{id}: schema");
        let observed = if schema_valid {
            model_root_reader_refusal(&package)
        } else {
            let locus = vector["locus"].as_str().expect("locus");
            let position = locus
                .strip_prefix("/semantic_graph/nodes/")
                .and_then(|rest| rest.split('/').next())
                .and_then(|index| index.parse::<usize>().ok())
                .expect("node locus");
            let node = &package["semantic_graph"]["nodes"][position];
            let preimage = structural_node_preimage(node).expect("structural node");
            assert!(
                !preimage_validator.is_valid(&preimage),
                "{id}: the closed node-preimage schema refuses it too"
            );
            Some(("malformed_wire", "", locus.to_owned()))
        };
        let expected = match vector["outcome"].as_str().expect("outcome") {
            "admitted" => None,
            refused => {
                let code = refused.strip_prefix("refused:").expect("refused outcome");
                Some((
                    code.to_owned(),
                    vector["cause"].as_str().unwrap_or("").to_owned(),
                    vector["locus"].as_str().expect("locus").to_owned(),
                ))
            }
        };
        assert_eq!(
            observed.map(|(code, cause, locus)| (code.to_owned(), cause.to_owned(), locus)),
            expected,
            "{id}"
        );
    }
    assert_eq!(
        ids,
        BTreeSet::from([
            "MR-00", "MR-01", "MR-02", "MR-03", "MR-04", "MR-06", "MR-09"
        ])
    );
}

#[trace("TC-310", "FR-322-AC-136")]
#[test]
fn tc_310_root_generated_occurrence_is_the_producer_asserted_source_relation() {
    for (name, package) in published_positive_packages() {
        let sources = package["lock"]["sources"].as_array().unwrap();
        for position in model_root_positions(&package) {
            let id = &package["semantic_graph"]["nodes"][position]["node_id"];
            let entries = package["source_map"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|entry| &entry["node_id"] == id)
                .collect::<Vec<_>>();
            assert_eq!(entries.len(), 1, "{name}: one source-map entry");
            assert_eq!(entries[0]["role"], "generated");
            assert_eq!(entries[0]["ordinal"], 0);
            let regions = entries[0]["regions"].as_array().unwrap();
            assert!(!regions.is_empty(), "{name}: at least one region");
            for region in regions {
                assert!(
                    sources.iter().any(|source| source == &region["source"]),
                    "{name}: the region names a lock source"
                );
                assert!(region["start"].as_u64() < region["end"].as_u64());
            }
        }
    }
}

#[trace("TC-310", "FR-322-AC-137")]
#[test]
fn tc_310_published_roots_are_the_producer_oracle() {
    let vectors = parsed_fixture(NODE_IDENTITY_VECTORS);
    let vector = vectors["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|vector| vector["name"] == "model-root-counter-cas")
        .expect("the model-root vector is published");
    let selection = json!({
        "identity": vector["preimage"]["owner"]["identity"],
        "digest": vector["preimage"]["owner"]["digest"],
    });
    assert_eq!(
        derived_model_root(&selection)["node_id"]["digest"],
        vector["sha256"],
        "the producer derivation reproduces the published root vector"
    );
    for (name, package) in published_positive_packages() {
        for selection in package["lock"]["model_selections"].as_array().unwrap() {
            let derived = derived_model_root(selection);
            assert!(
                package["semantic_graph"]["nodes"]
                    .as_array()
                    .unwrap()
                    .contains(&derived),
                "{name}: a producer deriving the root from the selection emits this node"
            );
        }
    }
}

const MODEL_TUPLE_UNION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-tuple-union-vectors.json");
/// The selected document of `acme/shapes`: a full FCD semantic IR 2.0.0
/// domain package. `domain_package` in the vectors is its projection.
const SHAPES_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-shapes.json");

/// FR-322-AC-151: the projection FR-154 intake reads from the `acme/shapes`
/// document, in document order: for each object type its supertypes and fields
/// (multiplicities as decimal strings), for each tuple its positions by ordinal
/// and for each variant type its cases in declared order.
fn shapes_project_document(document: &Value) -> Value {
    let meaning = |declared: &Value| -> String {
        document["constructs"]
            .as_array()
            .expect("constructs")
            .iter()
            .find(|construct| construct["kind"] == declared["kind"])
            .and_then(|construct| construct["construct"]["meaning"].as_str())
            .expect("a construct for the kind")
            .to_owned()
    };
    let (mut objects, mut tuples, mut variants) = (Vec::new(), Vec::new(), Vec::new());
    for declared in document["types"].as_array().expect("types") {
        match meaning(declared).as_str() {
            "quire.meaning.model.object-type/v1" => objects.push(json!({
                "node": declared["identity"],
                "interfaceFeatures": null,
                "supertypes": declared["supertypes"],
                "fields": declared["fields"].as_array().expect("fields").iter().map(|field| json!({
                    "name": field["name"],
                    "typeRef": field["typeRef"],
                    "presence": field["presence"],
                    "multiplicity": {
                        "lower": field["multiplicity"]["lower"].to_string(),
                        "upper": field["multiplicity"]["upper"].to_string(),
                        "ordered": field["multiplicity"]["ordered"],
                        "unique": field["multiplicity"]["unique"],
                    },
                    "redefines": null,
                })).collect::<Vec<_>>(),
                "operations": [],
            })),
            "quire.meaning.model.tuple-value-type/v1" => tuples.push(json!({
                "node": declared["identity"],
                "positions": declared["positions"].as_array().expect("positions").iter().enumerate()
                    .map(|(ordinal, position)| json!({"ordinal": ordinal, "typeRef": position["typeRef"]}))
                    .collect::<Vec<_>>(),
            })),
            "quire.meaning.model.variant-type/v1" => variants.push(json!({
                "node": declared["identity"],
                "cases": declared["variants"].as_array().expect("variants").iter().map(|case| json!({
                    "name": case["name"],
                    "payloadType": case.get("payloadType").cloned().unwrap_or(Value::Null),
                })).collect::<Vec<_>>(),
            })),
            other => panic!("unexpected meaning {other}"),
        }
    }
    json!({
        "identity": document["package"]["identity"],
        "version": document["package"]["version"],
        "object_types": objects,
        "tuple_types": tuples,
        "variant_types": variants,
    })
}

/// The type a `typeRef` of the `acme/shapes` projection derives (FR-322
/// "Model-owned members" step 4): a native name is its scalar, a package type
/// is its self-typed declaration node.
fn shapes_type_of(reference: &str) -> Value {
    match reference.strip_prefix("ix://quire/native/") {
        Some(native) => json!({"type": native}),
        None => json!({"declaration": reference.rsplit('/').next().expect("segment")}),
    }
}

fn shapes_declared<'a>(document: &'a Value, list: &str, name: &str) -> Option<&'a Value> {
    document[list]
        .as_array()?
        .iter()
        .find(|declared| declared["node"].as_str().is_some_and(|node| node.ends_with(&format!("/{name}"))))
}

/// FR-322 "Tuple value types and payload-bearing variant types": the family
/// of a model declaration node, read from the selected document.
fn shapes_family(document: &Value, name: &str) -> &'static str {
    if shapes_declared(document, "tuple_types", name).is_some() {
        return "tuple";
    }
    let variant = shapes_declared(document, "variant_types", name).expect("declared variant type");
    if variant["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .any(|case| !case["payloadType"].is_null())
    {
        "union"
    } else {
        "enum"
    }
}

/// The members of a model union: the cases in ascending UTF-8 byte order of
/// case name, each with its payload type (`None` for a nullary case).
fn shapes_members(document: &Value, name: &str) -> Vec<(String, Option<Value>)> {
    let variant = shapes_declared(document, "variant_types", name).expect("declared variant type");
    let mut members = variant["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| {
            (
                case["name"].as_str().expect("case name").to_owned(),
                case["payloadType"].as_str().map(shapes_type_of),
            )
        })
        .collect::<Vec<_>>();
    members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    members
}

fn shapes_refusal(code: &str, cause: &str) -> Value {
    json!({"refused": {"code": code, "cause": cause}})
}

/// Step 3 and 4 for a `position` member of a tuple value type.
fn shapes_position(document: &Value, tuple: &str, position: &Value) -> Value {
    let declared = shapes_declared(document, "tuple_types", tuple).expect("declared tuple");
    let positions = declared["positions"].as_array().expect("positions");
    let matched = key_ordinal(position).and_then(|index| {
        let slot = usize::try_from(&index).ok()?;
        positions.get(slot).map(|found| (index, found))
    });
    match matched {
        Some((index, found)) => json!({
            "resolves": format!("{}/p{index}", declared["node"].as_str().expect("node")),
            "member_type": shapes_type_of(found["typeRef"].as_str().expect("typeRef")),
        }),
        None => shapes_refusal("ill_typed", "operator-ineligible"),
    }
}

/// Step 4 for a field of an object type of the projection.
fn shapes_field_type(document: &Value, object: &str, field: &str) -> Value {
    let declared = shapes_declared(document, "object_types", object).expect("declared object");
    let field = declared["fields"]
        .as_array()
        .expect("fields")
        .iter()
        .find(|candidate| candidate["name"] == field)
        .expect("declared field");
    let element = shapes_type_of(field["typeRef"].as_str().expect("typeRef"));
    let multiplicity = &field["multiplicity"];
    if multiplicity["lower"] == "1" && multiplicity["upper"] == "1" {
        return if field["presence"] == "optional" {
            json!({"option": element})
        } else {
            element
        };
    }
    let kind = match (multiplicity["ordered"].as_bool(), multiplicity["unique"].as_bool()) {
        (Some(true), Some(true)) => "ordered_set",
        (Some(true), _) => "sequence",
        (_, Some(true)) => "set",
        _ => "bag",
    };
    json!({kind: element, "min": multiplicity["lower"], "max": multiplicity["upper"]})
}

/// FR-208 and FR-154 intake of the projection's tuple and variant types: the
/// first refusal in report order, with its code, cause and locus.
fn shapes_intake_refusal(document: &Value) -> Option<(&'static str, &'static str, String)> {
    let natives = [
        "Boolean", "Integer", "Text", "String", "UUID", "Timestamp", "Bytes", "JsonObject",
        "Float32", "Float64", "Duration",
    ];
    let resolves = |reference: &str| match reference.strip_prefix("ix://quire/native/") {
        Some(native) => natives.contains(&native),
        None => reference.strip_prefix("ix://acme/shapes/").is_some_and(|name| {
            shapes_declared(document, "tuple_types", name).is_some()
                || shapes_declared(document, "variant_types", name).is_some()
        }),
    };
    for (index, tuple) in document["tuple_types"].as_array().into_iter().flatten().enumerate() {
        let at = format!("/tuple_types/{index}");
        let positions = tuple["positions"].as_array().expect("positions");
        let mut seen = BTreeSet::new();
        for (slot, position) in positions.iter().enumerate() {
            if !seen.insert(position["ordinal"].to_string()) {
                return Some(("invalid_model_binding", "conflicting-binding", format!("{at}/positions/{slot}")));
            }
        }
        let extra = tuple
            .as_object()
            .expect("tuple")
            .keys()
            .any(|key| key != "node" && key != "positions");
        if positions.is_empty() || extra {
            return Some(("invalid_model_binding", "malformed-declaration", at));
        }
        for (slot, position) in positions.iter().enumerate() {
            let supported = position["typeRef"]
                .as_str()
                .is_some_and(|reference| reference != "ix://quire/native/Clock");
            if position["ordinal"] != slot || !supported {
                return Some(("invalid_model_binding", "malformed-declaration", format!("{at}/positions/{slot}")));
            }
        }
        for (slot, position) in positions.iter().enumerate() {
            if !resolves(position["typeRef"].as_str().expect("typeRef")) {
                return Some(("missing_declaration", "missing-name", format!("{at}/positions/{slot}")));
            }
        }
    }
    for (index, variant) in document["variant_types"].as_array().into_iter().flatten().enumerate() {
        for (slot, case) in variant["cases"].as_array().expect("cases").iter().enumerate() {
            if let Some(payload) = case["payloadType"].as_str()
                && (payload == "ix://quire/native/Clock" || !resolves(payload))
            {
                return Some((
                    "unsupported_construct",
                    "declaration-form",
                    format!("/variant_types/{index}/cases/{slot}/payloadType"),
                ));
            }
        }
    }
    None
}

/// The key a producer derives for the model declaration node of `name` from
/// the selected `document`: the JCS SHA-256 of the closed `ModelDeclarationNode`
/// preimage built from the declaring list the document holds `name` in and
/// its owner. Nothing else of the document is read.
fn declaration_key(document: &Value, name: &str) -> String {
    let (form, declared) = if let Some(tuple) = shapes_declared(document, "tuple_types", name) {
        ("tuple_value_type", tuple)
    } else {
        (
            "variant_type",
            shapes_declared(document, "variant_types", name).expect("declared variant type"),
        )
    };
    unqualified_jcs_sha256(&json!({
        "version": "quire.structural-node/v1",
        "node_tag": "model",
        "semantic_form": form,
        "semantic_type": null,
        "declaration": null,
        "recursion": null,
        "owner": {"kind": "model", "identity": document["identity"], "node": declared["node"]},
        "body": {"term": "aggregate", "members": []},
    }))
}

/// Whether an identity case holds of `document`: the key of its node, the
/// selection digest and the `model_root` key, each derived from the patched
/// document, agree with every field the case states.
fn identity_case_holds(document: &Value, case: &Value) -> bool {
    let node = case["node"].as_str().expect("identity node");
    let mut changed = document.clone();
    for patch in case["patch"].as_array().expect("patch") {
        apply_json_patch(&mut changed, patch);
    }
    let expected = &case["expected"];
    let selection_of = |candidate: &Value| {
        json!({
            "identity": candidate["identity"],
            "digest": unqualified_jcs_sha256(candidate),
        })
    };
    let states = |field: &str, observed: bool| expected[field].as_bool() == Some(observed);
    states(
        "node_key_unchanged",
        declaration_key(&changed, node) == declaration_key(document, node),
    ) && states(
        "selection_digest_changes",
        unqualified_jcs_sha256(&changed) != unqualified_jcs_sha256(document),
    ) && states(
        "root_key_changes",
        derived_model_root(&selection_of(&changed))["node_id"]
            != derived_model_root(&selection_of(document))["node_id"],
    ) && expected["family"]
        .as_str()
        .is_none_or(|family| shapes_family(&changed, node) == family)
        && expected["members_unchanged"].as_bool().is_none_or(|unchanged| {
            (shapes_members(&changed, node) == shapes_members(document, node)) == unchanged
        })
}

/// Whether `name` reaches itself through the position `typeRef`s and the
/// `payloadType`s of the document's tuples and variant types.
fn shapes_reaches_itself(document: &Value, name: &str) -> bool {
    let successors = |current: &str| -> Vec<String> {
        let mut found = Vec::new();
        if let Some(tuple) = shapes_declared(document, "tuple_types", current) {
            found.extend(tuple["positions"].as_array().into_iter().flatten().filter_map(
                |position| position["typeRef"].as_str().map(str::to_owned),
            ));
        }
        if let Some(variant) = shapes_declared(document, "variant_types", current) {
            found.extend(variant["cases"].as_array().into_iter().flatten().filter_map(
                |case| case["payloadType"].as_str().map(str::to_owned),
            ));
        }
        found
            .into_iter()
            .filter_map(|reference| reference.strip_prefix("ix://acme/shapes/").map(str::to_owned))
            .collect()
    };
    let mut seen = BTreeSet::new();
    let mut pending = successors(name);
    while let Some(current) = pending.pop() {
        if current == name {
            return true;
        }
        if seen.insert(current.clone()) {
            pending.extend(successors(&current));
        }
    }
    false
}

fn expected_refusal(expected: &Value) -> Option<(String, Option<String>)> {
    let refused = expected.get("refused")?;
    Some((
        refused["code"].as_str().expect("code").to_owned(),
        refused["cause"].as_str().map(str::to_owned),
    ))
}

/// FR-322-AC-151: the selected document of `acme/shapes` is a full FCD
/// semantic IR; the vectors' `domain_package` is exactly its projection, and
/// the row digest the lock pins is the JCS SHA-256 of the whole document.
#[trace("TC-940", "FR-322-AC-151")]
#[test]
fn tc_940_the_selected_document_is_a_full_semantic_ir_whose_projection_is_pinned() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let document = parsed_fixture(SHAPES_DOCUMENT);
    assert_eq!(vectors["selected_document"], "domain-package-acme-shapes.json");
    assert_eq!(document["contractVersion"], "2.0.0");
    assert_eq!(document["package"]["identity"], "acme/shapes");
    for member in ["constructs", "types"] {
        assert!(document[member].as_array().is_some_and(|list| !list.is_empty()), "{member}");
    }
    assert!(document.get("tuple_types").is_none() && document.get("variant_types").is_none());
    let tuple_construct = document["constructs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["construct"]["meaning"] == "quire.meaning.model.tuple-value-type/v1")
        .expect("the tuple-value-type construct is declared");
    assert_eq!(tuple_construct["construct"]["shape"], "tuple");
    assert_eq!(
        tuple_construct["construct"]["members"]["positions"],
        "required"
    );
    for declared in document["types"].as_array().unwrap() {
        if declared["kind"]["name"] == "tuple" {
            for position in declared["positions"].as_array().unwrap() {
                assert!(
                    position["origin"].is_object(),
                    "tuple positions carry FCD origins"
                );
            }
        }
    }
    let projection = shapes_project_document(&document);
    assert_eq!(projection, vectors["domain_package"], "the projection derives from the document");
    let package = parsed_fixture(include_str!(
        "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
    ));
    let row = &package["lock"]["model_selections"][0];
    assert_eq!(row["identity"], document["package"]["identity"]);
    assert_eq!(row["digest"], unqualified_jcs_sha256(&document), "the row pins the whole document");
    assert_eq!(vectors["selection_digest"], row["digest"]);
    assert_ne!(row["digest"], unqualified_jcs_sha256(&projection), "the projection is not digested");
    // The model_root key and the package id derive from that digest.
    let root = package["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["semantic_form"] == "model_root")
        .expect("model_root");
    assert_eq!(root["owner"]["digest"], row["digest"]);
    // Swapping a position type in the document changes the document, so the row digest.
    let mut swapped = document.clone();
    let pair = swapped["types"]
        .as_array_mut()
        .expect("types")
        .iter_mut()
        .find(|declared| declared["identity"] == "ix://acme/shapes/Pair")
        .expect("Pair");
    pair["positions"].as_array_mut().expect("positions").swap(0, 1);
    assert_ne!(unqualified_jcs_sha256(&swapped), row["digest"].as_str().unwrap());
    // The projection lists follow document order, and reordering two declarations changes the document.
    let mut reordered = document.clone();
    reordered["types"].as_array_mut().expect("types").swap(1, 2);
    assert_ne!(unqualified_jcs_sha256(&reordered), row["digest"].as_str().unwrap());
    assert_ne!(shapes_project_document(&reordered), projection);
}

/// FR-322-AC-140 to FR-322-AC-146: the declarations, the fixture and the
/// published identity vectors.
#[trace("TC-940", "FR-322-AC-140", "FR-322-AC-146")]
#[test]
fn tc_940_tuple_and_payload_variant_declarations_are_model_declaration_nodes() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let preimage_schema = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let preimage_validator = schema_for_def(&preimage_schema, "ModelDeclarationNode");
    let package = parsed_fixture(include_str!(
        "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
    ));
    assert_eq!(vectors["base"], "fixtures/positive-model-tuple-union.json");
    assert!(schema.is_valid(&package), "the fixture is schema-valid");
    assert_eq!(
        unqualified_jcs_sha256(&package_id_preimage(&package)),
        package["package_id"]["digest"].as_str().unwrap(),
        "the fixture carries its recomputed package id"
    );
    assert_eq!(
        vectors["selection_digest"],
        unqualified_jcs_sha256(&parsed_fixture(SHAPES_DOCUMENT)),
        "the selection digest is the JCS digest of the whole selected document (FR-322-AC-151)"
    );
    assert_eq!(
        package["lock"]["model_selections"][0]["digest"], vectors["selection_digest"],
        "the fixture selects the published document"
    );
    assert_eq!(
        read_package_semantics(&package, &parsed_fixture(OPERATION_CATALOG)).err().map(|_| ()),
        None,
        "the full reader admits the fixture"
    );
    let mut forms = BTreeSet::new();
    for declared in vectors["declaration_nodes"].as_array().expect("declaration nodes") {
        let name = declared["name"].as_str().expect("name");
        let preimage = &declared["preimage"];
        assert!(preimage_validator.is_valid(preimage), "{name}: closed preimage");
        assert_eq!(unqualified_jcs_sha256(preimage), declared["sha256"].as_str().unwrap(), "{name}: key");
        let form = preimage["semantic_form"].as_str().unwrap();
        forms.insert(form);
        if name == "Holder" {
            continue;
        }
        let node = package["semantic_graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| node["node_id"]["digest"] == declared["sha256"])
            .unwrap_or_else(|| panic!("{name}: the fixture carries the node"));
        assert_eq!(node["owner"], preimage["owner"], "{name}: owner");
        assert_eq!(node["semantic_form"], form, "{name}: form");
        assert!(node.get("declaration").is_none(), "{name}: no declaration");
        assert_eq!(node["body"], json!({"term": "aggregate", "members": []}), "{name}: body");
        assert_eq!(node["semantic_type"], node["node_id"], "{name}: self-typed");
        assert!(node["dependencies"].as_array().unwrap().is_empty(), "{name}: dependencies");
        assert!(structural_node_key_is_current(node), "{name}: key is current");
    }
    assert_eq!(
        forms,
        BTreeSet::from(["object_type", "tuple_value_type", "variant_type"]),
        "the declarations cover both new forms and an object control"
    );
    let published = parsed_fixture(NODE_IDENTITY_VECTORS);
    for (name, form, node) in [
        ("model-tuple-value-type-pair", "tuple_value_type", "ix://acme/shapes/Pair"),
        ("model-variant-type-shape-union", "variant_type", "ix://acme/shapes/Shape"),
    ] {
        let vector = published["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|vector| vector["name"] == name)
            .unwrap_or_else(|| panic!("{name} is published"));
        assert_eq!(vector["preimage"]["semantic_form"], form);
        assert_eq!(vector["preimage"]["owner"]["node"], node);
        assert!(preimage_validator.is_valid(&vector["preimage"]), "{name}: closed preimage");
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            vector["sha256"].as_str().unwrap(),
            "{name}: recomputes"
        );
    }
    // An unknown model form fails the schema, so the two new forms are
    // admitted by enumeration, not by a pattern.
    let mut candidate = package.clone();
    candidate["semantic_graph"]["nodes"][0]["semantic_form"] = json!("not_a_form");
    assert!(!schema.is_valid(&candidate), "an unknown model form fails the schema");

    // FR-322-AC-141, FR-322-AC-143, FR-322-AC-144: the package carries the
    // application nodes over the body-less declaration nodes, each keyed by
    // its application-node preimage, which is published.
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let all_preimages = jsonschema::validator_for(&preimage_schema).expect("preimage schema");
    let keyed = vectors["application_key_vectors"].as_array().expect("application key vectors");
    let mut vector_names = BTreeSet::new();
    for vector in keyed {
        let name = vector["name"].as_str().expect("name");
        assert!(vector_names.insert(name), "application key vector names are unique");
        assert!(all_preimages.is_valid(&vector["preimage"]), "{name}: closed preimage");
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            vector["sha256"].as_str().unwrap(),
            "{name}: key"
        );
        let node = nodes
            .iter()
            .find(|node| node["node_id"]["digest"] == vector["sha256"])
            .unwrap_or_else(|| panic!("{name}: the fixture carries the node"));
        assert_eq!(application_node_preimage(node, nodes), Some(vector["preimage"].clone()), "{name}");
        assert!(application_node_key_is_current(node, nodes), "{name}: key is current");
    }
    let count_of = |tag: &str, form: &str| {
        nodes
            .iter()
            .filter(|node| node["node_tag"] == tag && node["semantic_form"] == form)
            .count()
    };
    assert_eq!(count_of("value", "union_value"), 5, "union values over Shape and List");
    assert_eq!(count_of("value", "enum_value"), 1, "an enum value of Color");
    assert_eq!(count_of("expression", "query"), 2, "two position projections");
    assert_eq!(count_of("expression", "case"), 1, "one case over Shape");
    assert_eq!(count_of("expression", "binary"), 3, "three equalities");
    assert_eq!(keyed.len(), 5 + 2 + 1 + 3, "every application node is a published key vector");
    let union_value_over_shape = |case: &str| {
        keyed.iter().find(|vector| {
            vector["preimage"]["semantic_form"] == "union_value"
                && vector["preimage"]["semantic_type"]["digest"] == vectors["declaration_nodes"][3]["sha256"]
                && vector["preimage"]["body"]["members"][0]["name"] == case
        })
    };
    let shape_keys = ["Empty", "Flag", "Both"].map(|case| {
        union_value_over_shape(case).unwrap_or_else(|| panic!("a Shape value of {case} is vectored"))["sha256"].clone()
    });
    assert_eq!(shape_keys.iter().map(Value::to_string).collect::<BTreeSet<_>>().len(), 3, "the three Shape values key apart");
}

/// FR-322-AC-141 to FR-322-AC-145: family, position, field, member, union
/// value, enum value, equality, identity and intake cases over the projection.
#[trace(
    "TC-940",
    "FR-322-AC-141",
    "FR-322-AC-142",
    "FR-322-AC-143",
    "FR-322-AC-144",
    "FR-322-AC-145",
    "FR-322-AC-147"
)]
#[test]
fn tc_940_tuple_and_union_cases_run_from_the_published_vectors() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let document = &vectors["domain_package"];
    assert_eq!(shapes_intake_refusal(document), None, "the published document is admitted");
    let mut names = BTreeSet::new();
    let mut seen = |name: &Value| {
        assert!(names.insert(name.as_str().expect("case name").to_owned()), "case names are unique");
    };
    for case in vectors["family_cases"].as_array().unwrap() {
        seen(&case["name"]);
        assert_eq!(shapes_family(document, case["node"].as_str().unwrap()), case["expected"], "{}", case["name"]);
    }
    for case in vectors["position_cases"].as_array().unwrap() {
        seen(&case["name"]);
        // A refused position is a well-formed KeyOrdinal that matches none,
        // never one the reader fails to parse.
        assert!(key_ordinal(&case["position"]).is_some(), "{}: a KeyOrdinal", case["name"]);
        assert_eq!(
            shapes_position(document, case["tuple"].as_str().unwrap(), &case["position"]),
            case["expected"],
            "{}",
            case["name"]
        );
    }
    for case in vectors["field_type_cases"].as_array().unwrap() {
        seen(&case["name"]);
        assert_eq!(
            shapes_field_type(document, case["object"].as_str().unwrap(), case["field"].as_str().unwrap()),
            case["expected"],
            "{}",
            case["name"]
        );
    }
    for case in vectors["member_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let members = shapes_members(document, case["union"].as_str().unwrap());
        let observed = members
            .iter()
            .map(|(name, payload)| json!({"case": name, "arity": usize::from(payload.is_some()), "payload": payload}))
            .collect::<Vec<_>>();
        assert_eq!(Value::Array(observed), case["expected"], "{}", case["name"]);
    }
    let mismatch = || shapes_refusal("ill_typed", "type-mismatch");
    for case in vectors["union_value_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let union = case["union"].as_str().unwrap();
        let observed = if shapes_family(document, union) != "union" {
            mismatch()
        } else {
            match shapes_members(document, union).into_iter().find(|(name, _)| case["case"] == name.as_str()) {
                None => mismatch(),
                Some((_, payload)) => {
                    let supplied = case["payload"].as_array().unwrap();
                    if supplied.iter().cloned().collect::<Vec<_>>() == payload.into_iter().collect::<Vec<_>>() {
                        json!({"admitted": true})
                    } else {
                        mismatch()
                    }
                }
            }
        };
        assert_eq!(observed, case["expected"], "{}", case["name"]);
    }
    for case in vectors["enum_value_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let declared = case["type"].as_str().unwrap();
        let in_enum = shapes_family(document, declared) == "enum"
            && shapes_declared(document, "variant_types", declared).unwrap()["cases"]
                .as_array()
                .unwrap()
                .iter()
                .any(|candidate| candidate["name"] == case["case"]);
        let (code, cause) = expected_refusal(&case["expected"]).map_or((None, None), |(code, cause)| (Some(code), cause));
        assert_eq!(!in_enum, code.is_some(), "{}", case["name"]);
        if let Some(code) = code {
            assert_eq!(code, "invalid_semantic_graph");
            assert_eq!(cause, None);
        }
    }
    for case in vectors["equality_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let family = shapes_family(document, case["type"].as_str().unwrap());
        let admitted = match case["operation"].as_str().unwrap() {
            "quire.op.structural.eq" => ["tuple", "union"].contains(&family),
            "quire.op.enum.eq" => family == "enum",
            other => panic!("unexpected operation {other}"),
        };
        let observed = if admitted { json!({"admitted": true}) } else { shapes_refusal("ill_typed", "operator-ineligible") };
        assert_eq!(observed, case["expected"], "{}", case["name"]);
    }
    let key_of = |name: &str| {
        vectors["declaration_nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|declared| declared["name"] == name)
            .map(|declared| declared["sha256"].clone())
            .unwrap()
    };
    for declared in vectors["declaration_nodes"].as_array().unwrap() {
        let name = declared["name"].as_str().unwrap();
        if name == "Holder" {
            continue;
        }
        assert_eq!(
            declaration_key(document, name),
            declared["sha256"].as_str().unwrap(),
            "{name}: a producer deriving the key from the document gets the published key"
        );
    }
    for case in vectors["identity_cases"].as_array().unwrap() {
        seen(&case["name"]);
        if let Some(pair) = case["nodes"].as_array() {
            assert_ne!(key_of(pair[0].as_str().unwrap()), key_of(pair[1].as_str().unwrap()), "{}", case["name"]);
            assert_ne!(
                declaration_key(document, pair[0].as_str().unwrap()),
                declaration_key(document, pair[1].as_str().unwrap()),
                "{}",
                case["name"]
            );
            continue;
        }
        assert!(identity_case_holds(document, case), "{}", case["name"]);
        // The case can fail: each boolean it states, flipped, is false of the
        // patched document, so the node key is not read back from the case.
        for field in ["node_key_unchanged", "selection_digest_changes", "root_key_changes"] {
            let mut flipped = case.clone();
            let stated = flipped["expected"][field].as_bool().expect("a stated boolean");
            flipped["expected"][field] = json!(!stated);
            assert!(!identity_case_holds(document, &flipped), "{}: flipping {field} fails", case["name"]);
        }
    }
    // A key that did read the positions would change with them: the patched
    // document keys another declaration apart from the published one.
    let mut other = document.clone();
    other["tuple_types"][0]["node"] = json!("ix://acme/shapes/Renamed");
    assert_ne!(declaration_key(&other, "Renamed"), key_of("Pair"), "the owner is the key's input");
    for case in vectors["recursion_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let package = parsed_fixture(include_str!(
            "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
        ));
        let catalog = parsed_fixture(OPERATION_CATALOG);
        let oracle = OperationOracle::for_package(&catalog, &package);
        let node_of = |name: &str| {
            oracle
                .nodes
                .iter()
                .find(|node| node["owner"]["node"] == format!("ix://acme/shapes/{name}").as_str())
                .unwrap_or_else(|| panic!("{}: the fixture carries {name}", case["name"]))
        };
        let type_name = |node: &Value| match node["node_tag"].as_str() {
            Some("model") => node["owner"]["node"].as_str().unwrap().rsplit('/').next().unwrap().to_owned(),
            _ => {
                let form = node["semantic_form"].as_str().unwrap();
                form[..1].to_uppercase() + &form[1..]
            }
        };
        match case["name"].as_str().unwrap() {
            "TR-01" => {
                assert_eq!(shapes_intake_refusal(document), None, "{}", case["name"]);
                for name in case["expected"]["cycle"].as_array().unwrap() {
                    assert!(shapes_reaches_itself(document, name.as_str().unwrap()), "{name} is on a cycle");
                }
                assert!(!shapes_reaches_itself(document, "Pair"), "Pair is on none");
            }
            "TR-02" => {
                for name in case["nodes"].as_array().unwrap() {
                    let name = name.as_str().unwrap();
                    let node = node_of(name);
                    assert!(node.get("recursion_group").is_none(), "{name}: no recursion group");
                    assert_eq!(node["node_id"]["digest"], declaration_key(document, name), "{name}");
                    assert!(structural_node_key_is_current(node), "{name}");
                }
            }
            "TR-03" => {
                for (name, steps) in case["expected"].as_object().unwrap() {
                    let node = node_of(name);
                    let observed = oracle
                        .leaf_children(&node["node_id"])
                        .expect("a walk out of a model type")
                        .into_iter()
                        .map(|(path, child)| json!({"path": path, "type": type_name(oracle.node(child).unwrap())}))
                        .collect::<Vec<_>>();
                    assert_eq!(Value::Array(observed), *steps, "{name}");
                }
            }
            "TR-04" => {
                let leaves = oracle.expected_leaves(&node_of(case["type"].as_str().unwrap())["node_id"]);
                assert!(matches!(leaves, Some(ref found) if found.is_empty()), "{}", case["name"]);
                assert!(read_package_semantics(&package, &catalog).is_ok(), "the full reader admits it");
            }
            other => panic!("unexpected recursion case {other}"),
        }
    }
    for case in vectors["intake_cases"].as_array().unwrap() {
        seen(&case["name"]);
        let mut changed = document.clone();
        for patch in case["patch"].as_array().unwrap() {
            apply_json_patch(&mut changed, patch);
        }
        let observed = shapes_intake_refusal(&changed);
        match expected_refusal(&case["expected"]) {
            None => assert_eq!(observed, None, "{}", case["name"]),
            Some((code, cause)) => {
                let (observed_code, observed_cause, at) = observed.unwrap_or_else(|| panic!("{} is refused", case["name"]));
                assert_eq!(observed_code, code, "{}", case["name"]);
                assert_eq!(Some(observed_cause.to_owned()), cause, "{}", case["name"]);
                assert_eq!(at, case["expected"]["refused"]["at"].as_str().unwrap(), "{}", case["name"]);
            }
        }
    }
    let published = [
        "family_cases",
        "position_cases",
        "field_type_cases",
        "member_cases",
        "union_value_cases",
        "enum_value_cases",
        "equality_cases",
        "identity_cases",
        "recursion_cases",
        "intake_cases",
    ]
    .map(|list| vectors[list].as_array().expect(list).len());
    assert_eq!(names.len(), published.iter().sum::<usize>(), "every published case ran");
    assert_eq!(published, [5, 7, 3, 1, 8, 4, 5, 4, 4, 10], "no published case was dropped");
}

/// FR-322-AC-140 and FR-322-AC-146: the wire cases over the fixture.
#[trace("TC-940", "FR-322-AC-140", "FR-322-AC-146")]
#[test]
fn tc_940_tuple_node_wire_cases_run_from_the_published_vectors() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let preimage_schema = parsed_fixture(NODE_PREIMAGE_SCHEMA);
    let preimage_validator = schema_for_def(&preimage_schema, "ModelDeclarationNode");
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let base = parsed_fixture(include_str!(
        "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
    ));
    let mut ids = BTreeSet::new();
    for vector in vectors["wire_cases"].as_array().expect("wire cases") {
        let id = vector["id"].as_str().expect("id");
        assert!(ids.insert(id), "vector ids are unique");
        let mut package = base.clone();
        for patch in vector["patch"].as_array().expect("patch") {
            apply_json_patch(&mut package, patch);
        }
        assert_eq!(
            unqualified_jcs_sha256(&package_id_preimage(&package)),
            package["package_id"]["digest"].as_str().unwrap(),
            "{id}: the vector carries the recomputed package_id"
        );
        let schema_valid = schema.is_valid(&package);
        assert_eq!(schema_valid, vector["schema"] == "valid", "{id}: schema");
        let position_of = |locus: &str| {
            locus
                .strip_prefix("/semantic_graph/nodes/")
                .and_then(|rest| rest.split('/').next())
                .and_then(|index| index.parse::<usize>().ok())
                .expect("node locus")
        };
        let observed = if schema_valid {
            match read_package_semantics(&package, &catalog) {
                Ok(()) => None,
                Err(ReadRefusal::OwnerSelection(id)) => {
                    let position = package["semantic_graph"]["nodes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .position(|node| node["node_id"] == id)
                        .unwrap();
                    Some((
                        "missing_declaration".to_owned(),
                        "missing-selection".to_owned(),
                        format!("/semantic_graph/nodes/{position}/owner"),
                    ))
                }
                Err(ReadRefusal::StaleNodeKey(id)) => {
                    let position = package["semantic_graph"]["nodes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .position(|node| node["node_id"] == id)
                        .unwrap();
                    Some((
                        "invalid_package".to_owned(),
                        "stale-node-key".to_owned(),
                        format!("/semantic_graph/nodes/{position}/node_id"),
                    ))
                }
                Err(ReadRefusal::InvalidSemanticGraph(id)) => {
                    let position = package["semantic_graph"]["nodes"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .position(|node| node["node_id"] == id)
                        .unwrap();
                    Some((
                        "invalid_semantic_graph".to_owned(),
                        String::new(),
                        format!("/semantic_graph/nodes/{position}"),
                    ))
                }
                Err(ReadRefusal::Operation(refusal)) => {
                    let oracle = OperationOracle::for_package(&catalog, &package);
                    let mut ordered = oracle.nodes.iter().enumerate().collect::<Vec<_>>();
                    ordered.sort_by_key(|(_, node)| node["node_id"]["digest"].as_str().unwrap_or_default());
                    let (position, _) = ordered
                        .into_iter()
                        .find(|(_, node)| {
                            oracle.union_value_refusal(node).is_some()
                                || application_terms(&node["body"])
                                    .into_iter()
                                    .any(|term| oracle.classify_in(node, term).is_err())
                        })
                        .unwrap_or_else(|| panic!("{id}: a refusing node"));
                    Some((
                        refusal.code().to_owned(),
                        refusal.cause().to_owned(),
                        format!("/semantic_graph/nodes/{position}"),
                    ))
                }
                Err(other) => panic!("{id}: unexpected reader refusal {other:?}"),
            }
        } else {
            let locus = vector["locus"].as_str().expect("locus");
            let node = &package["semantic_graph"]["nodes"][position_of(locus)];
            if is_model_declaration(node) {
                let preimage = structural_node_preimage(node).expect("structural node");
                assert!(
                    !preimage_validator.is_valid(&preimage),
                    "{id}: the closed node-preimage schema refuses it too"
                );
            } else {
                let node_pointer = format!("/semantic_graph/nodes/{}", position_of(locus));
                assert!(
                    schema
                        .iter_errors(&package)
                        .any(|error| error.instance_path().to_string().starts_with(&node_pointer)),
                    "{id}: the V2 schema refuses the node"
                );
            }
            Some(("malformed_wire".to_owned(), String::new(), locus.to_owned()))
        };
        let expected = match vector["outcome"].as_str().expect("outcome") {
            "admitted" => None,
            refused => Some((
                refused.strip_prefix("refused:").expect("refused outcome").to_owned(),
                vector["cause"].as_str().unwrap_or("").to_owned(),
                vector["locus"].as_str().expect("locus").to_owned(),
            )),
        };
        assert_eq!(observed, expected, "{id}");
    }
    assert_eq!(ids.len(), 27, "every published wire case ran");
}

/// FR-322-AC-152 and FR-322-AC-153: the tuple.project wire cases refuse at the
/// application node, and a wire case states a `cause` exactly when its code
/// carries one.
#[trace("TC-940", "FR-322-AC-152", "FR-322-AC-153")]
#[test]
fn tc_940_wire_case_loci_and_causes_follow_the_refusal_vocabulary() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let base = parsed_fixture(include_str!(
        "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
    ));
    let carries_cause = [
        "invalid_package",
        "ill_typed",
        "ambiguous_declaration",
        "missing_declaration",
        "invalid_model_binding",
    ];
    let node = &base["semantic_graph"]["nodes"][31];
    assert_eq!(node["body"]["operation"]["identity"], "quire.op.tuple.project");
    assert_eq!(node["body"]["operation"]["member"]["position"], 0);
    let operand = &node["body"]["arguments"][0]["target"];
    let nodes = base["semantic_graph"]["nodes"].as_array().expect("nodes");
    let operand_node = nodes.iter().find(|n| n["node_id"] == *operand).expect("operand");
    assert_eq!(operand_node["semantic_form"], "parameter", "the Pair-typed parameter");
    for vector in vectors["wire_cases"].as_array().expect("wire cases") {
        let id = vector["id"].as_str().expect("id");
        let code = vector["outcome"].as_str().and_then(|o| o.strip_prefix("refused:"));
        let has_cause = vector.get("cause").is_some();
        assert_eq!(
            has_cause,
            code.is_some_and(|code| carries_cause.contains(&code)),
            "{id}: a cause is stated exactly when the code carries one"
        );
        if matches!(id, "TW-18" | "TW-19" | "TW-20" | "TW-26") {
            assert_eq!(vector["locus"], "/semantic_graph/nodes/31", "{id}: the application node");
        }
        if matches!(id, "TW-18" | "TW-19" | "TW-20") {
            assert_eq!(vector["outcome"], "refused:ill_typed", "{id}");
            assert_eq!(vector["cause"], "operator-ineligible", "{id}");
        }
    }
    // The FR-322 text names the same locus and rows.
    let fr322 = include_str!("../spec/objects/interfaces/FR-322-checked-package-artifact.md");
    assert!(fr322.contains("`/semantic_graph/nodes/31` in\n  `fixtures/positive-model-tuple-union.json`"));
    let row = |name: &str| {
        fr322
            .lines()
            .find(|line| line.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row"))
    };
    let ac152 = row("FR-322-AC-152");
    for id in ["TW-18", "TW-19", "TW-20", "TW-26"] {
        assert!(ac152.contains(id), "AC-152 lists {id}");
    }
    assert!(ac152.contains("/semantic_graph/nodes/31"));
    // AC-153's row lists exactly the cause-bearing rows, derived from the vocabulary.
    let ac153 = row("FR-322-AC-153");
    let (with_cause, without) = ac153.split_once("and the admitted case").expect("AC-153 shape");
    let expand = |text: &str| -> BTreeSet<String> {
        let ids: Vec<usize> = text
            .split("`TW-")
            .skip(1)
            .map(|rest| rest[..2].parse().expect("row number"))
            .collect();
        let mut out = BTreeSet::new();
        let mut index = 0;
        while index < ids.len() {
            let range_end = text.contains(&format!("`TW-{:02}` to `TW-{:02}`", ids[index], ids.get(index + 1).copied().unwrap_or(0)));
            if range_end {
                out.extend((ids[index]..=ids[index + 1]).map(|n| format!("TW-{n:02}")));
                index += 2;
            } else {
                out.insert(format!("TW-{:02}", ids[index]));
                index += 1;
            }
        }
        out
    };
    let mut expect_with = BTreeSet::new();
    let mut expect_without = BTreeSet::new();
    for vector in vectors["wire_cases"].as_array().expect("wire cases") {
        let id = vector["id"].as_str().expect("id").to_owned();
        if vector.get("cause").is_some() {
            expect_with.insert(id);
        } else {
            expect_without.insert(id);
        }
    }
    assert_eq!(expand(with_cause), expect_with, "AC-153 cause-bearing rows");
    assert_eq!(expand(without), expect_without, "AC-153 cause-free rows");
}

/// Every node reference (`quire.checked-semantic-node/v1` digest) in `value`:
/// reference terms, member declarations, result types and dependencies.
fn reference_term_targets(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            let in_domain =
                map.get("domain").and_then(Value::as_str) == Some("quire.checked-semantic-node/v1");
            if let Some(digest) = map.get("digest").and_then(Value::as_str).filter(|_| in_domain) {
                out.push(digest.to_owned());
            }
            map.values().for_each(|inner| reference_term_targets(inner, out));
        }
        Value::Array(items) => items.iter().for_each(|inner| reference_term_targets(inner, out)),
        _ => {}
    }
}

/// FR-322-AC-154: the `reference` targets of the graph that name no node and
/// are not deferred. A dangling target is deferred when it equals the recomputed
/// key of a model declaration node whose retained key is stale (the reference
/// dangles only because that node was rekeyed); every other dangling target is
/// a graph-shape refusal that precedes the stale-key stage.
fn without_id(node: &Value) -> Value {
    let mut node = node.clone();
    node.as_object_mut().expect("node").remove("node_id");
    node
}

fn undeferred_dangling_targets(package: &Value) -> Vec<String> {
    let nodes = package["semantic_graph"]["nodes"].as_array().expect("nodes");
    let present: BTreeSet<&str> = nodes
        .iter()
        .filter_map(|node| node["node_id"]["digest"].as_str())
        .collect();
    let recomputed: BTreeSet<String> = nodes
        .iter()
        .filter(|node| is_model_declaration(node) && !model_declaration_key_is_current(node))
        .filter_map(|node| {
            let mut empty = node.clone();
            empty["body"] = json!({"term": "aggregate", "members": []});
            structural_node_preimage(&empty)
        })
        .map(|preimage| unqualified_jcs_sha256(&preimage))
        .collect();
    let mut targets = Vec::new();
    nodes.iter().for_each(|node| reference_term_targets(&without_id(node), &mut targets));
    targets
        .into_iter()
        .filter(|target| !present.contains(target.as_str()) && !recomputed.contains(target))
        .collect()
}

/// FR-322-AC-154: TW-05 rekeys the `Pair` node over a non-empty body, so every
/// reference to its published key dangles; each equals the recomputed key, is
/// deferred, and the stale-node-key at node 0 is the first refusal. An
/// independent dangling reference is not deferred.
#[trace("TC-940", "FR-322-AC-154")]
#[test]
fn tc_940_rekey_dependent_dangling_references_defer_to_the_stale_key() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let base = parsed_fixture(include_str!(
        "../proposals/checked-package-v2/fixtures/positive-model-tuple-union.json"
    ));
    assert!(undeferred_dangling_targets(&base).is_empty(), "the base has no dangling reference");
    let vector = vectors["wire_cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|vector| vector["id"] == "TW-05")
        .expect("TW-05");
    assert_eq!(vector["outcome"], "refused:invalid_package");
    assert_eq!(vector["cause"], "stale-node-key");
    assert_eq!(vector["locus"], "/semantic_graph/nodes/0/node_id");
    let mut package = base.clone();
    for patch in vector["patch"].as_array().unwrap() {
        apply_json_patch(&mut package, patch);
    }
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let present: BTreeSet<&str> =
        nodes.iter().filter_map(|node| node["node_id"]["digest"].as_str()).collect();
    let mut targets = Vec::new();
    nodes.iter().for_each(|node| reference_term_targets(&without_id(node), &mut targets));
    let dangling = targets.iter().filter(|target| !present.contains(target.as_str())).count();
    assert!(dangling > 0, "rekeying Pair leaves references to its derived key dangling");
    assert!(undeferred_dangling_targets(&package).is_empty(), "all of them defer to the rekey");
    let mut independent = package.clone();
    independent["semantic_graph"]["nodes"][1]["body"] = json!({
        "term": "aggregate",
        "members": [{"term": "reference", "target": {
            "domain": "quire.checked-semantic-node/v1",
            "digest": "ff".repeat(32)
        }}]
    });
    assert_eq!(undeferred_dangling_targets(&independent), vec!["ff".repeat(32)]);
}


const CLASS_CATALOG_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/class-catalog-vectors.json");

/// The concrete classes of the CCF-01 subject, in canonical order, each as
/// (class, memory model): declared classes, `cend`, three drains, then `flush`.
const EXPECTED_CONCRETE_CLASSES: [(&str, &str); 10] = [
    ("Impl::Do", ""),
    ("Impl::E", ""),
    ("Impl::K", ""),
    ("Impl::T", ""),
    ("Impl::ev", ""),
    ("cend Impl::T", ""),
    ("memory drain", "ra"),
    ("memory drain", "sc"),
    ("memory drain", "tso"),
    ("memory flush", "tso"),
];

/// Reader-level class-catalog check over a schema-valid catalog: canonical
/// order by (class UTF-8 bytes, memory model), no duplicate catalog key, and
/// the effect, commit and cend classes the producer must catalogue.
fn class_catalog_reader_refusal(catalog: &Value) -> Option<&'static str> {
    for side in ["concrete", "abstract"] {
        let entries = catalog[side].as_array().expect("catalog side");
        let keys: Vec<(String, String)> = entries
            .iter()
            .map(|entry| {
                (
                    entry["class"].as_str().expect("class").to_owned(),
                    entry["memory_model"].as_str().unwrap_or("").to_owned(),
                )
            })
            .collect();
        for pair in keys.windows(2) {
            if pair[0] == pair[1] {
                return Some("duplicate_class");
            }
            if pair[0] > pair[1] {
                return Some("misordered_class");
            }
        }
    }
    let present: BTreeSet<(String, String)> = catalog["concrete"]
        .as_array()
        .expect("concrete")
        .iter()
        .map(|entry| {
            (
                entry["class"].as_str().expect("class").to_owned(),
                entry["memory_model"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect();
    for (class, model) in EXPECTED_CONCRETE_CLASSES {
        if !present.contains(&(class.to_owned(), model.to_owned())) {
            return Some("omitted_class");
        }
    }
    None
}

#[trace("TC-310", "FR-375-AC-28")]
#[test]
fn tc_310_class_catalog_carries_effect_commit_and_cend_classes() {
    let defs = parsed_fixture(SCHEMA)["$defs"].clone();
    let wrapper = json!({"$ref": "#/$defs/RefinementClassCatalog", "$defs": defs});
    let validator = jsonschema::validator_for(&wrapper).expect("catalog schema is valid");
    let vectors = parsed_fixture(CLASS_CATALOG_VECTORS);
    let mut admitted = 0;
    for case in vectors["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let catalog = &case["catalog"];
        let observed = if !validator.is_valid(catalog) {
            "schema_refused"
        } else {
            class_catalog_reader_refusal(catalog).unwrap_or("admitted")
        };
        assert_eq!(observed, case["expected"].as_str().expect("expected"), "{name}");
        if observed == "admitted" {
            admitted += 1;
            let concrete: Vec<(&str, &str)> = catalog["concrete"]
                .as_array()
                .unwrap()
                .iter()
                .map(|entry| {
                    (
                        entry["class"].as_str().unwrap(),
                        entry["memory_model"].as_str().unwrap_or(""),
                    )
                })
                .collect();
            assert_eq!(
                concrete,
                EXPECTED_CONCRETE_CLASSES,
                "{name}: the canonical concrete class order"
            );
        }
    }
    assert_eq!(admitted, 1, "exactly the positive vector is admitted");
    assert_eq!(
        vectors["cases"].as_array().unwrap().len(),
        15,
        "every single-drop, misordered and duplicate case is present"
    );
}

/// Index of the node a vectors file names, looked up by role in `node_indexes`.
fn timed_node_index(role: &str) -> usize {
    let vectors: Value = serde_json::from_str(TIMED_MODEL_NODE_VECTORS).unwrap();
    vectors["node_indexes"][role].as_u64().unwrap_or_else(|| panic!("node role {role}")) as usize
}

/// The scalar type node of a rendered type name, found by its semantic form.
fn scalar_type_node<'a>(nodes: &'a [Value], form: &str) -> &'a str {
    nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == form)
        .and_then(|node| node["node_id"]["digest"].as_str())
        .unwrap_or_else(|| panic!("scalar_type/{form} node"))
}

/// The type node that selects a slot of `collection`: the Integer node for a
/// sequence-like collection, and the key type's node for a map, read from the
/// rendered `Map<K, V>` type of the step's resolved row. `None` for a key type
/// with no node in the package.
fn slot_domain_node<'a>(nodes: &'a [Value], collection: &str, map_type: &str) -> Option<&'a str> {
    if collection != "map" {
        return Some(scalar_type_node(nodes, "integer"));
    }
    let key = map_type.strip_prefix("Map<")?.split_once(", ")?.0;
    let form = match key {
        "Text" => "text",
        "Integer" | "Int" => "integer",
        _ => return None,
    };
    Some(scalar_type_node(nodes, form))
}

/// Whether every slot step of a `clock_place` selects by the symbolic selector.
fn place_selectors_are_symbolic(place: &Value, nodes: &[Value]) -> bool {
    let steps = place["body"]["members"][1]["value"]["members"].as_array().unwrap();
    let rows = place["body"]["members"][2]["value"]["members"].as_array().unwrap();
    steps.iter().enumerate().filter(|(_, step)| step["members"][0]["value"]["value"] == "slot").all(|(at, step)| {
        let members = step["members"].as_array().unwrap();
        let collection = members[1]["value"]["value"].as_str().unwrap();
        let declaring = rows[at]["members"][1]["value"]["value"].as_str().unwrap();
        let selector = &members[2]["value"];
        slot_domain_node(nodes, collection, declaring)
            .is_some_and(|want| selector["term"] == "reference" && selector["target"]["digest"] == want)
    })
}

/// The guard and duplicate-place checks of FR-322 "Timed-model nodes" over a
/// whole package, as the cause and code a reader gives (`None` when admitted).
fn timed_guard_and_place_cause(package: &Value) -> Option<(&'static str, &'static str)> {
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let by_id = |id: &Value| nodes.iter().find(|node| node["node_id"] == *id).unwrap();
    for root in nodes.iter().filter(|node| node["semantic_form"] == "timed_model") {
        let binding = |name: &str| {
            root["body"]["members"]
                .as_array()
                .unwrap()
                .iter()
                .find(|member| member["name"] == name)
                .map(|member| member["value"]["members"].as_array().unwrap().clone())
                .unwrap_or_else(|| panic!("root binding {name}"))
        };
        let places = binding("clock_places");
        for (at, place) in places.iter().enumerate() {
            let path = &by_id(&place["target"])["body"]["members"][1];
            if places[..at].iter().any(|earlier| &by_id(&earlier["target"])["body"]["members"][1] == path) {
                return Some(("invalid_package", "duplicate-clock-place"));
            }
        }
        let listed = binding("guards");
        let owned = nodes.iter().filter(|node| node["semantic_form"] == "clock_guard");
        for guard in owned {
            if !listed.iter().any(|entry| entry["target"] == guard["node_id"]) {
                return Some(("invalid_package", "timed-node-reference"));
            }
        }
    }
    None
}

#[trace("TC-367", "FR-322-AC-148", "FR-322-AC-149", "FR-322-AC-150")]
#[test]
fn tc_367_slot_selector_vectors_are_symbolic_template_nodes() {
    let vectors: Value = serde_json::from_str(TIMED_SLOT_SELECTOR_VECTORS).expect("vectors JSON");
    let package: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).expect("fixture JSON");
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let schema = parsed_fixture(SCHEMA);
    let timed_schema = schema_for_def(&schema, "TimedModelNode");
    assert_eq!(vectors["type_nodes"]["integer"], scalar_type_node(nodes, "integer"));
    assert_eq!(vectors["type_nodes"]["text"], scalar_type_node(nodes, "text"));
    let mut keys = BTreeSet::new();
    let (mut positives, mut refusals) = (0, 0);
    for case in vectors["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let node = &case["node"];
        assert!(structural_node_key_is_current(node), "{id} key hashes its body");
        assert_eq!(node["semantic_type"], node["node_id"], "{id} is self-typed");
        assert_eq!(node["dependencies"], json!(body_reference_targets(&node["body"])), "{id}");
        assert!(timed_schema.is_valid(node), "{id} matches the closed schema");
        assert!(keys.insert(node["node_id"]["digest"].as_str().unwrap().to_owned()), "{id} key is distinct");
        let symbolic = place_selectors_are_symbolic(node, nodes);
        match case["expected"].as_str().unwrap() {
            "admitted" => {
                assert!(symbolic, "{id} carries symbolic selectors");
                positives += 1;
            }
            "refused" => {
                assert!(!symbolic, "{id} selector is refused");
                assert_eq!(case["expected_cause"], "malformed-clock-place");
                refusals += 1;
            }
            other => panic!("{id}: {other}"),
        }
        if let Some(role) = case["published_node"].as_str() {
            assert_eq!(*node, nodes[timed_node_index(role)], "{id} is the published place");
        }
        let slots = node["body"]["members"][1]["value"]["members"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|step| step["members"][0]["value"]["value"] == "slot")
            .count();
        for instance in case["instances"].as_array().into_iter().flatten() {
            assert_eq!(instance["slots"].as_array().unwrap().len(), slots, "{id}");
        }
    }
    assert_eq!((positives, refusals), (8, 4));

    // Guards never enter place identity: SL-07 and SL-08 share one path.
    let cases = vectors["cases"].as_array().unwrap();
    let by = |id: &str| cases.iter().find(|c| c["id"] == id).unwrap()["node"].clone();
    assert_eq!(by("SL-07")["body"]["members"][1], by("SL-08")["body"]["members"][1]);
    assert_ne!(by("SL-07")["node_id"], by("SL-08")["node_id"]);
    assert!(by("SL-08")["body"]["members"][3]["value"]["members"].as_array().unwrap().is_empty());

    // GD-01: the published tick root has empty clock aggregates.
    let tick: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES).unwrap();
    let tick_root = &tick["semantic_graph"]["nodes"][timed_node_index_of(&tick, "timed_model")];
    for member in tick_root["body"]["members"].as_array().unwrap().iter().skip(3) {
        assert!(member["value"]["members"].as_array().unwrap().is_empty(), "{}", member["name"]);
    }
    assert_eq!(timed_guard_and_place_cause(&tick), None);

    // GD-02 and GD-03 replay as schema-valid package mutations with recomputed ids.
    assert_eq!(timed_guard_and_place_cause(&package), None, "the published package is the control");
    let validator = jsonschema::validator_for(&schema).unwrap();
    for gd in vectors["guards"].as_array().unwrap().iter().filter(|g| g.get("mutation").is_some()) {
        let mut adverse = package.clone();
        let root_at = timed_node_index("timed_model");
        match gd["mutation"].as_str().unwrap() {
            "duplicate-place-different-guards" => {
                let mut twin = nodes[timed_node_index("clock_place")].clone();
                twin["body"]["members"][3]["value"]["members"] = json!([]);
                twin["dependencies"] = json!(body_reference_targets(&twin["body"]));
                let placeholder = json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": "ee".repeat(32)});
                twin["node_id"] = placeholder.clone();
                twin["semantic_type"] = placeholder.clone();
                let graph = adverse["semantic_graph"]["nodes"].as_array_mut().unwrap();
                graph.push(twin);
                let root = &mut graph[root_at];
                root["body"]["members"][3]["value"]["members"]
                    .as_array_mut().unwrap()
                    .push(json!({"term": "reference", "target": placeholder.clone()}));
                root["dependencies"].as_array_mut().unwrap().push(placeholder);
            }
            "unlisted-clock-guard" => {
                let guard = nodes[timed_node_index("clock_guard")]["node_id"].clone();
                let root = &mut adverse["semantic_graph"]["nodes"][root_at];
                let listed = root["body"]["members"][5]["value"]["members"].as_array_mut().unwrap();
                listed.retain(|entry| entry["target"] != guard);
                root["dependencies"].as_array_mut().unwrap().retain(|dep| *dep != guard);
            }
            other => panic!("unhandled guard vector {other}"),
        }
        rekey_stale_nodes(&mut adverse);
        assert!(validator.is_valid(&adverse), "{} is a schema-valid semantic control", gd["id"]);
        assert_eq!(
            adverse["package_id"]["digest"],
            unqualified_jcs_sha256(&package_id_preimage(&adverse)),
            "{} retains the recomputed package id",
            gd["id"]
        );
        let cause = timed_guard_and_place_cause(&adverse).expect("the mutation is refused");
        assert_eq!(cause, (gd["expected_code"].as_str().unwrap(), gd["expected_cause"].as_str().unwrap()), "{}", gd["id"]);
    }
}

/// Position of the first node of a package with the given semantic form.
fn timed_node_index_of(package: &Value, form: &str) -> usize {
    package["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .position(|node| node["semantic_form"] == form)
        .unwrap_or_else(|| panic!("{form} node"))
}

/// The reader-side checks of FR-322 "Presence" over one `presence_read` node,
/// against the vectors' places and parameters: the cause and code, and the
/// argument pointer, or `None` when the node is admitted.
fn presence_cause(
    case: &Value,
    places: &BTreeMap<String, Value>,
    parameters: &BTreeMap<String, Value>,
    owner_type: &Value,
    slot_type: &Value,
) -> Option<(&'static str, &'static str, String)> {
    let arguments = case["body"]["arguments"].as_array().unwrap();
    let target = |k: usize| arguments[k]["target"]["digest"].as_str().unwrap();
    let Some(place) = places.get(target(0)) else {
        return Some(("ill_typed", "operator-ineligible", "/body/arguments/0".to_owned()));
    };
    let slots = place["body"]["members"][1]["value"]["members"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|step| step["members"][0]["value"]["value"] == "slot")
        .count();
    if arguments.len() != 2 + slots {
        return Some(("ill_typed", "operator-ineligible", "/body/arguments".to_owned()));
    }
    let type_of = |k: usize| parameters.get(target(k)).map(|node| node["semantic_type"].clone());
    if type_of(1).as_ref() != Some(owner_type) {
        return Some(("ill_typed", "type-mismatch", "/body/arguments/1".to_owned()));
    }
    for k in 2..arguments.len() {
        if type_of(k).as_ref() != Some(slot_type) {
            return Some(("ill_typed", "type-mismatch", format!("/body/arguments/{k}")));
        }
    }
    None
}

/// A hidden presence member anywhere in the presence vectors' model, place or
/// parameter bodies, or a hidden presence field in the declared members.
fn presence_hidden_member(vectors: &Value) -> Result<(), String> {
    let hidden = |value: &Value| {
        let text = serde_json::to_string(value).unwrap();
        text.contains("\"presence\"") || text.contains("Option<Boolean>") || text.contains("Option<ix://quire/native/Boolean>")
    };
    let presence = &vectors["presence"];
    for (name, place) in presence["places"].as_object().unwrap() {
        if hidden(&place["node"]["body"]) {
            return Err(format!("hidden member in place {name}"));
        }
    }
    for (name, node) in presence["parameters"].as_object().unwrap() {
        if hidden(&node["body"]) {
            return Err(format!("hidden member in parameter {name}"));
        }
    }
    let join = &vectors["model_join"];
    if hidden(&join["node"]["body"]) {
        return Err("hidden member in the object_type node".to_owned());
    }
    for declared in ["declared_without_v", "declared_with_v"] {
        for field in join[declared]["fields"].as_array().unwrap() {
            let ty = field["typeRef"].as_str().unwrap();
            if field["name"] == "presence" || ty.contains("Boolean") && field["presence"] == "optional" {
                return Err(format!("hidden field {} in {declared}", field["name"]));
            }
        }
    }
    let names = |declared: &str| -> Vec<String> {
        join[declared]["fields"].as_array().unwrap().iter().map(|f| f["name"].as_str().unwrap().to_owned()).collect()
    };
    let (without, with) = (names("declared_without_v"), names("declared_with_v"));
    if with.len() != without.len() + 1 || with[..without.len()] != without[..] || with.last().map(String::as_str) != Some("v") {
        return Err("declared members are not exactly the baseline plus v".to_owned());
    }
    Ok(())
}

/// The `object_type` node is one node with and without `v`, and the `v`
/// place's resolved row joins the declared field.
fn presence_model_join(vectors: &Value, nodes: &[Value]) -> Result<(), String> {
    let join = &vectors["model_join"];
    let model = &join["node"];
    if !structural_node_key_is_current(model) {
        return Err("object_type key is stale".to_owned());
    }
    if !nodes.iter().any(|n| n == model) {
        return Err("object_type node is not the published node".to_owned());
    }
    if model["body"] != json!({"members": [], "term": "aggregate"}) {
        return Err("object_type body is not empty".to_owned());
    }
    let owner = join["declared_with_v"]["identity"].as_str().unwrap();
    if join["declared_without_v"]["identity"] != owner || model["owner"]["node"] != owner {
        return Err("declarations and node disagree on the owner".to_owned());
    }
    let field = join["declared_with_v"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "v")
        .ok_or("no field v")?;
    let place = &vectors["presence"]["places"]["v"]["node"];
    if place["body"]["members"][0]["value"]["target"] != model["node_id"] {
        return Err("the v place is not owned by the object_type node".to_owned());
    }
    let row = &place["body"]["members"][2]["value"]["members"][0]["members"];
    let text = |at: usize| row[at]["value"]["value"].as_str().unwrap();
    let declared = field["typeRef"].as_str().unwrap();
    let rendered = if field["presence"] == "optional" { format!("Option<{declared}>") } else { declared.to_owned() };
    if text(1) != owner || text(2) != field["identity"] || text(3) != rendered {
        return Err("the resolved row does not join the declared field".to_owned());
    }
    Ok(())
}

/// The owner/slot parameter types resolve to nodes the vectors name.
fn presence_type_nodes(vectors: &Value, nodes: &[Value]) -> Result<(), String> {
    let digest = |value: &Value| value["digest"].as_str().map(str::to_owned);
    let reference = vectors["type_nodes"]["owner_reference_type"].as_str().ok_or("no owner type")?;
    let node = nodes
        .iter()
        .find(|n| n["node_id"]["digest"] == reference && n["node_tag"] == "composite_type" && n["semantic_form"] == "reference")
        .ok_or("the owner reference type is not a published reference type node")?;
    if node["body"]["members"][0]["target"] != vectors["model_join"]["node"]["node_id"] {
        return Err("the owner reference type does not name the object_type node".to_owned());
    }
    let integer = scalar_type_node(nodes, "integer");
    for (name, want) in [("self", reference), ("other", reference), ("i", integer), ("e", integer)] {
        let ty = digest(&vectors["presence"]["parameters"][name]["semantic_type"]);
        if ty.as_deref() != Some(want) {
            return Err(format!("parameter {name} has the wrong type"));
        }
    }
    Ok(())
}

#[trace("TC-367", "FR-322-AC-155", "FR-322-AC-156", "FR-322-AC-157")]
#[test]
fn tc_367_presence_over_a_clock_place_is_a_derived_predicate_not_a_field() {
    let vectors: Value = serde_json::from_str(TIMED_SLOT_SELECTOR_VECTORS).expect("vectors JSON");
    let package: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).expect("fixture JSON");
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let schema = parsed_fixture(SCHEMA);
    let place_schema = schema_for_def(&schema, "TimedModelNode");
    let expression_schema = schema_for_def(&schema, "ExpressionNode");
    // The hidden-member scan runs first and on its own, before any key check.
    assert_eq!(presence_hidden_member(&vectors), Ok(()));
    assert_eq!(presence_model_join(&vectors, nodes), Ok(()));
    assert_eq!(presence_type_nodes(&vectors, nodes), Ok(()));
    let presence = &vectors["presence"];
    let mut places = BTreeMap::new();
    for (name, place) in presence["places"].as_object().unwrap() {
        let node = &place["node"];
        assert!(structural_node_key_is_current(node), "place {name} key");
        assert!(place_schema.is_valid(node), "place {name} schema");
        assert!(place_selectors_are_symbolic(node, nodes), "place {name} selectors");
        places.insert(node["node_id"]["digest"].as_str().unwrap().to_owned(), node.clone());
    }
    let mut parameters = BTreeMap::new();
    for (name, node) in presence["parameters"].as_object().unwrap() {
        assert!(structural_node_key_is_current(node), "parameter {name} key");
        parameters.insert(node["node_id"]["digest"].as_str().unwrap().to_owned(), node.clone());
    }
    let self_node = nodes.iter().find(|n| n["node_id"] == presence["parameters"]["self"]["node_id"]).unwrap();
    assert_eq!(*self_node, presence["parameters"]["self"], "self is the published parameter");
    let owner_type = self_node["semantic_type"].clone();
    let slot_type = nodes.iter().find(|n| n["node_tag"] == "scalar_type" && n["semantic_form"] == "integer").unwrap()["node_id"].clone();
    let boolean = nodes.iter().find(|n| n["node_tag"] == "scalar_type" && n["semantic_form"] == "boolean").unwrap()["node_id"].clone();
    let mut keys = BTreeSet::new();
    for case in presence["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let node = &case["node"];
        assert_eq!(
            node["node_id"]["digest"],
            unqualified_jcs_sha256(&application_node_preimage(node, &[]).expect("application node")),
            "{id} key hashes its application preimage"
        );
        assert!(application_node_shape_is_valid(node), "{id} shape");
        assert!(expression_schema.is_valid(node), "{id} expression schema");
        assert_eq!(node["semantic_form"], "presence_read");
        assert_eq!(node["semantic_type"], boolean, "{id} is Boolean");
        assert_eq!(node["body"]["operation"]["identity"], "quire.op.option.present");
        assert!(keys.insert(node["node_id"]["digest"].as_str().unwrap().to_owned()), "{id} key is distinct");
        let cause = presence_cause(node, &places, &parameters, &owner_type, &slot_type);
        match case["expected"].as_str().unwrap() {
            "admitted" => assert_eq!(cause, None, "{id}"),
            _ => {
                let (code, why, at) = cause.unwrap_or_else(|| panic!("{id} is refused"));
                assert_eq!((code, why), (case["expected_code"].as_str().unwrap(), case["expected_cause"].as_str().unwrap()), "{id}");
                assert!(at.ends_with(case["locus_suffix"].as_str().unwrap().trim_start_matches("body")), "{id} {at}");
            }
        }
    }
}

/// Each assertion of the presence test fails on its own mutation.
#[trace("TC-367", "FR-322-AC-155", "FR-322-AC-156", "FR-322-AC-157")]
#[test]
fn tc_367_presence_vector_assertions_each_fail_on_their_mutation() {
    let vectors: Value = serde_json::from_str(TIMED_SLOT_SELECTOR_VECTORS).unwrap();
    let package: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).unwrap();
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let mutate = |edit: &dyn Fn(&mut Value)| {
        let mut copy = vectors.clone();
        edit(&mut copy);
        copy
    };
    let hidden = |v: &Value| presence_hidden_member(v).expect_err("a hidden member is caught");
    assert!(hidden(&mutate(&|v| {
        v["presence"]["parameters"]["other"]["body"]["members"].as_array_mut().unwrap()
            .push(json!({"name": "presence", "term": "binding", "value": {"term": "literal"}}));
    })).contains("parameter other"));
    assert!(hidden(&mutate(&|v| {
        v["presence"]["places"]["v"]["node"]["body"]["members"].as_array_mut().unwrap()
            .push(json!({"name": "presence", "term": "binding", "value": {"term": "literal"}}));
    })).contains("place v"));
    assert!(hidden(&mutate(&|v| {
        v["model_join"]["node"]["body"]["members"].as_array_mut().unwrap()
            .push(json!({"name": "flag", "value": "Option<Boolean>"}));
    })).contains("object_type"));
    assert!(hidden(&mutate(&|v| {
        v["model_join"]["declared_with_v"]["fields"].as_array_mut().unwrap()
            .push(json!({"name": "v_present", "presence": "optional", "typeRef": "ix://quire/native/Boolean", "identity": "x", "multiplicity": {}}));
    })).contains("hidden field"));
    assert!(hidden(&mutate(&|v| {
        v["model_join"]["declared_with_v"]["fields"].as_array_mut().unwrap().pop();
    })).contains("not exactly"));

    let join = |v: &Value| presence_model_join(v, nodes).expect_err("the join is caught");
    assert!(join(&mutate(&|v| v["model_join"]["node"]["owner"]["node"] = json!("ix://acme/orders/Other"))).contains("key is stale"));
    assert!(join(&mutate(&|v| v["presence"]["places"]["v"]["node"]["body"]["members"][2]["value"]["members"][0]["members"][2]["value"]["value"] = json!("ix://acme/orders/Order/w"))).contains("does not join"));
    assert!(join(&mutate(&|v| v["model_join"]["declared_with_v"]["fields"][1]["presence"] = json!("required"))).contains("does not join"));
    assert!(join(&mutate(&|v| v["model_join"]["declared_with_v"]["identity"] = json!("ix://acme/orders/Other"))).contains("disagree"));
    assert!(join(&mutate(&|v| v["presence"]["places"]["v"]["node"]["body"]["members"][0]["value"]["target"]["digest"] = json!("00".repeat(32)))).contains("not owned"));

    let types = |v: &Value| presence_type_nodes(v, nodes).expect_err("the type join is caught");
    assert!(types(&mutate(&|v| v["type_nodes"]["owner_reference_type"] = json!("00".repeat(32)))).contains("not a published"));
    assert!(types(&mutate(&|v| v["presence"]["parameters"]["i"]["semantic_type"]["digest"] = json!("00".repeat(32)))).contains("parameter i"));

    // The oracle refuses a presence_read with an argument dropped.
    let places: BTreeMap<String, Value> = vectors["presence"]["places"].as_object().unwrap().values()
        .map(|p| (p["node"]["node_id"]["digest"].as_str().unwrap().to_owned(), p["node"].clone())).collect();
    let parameters: BTreeMap<String, Value> = vectors["presence"]["parameters"].as_object().unwrap().values()
        .map(|p| (p["node_id"]["digest"].as_str().unwrap().to_owned(), p.clone())).collect();
    let owner_type = vectors["presence"]["parameters"]["self"]["semantic_type"].clone();
    let slot_type = json!({"domain": SEMANTIC_NODE_DOMAIN, "digest": vectors["type_nodes"]["integer"]});
    let mut pr02 = vectors["presence"]["cases"][1]["node"].clone();
    assert_eq!(presence_cause(&pr02, &places, &parameters, &owner_type, &slot_type), None);
    pr02["body"]["arguments"].as_array_mut().unwrap().pop();
    assert!(presence_cause(&pr02, &places, &parameters, &owner_type, &slot_type).is_some());
}
