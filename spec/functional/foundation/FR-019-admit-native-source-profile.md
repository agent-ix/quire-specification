---
id: FR-019
title: "Enforce the native source profile admission contract"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-001
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
  - target: ix://agent-ix/quire-specification/NFR-001
    type: references
---
# FR-019: Enforce the native source profile admission contract

## Description

When a source unit selects the adopted native-source definition, the frontend shall enforce that definition's admission rules before producing a checked clause.

## Inputs

One exact source/profile selection, authored clauses and source regions,
explicit typed model declarations, and observation/operation bindings.

## Outputs

A checked Boolean clause under the selected definition or a located typed
refusal retaining the original source and rejected construct/declaration.

## Behavior

The replacement candidate is specified in the
[native-source amendment](../../../proposals/native-source-1-draft/README.md).
The frontend shall refuse every deferred source construct even inside an
unreachable branch. A backend's wider capability cannot admit a source construct.
The frontend shall require declared finite numeric and collection bounds rather
than derive them from storage width or sample values. Rational admission requires
exact normalization before range checks; refusal alone does not complete it.

The frontend shall preserve distinct authored absence/null channels or refuse
their declaration. Clause options do not define independently authored model
member presence or multiplicity; the shared-model owner selects that contract.
The frontend shall reject recursive containment and unsupported reference
navigation without erasing the original parent relationship or inventing bounds.

The frontend shall reject duplicate clause authorities, stale source bindings
and mismatched evaluation anchors before lowering. Invalid or unsupported input
does not produce a Boolean; runtime exhaustion remains incomplete.

The adopted state-finite/0-draft rules and fixtures remain historical evidence.
Their division/remainder, object/record equality, reachability and pre(parameter)
rules do not establish those capabilities in the replacement candidate. The
candidate becomes an admitted implementation profile only through explicit
definition adoption and corresponding compiler support.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-019-AC-1 | Signed-64 finite integer bounds and reject overflow are enforced; exact bounded rationals normalize before bounds. Float, implicit widening/conversion and integer division/remainder refuse, including in unreachable branches. | Test |
| FR-019-AC-2 | Distinct authored absence, null, required presence and multiplicity are preserved through a selected model contract or refused; an absent member cannot silently become an optional value, empty collection or explicit null. | Test |
| FR-019-AC-3 | Acyclic typed record paths are admitted; recursive containment, object/record equality, opaque-scalar dereference and deferred reference/closure operations refuse without rewriting the declaration. | Test |
| FR-019-AC-4 | Ordered duplicate-preserving sequences admit declared maxima at most 10000; maxima 10001, undeclared bounds, unsupported kinds and implicit conversions refuse, including nested or unused declarations. | Test |
| FR-019-AC-5 | A statically defined Boolean root is required; nullable/partial roots, truthiness coercion, user calls, recursion, reflection, I/O and ambient-store operations refuse with their source location. | Test |
| FR-019-AC-6 | Exactly one byte/revision/span-bound clause authority and the specified invariant/pre/post anchor are required; duplicate authority, stale bytes, bare context, foreign operation, forbidden pre access and cross-observation guard reuse refuse before lowering. | Test |

## Dependencies

- [FR-001](FR-001-select-native-profile.md): exact definition selection.
- [FR-003](FR-003-bind-source-correspondence.md): original byte/span correspondence.
- [NFR-001](../../non-functional/NFR-001-preserve-versioned-meaning.md): historical meaning.
- [Amendment](../../../proposals/native-source-1-draft/README.md): carried-forward table, anchor rules and open design gate.

## Status

Definition candidate under FS01/compiler #30. The acceptance criteria are
implementation obligations, not claims that the current compiler satisfies them.
