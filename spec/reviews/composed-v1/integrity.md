---
id: SR-402
title: "Integrity review of the composed native Quire v1 baseline"
type: SpecReview
analysis: integrity
scope: "FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130; US-005/006/008/009; StR-002; NFR-010/020/021/040/050–052; TM-003/004/006/007 and their TC/IT contracts; proposals/quire-v1"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
---

## Summary

The native requirements retain the full selected state, temporal and choreography scope, including external systems, related instances and finite populations, and have identifiable story, stakeholder and planned verification chains. Adoption remains conditional on closing F's exact definition dependencies; the temporal criterion-to-case disagreement also needs correction.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No integrity finding beyond the review-set limitations below. | FR-030; FR-035; IT-010 | none |

## Scope and provenance

This is the integrity lens of the owner-selected complete review set. It reviews
the frozen native tree above.
F's correction through `921993bbbef92a8010aec46d46277498de5e0288` is present.
Reserved FR-070–089 numbers are not a document quota. Earlier draft/status paragraphs are interpreted with the
explicit final “Composed review input” in shared-foundation.md.

The dependency lens SR-403 and the scope lens own the remaining F successor-definition closure
and F window-mapping findings. The observation candidates' own indexed digests
do not close their referenced rule bytes. The evidence lens SR-404 owns the
FR-091-AC-5 case-reference disagreement. These are incorporated as review-set
limitations rather than duplicated Findings rows here.

The installed integrity and failure-domain checklists were applied to the
requirements, grammar/contracts, current definition roots and retained-candidate
distinctions. No production code, executable helper, Cargo test, hosted workflow,
or external repository was changed or qualified. All new cases are Planned.

## Traceability matrix

The arrows below describe authored relationships, including the explicit
FR-to-US-to-StR path. They do not assert that every FR has a redundant direct
StR edge. Reverse `implements` edges are sufficient to find US-009's refinements.

| Story | Functional requirements | Stakeholder need | Authored verification and planned cases |
| --- | --- | --- | --- |
| US-005 | FR-020 | StR-002 through US-005 | Test; TC-020 covers all four current release-scope criteria, retaining partial/full capability distinctions. |
| US-005 | FR-030–035 and FR-130 | StR-002 through US-005 | Test; TC-030–035, with TC-030 also covering all six FR-130 criteria; IT-010 exercises linker/assessment binding. |
| US-005 | FR-036–047 | StR-002 through US-005 | Test; TC-036–047 covers source, names, exact numeric admission, queries, pre-state, graph, control, invocation and diagnostics. NFR-010 adds TC-048 resource controls. |
| US-005 | FR-048–049 | StR-002 through US-005 | Test; TC-049 and TC-170–174 cover temporal syntax and choreography identities, causal values, compensation stages, populations and deadlines. |
| US-006 | FR-050–062 | StR-002 through US-006 | Test; TC-050–061 and TC-064; TC-062/063 constrain resource/reproducibility behavior; IT-020/021 covers the producer and shared-result seams. |
| US-008 | FR-090–095 | StR-002 through US-008 | Test; TC-110–118, with the AC-5 binding discrepancy in SR-404/FND-016; IT-040/041 separates supported temporal correspondence from refusals. |
| US-009 | FR-110–116 | StR-002 through US-009 | Test; TC-140–146, TC-147 for executable-boundary inspection; IT-050/051 covers assessment and output consumers. |

The native scoped obligation extraction contains 251 FR acceptance criteria,
four NFR-040 acceptance criteria and nineteen NFR measurement rows. Every
scoped FR has an authored Test method and a planned TC reference. This is a
method-and-plan inventory, not a coverage percentage. StR-002's demonstration
and inspection criteria retain stakeholder validation distinct from component
test execution; its combined enterprise example is represented across
TC-030/035/049/170–174 and IT-010/021/050/051.

## NFR scope and atomicity

| NFR | Explicit affected behavior | Planned verification assessment |
| --- | --- | --- |
| NFR-010 | Source/token/node admission, dependencies, exact arithmetic, queries and finite graphs | TC-048 distinguishes zero, exact/sufficient and insufficient limits and charged allocation/expansion. |
| NFR-020 | Protocol structure, linking, instances, observations and result retention | TC-062 supplies boundary and no-business-action controls; its complete bound inventory needs the separate analysis identified by SR-404. |
| NFR-021 | Equivalent protocol presentations, exact dependencies and provenance | TC-063 compares permitted permutations and independently changed semantic dependencies. |
| NFR-040 | Temporal admission, evaluation, retained captures/history and lowering | TC-118 distinguishes bound overflow, exhaustion, eviction and restoration identity; four explicit ACs refine the measurement rows. |
| NFR-050 | Observation event size, keys, retained history and finite population/window bounds | Scope is clear; TC-142/144 do not yet specify all of these boundary measurements. See SR-404. |
| NFR-051 | Immutable admitted assessment inputs and settled replay/incremental outcomes | TC-144 compares dispositions; missing-identity/provenance controls need an explicit binding. See SR-404. |
| NFR-052 | All owned observation executable and qualification paths | TC-147 inventories source, generated executables, adapters and audits with independent forbidden-runtime/toolchain controls. |

Shared descriptions sometimes enumerate several fields of one coherent contract.
FR-061's orthogonal result dimensions and FR-049's typed protocol representation
remain one externally observable output obligation each, refined by separate
criteria. This review does not demand a separate document per field. FR-062
already owns the independently requested E02 projection, preserving canonical
result ownership. FR-093/094 have a narrow EARS wording issue recorded by SR-407;
their acceptance distinctions remain separately testable.

## Consistency and hidden-assumption checks

The native contracts agree on exact identity, source authority, immutable
captures, pure total predicates, typed failure causes and dependency-local
refusal. Related O1/O2 subjects, transport deliveries, attempts and business
effects remain distinct. Positive-length graph reachability handles cycles and
duplicates without an arbitrary false-producing cutoff; structural containment
and predicate-call cycles have their own refusal rules. Finite source bounds
remain separate from work/retention limits.

The selected temporal and protocol successors agree that decisive truth can
settle while a surrounding workflow remains open. Missing exact decision support
removes that truth; missing unrelated facts remain visible without changing it.
Closure, activation, execution, adequacy and supersession remain distinct through
F's current result/output requirements. Native timestamped/past semantics do not
depend on current TL support. Later projection/realizability research is not a
new prerequisite for global conformance.

No reviewed FR delegates semantics to an authenticated or paginated external API,
an ambient CLI, source registry tie-break, or an interactive scaffold command.
Those checklist probes therefore do not justify new network, retry, auth, or
scaffolding requirements. Backend failure already has explicit dispositions under FR-031/060 and protocol-contract.md;
they do not license a stub or implicit fallback.

A's raw compiled-model/definition-byte digests and B's RFC 8785 result digests
are intentionally separate domains. Their coexistence is not itself a
contradiction. A population window and E's inclusive temporal endpoint need an
explicit mapping, not a silent choice of one owner's rule. IT-010-SC-06 supplies the distinguishing time-30 and earlier-decisive-
witness controls; F's successor dependency/mapping agreement must make the
selected correspondence concrete.

`quire validate --scope . 'spec/**/*.md' --summary` passed on the frozen input:
232/232 documents grammar-clean and no structural failure. The known Coverage Status/Status catalog extraction mismatch
precludes using automated coverage totals as authored coverage; no column or
status was silently renamed.
