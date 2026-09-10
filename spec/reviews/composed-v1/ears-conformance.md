---
id: SR-407
title: "EARS conformance review of the composed native Quire v1 baseline"
type: SpecReview
analysis: ears-conformance
scope: "Requirement statements in FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130; StR-002; NFR-010/020/021/040/050–052; filament-core-data FR-100–103 at d6ac501dc4f7514aaa9973d83b049eb2e4b15e7f"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
---

## Summary

The engine reported no grammar findings for the 232-document native input or the four scoped D requirements. Semantic review of the 60 primary requirement fields and their behavior/constraint prose found two compound primary responses to simplify; the stated acceptance distinctions are concrete, and this wording review does not establish cross-contract agreement or adoption.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Manual non-singular finding: FR-093 Description says the evaluator “shall create a distinct obligation instance and shall evaluate each declared capture exactly once.” Owner E: use one response describing an obligation instance initialized with captures evaluated once at the activation anchor, or separate the two sentences while retaining the same criterion identities. Instance isolation and capture initialization remain separately observable; no new requirement-document split is requested. | FR-093; FR-093-AC-1/2/3/4 |
| FND-002 | medium | Manual non-singular finding: FR-094 Description says the evaluator “shall apply that assertion” and “shall preserve their independent progress/closure states” in one sentence. Owner E: state one response applying the assertion with scope-preserving results, or use separate sentences without changing the exact scope/closure/completeness axes. The two responses have concrete ACs, so this is a wording/traceability correction rather than an undefined temporal rule. | FR-094; FR-094-AC-3/4/7/8 |

## Deterministic validation

The exact commands were:

```text
quire --version
quire validate --scope . 'spec/**/*.md' --summary
quire validate --scope /home/peter/dev/worktrees/fcd-95-model-baseline 'spec/functional/FR-100*' 'spec/functional/FR-101*' 'spec/functional/FR-102*' 'spec/functional/FR-103*' --summary
```

Native validation ran from the reviewed worktree before writing these review
files, using Quire 0.31.0 (`cli 4f6ed024`, `engine 0.46.0@ca7362d4`). It reported
`232/232 docs grammar-clean (100%); 0 grammar finding(s): none` and
`174/356 criteria property-extractable (48%)`. D's targeted validation reported
`4/4 docs grammar-clean (100%); 0 grammar finding(s): none` and
`12/13 criteria property-extractable (92%)`. Duplicate installed archetype/inverse
edge warnings used first-wins resolution; they are not EARS findings.

These are document-level engine totals, not a statement-level conformance rate.
The semantic scope comprises 48 native FR Description fields, seven NFR
Statement fields, StR-002's Stakeholder Need and four D FR Description fields,
plus their requirement-bearing behavior/constraint text. US story wording, TC/IT
procedures and normative grammar productions were not scored as EARS
requirements. Both Findings rows are reviewer observations that the engine did
not report; the engine result is preserved without claiming it caught them.

## Semantic judgments

The native admission, linking, binding, evaluation and result-emission triggers
describe observable processing events. They need no mechanical replacement with
While merely because an implementation could run continuously. Requirements
about all equivalent inputs, immutable identities or retained historical bytes
can be ubiquitous invariants; NFRs do not need an invented event trigger.
The stakeholder subject in StR-002 is appropriate to its artifact type.

Negative admission conditions generally have concrete responses: refuse a
wrong identity/profile, retain incomplete missing input, or report an unsupported
requested mapping. A When-triggered binding operation with an explicit adverse
response is not automatically defective for lacking If/then. FR-060's backend
failure rule already states an unwanted-condition response, and the temporal
history/capture requirements retain the difference between absence, refused
bindings, inactive activation and incomplete execution.

No independent “fast,” “robust,” or unspecified “support” obligation was found
in the scoped primary fields. Descriptions such as retain exact bindings, refuse
substitution, normalize a rational before bounds, and preserve ordered
occurrences have concrete acceptance observations. Broad field lists in a
single result/binding output do not by themselves require separate artifacts.
NFR-052's two sentences refine one owned executable-language policy; the
negative exemption sentence is not mistaken for a second shall in one sentence.

The relevant semantic disputes are handled by the other review lenses: D's
static/runtime prerequisites, exact observation definition closure, temporal
window correspondence and result-state consistency cannot be repaired by EARS
wording alone. FR-115/116's integrated correction is included in this review;
no absent-consumer claim is inferred from earlier historical status prose.

This lens is advisory and does not invoke strict CI, qualify an evaluator,
change a requirement, or adopt any candidate definition. The two wording
dispositions should preserve existing meaning and criterion IDs and receive
the owning E review before the affected definition is revised explicitly.
