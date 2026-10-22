---
id: TC-145
title: "Report observation adequacy without promotion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-115
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-285
    type: verifies
---
## Description

Accepted FS04 portable OB03 result/adequacy controls. No executable standard coverage is claimed.

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
in an O1 result. Independently duplicate each required retained axis.
Supply global-conformance claims with the fifth typed closure record in each
state (`closed`, `open`, `incomplete`, `contradicted`) and its exact required
execution, branch and workflow premise identities. Independently omit or
duplicate that record and each required premise, cross-wire each to another
axis/scope, or substitute only decision-scope or surrounding-execution closure.
Supply a non-global claim with complete-global-conformance closure `not-required`
and no global premise set. Independently add a global premise, replace the
global claim's record with `not-required`, and replace each of the other four
progress/closure axes with `not-required`.

Drop trigger telemetry, mis-correlate a response, mutate assessment execution,
settlement basis, decision support, and each selected definition/profile
identity. Pair a confirmed injected fault with independently observed
mitigation, then introduce a late contradiction to retained decision support or
any retained closure axis. Exercise FR-285 known participation with empty,
singleton and multi-member required sets and all, some and no required instances
participating. Exercise unknown participation with known participants, missing
dependencies and typed reasons. Independently omit, duplicate, reorder and
cross-wire member references; mix variants; substitute a role definition,
component identity, display name and each E02 target participation label; and
compare empty, absent and unknown facts. Independently omit each required
admitted-observation, binding, capture, semantic-selection and subject identity
while holding any otherwise equal truth constant.

## Expected Results

All dimensions remain independently readable; dropped/mis-correlated inputs cannot
improve adequacy; fault/mitigation lineage retains property, binding, source, and
run identities. Decision-scope progress and closure, surrounding-execution
progress and closure, complete-global-conformance closure, and assessment
execution remain independently readable.
Settled truth retains its exact basis and deciding support under each compatible
surrounding-execution change. Omitting or cross-wiring any required retained
axis refuses the inconsistent contribution; explicit open/incomplete surrounding
execution is retained without erasing an independently supported settled truth.
Only a `closed` global closure record with every exact required premise permits
complete global success; open, incomplete and contradicted records retain their
non-success state without erasing independently settled per-obligation truth.
The non-global `not-required` record carries no global premises and cannot replace
another axis. Omitted, duplicated or cross-wired records/premises and invalid
`not-required` substitutions refuse. Definition drift refuses, and
a late contradiction preserves earlier bytes while producing only a linked
superseding or invalidating result.
Known and unknown participation retain their exact facts, empty is distinct from
absent and unknown, and mixed variants, duplicate or display-derived identities
and target labels in the source fact refuse without improving adequacy.
An equal disposition with any omitted required provenance identity fails rather
than satisfying reproducibility.
