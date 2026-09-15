---
id: FR-197
title: "Replay proof counterexamples through native execution"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
---

# FR-197: Replay proof counterexamples through native execution

## Description

When a proof backend reports a counterexample, the verification boundary SHALL
serialize its typed inputs canonically and replay them through native
`runtime::execute` under the same semantic package and domain.

## Inputs

Backend counterexample, package/run identity, model domain, claim and native
runtime selection.

## Outputs

Canonical counterexample with minimization lineage, backend/native verdict pair
and agreement/parity-failure result.

## Behavior

Serialization preserves exact value kinds, object/trace identities, state
anchors, bounds and source correspondence. Minimization creates linked revisions
and must preserve domain validity and failure. Agreement requires the same clause
verdict; decode, domain or verdict mismatch is an explicit backend parity failure.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-197-AC-1 | Canonical serialize/deserialize round-trips every backend-supported typed counterexample exactly. | Test (TC-219) |
| FR-197-AC-2 | Kani counterexamples replay through native execution with the same verdict on the bounded corpus. | Test (TC-219) |
| FR-197-AC-3 | Invalid-domain, decode or verdict mismatch is retained as parity failure and never rewritten as clause success/failure. | Test (TC-219) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
