---
id: SR-509
title: "Scope-boundary review of the runtime value path model"
type: SpecReview
analysis: scope-boundary
scope: "Whether the change stays inside its brief: the requirement amended, the findings deliberately left open, and the boundary between specification and implementation at a9368c1"
review_set: subset
evaluated_revision: "a9368c1"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-047, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-207, type: reviews }
---
## Summary

The change amends one behavioral requirement, adds one object artifact, narrows
one open gap and touches nothing else. FR-047 was verified to be the smallest
requirement that owns the locus obligation. All eight finding classes the brief
placed out of scope are untouched and remain recorded where they already were.

## Was the smallest owning requirement chosen

Four requirements were candidates.

| Candidate | Owns | Rejected because |
| --- | --- | --- |
| FR-046 | State invocation inputs, unavailable-input results | Owns *which* inputs must be present, not *how* a locus inside one is named. Its Outputs are validated requests and refusals, not loci. |
| FR-009 | Object identity comparison | Owns the root's subject only. Amending it would define a path in a requirement whose Outputs are "identity equality or a foreign-universe/type refusal". |
| FR-045 | Control evaluation and optional-value facts | Owns the option-value step's justification, not the path. |
| **FR-047** | Typed located causes; "structured cause details with exact available loci" | **Selected.** Its Outputs already are loci. FR-047-AC-4 already requires that "loci and identities remain attached where available". FR-274, the artifact recording the gap, already `specifies` FR-047. |

FR-047 is the correct owner and the amendment is additive to it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Placing the definition in FR-047 means a consumer that needs a runtime value path for a non-diagnostic purpose must cite a requirement about diagnostic classification. The alternative — a new standalone behavioral FR — would have allocated a new behavioral identifier for a definition with no behavior of its own, and would have left FR-047-AC-4's locus obligation pointing at nothing. The placement is the lesser cost and is recorded so a future reader does not read it as accidental. | FR-047 Description; FR-047 Outputs; FR-274 `specifies` FR-047 |

## Findings deliberately not addressed

The brief named eight classes as out of scope. All eight are untouched, and each
remains recorded where it already was.

| Left open | Still recorded in |
| --- | --- |
| FR-003 vs FR-036 source-span convention | FR-203 "Open gap" |
| Diagnostic code snake_case vs kebab-case spelling | FR-271, FR-283 |
| FR-047's six producer stages vs the catalog's eleven | FR-273 |
| Three `sha256-jcs` preimage rules | FR-201 |
| The catalog's own byte digest having no named domain | FR-270 |
| Delivery, attempt and effect identity | FR-222..FR-226, FR-235; SR-502 FND-002 |
| Temporal and observation window boundary disagreement | FR-258, FR-266 |

The JSON field/index path was explicitly in the brief as something to leave
unresolved unless an accepted requirement independently defines it. None does,
so it stays unresolved, and FR-274's open gap now names it as the sole remaining
structural gap of the three kinds.

SR-502's findings were not repaired. FND-006, which lists FR-274 among artifacts
keying on labels with no established spelling, is unchanged: the amendment fixes
structure and fixes no spelling, and both FR-047 and FR-274 say so.

## Specification versus implementation

The change states no wire format, no tag, no separator, no field name and no
type declaration. It fixes what two paths mean, when they are equal, how they
compare and what absence looks like. An implementation of `MissingInput` can now
be written without inventing any of those four, and must still choose its own
serialization, which the specification declines to constrain.

One residual ambiguity would still force an implementation choice outside the
specification: sorting a collection of paths drawn from more than one selected
observation has no defined result, because the observation identity has no key
order (SR-507 FD-001). An implementation that only ever sorts within one
diagnostic does not meet it.

## Verdict

In scope. The change amends one requirement, adds one artifact, narrows one gap,
and touches nothing else.
