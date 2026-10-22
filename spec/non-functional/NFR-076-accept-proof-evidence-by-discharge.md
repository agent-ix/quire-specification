---
id: NFR-076
title: "Accept proof evidence only by discharge"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-specification/NFR-075
    type: references
---

# NFR-076: Accept proof evidence only by discharge

## Statement

Every proof gate of a Quire runtime or code generation backend SHALL pass only
when, for each module it claims, it discharges at least one proposition inside
that module, a mutation inside that module turns it red, and every helper
shared by its expectation and the function under proof fails the proof when
mutated. A gate that declares no claimed module fails.

## Scope

- Applies to every proof gate whose result is offered as proof evidence for
  Quire code: the runtime gate over the exact kernel
  (agent-ix/quire-contract-runtime `src/exact/`, and the `quire-exact` gate
  that replaces it), and the generated-harness gate of
  agent-ix/quire-contract-codegen.
- Binds those gates in their own repositories, whether or not an epic or
  qualification gate counts their result.
- Proof evidence from other gates counts toward a qualification claim
  (NFR-075) only when those gates meet this requirement.
- Does not govern the prover's own soundness, the choice of prover, how a
  backend settles a claim or a terminal record, or the `proved` result a
  backend returns (FR-331).

## Rationale

Compiling code under a prover is necessary and not sufficient. A gate can run
green while it proves nothing about the code it claims: the prover compiles
the module and reaches no proposition in it, every check it reports is
unreachable, or the expectation and the function under proof share a helper,
so a mutation of the helper moves both sides and the proposition holds
vacuously. The exact kernel was once compiled under Kani with no proposition
discharged (agent-ix/quire-contract-runtime#53), and a mutated shared `decode`
once left a proof reporting VERIFICATION SUCCESSFUL. A gate that accepts such a
run reports coverage that does not exist.

## Behavior

A proof gate declares the set of modules it claims, in its own repository. A
gate that declares no claimed module discharges nothing and fails.

For each claimed module the gate passes only if all of these hold:

1. **Discharge floor.** The prover transcript shows at least one discharged
   check location inside the module. A check is discharged only when the
   backend reports its status as SUCCESS or proved. UNREACHABLE, FAILURE,
   UNDETERMINED and every other status are not discharged. A backend that
   reports no per-check status counts one check per obligation, and only for
   an obligation it reports proved.
2. **Mutation control.** At least one mutation injected inside the module
   turns the gate red. A mutant that leaves the gate green fails the gate. No
   mutant is exempted by an allow-list.
3. **Independent expectation.** The proof's expectation is derived
   independently of the function under proof. A helper is shared when the
   expectation and the function under proof both reach it, directly or
   transitively, in the built call graph. For each shared helper, a mutation
   of that helper is run and fails the proof. An expectation for an exact
   kernel operation, or for a runtime operation that calls one, comes from the
   QSpec operation vectors or from a checked-in specification model that
   calls no exact kernel operation, and so shares no kernel helper.

A claimed module that the prover compiles and in which it discharges no check
is reported `unreached`, and the gate fails. The gate's report names, for each
claimed module, its status (`discharged` with its discharged check count, or
`unreached`), each mutant run in it and whether that mutant turned the gate
red, and each shared helper and the result of its mutation.

A stubbed executor, a predetermined verdict and a build-only run are never
proof evidence. A gate whose pass rests on any of them fails.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Claimed modules with at least one discharged check | 100% | 100% | Transcript analysis per gate run |
| Claimed modules with at least one mutant that turns the gate red | 100% | 100% | Mutation run per gate run |
| Shared helpers whose mutation fails the proof | 100% | 100% | Call-graph analysis and mutation run |
| Passing gate runs resting on a stubbed executor, a predetermined verdict or a build-only run | 0 | 0 | Inspection of the gate's executor and report |

## Verification

Run each bound gate against the TC-953 controls: clean runs with and without
an independent expectation, a claimed module with no discharged check, a module
with only UNREACHABLE checks, a surviving mutant, a shared helper whose mutation
leaves the proof green, a kernel-operation expectation built from the
function under proof, and a stubbed executor. Confirm the gate passes only the
clean runs and reports each failing control with the module, mutant or helper
that caused it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| NFR-076-AC-1 | A gate that declares no claimed module fails. A gate whose claimed modules each show at least one SUCCESS or proved check location inside that module, with every mutation control and shared-helper mutation failing the proof, passes, and its report gives each module the status `discharged` and its discharged check count, names each mutant run in the module with whether it turned the gate red, and names each shared helper with the result of its mutation. | Test |
| NFR-076-AC-2 | A claimed module that the prover compiles and in which no check is discharged, including one whose only checks report UNREACHABLE or a status other than SUCCESS, is reported `unreached` and the gate fails. On a backend with no per-check status, a module whose obligations are all reported other than proved is `unreached`, and one proved obligation counts as one discharged check. | Test |
| NFR-076-AC-3 | A mutant injected inside a claimed module that leaves the gate green fails the gate, and the report names the module and the mutant. A gate configuration that exempts that mutant still fails. A claimed module with no mutant run fails. | Test |
| NFR-076-AC-4 | When the expectation and the function under proof both reach a helper, a mutation of that helper that leaves the proof green fails the gate, and the report names the helper. An expectation taken from the QSpec operation vectors, which reaches no helper of the function under proof, requires no helper mutation. | Test |
| NFR-076-AC-5 | A gate whose pass rests on a stubbed executor, a predetermined verdict or a build-only run fails, and contributes no proof evidence to a qualification claim (NFR-075). | Test |
| NFR-076-AC-6 | An expectation for an exact kernel operation, or for a runtime operation that calls one, that is derived from anything other than the QSpec operation vectors or a checked-in specification model that calls no exact kernel operation fails the gate, and the report names the operation. | Test |
| NFR-076-AC-7 | The runtime exact-kernel gate and the code generation generated-harness gate each meet NFR-076-AC-1 to AC-6 in their own repository, whether or not an epic or qualification gate counts their result. | Inspection |

## Dependencies

- [NFR-075](NFR-075-require-complete-qualification-coverage.md) owns the
  qualification claims that count proof evidence.
- QSL ADR-011 §2.3 places proof-stage acceptance in the QSL architecture; this
  requirement states the same rule for the backend repositories.
