---
id: FR-306
title: "Execute through equivalent backends"
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

# FR-306: Execute through equivalent backends

## Description

When a checked entry is executed through the interpreter, an ahead-of-time
(AOT) backend or a just-in-time (JIT) backend, the backend SHALL return the
interpreter's exact outcome.

The producer SHALL recompute every `prove`, `analyze` and `generate` result on
each invocation; no result is cached or reused between invocations.

## Inputs

A checked package, the selected entries, arguments, objects and accounting
limits, the selected execution backend and its preparation limits.

## Outputs

The interpreter's exact evaluation outcome, or a typed preparation failure.

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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-306-AC-1 | The AOT and JIT backends return the interpreter's outcome, typed cause, location, loss records and `incomplete` charge point for every entry of the conformance corpus and of generated programs, and the outcome names no backend. | Test (TC-226) |
| FR-306-AC-2 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |
| FR-306-AC-3 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |
| FR-306-AC-4 | `--engine` outside `interpreter`, `aot` and `jit` refuses as an unknown argument (exit 20); a named backend the binary does not link refuses before any stage as unsupported (exit 21); an entry a backend cannot translate settles `unsupported` for that entry only, and no backend substitutes another. | Test (TC-226) |
| FR-306-AC-5 | A JIT or AOT internal error is an internal failure (exit 30) and never a verdict, and reaching a preparation limit is a typed limit failure of preparation naming the limit, its value and its setting. | Test (TC-226) |
| FR-306-AC-6 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |
| FR-306-AC-7 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |
| FR-306-AC-8 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |
| FR-306-AC-9 | Withdrawn: it existed only for the result cache, which is removed; results are recomputed on every invocation. | Withdrawn |

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
