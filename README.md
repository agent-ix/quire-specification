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

[Native state-core proposal](proposals/state-core/README.md) contains draft
grammar, identity/snapshot boundaries and serialized fixture expectations.
[Agent A's review record](reviews/agent-a-fs01.md) records inspected evidence,
owner decisions and remaining implementation gates. These are review artifacts,
not an accepted profile or a working compiler.
