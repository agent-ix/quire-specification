---
id: FR-194
title: "Synthesize and independently check bounded candidates"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-194: Synthesize and independently check bounded candidates

## Description

When bounded synthesis is requested, the synthesizer SHALL search only the
declared candidate grammar/domain and keep candidate discovery separate from
independent correctness checking.

## Inputs

Checked specification, candidate grammar/domain, examples/constraints, search
bounds, synthesizer and independent checker profiles.

## Outputs

Candidate plus unchecked/checked status and proof/counterexample, unsatisfiable
within scope, incomplete, unsupported or failed result.

## Behavior

Search success yields an identified candidate, not a proof. The independent
checker consumes the candidate as a new subject under exact bounds. Search
exhaustion distinguishes proved no-candidate-in-domain from incomplete search.

## Search and validation contract

The request defines a finite candidate grammar, typed parameter domains,
symmetry rule, cost ordering and obligation. Search enumerates canonical
equivalence representatives in nondecreasing cost. A candidate becomes
`validated` only after an independently identified checker re-parses it and
discharges the obligation under the same subject and bounds. `no-candidate`
requires complete exhaustion of the declared canonical domain; any timeout,
provider loss or undisclosed pruning is incomplete.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-194-AC-1 | A found candidate is labeled unchecked until an independent checker proves the requested bounded obligation. | Test (TC-216) |
| FR-194-AC-2 | A failing candidate retains a replayable checker counterexample and synthesis lineage. | Test (TC-216) |
| FR-194-AC-3 | Complete no-candidate proof and resource-incomplete search remain distinct. | Test (TC-216) |
| FR-194-AC-4 | Canonical enumeration records the candidate grammar, domains, symmetry rule and cost ordering. | Test (TC-216) |
| FR-194-AC-5 | Validation uses an independently identified checker and the exact synthesis subject, assumptions and bounds. | Test (TC-216) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
