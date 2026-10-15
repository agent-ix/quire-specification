---
id: TC-110
title: "Select one exact temporal profile and clock"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
---
## Description

Accepted FS04 portable admission controls for exact temporal profile and clock
selection. No parser, evaluator or bridge coverage is claimed.

## Test Procedure

Create three otherwise identical native temporal subjects selecting the
event-position false-extension, fixed-sample false-extension and timestamped
finite-window profiles. For each, independently mutate the profile identity, clock identity, unit,
epoch/period, order authority and closure rule;
also supply zero and two profile selections and a backend that implements only a
different profile.

## Expected Results

The three valid subjects remain distinct. Each unknown, missing,
conflicting or multiply selected dimension refuses at the native clause without
evaluation. Backend availability never changes selection, and historical
profile bytes/results remain unchanged.
