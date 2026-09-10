# quire-specification

Private specification and qualification repository. The Rust tests qualify
authored interchange shapes and existing canonicalizer correspondence; no native
compiler, strict shared reader or released standard is implemented here.

## Charter

The open language standard: normative semantics, profiles, versioned interchange and required conformance material.

Native Quire v1 includes **state, temporal and protocol/choreography** feature
families under one editable language. The existing finite-state work is a
development profile. The [v1 redesign](proposals/quire-v1/README.md) defines the
broader product scope and its capability decisions; its specification and review
precede resumed profile-specific delivery.

## First work

The owner-approved [language research and capability assessment](proposals/quire-v1/README.md)
defines the expanded v1 roadmap. It preserves the first state profile as a
development foundation; it does not adopt unreviewed grammar or claim runtime
support for the newly selected capabilities.

Define scope and non-goals; distinguish language, profile, wire and implementation versions; reconcile model ownership with Filament; outline grammar/static/dynamic semantics; select open licenses for prose and reusable normative artifacts; define first positive/negative conformance cases. No unreviewed draft becomes normative.

## Boundaries

No duplicate evidence store, hidden normative dependency, or unqualified backend promise. Existing Quire/Quoin/Filament interfaces must be inspected before replacement or extraction. Research and private issue links must not enter a public release packet.

## Tracking

- [Creation ticket](https://github.com/agent-ix/quire-research/issues/29)
- [Program epic](https://github.com/agent-ix/quire-research/issues/28)
- [First specification issue](https://github.com/agent-ix/quire-specification/issues/1)
- [Private formalization board](https://github.com/orgs/agent-ix/projects/20)

Read [license decision](LICENSE-DECISION.md) before adding code or normative artifacts. Publication requires a fresh review; do not flip private history public.

## Proposed state-core packet

The [native-source ruling amendment](proposals/native-source-1-draft/README.md)
proposes the replacement definition and states its admission rules, historical
compatibility and remaining implementation gates. It does not replace the ongoing
cross-stream capability investigation or claim ruling conformance from old tests.

The [requirements index](spec/spec.md) covers FS01/FS02/FS03/FS05 through discrete
Quoin catalog artifacts. [Authoring status](docs/spec-workflow.md) records the
exact skills, validation results and outstanding review work.

[Native state-core proposal](proposals/state-core/README.md) contains draft
grammar, identity/snapshot boundaries and serialized fixture expectations.
[Agent A's review record](reviews/agent-a-fs01.md) records inspected evidence,
owner decisions and remaining implementation gates. These are review artifacts,
not a released standard or a working compiler.

The owner adopted the FS02/FS03/FS05 proposal at e897f81 as the target for
internal LC02 implementation on 2026-09-08. The
[decision record](docs/internal-implementation-adoption.md) preserves its exact
scope and the independent integration/publication gates.

## Verification reference amendment and local checks

[The versioned amendment](proposals/shared-reference-2-draft/README.md) adds B's
six artifact roles and selected JCS canonical identity without changing the
adopted state-core packet. [Fixture notes](proposals/shared-reference-2-draft/fixtures/README.md)
state which observations are qualified and which require B's strict reader.

Run these commands from this repository, using the pinned Rust 1.98.1 toolchain:

```sh
cargo test --locked --target-dir target
cargo fmt --all -- --check
cargo clippy --offline --locked --target-dir target --all-targets --all-features -- -D warnings
quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/2026-*.md' --summary
```

The first Cargo run may fetch the exact private B dependency; subsequent runs
can add --offline. All tests must execute locally; no hosted workflow exists.
The test-only crate has no production API. [Dependency rights](docs/qualification-dependencies.md)
record its grants and exact selected sources. TC completion and independent
consumer adoption remain separate from the five producer qualification tests.

The dated review glob selects the typed SpecReview artifacts. The two historical
agent-a review notes predate the artifact workflow and have no type frontmatter;
an unrestricted reviews glob reports those two existing structural errors.

## Required language

Executable implementation, including production and qualification tools/tests, is Rust. New TypeScript choices require explicit owner approval; surface all other executable languages. Quoin remains in its current implementation for now: contain its spread through versioned structured interfaces, do not rewrite it wholesale. Filament changes are excluded. See AGENTS.md and the language gate on the owning issue. Normative prose/schema need not be executable Rust.
