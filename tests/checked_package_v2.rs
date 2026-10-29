// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! I04 checked-package V2 and complete-value qualification vectors.

use ix_trace_rs::trace;
use num_bigint::BigInt;
use num_integer::Integer;
use quire_specification::{
    COMPLETE_VALUE_SELECTION_VECTORS, NATIVE_DIAGNOSTICS,
};
use quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
/// The catalog lives only in `quire-verification-contracts`; it is read from there.
const OPERATION_CATALOG: &str = CHECKED_OPERATION_CATALOG_V1;
const POSITIVE_OPERATION_IDENTITIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-operation-identities.json");
const POSITIVE_ALL_FAMILIES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-all-families.json");
const POSITIVE_TIMED_MODEL_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-timed-model-nodes.json");
const POSITIVE_TICK_TIMED_MODEL_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-tick-timed-model-nodes.json");
const TIMED_SLOT_SELECTOR_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/timed-slot-selector-vectors.json");
const POSITIVE_TIMED_SLOT_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-timed-slot-nodes.json");
const POSITIVE_TIMED_CLAUSE_NODES: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-timed-clause-nodes.json");
const SELECTED_TIMED_CLAUSE_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-timed-clauses.json");
const TIMED_CLAUSE_NODE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/timed-clause-node-vectors.json");
const DEPENDENCY_SELECTION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/dependency-selection-vectors.json");
const TIMED_MODEL_NODE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/timed-model-node-vectors.json");
const OPTION_VALUE_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/option-value-vectors.json");
const SELECTED_TIMED_SLOT_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-timed-slots.json");
const POSITIVE_COMPOUND_UNIT: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-compound-unit.json");
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

/// Whether `value` is a structural `NodeId`: exactly `package`, `path`, `ordinal`.
fn is_node_id(value: &Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == 3
            && object.contains_key("package")
            && object.contains_key("path")
            && object.contains_key("ordinal")
    })
}

/// The ascending node-id order, compared componentwise: the `package` bytes,
/// then each `path` element (a string sorts before a nested node id, nested
/// ids compare recursively, a shorter prefix sorts first), then the `ordinal`.
fn node_id_order(left: &Value, right: &Value) -> std::cmp::Ordering {
    let package = left["package"]
        .as_str()
        .map(str::as_bytes)
        .cmp(&right["package"].as_str().map(str::as_bytes));
    let empty = Vec::new();
    let left_path = left["path"].as_array().unwrap_or(&empty);
    let right_path = right["path"].as_array().unwrap_or(&empty);
    package
        .then_with(|| {
            left_path
                .iter()
                .zip(right_path)
                .map(|(left, right)| match (left.as_str(), right.as_str()) {
                    (Some(left), Some(right)) => left.as_bytes().cmp(right.as_bytes()),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => node_id_order(left, right),
                })
                .find(|order| order.is_ne())
                .unwrap_or_else(|| left_path.len().cmp(&right_path.len()))
        })
        .then_with(|| left["ordinal"].as_u64().cmp(&right["ordinal"].as_u64()))
}

/// A node id that sorts by [`node_id_order`], so a collection of ids orders and
/// compares componentwise.
#[derive(Clone, Debug, PartialEq, Eq)]
struct OrderedId(Value);

impl Ord for OrderedId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        node_id_order(&self.0, &other.0)
    }
}

impl PartialOrd for OrderedId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The node of `nodes` whose `node_id` equals `id`.
fn node_by_id<'a>(nodes: &'a [Value], id: &Value) -> Option<&'a Value> {
    nodes.iter().find(|node| node["node_id"] == *id)
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

/// Closed package-selection refusal vocabulary of the complete-value package selection.
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

/// Per-package selection slots read from the package-selection vectors.
struct PackageSelectionRules<'a> {
    trigger_vocabulary: BTreeSet<&'a str>,
    always: Vec<&'a str>,
    conditional: Vec<(&'a str, &'a str)>,
    exactly_one: Vec<(&'a str, &'a str, Vec<&'a str>)>,
}

impl<'a> PackageSelectionRules<'a> {
    fn from_vectors(lock: &'a Value) -> Self {
        let text = |value: &'a Value, what: &str| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("{what} is a string"))
        };
        Self {
            trigger_vocabulary: string_set(&lock["trigger_vocabulary"], "trigger vocabulary"),
            always: string_list(&lock["always_roles"], "always roles"),
            conditional: lock["conditional"]
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
            exactly_one: lock["exactly_one"]
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
/// body contains an application, plus every `value`/`union_value` or
/// `value`/`option_value` node, including nullary union constructions.
fn is_application_node(node: &Value) -> bool {
    !application_terms(&node["body"]).is_empty()
        || (node["node_tag"] == "value"
            && matches!(node["semantic_form"].as_str(), Some("union_value" | "option_value")))
}

/// Exact dependency list of an application-bearing node: every reference
/// target, every operation member declaration and every `reaches` member
/// `population` node of its body, plus the optional protocol-clause owner
/// model edge, unique and ascending by node id.
fn application_dependencies(term: &Value) -> Vec<Value> {
    let mut targets: Vec<&Value> = Vec::new();
    let mut pending = vec![term];
    while let Some(current) = pending.pop() {
        match current["term"].as_str() {
            Some("reference") => {
                if is_node_id(&current["target"]) {
                    targets.push(&current["target"]);
                }
            }
            Some("application") => {
                let member = &current["operation"]["member"];
                for link in [&member["declaration"], &member["population"]] {
                    if is_node_id(link) {
                        targets.push(link);
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
    targets.sort_by(|left, right| node_id_order(left, right));
    targets.dedup();
    targets.into_iter().cloned().collect()
}

/// Exact dependencies of a checked application node, including its
/// identity-bound protocol-clause owner edge when present.
fn application_node_dependencies(node: &Value) -> Vec<Value> {
    let mut targets = application_dependencies(&node["body"]);
    if let Some(owner) = node.get("owner_model") {
        targets.push(owner.clone());
    }
    targets.sort_by(node_id_order);
    targets.dedup();
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
    /// The admitted document of each `model_root` node.
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
            documents: package["semantic_graph"]["nodes"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|node| node["owner"]["kind"] == "model")
                .filter_map(|node| node["owner"]["identity"].as_str())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .filter_map(|identity| selected_model_document(package, identity))
                .collect(),
        }
    }

    /// The node an id names: a node of the importing graph, or an imported
    /// node of a bound dependency package, which keeps its own `package`.
    fn node(&self, id: &Value) -> Option<&'a Value> {
        self.nodes
            .iter()
            .chain(self.dependencies.iter().flat_map(|(_, dependency)| {
                dependency["semantic_graph"]["nodes"]
                    .as_array()
                    .into_iter()
                    .flatten()
            }))
            .find(|node| node["node_id"] == *id)
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
                ("scalar_type", "enum") => "enum",
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

    /// FR-322 present-option reader join: `semantic_type` is exactly an
    /// option node and its one `value` reference names a checked value of the
    /// option's element type. The locus is the first member that fails.
    fn option_value_refusal(&self, node: &Value) -> Option<(&'static str, Option<&'static str>, &'static str)> {
        if node["node_tag"] != "value" || node["semantic_form"] != "option_value" {
            return None;
        }
        let Some(semantic_type) = self.node(&node["semantic_type"]) else {
            return Some(("invalid_semantic_graph", None, "semantic_type"));
        };
        if semantic_type["node_tag"] != "composite_type"
            || semantic_type["semantic_form"] != "option"
        {
            return Some(("invalid_semantic_graph", None, "semantic_type"));
        }
        let element_type = self.inner_type(&node["semantic_type"])?;
        let payload = Self::binding(node, "value")?;
        let Some(payload_node) = self.node(&payload["target"]) else {
            return Some(("ill_typed", Some("type-mismatch"), "body/members/0/value"));
        };
        if payload_node["node_tag"] != "value" {
            return Some(("ill_typed", Some("type-mismatch"), "body/members/0/value"));
        }
        let payload_type = self.descriptor(payload)?.ty?;
        (!self.same_type(&payload_type, element_type))
            .then_some(("ill_typed", Some("type-mismatch"), "body/members/0/value"))
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
            // and the named dependency function's result type, the imported
            // node's own id (an imported node keeps its own package name).
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
                    let dimension = &unit["semantic_type"];
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

/// The finite-global profile's operators, as the protocol-node vectors list them.
fn protocol_operators() -> Vec<String> {
    parsed_fixture(PROTOCOL_NODE_VECTORS)["operators"]
        .as_array()
        .expect("protocol operator vectors are an array")
        .iter()
        .map(|entry| entry["operator"].as_str().expect("operator is text").to_owned())
        .collect()
}

/// The finite-global profile's one `state`/`transition` operator.
fn state_transition_operators() -> Vec<String> {
    vec!["transition".to_owned()]
}

/// FR-464-AC-1: every protocol operator of the vectors has exactly one argument
/// rule in the schema, laying out its arguments in the vector's order.
#[trace("TC-946", "FR-464-AC-1")]
#[test]
fn protocol_node_vectors_match_the_schema_argument_layouts() {
    let vectors = parsed_fixture(PROTOCOL_NODE_VECTORS);
    let actual = protocol_operators();
    assert_eq!(actual.len(), 20);
    let schema = parsed_fixture(SCHEMA);
    let rules = schema["$defs"]["ProtocolControlApplication"]["allOf"]
        .as_array()
        .expect("protocol application argument rules are an array");
    assert_eq!(rules.len(), actual.len());
    for vector in vectors["operators"].as_array().unwrap() {
        let operator = vector["operator"].as_str().unwrap();
        assert!(!vector["semantic_form"].as_str().unwrap().is_empty());
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
        assert!(!names.is_empty());
        assert_eq!(names, expected_names, "{operator} schema argument layout");
        assert_eq!(rule["then"]["properties"]["arguments"]["minItems"], names.len());
        assert_eq!(rule["then"]["properties"]["arguments"]["maxItems"], names.len());
    }
}

/// A guard term over the independent atoms of FR-464 "Nested `repeat` rule".
#[derive(Clone)]
enum RepeatGuard {
    Const(bool),
    Atom(&'static str),
    Not(Box<RepeatGuard>),
    And(Box<RepeatGuard>, Box<RepeatGuard>),
    Or(Box<RepeatGuard>, Box<RepeatGuard>),
}

impl RepeatGuard {
    fn collect(&self, found: &mut Vec<&'static str>, constants_are_atoms: bool) {
        let mut add = |name: &'static str| {
            if !found.contains(&name) {
                found.push(name);
            }
        };
        match self {
            Self::Const(value) => {
                if constants_are_atoms {
                    add(if *value { "true" } else { "false" });
                }
            }
            Self::Atom(name) => add(name),
            Self::Not(inner) => inner.collect(found, constants_are_atoms),
            Self::And(left, right) | Self::Or(left, right) => {
                // Last operand first, as FR-464 orders the unlisted atoms.
                right.collect(found, constants_are_atoms);
                left.collect(found, constants_are_atoms);
            }
        }
    }

    fn eval(&self, atoms: &[&'static str], valuation: u32, constants_are_atoms: bool) -> bool {
        let at = |name: &str| {
            let bit = atoms.iter().position(|atom| *atom == name).expect("atom");
            valuation >> bit & 1 == 1
        };
        match self {
            Self::Const(value) if constants_are_atoms => at(if *value { "true" } else { "false" }),
            Self::Const(value) => *value,
            Self::Atom(name) => at(name),
            Self::Not(inner) => !inner.eval(atoms, valuation, constants_are_atoms),
            Self::And(left, right) => {
                left.eval(atoms, valuation, constants_are_atoms)
                    && right.eval(atoms, valuation, constants_are_atoms)
            }
            Self::Or(left, right) => {
                left.eval(atoms, valuation, constants_are_atoms)
                    || right.eval(atoms, valuation, constants_are_atoms)
            }
        }
    }
}

/// A control for the progress model.
#[derive(Clone)]
enum Ctl {
    /// A step: `attempt`, `parallel`, `await` and the like.
    Step,
    Check,
    Sequence(Vec<Ctl>),
    Choice(Vec<Ctl>),
    Repeat(Box<Rep>),
}

#[derive(Clone)]
struct Rep {
    visible: Vec<&'static str>,
    guard: RepeatGuard,
    maximum: Option<u32>,
    body: Ctl,
    exhausted: Option<Ctl>,
}

/// The rule's clauses; each flip below is one mutant.
#[derive(Clone, Copy)]
struct NestedRule {
    /// Clause 2: `maximum` 0 contributes nothing.
    maximum_zero_blocks: bool,
    /// The valuation search stops at the first false valuation.
    stop_at_first_false: bool,
    /// Clause 1 reads the guard (not: every exit is feasible).
    reads_guard: bool,
    /// The `exhausted` control is never counted.
    exhausted_uncounted: bool,
    /// A Boolean constant is not an atom.
    constants_are_constants: bool,
    /// Atoms are enumerated in the listed order, not the reverse.
    listed_order: bool,
}

const NESTED_RULE: NestedRule = NestedRule {
    maximum_zero_blocks: true,
    stop_at_first_false: true,
    reads_guard: true,
    exhausted_uncounted: true,
    constants_are_constants: true,
    listed_order: true,
};

/// Whether `ctl` necessarily steps for an enclosing `repeat`, adding the work
/// units charged to `units`. `in_body` is true inside a `repeat`'s `body`.
fn model_steps(rule: NestedRule, ctl: &Ctl, in_body: bool, units: &mut u32) -> bool {
    match ctl {
        Ctl::Step => true,
        Ctl::Check => false,
        Ctl::Sequence(children) => children
            .iter()
            .map(|child| model_steps(rule, child, in_body, units))
            .collect::<Vec<_>>()
            .contains(&true),
        Ctl::Choice(children) => children
            .iter()
            .map(|child| model_steps(rule, child, in_body, units))
            .collect::<Vec<_>>()
            .iter()
            .all(|steps| *steps),
        Ctl::Repeat(rep) => {
            let body = model_steps(rule, &rep.body, true, units);
            let exhausted = rep
                .exhausted
                .as_ref()
                .is_some_and(|control| model_steps(rule, control, false, units));
            if !in_body {
                return false;
            }
            let mut atoms = rep.visible.clone();
            if !rule.listed_order {
                atoms.reverse();
            }
            rep.guard.collect(&mut atoms, !rule.constants_are_constants);
            let mut infeasible = rule.reads_guard;
            if rule.reads_guard {
                for valuation in 0..1_u32 << atoms.len() {
                    *units += 1;
                    if !rep
                        .guard
                        .eval(&atoms, valuation, !rule.constants_are_constants)
                    {
                        infeasible = false;
                        if rule.stop_at_first_false {
                            break;
                        }
                    }
                }
            }
            let blocked = rule.maximum_zero_blocks && rep.maximum == Some(0);
            let counted = body || (!rule.exhausted_uncounted && exhausted);
            infeasible && !blocked && counted
        }
    }
}

/// (the outer `repeat` is admitted, work units) of an outer `repeat` whose
/// body is `body`.
fn model_outer(rule: NestedRule, body: &Ctl) -> (bool, u32) {
    let mut units = 0;
    let admitted = model_steps(rule, body, true, &mut units);
    (admitted, units)
}

/// STD-539: an executable model of the nested `repeat` rule over the
/// controls, guards, maxima and expected outcomes and work units of TC-946
/// R13 to R20 and R22 to R24, so the vectors cannot drift from the rule;
/// every clause flip is a mutant that must disagree with at least one vector.
#[trace("TC-946", "FR-464-AC-21", "FR-464-AC-22", "FR-464-AC-23")]
#[test]
fn std_539_nested_repeat_rule_model_agrees_with_the_vectors() {
    use RepeatGuard::{And, Atom, Const, Not, Or};
    let a = || Box::new(Atom("a"));
    let b = || Box::new(Atom("b"));
    let eq_ab = || Box::new(Atom("eq(a, b)"));
    let eq_ba = || Box::new(Atom("eq(b, a)"));
    let a_or_not_a = || Or(a(), Box::new(Not(a())));
    let tautology = || {
        Or(
            Box::new(Or(Box::new(a_or_not_a()), b())),
            Box::new(Not(b())),
        )
    };
    let nested = |visible: &[&'static str],
                  guard: RepeatGuard,
                  maximum: Option<u32>,
                  body: Ctl,
                  exhausted: Option<Ctl>| {
        Ctl::Repeat(Box::new(Rep {
            visible: visible.to_vec(),
            guard,
            maximum,
            body,
            exhausted,
        }))
    };
    let simple = |guard: RepeatGuard, visible: &[&'static str], maximum: Option<u32>| {
        let exhausted = maximum.map(|_| Ctl::Check);
        nested(visible, guard, maximum, Ctl::Step, exhausted)
    };
    // (vector id, text the vector's Edit holds, the outer body, outer
    // admitted, work units).
    let mut cases: Vec<(&str, &str, Ctl, bool, u32)> = vec![
        ("R13", "guard is `true`, no `maximum`", simple(Const(true), &[], None), true, 1),
        (
            "R14",
            "`a or not a or b or not b`",
            simple(tautology(), &["a", "b"], Some(1)),
            true,
            4,
        ),
        ("R15", "guard is `true`;", simple(Const(true), &[], Some(0)), false, 1),
        (
            "R15",
            "`a or not a`",
            simple(a_or_not_a(), &["a"], Some(0)),
            false,
            2,
        ),
        (
            "R18",
            "own body is only a `check`;",
            nested(&[], Const(true), None, Ctl::Check, None),
            false,
            1,
        ),
        (
            "R18",
            "an `exhausted` `attempt`",
            nested(&[], Const(true), Some(1), Ctl::Check, Some(Ctl::Step)),
            false,
            1,
        ),
        (
            "R19",
            "guard being `true`",
            Ctl::Choice(vec![simple(Const(true), &[], None), Ctl::Step]),
            true,
            1,
        ),
        (
            "R19",
            "nested guard `a`",
            Ctl::Choice(vec![simple(Atom("a"), &["a"], None), Ctl::Step]),
            false,
            1,
        ),
        (
            "R20",
            "`quire.op.boolean.eq(a, b) or not quire.op.boolean.eq(a, b)`",
            simple(Or(eq_ab(), Box::new(Not(eq_ab()))), &["eq(a, b)"], None),
            true,
            2,
        ),
        (
            "R20",
            "not quire.op.boolean.eq(b, a)",
            simple(
                Or(eq_ab(), Box::new(Not(eq_ba()))),
                &["eq(a, b)", "eq(b, a)"],
                None,
            ),
            false,
            3,
        ),
        (
            "R22",
            "`E or not a or not b`",
            simple(
                Or(
                    Box::new(Or(Box::new(Atom("E")), Box::new(Not(a())))),
                    Box::new(Not(b())),
                ),
                &["E", "a", "b"],
                None,
            ),
            false,
            7,
        ),
        ("R23", "reference` to a Boolean", simple(Const(true), &[], None), true, 1),
        (
            "R23",
            "`a or true`",
            simple(Or(a(), Box::new(Const(true))), &["a"], None),
            true,
            2,
        ),
        ("R24", "`repeat max 1`", simple(Const(true), &[], Some(1)), true, 1),
    ];
    for (guard, name, visible, units) in [
        (Atom("a"), "`a`;", ["a", "b"], 1),
        (Not(a()), "`not a`", ["a", "b"], 2),
        (And(a(), Box::new(Not(a()))), "`a and not a`", ["a", "b"], 1),
        (Const(false), "`false`", ["a", "b"], 1),
        (Or(a(), b()), "`a or b`", ["a", "b"], 1),
    ] {
        for maximum in [None, Some(1)] {
            cases.push(("R16", name, simple(guard.clone(), &visible, maximum), false, units));
        }
    }
    // R17: a nested repeat beside a step, in either order, whatever its guard.
    for body in [
        Ctl::Sequence(vec![simple(Atom("a"), &["a"], None), Ctl::Step]),
        Ctl::Sequence(vec![Ctl::Step, simple(Atom("a"), &["a"], None)]),
    ] {
        cases.push(("R17", "the nested guard is `a`", body, true, 1));
    }
    let vectors = parsed_fixture(PROTOCOL_NODE_VECTORS);
    let edit_of = |id: &str| {
        vectors["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["test_case"] == "TC-946" && case["id"] == id)
            .and_then(|case| case["Edit to the base"].as_str())
            .unwrap_or_default()
            .to_string()
    };
    let disagreements = |rule: NestedRule| {
        cases
            .iter()
            .filter(|(_, _, body, admitted, units)| model_outer(rule, body) != (*admitted, *units))
            .count()
    };
    for (id, text, ..) in &cases {
        assert!(
            edit_of(id).contains(text),
            "vector {id} names the modelled case {text:?}"
        );
    }
    assert_eq!(
        disagreements(NESTED_RULE),
        0,
        "the rule reproduces every vector"
    );
    for (name, rule) in [
        (
            "maximum 0 contributes",
            NestedRule { maximum_zero_blocks: false, ..NESTED_RULE },
        ),
        (
            "search does not stop",
            NestedRule { stop_at_first_false: false, ..NESTED_RULE },
        ),
        (
            "guard is not read",
            NestedRule { reads_guard: false, ..NESTED_RULE },
        ),
        (
            "exhausted counted",
            NestedRule { exhausted_uncounted: false, ..NESTED_RULE },
        ),
        (
            "constants are atoms",
            NestedRule { constants_are_constants: false, ..NESTED_RULE },
        ),
        (
            "atom order reversed",
            NestedRule { listed_order: false, ..NESTED_RULE },
        ),
    ] {
        assert!(
            disagreements(rule) > 0,
            "mutant {name} disagrees with a vector"
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

/// The first refusal of a package's applications: nodes ascending by node id,
/// then each node's applications in pre-order.
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
    nodes.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
    nodes.into_iter().try_for_each(|node| {
        if let Some(refusal) = oracle.union_value_refusal(node) {
            return Err(refusal);
        }
        application_terms(&node["body"])
            .into_iter()
            .try_for_each(|term| oracle.classify_in(node, term).map(drop))
    })
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

/// A model declaration node (QSL FR-094): no `declaration`, the form of an
/// object type, a process, a state machine, a persistence interface, a
/// systems interface, a relationship or a population.
fn is_model_declaration(node: &Value) -> bool {
    node.get("declaration").is_none()
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

/// The declarations of the admitted document of the `model_selections` row
/// selecting `identity`, by FR-322 "Model-owned members" step 1: the
/// caller-supplied document of the row's identity. The document
/// is the package's own `model_bindings.domain_package` when it carries one
/// (already in projection form), else the published `acme/orders` semantic IR
/// document, whose declarations are the `domain_package` projection the
/// vectors derive from it.
fn selected_model_document(package: &Value, identity: &str) -> Option<Value> {
    let selects = |_document: &Value| true;
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

/// First semantic refusal of a schema-valid V2 package in the normative
/// reader order: graph shape, then the owner joins, then declaration refusals,
/// then operation refusals. Nodes are visited ascending by node id.
#[derive(Clone, Debug, PartialEq)]
enum ReadRefusal {
    /// `invalid_semantic_graph` at the `node_id` of an application-bearing node whose shape is invalid.
    GraphShape(Value),
    /// `invalid_package`/`invalid-derived-shape` at the `node_id` of a derived
    /// shape (reference, population, compound unit) whose body is not well
    /// formed (FR-322 "Derived type forms").
    DerivedShape(Value),
    /// `invalid_model_binding`/`malformed-declaration` at `package_read`: a
    /// `populations` entry of the selected document that is no FCD population.
    /// The locus is the `model_root` node's `/semantic_graph/nodes/<i>/owner`.
    PopulationRecord(String),
    /// `missing_declaration`/`missing-selection` at a node whose `owner` the
    /// lock does not select or whose selected document does not declare it.
    OwnerSelection(Value),
    /// `invalid_package`/`invalid-value` at a `/source_map/{i}` entry whose
    /// region names a source other than its declared node's `SourceOwner`.
    InvalidOwnerValue(String),
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
    /// Option-value semantic validation: a closed semantic type or a payload
    /// whose checked type differs from the option element type.
    OptionValue(&'static str, Option<&'static str>, Value, &'static str),
}

/// Graph-shape constraints of an application-bearing node: an expression
/// whose root is an application has the operator's semantic form, and its
/// dependencies are exactly its body references and member declarations,
/// plus the optional protocol-clause owner-model edge.
fn application_node_shape_is_valid(node: &Value) -> bool {
    application_node_shape_locus(node).is_none()
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
                    let unit_id = &unit_target["node_id"];
                    let valid_root = unit_target["node_tag"] == "scalar_type"
                        && unit_target["semantic_form"] == "unit"
                        && (unit_target["semantic_type"] == unit_target["node_id"]
                            || unit_target["dependencies"] == json!([unit_target["semantic_type"]]));
                    let valid_exponent = exponent.is_some_and(|member| {
                        member["value"]["term"] == "literal"
                            && member["value"]["value_kind"] == "integer"
                            && integer(&member["value"]["value"])
                                .is_some_and(|value| value != BigInt::from(0))
                    });
                    let ordered = previous
                        .is_none_or(|last: &Value| node_id_order(last, unit_id).is_lt());
                    previous = Some(unit_id);
                    !valid_root || !valid_exponent || !ordered
                })
            }
            _ => false,
        },
    )
}

/// A derived `Reference<O>` target (QSL FR-322): a declared model form,
/// whether source-declared in this package or resolved through a `ModelOwner`.
fn is_model_reference_target(node: &Value) -> bool {
    matches!(
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
/// ascending by node id. A `SourceOwner` joins a
/// `lock.sources` entry (`missing-selection` at the node) and agrees with the
/// source of every region of the node's `declaration` occurrence
/// (`invalid-value` at that source-map entry); a `ModelOwner` joins a
/// selected document that declares its `node`
/// (`missing-selection` at the node).
fn owner_join_refusal(package: &Value, nodes: &[Value]) -> Result<(), ReadRefusal> {
    let mut owned = nodes
        .iter()
        .filter(|node| node.get("owner").is_some())
        .collect::<Vec<_>>();
    owned.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
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
            Some("definition") => {
                let joined = package["lock"]["definition_selections"]
                    .as_array()
                    .is_some_and(|selections| {
                        selections.iter().any(|selection| {
                            selection["authority"] == owner["authority"]
                                && selection["identity"] == owner["identity"]
                        })
                    });
                if !joined {
                    return Err(unjoined);
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
                let joined = owner["identity"]
                    .as_str()
                    .is_some_and(|identity| selected_model_document(package, identity).is_some());
                if !joined {
                    return Err(unjoined);
                }
            }
            _ => return Err(unjoined),
        }
    }
    Ok(())
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

/// The graph-shape, owner-join and derived-shape stages that precede every
/// declaration, frame, state and operation refusal.
fn shape_stages_refusal(package: &Value, nodes: &[Value]) -> Result<(), ReadRefusal> {
    let mut applications = nodes
        .iter()
        .filter(|node| is_application_node(node))
        .collect::<Vec<_>>();
    applications.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
    if let Some(node) = applications
        .iter()
        .find(|node| !application_node_shape_is_valid(node))
    {
        return Err(ReadRefusal::GraphShape(node["node_id"].clone()));
    }
    owner_join_refusal(package, nodes)?;
    if let Some(node) = derived_shape_semantic_refusal(nodes) {
        return Err(ReadRefusal::DerivedShape(node["node_id"].clone()));
    }
    Ok(())
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
        let document_identity = &package["model_bindings"]["domain_package"]["identity"];
        let root = package["semantic_graph"]["nodes"]
            .as_array()
            .and_then(|nodes| {
                nodes.iter().position(|node| {
                    node["semantic_form"] == "model_root"
                        && node["owner"]["identity"] == *document_identity
                })
            })
            .expect("the model_root node of the read document");
        return Err(ReadRefusal::PopulationRecord(format!(
            "/semantic_graph/nodes/{root}/owner"
        )));
    }
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    shape_stages_refusal(package, nodes)?;
    let oracle = OperationOracle::for_package_with(catalog, package, &dependencies);
    if let Some(node) = nodes.iter().find(|node| oracle.enum_value_is_refused(node)) {
        return Err(ReadRefusal::InvalidSemanticGraph(node["node_id"].clone()));
    }
    if let Some((node, (code, cause, member))) = nodes.iter().find_map(|node| {
        oracle
            .option_value_refusal(node)
            .map(|refusal| (node, refusal))
    }) {
        return Err(ReadRefusal::OptionValue(
            code,
            cause,
            node["node_id"].clone(),
            member,
        ));
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
/// supplied (`missing_import`/`missing-selection`), at the entry's pointer.
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
        bound.push((identity.clone(), dependency.clone()));
    }
    Ok(bound)
}

/// The node a `dependency_reference` names in the bound dependency package
/// whose `name` is the term's `package`, when there is one.
fn dependency_reference_target<'a>(
    dependencies: &'a [(String, Value)],
    term: &Value,
) -> Option<&'a Value> {
    dependencies
        .iter()
        .find(|(_, dependency)| dependency["name"] == term["package"])?
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
/// node id, at each term's place in its node's pre-order term walk:
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
    ordered.sort_by(|(_, left), (_, right)| node_id_order(&left["node_id"], &right["node_id"]));
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
                .any(|entry| entry["identity"] == term["package"])
            {
                return Err(ReadRefusal::Dependency(
                    "missing_declaration",
                    "missing-selection",
                    format!("{pointer}/package"),
                ));
            }
            let library = dependencies
                .iter()
                .find(|(_, dependency)| dependency["name"] == term["package"])
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
/// `correspondence`/`abstraction_relation` node, ascending by node id:
/// its `semantic_type` is its own id and its `dependencies` are exactly the
/// unique node-id-ascending `type`, `population` and `context` targets of its
/// body.
fn abstraction_identity_refusal(nodes: &[Value]) -> Result<(), Value> {
    let mut relations = nodes
        .iter()
        .filter(|node| {
            node["node_tag"] == "correspondence" && node["semantic_form"] == "abstraction_relation"
        })
        .collect::<Vec<_>>();
    relations.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
    for node in relations {
        let body = &node["body"];
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
        expected.sort_by(node_id_order);
        expected.dedup();
        let holds = node["semantic_type"] == node["node_id"]
            && node["dependencies"].as_array() == Some(&expected);
        if !holds {
            return Err(node["node_id"].clone());
        }
    }
    Ok(())
}

/// Semantic refusal of a package's `declaration` members, in the normative
/// order: ambiguity is reported at the first node (node id ascending) that repeats a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeclarationRefusal {
    AmbiguousName,
}

/// Independent oracle over the `declaration` members of a schema-valid V2
/// package; the refusal names the offending node.
fn package_declaration_refusal(package: &Value) -> Result<(), (DeclarationRefusal, Value)> {
    let mut nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
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

/// The entries of one frame member as (declaring node id, field name,
/// kind); a relationship or `creates`/`deletes` entry has the empty name.
fn frame_entries(
    frame_node: &Value,
    group: FrameGroup,
) -> Vec<(OrderedId, String, FrameEntryKind, u8)> {
    frame_node["body"][group.key()]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| match group {
            FrameGroup::Modifies => {
                let id = OrderedId(is_node_id(&entry["declaration"]).then(|| entry["declaration"].clone())?);
                match entry["kind"].as_str()? {
                    // The scope enters the order key, `every-object` first.
                    "field" => Some((
                        id,
                        entry["name"].as_str()?.to_owned(),
                        FrameEntryKind::Field,
                        u8::from(entry["scope"] == "receiver"),
                    )),
                    "relationship" => {
                        Some((id, String::new(), FrameEntryKind::Relationship, 0))
                    }
                    _ => None,
                }
            }
            _ => Some((
                OrderedId(is_node_id(entry).then(|| entry.clone())?),
                String::new(),
                FrameEntryKind::Instance,
                0,
            )),
        })
        .collect()
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
/// by the offending entry's declaring node id and then its field name,
/// never by its position in the array.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FrameDefect {
    MeaningJoin {
        group: FrameGroup,
        id: OrderedId,
        name: String,
        cause: FrameJoinCause,
    },
    /// Two field entries of one declaration and name with different scopes
    /// (`conflicting-binding`): a meaning-join defect that ranks after every
    /// other one and before any canonical-order defect, by its place here.
    ConflictingBinding {
        id: OrderedId,
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
/// id to the domain declaration it recovers. The published schema
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
/// checked-package node `id` recovers, by the declaration node the fixture
/// states for it.
fn mm_owner_node<'a>(declaration_nodes: &'a [Value], id: &Value) -> Option<&'a str> {
    node_by_id(declaration_nodes, id).and_then(|node| node["owner"]["node"].as_str())
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
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut defects = Vec::new();
    let semantic_type = &frame_node["semantic_type"];
    let declaring_type = node_by_id(nodes, semantic_type);
    if !declaring_type.is_some_and(|node| {
        node["node_tag"] == "model"
            && matches!(
                node["semantic_form"].as_str(),
                Some("object_type" | "process" | "state_machine")
            )
    }) {
        defects.push(FrameDefect::MeaningJoin {
            group: FrameGroup::SemanticType,
            id: OrderedId(semantic_type.clone()),
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
                id: OrderedId(entry["declaration"].clone()),
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
                id: OrderedId(frame_node["node_id"].clone()),
                name: String::new(),
                cause: FrameJoinCause::MalformedDeclaration,
            });
        }
    }
    for group in FrameGroup::MEMBERS {
        let entries = frame_entries(frame_node, group);
        let keys = entries
            .iter()
            .map(|(id, name, _, scope)| (id.clone(), name.clone(), *scope))
            .collect::<Vec<_>>();
        let mut scopes = BTreeMap::<(OrderedId, String), BTreeSet<u8>>::new();
        for (id, name, kind, scope) in &entries {
            if *kind == FrameEntryKind::Field {
                scopes
                    .entry((id.clone(), name.clone()))
                    .or_default()
                    .insert(*scope);
            }
        }
        for ((id, name), kinds) in scopes {
            if kinds.len() > 1 {
                defects.push(FrameDefect::ConflictingBinding { id, name });
            }
        }
        let mut ascending = keys.clone();
        ascending.sort();
        if ascending != keys {
            defects.push(FrameDefect::CanonicalOrder { group });
        }
        for (id, name, kind, _) in entries {
            let resolved = node_by_id(nodes, &id.0);
            let is_dependency = dependencies.contains(&id.0);
            let meaning_ok = resolved.is_some_and(|node| frame_entry_admits(kind, node));
            if is_dependency && meaning_ok {
                // FR-340's field entries name a declaring node AND a field
                // name (FR-340-AC-13); resolve the name too when a model is
                // available, so an entry naming a real object type but no
                // field of that name is still a meaning-join defect
                // (`missing-name`), not silently admitted.
                let name_ok = kind != FrameEntryKind::Field
                    || model.is_none_or(|model| {
                        mm_owner_node(model.declaration_nodes, &id.0).is_some_and(|node| {
                            mm_declared_member(model.domain_package, node, "fields", &name)
                                .is_some()
                        })
                    });
                if name_ok {
                    continue;
                }
                defects.push(FrameDefect::MeaningJoin {
                    group,
                    id,
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
                id,
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
            // `.cause()` is never called on it, and
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
/// position. Frame nodes are visited in ascending node-id order,
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
    frames.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
    for frame in frames {
        if let Some(defect) = reported_frame_defect(frame, nodes, model.as_ref()) {
            let locus = match &defect {
                FrameDefect::MeaningJoin { id, .. }
                | FrameDefect::ConflictingBinding { id, .. } => id.0.clone(),
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
    let is_dependency = node["dependencies"]
        .as_array()
        .is_some_and(|deps| deps.contains(target));
    let resolved = node_by_id(nodes, target)
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
        let owner_node = mm_owner_node(model.declaration_nodes, context);
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
        .and_then(|parameter| node_by_id(nodes, &parameter["semantic_type"]))
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
        let owner_node = mm_owner_node(model.declaration_nodes, &context);
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
/// each in ascending node-id order, report their first defect.
fn package_state_refusal(package: &Value) -> Result<(), (StateRefusal, Value)> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let model = model_index(package);
    let mut ordered = nodes.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
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
/// anchor node-id order; the second anchor to claim a pair is the
/// offending node.
fn anchor_uniqueness_refusal(anchors: &[&Value]) -> Result<(), (StateRefusal, Value)> {
    let mut pairs = BTreeSet::new();
    for anchor in anchors {
        let context = OrderedId(anchor_binding(anchor, "context")["target"].clone());
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

/// A structural id no package publishes, named by `label`, for hand-authored
/// frame vectors below.
fn absent_node_id(label: &str) -> Value {
    json!({"package": "example", "path": ["absent", label], "ordinal": 0})
}

/// Id of the one node of `package` with this tag and form, narrowed by the
/// last segment of its declared name when several share a form.
fn published_node_id(package: &Value, tag: &str, form: &str, name: Option<&str>) -> Value {
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
    hits[0]["node_id"].clone()
}

/// Id of a node the all-families fixture publishes, by its role there.
fn all_families_node_id(role: &str) -> Value {
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
    published_node_id(&package, tag, form, name)
}

/// Id of a node the clause-operations fixture publishes, by its role there.
fn clause_node_id(role: &str) -> Value {
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    if role == "boolean_literal" {
        return package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["node_tag"] == "value" && node["body"]["value_kind"] == "boolean")
            .expect("a boolean literal is published")["node_id"]
            .clone();
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
            .expect("the self binder is published")["node_id"]
            .clone();
    }
    if role == "reference" {
        // The anchors' context type; the protocol clause's `over` type references the Order owner.
        let context = clause_node_id("object_type");
        return package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| {
                node["node_tag"] == "composite_type"
                    && node["semantic_form"] == "reference"
                    && node["body"]["members"][0]["target"] == context
            })
            .expect("the context reference type is published")["node_id"]
            .clone();
    }
    if role == "object_type" {
        // The anchors' context; the selected acme/orders Order node is the clause's owner.
        return published_node_id(&package, "model", "object_type", Some("ConfigVersion"));
    }
    let (tag, form) = match role {
        "parameter" => ("value", "parameter"),
        "frame" => ("state", "frame"),
        "anchor" => ("state", "operation_anchor"),
        "reference" => ("composite_type", "reference"),
        "boolean" => ("scalar_type", "boolean"),
        other => panic!("unknown clause-operations role {other}"),
    };
    published_node_id(&package, tag, form, None)
}

/// A `modifies` entry granting every link of the relationship `id` names.
fn relationship_entry(id: &Value) -> Value {
    json!({"kind": "relationship", "declaration": id})
}

/// A `modifies` entry granting writes to field `name` of the declaring node
/// `id` names.
fn field_entry(id: &Value, name: &str) -> Value {
    json!({
        "kind": "field",
        "declaration": id,
        "name": name,
        "scope": "every-object",
    })
}

/// A hand-authored `state`/`frame` node: real `node_id` shape, the
/// all-families fixture's `Account` object type as its
/// `semantic_type`, a `generated` occurrence, and an explicit `dependencies`
/// list and body, built directly from ids so every vector below
/// is independent of any shared comparator or generator function.
fn authored_frame_node(
    dependencies: &[&Value],
    modifies: &[Value],
    creates: &[&Value],
    deletes: &[&Value],
) -> Value {
    json!({
        "node_id": absent_node_id("93"),
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "state",
        "semantic_form": "frame",
        "semantic_type": all_families_node_id("account"),
        "dependencies": dependencies,
        "occurrences": [{"role": "generated", "ordinal": 0}],
        "body": {
            "term": "frame",
            "modifies": modifies,
            "creates": creates,
            "deletes": deletes,
            "deletes_scoped": [],
        },
    })
}

/// A meaning-join defect with the empty field name of a relationship,
/// `creates`/`deletes` or `semantic_type` position.
fn join_defect(group: FrameGroup, id: &Value, cause: FrameJoinCause) -> FrameDefect {
    FrameDefect::MeaningJoin {
        group,
        id: OrderedId(id.clone()),
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
    let relationship = all_families_node_id("relationship");
    let object_type_node = all_families_node_id("account");
    let process_node = all_families_node_id("process");
    let literal = all_families_node_id("literal");
    let absent = absent_node_id("ff");

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
            id: OrderedId(object_type_node.clone()),
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
    let event_node = all_families_node_id("event");
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
            id: OrderedId(event_node.clone()),
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
    wrong_type["semantic_type"] = event_node.clone();
    assert_eq!(
        frame_body_defects(&wrong_type, &nodes, None),
        vec![join_defect(
            FrameGroup::SemanticType,
            &event_node,
            FrameJoinCause::MalformedDeclaration
        )]
    );
    let mut process_type = authored_frame_node(&[], &[], &[], &[]);
    process_type["semantic_type"] = process_node.clone();
    assert_eq!(frame_body_defects(&process_type, &nodes, None), Vec::new());

    // A state machine declares fields but has no object universe, so it is a
    // field entry's declaring node and never a `creates` or `deletes` entry.
    let mut machine_nodes = nodes.clone();
    for node in &mut machine_nodes {
        if node["node_id"] == process_node {
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
    valid_entries.sort_by(|a, b| node_id_order(&a.0, &b.0));
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
    let account = all_families_node_id("account");
    let address = all_families_node_id("address");
    let mut entries = [
        (account.clone(), field_entry(&account, "address")),
        (address.clone(), field_entry(&address, "street")),
    ];
    entries.sort_by(|a, b| node_id_order(&a.0, &b.0));
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
    let relationship = all_families_node_id("relationship");

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

/// FR-340-AC-4: each member array is sorted ascending by entry node id and
/// duplicate-free. Duplicates are refused by the schema's `uniqueItems`
/// before this oracle runs; a misordered array is refused here, following
/// FR-322-AC-10's `invalid_semantic_graph` pattern for noncanonical order.
#[trace("TC-233", "FR-340-AC-4")]
#[test]
fn tc_233_frame_member_arrays_are_sorted_ascending_by_entry_id() {
    let nodes = parsed_fixture(POSITIVE_ALL_FAMILIES)["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .clone();
    let object_type_node = all_families_node_id("account");
    let process_node = all_families_node_id("process");
    let (low, high) = if node_id_order(&object_type_node, &process_node).is_lt() {
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
    // bytes, after the declaring node id.
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
    let relationship = all_families_node_id("relationship");
    let object_type_node = all_families_node_id("account");
    let process_node = all_families_node_id("process");

    // Vector A: modifies is misordered (canonical-order defect); deletes
    // names a relationship, wrong meaning for deletes (meaning-join defect).
    let mut descending = [
        (
            object_type_node.clone(),
            field_entry(&object_type_node, "balance"),
        ),
        (relationship.clone(), relationship_entry(&relationship)),
    ];
    descending.sort_by(|a, b| node_id_order(&b.0, &a.0));
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
        &if node_id_order(&process_node, &object_type_node).is_gt() {
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

/// FR-340-AC-6, tightened from "the same kind of diagnostic" to "the same
/// diagnostic": authoring the identical pair of meaning-join defects in two
/// different collection orders sorts to the identical minimum element, not
/// merely an element of the same variant. This is a direct
/// comparator-invariance check on the `FrameDefect` collection itself,
/// distinct from the group-swap check above: two real defects inside one
/// authored frame body are already canonically ordered by node id within
/// their own member array (AC-4), so a re-authored member array can never
/// exercise collection-order independence — only a direct `Vec<FrameDefect>`
/// built in two different orders can.
#[trace("TC-233", "FR-340-AC-6")]
#[test]
fn tc_233_frame_defect_ordering_does_not_depend_on_collection_order() {
    let mut ids = [
        all_families_node_id("account"),
        all_families_node_id("process"),
    ];
    ids.sort_by(node_id_order);
    let [lower, higher] = ids;
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

    // Two field entries of one declaring node tie on the node id and break
    // by field name.
    let named = |name: &str| FrameDefect::MeaningJoin {
        group: FrameGroup::Modifies,
        id: OrderedId(higher.clone()),
        name: name.to_owned(),
        cause: FrameJoinCause::MissingName,
    };
    let mut by_name = vec![named("total"), named("balance")];
    by_name.sort();
    assert_eq!(by_name, vec![named("balance"), named("total")]);

    // Each vector starts in a different collection order; sorting either one
    // must reach the identical canonical (lower-id-first) order, not
    // merely reach whatever order the other one reaches -- comparing the two
    // sorted vectors to each other, rather than to this fixed `expected`,
    // would hold trivially for any total order and could never catch a
    // reversed or dropped sort.
    let mut forward = vec![first.clone(), second.clone()];
    forward.sort();
    assert_eq!(
        forward, expected,
        "collecting the lower id first still sorts ascending by node id"
    );
    let mut backward = vec![second, first];
    backward.sort();
    assert_eq!(
        backward, expected,
        "collecting the higher id first still sorts to the same ascending order"
    );
}

/// Append a structural node to a fixture. The caller supplies the complete wire body and cross-fields;
/// this helper is used only by reader qualification vectors.
fn append_structural_node(package: &mut Value, mut node: Value) {
    let self_typed = node["semantic_type"] == node["node_id"]
        || matches!(
            node["semantic_form"].as_str(),
            Some("compound_unit" | "unit")
        );
    // A fresh structural id: the declared name for a declared node, else the
    // tag and form with a marker element, and the next free ordinal.
    let path = if node.get("declaration").is_some() {
        node["declaration"]["qualified_name"].clone()
    } else {
        json!([node["node_tag"], node["semantic_form"], "appended"])
    };
    let package_name = package["name"].clone();
    let ordinal = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture nodes are an array")
        .iter()
        .filter(|existing| {
            existing["node_id"]["path"] == path && existing["node_id"]["package"] == package_name
        })
        .count();
    node["node_id"] = json!({"package": package_name, "path": path, "ordinal": ordinal});
    if self_typed {
        node["semantic_type"] = node["node_id"].clone();
    }
    package["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("fixture nodes are an array")
        .push(node);
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
                OrderedId(entry["node_id"].clone()),
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
                    OrderedId(node["node_id"].clone()),
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
/// published schema already admits it as an ordinary declared `model` node.
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
/// admitted package; the importer selects it by its identity, calls its
/// `Int[0, 9] -> Boolean` function through a `dependency_reference` callee
/// with the Boolean node as `result_type` and no `dependencies` entry for the
/// target, and is admitted when the library is supplied. Each published
/// mutation keeps the V2 schema and is refused with exactly its recorded
/// code, cause and locus.
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
    assert_eq!(
        base["lock"]["dependency_selections"],
        json!([{"identity": identity}]),
        "the importer selects the library by its identity"
    );
    let call = &base["semantic_graph"]["nodes"][declared_index(&base, &["App", "check"])];
    assert_eq!(vectors["call"], call["declaration"]["qualified_name"]);
    let callee = &call["body"]["arguments"][0];
    assert_eq!(callee["term"], "dependency_reference");
    assert_eq!(callee["package"], library["name"]);
    assert_eq!(callee["node"], function["node_id"]);
    assert_eq!(
        call["body"]["result_type"]["path"],
        function["semantic_type"]["path"],
        "the call's result is the function's declared result type"
    );
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
            .any(|node| node["node_id"]["path"] == function["semantic_type"]["path"]),
        "the importer carries the result type node"
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

#[trace("TC-233", "FR-201-AC-9")]
#[test]
fn tc_233_package_selection_vectors_admit_exact_conditional_and_alternative_profiles() {
    let vectors = parsed_fixture(COMPLETE_VALUE_SELECTION_VECTORS);
    let selection = PackageSelectionRules::from_vectors(&vectors);
    let always = selection.always.clone();

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

/// Replace one application-bearing node of a package and recompute its exact
/// dependencies.
fn replace_application_node(package: &mut Value, index: usize, mut node: Value) {
    node["dependencies"] =
        Value::Array(application_dependencies(&node["body"]));
    package["semantic_graph"]["nodes"][index] = node;
}

fn node_index_by_id(package: &Value, id: &Value) -> Option<usize> {
    package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array")
        .iter()
        .position(|node| node["node_id"] == *id)
}

/// The backticked spellings of `text`.
fn backticked(text: &str) -> BTreeSet<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

#[trace("TC-233", "FR-322-AC-17")]
#[test]
fn tc_233_operation_catalog_is_closed_and_bound_to_both_schemas() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package_schema = parsed_fixture(SCHEMA);
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

    let defs = &package_schema["$defs"];
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
        "a profile law role names no fixed catalog definition: the package selection selects it"
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
            &defs["SemanticTerm"]["oneOf"][2]["properties"]["operator"]["enum"],
            "operator classes"
        ),
        entries
            .iter()
            .map(|entry| entry["operator"].as_str().expect("operator is a string"))
            .collect::<BTreeSet<_>>(),
        "every schema operator class has a catalogued operation and no other class does"
    );

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
            for prefix in ["/lock"] {
                candidate
                    .pointer_mut(&format!("{prefix}/profile_selections"))
                    .and_then(Value::as_array_mut)
                    .expect("profile selections")
                    .retain(|row| row["role"] != "temporal_profile");
            }
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

/// Index of the node whose id is `id` in a published package.
fn node_index(package: &Value, id: &Value) -> usize {
    node_index_by_id(package, id).unwrap_or_else(|| panic!("{id} is published"))
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

/// Replace one clause node's body through `edit`, refreshing the node's
/// dependencies as a producer would.
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
    let anchor = node_index(&package, &clause_node_id("anchor"));
    let frame = node_index(&package, &clause_node_id("frame"));
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
        json!({"term": "reference", "target": clause_node_id("parameter")}),
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
    let parameter = node_index(&package, &clause_node_id("parameter"));
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
            node(node_index(&package, &clause_node_id("object_type")))
        ),
        json!("field_declaration"),
        "no model node is a field declaration",
    );
    refused(
        &format!(
            "{}/semantic_form",
            node(node_index(&package, &clause_node_id("object_type")))
        ),
        json!("operation_declaration"),
        "no model node is an operation declaration",
    );
    refused(
        &format!(
            "{}/semantic_form",
            node(node_index(&package, &clause_node_id("object_type")))
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
        |b: &str| json!({"term": "reference", "target": clause_node_id(b)});
    let expect = |candidate: &Value, refusal: StateRefusal, locus: Value, why: &str| {
        match read_package_semantics(candidate, &catalog) {
            Err(ReadRefusal::State(actual, actual_locus)) => {
                assert_eq!(actual, refusal, "{why}: refusal");
                assert_eq!(actual_locus, locus, "{why}: locus");
            }
            other => panic!("{why}: expected a state refusal, got {other:?}"),
        }
    };
    let target = |b: &str| clause_node_id(b);

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
        .retain(|dep| *dep != clause_node_id("object_type"));
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
    let self_index = node_index(&retyped, &clause_node_id("parameter"));
    retyped["semantic_graph"]["nodes"][self_index]["semantic_type"] = target("boolean");
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
    let anchor = node_index(&package, &clause_node_id("anchor"));
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
    let frame = node_index(&package, &clause_node_id("frame"));
    let mut wrong = package.clone();
    wrong["semantic_graph"]["nodes"][frame]["semantic_type"] = target("reference");
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
    let anchor = node_index(&both, &clause_node_id("anchor"));
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
        |b: &str| json!({"term": "reference", "target": clause_node_id(b)});
    let to_constraint = |body: &mut Value| {
        body["operation"]["member"]["clause"] = json!("constraint");
    };
    let read = |candidate: &Value| {
        let candidate = candidate.clone();
        (read_package_semantics(&candidate, &catalog), candidate)
    };

    // CN-01: admitted.
    let constrained = edited_clause(&package, "invariant", to_constraint);
    let (admitted, edited) = read(&constrained);
    assert!(
        schema.is_valid(&edited),
        "a constraint clause validates against the V2 schema"
    );
    assert!(admitted.is_ok(), "a constraint is admitted: {admitted:?}");

    // CN-02: a constraint anchors at its type's node, never an operation anchor.
    let wrong = edited_clause(&package, "invariant", |body| {
        to_constraint(body);
        body["arguments"][1] = reference("anchor");
    });
    match read(&wrong) {
        (Err(ReadRefusal::State(refusal, locus)), _) => {
            assert_eq!(refusal, StateRefusal::MalformedDeclaration);
            assert_eq!(
                locus,
                clause_node_id("anchor")
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
        (Err(ReadRefusal::State(refusal, locus)), edited) => {
            assert_eq!(refusal, StateRefusal::OperatorIneligible);
            assert_eq!(
                locus,
                edited["semantic_graph"]["nodes"][clause_node_index(&edited, "constraint")]["node_id"]
            );
        }
        other => panic!(
            "a constraint binding a second parameter refuses: {:?}",
            other.0
        ),
    }
}
/// A minimal test-only domain package and model-declaration-node mapping,
/// keyed to the published clause package's own node ids (FR-322's
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
            "node_id": clause_node_id("object_type"),
            "owner": {
                "node": "test/config-version",
                "identity": "test/model-bindings",
            },
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
    let target = |b: &str| clause_node_id(b);

    let mut with_model = package.clone();
    with_model["model_bindings"] = clause_model_bindings(0);
    assert_eq!(
        read_package_semantics(&with_model, &catalog),
        Ok(()),
        "attaching a model that agrees with the published package changes nothing"
    );

    // FND-001: an undeclared frame field entry name.
    let frame_index = node_index(&with_model, &clause_node_id("frame"));
    let mut bad_field = with_model.clone();
    bad_field["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][0]["name"] =
        json!("no_such_field");
    assert_eq!(
        read_package_semantics(&bad_field, &catalog),
        Err(ReadRefusal::Frame(
            FrameRefusal::MissingName,
            target("object_type")
        )),
        "a frame field entry naming no field of its declaring object type refuses missing-name"
    );
    let mut bad_field_no_model = package.clone();
    let frame_index = node_index(&bad_field_no_model, &clause_node_id("frame"));
    bad_field_no_model["semantic_graph"]["nodes"][frame_index]["body"]["modifies"][0]["name"] =
        json!("no_such_field");
    assert_eq!(
        read_package_semantics(&bad_field_no_model, &catalog),
        Ok(()),
        "the same entry is silently admitted without a model to resolve its name against"
    );

    // FND-002 (FR-342 step 4): an anchor naming a nonexistent operation.
    let anchor_index = node_index(&with_model, &clause_node_id("anchor"));
    let mut bad_operation = with_model.clone();
    bad_operation["semantic_graph"]["nodes"][anchor_index]["body"]["members"][1]["value"]["value"] =
        json!("doesNotExist");
    assert_eq!(
        read_package_semantics(&bad_operation, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::MissingName,
            target("anchor")
        )),
        "an anchor naming an operation the context does not declare refuses missing-name"
    );
    let mut bad_operation_no_model = package.clone();
    let anchor_index = node_index(&bad_operation_no_model, &clause_node_id("anchor"));
    bad_operation_no_model["semantic_graph"]["nodes"][anchor_index]["body"]["members"][1]["value"]
        ["value"] = json!("doesNotExist");
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

    let anchor_index = node_index(&package, &clause_node_id("anchor"));
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
    second_anchor["node_id"] = absent_node_id("b7");
    second_anchor["body"]["members"][1]["value"]["value"] = json!("otherAttempt");
    shared_frame["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes")
        .push(second_anchor);
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
            json!({"term": "reference", "target": clause_node_id("object_type")});
    });
    assert_eq!(
        read_package_semantics(&both, &catalog),
        Err(ReadRefusal::State(
            StateRefusal::MalformedDeclaration,
            clause_node_id("object_type")
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
    for schema in [parsed_fixture(SCHEMA)] {
        assert!(
            string_set(&schema["$defs"]["OperationIdentity"]["enum"], "identities")
                .contains("quire.op.state.clause")
        );
    }
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));
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

/// A Leaf term of FR-322's body grammar.
fn is_leaf_term(term: &Value) -> bool {
    match term["term"].as_str() {
        Some("literal" | "reference" | "dependency_reference") => true,
        _ => false,
    }
}

/// A Group term: an `aggregate` whose members are each a Leaf or a `binding`
/// of a Leaf.
fn is_group_term(term: &Value) -> bool {
    term["term"] == "aggregate"
        && term["members"].as_array().is_some_and(|members| {
            members.iter().all(|member| {
                is_leaf_term(member)
                    || (member["term"] == "binding" && is_leaf_term(&member["value"]))
            })
        })
}

/// A Tuple term: an `aggregate` whose members are each a Leaf or a Group.
fn is_tuple_term(term: &Value) -> bool {
    term["term"] == "aggregate"
        && term["members"].as_array().is_some_and(|members| {
            members
                .iter()
                .all(|member| is_leaf_term(member) || is_group_term(member))
        })
}

/// A Member term: a Leaf, a Group, or a `binding` of a Leaf, a Group or a
/// Tuple.
fn is_member_term(term: &Value) -> bool {
    let value = &term["value"];
    is_leaf_term(term)
        || is_group_term(term)
        || (term["term"] == "binding"
            && (is_leaf_term(value)
                || is_group_term(value)
                || is_tuple_term(value)))
}

/// A Body term: a Leaf, an `application` of Members, an `aggregate` of
/// Members, a `frame` body or an `extension_form` body whose clause values
/// are each a `reference`. No production names itself or a higher
/// stratum, so the walk is a fixed number of levels deep.
fn is_body_term(term: &Value) -> bool {
    let all_members = |items: &Value| {
        items
            .as_array()
            .is_some_and(|items| items.iter().all(|item| is_member_term(item)))
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
        _ => is_leaf_term(term),
    }
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
    let node = json!({"package": "example", "path": ["scalar_type", "text"], "ordinal": 0});
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
    assert!(is_body_term(&body), "the body is a Body production");
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
            !is_body_term(&node_body(&nested)),
            "{id} is outside the grammar"
        );
        let mut referenced = parsed_fixture(POSITIVE_ALL_FAMILIES);
        *referenced.pointer_mut(pointer).expect("pointer exists") = mutation["flattened"].clone();
        assert!(
            schema.is_valid(&referenced),
            "{id} with a referenced subterm passes"
        );
        assert!(
            is_body_term(&node_body(&referenced)),
            "{id} referenced form is in the grammar"
        );
    }
}

fn walk_id(name: &str) -> Value {
    json!({"package": "walk", "path": [name], "ordinal": 0})
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
    for source in [SCHEMA] {
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

    for source in [SCHEMA] {
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

const INFINITE_TRACE: &str = "quire.temporal.infinite-trace/v1";
const TIMED: &str = "quire.temporal.timed/v1";

/// FR-250's five profile identities.
fn fr_250_profiles() -> Vec<&'static str> {
    vec![
        "quire.temporal.event-position.false-extension/v1",
        "quire.temporal.fixed-sample.false-extension/v1",
        "quire.temporal.timestamped-event.finite-window/v1",
        INFINITE_TRACE,
        TIMED,
    ]
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
    [SCHEMA]
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
            .map(Value::to_string)
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
    let declaration = &member["declaration"];
    let name = declaration["path"][0].as_str().unwrap_or_default();
    let declaring = node_by_id(nodes, declaration)
        .filter(|node| node["node_tag"] == "model" && node["semantic_form"] == "object_type");
    if declaring.is_none() {
        return Err(TemporalRefusal::MalformedDeclaration);
    }
    if !selected.contains(&name) {
        return Err(TemporalRefusal::MissingSelection);
    }
    let matches = effective_operations(domain, name)
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
    ordered.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
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
                return Err(TemporalRefusal::OperatorIneligible(node["node_id"]["path"][0].as_str().unwrap_or_default().to_owned()));
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
/// node (no application at its body root) carries a
/// `SourceOwner` when declared, and a `ModelOwner` when it is a model
/// declaration node or a clause function.
fn required_owner_kind(node: &Value) -> Option<&'static str> {
    let structural = node["body"]["term"] != "application";
    if !structural {
        None
    } else if node.get("declaration").is_some() {
        if node["node_tag"] == "scalar_type"
            && matches!(
                node["semantic_form"].as_str(),
                Some("enum" | "dimension" | "unit")
            )
        {
            Some("definition")
        } else {
            Some("source")
        }
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

/// FR-322-AC-51: a node carries `owner` exactly when it is declared by a
/// source or a model; the full reader admits every positive fixture.
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
            "definition".to_owned(),
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
    assert_eq!(mutations.len(), 7);
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

/// FR-322-AC-52: a declared node's owner and its source map agree. Moving the
/// declaration region to another lock source changes no node id and not the
/// package id, and is refused invalid-value at that source-map entry.
#[trace("TC-233", "FR-322-AC-52")]
#[test]
fn tc_233_a_moved_declaration_region_refuses_invalid_value_at_its_source_map_entry() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let mut package = parsed_fixture(POSITIVE_TWO_OWNERS_A);
    let other = parsed_fixture(POSITIVE_TWO_OWNERS_B)["lock"]["sources"][0].clone();
    let point = declared_index(&package, &["Point"]);
    let id = package["semantic_graph"]["nodes"][point]["node_id"].clone();
    add_lock_source(&mut package, &other);
    move_declaration_regions(&mut package, point, &other);
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
            node["dependencies"] =
        Value::Array(application_dependencies(&node["body"]));
        });
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

const DOMAIN_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-orders.json");

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
/// package. Returns the population node's id.
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
    append_structural_node(package, reference);
    let reference_id = last_id(package);
    set["body"]["members"][0]["target"] = reference_id.clone();
    set["dependencies"] = json!([reference_id]);
    append_structural_node(package, set);
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
    dependencies.sort_by(node_id_order);
    population["dependencies"] = json!(dependencies);
    append_structural_node(package, population);
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
            node["dependencies"] =
        Value::Array(application_dependencies(&node["body"]));
        });
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
                    Err(ReadRefusal::DerivedShape(population["node_id"].clone())),
                    "{id}: refused at the population node"
                );
            }
        }
    }
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
            Some(json!({"package": "absent", "path": ["absent"], "ordinal": 0}))
        } else if let Some(owner_node) = population["model_population_over"].as_str() {
            let order = node_index_where(&candidate, |node| {
                node["node_tag"] == "model" && node["owner"]["node"] == "ix://acme/orders/Order"
            });
            let object = if owner_node == "ix://acme/orders/Order" {
                candidate["semantic_graph"]["nodes"][order]["node_id"].clone()
            } else {
                let mut other = candidate["semantic_graph"]["nodes"][order].clone();
                other["owner"]["node"] = json!(owner_node);
                append_structural_node(&mut candidate, other);
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
                node["dependencies"] =
        Value::Array(application_dependencies(&node["body"]));
            });
            let member =
                &candidate["semantic_graph"]["nodes"][application]["body"]["operation"]["member"];
            assert!(member["population"].is_object(), "{id}");
            assert!(
                v2.is_valid(&candidate),
                "{id}: the member with a population validates against the V2 schema"
            );
            if without_dependency {
                let kept = Value::Array(application_dependencies(&published["semantic_graph"]["nodes"][application]["body"]));
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
            for schema in [parsed_fixture(SCHEMA)] {
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
    for schema in [parsed_fixture(SCHEMA)] {
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
    for schema in [parsed_fixture(SCHEMA)] {
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
    for schema in [parsed_fixture(SCHEMA)] {
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

const MODEL_OWNER_FORM_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-owner-form-vectors.json");

/// The clause-operations fixture with the form of its declaring type's node
/// changed from `object_type` to `form`.
fn clause_package_owned_by(form: &str) -> Value {
    let mut package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    let index = node_index(&package, &clause_node_id("object_type"));
    package["semantic_graph"]["nodes"][index]["semantic_form"] = json!(form);
    package
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
        let package = clause_package_owned_by(form);
        assert_eq!(
            read_package_semantics(&package, &catalog),
            Ok(()),
            "a {form} owner is admitted by the full reader"
        );
        let declaring = clause_node_id("object_type");
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
        let parameter = node_by_id(
            nodes,
            &invariant["body"]["arguments"][0]["members"][0]["target"],
        )
        .expect("the self parameter is published");
        let self_type = node_by_id(nodes, &parameter["semantic_type"])
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
            [node_index(&frame_owned, &all_families_node_id("process"))]
        .clone();
        declaring["semantic_form"] = json!(form);
        declaring["declaration"]["qualified_name"] = json!(["Example", "NotAContext"]);
        append_structural_node(&mut frame_owned, declaring);
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
        assert_eq!(
            read_package_semantics(&frame_owned, &catalog),
            Err(ReadRefusal::Frame(
                FrameRefusal::MalformedDeclaration,
                declaring_id
            )),
            "a frame typed by a {form} refuses at the frame step"
        );

        let mut package = clause_package_owned_by("process");
        let mut other = package["semantic_graph"]["nodes"]
            [node_index_where(&package, |node| node["semantic_form"] == "process")]
        .clone();
        other["semantic_form"] = json!(form);
        other["declaration"]["qualified_name"] = json!(["Config", "NotAContext"]);
        append_structural_node(&mut package, other);
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
        dependencies.sort_by(node_id_order);
        node["dependencies"] = json!(dependencies);
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
        let owner_id = match owner_form {
            "object_type" => all_families_node_id("account"),
            _ => all_families_node_id("process"),
        };
        if owner_form == "state_machine" {
            let index = node_index(&package, &owner_id);
            package["semantic_graph"]["nodes"][index]["semantic_form"] = json!("state_machine");
        }
        let frame = node_index_where(&package, |node| node["semantic_form"] == "frame");
        let frame_id = package["semantic_graph"]["nodes"][frame]["node_id"].clone();
        let owner = owner_id.clone();
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
        let decided = match read_package_semantics(&package, &catalog) {
            Ok(()) => json!({"admitted": true}),
            Err(ReadRefusal::Frame(refusal, locus)) => {
                let named = owner.clone();
                let framed = frame_id.clone();
                let at = if locus == named {
                    "declaration"
                } else if locus == framed {
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
    let mut previous_key: Option<(OrderedId, u64)> = None;
    for (index, row) in step_rows.iter().enumerate() {
        if row["node_id"] != *owner {
            return Some(format!("/refinement_step_rows/{index}"));
        }
        let ordinal = row["ordinal"].as_u64().expect("row ordinal");
        let key = (OrderedId(row["node_id"].clone()), ordinal);
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
    let Some(typed) = node_by_id(nodes, &subnode["semantic_type"]) else {
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

/// FR-469's reader joins over every `extension` node, ascending by node
/// id, reporting the first failing join of the first failing node.
fn extension_joins(package: &Value, held: &[HeldExtension]) -> Result<(), ExtensionRefusal> {
    let nodes = package["semantic_graph"]["nodes"]
        .as_array()
        .expect("package nodes are an array");
    let mut extensions = nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node["node_tag"] == "extension")
        .collect::<Vec<_>>();
    extensions.sort_by(|(_, left), (_, right)| node_id_order(&left["node_id"], &right["node_id"]));
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
            let Some(subnode) = node_by_id(nodes, &clause["value"]["target"]) else {
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
        targets.sort_by(node_id_order);
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

/// Apply `mutate` to the `Audit` form node.
fn mutate_audit_form(package: &mut Value, mutate: impl FnOnce(&mut Value)) {
    let index = extension_node_index(package, "Audit");
    mutate(&mut package["semantic_graph"]["nodes"][index]);
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
    let placeholder = json!({"package": "", "path": [], "ordinal": 0});
    let mut dependencies = body_targets(&body);
    dependencies.sort_by(node_id_order);
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
    append_structural_node(package, node);
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
        form.get("recursion_group").is_none()
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
            let target = node_by_id(nodes, &clause["value"]["target"])
                .expect("every clause value is its own node");
            clause_value_type(nodes, target)
        })
        .collect::<Vec<_>>();
    assert_eq!(types, ["Text", "Int[1, 3650]"]);
    let mut targets = body_targets(&form["body"]);
    targets.sort_by(node_id_order);
    assert_eq!(
        form["dependencies"],
        json!(targets),
        "node-id-ascending clause targets"
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
            targets.sort_by(node_id_order);
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
    assert_eq!(
        read_extension_package(&mistyped, &held),
        Err(ExtensionRefusal {
            naming: vec!["days".to_owned(), "Int[1, 3650]".to_owned(), "Text".to_owned()],
            ..extension_refusal("ill_typed", Some("type-mismatch"), pointer.clone())
        })
    );
    // X11: the days reference re-pointed at a key the graph lacks.
    let absent = json!({"package": "absent", "path": ["absent"], "ordinal": 0});
    let mut dangling = base.clone();
    mutate_audit_form(&mut dangling, |node| {
        node["body"]["clauses"][1]["value"]["target"] = absent.clone();
        let mut targets = body_targets(&node["body"]);
        targets.sort_by(node_id_order);
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
    let later = if node_id_order(&record, &node_id).is_gt() {
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
    let node = node_by_id(
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes"),
        &quota,
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
    for schema_text in [SCHEMA] {
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

/// The unique, id-ascending `reference` targets of a body, at any depth.
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
    found.sort_by(node_id_order);
    found.dedup();
    found
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

#[trace("TC-367", "FR-322-AC-123", "FR-322-AC-124", "FR-322-AC-125")]
#[test]
fn tc_367_timed_model_authority_bindings_are_explicit_and_joined() {
    let package: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).unwrap();
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let tick_package: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES).unwrap();
    let tick_nodes = tick_package["semantic_graph"]["nodes"].as_array().unwrap();
    let id = |node: &Value| node["node_id"].clone();
    let target = |node: &Value, member: usize| node["body"]["members"][member]["value"]["target"].clone();
    let root = &nodes[52];
    let dimension_id = target(root, 1);
    let dimension = nodes.iter().find(|node| id(node) == dimension_id).unwrap();
    assert_eq!(dimension["semantic_form"], "dimension");
    assert_eq!(nodes[43]["semantic_type"], dimension_id);

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
    assert_eq!(frame["semantic_type"], target(source, 2));

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
/// repeated `node_id` (graph shape), then the owner join.
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
                format!("/semantic_graph/nodes/{position}/owner"),
            ));
        }
    }
    let roots = model_root_positions(package);
    for position in &roots {
        let owner = &nodes[*position]["owner"];
        let supplied = owner["identity"]
            .as_str()
            .is_some_and(|identity| selected_model_document(package, identity).is_some());
        if !supplied {
            return Some((
                "missing_import",
                "missing-selection",
                format!("/semantic_graph/nodes/{position}/owner/identity"),
            ));
        }
    }
    None
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
        ("add", Value::Array(items)) => {
            items.insert(member.parse::<usize>().expect("index"), patch["value"].clone());
        }
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

/// FR-322-AC-135 and FR-322-AC-138: the published model-root vectors run
/// as RFC 6902 patches over `positive-all-families`. A `schema` of `invalid`
/// fails the V2 schema at the mutated node and is `malformed_wire` at its
/// locus; a `schema` of `valid` passes it, and the reader's first model-root
/// refusal (a repeated `node_id`, then the owner join) is exactly the recorded
/// outcome, cause and locus.
#[trace("TC-310", "FR-322-AC-135", "FR-322-AC-138")]
#[test]
fn tc_310_root_owner_schema_and_reader_stages_run_from_the_published_vectors() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
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
        let schema_valid = schema.is_valid(&package);
        assert_eq!(schema_valid, vector["schema"] == "valid", "{id}: schema");
        let observed = if schema_valid {
            model_root_reader_refusal(&package)
                .map(|(code, cause, locus)| (code.to_owned(), cause.to_owned(), locus))
        } else {
            let locus = vector["locus"].as_str().expect("locus");
            let node_pointer = locus
                .strip_prefix("/semantic_graph/nodes/")
                .and_then(|rest| rest.split('/').next())
                .map(|index| format!("/semantic_graph/nodes/{index}"))
                .expect("node locus");
            assert!(
                schema
                    .iter_errors(&package)
                    .any(|error| error.instance_path().to_string().starts_with(&node_pointer)),
                "{id}: the V2 schema refuses the node"
            );
            Some(("malformed_wire".to_owned(), String::new(), locus.to_owned()))
        };
        let expected = match vector["outcome"].as_str().expect("outcome") {
            "admitted" => None,
            refused => Some((
                refused
                    .strip_prefix("refused:")
                    .expect("refused outcome")
                    .to_owned(),
                vector["cause"].as_str().unwrap_or("").to_owned(),
                vector["locus"].as_str().expect("locus").to_owned(),
            )),
        };
        assert_eq!(observed, expected, "{id}");
    }
    assert_eq!(
        ids,
        BTreeSet::from(["MR-00", "MR-01", "MR-02", "MR-03", "MR-04", "MR-09"])
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

const MODEL_TUPLE_UNION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-tuple-union-vectors.json");
/// The selected document of `acme/shapes`: a full FCD semantic IR 2.0.0
/// domain package. `domain_package` in the vectors is its projection.
const SHAPES_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-shapes.json");

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
        "Float32", "Float64", "Duration", "Rational", "Decimal",
    ];
    // An FR-154 alias is read through to its target (chains included) before
    // the payload and position tests.
    let read_through = |reference: &str| -> String {
        let mut current = reference.to_owned();
        for _ in 0..16 {
            let next = document["aliases"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|alias| alias["node"] == current.as_str())
                .map(|alias| alias["target"].as_str().expect("alias target").to_owned());
            match next {
                Some(target) => current = target,
                None => break,
            }
        }
        current
    };
    let value_type = |reference: &str| {
        document["value_types"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|candidate| candidate["node"] == reference)
    };
    let resolves = |reference: &str| match read_through(reference).strip_prefix("ix://quire/native/") {
        Some(native) => natives.contains(&native),
        None => {
            let reference = read_through(reference);
            value_type(&reference).is_some()
                || reference.strip_prefix("ix://acme/shapes/").is_some_and(|name| {
                    shapes_declared(document, "tuple_types", name).is_some()
                        || shapes_declared(document, "variant_types", name).is_some()
                })
        }
    };
    // FR-322 "Admitted payload and position types": the four declared types
    // with no step 4 row are refused at intake (FR-322-AC-181, -182).
    // A package value type is refused when its native is `Rational` or `Decimal`, or `Text` without
    // the flat `lower`, `upper` and `profile` members (FR-154's value type
    // shape, as `model-member-type-vectors.json` publishes it).
    let unmapped = |reference: &str| match read_through(reference).strip_prefix("ix://quire/native/") {
        Some(native) => matches!(native, "Rational" | "Decimal" | "Text"),
        None => value_type(&read_through(reference)).is_some_and(|found| {
            match found["native"].as_str().and_then(|native| native.strip_prefix("ix://quire/native/")) {
                Some("Rational" | "Decimal") => true,
                Some("Text") => ["lower", "upper", "profile"].iter().any(|member| found.get(member).is_none()),
                _ => false,
            }
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
                .is_some_and(|reference| reference != "ix://quire/native/Clock" && !unmapped(reference));
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
                && (payload == "ix://quire/native/Clock" || unmapped(payload) || !resolves(payload))
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

/// FR-322-AC-141 to FR-322-AC-145: family, position, field, member, union
/// value, enum value, equality, identity and intake cases over the projection.
#[trace(
    "TC-940",
    "FR-322-AC-141",
    "FR-322-AC-142",
    "FR-322-AC-143",
    "FR-322-AC-144",
    "FR-322-AC-145",
    "FR-322-AC-147",
    "FR-322-AC-181",
    "FR-322-AC-182"
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
            None => {
                assert_eq!(case["expected"]["admitted"], true, "{}: an admitted case says so", case["name"]);
                assert_eq!(observed, None, "{}", case["name"]);
            }
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
        "recursion_cases",
        "intake_cases",
    ]
    .map(|list| vectors[list].as_array().expect(list).len());
    assert_eq!(names.len(), published.iter().sum::<usize>(), "every published case ran");
    assert_eq!(published, [5, 7, 3, 1, 8, 4, 5, 4, 35], "no published case was dropped");
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

/// The scalar type node of a rendered type name, found by its semantic form.
fn scalar_type_node(nodes: &[Value], form: &str) -> Value {
    nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == form)
        .map(|node| node["node_id"].clone())
        .unwrap_or_else(|| panic!("scalar_type/{form} node"))
}

/// The guard and duplicate-place checks of FR-322 "Timed-model nodes" over a
/// whole package, as the code, cause and RFC 6901 locus a reader gives (`None`
/// when admitted): a second node of a path is refused at its first forged
/// `resolved_path` row, before the root's aggregates (FR-322-AC-124), a repeated path whose rows all
/// join at the later reference of the root's `clock_places` (FR-322-AC-120), and a `clock_guard` node that no
/// root's `guards` lists refuses at that node's own `body` (FR-322-AC-243),
/// the first such node in ascending node-id order.
fn timed_guard_and_place_refusal(package: &Value) -> Option<(&'static str, &'static str, String)> {
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let position = |id: &Value| nodes.iter().position(|node| node["node_id"] == *id).unwrap();
    let mut listed_guards = Vec::new();
    for (root_at, root) in nodes.iter().enumerate().filter(|(_, node)| node["semantic_form"] == "timed_model") {
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
        // The row join of every place node (FR-322-AC-124) precedes the root's aggregates: a second node of one
        // path can differ only in its `resolved_path`, and the declared rows are the earlier node's, so the
        // first differing row of the later node is forged.
        for (at, place) in places.iter().enumerate() {
            let at_node = position(&place["target"]);
            let path = &nodes[at_node]["body"]["members"][1];
            if let Some(earlier) = places[..at].iter().find(|earlier| &nodes[position(&earlier["target"])]["body"]["members"][1] == path) {
                let (declared, rows) = (
                    nodes[position(&earlier["target"])]["body"]["members"][2]["value"]["members"].as_array().unwrap(),
                    nodes[at_node]["body"]["members"][2]["value"]["members"].as_array().unwrap(),
                );
                if let Some(row) = (0..rows.len()).find(|row| declared.get(*row) != rows.get(*row)) {
                    return Some((
                        "invalid_package",
                        "malformed-clock-place",
                        format!("/semantic_graph/nodes/{at_node}/body/members/2/value/members/{row}"),
                    ));
                }
            }
        }
        for (at, place) in places.iter().enumerate() {
            let path = &nodes[position(&place["target"])]["body"]["members"][1];
            if places[..at].iter().any(|earlier| &nodes[position(&earlier["target"])]["body"]["members"][1] == path) {
                return Some((
                    "invalid_package",
                    "duplicate-clock-place",
                    format!("/semantic_graph/nodes/{root_at}/body/members/3/value/members/{at}"),
                ));
            }
        }
        listed_guards.extend(binding("guards").into_iter().map(|entry| entry["target"].clone()));
    }
    let mut owned: Vec<usize> = (0..nodes.len()).filter(|at| nodes[*at]["semantic_form"] == "clock_guard").collect();
    owned.sort_by(|left, right| node_id_order(&nodes[*left]["node_id"], &nodes[*right]["node_id"]));
    owned
        .into_iter()
        .find(|at| !listed_guards.contains(&nodes[*at]["node_id"]))
        .map(|at| ("invalid_package", "timed-node-reference", format!("/semantic_graph/nodes/{at}/body")))
}

fn timed_guard_and_place_cause(package: &Value) -> Option<(&'static str, &'static str)> {
    timed_guard_and_place_refusal(package).map(|(code, cause, _)| (code, cause))
}

thread_local! {
    /// A deliberate defect in the CONTENTS of an order key, for the mutation tests only
    /// (`""` is the key as published).
    static KEY_FAULT: std::cell::Cell<&'static str> = const { std::cell::Cell::new("") };
}

/// The place path key of FR-322-AC-265: the steps of a `clock_place` path in
/// order, each as (kind rank, name bytes, first number, second number), where
/// kind ranks `field`, `tuple`, `union`, `slot`; a `field` step is
/// its `name`, a `union` step its `case`, a `tuple` step its index, a `slot` step its collection, minimum
/// and maximum (`unbounded` above every integer). Neither the selector nor
/// `resolved_path` is read.
fn timed_place_path_key(place: &Value) -> Vec<(u8, Vec<u8>, u64, u64)> {
    let fault = KEY_FAULT.with(|fault| fault.get());
    place["body"]["members"][1]["value"]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| {
            let members = step["members"].as_array().unwrap();
            let text = |at: usize| members[at]["value"]["value"].as_str().unwrap().as_bytes().to_vec();
            let int = |at: usize| members[at]["value"]["value"].as_u64().unwrap();
            match members[0]["value"]["value"].as_str().unwrap() {
                "field" => (0, if fault == "field-name" { Vec::new() } else { text(1) }, 0, 0),
                "tuple" => (1, Vec::new(), if fault == "tuple-index" { 0 } else { int(1) }, 0),
                "union" => (2, if fault == "union-case" { Vec::new() } else { text(1) }, 0, 0),
                "slot" => (
                    3,
                    if fault == "slot-collection" { Vec::new() } else { text(1) },
                    if fault == "slot-minimum" { 0 } else { int(3) },
                    if fault == "slot-maximum" { 0 } else { members[4]["value"]["value"].as_u64().unwrap_or(u64::MAX) },
                ),
                other => panic!("path step {other}"),
            }
        })
        .collect()
}

/// A comparable part of an order key (FR-322-AC-265). A rational is compared by
/// cross multiplication, so no rendering enters a key.
#[derive(Clone, Debug, PartialEq, Eq)]
struct KeyRational(u128, u128);

impl Ord for KeyRational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * other.1).cmp(&(other.0 * self.1))
    }
}

impl PartialOrd for KeyRational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum KeyAtom {
    Int(u64),
    Bytes(Vec<u8>),
    Region(String, u64, u64),
    Fraction(KeyRational),
    List(Vec<KeyAtom>),
}

/// The structural key of a term node (FR-322-AC-265): its form and the shape of its body, a
/// reference replaced by the key of its target, a literal by its kind and value. No node id,
/// no source region and no `type` reference enters it, so a term shared by many nodes keys
/// each of them the same wherever else it is used.
fn timed_term_key(nodes: &[Value], id: &Value, depth: u8) -> KeyAtom {
    let empty = KeyAtom::List(Vec::new());
    let Some(node) = nodes.iter().find(|node| node["node_id"] == *id) else { return empty };
    if depth == 0 {
        return empty;
    }
    KeyAtom::List(vec![
        KeyAtom::Bytes(node["semantic_form"].as_str().unwrap_or("").as_bytes().to_vec()),
        timed_term_shape(nodes, &node["body"], depth - 1),
    ])
}

fn timed_term_shape(nodes: &[Value], value: &Value, depth: u8) -> KeyAtom {
    let bytes = |text: &Value| KeyAtom::Bytes(text.as_str().unwrap_or("").as_bytes().to_vec());
    match value["term"].as_str() {
        Some("reference") => timed_term_key(nodes, &value["target"], depth),
        Some("literal") => KeyAtom::List(vec![
            bytes(&value["value_kind"]),
            KeyAtom::Bytes(value["value"].as_str().map_or_else(|| value["value"].to_string(), str::to_owned).into_bytes()),
        ]),
        Some("aggregate") => KeyAtom::List(value["members"].as_array().into_iter().flatten().map(|member| timed_term_shape(nodes, member, depth)).collect()),
        Some("binding") => KeyAtom::List(vec![bytes(&value["name"]), timed_term_shape(nodes, &value["value"], depth)]),
        Some("application") => KeyAtom::List(vec![
            bytes(&value["operator"]),
            KeyAtom::List(value["arguments"].as_array().into_iter().flatten().map(|argument| timed_term_shape(nodes, argument, depth)).collect()),
        ]),
        _ => KeyAtom::List(Vec::new()),
    }
}

/// Order inverted, for the mutation test that reads a term key descending.
fn timed_key_inverted(atom: KeyAtom) -> KeyAtom {
    match atom {
        KeyAtom::Int(n) => KeyAtom::Int(!n),
        KeyAtom::Bytes(bytes) => KeyAtom::Bytes(bytes.into_iter().map(|byte| 255 - byte).collect()),
        KeyAtom::List(items) => KeyAtom::List(items.into_iter().map(timed_key_inverted).collect()),
        other => other,
    }
}

/// The second part of the order key of a timed aggregate member (FR-322-AC-265),
/// after its minimum region triple. A `clock_place` is its place path key. With
/// `member_key`, the other forms are keyed so that nodes of one source atom are
/// totally ordered, and by structure only (`timed_term_key`):
/// a `clock_constraint` of kind `place` by (0, referenced place path key, owner term,
/// slot terms, comparison rank, bound), of kind `parameter` by (1, operation name,
/// parameter name, comparison rank, bound); a `clock_reset` of kind `place` by (0, place
/// path key, operation name, condition, binders, owner term, slot terms, value), of kind
/// `parameter` by (1, operation name, parameter name, value); a `time_invariant` by
/// (all of its bounds' keys in order, predicate); a `clock_guard` by its operation name; an
/// `urgency` by its kind word and operation name. The comparison rank is `lt`, `le`, `eq`,
/// `ge`, `gt`.
fn timed_member_key(package: &Value, id: &Value, path_key: bool, member_key: bool) -> KeyAtom {
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let find = |id: &Value| nodes.iter().find(|node| node["node_id"] == *id);
    let fault = KEY_FAULT.with(|fault| fault.get());
    let path_atoms = |place: &Value| -> KeyAtom {
        match find(place).filter(|node| node["semantic_form"] == "clock_place") {
            Some(node) if path_key => KeyAtom::List(
                timed_place_path_key(node)
                    .into_iter()
                    .map(|(kind, name, first, second)| {
                        KeyAtom::List(vec![KeyAtom::Int(kind.into()), KeyAtom::Bytes(name), KeyAtom::Int(first), KeyAtom::Int(second)])
                    })
                    .collect(),
            ),
            _ => KeyAtom::List(Vec::new()),
        }
    };
    let node = find(id);
    let form = node.and_then(|node| node["semantic_form"].as_str()).unwrap_or("");
    if form == "clock_place" {
        return path_atoms(id);
    }
    let empty = KeyAtom::List(Vec::new());
    let (true, Some(node)) = (member_key, node) else { return empty };
    let member = |at: usize| &node["body"]["members"][at]["value"];
    let text = |value: &Value| KeyAtom::Bytes(value["value"].as_str().unwrap_or("").as_bytes().to_vec());
    let operation_name = |anchor: &Value| {
        if fault == "operation" {
            return KeyAtom::Bytes(Vec::new());
        }
        find(anchor).map_or(KeyAtom::Bytes(Vec::new()), |anchor| text(&anchor["body"]["members"][1]["value"]))
    };
    let ranks = if fault == "comparison-rank" { ["lt", "eq", "le", "ge", "gt"] } else { ["lt", "le", "eq", "ge", "gt"] };
    let rank = |value: &Value| KeyAtom::Int(ranks.iter().position(|word| Some(*word) == value["value"].as_str()).unwrap_or(5) as u64);
    let fraction = |value: &Value| {
        let (numerator, denominator) = value["value"].as_str().unwrap_or("0/1").split_once('/').unwrap_or(("0", "1"));
        if fault == "bound" {
            return KeyAtom::Fraction(KeyRational(0, 1));
        }
        if fault == "bound-text" {
            return KeyAtom::Bytes(numerator.as_bytes().to_vec());
        }
        KeyAtom::Fraction(KeyRational(numerator.parse().unwrap_or(0), denominator.parse().unwrap_or(1)))
    };
    let term = |reference: &Value, fault_name: &str| {
        if !fault_name.is_empty() && fault == fault_name {
            return KeyAtom::List(Vec::new());
        }
        if fault == "term-region" {
            let at = package["source_map"].as_array().unwrap().iter().filter(|entry| entry["node_id"] == reference["target"]);
            let region = at
                .flat_map(|entry| entry["regions"].as_array().unwrap())
                .map(|region| (region["source"]["identity"].as_str().unwrap_or("").to_owned(), region["start"].as_u64().unwrap_or(0), region["end"].as_u64().unwrap_or(0)))
                .min()
                .map_or(KeyAtom::List(Vec::new()), |(source, start, end)| KeyAtom::Region(source, start, end));
            return region;
        }
        let key = timed_term_key(nodes, &reference["target"], 8);
        if fault == "term-order" { timed_key_inverted(key) } else { key }
    };
    let terms = |list: &Value, fault_name: &str| {
        if !fault_name.is_empty() && fault == fault_name {
            return KeyAtom::List(Vec::new());
        }
        KeyAtom::List(list["members"].as_array().into_iter().flatten().map(|entry| term(entry, "")).collect())
    };
    let value_of = |value: &Value| if fault == "reset-value" { KeyAtom::List(Vec::new()) } else { fraction(value) };
    match (form, member(0)["value"].as_str().unwrap_or("")) {
        ("clock_constraint", "place") => KeyAtom::List(vec![
            KeyAtom::Int(0), path_atoms(&member(1)["target"]), term(member(2), "owner-term"), terms(member(3), "slot-terms"), rank(member(4)), fraction(member(5)),
        ]),
        ("clock_constraint", _) => KeyAtom::List(vec![
            KeyAtom::Int(1), operation_name(&member(1)["target"]), text(member(2)), rank(member(3)), fraction(member(4)),
        ]),
        ("clock_reset", "place") => KeyAtom::List(vec![
            KeyAtom::Int(0), path_atoms(&member(2)["target"]), operation_name(&member(1)["target"]), term(member(6), "condition"),
            terms(member(5), "binders"), term(member(3), "owner-term"), terms(member(4), "slot-terms"), value_of(member(7)),
        ]),
        ("clock_reset", _) => KeyAtom::List(vec![KeyAtom::Int(1), operation_name(&member(1)["target"]), text(member(2)), value_of(member(3))]),
        ("time_invariant", _) => {
            let bounds = member(1)["members"].as_array().cloned().unwrap_or_default();
            let keep = if fault == "second-bound" { 1 } else { bounds.len() };
            KeyAtom::List(vec![
                KeyAtom::List(bounds.iter().take(keep).map(|bound| timed_member_key(package, &bound["target"], path_key, member_key)).collect()),
                term(member(0), "predicate"),
            ])
        }
        ("clock_guard", _) => operation_name(&member(0)["target"]),
        ("urgency", kind) => KeyAtom::List(vec![
            KeyAtom::Bytes(kind.as_bytes().to_vec()),
            if kind == "operation" { text(member(2)) } else { KeyAtom::Bytes(Vec::new()) },
        ]),
        _ => empty,
    }
}

/// FR-322-AC-260 and FR-322-AC-265: the locus of the first aggregate member
/// (a timed root's six reference aggregates, each place's `guards`, the
/// constraints of each guard part and the bounds of an invariant) whose order
/// key is not strictly above its predecessor's, or `None` when every
/// aggregate is in declared order. The order key is the minimum (source
/// identity, start, end) over every region of every `source_map` entry of the
/// member node and, when `path_key` is on, the place path key after it;
/// `nested` reads the lists the clause forms nest.
fn timed_aggregate_order_refusal_ruled(package: &Value, path_key: bool, member_key: bool, nested: bool) -> Option<String> {
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let key = |id: &Value| {
        let region = package["source_map"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["node_id"] == *id)
            .flat_map(|entry| entry["regions"].as_array().unwrap())
            .map(|region| (region["source"]["identity"].as_str().unwrap().to_owned(), region["start"].as_u64().unwrap(), region["end"].as_u64().unwrap()))
            .min()
            .expect("a source_map region");
        (region, timed_member_key(package, id, path_key, member_key))
    };
    let first_out_of_order = |members: &[Value]| {
        (1..members.len()).find(|at| key(&members[*at]["target"]) <= key(&members[at - 1]["target"]))
    };
    for (at, node) in nodes.iter().enumerate() {
        // (locus prefix, members, offset of members[0] in its aggregate)
        let mut lists: Vec<(String, Vec<Value>, usize)> = Vec::new();
        let members_of = |binding: usize| node["body"]["members"][binding]["value"]["members"].as_array().unwrap().clone();
        match node["semantic_form"].as_str() {
            Some("timed_model") => {
                for binding in 3..=8 {
                    lists.push((format!("/semantic_graph/nodes/{at}/body/members/{binding}/value/members"), members_of(binding), 0));
                }
            }
            Some("clock_guard") if nested => {
                for (part, entry) in members_of(1).iter().enumerate() {
                    let constraints = entry["members"].as_array().unwrap()[1..].to_vec();
                    lists.push((format!("/semantic_graph/nodes/{at}/body/members/1/value/members/{part}/members"), constraints, 1));
                }
            }
            Some("time_invariant") if nested => {
                lists.push((format!("/semantic_graph/nodes/{at}/body/members/1/value/members"), members_of(1), 0));
            }
            _ => continue,
        }
        for (prefix, members, offset) in lists {
            if let Some(member) = first_out_of_order(&members) {
                return Some(format!("{prefix}/{}", member + offset));
            }
        }
    }
    None
}

fn timed_aggregate_order_refusal(package: &Value) -> Option<String> {
    timed_aggregate_order_refusal_ruled(package, true, true, true)
}

/// Each reading of the order rule fails on its mutation: a source region moved
/// before or onto its predecessor's, a listing sorted by node id, the key read
/// from a different region, and an out-of-order `guards` of a place.
#[trace("TC-367", "FR-322-AC-260")]
#[test]
fn tc_367_aggregate_order_oracle_fails_on_each_mutation() {
    let selected = parsed_fixture(POSITIVE_TIMED_SLOT_NODES);
    assert_eq!(timed_aggregate_order_refusal(&selected), None);
    let place_at = |package: &Value, at: usize| -> Value {
        let root = &package["semantic_graph"]["nodes"][timed_node_index_of(package, "timed_model")];
        root["body"]["members"][3]["value"]["members"][at]["target"].clone()
    };
    let root_at = timed_node_index_of(&selected, "timed_model");
    let entry_of = |package: &mut Value, id: &Value| -> usize {
        package["source_map"].as_array().unwrap().iter().position(|entry| entry["node_id"] == *id).unwrap()
    };
    // The second place's region moved onto the first's: equal keys refuse.
    let mut equal = selected.clone();
    let (first, second) = (place_at(&equal, 0), place_at(&equal, 1));
    let (a, b) = (entry_of(&mut equal, &first), entry_of(&mut equal, &second));
    equal["source_map"][b]["regions"][0] = equal["source_map"][a]["regions"][0].clone();
    // One region is one minimum triple; the two paths differ, so the place path key orders them (AO-08).
    assert_eq!(timed_aggregate_order_refusal(&equal), None);
    assert_eq!(
        timed_aggregate_order_refusal_ruled(&equal, false, true, true),
        Some(format!("/semantic_graph/nodes/{root_at}/body/members/3/value/members/1")),
        "without the path key a shared region is an equal key"
    );
    // The first place's region moved after the second's.
    let mut moved = selected.clone();
    moved["source_map"][a]["regions"][0]["start"] = json!(9000);
    moved["source_map"][a]["regions"][0]["end"] = json!(9001);
    assert_eq!(
        timed_aggregate_order_refusal(&moved),
        Some(format!("/semantic_graph/nodes/{root_at}/body/members/3/value/members/1"))
    );
    // A different source identity orders before the position: identity first.
    let mut identity = selected.clone();
    identity["source_map"][b]["regions"][0]["source"]["identity"] = json!("a");
    identity["source_map"][a]["regions"][0]["source"]["identity"] = json!("b");
    assert!(timed_aggregate_order_refusal(&identity).is_some());
    // A repeated member of an aggregate is not strictly ascending.
    let mut repeated = selected.clone();
    let urgencies = repeated["semantic_graph"]["nodes"][root_at]["body"]["members"][7]["value"]["members"].clone();
    repeated["semantic_graph"]["nodes"][root_at]["body"]["members"][7]["value"]["members"][1] = urgencies[0].clone();
    assert_eq!(
        timed_aggregate_order_refusal(&repeated),
        Some(format!("/semantic_graph/nodes/{root_at}/body/members/7/value/members/1"))
    );
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
    places: &BTreeMap<OrderedId, Value>,
    parameters: &BTreeMap<OrderedId, Value>,
    owner_type: &Value,
    slot_type: &Value,
) -> Option<(&'static str, &'static str, String)> {
    let arguments = case["body"]["arguments"].as_array().unwrap();
    let target = |k: usize| OrderedId(arguments[k]["target"].clone());
    let Some(place) = places.get(&target(0)) else {
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
    let type_of = |k: usize| parameters.get(&target(k)).map(|node| node["semantic_type"].clone());
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
    let reference = &vectors["type_nodes"]["owner_reference_type"];
    let node = nodes
        .iter()
        .find(|n| n["node_id"] == *reference && n["node_tag"] == "composite_type" && n["semantic_form"] == "reference")
        .ok_or("the owner reference type is not a published reference type node")?;
    if node["body"]["members"][0]["target"] != vectors["model_join"]["node"]["node_id"] {
        return Err("the owner reference type does not name the object_type node".to_owned());
    }
    let integer = scalar_type_node(nodes, "integer");
    for (name, want) in [("self", reference), ("other", reference), ("i", &integer), ("e", &integer)] {
        if vectors["presence"]["parameters"][name]["semantic_type"] != *want {
            return Err(format!("parameter {name} has the wrong type"));
        }
    }
    Ok(())
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
    assert!(join(&mutate(&|v| v["model_join"]["node"]["owner"]["node"] = json!("ix://acme/orders/Other"))).contains("not the published node"));
    assert!(join(&mutate(&|v| v["presence"]["places"]["v"]["node"]["body"]["members"][2]["value"]["members"][0]["members"][2]["value"]["value"] = json!("ix://acme/orders/Order/w"))).contains("does not join"));
    assert!(join(&mutate(&|v| v["model_join"]["declared_with_v"]["fields"][1]["presence"] = json!("required"))).contains("does not join"));
    assert!(join(&mutate(&|v| v["model_join"]["declared_with_v"]["identity"] = json!("ix://acme/orders/Other"))).contains("disagree"));
    assert!(join(&mutate(&|v| v["presence"]["places"]["v"]["node"]["body"]["members"][0]["value"]["target"] = json!({"package": "absent", "path": ["absent"], "ordinal": 0}))).contains("not owned"));

    let types = |v: &Value| presence_type_nodes(v, nodes).expect_err("the type join is caught");
    assert!(types(&mutate(&|v| v["type_nodes"]["owner_reference_type"] = json!({"package": "absent", "path": ["absent"], "ordinal": 0}))).contains("not a published"));
    assert!(types(&mutate(&|v| v["presence"]["parameters"]["i"]["semantic_type"] = json!({"package": "absent", "path": ["absent"], "ordinal": 0}))).contains("parameter i"));

    // The oracle refuses a presence_read with an argument dropped.
    let places: BTreeMap<OrderedId, Value> = vectors["presence"]["places"].as_object().unwrap().values()
        .map(|p| (OrderedId(p["node"]["node_id"].clone()), p["node"].clone())).collect();
    let parameters: BTreeMap<OrderedId, Value> = vectors["presence"]["parameters"].as_object().unwrap().values()
        .map(|p| (OrderedId(p["node_id"].clone()), p.clone())).collect();
    let owner_type = vectors["presence"]["parameters"]["self"]["semantic_type"].clone();
    let slot_type = vectors["type_nodes"]["integer"].clone();
    assert_eq!(vectors["presence"]["cases"][1]["id"], "PR-03");
    let mut pr03 = vectors["presence"]["cases"][1]["node"].clone();
    assert_eq!(presence_cause(&pr03, &places, &parameters, &owner_type, &slot_type), None);
    pr03["body"]["arguments"].as_array_mut().unwrap().pop();
    assert!(presence_cause(&pr03, &places, &parameters, &owner_type, &slot_type).is_some());
}

const CLOCK_PRESENCE_FRAME_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/clock-presence-frame-vectors.json");
/// How an oracle reads a place that becomes present with no write of its clock.
#[derive(Clone, Copy, PartialEq)]
enum UnwrittenPresence {
    /// FR-415-AC-21: there is no successor (or the static refusal).
    NoSuccessor,
    /// The defect STD-542 rules out: an implicit reset to 0.
    ImplicitReset,
}

/// The outcome of one frame vector under the one rule: a write fixes the
/// place; with no write an `Option<Clock>` keeps its state and a union also
/// admits the clockless case; the requirement then filters the candidates and
/// is never itself a write.
fn frame_outcome(case: &Value, unwritten: UnwrittenPresence) -> Value {
    // The union form (PR-02) is dropped from FR-415: only `Option<Clock>` places remain.
    let (present, clock) = match case["form"].as_str().unwrap() {
        "option" => (case["pre"]["present"].as_bool().unwrap(), case["pre"]["clock"].clone()),
        other => panic!("form {other}"),
    };
    let requires = case["requires"].as_str();
    let wants_present = requires.map(|text| !text.starts_with("not "));
    let write = &case["write"];
    let kept = json!({"present": present, "clock": clock});
    let dropped = json!({"present": false, "clock": null});
    let mut candidates = Vec::new();
    if write.is_null() {
        if wants_present == Some(true) && !present {
            if case["precondition_establishes_absent"] == true {
                return json!({"refused": {"code": "invalid_timed_model", "cause": "clock-presence-unwritten"}});
            }
            if unwritten == UnwrittenPresence::ImplicitReset {
                return json!([{"present": true, "clock": "0 ms"}]);
            }
        }
        candidates.push(kept);
    } else if write.get("absent").is_some() || write["constructor"] == "Idle" {
        candidates.push(dropped);
    } else {
        candidates.push(json!({"present": true, "clock": write["clock"]}));
    }
    Value::Array(
        candidates
            .into_iter()
            .filter(|candidate| wants_present.is_none_or(|wanted| candidate["present"] == wanted))
            .collect(),
    )
}

/// Every way the FR-415-AC-21 frame vectors disagree with the rule.
fn frame_defects(vectors: &Value, unwritten: UnwrittenPresence) -> Vec<String> {
    let mut defects = Vec::new();
    let cases = vectors["cases"].as_array().unwrap();
    for case in cases {
        let id = case["id"].as_str().unwrap();
        if frame_outcome(case, unwritten) != case["expected"] {
            defects.push(format!("{id} outcome differs from the rule"));
        }
    }
    let ids: Vec<_> = cases.iter().map(|case| case["id"].as_str().unwrap()).collect();
    if ids != ["FP-01", "FP-03", "FP-05", "FP-07", "FP-09", "FP-11", "FP-13"] {
        defects.push("the vector ids are not FP-01, FP-03, .., FP-13".to_owned());
    }
    defects
}

/// FR-415-AC-21 (STD-542): under an operation's frame a place becomes present
/// only by a write of its clock value, `Option<Clock>` and union case alike;
/// there is no implicit reset, a requirement is never a write.
#[trace("TC-367", "FR-415-AC-21")]
#[test]
fn tc_367_a_place_becomes_present_only_by_a_write_of_its_clock() {
    let vectors = parsed_fixture(CLOCK_PRESENCE_FRAME_VECTORS);
    assert_eq!(frame_defects(&vectors, UnwrittenPresence::NoSuccessor), Vec::<String>::new());
}

/// Each reading of the rule fails on its mutation: an implicit reset, a vector
/// whose expected outcome changes and an unpaired form.
#[trace("TC-367", "FR-415-AC-21")]
#[test]
fn tc_367_frame_presence_vectors_fail_on_each_mutation() {
    let vectors = parsed_fixture(CLOCK_PRESENCE_FRAME_VECTORS);
    let reset = frame_defects(&vectors, UnwrittenPresence::ImplicitReset);
    assert!(reset.iter().any(|d| d.starts_with("FP-01 ")), "{reset:?}");
    let edit = |change: &dyn Fn(&mut Value)| {
        let mut copy = vectors.clone();
        change(&mut copy);
        frame_defects(&copy, UnwrittenPresence::NoSuccessor)
    };
    assert!(!edit(&|v| v["cases"][1]["expected"] = json!([{"present": true, "clock": "0 ms"}])).is_empty());
    assert!(!edit(&|v| v["cases"][4]["expected"] = json!([{"present": true, "clock": null}])).is_empty());
    assert!(!edit(&|v| v["cases"][1]["write"]["clock"] = json!("6 ms")).is_empty());
    assert!(!edit(&|v| { v["cases"].as_array_mut().unwrap().pop(); }).is_empty());
    // `not present` read as a write of absent for the Option bit.
    assert!(!edit(&|v| v["cases"][6]["expected"] = json!([{"present": false, "clock": null}])).is_empty());
    // The static refusal dropped or renamed.
    assert!(!edit(&|v| v["cases"][5]["expected"] = json!([])).is_empty());
    assert!(!edit(&|v| v["cases"][5]["expected"]["refused"]["cause"] = json!("clock-write")).is_empty());
}

const CONTRACT_PACKAGE_SELECTION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/contract-package-selection-vectors.json");
const TC935_CONTRACT_PACKAGE_PROFILE: &str =
    include_str!("../spec/test-cases/TC-935-contract-package-profile.md");

/// The labels of a table cell such as `[U, T]`, in order.
fn tc935_labels(cell: &str) -> Vec<String> {
    cell.trim()
        .trim_matches('`')
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|label| label.trim().to_owned())
        .filter(|label| !label.is_empty())
        .collect()
}

/// FR-322-AC-180: the machine-readable TC-935 selection vector is the
/// authoritative form, and the Markdown of TC-935 (its fenced JSON block and
/// its case table) equals it exactly.
#[trace("TC-935", "FR-322-AC-180")]
#[test]
fn tc_935_selection_vectors_are_published_and_agree_with_the_markdown() {
    tc935_agreement(CONTRACT_PACKAGE_SELECTION_VECTORS, TC935_CONTRACT_PACKAGE_PROFILE);
}

fn tc935_agreement(vector_text: &str, markdown: &str) {
    let vectors: Value = serde_json::from_str(vector_text).expect("vector file is JSON");
    assert_eq!(vectors["version"], "quire.contract-package-selection-vectors/v1");

    // The fenced JSON block under "Typed source-selection vectors".
    let section = markdown
        .split("## Typed source-selection vectors")
        .nth(1)
        .expect("TC-935 has the typed source-selection section");
    let fence = section
        .split("```json\n")
        .nth(1)
        .and_then(|rest| rest.split("\n```").next())
        .expect("the section has a fenced JSON block");
    let block: Value = serde_json::from_str(fence).expect("the fenced block is JSON");
    assert_eq!(block, vectors["selections"], "the fenced block equals `selections`");

    let selections = vectors["selections"].as_object().expect("selections");
    assert_eq!(
        selections.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        BTreeSet::from(["T", "D", "U"])
    );
    for (label, row) in selections {
        assert_eq!(
            row.as_object().expect("row").keys().map(String::as_str).collect::<Vec<_>>(),
            ["definition", "role"],
            "{label}: a closed Selection row"
        );
        let mut definition: Vec<&str> =
            row["definition"].as_object().expect("definition").keys().map(String::as_str).collect();
        definition.sort_unstable();
        assert_eq!(definition, ["authority", "identity"], "{label}: closed definition");
    }

    // The case table: `| Case | required | lock | Expected |` rows.
    let rows: Vec<Vec<String>> = section
        .lines()
        .filter(|line| line.starts_with("| "))
        .map(tc935_table_cells)
        .filter(|cells| cells.len() == 4 && cells[0] != "Case" && !cells[0].starts_with("---"))
        .collect();
    let cases = vectors["compatibility_cases"].as_array().expect("cases");
    assert_eq!(rows.len(), cases.len(), "one table row per published case");
    for (cells, case) in rows.iter().zip(cases) {
        let labels = |key: &str| -> Vec<String> {
            case[key]
                .as_array()
                .expect(key)
                .iter()
                .map(|label| label.as_str().expect("label").to_owned())
                .collect()
        };
        assert_eq!(cells[0], case["case"].as_str().expect("case"));
        assert_eq!(tc935_labels(&cells[1]), labels("required_source_selections"), "{}", cells[0]);
        assert_eq!(tc935_labels(&cells[2]), labels("lock_profile_selections"), "{}", cells[0]);
        assert_eq!(
            cells[3],
            case["expected"].as_str().expect("expected"),
            "{}: the Expected-result cell equals the published `expected`",
            cells[0]
        );
        for label in labels("required_source_selections")
            .into_iter()
            .chain(labels("lock_profile_selections"))
        {
            assert!(selections.contains_key(&label), "{}: label {label} is published", cells[0]);
        }
    }

    // The "Repeat ..." controls: the paragraph after the table is exactly the
    // published sentences, in order (whitespace-normalised).
    let after_table = section
        .rsplit_once("\n| ")
        .and_then(|(_, tail)| tail.split_once("\n\n"))
        .map(|(_, rest)| rest)
        .expect("a paragraph follows the case table");
    let paragraph = after_table.split("\n\n").next().expect("paragraph");
    let published: Vec<&str> = vectors["repeat_controls"]
        .as_array()
        .expect("repeat_controls")
        .iter()
        .map(|sentence| sentence.as_str().expect("sentence"))
        .collect();
    assert!(!published.is_empty(), "controls are published");
    assert_eq!(
        paragraph.split_whitespace().collect::<Vec<_>>().join(" "),
        published.join(" "),
        "the Repeat controls equal `repeat_controls`"
    );
}

/// The cells of one Markdown table row. A `|` inside a code span or escaped as
/// `\|` belongs to its cell instead of ending it.
fn tc935_table_cells(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut in_code = false;
    let mut chars = line.trim().chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if chars.peek() == Some(&'|') => {
                cell.push('|');
                chars.next();
            }
            '`' => {
                in_code = !in_code;
                cell.push(ch);
            }
            '|' if !in_code => cells.push(std::mem::take(&mut cell)),
            _ => cell.push(ch),
        }
    }
    cells.push(cell);
    // The row starts and ends with a delimiter: drop the empty outer cells.
    let mut cells: Vec<String> = cells.into_iter().map(|cell| cell.trim().to_owned()).collect();
    cells.remove(0);
    if cells.last().is_some_and(String::is_empty) {
        cells.pop();
    }
    cells
}

/// A cell containing `|` (in a code span or escaped) stays one cell.
#[trace("TC-935", "FR-322-AC-180")]
#[test]
fn tc_935_table_cells_keep_a_pipe_inside_a_cell() {
    assert_eq!(
        tc935_table_cells("| a | `x | y` | c \\| d | e |"),
        ["a", "`x | y`", "c | d", "e"]
    );
}

/// Mutation check for FR-322-AC-180: a changed identity, a swapped lock order,
/// a renamed case, an edited Repeat control and an edited Expected cell each make the agreement assertion fail.
#[trace("TC-935", "FR-322-AC-180")]
#[test]
fn tc_935_selection_vector_disagreement_is_detected() {
    let mutations: [(&str, &str); 5] = [
        ("quire.temporal.infinite-trace/v1", "quire.temporal.infinite-trace/v2"),
        (r#""lock_profile_selections": ["U", "T"]"#, r#""lock_profile_selections": ["T", "U"]"#),
        ("\"unrelated selection\"", "\"unrelated\""),
        ("Repeat the unrelated case with lock order", "Repeat the unrelated case with lock order EDIT"),
        ("Both items lower; output retains `[U, T]`", "Both items lower; output retains `[T]`"),
    ];
    for (from, to) in mutations {
        assert!(CONTRACT_PACKAGE_SELECTION_VECTORS.contains(from), "{from}");
        let mutated = CONTRACT_PACKAGE_SELECTION_VECTORS.replacen(from, to, 1);
        let outcome = std::panic::catch_unwind(|| {
            tc935_agreement(&mutated, TC935_CONTRACT_PACKAGE_PROFILE)
        });
        assert!(outcome.is_err(), "mutation `{to}` must be detected");
    }
}

/// Every switch of the clause oracle; the mutation test pins each to a vector.
const TIMED_CLAUSE_RULES: [&str; 22] = [
    "state-anchor-types", "path-key", "member-key", "unlisted", "nested-order", "anchor-kind", "anchor-model", "no-point", "parameter-declared", "parameter-clock",
    "owner-type", "slot-count", "slot-domain", "part-predicate", "same-anchor",
    "invariant-predicate", "invariant-bound", "frame-modifies", "binders", "condition", "one-guard", "root-constraints",
];

/// The refusal of FR-322 "Clause nodes" over a package whose root lists timed
/// clauses, as (code, cause, RFC 6901 locus), or `None` when admitted. The
/// order of the aggregates is read first (FR-322-AC-265), then the joins in
/// ascending node-id order, then the model-wide membership rules.
/// `skip` names one rule to switch off, so that a mutation test can show that
/// a published vector pins each rule; `""` reads them all.
fn timed_clause_refusal(
    package: &Value,
    document: &Value,
    skip: &str,
) -> Option<(&'static str, Option<&'static str>, String)> {
    const REF: &str = "timed-node-reference";
    let on = |rule: &str| {
        assert!(TIMED_CLAUSE_RULES.contains(&rule), "{rule} is a published switch of the clause oracle");
        skip != rule
    };
    if let Some(locus) = timed_aggregate_order_refusal_ruled(package, on("path-key"), on("member-key"), on("nested-order")) {
        return Some(("invalid_semantic_graph", None, locus));
    }
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let at_of = |id: &OrderedId| nodes.iter().position(|node| node["node_id"] == id.0);
    let node = |id: &OrderedId| at_of(id).map(|at| &nodes[at]);
    let bound = |node: &Value, name: &str| {
        node["body"]["members"].as_array().unwrap().iter().find(|member| member["name"] == name).unwrap()["value"].clone()
    };
    let target = |value: &Value| OrderedId(value["target"].clone());
    // The state step (FR-342 "Reader order" step 3) runs before the timed-model step (FR-322-AC-121): an
    // anchor and its frame are both typed by the anchor's `context`, else `invalid_model_binding` at the anchor.
    if on("state-anchor-types") {
        let mut anchors: Vec<usize> = (0..nodes.len()).filter(|at| nodes[*at]["semantic_form"] == "operation_anchor").collect();
        anchors.sort_by(|left, right| node_id_order(&nodes[*left]["node_id"], &nodes[*right]["node_id"]));
        for at in anchors {
            let anchor = &nodes[at];
            let context = target(&anchor["body"]["members"][0]["value"]);
            let frame = node(&target(&anchor["body"]["members"][2]["value"]));
            let typed = anchor["semantic_type"] == context.0
                && frame.is_some_and(|frame| frame["semantic_type"] == context.0);
            if !typed {
                return Some(("invalid_model_binding", Some("malformed-declaration"), format!("/semantic_graph/nodes/{at}")));
            }
        }
    }
    let word = |node: &Value, at: usize| node["body"]["members"][at]["value"]["value"].as_str().unwrap_or("").to_owned();
    let root_at = nodes.iter().position(|node| node["semantic_form"] == "timed_model").unwrap();
    let root = &nodes[root_at];
    let model = target(&bound(root, "model"));
    let listed = |name: &str| -> Vec<OrderedId> {
        bound(root, name)["members"].as_array().unwrap().iter().map(&target).collect()
    };
    let (guards, constraints) = (listed("guards"), listed("constraints"));
    let places = listed("clock_places");
    let first_fields: BTreeSet<String> = places
        .iter()
        .map(|place| node(place).unwrap()["body"]["members"][1]["value"]["members"][0]["members"][1]["value"]["value"].as_str().unwrap().to_owned())
        .collect();
    // The operation an anchor names, and the declared type of one of its parameters.
    let anchor_ok = |id: &OrderedId| {
        let Some(anchor) = node(id) else { return false };
        if on("anchor-kind") && anchor["semantic_form"] != "operation_anchor" {
            return false;
        }
        let Some(members) = anchor["body"]["members"].as_array() else { return true };
        (!on("anchor-model") || target(&members[0]["value"]) == model) && (!on("no-point") || members.len() == 3)
    };
    let parameter_ok = |anchor: &OrderedId, name: &str| {
        let operation = word(node(anchor).unwrap(), 1);
        let order = document["types"].as_array().unwrap().iter().find(|ty| ty["identity"].as_str().unwrap().ends_with("/Order")).unwrap();
        let parameter = order["operations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|op| op["name"] == operation.as_str())
            .and_then(|op| op["params"].as_array().unwrap().iter().find(|param| param["name"] == name));
        match parameter {
            None => !on("parameter-declared"),
            Some(param) => param["typeRef"] == "ix://quire/native/Clock" || !on("parameter-clock"),
        }
    };
    let frame_fields = |anchor: &OrderedId| -> BTreeSet<String> {
        let frame = node(&target(&node(anchor).unwrap()["body"]["members"][2]["value"])).unwrap();
        frame["body"]["modifies"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["kind"] == "field" && entry["declaration"] == model.0)
            .map(|entry| entry["name"].as_str().unwrap().to_owned())
            .collect()
    };
    let owner_ok = |id: &OrderedId| {
        let Some(owner) = node(id) else { return false };
        let ty = OrderedId(owner["semantic_type"].clone());
        ty == model
            || node(&ty).is_some_and(|ty| ty["semantic_form"] == "reference" && target(&ty["body"]["members"][0]) == model)
    };
    let boolean = |id: &OrderedId| {
        node(id).is_some_and(|term| {
            ["expression", "value"].contains(&term["node_tag"].as_str().unwrap())
                && node(&OrderedId(term["semantic_type"].clone())).is_some_and(|ty| ty["semantic_form"] == "boolean")
        })
    };
    let slot_domains = |place: &Value| -> Vec<OrderedId> {
        place["body"]["members"][1]["value"]["members"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|step| step["members"][0]["value"]["value"] == "slot")
            .map(|step| target(&step["members"][2]["value"]))
            .collect()
    };
    // owner/slots of a place clause at binding `base` (place), base+1 (owner), base+2 (slots).
    let place_use = |at: usize, clause: &Value, base: usize| -> Option<(&'static str, Option<&'static str>, String)> {
        let body = |tail: String| format!("/semantic_graph/nodes/{at}/body{tail}");
        let place = node(&target(&clause["body"]["members"][base]["value"]));
        let Some(place) = place.filter(|place| place["semantic_form"] == "clock_place") else {
            return Some(("invalid_package", Some(REF), body(format!("/members/{base}/value/target"))));
        };
        if on("owner-type") && !owner_ok(&target(&clause["body"]["members"][base + 1]["value"])) {
            return Some(("ill_typed", Some("type-mismatch"), body(format!("/members/{}/value", base + 1))));
        }
        let domains = slot_domains(place);
        let slots = clause["body"]["members"][base + 2]["value"]["members"].as_array().unwrap();
        if on("slot-count") && slots.len() != domains.len() {
            return Some(("ill_typed", Some("operator-ineligible"), body(format!("/members/{}/value", base + 2))));
        }
        for (k, (slot, domain)) in slots.iter().zip(&domains).enumerate() {
            let term = node(&target(slot));
            let typed = term.is_some_and(|term| {
                ["expression", "value"].contains(&term["node_tag"].as_str().unwrap()) && term["semantic_type"] == domain.0
            });
            if on("slot-domain") && !typed {
                return Some(("ill_typed", Some("type-mismatch"), body(format!("/members/{}/value/members/{k}", base + 2))));
            }
        }
        None
    };
    // Every timed node the root does not represent refuses at its own `body` (FR-322-AC-243 for a guard).
    let mut represented: BTreeSet<OrderedId> = BTreeSet::from([OrderedId(root["node_id"].clone())]);
    for binding in &root["body"]["members"].as_array().unwrap()[2..] {
        match binding["value"]["term"].as_str() {
            Some("reference") => {
                represented.insert(OrderedId(binding["value"]["target"].clone()));
            }
            _ => represented.extend(binding["value"]["members"].as_array().into_iter().flatten().map(|entry| OrderedId(entry["target"].clone()))),
        }
    }
    let mut unlisted: Vec<usize> =
        (0..nodes.len()).filter(|at| nodes[*at]["node_tag"] == "timed_model" && !represented.contains(&OrderedId(nodes[*at]["node_id"].clone()))).collect();
    unlisted.sort_by(|left, right| node_id_order(&nodes[*left]["node_id"], &nodes[*right]["node_id"]));
    if on("unlisted") {
        if let Some(at) = unlisted.first() {
            return Some(("invalid_package", Some(REF), format!("/semantic_graph/nodes/{at}/body")));
        }
    }
    let mut used = BTreeSet::new();
    let mut order: Vec<usize> = (0..nodes.len()).filter(|at| nodes[*at]["node_tag"] == "timed_model").collect();
    order.sort_by(|left, right| node_id_order(&nodes[*left]["node_id"], &nodes[*right]["node_id"]));
    for at in order {
        let clause = &nodes[at];
        let body = |tail: String| format!("/semantic_graph/nodes/{at}/body{tail}");
        let refuse = |tail: String| Some(("invalid_package", Some(REF), body(tail)));
        match clause["semantic_form"].as_str().unwrap() {
            "clock_constraint" => {
                if word(clause, 0) == "place" {
                    if let Some(refusal) = place_use(at, clause, 1) {
                        return Some(refusal);
                    }
                } else {
                    let anchor = target(&clause["body"]["members"][1]["value"]);
                    if !anchor_ok(&anchor) {
                        return refuse("/members/1/value/target".into());
                    }
                    if !parameter_ok(&anchor, &word(clause, 2)) {
                        return refuse("/members/2/value".into());
                    }
                }
            }
            "clock_guard" => {
                let anchor = target(&clause["body"]["members"][0]["value"]);
                if !anchor_ok(&anchor) {
                    return refuse("/members/0/value/target".into());
                }
                for (part, entry) in clause["body"]["members"][1]["value"]["members"].as_array().unwrap().iter().enumerate() {
                    let members = entry["members"].as_array().unwrap();
                    if on("part-predicate") && !boolean(&target(&members[0]["value"])) {
                        return refuse(format!("/members/1/value/members/{part}/members/0/value/target"));
                    }
                    for (k, member) in members.iter().enumerate().skip(1) {
                        let constraint = target(member);
                        let at_member = format!("/members/1/value/members/{part}/members/{k}/target");
                        let named = node(&constraint).filter(|n| n["semantic_form"] == "clock_constraint");
                        let Some(named) = named else {
                            return refuse(at_member);
                        };
                        used.insert(constraint);
                        if on("same-anchor") && word(named, 0) == "parameter" && target(&named["body"]["members"][1]["value"]) != anchor {
                            return refuse(at_member);
                        }
                    }
                }
            }
            "time_invariant" => {
                if on("invariant-predicate") && !boolean(&target(&clause["body"]["members"][0]["value"])) {
                    return refuse("/members/0/value/target".into());
                }
                for (k, member) in clause["body"]["members"][1]["value"]["members"].as_array().unwrap().iter().enumerate() {
                    let constraint = target(member);
                    let at_member = format!("/members/1/value/members/{k}/target");
                    let named = node(&constraint).filter(|n| n["semantic_form"] == "clock_constraint");
                    let Some(named) = named else {
                        return refuse(at_member);
                    };
                    used.insert(constraint);
                    if on("invariant-bound") && !(word(named, 0) == "place" && ["lt", "le"].contains(&word(named, 4).as_str())) {
                        return refuse(at_member);
                    }
                }
            }
            "clock_reset" => {
                let anchor = target(&clause["body"]["members"][1]["value"]);
                if !anchor_ok(&anchor) {
                    return refuse("/members/1/value/target".into());
                }
                if word(clause, 0) == "place" {
                    if let Some(refusal) = place_use(at, clause, 2) {
                        return Some(refusal);
                    }
                    let place = node(&target(&clause["body"]["members"][2]["value"])).unwrap();
                    let first = place["body"]["members"][1]["value"]["members"][0]["members"][1]["value"]["value"].as_str().unwrap();
                    if on("frame-modifies") && !frame_fields(&anchor).contains(first) {
                        return refuse("/members/2/value/target".into());
                    }
                    let slots: Vec<OrderedId> = clause["body"]["members"][4]["value"]["members"].as_array().unwrap().iter().map(|s| target(s)).collect();
                    let mut previous: Option<usize> = None;
                    for (k, binder) in clause["body"]["members"][5]["value"]["members"].as_array().unwrap().iter().enumerate() {
                        let binder = target(binder);
                        let position = slots.iter().position(|slot| *slot == binder);
                        let parameter = node(&binder).is_some_and(|n| n["semantic_form"] == "parameter");
                        if on("binders") && !(parameter && position.is_some() && position > previous) {
                            return refuse(format!("/members/5/value/members/{k}/target"));
                        }
                        previous = position;
                    }
                    if on("condition") && !boolean(&target(&clause["body"]["members"][6]["value"])) {
                        return refuse("/members/6/value/target".into());
                    }
                } else {
                    if !parameter_ok(&anchor, &word(clause, 2)) {
                        return refuse("/members/2/value".into());
                    }
                    if on("frame-modifies") && frame_fields(&anchor).is_disjoint(&first_fields) {
                        return refuse("/members/2/value".into());
                    }
                }
            }
            _ => {}
        }
    }
    let mut seen = BTreeSet::new();
    for guard in &guards {
        let anchor = target(&node(guard).unwrap()["body"]["members"][0]["value"]);
        if on("one-guard") && !seen.insert(anchor) {
            return Some(("invalid_package", Some(REF), format!("/semantic_graph/nodes/{}/body/members/0/value/target", at_of(guard).unwrap())));
        }
    }
    for (k, constraint) in constraints.iter().enumerate() {
        if on("root-constraints") && !used.contains(constraint) {
            return Some(("invalid_package", Some(REF), format!("/semantic_graph/nodes/{root_at}/body/members/8/value/members/{k}/target")));
        }
    }
    None
}

/// The package a clause vector describes: the clause fixture with the case's
/// RFC 6902 patch applied.
fn timed_clause_case_package(case: &Value) -> Value {
    let mut package = parsed_fixture(POSITIVE_TIMED_CLAUSE_NODES);
    for operation in case.get("patch").and_then(Value::as_array).into_iter().flatten() {
        apply_json_patch(&mut package, operation);
    }
    package
}

/// Replays every case of the clause vectors: the patch applied to the clause
/// fixture, schema validity, current keys and package id, and the refusal.
fn timed_clause_vector_defects(skip: &str) -> Vec<String> {
    let vectors = parsed_fixture(TIMED_CLAUSE_NODE_VECTORS);
    let document = parsed_fixture(SELECTED_TIMED_CLAUSE_DOCUMENT);
    let validator = jsonschema::validator_for(&parsed_fixture(SCHEMA)).unwrap();
    let mut defects = Vec::new();
    for case in vectors["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let package = timed_clause_case_package(case);
        let valid = validator.is_valid(&package);
        if case["schema"] == "invalid" {
            if valid {
                defects.push(format!("{id}: schema-valid"));
            }
            continue;
        }
        // CL-03's anchor carries an `execution_point`, which the V2 schema's generic `SemanticTerm` body rule
        // rejects for every node (a gap that predates this change), so only that vector skips schema validity.
        if !valid && case["schema"] != "anchor-point-schema-gap" {
            defects.push(format!("{id}: schema-invalid"));
        }
        let got = timed_clause_refusal(&package, &document, skip);
        match case["expected"].as_str().unwrap() {
            "admitted" => {
                if got.is_some() {
                    defects.push(format!("{id}: refused {got:?}"));
                }
            }
            _ => {
                let want = (
                    case["expected_code"].as_str().unwrap().to_owned(),
                    case["expected_cause"].as_str().map(str::to_owned),
                    case["locus"].as_str().unwrap().to_owned(),
                );
                let got = got.map(|(code, cause, locus)| (code.to_owned(), cause.map(str::to_owned), locus));
                if got.as_ref() != Some(&want) {
                    defects.push(format!("{id}: {got:?} is not {want:?}"));
                }
                if !want.2.ends_with(case["locus_suffix"].as_str().unwrap()) {
                    defects.push(format!("{id}: locus suffix"));
                }
            }
        }
    }
    defects
}

#[trace("TC-367", "FR-322-AC-262", "FR-322-AC-263", "FR-322-AC-264", "FR-322-AC-265", "FR-342-AC-11")]
#[test]
fn tc_367_timed_clause_vectors_replay_to_their_refusals() {
    let vectors = parsed_fixture(TIMED_CLAUSE_NODE_VECTORS);
    let ids: Vec<&str> = vectors["cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap()).collect();
    let expected: Vec<String> = (1..=56).map(|n| format!("CL-{n:02}")).collect();
    assert_eq!(ids, expected.iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(timed_clause_vector_defects(""), Vec::<String>::new());
}

/// Each rule of the clause oracle is pinned by a published vector: switching
/// the rule off makes the replay disagree with exactly that vector.
#[trace("TC-367", "FR-322-AC-262", "FR-322-AC-263", "FR-322-AC-264", "FR-322-AC-265")]
#[test]
fn tc_367_timed_clause_oracle_fails_on_each_mutation() {
    let pinned = [
        ("anchor-kind", "CL-02"),
        ("no-point", "CL-03"),
        ("one-guard", "CL-04"),
        ("unlisted", "CL-05"),
        ("unlisted", "CL-26"),
        ("member-key", "CL-29"),
        ("root-constraints", "CL-06"),
        ("invariant-bound", "CL-07"),
        ("owner-type", "CL-08"),
        ("slot-count", "CL-09"),
        ("slot-domain", "CL-10"),
        ("frame-modifies", "CL-11"),
        ("binders", "CL-12"),
        ("condition", "CL-13"),
        ("parameter-declared", "CL-14"),
        ("parameter-clock", "CL-15"),
        ("nested-order", "CL-16"),
        ("same-anchor", "CL-21"),
        ("part-predicate", "CL-22"),
        ("invariant-predicate", "CL-23"),
        ("invariant-bound", "CL-24"),
        ("anchor-model", "CL-25"),
        ("state-anchor-types", "CL-54"),
        ("member-key", "CL-33"),
        ("member-key", "CL-35"),
    ];
    // Every switch but the path key (pinned by AO-08, tc_367_place_path_key_decides_a_shared_region) has a vector.
    let covered: BTreeSet<&str> = pinned.iter().map(|(rule, _)| *rule).chain(["path-key"]).collect();
    assert_eq!(covered, TIMED_CLAUSE_RULES.into_iter().collect::<BTreeSet<_>>());
    for (rule, vector) in pinned {
        let defects = timed_clause_vector_defects(rule);
        assert!(
            defects.iter().any(|defect| defect.starts_with(vector)),
            "switching off {rule} must be caught by {vector}: {defects:?}"
        );
    }
}

/// The two order readings of FR-322-AC-265: without the path key a shared
/// region is an equal key, so `AO-08` would be refused and `AO-09` and `AO-10`
/// would be refused at no distinguishing locus; with the key read from the
/// selector or the resolved path rows it would not be a function of the path.
#[trace("TC-367", "FR-322-AC-260", "FR-322-AC-265")]
#[test]
fn tc_367_place_path_key_decides_a_shared_region() {
    let vectors = parsed_fixture(TIMED_SLOT_SELECTOR_VECTORS);
    let selected = parsed_fixture(POSITIVE_TIMED_SLOT_NODES);
    let case_package = |id: &str| {
        let case = vectors["aggregate_order"]["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap();
        let mut package = selected.clone();
        for operation in case["patch"].as_array().unwrap() {
            apply_json_patch(&mut package, operation);
        }
        (package, case["locus"].as_str().map(str::to_owned))
    };
    let (ordered, _) = case_package("AO-08");
    assert_eq!(timed_aggregate_order_refusal(&ordered), None, "AO-08: a shared region in path order is admitted");
    assert!(timed_aggregate_order_refusal_ruled(&ordered, false, true, true).is_some(), "without the path key the shared region is an equal key");
    let (unordered, locus) = case_package("AO-09");
    assert_eq!(timed_aggregate_order_refusal(&unordered), locus, "AO-09");
    let (repeated, locus) = case_package("AO-10");
    assert_eq!(timed_aggregate_order_refusal(&repeated), locus, "AO-10: equal region and equal path");
    assert_eq!(timed_guard_and_place_cause(&repeated), Some(("invalid_package", "malformed-clock-place")), "AO-10 would otherwise be refused at the twin's forged row");
    // The key reads the path and not the declared rows: a place whose resolved rows are swapped keeps its place in the order.
    let mut rows = ordered.clone();
    let place_at = rows["semantic_graph"]["nodes"].as_array().unwrap().iter().position(|n| {
        n["semantic_form"] == "clock_place" && n["body"]["members"][1]["value"]["members"][0]["members"][1]["value"]["value"] == "b"
    }).unwrap();
    rows["semantic_graph"]["nodes"][place_at]["body"]["members"][2]["value"]["members"][0]["members"][1]["value"]["value"] = json!("zz");
    assert_eq!(timed_aggregate_order_refusal(&rows), None, "resolved_path does not enter the order key");
    // The slot step breaks a tie by collection, then minimum, then maximum, with `unbounded` last.
    let slot = |collection: &str, minimum: u64, maximum: Value| {
        json!({"body": {"members": [
            {"name": "model", "term": "binding", "value": {"term": "reference", "target": {"package": "absent", "path": ["x"], "ordinal": 0}}},
            {"name": "path", "term": "binding", "value": {"term": "aggregate", "members": [{"term": "aggregate", "members": [
                {"name": "kind", "term": "binding", "value": {"value": "slot"}},
                {"name": "collection", "term": "binding", "value": {"value": collection}},
                {"name": "selector", "term": "binding", "value": {"term": "reference"}},
                {"name": "minimum", "term": "binding", "value": {"value": minimum}},
                {"name": "maximum", "term": "binding", "value": {"value": maximum}}]}]}}]}})
    };
    let keys: Vec<_> = [
        slot("sequence", 0, json!(3)),
        slot("sequence", 0, json!("unbounded")),
        slot("sequence", 1, json!(2)),
        slot("set", 0, json!(1)),
    ]
    .iter()
    .map(timed_place_path_key)
    .collect();
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]), "slot keys ascend by collection, minimum, maximum with unbounded last");
}

/// The type node that selects a slot of `collection`: the Integer node for a
/// sequence-like collection, and the key type's node for a map, read from the
/// rendered `Map<K, V>` type of the step's resolved row. `None` for a key type
/// with no node in the package.
fn slot_domain_node(nodes: &[Value], collection: &str, map_type: &str) -> Option<Value> {
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
            .is_some_and(|want| selector["term"] == "reference" && selector["target"] == want)
    })
}

/// Index of the node a vectors file names, looked up by role in `node_indexes`.
fn timed_node_index(role: &str) -> usize {
    let vectors: Value = serde_json::from_str(TIMED_MODEL_NODE_VECTORS).unwrap();
    vectors["node_indexes"][role].as_u64().unwrap_or_else(|| panic!("node role {role}")) as usize
}

/// Every node id a node carries in its body, dependencies or type must name a
/// node of that vector's complete node document.
fn assert_semantic_node_refs_resolve(nodes: &[Value], label: &str) {
    fn visit(value: &Value, path: &str, nodes: &[Value], label: &str) {
        match value {
            Value::Object(fields) => {
                if is_node_id(value) {
                    assert!(
                        node_by_id(nodes, value).is_some(),
                        "{label}{path} references absent node {value}"
                    );
                    return;
                }
                for (name, child) in fields {
                    if name != "node_id" {
                        visit(child, &format!("{path}/{name}"), nodes, label);
                    }
                }
            }
            Value::Array(children) => {
                for (index, child) in children.iter().enumerate() {
                    visit(child, &format!("{path}/{index}"), nodes, label);
                }
            }
            _ => {}
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        visit(node, &format!("/{index}"), nodes, label);
    }
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
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let div = node_with_operation(&package, "quire.op.rational.div");
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


/// FR-322-AC-84: supply is checked entry by entry in lock order and the first
/// unsupplied entry is the only one reported.
#[trace("TC-233", "FR-322-AC-84")]
#[test]
fn tc_233_supply_refuses_the_first_failing_entry_in_lock_order() {
    let vectors = parsed_fixture(DEPENDENCY_SELECTION_VECTORS);
    let supply = vectors["supply_vectors"].as_array().expect("supply vectors");
    assert_eq!(supply.len(), 3);
    for vector in supply {
        let id = vector["id"].as_str().expect("id");
        let mut outcome = "admitted".to_owned();
        for (index, entry) in vector["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .enumerate()
        {
            if entry["supply"] == "absent" {
                outcome = format!(
                    "refused:missing_import/missing-selection@/lock/dependency_selections/{index}"
                );
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

/// The published `positive-all-families` package with `entries` as its
/// `dependency_selections`.
fn with_dependency_selections(entries: &Value) -> Value {
    let mut package = parsed_fixture(POSITIVE_ALL_FAMILIES);
    package["lock"]["dependency_selections"] = entries.clone();
    package
}

/// FR-322-AC-35 and FR-322-AC-84: a dependency selection is exactly
/// `{identity}`, the selections are strictly ascending by the identity's UTF-8
/// bytes, and the lock carries no model section.
#[trace("TC-233", "FR-322-AC-35")]
#[trace("TC-233", "FR-322-AC-84")]
#[test]
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
    let package = with_dependency_selections(entries);
    let errors: Vec<String> = schema
        .iter_errors(&package)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect();
    assert!(
        errors.is_empty(),
        "a package with dependency selections is admitted: {errors:?}"
    );

    for mutation in vectors["entry_mutations"].as_array().expect("entry mutations") {
        let id = mutation["id"].as_str().expect("mutation id is a string");
        let mut mutated = entries.clone();
        mutated[0] = mutation["entry"].clone();
        let candidate = with_dependency_selections(&mutated);
        let errors: Vec<String> = schema
            .iter_errors(&candidate)
            .map(|error| error.instance_path().to_string())
            .collect();
        assert!(!errors.is_empty(), "{id} must fail the public schema");
        assert!(
            errors
                .iter()
                .all(|path| path.starts_with("/lock/dependency_selections/0")),
            "{id} fails the schema only at the mutated entry: {errors:?}"
        );
    }

    let mut with_model = package.clone();
    with_model["lock"]["model_selections"] = json!([{"identity": "test/orders"}]);
    assert!(
        !schema.is_valid(&with_model),
        "a lock with a model_selections member must fail the public schema"
    );

    for vector in vectors["order_vectors"].as_array().expect("order vectors") {
        let id = vector["id"].as_str().expect("order vector id is a string");
        let selections = &vector["dependency_selections"];
        assert!(
            schema.is_valid(&with_dependency_selections(selections)),
            "{id} is admitted by the schema; the order rule, not the schema, decides it"
        );
        let identities = selections
            .as_array()
            .expect("selections")
            .iter()
            .map(|selection| selection["identity"].as_str().unwrap().as_bytes())
            .collect::<Vec<_>>();
        let mut outcome = "admitted".to_owned();
        let mut loci = Vec::new();
        for index in 1..identities.len() {
            if identities[index - 1] == identities[index] {
                outcome = "refused:invalid_package/conflicting-definition".to_owned();
                loci = vec![
                    format!("/lock/dependency_selections/{}", index - 1),
                    format!("/lock/dependency_selections/{index}"),
                ];
                break;
            }
            if identities[index - 1] > identities[index] {
                outcome = "refused:invalid_package/invalid-value".to_owned();
                loci = vec![format!("/lock/dependency_selections/{index}")];
                break;
            }
        }
        assert_eq!(vector["outcome"], outcome, "{id} order outcome");
        assert_eq!(vector["loci"], json!(loci), "{id} order loci");
    }
}

/// VT-01 to VT-04 (FR-322-AC-71): a `model`/`variant_type` node is a model
/// declaration node, as an `event_type` node is. With a `ModelOwner` and no
/// `declaration` the V2 schema admits it; with no `owner`, or with a
/// `SourceOwner` and no `declaration`, it fails the schema.
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

}

/// ET-01 to ET-04 (FR-322-AC-68): a `model`/`event_type` node is a model
/// declaration node. With a `ModelOwner` and no `declaration` the V2 schema
/// admits it; with no `owner`, with a `SourceOwner` and no `declaration`, or
/// with a `declaration` beside a `ModelOwner` it fails the schema.
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

}


/// FR-322-AC-140 and FR-322-AC-146: the wire cases over the fixture.
#[trace("TC-940", "FR-322-AC-140", "FR-322-AC-146")]
#[test]
fn tc_940_tuple_node_wire_cases_run_from_the_published_vectors() {
    let vectors = parsed_fixture(MODEL_TUPLE_UNION_VECTORS);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
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
                    ordered.sort_by(|(_, left), (_, right)| node_id_order(&left["node_id"], &right["node_id"]));
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
            let node_pointer = format!("/semantic_graph/nodes/{}", position_of(locus));
            assert!(
                schema
                    .iter_errors(&package)
                    .any(|error| error.instance_path().to_string().starts_with(&node_pointer)),
                "{id}: the V2 schema refuses the node"
            );
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
    assert_eq!(ids.len(), 26, "every published wire case ran");
}


#[trace("TC-367", "FR-322-AC-260")]
#[test]
fn tc_367_timed_aggregates_list_their_members_in_declared_order() {
    let vectors = parsed_fixture(TIMED_SLOT_SELECTOR_VECTORS);
    let selected = parsed_fixture(POSITIVE_TIMED_SLOT_NODES);
    let schema = parsed_fixture(SCHEMA);
    let validator = jsonschema::validator_for(&schema).unwrap();
    let section = &vectors["aggregate_order"];
    assert!(section["rule"].as_str().unwrap().starts_with("FR-322-AC-260, FR-322-AC-265"));
    assert_eq!(timed_aggregate_order_refusal(&selected), None, "the selected package is in declared order");
    // Declared order is not node-id order: the control's place listing is not sorted by node id.
    let places_of = |package: &Value| -> Vec<Value> {
        let root = &package["semantic_graph"]["nodes"][timed_node_index_of(package, "timed_model")];
        root["body"]["members"][3]["value"]["members"]
            .as_array().unwrap().iter().map(|entry| entry["target"].clone()).collect()
    };
    let control = places_of(&selected);
    let mut by_id = control.clone();
    by_id.sort_by(node_id_order);
    assert_ne!(control, by_id, "the published order is not the node-id order");
    // GD-02 appends its second place last: in order, so it refuses only as a repeated place.
    for gd in vectors["guards"].as_array().unwrap().iter().filter(|g| g["id"] == "GD-02") {
        let mut adverse = selected.clone();
        for operation in gd["patch"].as_array().unwrap() {
            apply_json_patch(&mut adverse, operation);
        }
        assert_eq!(timed_aggregate_order_refusal(&adverse), None, "GD-02 appends in order");
    }
    let cases = section["cases"].as_array().unwrap();
    let ids: Vec<_> = cases.iter().map(|case| case["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["AO-01", "AO-02", "AO-03", "AO-04", "AO-05", "AO-06", "AO-07", "AO-08", "AO-09", "AO-10"]);
    let (mut refused, mut admitted_patched) = (0, 0);
    for case in cases {
        let id = case["id"].as_str().unwrap();
        assert_eq!(case["fixture"], vectors["selected_package"], "{id}");
        let Some(patch) = case.get("patch") else {
            assert_eq!(case["expected"], "admitted", "{id}");
            continue;
        };
        if case["expected"] == "admitted" {
            let mut admitted = selected.clone();
            for operation in patch.as_array().unwrap() {
                apply_json_patch(&mut admitted, operation);
            }
            assert!(validator.is_valid(&admitted), "{id} is schema-valid");
            assert_eq!(timed_aggregate_order_refusal(&admitted), None, "{id} is in order");
            admitted_patched += 1;
            continue;
        }
        let mut adverse = selected.clone();
        for operation in patch.as_array().unwrap() {
            apply_json_patch(&mut adverse, operation);
        }
        assert_eq!(case["schema"], "valid");
        assert!(validator.is_valid(&adverse), "{id} is schema-valid");
        assert_eq!(case["expected"], "refused");
        assert_eq!(case["expected_code"], "invalid_semantic_graph");
        assert!(case["expected_cause"].is_null(), "{id} has no cause");
        let locus = timed_aggregate_order_refusal(&adverse).unwrap_or_else(|| panic!("{id} is refused"));
        assert_eq!(locus, case["locus"].as_str().unwrap(), "{id}");
        assert!(locus.ends_with(case["locus_suffix"].as_str().unwrap()), "{id}");
        refused += 1;
    }
    assert_eq!((refused, admitted_patched), (7, 2));
    // AO-03 is exactly the node-id-ordered listing.
    let id_case = cases.iter().find(|case| case["id"] == "AO-03").unwrap();
    let mut listed = selected.clone();
    for operation in id_case["patch"].as_array().unwrap() {
        apply_json_patch(&mut listed, operation);
    }
    assert_eq!(places_of(&listed), by_id);
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
    let mut ids = Vec::new();
    let (mut positives, mut refusals) = (0, 0);
    for case in vectors["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let node = &case["node"];
        assert_eq!(node["semantic_type"], node["node_id"], "{id} is self-typed");
        assert_eq!(node["dependencies"], json!(body_reference_targets(&node["body"])), "{id}");
        assert!(timed_schema.is_valid(node), "{id} matches the closed schema");
        assert!(!ids.contains(&node["node_id"]), "{id} node id is distinct");
        ids.push(node["node_id"].clone());
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
    assert_eq!((positives, refusals), (7, 4));

    // A `clock_place` has exactly `model`, `path` and `resolved_path`: no case carries a `guards` binding.
    for case in vectors["cases"].as_array().unwrap() {
        let names: Vec<_> = case["node"]["body"]["members"].as_array().unwrap().iter().map(|m| m["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["model", "path", "resolved_path"], "{}", case["id"]);
    }

    // GD-01: the published tick root has empty clock aggregates.
    let tick: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES).unwrap();
    let tick_root = &tick["semantic_graph"]["nodes"][timed_node_index_of(&tick, "timed_model")];
    for member in tick_root["body"]["members"].as_array().unwrap().iter().skip(3) {
        assert!(member["value"]["members"].as_array().unwrap().is_empty(), "{}", member["name"]);
    }
    assert_eq!(timed_guard_and_place_cause(&tick), None);

    // GD-02 and GD-03 replay from their published RFC 6902 patches (FR-322-AC-242).
    let selected: Value = serde_json::from_str(POSITIVE_TIMED_SLOT_NODES).expect("selected package JSON");
    assert_eq!(vectors["selected_package"], "fixtures/positive-timed-slot-nodes.json");
    assert_eq!(timed_guard_and_place_refusal(&selected), None, "the selected package is the control");
    let validator = jsonschema::validator_for(&schema).unwrap();
    let mut replayed = 0;
    for gd in vectors["guards"].as_array().unwrap().iter().filter(|g| g.get("patch").is_some()) {
        let mut adverse = selected.clone();
        for operation in gd["patch"].as_array().unwrap() {
            apply_json_patch(&mut adverse, operation);
        }
        assert_eq!(gd["fixture"], vectors["selected_package"], "{} patches the selected package", gd["id"]);
        assert_eq!(gd["schema"], "valid");
        assert!(validator.is_valid(&adverse), "{} is a schema-valid semantic control", gd["id"]);
        let (code, cause, locus) = timed_guard_and_place_refusal(&adverse).expect("the mutation is refused");
        assert_eq!(
            (code, cause, locus.as_str()),
            (
                gd["expected_code"].as_str().unwrap(),
                gd["expected_cause"].as_str().unwrap(),
                gd["locus"].as_str().unwrap()
            ),
            "{}",
            gd["id"]
        );
        let node_pointer = locus.split('/').take(4).collect::<Vec<_>>().join("/");
        assert_eq!(locus, format!("{node_pointer}/{}", gd["locus_suffix"].as_str().unwrap()), "{}", gd["id"]);
        replayed += 1;
    }
    assert_eq!(replayed, 2, "GD-02 and GD-03 are machine-readable patches");
}


#[trace("TC-367", "FR-322-AC-126", "FR-322-AC-127", "FR-322-AC-120")]
#[test]
fn tc_367_timed_model_semantic_refusals_are_replayable_packages() {
    let positive: Value = serde_json::from_str(POSITIVE_TIMED_MODEL_NODES).unwrap();
    let vectors: Value = serde_json::from_str(TIMED_MODEL_NODE_VECTORS).unwrap();
    let schema = parsed_fixture(SCHEMA);
    let validator = jsonschema::validator_for(&schema).unwrap();
    let rows = vectors["semantic_refusals"].as_array().unwrap();
    assert_eq!(rows.len(), 22);
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
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][3]["value"]["target"] =
                    json!({"package": "absent", "path": ["absent"], "ordinal": 0});
            }
            "function-as-tick-operation" => {
                adverse["semantic_graph"]["nodes"][44]["body"]["members"][3]["value"]["target"] =
                    adverse["semantic_graph"]["nodes"][5]["node_id"].clone();
            }
            "second-place-forged-row" => {
                // A DISTINCT second place of the same path (a clock place has no field but its
                // path, its model and its derived `resolved_path`, so only that table can differ:
                // a different body and key), listed after the first and starting after every
                // published place: in declared order, so only the repeated path is refused
                // (FR-322-AC-260 does not apply). Repeating one node reference would repeat
                // its order key and refuse as an order defect first.
                let root = if is_tick { 45 } else { 52 };
                let mut second = adverse["semantic_graph"]["nodes"][45].clone();
                let rows = second["body"]["members"][2]["value"]["members"].as_array_mut().unwrap();
                let identity = rows.last_mut().unwrap()["members"][2]["value"]["value"].as_str().unwrap().to_owned();
                rows.last_mut().unwrap()["members"][2]["value"]["value"] = json!(format!("{identity}#again"));
                second["dependencies"] = json!(body_reference_targets(&second["body"]));
                second["node_id"]["ordinal"] = json!(1);
                second["semantic_type"] = second["node_id"].clone();
                let id = second["node_id"].clone();
                let mut entry = adverse["source_map"].as_array().unwrap().iter()
                    .find(|entry| entry["node_id"] == adverse["semantic_graph"]["nodes"][45]["node_id"]).unwrap().clone();
                entry["node_id"] = id.clone();
                entry["regions"][0]["start"] = json!(9000);
                entry["regions"][0]["end"] = json!(9001);
                let nodes = adverse["semantic_graph"]["nodes"].as_array_mut().unwrap();
                let at = nodes.len() - 1;
                nodes.insert(at, second);
                let places = &mut nodes[root];
                places["body"]["members"][3]["value"]["members"]
                    .as_array_mut().unwrap().push(json!({"term": "reference", "target": id.clone()}));
                places["dependencies"].as_array_mut().unwrap().push(id);
                places["dependencies"].as_array_mut().unwrap().sort_by(node_id_order);
                adverse["source_map"].as_array_mut().unwrap().push(entry);
            }
            "repeated-clock-place-reference" => {
                // One node reference listed twice repeats its order key (FR-322-AC-260).
                let places = adverse["semantic_graph"]["nodes"][52]["body"]["members"][3]["value"]["members"]
                    .as_array_mut().unwrap();
                let d = places[0].clone();
                places.push(d);
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
                root["dependencies"].as_array_mut().unwrap().sort_by(node_id_order);
            }
            "source-unit-length" | "source-unit-derived-non-time" | "source-unit-derived-time" => {
                let mutation = row["mutation"].as_str().unwrap();
                let name = match mutation {
                    "source-unit-length" => "Length",
                    "source-unit-derived-non-time" => "Area",
                    _ => "Duration",
                };
                let time_id = adverse["semantic_graph"]["nodes"][42]["node_id"].clone();
                let named = |name: &str| json!({"package": "example", "path": ["Example", name], "ordinal": 0});
                let mut dimension = adverse["semantic_graph"]["nodes"][42].clone();
                dimension["declaration"]["qualified_name"] = json!(["Example", name]);
                let dimension_id = named(name);
                if name == "Area" || name == "Duration" {
                    let base_id = if name == "Area" {
                        let mut length = dimension.clone();
                        length["declaration"]["qualified_name"] = json!(["Example", "Length"]);
                        let id = named("Length");
                        length["node_id"] = id.clone();
                        length["semantic_type"] = id.clone();
                        adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(length);
                        id
                    } else { time_id };
                    dimension["dependencies"] = json!([base_id]);
                }
                dimension["node_id"] = dimension_id.clone();
                dimension["semantic_type"] = dimension_id.clone();
                adverse["semantic_graph"]["nodes"].as_array_mut().unwrap().push(dimension);
                let mut unit = adverse["semantic_graph"]["nodes"][43].clone();
                let unit_id = named("u");
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
                deps.sort_by(node_id_order);
            }
            other => panic!("unhandled semantic timed-model vector {other}"),
        }
        assert!(validator.is_valid(&adverse), "{} is a schema-valid semantic control", row["id"]);
        let expected = match row["mutation"].as_str().unwrap() {
            "remove-timed-root" => ("invalid_package", "missing-timed-model", "/semantic_graph/nodes"),
            "dangling-tick-operation" | "function-as-tick-operation" => (
                "invalid_package", "timed-node-reference", "/semantic_graph/nodes/44/body/members/3/value/target",
            ),
            "second-place-forged-row" => (
                "invalid_package", "malformed-clock-place", "/semantic_graph/nodes/59/body/members/2/value/members/1",
            ),
            "repeated-clock-place-reference" => (
                "invalid_semantic_graph", "", "/semantic_graph/nodes/52/body/members/3/value/members/1",
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
        let expected_cause = if expected.1.is_empty() { Value::Null } else { json!(expected.1) };
        assert_eq!(row["expected_cause"], expected_cause);
        assert_eq!(row["locus"], expected.2);
        if row["mutation"] == "repeated-clock-place-reference" {
            // The same node listed twice is refused as an order defect with no cause, at the repeated member.
            assert_eq!(
                timed_aggregate_order_refusal(&adverse).as_deref(),
                Some(row["locus"].as_str().unwrap()),
                "TM-30 repeats an order key, so the AC-260 order refusal is at the repeated member"
            );
            let places = adverse["semantic_graph"]["nodes"][52]["body"]["members"][3]["value"]["members"].as_array().unwrap();
            assert_eq!(places[0], places[1], "TM-30 lists one node reference twice");
        }
        if row["mutation"] == "second-place-forged-row" {
            assert_eq!(timed_aggregate_order_refusal(&adverse), None, "TM-12 is in declared order, so only the forged row of the second node of the path is refused");
            let places = adverse["semantic_graph"]["nodes"][52]["body"]["members"][3]["value"]["members"].as_array().unwrap();
            assert_ne!(places[0], places[1], "the second node of the path is a distinct node, not a repeated reference");
            assert_eq!(
                timed_guard_and_place_refusal(&adverse),
                Some(("invalid_package", "malformed-clock-place", row["locus"].as_str().unwrap().to_owned())),
                "TM-12: the row join of the second node precedes the root's repeated-path check"
            );
        }
    }
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
    let tick_package: Value = serde_json::from_str(POSITIVE_TICK_TIMED_MODEL_NODES)
        .expect("tick timed fixture JSON");
    assert!(package_validator.is_valid(&tick_package));
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
        let dimension = &unit["semantic_type"];
        assert!(
            unit["dependencies"].as_array().unwrap().contains(dimension),
            "unit node {index} depends on its dimension"
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
        assert_eq!(
            node["dependencies"],
            json!(body_reference_targets(&node["body"])),
            "{name} dependencies match its resolved body references"
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


/// FR-322 "Integer literal operands": the integer operators' integer literal
/// operands, typed at the builtin Integer node, are inline `literal` terms in
/// every published producer-output package (`positive_fixtures()`, which omits
/// the reader-admission fixture `positive-integer-literal-references.json`).
#[trace("TC-233", "FR-322-AC-92")]
#[test]
fn tc_233_integer_operator_literal_operands_are_inline_in_the_published_packages() {
    let operators = [
        "quire.op.integer.add",
        "quire.op.integer.sub",
        "quire.op.integer.mul",
        "quire.op.integer.negate",
        "quire.op.integer.eq",
        "quire.op.integer.ne",
    ];
    
    let mut inline = BTreeMap::<String, usize>::new();
    for (name, package) in positive_fixtures() {
        let nodes = package["semantic_graph"]["nodes"].as_array().expect("nodes");
        let integer = nodes
            .iter()
            .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
            .map(|node| node["node_id"].clone());
        for node in nodes.iter().filter(|node| is_application_node(node)) {
            let operation = node["body"]["operation"]["identity"].as_str().unwrap_or_default();
            if !operators.contains(&operation) {
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
    let adverse = parsed_fixture(ADVERSE);
    let cases = adverse["population_reference_cases"]
        .as_array()
        .expect("cases");
    assert_eq!(cases.len(), 15);
    let mut admitted = BTreeMap::<String, Value>::new();
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
            candidate["model_bindings"] = json!({"domain_package": document});
            let placeholder = json!({
                "package": candidate["name"],
                "path": ["relation", "population", case["owner_node"].as_str().unwrap_or("unowned")],
                "ordinal": 0
            });
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
            candidate["semantic_graph"]["nodes"]
                .as_array_mut()
                .expect("fixture nodes are an array")
                .push(node);
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
                population_id.clone(),
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
                    let root = model_root_positions(&candidate)[0];
                    assert_eq!(
                        locus,
                        format!("/semantic_graph/nodes/{root}/owner"),
                        "{id}: the model_root owner"
                    );
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
    assert_ne!(counters, archive, "two populations are two nodes");
}


/// FR-341-AC-15, FR-341-AC-17, FR-464-AC-19: the published transition package
/// carries one Boolean model-field `state`/`transition` node whose body is the
/// five-binding `quire.op.state.transition` application. The closed
/// schema refuses each body mutation, the operation step admits it
/// and refuses another profile operator, and the
/// transition depends on the anchor, subject, pre and post nodes.
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
    unselected
        .pointer_mut("/lock/profile_selections")
        .and_then(Value::as_array_mut)
        .expect("profile selections")
        .retain(|row| row["role"] != "protocol_profile");
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

    // The transition depends on the anchor, subject, pre and post nodes its
    // five bindings name, and a source region is outside every node id.
    let dependencies = package["semantic_graph"]["nodes"][transition]["dependencies"]
        .as_array()
        .expect("dependencies");
    for index in [0, 1, 3, 4] {
        let argument = &arguments[index]["value"];
        let targets = if argument["term"] == "reference" {
            vec![argument["target"].clone()]
        } else {
            argument["members"]
                .as_array()
                .expect("members")
                .iter()
                .map(|member| member["target"].clone())
                .collect()
        };
        for target in targets {
            assert!(dependencies.contains(&target), "the transition depends on {target}");
        }
    }
    let ids = |candidate: &Value| {
        candidate["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .map(|node| OrderedId(node["node_id"].clone()))
            .collect::<BTreeSet<_>>()
    };
    let mut regions = package.clone();
    regions["source_map"][0]["regions"][0]["start"] = json!(40);
    assert_eq!(ids(&regions), ids(&package), "a source region moves no node id");
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

/// FR-322-AC-23: an application's `result_type` and a literal's `type` are
/// checked against the operation and the value kind, not inferred: a root
/// `result_type` differing from the node's `semantic_type`, a result type the
/// operation cannot produce and an inline literal whose type disagrees with
/// its value kind each refuse `ill_typed`/`operator-ineligible`.
#[trace("TC-233", "FR-322-AC-23")]
#[test]
fn tc_233_application_result_types_and_literal_types_are_checked_not_inferred() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let package = parsed_fixture(POSITIVE_OPERATION_IDENTITIES);
    let div = node_index_where(&package, |node| {
        node["body"]["operation"]["identity"] == "quire.op.rational.div"
    });
    let id = |role: &str| {
        let (tag, form) = match role {
            "rational" => ("scalar_type", "rational"),
            "text" => ("scalar_type", "text"),
            "range" => ("bounded_domain", "integer_range"),
            other => panic!("unknown operation-identities role {other}"),
        };
        published_node_id(&package, tag, form, None)
    };
    let with = |edit: &dyn Fn(&mut Value)| {
        let mut candidate = package.clone();
        edit(&mut candidate);
        package_operations_refusal(&candidate, &catalog)
    };
    assert_eq!(package_operations_refusal(&package, &catalog), Ok(()));
    assert_eq!(
        with(&|candidate| {
            candidate["semantic_graph"]["nodes"][div]["body"]["result_type"] = id("text");
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

/// FR-322-AC-54: the `correspondence`/`abstraction_relation` node is left to
/// FR-451, whose identity step refuses an edited body as
/// `invalid_semantic_graph`.
#[trace("TC-233", "FR-322-AC-54")]
#[test]
fn tc_233_an_abstraction_relation_node_is_left_to_fr_451() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_CLAUSE_OPERATIONS);
    assert!(schema.is_valid(&package));
    let index = node_index_where(&package, |node| {
        node["semantic_form"] == "abstraction_relation"
    });
    let node = &package["semantic_graph"]["nodes"][index];
    assert!(node.get("owner").is_none());
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

/// FR-322-AC-151: the selected document of `acme/shapes` is a full FCD
/// semantic IR; the vectors' `domain_package` is exactly its projection.
#[trace("TC-940", "FR-322-AC-151")]
#[test]
fn tc_940_the_selected_document_is_a_full_semantic_ir_whose_projection_is_derived_from_it() {
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
    let root = package["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["semantic_form"] == "model_root")
        .expect("model_root");
    assert_eq!(root["owner"]["identity"], document["package"]["identity"]);
    // Swapping a position type in the document changes the document.
    let mut swapped = document.clone();
    let pair = swapped["types"]
        .as_array_mut()
        .expect("types")
        .iter_mut()
        .find(|declared| declared["identity"] == "ix://acme/shapes/Pair")
        .expect("Pair");
    pair["positions"].as_array_mut().expect("positions").swap(0, 1);
    assert_ne!(swapped, document);
    // The projection lists follow document order, and reordering two declarations changes the document.
    let mut reordered = document.clone();
    reordered["types"].as_array_mut().expect("types").swap(1, 2);
    assert_ne!(reordered, document);
    assert_ne!(shapes_project_document(&reordered), projection);
}

/// FR-322-AC-261, FR-322-AC-266: the clause fixture meets the V2 schema, is
/// admitted by the clause oracle, every clause node carries exactly the
/// binding names the published `forms` give, and every guard, reset,
/// invariant and constraint is listed by the timed root.
#[trace("TC-367", "FR-322-AC-261", "FR-322-AC-266")]
#[test]
fn tc_367_timed_clause_fixture_is_closed_and_selected() {
    let vectors = parsed_fixture(TIMED_CLAUSE_NODE_VECTORS);
    let package = parsed_fixture(POSITIVE_TIMED_CLAUSE_NODES);
    let document = parsed_fixture(SELECTED_TIMED_CLAUSE_DOCUMENT);
    let validator = jsonschema::validator_for(&parsed_fixture(SCHEMA)).unwrap();
    assert!(validator.is_valid(&package), "the clause fixture meets the V2 schema");
    assert_eq!(timed_clause_refusal(&package, &document, ""), None, "CL-01 is admitted");
    let names = |node: &Value| -> Vec<String> {
        node["body"]["members"].as_array().unwrap().iter().map(|member| member["name"].as_str().unwrap().to_owned()).collect()
    };
    let published = |value: &Value| -> Vec<String> { value.as_array().unwrap().iter().map(|n| n.as_str().unwrap().to_owned()).collect() };
    let forms = &vectors["forms"];
    let mut seen = BTreeSet::new();
    for node in package["semantic_graph"]["nodes"].as_array().unwrap() {
        let kind = node["body"]["members"][0]["value"]["value"].as_str().unwrap_or("");
        let expected = match (node["semantic_form"].as_str().unwrap(), kind) {
            ("clock_constraint", "place") => &forms["clock_constraint"]["kind_place"],
            ("clock_constraint", "parameter") => &forms["clock_constraint"]["kind_parameter"],
            ("clock_reset", "place") => &forms["clock_reset"]["kind_place"],
            ("clock_reset", "parameter") => &forms["clock_reset"]["kind_parameter"],
            ("clock_guard", _) => &forms["clock_guard"]["body"],
            ("time_invariant", _) => &forms["time_invariant"]["body"],
            _ => continue,
        };
        assert_eq!(names(node), published(expected), "{} {kind}", node["semantic_form"]);
        seen.insert((node["semantic_form"].as_str().unwrap().to_owned(), kind.to_owned()));
        if node["semantic_form"] == "clock_guard" {
            for part in node["body"]["members"][1]["value"]["members"].as_array().unwrap() {
                let members = part["members"].as_array().unwrap();
                assert_eq!(members[0]["name"], "predicate");
                assert!(members.len() >= 2 && members[1..].iter().all(|m| m["term"] == "reference"), "a part is a predicate then bare constraint references");
            }
        }
        if node["semantic_form"] == "clock_constraint" {
            let comparison = node["body"]["members"].as_array().unwrap().iter().find(|m| m["name"] == "comparison").unwrap();
            assert!(published(&forms["clock_constraint"]["comparison"]).contains(&comparison["value"]["value"].as_str().unwrap().to_owned()));
        }
    }
    assert_eq!(seen.len(), 6, "every clause form in both kinds is published: {seen:?}");
    for (role, form, kind) in [
        ("clock_constraint_place", "clock_constraint", "place"),
        ("clock_constraint_parameter", "clock_constraint", "parameter"),
        ("clock_reset_place", "clock_reset", "place"),
        ("clock_reset_parameter", "clock_reset", "parameter"),
    ] {
        for at in vectors["node_indexes"][role].as_array().unwrap() {
            let node = &package["semantic_graph"]["nodes"][at.as_u64().unwrap() as usize];
            assert_eq!((node["semantic_form"].as_str().unwrap(), node["body"]["members"][0]["value"]["value"].as_str().unwrap()), (form, kind), "{role}");
        }
    }
    for (role, form) in [("clock_guard", "clock_guard"), ("time_invariant", "time_invariant"), ("operation_anchor", "operation_anchor"), ("frame", "frame")] {
        for at in vectors["node_indexes"][role].as_array().unwrap() {
            assert_eq!(package["semantic_graph"]["nodes"][at.as_u64().unwrap() as usize]["semantic_form"], form, "{role}");
        }
    }
    let root = &package["semantic_graph"]["nodes"][timed_node_index_of(&package, "timed_model")];
    let listed: Vec<Value> = ["resets", "guards", "invariants", "constraints"]
        .iter()
        .flat_map(|name| {
            let member = root["body"]["members"].as_array().unwrap().iter().find(|m| m["name"] == *name).unwrap();
            member["value"]["members"].as_array().unwrap().iter().map(|entry| entry["target"].clone()).collect::<Vec<_>>()
        })
        .collect();
    for node in package["semantic_graph"]["nodes"].as_array().unwrap() {
        if ["clock_guard", "clock_reset", "time_invariant", "clock_constraint"].contains(&node["semantic_form"].as_str().unwrap()) {
            assert!(listed.contains(&node["node_id"]), "{} is listed", node["semantic_form"]);
        }
    }
}

/// FR-322-AC-267: the three excluded source forms are published as a rule
/// with their refused and built controls, and the reader rows of the
/// exclusions patch the node they are refused at.
#[trace("TC-367", "FR-322-AC-267", "FR-415-AC-25")]
#[test]
fn tc_367_excluded_clause_forms_are_published() {
    let vectors = parsed_fixture(TIMED_CLAUSE_NODE_VECTORS);
    let unbuilt = &vectors["excluded_forms"];
    assert!(unbuilt.get("ticket").is_none(), "an exclusion is a rule and names no ticket");
    assert_eq!(unbuilt["rule"], "FR-322-AC-267");
    let ids: Vec<&str> = unbuilt["cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["GJ-01", "GJ-02", "GJ-03", "CW-01", "CW-02", "CW-03", "CW-04", "CW-05", "CW-06", "SD-01", "SD-02"]);
    let case_ids: BTreeSet<&str> = vectors["cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap()).collect();
    let package = parsed_fixture(POSITIVE_TIMED_CLAUSE_NODES);
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    for case in unbuilt["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        match case["expected"].as_str().unwrap() {
            "refused" => {
                assert_eq!(case["expected_code"], "unsupported_construct", "{id}");
                assert_eq!(case["expected_cause"], "expression-form", "{id}");
                match case["reader_row"].as_str() {
                    Some(row) => assert!(case_ids.contains(row), "{id}: reader row {row} is a published case"),
                    None => assert!(case["reader_row_note"].is_string(), "{id}: no reader row is stated and why"),
                }
            }
            "built" => {
                if case.get("node_index").is_none() {
                    assert_eq!(id, "GJ-03", "only GJ-03 has no package row of its own");
                    assert!(case["package_note"].is_string(), "{id}: says why it has no package row");
                    continue;
                }
                assert_eq!(case["package_case"], "CL-01", "{id}");
                let node = &nodes[case["node_index"].as_u64().unwrap() as usize];
                let expect = &case["built_node"];
                assert_eq!(node["semantic_form"], expect["form"], "{id}: the control is a built {}", expect["form"]);
                let bound = |name: &str| node["body"]["members"].as_array().unwrap().iter().find(|m| m["name"] == name).unwrap()["value"].clone();
                if expect["form"] == "clock_guard" {
                    let parts = bound("parts")["members"].as_array().unwrap().clone();
                    assert_eq!(parts.len() as u64, expect["parts"].as_u64().unwrap(), "{id}: parts");
                    assert!(parts.iter().all(|part| part["members"].as_array().unwrap().len() >= 2), "{id}: every part holds a clock constraint");
                } else {
                    let place_id = bound("place")["target"].clone();
                    let place = nodes.iter().find(|n| n["node_id"] == place_id).unwrap();
                    let first = place["body"]["members"][1]["value"]["members"][0]["members"][1]["value"]["value"].as_str().unwrap();
                    assert_eq!(first, expect["place_field"].as_str().unwrap(), "{id}: the reset writes the place that begins with the field");
                    assert_eq!(bound("binders")["members"].as_array().unwrap().len() as u64, expect["binders"].as_u64().unwrap(), "{id}: binders");
                }
            }
            other => assert_eq!((other, id), ("no-node", "CW-05")),
        }
    }
    for (id, expected, locus) in [
        ("CL-20", "refused-by-schema", "/semantic_graph/nodes/86/body"),
        ("CL-55", "refused-by-schema", "/semantic_graph/nodes/82/body"),
        ("CL-56", "refused", "/semantic_graph/nodes/84/body/members/4/value"),
    ] {
        let case = vectors["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap();
        assert_eq!(case["expected"], expected, "{id}");
        assert_eq!(case["locus"], locus, "{id} locus");
        let touched = |prefix: &str| case["patch"].as_array().unwrap().iter().any(|op| op["path"].as_str().unwrap().starts_with(prefix));
        let node = locus.split('/').nth(3).unwrap();
        assert!(touched(&format!("/semantic_graph/nodes/{node}/")), "{id}: the patch changes the node it is refused at");
    }
}

/// FR-322-AC-53: the same declarations under two packages are two packages:
/// both are admitted, every node a package declares carries its own package
/// name in its node id, and the two graphs are structurally unequal.
#[trace("TC-233", "FR-322-AC-53")]
#[test]
fn tc_233_two_owners_differ_by_package_in_every_node_id() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let a = parsed_fixture(POSITIVE_TWO_OWNERS_A);
    let b = parsed_fixture(POSITIVE_TWO_OWNERS_B);
    for (name, package) in [("a", &a), ("b", &b)] {
        assert!(schema.is_valid(package), "{name} is a V2 package");
        assert_eq!(read_package_semantics(package, &catalog), Ok(()), "{name}");
        for node in package["semantic_graph"]["nodes"].as_array().expect("nodes") {
            assert_eq!(node["node_id"]["package"], package["name"], "{name}: ids carry the package");
        }
    }
    assert_ne!(a["name"], b["name"]);
    let declared = |package: &Value, name: &str| -> Vec<Value> {
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|node| node["declaration"]["qualified_name"] == json!([name]))
            .map(|node| node["node_id"].clone())
            .collect()
    };
    for name in ["Point", "List", "three"] {
        let (left, right) = (declared(&a, name), declared(&b, name));
        assert_eq!(left.len(), 1, "{name} is declared once");
        assert_eq!(right.len(), 1, "{name} is declared once");
        assert_ne!(left, right, "{name} ids differ, since their package differs");
        assert_eq!(left[0]["path"], right[0]["path"], "{name}: the structural path is shared");
    }
    assert_ne!(a, b, "the two packages are structurally unequal");
}

/// A validator for one `$defs` entry of the V2 schema.
fn def_validator(name: &str) -> jsonschema::Validator {
    let defs = parsed_fixture(SCHEMA)["$defs"].clone();
    jsonschema::validator_for(&json!({"$ref": format!("#/$defs/{name}"), "$defs": defs}))
        .expect("def schema")
}

/// FR-322-AC-36: a `dependency_reference` term carries a nonempty dependency
/// package name and a structural `NodeId`; an empty package, a missing node
/// and an extra member each fail the schema.
#[trace("TC-233", "FR-322-AC-36")]
#[test]
fn tc_233_a_dependency_reference_term_carries_package_and_node_id() {
    let validator = def_validator("SemanticTerm");
    let term = json!({
        "term": "dependency_reference",
        "package": "test/predicates",
        "node": {"package": "test/predicates", "path": ["scalar_type", "integer"], "ordinal": 0}
    });
    assert!(validator.is_valid(&term));
    let mut empty = term.clone();
    empty["package"] = json!("");
    assert!(!validator.is_valid(&empty), "an empty package fails");
    let mut missing = term.clone();
    missing.as_object_mut().unwrap().remove("node");
    assert!(!validator.is_valid(&missing), "a missing node fails");
    let mut extra = term.clone();
    extra["extra"] = json!(1);
    assert!(!validator.is_valid(&extra), "an extra member fails");
}

/// FR-322-AC-131: `ExecutionPoint` has exactly the four variants; each
/// rejects unknown tags, missing and extra members and empty names.
#[trace("TC-233", "FR-322-AC-131")]
#[test]
fn tc_233_execution_point_has_exactly_four_closed_variants() {
    let validator = def_validator("ExecutionPoint");
    for (kind, member) in [
        ("initialization", "name"),
        ("handler", "name"),
        ("pre", "operation"),
        ("post", "operation"),
    ] {
        let point = json!({"kind": kind, member: "start"});
        assert!(validator.is_valid(&point), "{kind} is admitted");
        let mut empty = point.clone();
        empty[member] = json!("");
        assert!(!validator.is_valid(&empty), "{kind}: an empty name fails");
        let mut extra = point.clone();
        extra["extra"] = json!(1);
        assert!(!validator.is_valid(&extra), "{kind}: an extra member fails");
        assert!(
            !validator.is_valid(&json!({"kind": kind})),
            "{kind}: a missing member fails"
        );
    }
    assert!(!validator.is_valid(&json!({"kind": "other", "name": "start"})));
}

/// FR-322-AC-200: a `relation`/`refinement` node's closed `RefinementBody`
/// requires `name`, an `Identifier`.
#[trace("TC-310", "FR-322-AC-200")]
#[test]
fn tc_310_refinement_body_requires_an_identifier_name() {
    let defs = parsed_fixture(SCHEMA)["$defs"].clone();
    let body = &defs["RefinementBody"];
    assert!(body["required"].as_array().unwrap().contains(&json!("name")));
    assert_eq!(body["properties"]["name"]["$ref"], "#/$defs/Identifier");
    assert_eq!(body["additionalProperties"], false);
    let validator = def_validator("RefinementBody");
    assert!(!validator.is_valid(&json!({"term": "refinement"})), "a body without name fails");
}


/// Every node body of every published V2 package is in the closed
/// five-stratum body grammar.
#[trace("TC-427", "FR-322-AC-39")]
#[test]
fn tc_427_published_bodies_follow_the_five_stratum_body_grammar() {
    for (label, package) in published_v2_packages() {
        for node in package["semantic_graph"]["nodes"]
            .as_array()
            .expect("the graph has nodes")
        {
            assert!(
                is_body_term(&node["body"]),
                "{label}: node {} has a body in the grammar",
                node["node_id"]
            );
        }
    }
}


/// FR-272-AC-7: every package-read refusal cause of the operation, declaration
/// and frame steps pairs with exactly one catalogued code in the Diagnostic
/// schema, and no two refusals share a cause.
#[trace("TC-233", "FR-272-AC-7")]
#[test]
fn tc_233_operation_and_declaration_causes_pair_with_one_catalogued_code() {
    let diagnostic = schema_for_def(&parsed_fixture(SCHEMA), "Diagnostic");
    let codes = ["invalid_package", "ill_typed", "ambiguous_declaration"];
    let pairs = OperationRefusal::ALL
        .into_iter()
        .map(|refusal| (refusal.cause(), refusal.code()))
        .chain([("ambiguous-name", "ambiguous_declaration")])
        .chain(
            [
                FrameRefusal::MissingName,
                FrameRefusal::MalformedDeclaration,
                FrameRefusal::ConflictingBinding,
            ]
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
        for row in codes {
            let entry = json!({"stage": "package_read", "code": row, "cause_tag": cause, "details": [], "loci": []});
            assert_eq!(
                diagnostic.is_valid(&entry),
                row == code,
                "the Diagnostic schema pairs {cause} with `{row}` exactly when it is its code"
            );
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
                node["node_id"]
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
    ambiguous["semantic_graph"]["nodes"][call] = node;
    v2.validate(&ambiguous)
        .unwrap_or_else(|error| panic!("a duplicated name is schema-valid: {error}"));
    let later = [call, callee]
        .into_iter()
        .map(|index| ambiguous["semantic_graph"]["nodes"][index]["node_id"].clone())
        .max_by(|left, right| node_id_order(left, right))
        .expect("two declaring nodes");
    assert_eq!(
        package_declaration_refusal(&ambiguous),
        Err((DeclarationRefusal::AmbiguousName, later)),
        "the second declaring node in node-id order refuses as ambiguous"
    );
}


#[trace("TC-966", "FR-481-AC-1", "FR-481-AC-2", "FR-481-AC-6")]
#[test]
fn tc_966_hybrid_model_nodes_meet_the_schema_and_are_self_typed() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let nodes = vectors["nodes"].as_array().expect("nodes");
    let mut positive_document = nodes.clone();
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
    let ids = nodes
        .iter()
        .map(|node| OrderedId(node["node_id"].clone()))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), nodes.len(), "every node id is unique");
    for (index, node) in nodes.iter().enumerate() {
        assert!(
            node_schema.is_valid(node),
            "node {index} meets SemanticNode"
        );
        if node["node_tag"] == "hybrid" {
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

#[trace("TC-966", "FR-481-AC-8", "FR-481-AC-9")]
#[test]
fn tc_966_restored_hybrid_forms_meet_the_schema_and_are_self_typed() {
    let vectors = parsed_fixture(HYBRID_NODE_VECTORS);
    let base = vectors["nodes"].as_array().expect("nodes");
    let restored = vectors["restored_nodes"].as_array().expect("restored nodes");
    let roles = &vectors["restored_roles"];
    let all = base.iter().chain(restored).cloned().collect::<Vec<_>>();
    let node_schema = schema_for_def(&parsed_fixture(SCHEMA), "SemanticNode");
    let ids = all
        .iter()
        .map(|node| OrderedId(node["node_id"].clone()))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), all.len(), "every node id is unique");
    for (index, node) in restored.iter().enumerate() {
        assert!(node_schema.is_valid(node), "restored node {index} meets SemanticNode");
        if node["node_tag"] == "hybrid" {
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

/// FR-322-AC-244: a present option is a value node with an identity-bound
/// payload reference; it has no operator identity.
#[trace("TC-971", "FR-322-AC-244")]
#[test]
fn tc_971_option_value_body_is_closed_and_binds_type_and_payload() {
    let schema_document: Value = serde_json::from_str(SCHEMA).expect("published schema is JSON");
    let node_schema = json!({
        "$schema": schema_document["$schema"],
        "$defs": schema_document["$defs"],
        "$ref": "#/$defs/SemanticNode"
    });
    let schema = jsonschema::validator_for(&node_schema).expect("semantic-node schema is valid");
    let vectors = parsed_fixture(OPTION_VALUE_VECTORS);
    let fixture = parsed_fixture(POSITIVE_INTEGER_LITERAL_REFERENCES);
    let boolean_fixture = parsed_fixture(POSITIVE_CONTROL_OPERATIONS);
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let template = fixture["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture nodes are an array")
        .iter()
        .find(|node| node["node_tag"] == "value" && node["semantic_form"] == "literal")
        .expect("fixture has a literal node")
        .clone();
    let select = |from: &Value, selector: &Value| {
        from["semantic_graph"]["nodes"]
            .as_array()
            .expect("fixture nodes are an array")
            .iter()
            .filter(|node| {
                node["node_tag"] == selector["node_tag"]
                    && node["semantic_form"] == selector["semantic_form"]
                    && selector
                        .get("value_kind")
                        .is_none_or(|kind| node["body"]["value_kind"] == *kind)
                    && selector
                        .get("value")
                        .is_none_or(|value| node["body"]["value"] == *value)
            })
            .nth(selector["ordinal"].as_u64().expect("selector ordinal") as usize)
            .unwrap_or_else(|| panic!("fixture node selector resolves: {selector}"))
            .clone()
    };
    let package_name = fixture["name"].clone();
    let build_node = |type_id: &Value, payload_id: &Value| {
        let id = json!({
            "package": package_name,
            "path": ["value", "option_value", type_id, payload_id],
            "ordinal": 0
        });
        let mut node = template.clone();
        node["node_tag"] = json!("value");
        node["semantic_form"] = json!("option_value");
        node["semantic_type"] = type_id.clone();
        node["node_id"] = id.clone();
        node["body"] = json!({"term":"aggregate","members":[{"term":"binding","name":"value","value":{"term":"reference","target":payload_id}}]});
        node["dependencies"] = json!([payload_id]);
        (node, id)
    };
    let build_option_type = |element_type_id: &Value| {
        let id = json!({
            "package": package_name,
            "path": ["composite_type", "option", element_type_id],
            "ordinal": 0
        });
        let node = json!({
            "schema_version":"quire.checked-semantic-graph/v2",
            "node_tag":"composite_type",
            "semantic_form":"option",
            "node_id":id,
            "semantic_type":id,
            "occurrences":[{"role":"generated","ordinal":0}],
            "dependencies":[element_type_id],
            "body":{"term":"aggregate","members":[{"term":"reference","target":element_type_id}]}
        });
        (node, id)
    };
    let install = |package: &mut Value, node: Value| {
        let nodes = package["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("package nodes are an array");
        nodes.push(node);
        nodes.sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
    };

    let positives = vectors["positive"]
        .as_array()
        .expect("positive vectors are an array");
    let mut observed_ids = BTreeSet::new();
    let mut valid_package = None;
    let mut selected_type = None;
    let mut selected_payload = None;
    for vector in positives {
        let element_type = select(&fixture, &vector["element_type_selector"]);
        let payload_node = select(&fixture, &vector["payload_selector"]);
        let (type_node, _) = build_option_type(&element_type["node_id"]);
        let (node, id) = build_node(&type_node["node_id"], &payload_node["node_id"]);
        assert!(
            observed_ids.insert(OrderedId(id)),
            "positive ids are distinct"
        );
        assert!(
            schema.is_valid(&node),
            "{} is a closed option-value node",
            vector["id"]
        );
        let mut package = fixture.clone();
        install(&mut package, type_node.clone());
        install(&mut package, node);
        assert_eq!(
            read_package_semantics(&package, &catalog),
            Ok(()),
            "{} is admitted by the strict reader",
            vector["id"]
        );
        if vector["id"] == "OV-01" {
            valid_package = Some(package);
            selected_type = Some(type_node);
            selected_payload = Some(payload_node);
        }
    }
    assert_eq!(
        observed_ids.len(),
        2,
        "payload identity changes option identity"
    );

    let base_package = valid_package.expect("OV-01 package");
    let type_node = selected_type.expect("OV-01 option type");
    let payload_node = selected_payload.expect("OV-01 payload");
    for refusal in vectors["schema_refusals"]
        .as_array()
        .expect("schema refusal vectors are an array")
    {
        let (mut node, _) = build_node(&type_node["node_id"], &payload_node["node_id"]);
        let binding = &mut node["body"]["members"];
        match refusal["mutation"].as_str() {
            Some("empty-members") => *binding = json!([]),
            Some("wrong-binding-name") => binding[0]["name"] = json!("payload"),
            Some("inline-literal") => binding[0]["value"] = payload_node["body"].clone(),
            Some("multiple-bindings") => {
                let members = binding.as_array_mut().expect("binding members are an array");
                members.push(members[0].clone());
            }
            other => panic!("unknown schema mutation: {other:?}"),
        }
        assert!(
            !schema.is_valid(&node),
            "{} is refused by OptionValueBody",
            refusal["id"]
        );
    }

    let reader_refusals = vectors["reader_refusals"]
        .as_array()
        .expect("reader refusal vectors are an array");
    for refusal in reader_refusals {
        let mut package = base_package.clone();
        let index = node_index_where(&package, |node| node["semantic_form"] == "option_value");
        let mut node = package["semantic_graph"]["nodes"][index].clone();
        match refusal["mutation"].as_str() {
            Some("semantic-type-is-payload-node") => {
                node["semantic_type"] = payload_node["node_id"].clone();
            }
            Some("semantic-type-is-composite-non-option") => {
                let composite_type = select(
                    &fixture,
                    &json!({
                        "node_tag": "composite_type",
                        "semantic_form": "record",
                        "ordinal": 0
                    }),
                );
                node["semantic_type"] = composite_type["node_id"].clone();
            }
            Some("payload-target-is-option-type-node") => {
                node["body"]["members"][0]["value"]["target"] = type_node["node_id"].clone();
                node["dependencies"] = json!([type_node["node_id"]]);
            }
            Some("payload-target-is-boolean-literal") => {
                let boolean_payload = select(&boolean_fixture, &refusal["payload_selector"]);
                let boolean_type = boolean_fixture["semantic_graph"]["nodes"]
                    .as_array()
                    .expect("Boolean fixture nodes are an array")
                    .iter()
                    .find(|candidate| {
                        candidate["node_tag"] == "scalar_type"
                            && candidate["semantic_form"] == "boolean"
                    })
                    .expect("Boolean fixture has its scalar type")
                    .clone();
                install(&mut package, boolean_type);
                install(&mut package, boolean_payload.clone());
                node["body"]["members"][0]["value"]["target"] = boolean_payload["node_id"].clone();
                node["dependencies"] = json!([boolean_payload["node_id"]]);
            }
            Some("dependencies-have-extra-target") => {
                let element_type = select(
                    &fixture,
                    &json!({
                        "node_tag": "scalar_type",
                        "semantic_form": "integer",
                        "ordinal": 0
                    }),
                );
                node["dependencies"] = json!([
                    payload_node["node_id"].clone(),
                    element_type["node_id"].clone()
                ]);
            }
            other => panic!("unknown reader mutation: {other:?}"),
        }
        let replacement_index = node_index_where(&package, |candidate| {
            candidate["semantic_form"] == "option_value"
        });
        package["semantic_graph"]["nodes"][replacement_index] = node;
        package["semantic_graph"]["nodes"]
            .as_array_mut()
            .unwrap()
            .sort_by(|left, right| node_id_order(&left["node_id"], &right["node_id"]));
        let result = read_package_semantics(&package, &catalog);
        let (code, cause, locus, index) = match result {
            Err(ReadRefusal::GraphShape(id)) => {
                let index = node_index_where(&package, |node| node["node_id"] == id);
                let member = application_node_shape_locus(&package["semantic_graph"]["nodes"][index])
                    .expect("a graph-shape refusal has a member locus");
                (
                    "invalid_semantic_graph",
                    None,
                    format!("/semantic_graph/nodes/{index}{member}"),
                    index,
                )
            }
            Err(ReadRefusal::OptionValue(code, cause, id, member)) => {
                let index = node_index_where(&package, |node| node["node_id"] == id);
                (code, cause, format!("/semantic_graph/nodes/{index}/{member}"), index)
            }
            other => panic!("{} had unexpected reader result: {other:?}", refusal["id"]),
        };
        assert_eq!(code, refusal["expected_code"], "{} code", refusal["id"]);
        assert_eq!(cause, refusal["expected_cause"].as_str(), "{} cause", refusal["id"]);
        let expected_locus = refusal["expected_locus"]
            .as_str()
            .expect("expected locus")
            .replace("{index}", &index.to_string());
        assert_eq!(locus, expected_locus, "{} locus", refusal["id"]);
    }
}

/// The type definition of `identity` in a selected FCD document.
fn selected_type<'a>(document: &'a Value, identity: &str) -> Option<&'a Value> {
    document["types"].as_array()?.iter().find(|declared| declared["identity"] == identity)
}

/// The element type FR-322 "Member type" derives from a `typeRef`: a `sequence`
/// type definition renders as `Sequence<` its `items` `>` and a `map` one as
/// `Map<Text, ` its `values` `>`, any other reference as its own identity.
fn selected_element(document: &Value, type_ref: &str) -> String {
    match selected_type(document, type_ref) {
        Some(declared) if declared["kind"] == "sequence" => {
            format!("Sequence<{}>", selected_element(document, declared["items"].as_str().expect("items")))
        }
        Some(declared) if declared["kind"] == "map" => {
            format!("Map<Text, {}>", selected_element(document, declared["values"].as_str().expect("values")))
        }
        _ => type_ref.to_owned(),
    }
}

/// A field's `resolved_path` row members after the step index: declared type,
/// member identity, rendered member type, collection kind, minimum, maximum.
/// A clock field of presence `optional` has the element `Option<Clock>`, and the
/// multiplicity then gives the collection (FR-322 "Member type" table).
fn selected_field_row(document: &Value, owner: &str, name: &str) -> Vec<String> {
    let declared = selected_type(document, owner).unwrap_or_else(|| panic!("type {owner}"));
    let field = declared["fields"]
        .as_array()
        .and_then(|fields| fields.iter().find(|field| field["name"] == name))
        .unwrap_or_else(|| panic!("field {owner}/{name}"));
    let type_ref = field["typeRef"].as_str().unwrap();
    let mut element = selected_element(document, type_ref);
    if field["presence"] == "optional" {
        assert_eq!(type_ref, "ix://quire/native/Clock", "only the clock member shape is ruled");
        element = format!("Option<{element}>");
    }
    let multiplicity = &field["multiplicity"];
    let (lower, upper) = (multiplicity["lower"].as_u64().unwrap(), multiplicity["upper"].as_u64());
    let (kind, rendered, min, max) = if (lower, upper) == (1, Some(1)) {
        match selected_type(document, type_ref).map(|declared| declared["kind"].as_str().unwrap()) {
            Some(collection @ ("sequence" | "map")) => (collection.to_owned(), element, "0".to_owned(), "unbounded".to_owned()),
            _ => ("none".to_owned(), element, String::new(), String::new()),
        }
    } else {
        let (kind, spelling) = match (multiplicity["ordered"].as_bool().unwrap(), multiplicity["unique"].as_bool().unwrap()) {
            (true, false) => ("sequence", "Sequence"),
            (false, true) => ("set", "Set"),
            (false, false) => ("bag", "Bag"),
            (true, true) => ("ordered_set", "OrderedSet"),
        };
        (
            kind.to_owned(),
            format!("{spelling}<{element}>"),
            lower.to_string(),
            upper.map_or_else(|| "unbounded".to_owned(), |upper| upper.to_string()),
        )
    };
    vec![owner.to_owned(), field["identity"].as_str().unwrap().to_owned(), rendered, kind, min, max]
}

/// The inner type of a rendered collection type, and whether it is a map.
fn rendered_inner(rendered: &str) -> Option<(String, bool)> {
    if rendered.starts_with("Map<") && rendered.ends_with('>') {
        return Some((rendered.to_owned(), true));
    }
    ["Sequence<", "Set<", "Bag<", "OrderedSet<"].into_iter().find_map(|prefix| {
        rendered.strip_prefix(prefix).and_then(|rest| rest.strip_suffix('>')).map(|inner| (inner.to_owned(), false))
    })
}

/// Every `resolved_path` row a place needs, derived from the selected document
/// alone: the `field` steps join the declared fields and the `slot` steps the
/// collection types they select from.
fn selected_document_rows(document: &Value, place: &Value, model: &str) -> Result<Vec<Vec<String>>, String> {
    let steps = place["body"]["members"][1]["value"]["members"].as_array().unwrap();
    let mut owner = model.to_owned();
    let mut previous: Option<Vec<String>> = None;
    let mut rows = Vec::new();
    for step in steps {
        let text = |at: usize| step["members"][at]["value"]["value"].clone();
        let row = match text(0).as_str().unwrap() {
            "field" => {
                let name = text(1).as_str().unwrap().to_owned();
                let row = selected_field_row(document, &owner, &name);
                owner.clone_from(&row[2]);
                row
            }
            "slot" => {
                let parent = previous.as_ref().ok_or("a slot step needs a collection row")?;
                let (inner, is_map) = rendered_inner(&parent[2]).ok_or("a slot step selects from a collection type")?;
                let (declaring, element) = if is_map {
                    let value = inner.strip_suffix('>').unwrap().split_once(", ").unwrap().1.to_owned();
                    (inner.clone(), value)
                } else {
                    (inner.clone(), inner)
                };
                let (kind, min, max) = match rendered_inner(&element) {
                    Some((_, true)) => ("map", "0", "unbounded"),
                    Some(_) => ("sequence", "0", "unbounded"),
                    None => ("none", "", ""),
                };
                let slot_kind = text(1);
                if slot_kind != parent[3] || text(3).to_string().trim_matches('"') != parent[4] || text(4).to_string().trim_matches('"') != parent[5] {
                    return Err("the slot step does not carry its collection's kind and bounds".to_owned());
                }
                owner.clone_from(&element);
                vec![declaring, format!("{}[slot]", parent[2]), element, kind.to_owned(), min.to_owned(), max.to_owned()]
            }
            other => return Err(format!("unexpected step {other}")),
        };
        previous = Some(row.clone());
        rows.push(row);
    }
    Ok(rows)
}

/// The rows a place carries, as the same vectors of text.
fn place_rows(place: &Value) -> Vec<Vec<String>> {
    place["body"]["members"][2]["value"]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (1..7)
                .map(|at| row["members"][at]["value"]["value"].as_str().unwrap().to_owned())
                .collect()
        })
        .collect()
}

/// The selected-package checks of FR-322-AC-240 over the vectors, the
/// document and the package, as the first defect found (`Ok` when admitted).
fn selected_timed_slot_defect(vectors: &Value, document: &Value, package: &Value) -> Result<(), String> {
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let root = nodes.iter().find(|node| node["semantic_form"] == "model_root").ok_or("no model_root")?;
    let row = &json!({"identity": root["owner"]["identity"]});
    if row["identity"] != document["package"]["identity"] {
        return Err("the model_root does not name the document".to_owned());
    }
    let derived = json!({"package": row["identity"], "path": ["model", "model_root"], "ordinal": 0});
    if root["node_id"] != derived || root["semantic_type"] != derived {
        return Err("the model_root is not derived from the identity".to_owned());
    }
    let model = "ix://acme/orders/Order";
    let mut positioned = Vec::new();
    for case in vectors["cases"].as_array().unwrap() {
        if let Some(at) = case["package_node"].as_u64() {
            positioned.push((case["id"].as_str().unwrap().to_owned(), at as usize, case["node"].clone()));
        }
    }
    for (name, place) in vectors["presence"]["places"].as_object().unwrap() {
        positioned.push((format!("place {name}"), place["package_node"].as_u64().unwrap() as usize, place["node"].clone()));
    }
    for (name, parameter) in vectors["presence"]["parameters"].as_object().unwrap() {
        let at = vectors["presence"]["parameter_package_nodes"][name].as_u64().unwrap() as usize;
        positioned.push((format!("parameter {name}"), at, parameter.clone()));
    }
    for case in vectors["presence"]["cases"].as_array().unwrap() {
        if let Some(at) = case["package_node"].as_u64() {
            positioned.push((case["id"].as_str().unwrap().to_owned(), at as usize, case["node"].clone()));
        }
    }
    let mut listed_places = BTreeSet::new();
    if let Some(root) = nodes.iter().find(|node| node["semantic_form"] == "timed_model") {
        let places = root["body"]["members"][3]["value"]["members"].as_array().unwrap();
        listed_places.extend(places.iter().map(|entry| OrderedId(entry["target"].clone())));
    }
    for (name, at, node) in &positioned {
        if nodes.get(*at) != Some(node) {
            return Err(format!("{name} is not the published node at {at}"));
        }
        if node["semantic_form"] == "clock_place" {
            if !listed_places.contains(&OrderedId(node["node_id"].clone())) {
                return Err(format!("{name} is not listed by the timed root"));
            }
            let want = selected_document_rows(document, node, model)?;
            if want != place_rows(node) {
                return Err(format!("{name}: the rows do not join the selected document"));
            }
        }
    }
    let placed = nodes.iter().filter(|node| node["semantic_form"] == "clock_place").count();
    if placed != listed_places.len() {
        return Err("the timed root lists a different set of places than the graph holds".to_owned());
    }
    Ok(())
}

#[trace("TC-367", "FR-322-AC-240", "FR-322-AC-241")]
#[test]
fn tc_367_timed_slot_vectors_name_a_complete_selected_package() {
    let vectors = parsed_fixture(TIMED_SLOT_SELECTOR_VECTORS);
    let document = parsed_fixture(SELECTED_TIMED_SLOT_DOCUMENT);
    let package = parsed_fixture(POSITIVE_TIMED_SLOT_NODES);
    let base = parsed_fixture(POSITIVE_TIMED_MODEL_NODES);
    assert_eq!(vectors["selected_document"], "domain-package-acme-timed-slots.json");
    assert_eq!(vectors["selected_package"], "fixtures/positive-timed-slot-nodes.json");
    assert_eq!(document["contractVersion"], "2.0.0");
    assert_eq!(document["package"]["identity"], "acme/orders");
    assert_eq!(selected_timed_slot_defect(&vectors, &document, &package), Ok(()));

    // The package is complete and schema-valid.
    let schema = parsed_fixture(SCHEMA);
    assert!(jsonschema::validator_for(&schema).unwrap().is_valid(&package));
    let nodes = package["semantic_graph"]["nodes"].as_array().unwrap();
    let ids: BTreeSet<_> = nodes.iter().map(|node| OrderedId(node["node_id"].clone())).collect();
    assert_eq!(ids.len(), nodes.len(), "node ids are unique");
    for (at, node) in nodes.iter().enumerate() {
        let entries = package["source_map"].as_array().unwrap().iter().filter(|entry| entry["node_id"] == node["node_id"]).count();
        assert_eq!(entries, 1, "node {at} has one source_map entry");
    }
    // Every published base node is unchanged except the timed root, and the operation anchor, its frame and
    // the model root, which follow the places added before them.
    let base_nodes = base["semantic_graph"]["nodes"].as_array().unwrap();
    for (at, node) in base_nodes.iter().enumerate().filter(|(at, _)| ![52, 57, 58, 59].contains(at)) {
        assert_eq!(nodes[at], *node, "base node {at} is unchanged");
    }
    for (base_at, at) in [(57, 70), (58, 71)] {
        assert_eq!(nodes[at], base_nodes[base_at], "base node {base_at} follows the added places at {at}");
    }
    assert_eq!(nodes.last().unwrap()["semantic_form"], "model_root", "the root is last");

    // The vectors' places SL-01..SL-07 equal their published cases and are all admitted nodes.
    for id in ["SL-01", "SL-02", "SL-03", "SL-04", "SL-05", "SL-06", "SL-07"] {
        let case = vectors["cases"].as_array().unwrap().iter().find(|case| case["id"] == id).unwrap();
        assert_eq!(nodes[case["package_node"].as_u64().unwrap() as usize], case["node"], "{id}");
    }

    // AC-241: the nested Sequence is declared by FCD fields alone.
    let nested = &vectors["nested_collection"];
    let field = selected_type(&document, "ix://acme/orders/Order").unwrap()["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["identity"] == nested["field"])
        .expect("field g");
    assert_eq!(field["typeRef"], nested["typeRef"]);
    assert_eq!(field["multiplicity"], nested["multiplicity"]);
    let definition = selected_type(&document, nested["typeRef"].as_str().unwrap()).unwrap();
    assert_eq!(definition["kind"], "sequence");
    assert_eq!(definition["items"], "ix://quire/native/Clock");
    assert_eq!(json!(definition["items"]), nested["type_definition"]["items"]);
    assert_eq!(selected_field_row(&document, "ix://acme/orders/Order", "g")[2], nested["rendered"].as_str().unwrap());
    // Only FCD typeDefinition and field members appear in the new definitions.
    for declared in document["types"].as_array().unwrap() {
        let allowed: &[&str] = match declared["kind"].as_str() {
            Some("sequence") => &["constraints", "displayName", "extensions", "identity", "items", "kind", "origin", "roles", "unknownPolicy"],
            Some("map") => &["constraints", "displayName", "extensions", "identity", "values", "kind", "origin", "roles", "unknownPolicy"],
            _ => &["constraints", "displayName", "extensions", "fields", "identity", "identityFields", "kind", "operations", "origin", "relationships", "roles", "supertypes", "unknownPolicy"],
        };
        for member in declared.as_object().unwrap().keys() {
            assert!(allowed.contains(&member.as_str()), "{} has no FCD member {member}", declared["identity"]);
        }
    }
}

#[trace("TC-367", "FR-322-AC-240", "FR-322-AC-241")]
#[test]
fn tc_367_selected_package_assertions_each_fail_on_their_mutation() {
    let vectors = parsed_fixture(TIMED_SLOT_SELECTOR_VECTORS);
    let document = parsed_fixture(SELECTED_TIMED_SLOT_DOCUMENT);
    let package = parsed_fixture(POSITIVE_TIMED_SLOT_NODES);
    let defect = |vectors: &Value, document: &Value, package: &Value| {
        selected_timed_slot_defect(vectors, document, package).expect_err("the mutation is caught")
    };
    let edit_document = |edit: &dyn Fn(&mut Value)| {
        let mut copy = document.clone();
        edit(&mut copy);
        copy
    };
    // A changed document makes the join fail where the declaration changed.
    let swapped_items = edit_document(&|d| {
        d["types"].as_array_mut().unwrap().iter_mut().find(|t| t["identity"] == "ix://acme/orders/ClockRow").unwrap()["items"] = json!("ix://quire/native/Integer");
    });
    let found = defect(&vectors, &swapped_items, &package);
    assert!(found.contains("do not join"), "the nested typeRef shape decides the g rows: {found}");
    let flat_g = edit_document(&|d| {
        let order = d["types"].as_array_mut().unwrap().iter_mut().find(|t| t["identity"] == "ix://acme/orders/Order").unwrap();
        order["fields"].as_array_mut().unwrap().iter_mut().find(|f| f["name"] == "g").unwrap()["typeRef"] = json!("ix://quire/native/Clock");
    });
    assert!(defect(&vectors, &flat_g, &package).contains("slot step selects from a collection"));
    let wrong_bound = edit_document(&|d| {
        let order = d["types"].as_array_mut().unwrap().iter_mut().find(|t| t["identity"] == "ix://acme/orders/Order").unwrap();
        order["fields"].as_array_mut().unwrap().iter_mut().find(|f| f["name"] == "ts").unwrap()["multiplicity"]["upper"] = json!(4);
    });
    assert!(defect(&vectors, &wrong_bound, &package).contains("slot step does not carry"));
    // The vectors and the package.
    let mut moved = vectors.clone();
    moved["cases"][4]["package_node"] = json!(2);
    assert!(defect(&moved, &document, &package).contains("not the published node"));
    let mut unlisted = package.clone();
    let root_at = unlisted["semantic_graph"]["nodes"].as_array().unwrap().iter().position(|n| n["semantic_form"] == "timed_model").unwrap();
    unlisted["semantic_graph"]["nodes"][root_at]["body"]["members"][3]["value"]["members"].as_array_mut().unwrap().remove(0);
    assert!(defect(&vectors, &document, &unlisted).contains("not listed"));
}

/// FR-322-AC-169: the published compound-unit package is a closed V2 package
/// that the reader admits; its compound unit names the root unit it
/// exponentiates and depends on exactly that unit.
#[trace("TC-940", "FR-322-AC-169")]
#[test]
fn tc_940_the_compound_unit_package_is_admitted_with_its_unit_reference() {
    let catalog = parsed_fixture(OPERATION_CATALOG);
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("V2 schema is valid");
    let package = parsed_fixture(POSITIVE_COMPOUND_UNIT);
    schema
        .validate(&package)
        .unwrap_or_else(|error| panic!("the compound-unit package is a closed V2 package: {error}"));
    assert_eq!(read_package_semantics(&package, &catalog), Ok(()));
    let nodes = package["semantic_graph"]["nodes"].as_array().expect("nodes");
    let compound = nodes
        .iter()
        .find(|node| node["semantic_form"] == "compound_unit")
        .expect("the compound unit is published");
    let unit = compound["body"]["members"][0]["members"][0]["value"]["target"].clone();
    assert_eq!(compound["dependencies"], json!([unit]));
    let named = node_by_id(nodes, &unit).expect("the unit node is published");
    assert_eq!(named["semantic_form"], "unit");
}
