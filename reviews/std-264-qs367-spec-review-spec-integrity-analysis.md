---
id: SR-15514
title: "integrity review of STD-264 FR-323 missing occurrence_key refusal (QSpec PR #367)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@daaf4e892ecf5537d43002202eb1c3324b699270; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---
# SR-15514: integrity review of STD-264 FR-323 missing occurrence_key refusal (QSpec PR #367)

## Summary

Ticket: STD-264. Companion to QSL #1221, raised from SR-15513 FND-003. Diff `origin/main...daaf4e892`
(QSpec main 429c59d), 2 files. It changes three things:

- Decode rule 1 gains a last clause: a `function` request with no `occurrence_key` refuses
  `invalid_runtime_input`/`missing-member` at `/replay/occurrence_key`.
- FR-323-AC-22 is new.
- TC-278 gains RE-42: RE-01's request without its key, then also without a byte-provision entry.

The clause is in rule 1, so the refusal comes at decode, before the byte-provision rule and any
recompile. The rule's "first that applies" ordering puts the target-member refusals ahead of it. The
code and pointer match QSL FR-071 and FR-071-AC-15. RE-01 is a `function` request that carries an
occurrence key, so removing the key isolates the new case. AC-22 and RE-42 are unique ids. Gates:

- `cargo test --locked` passes, the same as on main.
- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/2026-*.md' --summary` prints
  the same lines as on main, apart from the expected +1 criterion count. No new failure.

No QSL conformance run is needed yet. TC-278 vectors are run by QSL's replay tests, and QSL's
`ReplayRequestWire` has no `occurrence_key` member yet. The code lane's TC-185 step 7 should cite
RE-42 when it lands.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Mergeable, and it must merge before QSL #1221.
