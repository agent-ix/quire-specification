---
id: SR-727
title: "Spec review — PR #164 NFR-061 drop observer tracking"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@3dcefcf070a0b182a19b853932d3c725b25742d2; PR #164 diff against origin/main e56756f: NFR-061, AD-004, TC-155, spec/output-mapping/tests.md, spec/spec.md; cross-checked FR-120, FR-122, FR-123, FR-124, FR-297, TC-151, TC-152, TC-153, plan/Plan-007"
review_set: subset
---
# Spec review — PR #164 NFR-061 drop observer tracking

## Summary

Ticket: none found (branch `task/nfr061-drop-observer-tracking`, title
"NFR-061: ..."; `NFR` is not a Linear team key). Integrity pass over the
PR #164 spec diff.

The removals are clean. No version, invocation, dependency-identity, licence or
rights wording and no Rust 1.98.1 pin remains in NFR-061, TC-155, the NFR-061
rows of `spec/output-mapping/tests.md` and `spec/spec.md`, or the NFR-061
lines of AD-004. FR-122/123/124 and TC-151/152/153 carry no such wording
("revision `1-draft.1`" there is profile selection, not tracking). The kept
behaviour survives: an optional downstream observer, a pure Rust mapping
result that does not depend on it, a typed refusal with the package unchanged,
observer results never used as native truth, no mandatory foreign-runtime
dependency, and `publish = false`. `quire validate` output is identical to
origin/main apart from the HEAD line (both exit 1, 142 documents, same 334
error lines).

## Verdict

**Changes requested.** One medium finding and two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NFR-061 still states "SHALL use Rust with `publish = false`", but the PR deleted the only metric row and verification step that checked it. That clause now has no measurement. Add `publish = false` to the Cargo.toml verification sentence, which already reads the same manifest. | spec/non-functional/NFR-061-contain-output-mapping-dependencies.md:16-17,24-34 |
| FND-002 | low | "Cargo.toml review" does not catch a mandatory foreign runtime that is started as a subprocess (for example `java`), which is how an external parser/tool usually runs. The observer-removal test catches this for the observer only. Name the removal test as a method on that metric row too. | spec/non-functional/NFR-061-contain-output-mapping-dependencies.md:26,31-33 |
| FND-003 | low | Plan-007 still describes NFR-061 as "exact dependency/version/rights inventory". This text repeats the removed wording and is outside the PR's file list. Delete the clause. | plan/Plan-007-fs06-output-mapping-acceptance/plan.md:39-40 |
