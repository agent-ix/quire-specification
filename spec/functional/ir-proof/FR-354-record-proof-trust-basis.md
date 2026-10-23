---
id: FR-354
title: "Record a proved settlement's certification"
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
  - target: ix://agent-ix/quire-specification/FR-305
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-314
    type: references
---

# FR-354: Record a proved settlement's certification

## Description

When the registered SMT backend settles a claim `proved`, the result SHALL
carry the FR-331 `certification` `certified` when the qualified core's
Alethe checker (QSL FR-314) verifies the cvc5 Alethe certificate against the
queries the core itself encodes for that item, and `uncertified` when the
proof comes with no certificate or the checker cannot verify it. When a
plugin (FR-305) settles a claim `proved`, the result SHALL carry the
certification `trusted`.

## Inputs

A `proved` verdict from the registered SMT backend for a bounded-complete or
inductive item and, when cvc5 produced one, its Alethe certificate: each
refuted query and its proof. For a plugin, a `proved` result from that plugin
and its backend identity.

## Outputs

The `proved` result with exactly one certification (`certified`,
`uncertified` or `trusted`), or, for a certificate the checker shows wrong,
an `inconclusive` result with cause `certificate-rejected{rule, at}` (FR-331),
`rule` the check rule that failed and `at` where it failed.

## Behavior

The core's checker rebuilds each query the certificate claims to refute from
the checked package and the item, and requires the carried query to equal it
byte for byte, so a certificate for any other query is shown wrong. It then
checks each proof step against a named set of checked Alethe rules, reading
nothing from the solver but the certificate.

| Checker outcome | Settlement |
| --- | --- |
| Every carried query equals the core's, and every proof refutes it with checked rules | `proved`, `certified` |
| No certificate came with the proof | `proved`, `uncertified` |
| A step uses a rule outside the checked set and no check fails | `proved`, `uncertified` |
| A carried query differs from the core's, a step does not follow from its premises, a proof ends short of the empty clause, or the certificate's shape does not match the proof basis | `inconclusive`, cause `certificate-rejected{rule, at}` |

The checker is part of the qualified core, so every SMT certificate that
arrives is checked. Z3's independent cross-check (FR-349) is a parity signal
recorded alongside the certification, as the result's FR-331 `cross_check`
member; it never changes the certification.

A plugin's `proved` result cannot be replayed and has no in-core certificate
checker (FR-305 "Trusted proofs"). Its settlement stays `proved`, carries the
certification `trusted` and names the plugin's backend identity. A renderer
shows the certification beside the verdict. `trusted` belongs to plugin
results alone, and a plugin's result carries `trusted` alone.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-354-AC-1 | Every SMT `proved` result carries exactly one of `certified` or `uncertified`, and every plugin `proved` result carries `trusted`; the certification comes from the checker outcome or the producer, never from the verdict alone. | Test (TC-269) |
| FR-354-AC-2 | A certificate whose every query equals the core's encoding and whose every proof refutes its query with checked rules settles `proved`, `certified`. | Test (TC-269) |
| FR-354-AC-3 | A `proved` result with no certificate, and one whose certificate uses a rule outside the checked set and fails no check, settles `proved`, `uncertified`. | Test (TC-269) |
| FR-354-AC-4 | A certificate whose carried query differs from the core's encoding, including one whose proof is internally valid, settles `inconclusive` with cause `certificate-rejected{rule, at}` naming the query; so does one with an invalid step, a proof that ends short of the empty clause, or a certificate whose shape does not match the proof basis, each naming its rule and where it failed. None settles `proved`. | Test (TC-269) |
| FR-354-AC-5 | The Z3 cross-check result is recorded alongside the certification, as the FR-331 `cross_check` member of the `proved` result, as an independent parity signal and never changes it: TB-01's certified proof with an agreeing and with a disagreeing cross-check settles `proved`, `certified` both times, with `cross_check` verdict `proved` and `refuted`. | Test (TC-269) |
| FR-354-AC-6 | A plugin's `proved` result settles `proved` with the certification `trusted` and the plugin's backend identity, the rendered outcome shows `trusted` beside the verdict, and an SMT backend's `proved` result carries `certified` or `uncertified`. | Test (TC-269) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  provider contract and evidence-accounting architecture the certification
  is recorded under.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  owns the `certification` member and its three values.
- [FR-349](FR-349-register-smt-backend.md) supplies the registered SMT
  backend and the Z3 cross-check recorded beside the certification.
- [FR-305](../tooling/FR-305-load-explicit-rust-plugins.md) supplies the
  plugin whose `proved` result carries `trusted`.
- [FR-197](FR-197-replay-backend-counterexamples.md) supplies the parity
  precedent (independent, replayable, never-silently-resolved verdict
  agreement) this requirement follows for the Z3 cross-check.
- QSL FR-314 specifies the core's Alethe checker and its checked rule set;
  QSL FR-315 the one query encoding the backend and the checker share.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
