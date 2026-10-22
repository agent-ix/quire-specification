---
id: SR-15860
title: "spec-review of the stale-dependency inconclusive cause FR-331 and TC-431 (quire-specification PR 373)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@273b57e42; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/TC-431
    type: references
---

# Spec review of the stale-dependency cause (quire-specification PR 373)

## Summary

Ticket: STD-266 (QSpec half; paired with quire-spec-language PR 1246,
SR-15870, which merges after this one). One spec-review pass over the PR
diff at 273b57e, with a code-review lane over the schema and vector JSON
folded in. Oracles: QSpec FR-331, FR-399, TC-431, and QSL FR-121, FR-127,
FR-169, FR-282, FR-289, FR-290, FR-357, FR-358 and ADR-015 at the #1246
head.

Gates, run in a separate worktree at 273b57e:

- `quire validate --scope . 'spec/**/*.md'`: 86 failure lines, the baseline.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 23 and
  119 passed, including the new positive result 4 and the adverse vector
  `stale-dependency-without-recompiled`.
- QSL conformance: no QSL source reads `backend-provider-v1`.
  `cargo test -p qsl-package --lib checked_v2` with `QSPEC_DIR` set to this
  head passed 42.

Examined, clean:

- Wire shape. The schema arm `{type: stale-dependency, requested,
  recompiled}` (each a `PackageId`, both required, no other members), the
  FR-331 inconclusive table cell and the QSL FR-127 spelling
  `stale-dependency{requested, recompiled}` all agree.
- Vectors. Result 4 gets a disposition and an accounting row, and the
  owners map gains `/results/4/cause` owned by FR-331. The adverse patch
  removes `recompiled` at that pointer. The placeholder digests follow the
  file's existing style. They derive from nothing, so there is nothing to
  recompute.
- Core-only placement. Adding the cause to FR-331's core-only causes makes
  the existing `unknown-cause` row reject it in a plugin's entry.
- `requested` means the request's `package_id` or the certificate's. That
  matches QSL FR-169's `PackageMismatch{certificate, recompiled}` mapping.
- FR-331-AC-35 and BP-26 are unique on main and in the other 13 open PRs.
  No other open PR touches `vectors.json`.
- No status wording and no compat layer.
- Lead defaults (the spelling, `inconclusive`, core-only) are sound.

Mergeability: MERGEABLE, and origin/main (5ee0071) is an ancestor of the head.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [major] The new paragraph says `stale-dependency` settles "an item whose replay, or whose core certificate check, recompiled a package whose `package_id` differs from the one the replay request or the certificate names". That covers every replay and every FR-282 checker. QSL contradicts it in three places. First, `TerminalValue::from_replay_refusal` (FR-121-AC-16, used by FR-289-AC-6 and FR-358) maps every non-fault refusal, `PackageIdMismatch` included, to `replay-refused`. Second, FR-357-AC-11 and FR-358-AC-3 settle a stale `package_id` as `ReplayRefused(code)`. Third, QSL's lead default keeps `DependencyIdentityMismatch`, a stale `dependencies` entry the request also names, as `replay-refused`. The PR body lists the scalar-parity paths as ungrounded instead of settling them. Fix (a planner decision): narrow the sentence to the proved package's own `package_id` in a model-check or state-graph replay and in the state-graph certificate check, and say every other replay refusal stays `replay-refused`. Or keep the general sentence and have QSL #1246 move every path. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:519-524 | wrong-requirement |
| FND-002 | medium | [minor] FR-331-AC-35 is verified `Test (TC-431)`, but its third clause (a plugin's `stale-dependency` entry refused `invalid_result`/`unknown-cause` at `/cause/type`) is a reader cause the schema run cannot produce. BP-26 does that clause "by inspection", and FR-331-AC-34 verifies the same kind of plugin refusal (`replay-parity`) as `Inspection`, noting that "the schema run does not discharge the cause". Move the plugin clause into AC-34 and BP-25, or give AC-35 both methods. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:696; spec/test-cases/TC-431-backend-provider-wire-vectors.md:53 | correct-requirement-no-evidence |
| FND-003 | low | [minor] The `stale-dependency` paragraph sits between two bullets of the list introduced by "A plugin's entry ... so:". That splits the list, and the last bullet ("A plugin's `counterexamples` entry carries ... no `transcript`") now follows a paragraph about core replay, cut off from its lead-in. Move the paragraph after the list. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:514-526 | correct-requirement-no-evidence |

## Verdict

CHANGES REQUESTED. FND-001 blocks merge. It contradicts QSL FR-121,
FR-289, FR-357 and FR-358 as written, and resolving it needs a planner
decision on how far `stale-dependency` reaches. The pair should apply the
same answer (SR-15870 FND-002). FND-002 and FND-003 are quick fixes. The wire
shape, the schema arm, the vectors, the core-only placement and the gates
are clean. Merge this PR before #1246.
