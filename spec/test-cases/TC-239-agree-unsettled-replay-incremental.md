---
id: TC-239
title: "Agree replay and incremental assessment on an unsettled temporal result"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-114
    type: verifies
---

# TC-239: Agree replay and incremental assessment on an unsettled temporal result

## Description

Non-conclusive-agreement controls for FR-114-AC-4. Confirms QObs batch and
incremental assessment agree on a temporal result carrying settlement basis
`unsettled` and neither mode resolves it to a settled truth.

## Test Procedure

Use an infinite-trace obligation that returns `inconclusive`
([FR-161](../functional/temporal/FR-161-analyze-infinite-trace-properties.md),
[FR-341](../objects/temporal/FR-341-infinite-trace-result-disposition.md))
because its finite observed prefix has neither a conformance lasso nor a
complete bad prefix. Run the same admitted observations and progress through
the batch and incremental QObs paths. Record the reported disposition label,
settlement basis and progress identity from both paths. Extend the observed
prefix with one further admitted event that still does not close a lasso or bad
prefix, and rerun both paths. Independently attempt to force the incremental
path to report `satisfied`/`violated` for the unchanged obligation while its
basis remains `unsettled`.

## Expected Results

Both paths report the same `inconclusive` disposition, the same `unsettled`
settlement basis and the same progress identity for the initial prefix. After
the extension, both paths again agree with each other, and neither path
resolves the obligation to a settled truth. The forced-resolution control is
refused: an `unsettled` basis cannot be reported alongside a `satisfied` or
`violated` truth in either mode.
