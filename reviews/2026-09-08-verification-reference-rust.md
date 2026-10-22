---
id: SR-017
title: "Code and Rust review of verification reference qualification"
type: SpecReview
analysis: code-review
scope: "Cargo.toml, Cargo.lock, tests/shared_reference.rs and amendment fixtures"
review_set: subset
review_date: "2026-09-08"
---

## Summary

The Rust qualification crate validates the new schema and fixture bytes and
calls B's real pinned canonicalizer. It has no production shared-reader API.

## Verdict

PASS for the implemented producer/schema/JCS qualification scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved code finding in the qualified producer scope. | tests/shared_reference.rs; Cargo.lock |

## Applied review

Used the actual /home/peter/dev/agent-skills/code-review/SKILL.md and
/home/peter/dev/agent-skills/rust-review/SKILL.md, plus rust-style defaults and
this repository's AGENTS.md. No applicable AssuranceProfile exists in spec/.
The owner-selected bare single-line ix_trace_rs::trace attributes override the
skills' historical doc-comment examples. Missing inputs fail the required lane;
they do not skip. Optional gap-analysis semantic comparison was not requested.

## Rust and architecture checks

Only tests and private fixture helpers are introduced. The crate has no public
library, exported error envelope, production CLI, hidden reader, unsafe code,
async task, lock, clock-dependent assertion or alternate canonicalizer.
The private test-only IntakeFailure enum is not a published error boundary.
Recoverable production errors are not converted into panics by a new API;
fixture load assertions fail setup in this required qualification test lane.

The loader bounds file reads, restricts authored relative fixture paths and
checks selected size/depth boundaries. The JSON match is exhaustive. Tests use
real schema validation and actual B library calls. Golden canonical bytes were
authored independently before implementation; expected SHA-256 is computed over
those fixed bytes rather than the canonicalizer's own returned bytes.

All five tests carry compiler-checked IT success-criterion tags. They intentionally
do not claim complete TC-001–004/FR-017–018 consumer coverage. The schema accepts
nearby valid roles while the negative vector prescribes a semantic refusal;
the test does not pretend a local equality helper is B's real strict reader.
Full source/identity matching and report assembly remain B-owned.

## Resolved observations

The initial prototype assumed B's legacy bounded JSON helper rejected duplicate
decoded keys. Actual execution failed that assertion (four tests passed, one
failed; the duplicate map collapsed to a single a entry). B's own FR-016/TC-029
already requires that pending strict-reader behavior. The implemented test was
scoped to its actual producer/JCS boundary and the raw negative fixture remains
required for IT-003-SC-03. This is an explicit limitation, not a duplicate-intake
qualification or a disabled passing test.

The first serde_json pin conflicted with B's typify requirements; 1.0.151 resolves
the actual graph. The first rights draft mislabeled ix-trace-rs; actual Cargo
metadata established AGPL-3.0-or-later and the committed inventory is corrected.
These observations are retained rather than claimed never to have occurred.

## Actual gates

```text
cargo +1.98.1 fmt --all -- --check
exit 0
cargo +1.98.1 clippy --offline --locked --target-dir target --all-targets --all-features -- -D warnings
exit 0
cargo +1.98.1 test --offline --locked --target-dir target
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cargo metadata records all 114 selected packages, including this crate, with no
missing declared license. There is no deny.toml; no cargo-deny result is claimed.
The adopted state-core and LICENSE-DECISION.md have zero diff from e897f81.
No B workspace, hosted workflow or public issue was changed.

The first unrestricted root-review validation also selected the two historical
untyped agent-a notes and failed on their missing type fields. The documented
scope now explicitly selects dated SpecReview artifacts; it does not claim those
historical notes have become typed. All changed spec, plan and dated review
artifacts validate, with the known registry duplicates retained.

## Implementation gap discovery

The only new executable surface is the reviewed qualification test crate.
Its bounds, byte hashes, closed-schema controls and canonicalizer correspondence
are owned by IT-003 and FR-017/018. No unspecified production capability or
stub is introduced. Full reader/report and native state evaluation are still
unimplemented within this repository's scope and are not masked by green tests.
The companion gap review records the remaining plan/matrix work.
