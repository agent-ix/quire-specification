# quire-specification

Private specification and qualification repository. The Rust tests qualify
authored interchange shapes and existing canonicalizer correspondence; no native
compiler, strict shared reader or released standard is implemented here.

## Charter

The open language standard: normative semantics, profiles, versioned interchange and required conformance material.

Native Quire V1 includes the complete
[atomic capability inventory](docs/v1-capability-inventory.md) under one
editable language and its typed runtime, analysis, proof, mapping and tooling
consumers. The existing finite-state work is a development profile. The
[historical V1 design](proposals/quire-v1/README.md) remains evidence for the
initial state, temporal and protocol/choreography expansion; Plan-008 supersedes
its deferred scope decisions. Complete architecture, specification and review
precede resumed delivery.

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

This repository is licensed under `AGPL-3.0-or-later`; see [LICENSE](LICENSE). Record the provenance and grants of any copied or generated artifact before adding it. Publication requires a fresh review; do not flip private history public.

## Native profile packets

The [native-source ruling amendment](proposals/native-source-1-draft/README.md)
preserves the superseded `state-finite/1-draft` candidate, its carried-forward
admission rules and historical rationale. FS01's accepted
[language charter](spec/assurance/AD-003-native-quire-language-charter.md) selects
the composed profile hierarchy instead. Neither artifact claims broader
implementation or qualification from historical tests.

The [requirements index](spec/spec.md) covers FS01/FS02/FS03/FS05 through discrete
Quoin catalog artifacts. [Authoring status](docs/spec-workflow.md) records the
exact skills, validation results and outstanding review work.

[Atomic V1 capability inventory](docs/v1-capability-inventory.md) is the binding
scope authority. The [functional capability ledger](docs/functional-capability-ledger.md)
is a broad cross-stream roll-up of source authorities, evidence and routing
decisions; it cannot defer an atomic capability and is not an
implementation-completion claim.

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
cargo test --locked --target-dir target-codex-backends
cargo fmt --all -- --check
cargo clippy --offline --locked --target-dir target-codex-backends --all-targets --all-features -- -D warnings
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
