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

A `proved` verdict from the registered SMT backend, the exact query identity
it settled (a digest over the query bytes plus the solver options selected
for that run), and, when produced, the cvc5 Alethe proof certificate for that
exact query identity.

## Outputs

The `proved` verdict paired with exactly one trust-basis value
(`certificate-checked` or `solver-asserted`) and, for `certificate-checked`,
the Carcara check run identity.

## Behavior

A `proved` settlement is never recorded without its trust basis. The query
identity a certificate is checked against is a digest over the exact query
bytes together with the solver options pin selected for that run; a
certificate whose query identity does not match the settled claim's query
identity is a foreign certificate and never yields `certificate-checked`,
regardless of whether Carcara itself passes it. When cvc5 produces an Alethe
certificate for the exact settled query identity and Carcara checks that
certificate and passes, the settlement records `certificate-checked` and
retains the Carcara run identity and options pin. When no certificate is
produced for the exact query identity, Carcara does not pass it, or a
produced certificate's query identity does not match, the settlement records
`solver-asserted`: cvc5's word alone, with no independent proof check. A
`solver-asserted` settlement is not downgraded to `refuted` or `inconclusive`
on that basis alone; it remains `proved` with the weaker, explicit basis
named. A recorded basis changes only through a new Carcara check run that
produces its own new record; no existing record is rewritten in place, and
the record distinguishes a basis that was never checked (no certificate
existed to check) from one that was checked and rejected (a certificate
existed and failed, or matched no query identity). Carcara absence is a
solver-capability absence, not a verdict absence: when Carcara is not
registered, no certificate can be checked for any settlement, the resulting
`solver-asserted` basis records a warning naming the absent
certificate-checking capability, and this never blocks the `proved` verdict
or silently upgrades a `solver-asserted` basis to `certificate-checked`. Z3's
independent cross-check (FR-349) is a separate parity signal from the trust
basis and is recorded alongside it, not folded into it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-354-AC-1 | A `proved` settlement always carries exactly one of `certificate-checked` or `solver-asserted`; neither is inferred from verdict alone. | Test (TC-269) |
| FR-354-AC-2 | `certificate-checked` requires Carcara to have checked the cvc5 Alethe certificate for the exact settled query identity (a digest over the query bytes plus solver options) and passed it; the Carcara run identity is retained. | Test (TC-269) |
| FR-354-AC-3 | A missing certificate or a failing Carcara check records `solver-asserted` and does not change the `proved` verdict. | Test (TC-269) |
| FR-354-AC-4 | A recorded trust basis changes only through a new Carcara check run that produces its own new record, not by rewriting an existing record in place; the new record distinguishes a basis that was never checked (no certificate existed to check) from one that was checked and rejected (a certificate existed and failed, or matched no query identity). | Test (TC-269) |
| FR-354-AC-5 | The Z3 cross-check result is recorded alongside the trust basis as an independent parity signal, never merged into the trust-basis value. | Test (TC-269) |
| FR-354-AC-6 | A certificate whose query identity does not match the settled claim's query identity never yields `certificate-checked`, even when Carcara passes it. | Test (TC-269) |
| FR-354-AC-7 | Carcara's absence from registration records `solver-asserted` with a warning naming the absent certificate-checking capability. | Test (TC-269) |

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
