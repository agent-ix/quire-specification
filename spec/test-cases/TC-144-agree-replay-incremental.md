---
id: TC-144
title: "Agree replay and incremental assessment"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-114
    type: verifies
---
## Description

Accepted FS04 portable OB02 replay/incremental controls. No executable standard coverage is claimed.

## Test Procedure

Run the same admitted O1/O2 observations and progress through batch and
incremental paths. On an open subject, use an admitted `p` witness at 29 for
`eventually[0,30] holds(p)`, and independently an admitted `not p`
counterexample at 29 for `always[0,30] holds(p)`; retain the open
progress/closure input in both controls. Use a third open prefix with neither
decisive basis, a missed-deadline obligation, missing history, ambiguous
membership, and an unsupported obligation.

At the declared active-key and retained-history limits, record the actual
retained active-key and history counts before and after assessment; repeat one
key and one history item over each limit. The one-over controls must observe
that no allocation or retained state exceeds its declared bound, rather than
only observing a refusal.

## Expected Results

Batch and incremental assessment agree on the satisfied truth and its decisive
witness, the violated truth and its decisive counterexample, and the
missed-deadline result. The first two remain settled while progress/closure is
open; only the no-basis prefix remains pending. Every other adverse control
stays incomplete or refused as applicable and never becomes a
production-monitor execution claim. Exact limits retain no more than their
declared active-key/history state; one-over results name their subject and
exceeded bound and retain no excess state.
