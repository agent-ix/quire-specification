---
id: SR-20261104200626
title: "code-review review of IR-1005 temporal bridge spec (quire-specification)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; IR-1005; spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md; spec/test-cases/TC-117-native-tl-correspondence.md"
review_set: subset
---

## Summary

Ticket IR-1005, PR 629. Reviewed the temporal bridge specification change using the code-review method.

## Verdict

**PASS** — No defects found in this method.

Reviewed only the two changed Markdown artifacts. No production code, schema, fixtures, dependency manifests or executable tests change; Rust review and implementation gap analysis are not applicable. No copied owner carrier, generated identity, digest/pin machinery, parser callback or evaluator seam is introduced. Existing public QSL source was inspected as contract evidence, not compiled runtime qualification.

No active AssuranceProfile document applies to these changed artifacts. No Cargo tests or final aggregate gate ran in this review; authored acceptance procedures remain planned. Actual owner-source inspection is not runtime evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined Scope

```yaml
scope:
- id: FR-095-AC-1
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: Independent native and TL evaluators emit equal truth/pending outcomes
    for every member of the declared finite exhaustive bounded-future formula/trace
    population in the event-position mapping domain; a proposition-only formula under
    either false-extension profile selects the matching target from surrounding-execution
    closure.
- id: FR-095-AC-2
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: The one-position `always[0,1] p` vector rejects a finite-window-to-`mltl.closed-trace/v1`
    equivalence claim.
- id: FR-095-AC-3
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: Timestamped-event and mixed past-and-future requests return typed unsupported
    mappings without emitting a substitute formula; a past-only request with origin-complete
    history maps to `mltl.origin-complete-history/v1`, and with a history-cutoff boundary
    is `incomplete` with cause `missing-history`.
- id: FR-095-AC-4
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: Mutating any native/TL profile, source/span, model/predicate, clock, capture,
    interval, closure/history or bridge identity breaks correspondence and names that
    axis.
- id: FR-095-AC-5
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: A numeric, nullable, unavailable or pending native value cannot enter the
    TL proposition population.
- id: FR-095-AC-6
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: Fixed-sample lowering retains epoch/period/unit, reports an absent runtime
    sample as incomplete, and refuses a changed period or event-count reinterpretation
    even where the TL formula bytes match.
- id: FR-095-AC-7
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: A supported TL result is accepted only when the correspondence record preserves
    assessment execution, both progress/closure axes, settlement basis, exact decision
    support, completeness and supersession; mutating or omitting one axis cannot yield
    a healthy native result.
- id: FR-095-AC-8
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: Every generated formula, trace/history, request and mapped result passes
    its target owner's public reader and is retained as the exact admitted typed value
    with its public name/version and selected definition references. Mutating a preserved
    typed key changes the corresponding exposed member and cannot pass strict rereading;
    no content hash or synthetic record reference substitutes for an owner value.
- id: FR-095-AC-9
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: context_only
  excerpt: An infinite-trace unbounded-property request returns a typed unsupported
    mapping naming the absent TL target without emitting a substitute formula.
- id: FR-095-AC-10
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: 'Independently admit identical producer bytes twice and obtain a subject/classification
    from the first admission: projection with the second admission refuses before
    TL construction. Original admission membership survives an unchanged package clone,
    but a subject borrowed from one package cannot authorize another borrow. Changing
    a public classification disposition, premise or retained fact while keeping its
    sealed selection refuses on complete reclassification.'
- id: FR-095-AC-11
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: Two occurrences sharing a temporal clause node retain distinct node/role/ordinal
    selections and their original name/clock rows; selected model package identities
    retain their checked-owner order and no source alias is inferred. Missing or substituted
    occurrence/instance/observation selections refuse before TL construction. For
    a correction, substituting a different complete prior owner result with the same
    result identifier fails non-evaluating predecessor validation and yields no final
    agreement Boolean.
- id: FR-095
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: 'The bridge shall consume the actual checked package, subject view and

    classification through the public owner APIs, preserving the opaque

    admitted-package binding that QSL retains from the checked-package owner. It

    shall first verify the subject belongs to the supplied borrowed package and

    the classification retains that package''s admission, then reclassify the

    subject with the same surrounding-execution closure and compare the complete

    result. Equal bytes, graph contents, package names, profile definitions or

    clause nodes do not authorize a separately admitted package. An unchanged clone

    may '
- id: FR-095
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: 'An admitted `quire.checked-package/v2` package, its QSL FR-515 checked
    temporal

    subject view and the QSL FR-527 classification of that subject against FR-045''s

    support table; the native owner''s admitted evaluation evidence and actual

    FR-052 report; total-Boolean predicate projections that Contract IR FR-350

    builds from the native `holds` leaves and explicit valuation cells; selected

    observation and clock bindings; and an exact bridge-definition identity naming

    the TL syntax and evaluator profiles it targets. Occurrence and instance

    selections are independently bound inputs, not consequences of'
- id: FR-095
  path: spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md
  role: examined
  excerpt: "- Contract IR FR-350 implements this correspondence: it constructs and\n\
    \  validates the formula and valuation projection, correspondence record and\n\
    \  result join. This requirement supplies the temporal meaning.\n- QSL FR-045\
    \ owns the support table; FR-515 supplies the checked temporal\n  subject of the\
    \ admitted checked-package/v2 and FR-527 supplies its\n  authenticated classification\
    \ and owner-issued package binding. FR-052/FR-043\n  supply the actual native\
    \ evaluation report and complete predecessor\n  validation. QSL does not construct\
    \ the TL correspondence. FR-050's legacy\n  compiled-protocol `/"
- id: TC-117
  path: spec/test-cases/TC-117-native-tl-correspondence.md
  role: examined
  excerpt: 'For event-position correspondence, exhaustively enumerate ASTs of depth
    at most

    two from constants, propositions `{p,q}`, negation, conjunction and the four

    future operators using every interval `0 <= a <= b <= 3`; enumerate trace

    lengths zero through five, every total Boolean valuation, every valid anchor and

    both closed and open-prefix lifecycle states. Include a proposition-only formula

    under both false-extension profiles and both surrounding-execution closures;

    require closed-trace for closed execution and online-prefix for open execution.
    Record the population cardinality

    and any infeasib'
- id: FR-350
  path: agent-ix/quire-contract-ir/spec/temporal/functional/FR-350-native-temporal-tl-correspondence.md
  role: context_only
  excerpt: The correspondence record SHALL retain the package name, declaration and
    clause names, selected source `{authority, identity}` and span, selected model
    package identities from `CheckedPackageV2::selected_model_packages` in its admitted
    identity order, model/type/predicate bindings, anchor, captures, clock and observation
    binding, every interval, closure/history premises, the exact owner-admitted TL
    formula, request, trace/history and valuation values, their public versions and
    the bridge definition `{authority, identity}`.
```
