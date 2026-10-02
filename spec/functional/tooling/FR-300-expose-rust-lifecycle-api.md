---
id: FR-300
title: "Expose a stable typed Rust lifecycle API"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
---

# FR-300: Expose a stable typed Rust lifecycle API

## Description

When a Rust consumer uses the native lifecycle API, the library SHALL expose
the parse, select, check, package, execute, monitor, analyze, lower, generate,
prove, replay, inspect and render operations over typed requests and typed
outcomes, each taking explicit limits and a cancellation handle.

## Inputs

A typed request naming its source or artifact identity, its profile and its
limits; the exact predecessor-stage artifact where the operation has one; and
a cancellation handle the caller owns.

## Outputs

The operation's typed outcome: the stage artifact or per-item terminal
records with their category and diagnostics, or a typed stage failure. No
panic crosses the public boundary.

## Behavior

Each operation consumes the previous stage's typed artifact and preserves
source, definition and capability identity. No operation reparses diagnostic
text, reads ambient configuration, an environment variable, a search path or
a clock, or silently performs a later stage.

## Lifecycle surface

| Operation | Role | Input | Output |
| --- | --- | --- | --- |
| `parse` | Read source into syntax | source bytes, source identity, S1 and S2 limits | the parsed source |
| `select` | Admit the domain packages a unit's `model` declarations select (FR-154) | domain package documents keyed by digest, the unit's model selections | the admitted models |
| `check` | Check and link | the parsed source, the admitted models, the dependency input, the lock evidence | the checked package |
| `package` | Emit the checked package (FR-322) | the checked package | the emitted `quire.checked-package/v2` bytes with their source provision |
| `execute` | Call a checked function or evaluate a checked clause | the checked package, the selected entry, arguments, objects, accounting limits, the selected execution backend (FR-306) | the evaluation or a typed call failure |
| `monitor` | Check a supplied finite or lasso trace against temporal clauses | the checked package, clause selections, a trace | one verdict per clause |
| `analyze` | Decide claims over every behaviour with an in-process engine | the checked package, claim items, the subject binding, engine limits | one FR-331 terminal record per item |
| `lower` | Read the emitted package into target-neutral IR | the emitted package and its expected `package_id` | IR nodes or a typed refusal |
| `generate` | Emit backend artifacts from IR | IR nodes, the routed backend per item, the generation target | FR-331 `artifacts` with source maps; an unsupported item emits nothing |
| `prove` | Negotiate, generate, run, replay and settle | the emitted package from the same run's `check`, claim items, the provider registry, provider limits | one FR-331 terminal record per item |
| `replay` | Re-execute a counterexample | a replay request with its byte provision (FR-323) | the replay result |
| `inspect` | Build a typed view of a package, node, outcome, counterexample, trace or state graph | the subject and a view selector | the view |
| `render` | Turn an outcome or view into text or Graphviz DOT | the outcome or view and the source byte provision | bytes |

Every request carries its source or artifact identity, its exact profile,
its limits and a cancellation handle. Every limit is caller-configurable with
a published default (FR-460), and a reached limit names the limit, its value
and its setting (FR-461). Every outcome carries one typed disposition, its
diagnostics and, on success, the stage artifact. Artifacts are immutable,
thread-safe values, and each operation requires its exact predecessor type,
so a wrong-stage input fails to compile. Every operation is a function of its
request except for cancellation.

Each serialized outcome or view carries a `format` identity that names its
document type, for example `quire-outcome/1`. A reader accepts exactly the
format identity it reads and refuses any other.

## Cancellation

The cancellation handle is cloneable and owned by the caller. An operation
polls it wherever it charges a work meter or a stage limit, so a cancelled
run stops within one charge. A cancelled operation returns the typed failure
`cancelled` with cause `requested` or `deadline` and emits no partial
artifact; its category is incomplete. A deadline is set by the frontend that
owns the timer, never read from a clock by the library. A cancelled item
inside `prove` or `analyze` settles FR-331 `incomplete` with its cause kept.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-300-AC-1 | Every lifecycle operation accepts its typed request with explicit limits and a `Cancel` value, returns its typed outcome, and preserves the source, definition and capability identity of its input. | Test (TC-220) |
| FR-300-AC-2 | Passing an operation the artifact of the wrong stage fails to compile, and a request naming an unknown profile or capability returns a typed failure and no partial artifact. | Test (TC-220) |
| FR-300-AC-3 | The same request through the library and through the CLI gives byte-equal JSON outcome documents. | Test (TC-220) |
| FR-300-AC-4 | Cancelling a running `check`, `execute`, `analyze` or `prove` stops it within one charge; `check` and `execute` return the typed `cancelled` failure with cause `requested` and emit no partial artifact, and each open `analyze` or `prove` item settles `incomplete` with the cancellation cause. | Test (TC-220) |
| FR-300-AC-5 | A serialized outcome carries its `format` identity, and a reader given a document with any other format identity refuses it. | Test (TC-220) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.

## References

- Linear STD-141; QSL ADR-029 LC-1 to LC-3, OP-1.
