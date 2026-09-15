---
id: FR-194
title: "Synthesize and independently check bounded candidates"
type: FR
relationships:
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-194-AC-1 | A found candidate is labeled unchecked until an independent checker proves the requested bounded obligation. | Test (TC-216) |
| FR-194-AC-2 | A failing candidate retains a replayable checker counterexample and synthesis lineage. | Test (TC-216) |
| FR-194-AC-3 | Complete no-candidate proof and resource-incomplete search remain distinct. | Test (TC-216) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
