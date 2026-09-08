# quire-specification

Private bootstrap, 2026-09-07. Repository shell and specification work only; no implemented or released capability is claimed.

## Charter

The open language standard: normative semantics, profiles, versioned interchange and required conformance material.

## First work

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

## Implementation language — owner directive

Executable implementation, including production and qualification tools/tests, is Rust. New TypeScript choices require explicit owner approval; surface all other executable languages. Quoin remains in its current implementation for now: contain its spread through versioned structured interfaces, do not rewrite it wholesale. Filament changes are excluded. See AGENTS.md and the language gate on the owning issue. Normative prose/schema need not be executable Rust.
