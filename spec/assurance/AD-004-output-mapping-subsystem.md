---
id: AD-004
title: "Non-authoritative output-mapping subsystem"
type: ArchitectureDescription
status: accepted
owner: quire-standard
system: native Quire output mapping and loss reporting
relationships:
  - { target: ix://agent-ix/quire-specification/AD-001, type: references }
  - { target: ix://agent-ix/quire-specification/FR-116, type: references }
  - { target: ix://agent-ix/quire-specification/FR-120, type: realizes }
  - { target: ix://agent-ix/quire-specification/FR-121, type: realizes }
---
# Non-authoritative output-mapping subsystem

## System Boundary

FS06 refines AD-001's output branch; it does not replace PR #15's architecture.
S9 produces optional derived OCL, SysML/KerML or FRETish output and a complete
source-bound loss report. It also owns the authoring/tool boundary for the
lossless CST, formatter, language server, CLI, WASM surface, metadata/views,
Rust plugin manifests, AOT/JIT providers and reusable semantic libraries. It
never admits generated target text as source or uses a foreign runtime to decide
native truth.

## Views

The component and flow views below allocate shared orchestration separately from
target-specific correspondence and downstream structural observation.

## Components and ownership

| Component | Owns | Must not own |
| --- | --- | --- |
| Common coordinator | strict request intake, exact profile dispatch, complete ordered record set, resource reservation and generated-package identity | target-specific syntax/semantics or source parsing |
| Target profile mapper | one exact bounded correspondence and deterministic target bytes | shared loss meaning, native semantics or another target |
| Package assembler | immutable bytes/manifest/record binding and raw/JCS digests | target parser verdict or evidence sufficiency |
| Optional structural observer | exact package/tool reference and parse/typecheck observation | mapping classification, package mutation or native verdict |
| Authoring tools | lossless source edits, stable formatting, incremental diagnostics and typed lifecycle APIs | semantic reinterpretation or fabricated spans |
| Tool/provider registry | exact Rust manifest validation, capability negotiation, isolation and provenance | ambient discovery, semantic admission or effect authorization |

QSpec owns the standard contracts. Qcir #55, #56 and #57 own the separate Rust
target mappers behind the common boundary. No target implementation is required
to link or invoke another target's implementation.

## Data and control flow

The coordinator validates the complete native package, requested obligation set,
model/semantic selections, target profile and limits. It then dispatches each
obligation to exactly one selected mapper. The assembler exposes a package only
after every requested obligation has exactly one valid FR-298 record. A failed
whole-request check or exhausted reservation exposes no partial package.

## Invariants

- Native source/profile identity and target profile identity are independent.
- `preserved` is proven per obligation and is never a default.
- Conditions and unrepresented source facts remain machine-readable even when
  plausible target text exists.
- Target bytes and the mapping manifest have separate raw/JCS digest domains.
- Parser/tool observations are causally downstream of the immutable package.
- Observation adequacy and protocol adequacy retain their separate FR-061
  vocabularies; the mapping layer exposes no bare adequacy field.
- New first-party executable paths are Rust 1.98.1 and `publish = false`.
- Plugins are either compile-time Rust trait implementations or isolated Rust
  executables using a versioned wire contract; V1 defines no unstable dynamic
  library ABI.
- AOT/JIT execution is an exact backend provider with cache keys over semantic
  package, target, provider/toolchain and options; cache hits preserve native
  runtime parity and invalidation.

## Decisions

| Choice | Alternative rejected | Consequence |
| --- | --- | --- |
| One common envelope with three target mappers | Three unrelated exporter result models | Identity, loss and package behavior cannot drift between targets. |
| One target per request/package | Multi-target fallback inside one result | Cross-target substitution is impossible and each package has one exact profile. |
| Structural observer after assembly | Foreign parser/tool in the semantic path | Pure Rust generation is reproducible without a foreign runtime. |
| Complete per-obligation records | Best-effort text plus warnings | Unsupported meaning remains machine-readable and cannot disappear. |
| Lossless CST is distinct from checked AST | Reconstruct formatting from semantic nodes | Trivia, invalid-source recovery and exact source edits remain available without changing meaning. |
| Versioned Rust providers, no ambient extension | Load arbitrary callbacks or dynamic-library ABI | Capabilities, effects, rights and provenance are explicit and testable. |

## Failure containment

Unknown versions/features, stale identities, unsupported constructs, ambiguous
bindings, invalid source/result inputs, arithmetic overflow, rights/dependency
failure and resource exhaustion produce typed refusal or incomplete operation
outcomes. No failure path emits a substitute obligation or smaller successful
package.

## Risks

| Risk | Response | Owner |
| --- | --- | --- |
| Plausible text is called equivalent | Preservation requires a complete FR-298 record; parsing is downstream only. | common coordinator / qcir target owner |
| Target profiles diverge in loss meaning | All consume FR-269 and the same FR-298/299 invariants. | QSpec + qcir #52 |
| Model/exporter invents declarations | SysML/OCL mappings require exact producer correspondences and refuse ambiguity. | qcir #55/#56 + model owner |
| Temporal units or end semantics drift | FRETish mappings retain explicit clock/completeness conditions and a narrow selected interval set. | qcir #57 + temporal owner |
| Optional tool dependency becomes mandatory | NFR-061 requires removable, exact-version observers and a pure Rust result. | each implementation owner |

## Handoff

Qcir #55/#56/#57 consume FR-120–125, FR-297–299 and NFR-060/061. Each ticket
runs its own scoped `/specify` and `/spec-review` if implementation reveals a
new interface or correspondence choice, then local Rust review/gap gates.
