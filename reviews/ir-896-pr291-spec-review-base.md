---
id: SR-13368
title: "Base review of QSpec PR #291 (IR-896): one pairing for unsupported-wire and unsupported-feature"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@866c90d; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, tests/checked_package_v2.rs; ticket IR-896"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-330
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: IR-896. PR: quire-specification#291 at 866c90d, scoped to `git diff refs/remotes/origin/main...866c90d` (merge base 9d7268c = origin/main): FR-322 prose and new FR-322-AC-78, a TC-233 `verifies` edge, and the new test tc_233_unsupported_wire_and_feature_causes_have_one_pairing. Spec-only otherwise; no schema or vector change. Every claim was re-checked against the sources, not against the author or luna.

No contradiction exists. On origin/main 9d7268c, FR-322 (the vocabulary sentence at :605-613) names `unsupported-wire` and `unsupported-feature` nowhere; grep over spec/, proposals/ and tests/ on main finds those causes only in FR-272 :69-70 (`unknown_wire`, `unknown_required_feature`), native-diagnostics :64-65 (same), the schema's `DiagnosticCausePairing` (:180-181, same), FR-144 and TC-030. FR-272's `unknown_contract_version` and `unknown_required_capability` rows (merged #290) say 'None; the code is the whole classification'. FR-322's vocabulary contains neither `unknown_wire` nor `unknown_required_feature`, and FR-330 :186-193 fixes a nested `invalid_body.refusal` to an FR-322 code and 'the FR-272 spelling paired by FR-322', so neither cause can appear in a valid nested refusal. Contract IR's spec has no such pairing either. The only divergence is Contract IR code: agent-ix/quire-contract-ir main 000e461 crates/quire-contract-model/src/checked_package/v2/contract_package.rs:1416-1417 (`Cause::UnsupportedWire => code == Code::UnknownContractVersion`, `Cause::UnsupportedFeature => code == Code::UnknownRequiredCapability`). It is in quire-contract-ir, not quire-contract-codegen. luna's report is not reproduced, and the author's account matches the sources.

The PR text matches #290's no-cause rows: FR-272's `unknown_contract_version` and `unknown_required_capability` rows read 'None'. It names the eight codes correctly, and it locates the divergence at the right file and lines (in quire-contract-ir). Its claim that 'FR-322's rule that a refusal's (code, cause) pair must be admitted already refuses [it] as invalid-value' is only true through FR-330 (FND-002).

Gates run on 866c90d in a scratch clone: `cargo test --test checked_package_v2 --test backend_provider_v1` passes (9 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2) on FR-322 exits 3 with the same 14 lines as origin/main 9d7268c (missing `id`, `semantic.record-invalid`, line-74 EARS warnings, one given-when-then AC warning); the only difference is that warning's line number (1531 -> 1539). TC-233 validates clean on both. `quire matrix --format tsv`: the only change from main is the new row FR-322-AC-78 `tagged` at tests/checked_package_v2.rs:10398; no other criterion changed status.

Mutation runs of the new test (each reverted after): widening the schema's `unsupported-wire` pairing to `unknown_wire|unknown_required_feature` fails it; deleting the `unsupported-feature` pairing fails it; listing `unsupported-wire` in FR-272's `unknown_contract_version` row fails it; removing `unsupported-wire` from FR-272's `unknown_wire` row fails it. Rewriting FR-322's new prose to pair the causes with `unknown_contract_version`/`unknown_required_capability` passes it (the test does not read FR-322; AC-78 does not claim it does). No mutation of any reader can reach it: it calls no reader and reads no vector.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-78 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-reader-code-cause-statement | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233-AC-78-edge | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| FR-322-read-vocabulary | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |
| FR-272-unknown_wire-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-unknown_required_feature-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-unknown_contract_version-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-unknown_required_capability-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-noncanonical_wire-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | context_only |
| native-diagnostics-wire-feature-rows | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| schema-DiagnosticCausePairing | proposals/checked-package-v2/schema.json | examined |
| FR-330-invalid-body-refusal | spec/objects/interfaces/FR-330-contract-package.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The refusal clause of FR-322-AC-78 has no test that can fail. The clause is 'a refusal record pairing either with `unsupported-wire` or `unsupported-feature` is refused `invalid-value` at its cause pointer'. The AC is marked Test (TC-233), and its only test, tc_233_unsupported_wire_and_feature_causes_have_one_pairing, reads the schema and the FR-272 rows only. It calls no reader and reads no published vector; QSpec publishes no adverse vector holding such a refusal record. Mutation confirms this: the schema and FR-272 halves are guarded, and nothing reaches the refusal half. This half is the behaviour IR-896 needs, because Contract IR's validator (v2/contract_package.rs:1416-1417) currently admits exactly these records. So the matrix shows AC-78 `tagged` while a reader that admits them still conforms to every QSpec test. Fix: either drop the refusal clause from AC-78, leaving it to FR-330 and a Contract IR conformance test, or publish two adverse `invalid_body.refusal` records (`unknown_contract_version`/`unsupported-wire` and `unknown_required_capability`/`unsupported-feature`) with their expected refusal, and have the test assert them. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1596 |
| FND-002 | medium | AC-78's refusal clause goes beyond what FR-322's statement says, and it names no code. The statement (:613-620) says only that such a record 'is unpaired', a term FR-322 never defines, and FR-322 states no refusal for it. FR-322's own `read` takes no refusal records as input: the records the AC means are FR-330's nested `invalid_body.refusal` objects. Refusing them comes from FR-330 :186-193 (`cause` uses 'the FR-272 spelling paired by FR-322') and :389-393 (an out-of-domain member is `invalid_package` with cause `invalid-value` at that member's pointer). The AC says 'refused `invalid-value`' without the code `invalid_package` and without naming the reader. Fix: state the rule where the reader is defined (FR-330), or cite it, and spell the outcome in full: the ContractPackage reader refuses `invalid_package`/`invalid-value` at the nested refusal's `cause` pointer. Replace 'unpaired' with that outcome. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1596 |
| FND-003 | medium | The 'Eight of these codes ... carry no cause (FR-272)' list is incomplete, so a reader will infer that the other twelve vocabulary codes take causes. `invalid_source_map` has no FR-272 row, so it has no closed cause set either. `noncanonical_wire` has causes in FR-272, but FR-272 :92 says the checked-package V2 reader's refusal 'carries neither cause nor `document_pointer`'. Contract IR's `causes_pair` also rejects every cause with either code. Either list all the codes the FR-322 reader reports with no cause (ten), or rephrase the sentence so it does not read as the complete set. | spec/objects/interfaces/FR-322-checked-package-artifact.md:613-617 |
| FND-004 | low | TC-233 gains a `verifies` edge to FR-322-AC-78, but its Test Procedure and Expected Results never name AC-78. Its neighbours do: FR-322-AC-47 at :173 and FR-322-AC-77 at :415. The generic sentence at :144-145 ('require each cause in exactly its code's row and the Diagnostic schema pairing') does not cover AC-78's no-cause assertion. Add a step citing FR-322-AC-78. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:14-15 |
| FND-005 | low | 'this is the one pairing (FR-322-AC-78)' repeats the 'only' already in the sentence, in the voice of a reply to a dispute. It adds no behaviour. Drop it, or keep just the AC reference. | spec/objects/interfaces/FR-322-checked-package-artifact.md:617-620 |

## Verdict

Not mergeable as is: one high, two medium, two low. There is no spec contradiction. FR-322, FR-272, native-diagnostics and the schema agree, and the divergence is Contract IR code. AC-78's pairing halves are true and are guarded by a test that fails when the pairing is wrong. The high finding is AC-78's refusal clause: it has no test, and it states its outcome without the code `invalid_package` in an FR that does not read refusal records. Dropping that clause, or moving it to FR-330 in full, clears FND-001 and FND-002.

## New findings (disposition pass 1)

Against head f82969b (fix commit f82969b on top of 866c90d). `cargo test --test checked_package_v2 --test backend_provider_v1` passes (9, 108). `quire validate` on FR-322 and TC-233 gives the same output as origin/main 9d7268c once line numbers are ignored. `quire matrix`: the only change from main is the new row FR-322-AC-78, `tagged` at tests/checked_package_v2.rs:10358 and :10401. Value check, run as mutations on main and on the head: on origin/main, widening or deleting the schema's `unsupported-wire`/`unsupported-feature` pairing, and removing `unsupported-wire` from FR-272's `unknown_wire` row, each pass the whole checked_package_v2 suite. On the head, the first two fail the extended AC-47 test and the third fails the new test. Listing `unsupported-wire` in FR-272's `unknown_contract_version` row fails both the new test and the existing FR-272-AC-22 test, so that half was already guarded. So the PR protects something real but small: the schema pairing of two causes and their FR-272 owner rows, which no test guarded before. It does not touch the Contract IR divergence, which Contract IR's own tests must catch.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The statement's refusal requirement ('the reader SHALL refuse a refusal record that pairs either with a cause') cites FR-322-AC-78, but AC-78 now states only the pairing and the empty FR-272 rows. No AC holds the refusal and no test checks it, so the citation points at an AC that does not contain the requirement it is attached to. Fixing FND-002 by citing FR-330 instead of restating the rule also fixes this; otherwise move the citation onto the pairing clause only. | spec/objects/interfaces/FR-322-checked-package-artifact.md:613-618 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f82969b: The cause `unsupported-wire` is paired only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, in the FR-272 rows and the `Diagnostic` schema alike, and the FR-272 rows of `unknown_contract_version` and `unknown_required_capability` list no cause. |
| FND-002 | still-open | The refusal clause left the AC but moved into the statement (:617-618) as 'the reader SHALL refuse a refusal record that pairs either with a cause'. 'The reader' is FR-322's V2 reader, which reads no refusal records; the nested `invalid_body.refusal` records belong to FR-330's ContractPackage reader (FR-330 :186-193, :389-393). The outcome still has no code and no locus. Replace the clause with a citation of FR-330 that names the ContractPackage reader and its `invalid_package`/`invalid-value` refusal at the nested `cause` pointer, or drop it. |
| FND-003 | fixed | fixed in f82969b: A V2 reader SHALL pair the cause `unsupported-wire` only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, as FR-272 and the `Diagnostic` schema's `DiagnosticCausePairing` do; `unknown_contract_version` and `unknown_required_capability` carry no cause (FR-272), so t |
| FND-004 | fixed | fixed in f82969b: Parse the FR-272 rows of `unknown_wire`, `unknown_required_feature`, `unknown_contract_version` and `unknown_required_capability` and the `Diagnostic` schema's pairing of `unsupported-wire` and `unsupported-feature`, and require FR-322-AC-78's one pairing. |
| FND-005 | fixed | fixed in f82969b: A V2 reader SHALL pair the cause `unsupported-wire` only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, as FR-272 and the `Diagnostic` schema's `DiagnosticCausePairing` do; `unknown_contract_version` and `unknown_required_capability` carry no cause (FR-272), so t |
