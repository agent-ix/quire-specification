---
id: FR-019
title: "Enforce the native source profile admission contract"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-001
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
---
# FR-019: Enforce the native source profile admission contract

## Description

When a source unit selects an adopted native definition, the frontend shall enforce that definition's admission rules before producing a checked clause.

## Inputs

One exact source/profile selection, authored clauses and source regions,
explicit typed model declarations, and observation/operation bindings.

## Outputs

A checked semantic declaration under the selected definition or a located typed
refusal retaining the original source and rejected construct/declaration.

## Behavior

The carried-forward admission baseline is preserved in the superseded
[native-source amendment](../../../proposals/native-source-1-draft/README.md)
and refined by the composed state definitions selected in
[AD-003](../../assurance/AD-003-native-quire-language-charter.md).
The frontend shall refuse every construct prohibited by the exact selected
profile even inside an unreachable branch. A backend's wider capability cannot
admit a source construct. The complete-V1 profile bundle shall admit every
positive construct in the atomic inventory through its owning facet; historical
bounded profiles retain their narrower admission. Finite consumers shall require
declared numeric and collection bounds rather than derive them from storage width
or sample values. Exact numeric admission shall apply the selected normalization
or IEEE rounding contract before range checks.

The frontend shall preserve distinct authored absence/null channels or refuse
their declaration. Clause options do not define independently authored model
member presence or multiplicity; the shared-model owner selects that contract.
The frontend shall admit finite recursive values, structural record operations
and closed model/reference navigation under the complete-V1 facets. Where an
older profile prohibits them or required closure/bounds are unavailable, the
frontend shall refuse without erasing the original relationship or inventing
bounds.

The frontend shall reject duplicate clause authorities, stale source bindings
and mismatched evaluation anchors before lowering. Invalid or unsupported input
does not produce a Boolean; runtime exhaustion remains incomplete.

The adopted `state-finite/0-draft` rules and fixtures remain historical evidence.
Their division/remainder, object/record equality, reachability and pre(parameter)
rules do not establish those capabilities in the composed profiles. The
superseded `state-finite/1-draft` identity shall not be admitted or aliased.
Each composed definition becomes an admitted implementation profile only through
its owning semantic acceptance and corresponding compiler support.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-019-AC-1 | Historical bounded profiles preserve their signed-64, exact-rational and prohibited-construct rules; the complete-V1 facets admit exact decimal, explicit IEEE float, mathematical integer and profile-selected division/remainder without changing historical bytes. | Test |
| FR-019-AC-2 | Distinct authored absence, null, required presence and multiplicity are preserved through a selected model contract or refused; an absent member cannot silently become an optional value, empty collection or explicit null. | Test |
| FR-019-AC-3 | The complete model facet admits typed structural equality, finite recursive values and explicit closed-environment reference operations; unavailable closure or a profile-prohibited use refuses without rewriting the declaration. | Test |
| FR-019-AC-4 | The complete collection facet admits bounded sequences, sets, bags and ordered sets plus explicit conversions; absent consumer bounds, implicit lossy conversion or a profile-prohibited kind refuses, including in unused declarations. | Test |
| FR-019-AC-5 | Complete-V1 declarations admit total pure typed functions and proved recursion; truthiness coercion, reflection, I/O, mutation and ambient-store operations refuse with their source location. | Test |
| FR-019-AC-7 | Selecting every complete-V1 facet admits the full language capability inventory subject only to its normative premises; an implementation gap is reported as unimplemented capability, not as product-scope deferral. | Test (TC-180) |
| FR-019-AC-6 | Exactly one byte/span-bound clause authority and the specified invariant/pre/post anchor are required; duplicate authority, stale bytes, bare context, foreign operation, forbidden pre access and cross-observation guard reuse refuse before lowering. | Test |

## Dependencies

- [FR-001](FR-001-select-native-profile.md): exact definition selection.
- [FR-003](FR-003-bind-source-correspondence.md): original byte/span correspondence.
- [Amendment](../../../proposals/native-source-1-draft/README.md): carried-forward table, anchor rules and open design gate.
- [AD-003](../../assurance/AD-003-native-quire-language-charter.md): accepted profile and support-stage boundaries.

## Status

Accepted FS01 admission constraint for composed state definitions. The acceptance
criteria remain implementation obligations; merged bounded-scalar/query/graph
behavior is partial evidence and does not establish every producer, backend or
qualification boundary.
