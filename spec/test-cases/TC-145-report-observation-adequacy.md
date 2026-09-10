---
id: TC-145
title: "Report observation adequacy without promotion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-115
    type: verifies
---
## Description

Planned OB03 result/adequacy controls. No executable test or completed coverage is claimed.

## Test Procedure

Compare healthy, violating, untriggered, missing-data, and unsupported traces;
include a completed, triggered `eventually[0,30] holds(p)` result satisfied by
an admitted decisive witness and an `always[0,30] holds(p)` result violated by an
admitted decisive counterexample while both the decision scope and surrounding
execution remain open. Retain decision-scope progress, decision-scope closure,
surrounding-execution progress and surrounding-execution closure as four
independently identified facts. Vary surrounding-execution progress under the
selected contract while holding the decision-scope facts, settlement basis and
exact deciding support fixed; truth must remain unchanged. Also compare a settled
closed-scope result with the same decision-scope facts/support under compatible
open and closed surrounding-execution states.

Independently remove each of the four required retained progress/closure axes.
Then independently cross-wire decision-scope progress with surrounding-execution
progress, decision-scope closure with surrounding-execution closure,
surrounding-execution progress with decision-scope progress, and
surrounding-execution closure with decision-scope closure; mutate only the target
axis in each case. Repeat each substitution with the corresponding axis from O2
in an O1 result. Supply a complete-global-conformance claim with its required
closure premises present, then independently remove each required closure
premise or substitute only the settled obligation's decision-scope closure.

Drop trigger telemetry, mis-correlate a response, mutate assessment execution,
settlement basis, decision support, and each selected definition/profile
identity, revision, and digest. Pair a confirmed injected fault with independently
observed mitigation, then introduce a late contradiction to retained decision
support or either retained closure axis.
Independently omit each required admitted-observation, binding, capture,
semantic-revision, and subject identity while holding any
otherwise equal truth constant.

## Expected Results

All dimensions remain independently readable; dropped/mis-correlated inputs cannot
improve adequacy; fault/mitigation lineage retains property, binding, source, and
run identities. Decision-scope progress and closure, surrounding-execution
progress and closure, and assessment execution remain independently readable.
Settled truth retains its exact basis and deciding support under each compatible
surrounding-execution change. Omitting or cross-wiring any required retained
axis refuses the inconsistent contribution; explicit open/incomplete surrounding
execution is retained without erasing an independently supported settled truth.
Neither obligation settlement nor decision-scope closure supplies the missing
closure premises for complete global conformance. Definition drift refuses, and
a late contradiction preserves earlier bytes while producing only a linked
superseding or invalidating result.
An equal disposition with any omitted required provenance identity fails rather
than satisfying reproducibility.
