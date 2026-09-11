---
id: SR-400
title: "Composed native Quire v1 baseline review and disposition"
type: SpecReview
analysis: base
scope: "proposals/quire-v1; AD-001; StR-002; US-005/006/008/009; FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130; NFR-010/020/021/040/050/051/052; TM-003/004/006/007 and associated TC/IT; external D baseline 1.2 and FR-100–103"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
correction_evaluated_revision: "9d47ecabe0662f44b98d761c5019dd50c2eee57e"
review_date: "2026-09-10"
---

## Summary

**Current correction verdict: PASS for private PR #15 merge.** The bounded
[correction recheck](#pr-15-correction-recheck) below resolves the original
composed findings and SR-408. It covers the integrated semantics at `d8d984b`,
the whitespace correction at `9d47eca`, and the current navigation/disposition
updates. It does not claim executed v1 conformance, external producer acceptance
or public standard adoption.

The following initial verdict and findings remain the record at `788b161`:

**Changes required before accepting the composed baseline.** D and F have
delivered their contributions. The remaining semantic findings are specific
disagreements in static versus runtime prerequisites, decisive truth on open or
partly observed subjects, clock/window correspondence and immutable identity
selection. The architecture retains one lexer and expression grammar, typed
family representations and separate model, execution and evidence authorities;
this review does not call for replacing those foundations.

This is the owner-selected QUOIN base-plus-seven review at PR readiness, not an
intermediate review or an implementation-coverage audit. It evaluates the exact
native revision above with F through
`921993bbbef92a8010aec46d46277498de5e0288` and D's external contract through
`d6ac501dc4f7514aaa9973d83b049eb2e4b15e7f`. Prior B/E reviews retain their original
scope and are not substituted for this composed review.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | A's capability crosswalk and earlier shared-foundation status paragraphs still call D's standard-facing contribution absent and F's result correction pending, despite the final intake recording both. This obscures the real remaining corrections and can create a false delivery gate. Owner A: update the unpinned navigation/status material to the delivered revisions and link the concrete review findings. Preserve immutable rule snapshots and do not claim the baseline accepted. | capabilities.md, V1-001/003/049 and related crosswalk rows; shared-foundation.md, Authoring integration and Composed review input; SR-403 | wrong-requirement |

## Complete review set and disposition

The linked artifacts hold the finding details and distinguishing cases. Repeated
discussion of one defect under another lens is not an additional ticket or gate.

| Analysis | Artifact | Required disposition / owner |
| --- | --- | --- |
| Base | SR-400, this document | A corrects navigation/status and integrates the bounded corrections below. |
| Failure domain | [SR-401](failure-domain.md) | D narrows the blanket unavailable-input truth veto and specifies canonical numeric semantics; F corrects FR-114's blanket open-prefix veto. B/E's exact support and closure distinctions remain authoritative. |
| Integrity | [SR-402](integrity.md) | D/A distinguish static linking prerequisites from concrete assessment prerequisites; a future template can type-check before observations exist. |
| Dependency | [SR-403](dependency.md) | F/A publish successor immutable dependency selections after the affected corrections; E corrects FR-090's obsolete FR-020 prerequisite. |
| Evidence | [SR-404](evidence.md) | E/F/B reconcile identified criterion, metric, planned-case and catalog-method bindings. These are corrections to existing cases, not a demand for a full qualification campaign now. |
| Risk and complexity | [SR-405](risk-complexity.md) | A refines existing implementation tickets by accepted contract boundaries; B/D/E/F retain their existing ownership and C continues independently. |
| Scope boundary | [SR-406](scope-boundary.md) | F/D/E/A define runtime window-to-clock coverage; D/A define correspondence between canonical producer objects and raw native definition/model bytes without equating digest domains. |
| EARS conformance | [SR-407](ears-conformance.md) | E resolves the identified compound trigger wording without changing temporal meaning. |

All semantic findings remain open at the evaluated revision. Fix the affected
requirements/cases through their owners, integrate the ready commits and check
those changes. A then closes the selected definition dependencies and records
finding dispositions. Repeating all eight analyses on every edit is unnecessary.
Missing local FR-070–089 files are not a defect: those numbers were reservations,
and D's actual external FR-100–103 are indexed in the master specification.

## Architecture and requirements assessment

AD-001 and FR-036/037 establish original-byte lexical recognition and a shared
expression parser. Resolution, typing, definedness and exact profile admission
follow parsing. State expressions, temporal formulas and protocol control have
distinct typed representations; temporal `holds` leaves and protocol guards
reuse the common value language. The native package remains inspectable when a
requested backend cannot support it. This is a suitable foundation for evolving
the language without adding text parsers to adapters or tying syntax to one IR.

The grammar gives whole-unit consumption, precedence, associativity, native
binder versus qualified model-member rules and original half-open source spans.
The state contract specifies exact arithmetic, pure acyclic predicates,
ordered occurrence-preserving queries, observation-qualified captures and
positive-length finite-graph reachability. Choreography includes explicit roles,
causal control, related instances, compensation registration/activation and
commit/recovery. Finite populations/windows and external participants remain
required scope. General realizability and execution of business actions remain
separate from global observational conformance.

The remaining defects do not require a new lexer, parser generator, universal
execution AST, model store or evidence system. In particular, no runtime
population/window should become a prerequisite for parsing or type-checking a
static template. Runtime authority and complete decision support are validated
at the consuming assessment boundary.

Scoped FRs have named inputs, outputs, behavior and acceptance criteria; the
story/stakeholder chains are recorded by SR-402. Multiple criteria for one
coherent output contract need not become separate documents. Numeric and work
limits have concrete boundary controls; caller-selected limits are explicit
inputs rather than invented universal performance promises. Typed refusal,
unsupported, resource-incomplete, pending and decisive truth retain separate
meanings. The open findings identify the places where that intended distinction
has not yet survived composition.

## Planned verification: six-rule assessment

The four matrices contain 58 planned TC artifacts, with integration scenarios
in IT-010/020/021/040/041/050/051. A planned procedure is not executed evidence.

| Rule | Assessment at the evaluated revision |
| --- | --- |
| Criterion coverage | All 251 scoped FR criteria name planned Test cases. SR-404 records the FR-091-AC-5 case disagreement and incomplete NFR metric bindings. No claim of complete executed coverage. |
| Valid option combinations | TC-030/032/040 distinguish edition, outer/inner profile, feature and backend selections; TC-031/061/145/146 cover result axes. F's immutable dependency closure and the window mapping are still open, so complete composed permutations cannot yet be claimed. |
| Constraint boundaries | Exact numeric, sequence, source, traversal, protocol and temporal limits have planned controls. SR-404 identifies missing independent event-size/active-key/retained-state controls in F's current cases. |
| Error paths | Wrong identity/kind/anchor, ambiguous binding, unsupported backend, missing input and resource stops are explicit. SR-401/402/406 require additional distinguishing controls for the inconsistent shared boundaries. |
| State transitions | Activation, first compensation registration/trigger, retry, commit, partial recovery, progress/closure and supersession have planned cases. FR-114/TC-144 must add settled-open replay agreement as SR-401 requires. |
| Edge cases | Empty versus missing, duplicate receipt versus effect, concurrent orders, split shipments, exact deadlines, reference cycles, historical captures and late contradictions are represented. An authored example does not prove the implementation handles it. |

The adviser is one input to SR-404's method assessment. Its recommendations
do not remove relevant property, mutation, differential or resource methods,
and invented Method-cell labels do not automatically become catalog methods.
The known catalog `Coverage Status`/`Status` mismatch prevents using engine totals
as proof that authored matrix rows are verified; no mass column rename or false
Tested status is part of this review.

## Carried-forward admission rules

The owner's 2026-09-09 ruling remains the historical baseline. The composed v1
extensions are explicit proposed successor selections, not retroactive claims
that the first profile admitted them.

| Admission concern | Retained rule or explicit successor boundary |
| --- | --- |
| Source authority and loci | FR-030/036/047 retain one native editable authority, original bytes, exact revisions/digests and structured located causes. External textual forms remain output mappings. |
| Exact numeric domain | FR-039/044 retain finite signed-64 integer bounds, exact rational normalization before bounds and proved definedness. No float, saturation, implicit conversion or integer division/remainder. Rational division requires nonzero and range proof. |
| Presence, null and invalid | FR-046 and D FR-100 preserve authored presence independently of multiplicity; unsupported distinct channels refuse. Invalid or unavailable input does not become false or option-none. |
| Records and references | Historical acyclic records and no scalar-to-object/ambient lookup remain. The new graph facet explicitly selects finite typed reference semantics; original definition bytes are unchanged. |
| Collections and calls | Historical ordered duplicate-preserving finite sequences and maximum 10000 remain. Shared predicates/queries receive new selected semantics; no recursion, reflection, I/O or implicit truthiness is added. |
| Evaluation instants | FR-042/046 retain invariant/pre/post anchors, immutable captures and exact invocation observations. A bare context type is not an observation. |
| ConfigVersion design gate | A minimum-only integer still lacks a finite upper bound; the parent relation is not repaired with a recursive inline record, fabricated maximum or erased relationship. |
| Temporal and mapping boundary | Native temporal meaning is separately selected; FR-095 owns exact TL correspondence and explicit unsupported mapping. Output parser acceptance does not establish equivalent meaning. |
| Executable and rights boundary | Rust production/qualification and manual-dispatch-only hosted workflows remain owner constraints. Retained Quoin tooling is not permission to expand foreign runtime semantics. No public release, relicensing or source-copying decision occurs here. |

The historical native-source amendment is byte-identical to PR #11's preserved
`71db286` input. Its corresponding implementation/qualification obligations are
not discharged by this review.

## Validation and practical limits

On the frozen native input, `quire validate --scope . 'spec/**/*.md' --summary`
reported **232/232 grammar-clean documents, zero grammar findings**. Property
extraction reported 174/356 extractable criteria across the repository; that
is extraction availability, not proved correctness or scoped test coverage.
The 70-entry definition byte census passed, and F's seven integrated result
correction files match its supplied commit exactly. No duplicate frontmatter IDs
were found. The read-only link fixer primarily proposed adding Markdown links
to already-authored IDs; those proposals are not broken-reference counts.

These checks validate artifact shape and identity. They do not qualify the EBNF
against an implementation, execute the planned cases or establish backend
equivalence. No Cargo/build campaign or hosted workflow ran. The final review
artifacts receive structural validation separately from the frozen input.

## PR #15 correction recheck

A rechecked the affected definitions, contracts, planned controls and navigation
after integration. This completes the original PR-readiness review cycle; it is
not another architecture review or a claim that the original FAIL was a PASS.

| Finding group | Current disposition and evidence |
| --- | --- |
| SR-400 FND-001 | Resolved. The capability crosswalk now distinguishes integrated D/F/B/E contracts from outstanding implementation, and assigns the compiled payload/reader to compiler #40 rather than L2 parsing/linking. |
| SR-401/402/406 semantic boundaries | Resolved in the package and observation/range contracts with D's `6259d3a` input: static templates need no future observations; producer canonical numbers and native byte digests retain separate domains and exact correspondence; all three clocks retain coverage/member/record boundaries; complete exact support can settle truth despite unrelated incompleteness. |
| SR-403 definition selection | Resolved by the owner's single-current-draft workflow and the consolidated definition closure. Immutable runtime selections remain required; per-edit successor publication is superseded. FR-090 now depends on FR-030. |
| SR-404/407 planned controls and wording | Resolved by the integrated resource/provenance and method bindings, FR-091/TC-115 deadline/empty-window controls, and FR-093/094 wording corrections. The planned cases remain unexecuted. |
| SR-408 FND-001/002 | Resolved. The unsuffixed files contain the latest semantics and revision labels; obsolete copies are absent. Protocol draft 4 selects state graph and temporal common draft 3. All 99 relative file links across the 15 definition-directory documents resolve. |
| SR-408 FND-003/005 | Resolved. Obsolete internal digest selectors are removed; external selections and runtime digest checks remain. The ledger and definition index agree. Temporal roots retain their pre-consolidation meanings, including decisive settlement and activation-unknown distinctions. |
| SR-408 FND-004 and B/F closure follow-up | Resolved. Historical findings are dispositioned in shared-foundation.md, including SR-302 FND-005. FR-115/116, TC-145/146, IT-051 and the mapping contract preserve five independent axes, exact global premises, explicit mapping loss and immutable supersession. |
| PR diff cleanliness | Resolved at 9d47eca: removed 21 blank EOF lines. The earlier working-tree-only check did not establish PR-wide cleanliness; the full base-to-head check now passes. |

[B's consumer recheck](https://github.com/agent-ix/quire-specification/pull/15#issuecomment-5627318474)
and [E's temporal recheck](https://github.com/agent-ix/quire-specification/pull/15#issuecomment-5627369978)
confirm their owned integration gates at `d8d984b`. A's targeted recheck agrees.
SR-408's original file and evaluated FAIL verdict are unchanged.

Actual checks at `9d47eca`, with base `089f3bb`:

- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/2026-*.md' --summary`:
  **250/250 grammar-clean, zero grammar findings**.
- `git diff --check origin/main...HEAD`: **exit 0**, covering the complete PR.

The final review/navigation edits receive scoped validation and the complete PR
diff check before push. No new Rust behavior is included in these corrections;
the five passing reference tests recorded by SR-408 remain evidence at `3972783`,
not a newly executed current-head test or full-v1 conformance claim.

No open specification finding in this correction scope prevents the private
merge. FCD #97's own review/producer acceptance, the compiler #40 artifact
contract, concrete implementation/qualification and public rights/adoption
decisions retain their separate owners and evidence requirements. C's assurance
campaign does not gate this baseline.
