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

## Implementation language — owner directive

Executable implementation, including production and qualification tools/tests, is Rust. New TypeScript choices require explicit owner approval; surface all other executable languages. Quoin remains in its current implementation for now: contain its spread through versioned structured interfaces, do not rewrite it wholesale. Filament changes are excluded. See AGENTS.md and the language gate on the owning issue. Normative prose/schema need not be executable Rust.
