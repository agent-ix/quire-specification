---
id: FR-354
title: "Record a proved settlement's trust basis"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-349
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
---

# FR-354: Record a proved settlement's trust basis

## Description

When an SMT backend settles a claim `proved`, the evidence record SHALL carry
an exact trust basis: `certificate-checked` when the cvc5 Alethe certificate
for the exact query passes Carcara, otherwise `solver-asserted`.

## Inputs

A `proved` verdict from the registered SMT backend, the exact query it
settled, and, when produced, the cvc5 Alethe proof certificate for that exact
query.

## Outputs

The `proved` verdict paired with exactly one trust-basis value
(`certificate-checked` or `solver-asserted`) and, for `certificate-checked`,
the Carcara check run identity.

## Behavior

A `proved` settlement is never recorded without its trust basis. When cvc5
produces an Alethe certificate for the exact settled query and Carcara checks
that certificate and passes, the settlement records `certificate-checked` and
retains the Carcara run identity and options pin. When no certificate is
produced for the exact query, or Carcara does not pass it, the settlement
records `solver-asserted`: cvc5's word alone, with no independent proof check.
A `solver-asserted` settlement is not downgraded to `refuted` or
`inconclusive` on that basis alone; it remains `proved` with the weaker,
explicit basis named. Carcara absence is a solver-capability absence, not a
verdict absence: a missing or failing Carcara check never blocks the `proved`
verdict, and never silently upgrades a `solver-asserted` basis to
`certificate-checked`. Z3's independent cross-check (FR-349) is a separate
parity signal from the trust basis and is recorded alongside it, not folded
into it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-354-AC-1 | A `proved` settlement always carries exactly one of `certificate-checked` or `solver-asserted`; neither is inferred from verdict alone. | Test (TC-269) |
| FR-354-AC-2 | `certificate-checked` requires Carcara to have checked the cvc5 Alethe certificate for the exact settled query and passed it; the Carcara run identity is retained. | Test (TC-269) |
| FR-354-AC-3 | A missing certificate or a failing Carcara check records `solver-asserted` and does not change the `proved` verdict. | Test (TC-269) |
| FR-354-AC-4 | `solver-asserted` is never silently upgraded to `certificate-checked`, and a `certificate-checked` basis is never downgraded without a re-run. | Test (TC-269) |
| FR-354-AC-5 | The Z3 cross-check result is recorded alongside the trust basis as an independent parity signal, never merged into the trust-basis value. | Test (TC-269) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  provider contract and evidence-accounting architecture this requirement's
  trust basis is recorded under.
- [FR-349](FR-349-register-smt-backend.md) supplies the registered SMT
  backend and the Z3 cross-check this requirement's trust basis is paired
  with.
- [FR-197](FR-197-replay-backend-counterexamples.md) supplies the parity
  precedent (independent, replayable, never-silently-resolved verdict
  agreement) this requirement follows for the Z3 cross-check.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
