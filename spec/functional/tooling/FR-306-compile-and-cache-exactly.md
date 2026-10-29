---
id: FR-306
title: "Execute through equivalent backends and cache by content key"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-305
    type: references
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
---

# FR-306: Execute through equivalent backends and cache by content key

## Description

When a checked entry is executed through the interpreter, an ahead-of-time
(AOT) backend or a just-in-time (JIT) backend, the backend SHALL return the
interpreter's exact outcome.

When a `prove`, `analyze` or `generate` result is cached, the cache SHALL key
it by one canonical content identity over every input that can change it.

## Inputs

A checked package, the selected entries, arguments, objects and accounting
limits, the selected execution backend and its preparation limits; for the
cache, the operation, the subject's `package_id`, the canonical request, the
provider's `BackendId` and manifest descriptor (FR-331), the generation target
and a store directory the caller names.

## Outputs

The interpreter's exact evaluation outcome, or a typed preparation failure;
a cached result equal to the fresh result for the same key, or a miss.

## Execution backends

The interpreter, AOT and JIT backends implement one execution-backend
interface. Each prepares the selected entries of one checked package, and
only a checked package is a preparation input. An entry a backend cannot run
settles `unsupported` for that entry only. The interpreter is the reference:
it defines what a checked package means, and counterexample replay runs only
through it.

The caller selects the backend in the request. The CLI selection is
`--engine interpreter|aot|jit`, with `interpreter` as the default. A name
outside that set refuses as an unknown argument (exit 20). A name in the set
whose backend the binary does not link refuses before any stage as
unsupported (exit 21). A backend never substitutes another backend.

An AOT or JIT run returns the same outcome as the interpreter for the same
package, entry, arguments, objects and accounting limits:

1. **Determinism.** The result depends only on those inputs. Generated code
   reads no clock, random source, environment or thread state, and visits
   collections in the interpreter's canonical order.
2. **Exact arithmetic.** Every value operation has the kernel's exact
   semantics: integers and rationals are exact, and decimal and IEEE
   operations follow the selected profile exactly. A machine-width fast path
   is used only where a checked bound shows the exact result fits.
3. **Outcomes.** Completed, undefined, refused and incomplete outcomes match
   the interpreter's, with the same typed cause, the same location and the
   same loss records.
4. **Budgets.** For every accounting limit, the run is `incomplete` at the
   same charge point and limit as the interpreter's run.
5. **Limits.** Size and work are bounded by the caller's limits, checked
   explicitly, never by depth (FR-460); host stack exhaustion is never the
   failure. Compile work and code size are bounded by the preparation limits,
   and reaching one is a typed limit failure of preparation.
6. **Failures.** An internal backend error is an internal failure (exit 30)
   and never a verdict.
7. **No effects.** Generated code performs no I/O and calls only the kernel
   and the semantic-value runtime.

The outcome carries no member naming the backend that ran. Code generation
negotiates per node before emission, and an item not settled `supported`
emits nothing (FR-331-AC-2).

## Cache

- **What is cached.** `prove` terminal records, `analyze` terminal records
  and `generate` artifacts of first-party providers: a first-party provider
  is one the producer runs as its own engine (FR-305), linked or as a
  process, whether it is a pure in-process engine or fronts an external tool
  (FR-331 "Backend tool"). A plugin's results are never stored. Parsing,
  checking, packaging, execution and monitoring run every time.
- **Key.** One structural key: the closed object
  `{operation, package_id, request, backend, manifest, target}`, compared
  componentwise, with no hash computed:
  - `operation`: `prove`, `analyze` or `generate`;
  - `package_id`: the subject's FR-322 identity, whose preimage binds the
    source, profile, model and dependency selections;
  - `request`: the canonical request (the items by occurrence key, the
    claims, the domains and bounds, the options that affect a result, and
    every deterministic budget that can move a result to `incomplete`);
  - `backend`: the provider's `BackendId`, the identity string alone;
  - `manifest`: the provider's complete FR-331 descriptor, `tool` included
    (FR-331 "Backend tool"), so a tool upgrade or an engine release that
    changes only `tool.version` changes the key while `backend` stays the
    same;
  - `target`: for `generate` only, the generation target and its options;
    the member is absent for `prove` and `analyze`.

  No other member enters the key.
- **Outside the key.** Paths, timestamps, wall-clock deadlines and budgets (a
  hybrid solver's `max_solver_time`, FR-193), application
  version strings and the execution backend that ran. A provider's tool
  version is not an application version string: it is `tool.version` in
  the descriptor, inside the key through `manifest`, both for an external
  tool and for the crate that implements a pure in-process engine. A wall-clock budget can
  change a result, but every result it limits is time-limited and never
  stored ("Never stored" below), so the key never has to bind it.
- **Invalidation.** Any change to a key input changes the key, so a changed
  input is a miss and nothing else invalidates an entry. The store is the set
  of entries, each holding its key.
- **Store and read.** An entry holds its key and the
  stored value. A read accepts an entry only when the entry's stored key and
  the requested key are equal componentwise and the value
  parses with the typed reader that reads a fresh result; anything else is a
  miss. A writer writes to a temporary name and renames it into place, and
  two writers of one key write identical bytes. Eviction keeps the store
  under the caller's size limit.
- **Never stored.** A result whose category is internal failure, a cancelled
  or deadline result, a time-limited `incomplete`, any member recording wall
  time, every result a plugin produced (FR-305), and every
  `tool-unavailable` result.
- **Read after the probe.** For an item routed to a backend that fronts an
  external tool, the cache SHALL be read only after that item's FR-290 tool
  probe passes. The probe passes only when the executable about to run
  reports the descriptor's `tool.version` (FR-331 "Backend tool"), so a hit
  is returned only for the tool that would run. A failed probe settles the
  item `unsupported` with `tool-unavailable`, and no entry is read for it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-306-AC-1 | The AOT and JIT backends return the interpreter's outcome, typed cause, location, loss records and `incomplete` charge point for every entry of the conformance corpus and of generated programs, and the outcome names no backend. | Test (TC-226) |
| FR-306-AC-2 | Changing the operation, the `package_id`, any request member that can change a result, the provider's `BackendId`, any member of the provider's descriptor (its manifest) or, for `generate`, the generation target or its options changes the cache key and misses, while changing a path, a timestamp, a wall-clock deadline, the application version string or the execution backend leaves it unchanged. | Test (TC-226) |
| FR-306-AC-3 | An entry whose stored key and requested key are not equal, or whose value fails the typed reader, is a miss; an internal-failure, cancelled, deadline or time-limited result, every plugin result and every `tool-unavailable` result are never stored. | Test (TC-226) |
| FR-306-AC-4 | `--engine` outside `interpreter`, `aot` and `jit` refuses as an unknown argument (exit 20); a named backend the binary does not link refuses before any stage as unsupported (exit 21); an entry a backend cannot translate settles `unsupported` for that entry only, and no backend substitutes another. | Test (TC-226) |
| FR-306-AC-5 | A JIT or AOT internal error is an internal failure (exit 30) and never a verdict, and reaching a preparation limit is a typed limit failure of preparation naming the limit, its value and its setting. | Test (TC-226) |
| FR-306-AC-6 | After a `prove` result of a first-party linked provider fronting an external tool is stored, a read with the same inputs and a descriptor that differs only in `tool.version` computes a different key with the same `backend` member and misses, and never returns the stored result; a read with the unchanged descriptor hits. | Test (TC-226) |
| FR-306-AC-7 | After a `prove` result of a first-party linked provider fronting an external tool is stored, the tool executable on the host is replaced by another version with no edit to any descriptor source: a run in the same invocation probes the new version, settles the item `unsupported` with `tool-unavailable` naming both versions, does not return the stored result and stores nothing; the next invocation builds the descriptor with the new `tool.version`, computes a different key with the same `backend` member, misses and runs the tool. | Test (TC-226) |
| FR-306-AC-8 | After an `analyze` result of a pure in-process engine whose descriptor's `tool` is its implementing crate at one version is stored, a producer build whose engine crate has another version computes a different key with the same `backend` member for the same request, misses and runs the engine; the same build reads the entry it stored back as a hit. | Test (TC-226) |
| FR-306-AC-9 | Withdrawn: it existed only for `tool.build`, the build-input hash of the engine's code, which the descriptor no longer carries; the engine is identified by its crate name and version. | Withdrawn |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.

## References

- Linear STD-141; QSL ADR-029 EB-1 to EB-8, CA-1 to CA-5, RU-5.
