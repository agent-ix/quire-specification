---
id: TC-117
title: "Prove or refuse native-to-TL temporal correspondence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Description

Accepted FS04 portable differential and identity-mutation controls for the
native temporal bridge. Contract IR FR-350/TC-461 allocate execution to the
bridge in `quire-integration` (IR-1005); publication of this procedure is not
runtime or exhaustive-coverage credit.

## Test Procedure

For event-position correspondence, exhaustively enumerate ASTs of depth at most
two from constants, propositions `{p,q}`, negation, conjunction and the four
future operators using every interval `0 <= a <= b <= 3`; enumerate trace
lengths zero through five, every total Boolean valuation, every valid anchor and
both closed and open-prefix lifecycle states. Include a proposition-only formula
under both false-extension profiles and both surrounding-execution closures;
require closed-trace for closed execution and online-prefix for open execution. Record the population cardinality
and any infeasible resource stop. For fixed-sample correspondence, exercise the
same finite population under two declared rational periods and epochs. Compare
an independent native oracle with the exact selected TL evaluator profiles.
Include the one-position `always[0,1] p` closure discriminator. Request
timestamped finite-window; every bounded-past operator defined by FR-092 alone
under both false-extension profiles with an execution-origin boundary and with
a history-cutoff boundary; and a past operator combined with a future operator. Independently
mutate every native/TL profile, source/span, model/predicate, clock, capture,
interval, closure/history and bridge identity. For each mapped result,
independently omit or change assessment execution, decision-scope progress and
closure, surrounding-execution progress and closure, settlement basis, one
decision-support identity, completeness and supersession. Offer numeric,
nullable, unavailable and pending proposition inputs.

Before each projection, use the actual checked-package/v2 subject and QSL
classification. Admit the same producer bytes independently a second time and
cross-wire the original subject/classification with it; separately test an
unchanged clone of the original admission and the original subject borrow.
Mutate one public support, premise or retained classification field while
keeping the sealed selection, then reclassify the original bound subject.
Require refusal before TL construction for every foreign admission or changed
classification; keep clone admission membership distinct from view borrow
membership.

Use two occurrences that share a clause node but have different original
node/role/ordinal keys and name/clock rows. Select each explicitly; cross-wire
an occurrence, valuation instance, admitted observation or owner numeric
anchor. Read selected model package identities in their owner-admitted order,
and require no caller-authored or inferred source model alias map. Retain the
actual owner-read formula, trace/history, request and mapped-result values and
check exposed typed members, public names/versions and definition references
rather than a content digest or bridge record reference.

For each corrected result, supply the actual complete immutable direct
predecessor values. Substitute a different complete native or TL owner result
with the same result identifier; also try a missing, skipped and foreign
predecessor. Validate lineage through the owners' non-evaluating operations,
without invoking either evaluator from projection, join or strict reading.

## Expected Results

Every member of the declared finite population agrees and retains full
correspondence, or a resource stop is reported without an exhaustive claim.
A proposition-only formula selects the closed-trace target for closed
execution and online-prefix for open execution under both false-extension
profiles. Finite-window, timestamped and mixed past-and-future mappings return
typed unsupported without a substitute formula; each past-only mapping with an
execution-origin boundary maps
to `mltl.origin-complete-history/v1` and agrees with the native oracle, and with a
history-cutoff boundary is `incomplete` with cause `missing-history`. Each identity mutation fails on its own axis, and only total
native Boolean values enter the TL proposition population. Equal TL truth or
formula bytes never supply a missing result axis; every valid mapping retains
the native/shared-result metadata outside the TL formula where necessary.

Foreign admissions, changed classification facts and cross-wired occurrence,
instance or observation selections refuse before any TL output is admitted.
Owner-read typed values and selected model package order remain exact; no
source alias, admission capability or artifact identity is invented. A
same-identifier but different full predecessor establishes no supersession,
and every non-final join has no agreement Boolean. These are planned bridge
acceptance procedures; no test or integration row becomes Passed merely by
publishing this spec correction.
