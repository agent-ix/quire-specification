---
id: SR-202611101500004
title: "Base review of QSL-1876 observation binding derivation"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/objects/observation/FR-261-observation-binding.md, spec/test-cases/TC-140-bind-typed-observations.md (PR #648 diff)"
review_set: subset
---
## Summary

Ticket: QSL-1876. Base checklist review of PR #648, which rules that an admitted
observation's checked node, workflow instance and repeat occurrence are derived
from its declared binding identity and FR-287 subject. The derivation direction
(no carried field, no caller attribution) is sound and the instance half agrees
with FR-287 and `RecordView::subject()`. The node half is not established: the
observation contract lets one binding serve several roles, so a binding identity
does not by itself name one owning native declaration. TC-140 does not exercise
the AC-4 refusal clauses.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Node derivation equates the declared binding identity with the declared observation role identity; the observation contract defines them separately and says "One explicit binding may be referenced by several compatible role mappings", so a binding identity can resolve to several owning declarations and the node is not determined | spec/objects/observation/FR-261-observation-binding.md:26-33, proposals/quire-v1/observation-contract.md:70-79 |
| FND-002 | medium | TC-140 does not test AC-4's refusal clauses: no caller-supplied attribution is offered and refused, no binding identity that resolves to no declaration, and no non-binding field (trace id, signal identity, timestamp) is mutated to show node/instance/ordinal unchanged | spec/test-cases/TC-140-bind-typed-observations.md:28,44 |
| FND-003 | low | TC-140 procedure edit breaks the file's line wrapping: a long added line followed by a stub "Supply a correction with the exact immutable" line | spec/test-cases/TC-140-bind-typed-observations.md:28-29 |

## Verdict

FAIL. Sound and consistent: no carried field or generated key; instance = FR-287
subject (authority, kind, object identity) matching `RecordView::subject()`
(quire-observation src/authority/observation.rs:257); `binding_identity()` exists
(line 239) as an opaque string. The node rule needs a stated one-to-one mapping
from binding identity to owning declaration (or the role identity carried, or a
resolution through the assessment's role mapping) before "no new view accessor"
holds. The `quire validate` "required 'id' frontmatter missing" error is
pre-existing and repo-wide for `object:` FRs (FR-260, FR-287 and FR-296, all
untouched, report it under quire 0.38.3); it is out of scope for this PR.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Under set resolution, a record with another binding identity can resolve to an overlapping node set, so TC-140's expected "a different node" holds only if the fixture's second binding resolves to a disjoint set; the procedure does not say so | spec/test-cases/TC-140-bind-typed-observations.md:30,51 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Checked node now resolves the binding identity through the assessment's declared role mapping to a set of owning declarations; membership in the set decides the node; an unresolved binding identity refuses (FR-261, FR-260-AC-5). |
| FND-002 | fixed | TC-140 now offers caller attribution, admits an unresolvable binding identity, and mutates trace ID, signal name and timestamp, with matching expected results (FR-260-AC-5, AC-8, AC-9). |
| FND-003 | fixed | Procedure and expected results are rewrapped and the stub line is gone. One merged line is about 100 characters, which is cosmetic only. |
