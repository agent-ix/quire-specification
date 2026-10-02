---
id: FR-305
title: "Load explicit Rust plugins"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
  - target: ix://agent-ix/quire-specification/FR-462
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
---

# FR-305: Load explicit Rust plugins

## Description

When a caller names a plugin, the host SHALL register the plugin's FR-331
manifest, route items to it only through capability negotiation, and read
every plugin result with the typed FR-331 reader before it settles any item.

## Inputs

A compile-time Rust provider linked into the binary, or the path of a plugin
executable; the items routed to the plugin; the caller's run budgets.

## Outputs

One FR-331 terminal record per item routed to the plugin, or a typed
tool-failure or `incomplete` settlement of each affected item.

## Behavior

A plugin is either a compile-time Rust implementation of the provider trait,
linked into the binary, or an executable that runs as its own
operating-system process and speaks the `quire.plugin-wire/v1` wire
([FR-462](FR-462-speak-the-plugin-wire.md)). The host loads no dynamic
library. An installed plugin is a trusted program, like any program the user
installs, and runs with the user's own authority.

## Provider boundary

- **Registration.** The registry is built from the compile-time providers
  linked into the binary and the plugin processes the caller names, each
  giving its FR-331 manifest (a process plugin in its `hello` frame). Every
  manifest becomes a backend descriptor by the same conversion. A plugin
  cannot alter admission, source identity or native meaning: the front end
  takes no registry (FR-339-AC-3).
- **Negotiation.** An item reaches a process plugin only when FR-290
  negotiation settles it from the plugin's advertised (kind, mode) pairs,
  domains and bounds. Negotiation never calls the plugin.
- **Run budgets.** The caller's deadline, frame and total byte limits, and
  the process memory and CPU-time limits where the platform provides them,
  bound each plugin run.
- **Typed output.** Every plugin result is read with the typed FR-331 reader.
  A plugin never produces a checked object: it receives v2 bytes, and the IR
  reader revalidates their `package_id`.
- **Counterexamples.** A plugin's counterexample is canonical assignments.
  The item settles `refuted` only when the reference evaluator's replay
  reproduces it.
- **Trusted proofs.** A plugin's `proved` result cannot be replayed and has
  no in-core certificate checker. Its terminal record keeps the result
  `proved`, names the plugin's backend identity and carries the basis label
  `trusted`. A renderer shows the label beside the verdict.
- **Failures.** A crash, malformed output, an undeclared capability and every
  other FR-462 wire failure settle each affected item FR-331 `failed` with a
  typed tool-failure cause and never a verdict.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-305-AC-1 | A compile-time provider and a process plugin each register through their FR-331 manifest, receive only the items negotiation routes to them, and return results that the typed FR-331 reader admits; registering either changes no checked package byte. | Test (TC-225) |
| FR-305-AC-2 | A plugin `proved` result settles `proved` with the plugin's backend identity and the basis label `trusted`, and the rendered outcome shows `trusted` beside the verdict. | Test (TC-225) |
| FR-305-AC-3 | Malformed, oversized, timed-out or crashing plugin output settles each affected item `failed` or `incomplete` with its typed cause, never a verdict, and items routed elsewhere are unaffected. | Test (TC-225) |
| FR-305-AC-4 | A plugin counterexample settles `refuted` only when replay reproduces it; a counterexample replay does not reproduce settles `inconclusive`. | Test (TC-225) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.

## References

- Linear STD-141; QSL ADR-029 PL-1, PL-7, PV-1, RU-4.
