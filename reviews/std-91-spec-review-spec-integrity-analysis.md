---
id: SR-14610
title: 'Spec integrity analysis of PR #351: FR-201 digest domain for the CG obligation identity (STD-91)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@f26c632; git diff origin/main(97bfd65)...f26c632 (PR #351, branch std-fr322-std-91): FR-201 Values row and FR-201-AC-10, FR-201 Dependencies, AD-016 arrow 5 and seed vector, TC-952 (new), TC-272 precondition; ticket STD-91'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-201
  type: reviews
- target: ix://agent-ix/quire-specification/AD-016
  type: reviews
- target: ix://agent-ix/quire-specification/TC-952
  type: reviews
- target: ix://agent-ix/quire-specification/TC-272
  type: reviews
---

## Summary

Ticket: STD-91. PR: quire-specification#351, head f26c632. The check reads the new FR-201 value against the ticket, QSL ADR-013 O-09 (quire-spec-language origin/main) and CG `KaniObligationIdentity` (quire-contract-codegen origin/main 4fb3402, `src/kani/identity.rs`; the PR body's `src/kani_obligations.rs` no longer exists there). Ids: FR-201-AC-10 and TC-952 are new and unique. Main ends FR-201 at AC-9 (AC-5/6 unused) and holds TC-950 as the highest TC-95x. No other open QSpec PR adds FR-201-AC-10 or TC-952 (#207 edits FR-201-AC-8; #341 holds TC-951). The PR merges cleanly with every other open QSpec PR on FR-201, AD-016 and TC-272. A grep of spec/ and proposals/ (reviews/ excluded) for `TK-07`, `QC-4` and the old OPEN marker finds no stale statement left in QSpec. Gates: `quire validate --scope . spec/**/*.md` reports 134 failed documents on main 97bfd65 and on the PR, but the diagnostic multiset differs by one: FR-201 gains `semantic.invalid-model-cell: value "quire.ir-proof.obligation-identity/v1" is not an Identifier` at line 47 (FND-004 of SR-14610). TC-952 and TC-272 validate with no diagnostic. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 117 and 14.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-201 value quire.ir-proof.obligation-identity/v1 | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 | examined |
| AD-016 arrow 5 Identity retained | spec/assurance/AD-016-semantic-family-extension-path.md:189 | examined |
| AD-016 seed vector obligationIdentitySha256 | spec/assurance/AD-016-semantic-family-extension-path.md:274 | examined |
| FR-201 Dependencies (O-09 / AD-016 bullet) | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 | examined |
| TC-272 precondition | spec/test-cases/TC-272-replay-source-structural-separation.md:25 | examined |
| FR-201 | spec/objects/foundation/FR-201-canonical-identity-domain.md:17 | context_only |
| QSL ADR-013 O-09 | agent-ix/quire-spec-language@origin/main spec/decisions/ADR-013-canonical-type-package-conversion-ownership.md:296-333 | context_only |
| CG KaniObligationIdentity | agent-ix/quire-contract-codegen@4fb3402 src/kani/identity.rs:110-145 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The PR binds `quire.ir-proof.obligation-identity/v1` to two different preimages. AD-016 arrow 5 now says the domain's preimage is "exactly this content", the digest "over every `KaniObligationIdentity` member except `source_span`". On CG main that struct also carries `ceilings`, `oracles`, `module_symbol`, `harness_symbol`, `contract_symbol`, `subject_path`, `results`, `solver`, `unwind` and `options`. The new FR-201 row, following QSL ADR-013 O-09, lists only the subject node id, its occurrence key, the obligation kind and the `arguments`. Two conformers can therefore mint different digests for one obligation under one domain. FR-201-AC-10 cannot tell the readings apart, because no TC-952 vector changes a member outside the four (for example `unwind` or `solver`). One preimage has to be chosen and the other statement aligned to it. | spec/assurance/AD-016-semantic-family-extension-path.md:189 |
| FND-002 | medium | The row's preimage covers "a clause, an operation application or a function" with `arguments`, "each a parameter node id with its declared per-argument domain", and says "CG computes it (`KaniObligationIdentity`)". O-09 has a fifth form, the parity claim (FR-357/FR-358, "O-09 parity preimage"). Its subject is an application node, but its `arguments` are `{operand, position, domain}` with tagged `graph_child`/`inline_literal` operands and tagged `range`/`bounds` domains. QSL itself recomputes and checks that digest, and CG carries scalar obligations in a separate struct. The row either names the parity form and the party that recomputes it, or says that parity identities are outside this domain. | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 |
| FND-003 | medium | The ticket asks for the owning lane's input on the `<owning-area>` segment rather than a unilateral mint, because a domain cannot be renamed once identities exist under it. The PR mints `quire.ir-proof.obligation-identity/v1` as a lead default, and its body asks CG / ADR-013 TK-05 to confirm before merge. `ir-proof` names a QSpec spec folder (FR-196, FR-197). ADR-013 OQ-H places the `ObligationIdentity` type in QSL `qsl-replay`, CG computes it, and QSL recomputes the parity form, so the area segment states an ownership the owners have not agreed. This is merge-blocking until the CG/QSL owner confirms the name (or picks `quire.contract-ir.obligation-identity/v1`, or a `qsl-replay`-area name). | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 |
| FND-004 | low | The touched file adds a diagnostic: `quire validate` reports `semantic.invalid-model-cell: value "quire.ir-proof.obligation-identity/v1" is not an Identifier` at FR-201 line 47. The failed-document count stays 134 only because FR-201 already fails with the same diagnostic on all 20 existing domain rows (the `enumeration` archetype types the Value cell as an Identifier). The PR body discloses this. The gate expects touched files to add none, so the owner either accepts the known archetype gap for this row or fixes it in the archetype or table first. | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 |

## Verdict

Not mergeable as is. Two major findings put contradictory or incomplete normative content into an identity-bearing domain (FND-001, FND-002). One major merge-blocking decision is open: the domain name needs the owning lane's confirmation, as the ticket requires (FND-003). One minor gate miss (FND-004). The `source_span` exclusion that the ticket calls the substantive requirement is stated consistently in FR-201, AD-016, TC-952 and O-09. The AD-016 seed vector, TC-272 and the FR-201 Dependencies bullet are consistent with the new row. The PR body contains no review-finding paraphrase.

## Dispositions

Round 1, at head 16398f2 (fix commits a5a0f13 and 16398f2), checked against QSL ADR-013 O-09 on quire-spec-language origin/main and the STD-91 ticket comments. Labels: major blocks merge, minor does not.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-016 arrow 5 now states the FR-201 preimage and puts every other member outside it. After: "Obligation identity = the digest over exactly the members that the FR-201 `quire.ir-proof.obligation-identity/v1` domain lists: the subject node id, its occurrence key, the obligation kind and `arguments`. The other `KaniObligationIdentity` members (resource ceilings, embedded oracles, module, harness and contract symbols, subject path, results, solver, unwind and options) describe how the harness is generated and run and are outside the preimage, as is `source_span`." FR-201 lists the same outside members, FR-201-AC-10 names them, and TC-952 OI-08/OI-10 vary them. |
| FND-002 | fixed | The FR-201 row names the parity form and says who computes it, in line with O-09. After: "the application node of a parity claim (occurrence role `expression`) ... for a parity claim it holds one entry per operand position, each `{operand, position, domain}`, as the QSL ADR-013 O-09 parity preimage encodes it. A per-step protocol record is the one other form: its object has exactly the members `subject_kind` (the string `protocol-refinement-step`) and `record` (the canonical `StepSimulationRecord`), and no occurrence key. ... CG mints the digest for every form; QSL recomputes the parity form from the same preimage." The encoding detail of the parity occurrence key is a new minor finding (FND-006). |
| FND-003 | fixed | The CG/IR owner confirmed the name on STD-91 (comment 6950fa8c, 2026-10-22, luna:IR:plan): "the CG side takes the lead default, quire.ir-proof.obligation-identity/v1 ... No CG objection and no CG-specific domain alternative." FR-201 states the name as the normative value: "`quire.ir-proof.obligation-identity/v1` | CG obligation identity." The QSL ADR-013 QC-4/TK-05 updates are QSL-side work. The PR body still calls the name an unconfirmed lead default (new minor finding FND-005). |
| FND-004 | still-open | Minor. Unchanged: `quire validate` still reports `semantic.invalid-model-cell: value "quire.ir-proof.obligation-identity/v1" is not an Identifier` at FR-201 line 47. It is the only diagnostic the PR adds (134 failed documents on main bb6d08a and on head). The PR body discloses it. Missing: an owner acceptance of the known `enumeration` archetype gap for this row, or an archetype fix. Does not block merge. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | Minor. The PR body section "Lead default, to be confirmed before merge" still says the CG and QSL owners "must confirm the name (and the preimage above) before this merges" and that "the lead is asking the IR plan lane". The CG/IR owner has since confirmed on STD-91 (comment 6950fa8c). The PR record should say that the name was confirmed, by whom and where, so the merge record does not read as an unconfirmed mint. | PR #351 body |
| FND-006 | medium | Minor. The FR-201 row gives every form, the parity claim included, the occurrence key "(`node_id`, `role`, `ordinal`)" and refers to the O-09 parity preimage only for `arguments`. O-09 encodes the parity preimage with the members `arguments`, `node`, `obligation_kind` and `occurrence_key` = `{ordinal, role}`, with the node id carried once as `node`. A conformer that encodes a three-member occurrence key would mint a parity digest that QSL's recomputation refuses (`ScalarIdentity` content-mismatch). FR-201 gives no member names for any form, so byte-level conformance already comes from O-09. Fix: say the parity form's whole preimage is the O-09 parity preimage, not only its `arguments`. | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 |
| FND-007 | low | Minor. Head 16398f2 does not contain origin/main: it is five commits behind bb6d08a (#338, #348, #339, #340, #343). None of them touches FR-201, AD-016, TC-272 or TC-952, and `git merge-tree` merges cleanly. Bring the branch up to date with main before landing, as the gate requires. | PR #351 head 16398f2 |
