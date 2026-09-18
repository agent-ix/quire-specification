// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-208 business-model-meaning binding rules (TC-235), qualified directly.
//!
//! QSpec has no production binder here -- the native compiler is
//! quire-spec-language -- so this file is QSpec's own oracle implementation
//! of the FR-208 structural admission rules TC-235 names: a minimal Quire
//! meaning binder over hand-authored construct nodes shaped the way FR-154
//! admission reads them (`TypeDefinition.fields`, `.identityFields`,
//! `.occurrenceField`, `.operations`, `.states`, `.transitions`, `.steps`,
//! `.persists`, `.members`, `.supertypes`, `.clauses`, `Field.typeRef`,
//! `Operation.pre`). Every test asserts both the rule's admitted case (a
//! K2-shaped node, matching TC-235's own fixture names) and the exact
//! violation FR-208 names for it, so a binder that stops enforcing a rule
//! fails the test that backs it -- not a hardcoded pass.

use ix_trace_rs::trace;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Meaning {
    ObjectType,
    ValueType,
    RecordValueType,
    EventType,
    StateMachine,
    Process,
    PersistenceInterface,
    Namespace,
}

impl Meaning {
    /// The six business model meanings FR-208 names, less object type.
    fn is_business(self) -> bool {
        matches!(
            self,
            Meaning::RecordValueType
                | Meaning::EventType
                | Meaning::StateMachine
                | Meaning::Process
                | Meaning::PersistenceInterface
                | Meaning::Namespace
        )
    }
}

#[derive(Clone)]
struct FieldDecl {
    name: &'static str,
    type_ref: &'static str,
}

#[derive(Clone)]
struct ClauseDecl {
    name: &'static str,
    language: &'static str,
}

#[derive(Clone)]
struct OperationDecl {
    name: &'static str,
    precondition: Option<&'static str>,
}

#[derive(Clone)]
struct TransitionDecl {
    from: &'static str,
    to: &'static str,
    trigger: &'static str,
    guard: Option<&'static str>,
    emits: Vec<&'static str>,
}

#[derive(Clone)]
struct StepDecl {
    name: &'static str,
    consumes: Vec<&'static str>,
    emits: Vec<&'static str>,
}

#[derive(Clone)]
struct Node {
    id: &'static str,
    meaning: Meaning,
    fields: Vec<FieldDecl>,
    identity_fields: Vec<&'static str>,
    occurrence_field: Option<&'static str>,
    operations: Vec<OperationDecl>,
    states: Vec<&'static str>,
    transitions: Vec<TransitionDecl>,
    steps: Vec<StepDecl>,
    persists: Vec<&'static str>,
    members: Vec<&'static str>,
    supertypes: Vec<&'static str>,
    clauses: Vec<ClauseDecl>,
    value_target: Option<&'static str>,
}

impl Node {
    fn empty(id: &'static str, meaning: Meaning) -> Self {
        Node {
            id,
            meaning,
            fields: Vec::new(),
            identity_fields: Vec::new(),
            occurrence_field: None,
            operations: Vec::new(),
            states: Vec::new(),
            transitions: Vec::new(),
            steps: Vec::new(),
            persists: Vec::new(),
            members: Vec::new(),
            supertypes: Vec::new(),
            clauses: Vec::new(),
            value_target: None,
        }
    }
}

fn f(name: &'static str, type_ref: &'static str) -> FieldDecl {
    FieldDecl { name, type_ref }
}

fn op(name: &'static str, precondition: Option<&'static str>) -> OperationDecl {
    OperationDecl { name, precondition }
}

fn trans(
    from: &'static str,
    to: &'static str,
    trigger: &'static str,
    guard: Option<&'static str>,
    emits: Vec<&'static str>,
) -> TransitionDecl {
    TransitionDecl {
        from,
        to,
        trigger,
        guard,
        emits,
    }
}

fn step(name: &'static str, consumes: Vec<&'static str>, emits: Vec<&'static str>) -> StepDecl {
    StepDecl {
        name,
        consumes,
        emits,
    }
}

fn clause(name: &'static str, language: &'static str) -> ClauseDecl {
    ClauseDecl { name, language }
}

struct Package(BTreeMap<&'static str, Node>);

impl Package {
    fn new(nodes: Vec<Node>) -> Self {
        Package(nodes.into_iter().map(|node| (node.id, node)).collect())
    }

    fn get(&self, id: &str) -> Option<&Node> {
        self.0.get(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Refusal {
    code: &'static str,
    cause: &'static str,
    node: String,
    member_path: String,
}

impl Refusal {
    fn new(
        code: &'static str,
        cause: &'static str,
        node: impl Into<String>,
        member_path: impl Into<String>,
    ) -> Self {
        Refusal {
            code,
            cause,
            node: node.into(),
            member_path: member_path.into(),
        }
    }
}

/// Whether `type_name` resolves through a value-type binding chain to the
/// native `Timestamp`.
fn resolves_to_timestamp(package: &Package, type_name: &str) -> bool {
    if type_name == "Timestamp" {
        return true;
    }
    match package.get(type_name) {
        Some(node) if node.meaning == Meaning::ValueType => match node.value_target {
            Some(target) => resolves_to_timestamp(package, target),
            None => false,
        },
        _ => false,
    }
}

/// FR-208-AC-4 (record value type): no identity field, no operation.
fn check_record_value_type(node: &Node) -> Result<(), Refusal> {
    if !node.identity_fields.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.identityFields",
        ));
    }
    if !node.operations.is_empty() {
        return Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            node.id,
            "TypeDefinition.operations[0]",
        ));
    }
    Ok(())
}

/// FR-208-AC-4 (event type): no identity field, no operation, and an
/// `occurrenceField` that names a field of the node whose type resolves to
/// `Timestamp`.
fn check_event_type(package: &Package, node: &Node) -> Result<(), Refusal> {
    if !node.identity_fields.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.identityFields",
        ));
    }
    if !node.operations.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.operations[0]",
        ));
    }
    if let Some(target) = node.occurrence_field {
        let Some(field) = node.fields.iter().find(|field| field.name == target) else {
            return Err(Refusal::new(
                "missing_declaration",
                "missing-name",
                node.id,
                "TypeDefinition.occurrenceField",
            ));
        };
        if !resolves_to_timestamp(package, field.type_ref) {
            return Err(Refusal::new(
                "invalid_model_binding",
                "malformed-declaration",
                node.id,
                "TypeDefinition.occurrenceField",
            ));
        }
    }
    Ok(())
}

/// FR-208-AC-5 (state machine): non-empty operations, states and
/// transitions; every state name unique; every transition's `from`, `to`
/// and `trigger` name a declared state/operation, its `guard` a `quire`
/// clause of the node, and its `emits` entries event types.
fn check_state_machine(package: &Package, node: &Node) -> Result<(), Refusal> {
    if node.operations.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.operations",
        ));
    }
    if node.states.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.states",
        ));
    }
    if node.transitions.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.transitions",
        ));
    }
    let mut seen_states = BTreeSet::new();
    for (index, state) in node.states.iter().enumerate() {
        if !seen_states.insert(*state) {
            return Err(Refusal::new(
                "invalid_model_binding",
                "conflicting-binding",
                node.id,
                format!("TypeDefinition.states[{index}]"),
            ));
        }
    }
    for (index, transition) in node.transitions.iter().enumerate() {
        if !node.states.contains(&transition.from) {
            return Err(Refusal::new(
                "missing_declaration",
                "missing-name",
                node.id,
                format!("TypeDefinition.transitions[{index}].from"),
            ));
        }
        if !node.states.contains(&transition.to) {
            return Err(Refusal::new(
                "missing_declaration",
                "missing-name",
                node.id,
                format!("TypeDefinition.transitions[{index}].to"),
            ));
        }
        if !node.operations.iter().any(|o| o.name == transition.trigger) {
            return Err(Refusal::new(
                "missing_declaration",
                "missing-name",
                node.id,
                format!("TypeDefinition.transitions[{index}].trigger"),
            ));
        }
        if let Some(guard) = transition.guard {
            let Some(guard_clause) = node.clauses.iter().find(|c| c.name == guard) else {
                return Err(Refusal::new(
                    "missing_declaration",
                    "missing-name",
                    node.id,
                    format!("TypeDefinition.transitions[{index}].guard"),
                ));
            };
            if guard_clause.language != "quire" {
                return Err(Refusal::new(
                    "invalid_model_binding",
                    "malformed-declaration",
                    node.id,
                    format!("TypeDefinition.transitions[{index}].guard"),
                ));
            }
        }
        for (emit_index, emitted) in transition.emits.iter().enumerate() {
            match package.get(emitted) {
                None => {
                    return Err(Refusal::new(
                        "missing_declaration",
                        "missing-name",
                        node.id,
                        format!("TypeDefinition.transitions[{index}].emits[{emit_index}]"),
                    ));
                }
                Some(target) if target.meaning != Meaning::EventType => {
                    return Err(Refusal::new(
                        "invalid_model_binding",
                        "malformed-declaration",
                        node.id,
                        format!("TypeDefinition.transitions[{index}].emits[{emit_index}]"),
                    ));
                }
                Some(_) => {}
            }
        }
    }
    Ok(())
}

/// FR-208-AC-6 (process): non-empty identity fields and steps; every step
/// name unique; every step's `consumes`/`emits` entries name event types.
fn check_process(package: &Package, node: &Node) -> Result<(), Refusal> {
    if node.identity_fields.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.identityFields",
        ));
    }
    if node.steps.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.steps",
        ));
    }
    let mut seen_steps = BTreeSet::new();
    for (index, decl) in node.steps.iter().enumerate() {
        if !seen_steps.insert(decl.name) {
            return Err(Refusal::new(
                "invalid_model_binding",
                "conflicting-binding",
                node.id,
                format!("TypeDefinition.steps[{index}]"),
            ));
        }
    }
    for (index, decl) in node.steps.iter().enumerate() {
        for (list_name, entries) in [("consumes", &decl.consumes), ("emits", &decl.emits)] {
            for (entry_index, name) in entries.iter().enumerate() {
                match package.get(name) {
                    None => {
                        return Err(Refusal::new(
                            "missing_declaration",
                            "missing-name",
                            node.id,
                            format!("TypeDefinition.steps[{index}].{list_name}[{entry_index}]"),
                        ));
                    }
                    Some(target) if target.meaning != Meaning::EventType => {
                        return Err(Refusal::new(
                            "invalid_model_binding",
                            "malformed-declaration",
                            node.id,
                            format!("TypeDefinition.steps[{index}].{list_name}[{entry_index}]"),
                        ));
                    }
                    Some(_) => {}
                }
            }
        }
    }
    Ok(())
}

/// FR-208-AC-7 (persistence interface): no fields, non-empty operations,
/// non-empty `persists` of object types, no operation contract and no
/// `quire` clause.
fn check_persistence_interface(package: &Package, node: &Node) -> Result<(), Refusal> {
    if !node.fields.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.fields[0]",
        ));
    }
    if node.operations.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.operations",
        ));
    }
    if node.persists.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.persists",
        ));
    }
    for (index, target_id) in node.persists.iter().enumerate() {
        match package.get(target_id) {
            None => {
                return Err(Refusal::new(
                    "missing_declaration",
                    "missing-name",
                    node.id,
                    format!("TypeDefinition.persists[{index}]"),
                ));
            }
            Some(target) if target.meaning != Meaning::ObjectType => {
                return Err(Refusal::new(
                    "invalid_model_binding",
                    "malformed-declaration",
                    node.id,
                    format!("TypeDefinition.persists[{index}]"),
                ));
            }
            Some(_) => {}
        }
    }
    if let Some(contracted) = node.operations.iter().find(|o| o.precondition.is_some()) {
        return Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            format!("{}/{}", node.id, contracted.name),
            "Operation.pre",
        ));
    }
    if let Some(quire_clause) = node.clauses.iter().find(|c| c.language == "quire") {
        return Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            format!("{}/{}", node.id, quire_clause.name),
            "TypeDefinition.clauses[0]",
        ));
    }
    Ok(())
}

/// FR-208-AC-8 (namespace): no fields, no operations, every member names a
/// package node that is not itself a namespace, and no `quire` clause.
fn check_namespace(package: &Package, node: &Node) -> Result<(), Refusal> {
    if !node.fields.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.fields[0]",
        ));
    }
    if !node.operations.is_empty() {
        return Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            node.id,
            "TypeDefinition.operations[0]",
        ));
    }
    for (index, member_id) in node.members.iter().enumerate() {
        match package.get(member_id) {
            None => {
                return Err(Refusal::new(
                    "missing_declaration",
                    "missing-name",
                    node.id,
                    format!("TypeDefinition.members[{index}]"),
                ));
            }
            Some(member) if member.meaning == Meaning::Namespace => {
                return Err(Refusal::new(
                    "invalid_model_binding",
                    "malformed-declaration",
                    node.id,
                    format!("TypeDefinition.members[{index}]"),
                ));
            }
            Some(_) => {}
        }
    }
    if let Some(quire_clause) = node.clauses.iter().find(|c| c.language == "quire") {
        return Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            format!("{}/{}", node.id, quire_clause.name),
            "TypeDefinition.clauses[0]",
        ));
    }
    Ok(())
}

/// FR-208-AC-9: a field `typeRef` may name a record value type; naming any
/// other business model meaning is `malformed-declaration` at `Field.typeRef`.
fn check_field_types(package: &Package, node: &Node) -> Result<(), Refusal> {
    for field in &node.fields {
        if let Some(target) = package.get(field.type_ref)
            && target.meaning.is_business()
            && target.meaning != Meaning::RecordValueType
        {
            return Err(Refusal::new(
                "invalid_model_binding",
                "malformed-declaration",
                format!("{}/{}", node.id, field.name),
                "Field.typeRef",
            ));
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum ClauseRole {
    Invariant,
    Guard,
}

/// FR-208-AC-10: a clause named by some transition's `guard` is a guard
/// only, never an invariant; every other `quire` clause of a record value
/// type, event type, state machine or process is an invariant.
fn classify_clause(node: &Node, clause_name: &str) -> ClauseRole {
    if node
        .transitions
        .iter()
        .any(|transition| transition.guard == Some(clause_name))
    {
        ClauseRole::Guard
    } else {
        ClauseRole::Invariant
    }
}

/// FR-208-AC-11: a precondition that reads `pre(...)` is
/// `wrong_snapshot`/`forbidden-pre-read`.
fn check_precondition(node_id: &str, operation: &OperationDecl) -> Result<(), Refusal> {
    if let Some(text) = operation.precondition
        && text.contains("pre(")
    {
        return Err(Refusal::new(
            "wrong_snapshot",
            "forbidden-pre-read",
            format!("{node_id}/{}", operation.name),
            "Operation.pre",
        ));
    }
    Ok(())
}

/// FR-208-AC-12: a `supertypes` entry must name a node of the node's own
/// meaning, and the supertype graph must be acyclic.
fn check_generalization(package: &Package, node: &Node) -> Result<(), Refusal> {
    for (index, super_id) in node.supertypes.iter().enumerate() {
        match package.get(super_id) {
            None => {
                return Err(Refusal::new(
                    "missing_declaration",
                    "missing-name",
                    node.id,
                    format!("TypeDefinition.supertypes[{index}]"),
                ));
            }
            Some(super_node) if super_node.meaning != node.meaning => {
                return Err(Refusal::new(
                    "invalid_model_binding",
                    "malformed-declaration",
                    node.id,
                    format!("TypeDefinition.supertypes[{index}]"),
                ));
            }
            Some(_) => {}
        }
    }
    if reaches(package, node.id, node.id, &mut BTreeSet::new()) {
        return Err(Refusal::new(
            "invalid_model_binding",
            "specialization-cycle",
            node.id,
            "TypeDefinition.supertypes",
        ));
    }
    Ok(())
}

/// Whether following `from`'s `supertypes` chain (excluding the zero-length
/// path) ever reaches `target`.
fn reaches<'a>(
    package: &'a Package,
    from: &'a str,
    target: &str,
    seen: &mut BTreeSet<&'a str>,
) -> bool {
    let Some(node) = package.get(from) else {
        return false;
    };
    for super_id in &node.supertypes {
        if *super_id == target {
            return true;
        }
        if seen.insert(super_id) && reaches(package, super_id, target, seen) {
            return true;
        }
    }
    false
}

/// FR-208-AC-4: a record-value-type node with an identity field or an
/// operation cannot bind; one with neither binds cleanly.
#[trace("TC-235", "FR-208-AC-4")]
#[test]
fn ac4_record_value_type_forbids_identity_fields_and_operations() {
    let clean = Node {
        fields: vec![f("amount_minor", "Int"), f("currency", "String")],
        ..Node::empty("Money", Meaning::RecordValueType)
    };
    assert_eq!(check_record_value_type(&clean), Ok(()));

    let with_identity = Node {
        identity_fields: vec!["amount_minor"],
        ..clean.clone()
    };
    assert_eq!(
        check_record_value_type(&with_identity),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "Money",
            "TypeDefinition.identityFields"
        ))
    );

    let with_operation = Node {
        operations: vec![op("add", None)],
        ..clean
    };
    assert_eq!(
        check_record_value_type(&with_operation),
        Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            "Money",
            "TypeDefinition.operations[0]"
        ))
    );
}

/// FR-208-AC-4: an event type with an identity field is
/// `invalid_model_binding`/`malformed-declaration` (the issue's own
/// example); an `occurrenceField` naming a field of another type or naming
/// no field at all is refused with its own distinct cause.
#[trace("TC-235", "FR-208-AC-4")]
#[test]
fn ac4_event_type_with_identity_field_is_malformed_declaration() {
    let package = Package::new(vec![
        Node {
            value_target: Some("Timestamp"),
            ..Node::empty("Instant", Meaning::ValueType)
        },
        Node {
            fields: vec![f("occurred_at", "Instant"), f("order_id", "UUID")],
            occurrence_field: Some("occurred_at"),
            ..Node::empty("OrderPlaced", Meaning::EventType)
        },
    ]);
    let clean = package.get("OrderPlaced").unwrap();
    assert_eq!(check_event_type(&package, clean), Ok(()));

    let with_identity = Node {
        identity_fields: vec!["order_id"],
        ..clean.clone()
    };
    assert_eq!(
        check_event_type(&package, &with_identity),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderPlaced",
            "TypeDefinition.identityFields"
        ))
    );

    let with_operation = Node {
        operations: vec![op("replay", None)],
        ..clean.clone()
    };
    assert_eq!(
        check_event_type(&package, &with_operation),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderPlaced",
            "TypeDefinition.operations[0]"
        ))
    );

    let occurrence_of_wrong_type = Node {
        occurrence_field: Some("order_id"),
        ..clean.clone()
    };
    assert_eq!(
        check_event_type(&package, &occurrence_of_wrong_type),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderPlaced",
            "TypeDefinition.occurrenceField"
        ))
    );

    let occurrence_naming_nothing = Node {
        occurrence_field: Some("at"),
        ..clean.clone()
    };
    assert_eq!(
        check_event_type(&package, &occurrence_naming_nothing),
        Err(Refusal::new(
            "missing_declaration",
            "missing-name",
            "OrderPlaced",
            "TypeDefinition.occurrenceField"
        ))
    );
}

/// FR-208-AC-5: a state machine needs operations, states and transitions; a
/// repeated state name is `conflicting-binding`; a transition naming an
/// undeclared state is `missing-name`; a guard clause not in `quire`, or an
/// `emits` entry that is not an event type, is `malformed-declaration`.
#[trace("TC-235", "FR-208-AC-5")]
#[test]
fn ac5_state_machine_binds_states_and_transitions_and_no_firing() {
    let package = Package::new(vec![Node::empty("OrderPlaced", Meaning::EventType)]);

    let clean = Node {
        fields: vec![f("placed", "Boolean")],
        operations: vec![op("place", None), op("cancel", None)],
        states: vec!["Draft", "Placed", "Cancelled"],
        transitions: vec![
            trans(
                "Draft",
                "Placed",
                "place",
                Some("NotYetPlaced"),
                vec!["OrderPlaced"],
            ),
            trans("Draft", "Cancelled", "cancel", None, vec![]),
        ],
        clauses: vec![clause("NotYetPlaced", "quire")],
        ..Node::empty("OrderLifecycle", Meaning::StateMachine)
    };
    assert_eq!(check_state_machine(&package, &clean), Ok(()));

    let no_operations = Node {
        operations: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &no_operations),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderLifecycle",
            "TypeDefinition.operations"
        ))
    );

    let no_states = Node {
        states: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &no_states),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderLifecycle",
            "TypeDefinition.states"
        ))
    );

    let no_transitions = Node {
        transitions: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &no_transitions),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderLifecycle",
            "TypeDefinition.transitions"
        ))
    );

    let repeated_state = Node {
        states: vec!["Draft", "Placed", "Cancelled", "Draft"],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &repeated_state),
        Err(Refusal::new(
            "invalid_model_binding",
            "conflicting-binding",
            "OrderLifecycle",
            "TypeDefinition.states[3]"
        ))
    );

    let transition_to_unknown_state = Node {
        transitions: vec![trans("Draft", "Shipped", "place", None, vec![])],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &transition_to_unknown_state),
        Err(Refusal::new(
            "missing_declaration",
            "missing-name",
            "OrderLifecycle",
            "TypeDefinition.transitions[0].to"
        ))
    );

    let guard_not_quire = Node {
        clauses: vec![clause("NotYetPlaced", "ocl")],
        ..clean.clone()
    };
    assert_eq!(
        check_state_machine(&package, &guard_not_quire),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderLifecycle",
            "TypeDefinition.transitions[0].guard"
        ))
    );

    let emits_wrong_meaning = Node {
        transitions: vec![trans(
            "Draft",
            "Placed",
            "place",
            Some("NotYetPlaced"),
            vec!["Money"],
        )],
        ..clean
    };
    let package_with_money = Package::new(vec![
        Node::empty("OrderPlaced", Meaning::EventType),
        Node::empty("Money", Meaning::RecordValueType),
    ]);
    assert_eq!(
        check_state_machine(&package_with_money, &emits_wrong_meaning),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderLifecycle",
            "TypeDefinition.transitions[0].emits[0]"
        ))
    );
}

/// FR-208-AC-6: a process needs an identity field and steps; a repeated
/// step name is `conflicting-binding`; a step `consumes`/`emits` entry that
/// is not an event type is `malformed-declaration`.
#[trace("TC-235", "FR-208-AC-6")]
#[test]
fn ac6_process_binds_correlation_key_and_steps_in_order() {
    let package = Package::new(vec![Node::empty("OrderPlaced", Meaning::EventType)]);

    let clean = Node {
        identity_fields: vec!["correlation_id"],
        steps: vec![
            step("receive_order", vec!["OrderPlaced"], vec![]),
            step("reserve_stock", vec![], vec![]),
            step("await_shipment", vec![], vec![]),
        ],
        ..Node::empty("Fulfilment", Meaning::Process)
    };
    assert_eq!(check_process(&package, &clean), Ok(()));

    let no_identity_field = Node {
        identity_fields: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_process(&package, &no_identity_field),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "Fulfilment",
            "TypeDefinition.identityFields"
        ))
    );

    let no_steps = Node {
        steps: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_process(&package, &no_steps),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "Fulfilment",
            "TypeDefinition.steps"
        ))
    );

    let repeated_step_name = Node {
        steps: vec![
            step("receive_order", vec![], vec![]),
            step("reserve_stock", vec![], vec![]),
            step("receive_order", vec![], vec![]),
        ],
        ..clean.clone()
    };
    assert_eq!(
        check_process(&package, &repeated_step_name),
        Err(Refusal::new(
            "invalid_model_binding",
            "conflicting-binding",
            "Fulfilment",
            "TypeDefinition.steps[2]"
        ))
    );

    let consumes_wrong_meaning = Node {
        steps: vec![step("receive_order", vec!["Money"], vec![])],
        ..clean
    };
    let package_with_money = Package::new(vec![
        Node::empty("OrderPlaced", Meaning::EventType),
        Node::empty("Money", Meaning::RecordValueType),
    ]);
    assert_eq!(
        check_process(&package_with_money, &consumes_wrong_meaning),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "Fulfilment",
            "TypeDefinition.steps[0].consumes[0]"
        ))
    );
}

/// FR-208-AC-7: a persistence interface refuses an empty `persists` list
/// (the issue's own example), a field, a `persists` entry that is not an
/// object type, and an operation contract.
#[trace("TC-235", "FR-208-AC-7")]
#[test]
fn ac7_persistence_interface_refuses_an_empty_persists_list() {
    let package = Package::new(vec![
        Node::empty("Order", Meaning::ObjectType),
        Node::empty("Money", Meaning::RecordValueType),
    ]);

    let clean = Node {
        operations: vec![op("get", None), op("save", None)],
        persists: vec!["Order"],
        ..Node::empty("OrderStore", Meaning::PersistenceInterface)
    };
    assert_eq!(check_persistence_interface(&package, &clean), Ok(()));

    let empty_persists = Node {
        persists: vec![],
        ..clean.clone()
    };
    assert_eq!(
        check_persistence_interface(&package, &empty_persists),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderStore",
            "TypeDefinition.persists"
        ))
    );

    let with_field = Node {
        fields: vec![f("cache", "Order")],
        ..clean.clone()
    };
    assert_eq!(
        check_persistence_interface(&package, &with_field),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderStore",
            "TypeDefinition.fields[0]"
        ))
    );

    let persists_wrong_meaning = Node {
        persists: vec!["Money"],
        ..clean.clone()
    };
    assert_eq!(
        check_persistence_interface(&package, &persists_wrong_meaning),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderStore",
            "TypeDefinition.persists[0]"
        ))
    );

    let with_operation_contract = Node {
        operations: vec![op("get", None), op("save", Some("true"))],
        ..clean
    };
    assert_eq!(
        check_persistence_interface(&package, &with_operation_contract),
        Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            "OrderStore/save",
            "Operation.pre"
        ))
    );
}

/// FR-208-AC-8: a namespace refuses a field (the issue's own example), an
/// operation, a member naming another namespace, and a `quire` clause.
#[trace("TC-235", "FR-208-AC-8")]
#[test]
fn ac8_namespace_with_a_field_is_refused() {
    let package = Package::new(vec![
        Node::empty("Order", Meaning::ObjectType),
        Node::empty("Money", Meaning::RecordValueType),
        Node::empty("OrderPlaced", Meaning::EventType),
        Node::empty("Billing", Meaning::Namespace),
    ]);

    let clean = Node {
        members: vec!["Order", "Money", "OrderPlaced"],
        ..Node::empty("OrderManagement", Meaning::Namespace)
    };
    assert_eq!(check_namespace(&package, &clean), Ok(()));

    let with_field = Node {
        fields: vec![f("name", "String")],
        ..clean.clone()
    };
    assert_eq!(
        check_namespace(&package, &with_field),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderManagement",
            "TypeDefinition.fields[0]"
        ))
    );

    let with_operation = Node {
        operations: vec![op("close", None)],
        ..clean.clone()
    };
    assert_eq!(
        check_namespace(&package, &with_operation),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderManagement",
            "TypeDefinition.operations[0]"
        ))
    );

    let member_is_a_namespace = Node {
        members: vec!["Order", "Money", "OrderPlaced", "Billing"],
        ..clean.clone()
    };
    assert_eq!(
        check_namespace(&package, &member_is_a_namespace),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "OrderManagement",
            "TypeDefinition.members[3]"
        ))
    );

    let with_clause = Node {
        clauses: vec![clause("Scoped", "quire")],
        ..clean
    };
    assert_eq!(
        check_namespace(&package, &with_clause),
        Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            "OrderManagement/Scoped",
            "TypeDefinition.clauses[0]"
        ))
    );
}

/// FR-208-AC-9: a field `typeRef` may name a record value type; naming any
/// other business model meaning is `malformed-declaration` at
/// `Field.typeRef`.
#[trace("TC-235", "FR-208-AC-9")]
#[test]
fn ac9_a_field_typeref_may_name_only_a_record_value_type() {
    let package = Package::new(vec![
        Node::empty("Money", Meaning::RecordValueType),
        Node::empty("OrderPlaced", Meaning::EventType),
    ]);

    let clean = Node {
        fields: vec![f("order_id", "UUID"), f("total", "Money")],
        ..Node::empty("Order", Meaning::ObjectType)
    };
    assert_eq!(check_field_types(&package, &clean), Ok(()));

    let field_names_an_event_type = Node {
        fields: vec![f("order_id", "UUID"), f("ref", "OrderPlaced")],
        ..clean
    };
    assert_eq!(
        check_field_types(&package, &field_names_an_event_type),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "Order/ref",
            "Field.typeRef"
        ))
    );
}

/// FR-208-AC-10: a clause named by a transition's `guard` is a guard only,
/// never an invariant; a persistence interface's `quire` clause is
/// `unsupported_construct`/`declaration-form`.
#[trace("TC-235", "FR-208-AC-10")]
#[test]
fn ac10_a_guard_clause_is_classified_as_a_guard_never_an_invariant() {
    let node = Node {
        operations: vec![op("place", None)],
        states: vec!["Draft", "Placed"],
        transitions: vec![trans(
            "Draft",
            "Placed",
            "place",
            Some("NotYetPlaced"),
            vec![],
        )],
        clauses: vec![clause("NotYetPlaced", "quire"), clause("Extra", "quire")],
        ..Node::empty("OrderLifecycle", Meaning::StateMachine)
    };
    assert_eq!(classify_clause(&node, "NotYetPlaced"), ClauseRole::Guard);
    assert_eq!(classify_clause(&node, "Extra"), ClauseRole::Invariant);
}

/// FR-208-AC-10: a persistence interface's `quire` clause is
/// `unsupported_construct`/`declaration-form`, distinct from a logical
/// invariant violation.
#[trace("TC-235", "FR-208-AC-10")]
#[test]
fn ac10_persistence_interface_refuses_a_quire_clause() {
    let package = Package::new(vec![Node::empty("Order", Meaning::ObjectType)]);
    let store = Node {
        operations: vec![op("get", None), op("save", None)],
        persists: vec!["Order"],
        ..Node::empty("OrderStore", Meaning::PersistenceInterface)
    };
    assert_eq!(check_persistence_interface(&package, &store), Ok(()));

    let with_clause = Node {
        clauses: vec![clause("Healthy", "quire")],
        ..store
    };
    assert_eq!(
        check_persistence_interface(&package, &with_clause),
        Err(Refusal::new(
            "unsupported_construct",
            "declaration-form",
            "OrderStore/Healthy",
            "TypeDefinition.clauses[0]"
        ))
    );
}

/// FR-208-AC-11: a precondition that reads `pre(...)` is
/// `wrong_snapshot`/`forbidden-pre-read`.
#[trace("TC-235", "FR-208-AC-11")]
#[test]
fn ac11_a_precondition_that_reads_pre_is_forbidden_pre_read() {
    let clean = op("place", Some("not self.placed"));
    assert_eq!(check_precondition("OrderLifecycle", &clean), Ok(()));

    let forbidden = op("place", Some("pre(self.placed) = false"));
    assert_eq!(
        check_precondition("OrderLifecycle", &forbidden),
        Err(Refusal::new(
            "wrong_snapshot",
            "forbidden-pre-read",
            "OrderLifecycle/place",
            "Operation.pre"
        ))
    );
}

/// FR-208-AC-12: generalization requires a same-meaning supertype and an
/// acyclic supertype graph; a subtype inherits its ancestor's fields and
/// conforms to it.
#[trace("TC-235", "FR-208-AC-12")]
#[test]
fn ac12_generalization_requires_same_meaning_supertypes_and_no_cycle() {
    let package = Package::new(vec![
        Node {
            fields: vec![f("amount_minor", "Int"), f("currency", "String")],
            ..Node::empty("Money", Meaning::RecordValueType)
        },
        Node::empty("Order", Meaning::ObjectType),
        Node {
            fields: vec![
                f("amount_minor", "Int"),
                f("currency", "String"),
                f("tax_minor", "Int"),
            ],
            supertypes: vec!["Money"],
            ..Node::empty("TaxedMoney", Meaning::RecordValueType)
        },
    ]);
    let clean = package.get("TaxedMoney").unwrap();
    assert_eq!(check_generalization(&package, clean), Ok(()));

    let wrong_meaning_supertype = Node {
        supertypes: vec!["Order"],
        ..clean.clone()
    };
    assert_eq!(
        check_generalization(&package, &wrong_meaning_supertype),
        Err(Refusal::new(
            "invalid_model_binding",
            "malformed-declaration",
            "TaxedMoney",
            "TypeDefinition.supertypes[0]"
        ))
    );

    let missing_supertype = Node {
        supertypes: vec!["Unknown"],
        ..clean.clone()
    };
    assert_eq!(
        check_generalization(&package, &missing_supertype),
        Err(Refusal::new(
            "missing_declaration",
            "missing-name",
            "TaxedMoney",
            "TypeDefinition.supertypes[0]"
        ))
    );

    // A supertype cycle: Money -> TaxedMoney -> Money.
    let mut cyclic_nodes = package.0.clone();
    cyclic_nodes.get_mut("Money").unwrap().supertypes = vec!["TaxedMoney"];
    let cyclic_package = Package(cyclic_nodes);
    assert_eq!(
        check_generalization(&cyclic_package, cyclic_package.get("TaxedMoney").unwrap()),
        Err(Refusal::new(
            "invalid_model_binding",
            "specialization-cycle",
            "TaxedMoney",
            "TypeDefinition.supertypes"
        ))
    );
}
