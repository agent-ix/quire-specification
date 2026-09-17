---
id: SR-503
title: "Dependency review of the domain object layer"
type: SpecReview
analysis: dependency
scope: "spec/objects/ FR-200 through FR-283, the 38 behavioral FRs given references back-edges, and spec/spec.md registration"
review_set: subset
evaluated_revision: "d6e19a5"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-250, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-260, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-280, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-050, type: reviews }
---
## Summary

The change is enablement, not feature work. It adds 73 object artifacts under
`spec/objects/` and registers them in `spec/spec.md`, and it touches the
existing behavioral requirements only by appending `references` back-edges to
their frontmatter `relationships` list.

Downstream breakage risk is nil at the artifact-identity level and was measured,
not assumed. `git diff origin/main HEAD -- spec/functional | grep '^[+-]id:'`
returns nothing, so no existing requirement identifier moved. The unified diff
restricted to `spec/functional` at `-U0` contains only added `- target:` and
`type: references` lines: 141 `type: references` additions and 141 matching
`- target:` additions, and no deletions and no body, Description, Behavior or
Acceptance-Criteria change anywhere. `spec/functional/FR-050-bind-protocol-instances.md`
is representative: the diff hunk is `@@ -7,6 +7,20 @@` inside `relationships:`
and nothing else.

The 141 added `references` edges are the exact inverse of the 141 `specifies`
edges authored on the object artifacts, so the back-edge set is derived rather
than independently asserted. Nothing a consumer resolves today changes meaning.

The real dependency debt this change creates is in the identifier space and in
the internal object graph, not at the repository boundary.

## Consumer exposure

Consumers resolve this repository by `ix://` URI and none of them vendors a copy
of these artifacts, so an additive frontmatter edge cannot break a build.

- `quire-protocol` references exactly: `FR-047`, `FR-050`, `FR-054`, `FR-055`,
  `FR-058`, `FR-059`, `FR-060`, `FR-061`, `FR-062`, `NFR-020`, `NFR-021`. Its
  `spec/functional/FR-002-link-finite-protocol-representation.md` line 68 also
  carries the prose range "`FR-050` through `FR-054`" and its
  `spec/functional/FR-001-admit-selected-protocol-profile.md` line 78 the prose
  range "`FR-050` through `FR-062`". Every one of those identifiers survives
  this change unchanged, and no new identifier was inserted inside `FR-050`
  through `FR-062`.
- `quire-observation` references no `quire-specification` identifier at all;
  `grep -rhoE 'quire-specification/[A-Za-z]+-[0-9]+' /home/peter/dev/quire-observation`
  returns nothing.
- `quire-verification` has no `spec/` directory on disk, so it consumes nothing
  from this repository today.

No consumer action is required to adopt this change. A consumer that later
chooses to traverse the new `references` edges out of `FR-050` must pin the same
`quire-specification` revision, because those seven targets (`FR-210` through
`FR-215` and `FR-217`) exist only from this revision forward.

## Verdict

**Approve as enablement, with the identifier-space allocation to be settled
before merge.** Findings FND-001 and FND-002 are allocation and numbering
decisions that get materially more expensive after these identifiers are cited
from other branches and other repositories; they do not block the semantic
content. FND-003 through FND-007 are internal graph defects that a follow-up on
this same branch can close. Nothing in the change forces a consumer to act.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-200–FR-283 claim is unreserved. `docs/functional-capability-ledger.md` is not in `git diff origin/main HEAD --stat`, and its workstream table at lines 36–43 allocates owners but no identifier ranges, so nothing records that FR-200+ is taken. The range is genuinely free today — `git grep -hoE '^id: FR-[0-9]+' <ref>` over all 20 local and 14 remote refs yields no FR-2xx on any ref other than this branch — but six agent branches allocate identifiers concurrently and the highest occupied identifier elsewhere is FR-130. | docs/functional-capability-ledger.md:36-43; git branch -a |
| FND-002 | high | Four of the seven subsystem blocks are full on their first commit, and one has already overflowed. obligation occupies FR-230..FR-239 (10 of 10), result FR-240..FR-249 (10 of 10), temporal FR-250..FR-259 (10 of 10), observation FR-260..FR-269 (10 of 10). The four result artifacts that did not fit were placed at FR-280..FR-283 while still living in `spec/objects/result/`, so the result subsystem is already discontiguous. Only foundation (FR-200..FR-206), protocol (FR-210..FR-217), causality (FR-220..FR-228) and diagnostics (FR-270..FR-274) retain headroom. | spec/objects/result/FR-280-assumption-state.md:2; spec/objects/result/FR-283-projection-refusal-code.md:2 |
| FND-003 | medium | FR-280..FR-283 are orphans in the object graph. `spec/objects/result/FR-240-canonical-protocol-result.md` lines 7–33 reference FR-241 through FR-248 plus FR-201, and `FR-249` references FR-240, but no artifact anywhere references FR-280, FR-281, FR-282 or FR-283. They specify FR-061 and FR-062 like their siblings yet hang off no aggregate, which is the structural consequence of the FND-002 spill. | spec/objects/result/FR-240-canonical-protocol-result.md:7-33 |
| FND-004 | medium | Two mutual `references` cycles make the object graph non-acyclic. `spec/objects/temporal/FR-250-temporal-semantic-profile.md:13` references FR-251 and `spec/objects/temporal/FR-251-tl-target-profile.md:9` references FR-250. `spec/objects/observation/FR-265-selected-snapshot.md:9` references FR-266 and `spec/objects/observation/FR-266-event-time-window.md:11` references FR-265. The `contains` sub-graph is acyclic; only `references` cycles. | spec/objects/temporal/FR-250-temporal-semantic-profile.md:13; spec/objects/observation/FR-266-event-time-window.md:11 |
| FND-005 | medium | The declared bounded context composes almost nothing. `spec/objects/foundation/FR-200-quire-assessment-domain.md` is `object: domain` and its only object edge is `contains` FR-201, so 72 of the 73 artifacts are unreachable by traversal from the context that claims to own them. Its own Description directs a reader to "resolve that noun through the object artifacts contained in this bounded context", which the edge set does not support. | spec/objects/foundation/FR-200-quire-assessment-domain.md:7-8,14 |
| FND-006 | medium | Workstream F's requirements are typed from workstream B's directory. `spec/objects/result/` carries `specifies` edges onto FR-115 from FR-243, FR-245, FR-247 and FR-281, and onto FR-116 from FR-247, while FR-116 is also typed from `spec/objects/observation/FR-269-consumer-mapping-loss.md`. The ledger reserves "Correlation, completeness, progress and observation/window correspondence" to F, so `spec/objects/result/` needs F review and FR-116 now has two owners. | docs/functional-capability-ledger.md:43; spec/objects/result/FR-247-complete-global-conformance-closure-state.md; spec/objects/observation/FR-269-consumer-mapping-loss.md |
| FND-007 | low | The obligation subsystem is structurally isolated. FR-230..FR-239 carry no object edge outside `spec/objects/obligation/`, yet FR-235 and causality's FR-225 and FR-226 all `specifies` FR-058, so the same retry-and-partial-recovery prose is typed twice with no edge relating a compensation attempt to an operation attempt or a business effect. | spec/objects/obligation/FR-235-compensation-attempt.md; spec/objects/causality/FR-225-operation-attempt.md |
