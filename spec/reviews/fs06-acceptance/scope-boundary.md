---
id: SR-553
title: "Scope-boundary review of FS06 output mapping"
type: SpecReview
analysis: scope-boundary
scope: "AD-004, common mapping boundary, target mappers and optional structural observers"
review_set: subset
evaluated_revision: "task/7 working tree based on 0dd4384"
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-004, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-120, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-125, type: reviews }
---
# Scope-boundary review of FS06 output mapping

## Summary

PASS. The architecture allocates common orchestration, target correspondence,
package assembly and structural observation as distinct responsibilities.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | Resolved: multi-target fallback could substitute a different consumer for unsupported meaning. Each request/package now selects exactly one target; another target requires another independent request. | AD-004; FR-120; FR-297 |
| FND-006 | high | Resolved: a parser/typechecker in the generation path could silently become semantic authority. Observers now run only after immutable assembly and produce separate structural evidence. | AD-004; FR-125; FR-299; NFR-061 |
| FND-007 | medium | Resolved: generated SysML declarations could become a second model authority. The target profile permits only exact correspondences to spec-declared model declarations and treats local names as display. | FR-123; SysML/KerML profile |
| FND-008 | low | No ownership collision found. QSpec owns standard contracts, qcir owns Rust exporters and the TL-* owner retains native/TL semantics. | AD-004; IT-060; qcir #55/#56/#57 |

## Boundary verdicts

| Boundary | Inside | Outside |
| --- | --- | --- |
| Common coordinator | strict selection, dispatch, complete accounting, resources | target syntax and source parsing |
| Target mapper | one selected correspondence and deterministic bytes | another target or shared loss semantics |
| Package assembler | byte/record binding and identities | parser verdict and evidence sufficiency |
| Structural observer | exact-version parse/typecheck observation | native truth, preservation and package mutation |
