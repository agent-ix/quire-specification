---
id: TC-005
title: "Select composed native definitions without fallback"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-001
    type: verifies
---
## Description

Distinguish the adopted historical definition, the superseded replacement
candidate and an adopted composed definition. This is planned compiler
acceptance, not an executed standard test.

## Test Procedure

Verify the `proposals/state-core` definition and fixture bytes against the
digests pinned in `typing-cases.json`, `reference-cases.json` and
`SHA256SUMS`. Select the unavailable
superseded candidate through the actual parser/package reader and assert a
located refusal. After an owning semantic ticket explicitly adopts a composed
definition, select its exact version/digests and mutate each independently;
attempt replay of an old semantic subject under the new selection. Use a valid
exact selected definition as the positive control.

## Expected Results

Historical bytes and subjects retain their meaning. An unavailable or mismatched
definition yields no checked clause and never falls back. A supported exact
composed selection binds its own definition and new derivation. The superseded
candidate is never aliased to either selection.
