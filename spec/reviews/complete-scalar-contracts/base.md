---
id: SR-588
title: "Composite base review of complete scalar contracts"
type: SpecReview
analysis: base
scope: "QSpec #68 complete scalar definitions, I04 nominal identities, requirements and property vectors"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS after remediation for preimplementation use by QSL #118 and #119. Exact
decimal, Unicode text, enum identity, dimension/unit, integer-division, IEEE and
equality meaning is selected by immutable definitions rather than host defaults.
The review does not claim that QSL has implemented the planned TC-185–194
evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5881 | high | Resolved: mutable links and installed Unicode/IEEE behavior were replaced by exact definition artifacts, an immutable rule manifest and byte digests. | value-complete.md; value-complete-rules.json; value-text-unicode-17.md |
| FND-5882 | high | Resolved: nominal enum/dimension/unit identity preimages now have a closed I04 V2 schema, positive hashes, adverse mutations and Rust qualification. | FR-322; checked-package-v2/node-identity-preimage.schema.json; node-identity-vectors.json |
| FND-5883 | medium | Resolved: scalar resource outcomes now use one selected `u64` limit object and named ordered charge points. | value-accounting.md; FR-140–149 |
| FND-5884 | high | Resolved (F1 re-review): the lock separates a byte-verified `qualification_catalog` (root, always-selected leaves, schemas, vectors, conditional Unicode/IEEE, three division alternatives and nine rules) from `package_selection`; Unicode is selected only on `text_bearing`, IEEE only on `ieee_operation` and exactly one division profile only on `integer_div_rem`. Selection vectors and a Rust oracle refuse two division profiles, untriggered IEEE/division and missing Unicode. | complete-value-lock.json; complete-value-selection-vectors.json; value-complete.md; checked_package_v2.rs |
| FND-5885 | high | Resolved: evaluator-created compound-unit values have their own versioned identity domain and never mint I04 source/checked-semantic nodes. | value-compound-unit.md; FR-142; interface-004 |
| FND-5886 | high | Resolved: adding nominal preimages to the closed V1 node would break every frozen V1 reader. V1 is retained byte-for-byte and the enhanced wire, graph and package identity are explicitly versioned as V2 with bidirectional refusal tests. | checked-package-v1; checked-package-v2; FR-322-AC-11; TC-217 |
| FND-5887 | high | Resolved: V2 now has an authoritative package identity-domain member and an explicit V1-to-V2 re-link/re-check contract with typed package/node correspondence; byte relabeling and inferred nominal evidence are forbidden. F2 re-review: the result is the closed `MigrationOutcome` sum `relinked { correspondence }` / `refused { code, subject }` with exactly five codes, byte-only reconstruction distinct from missing input. F3 re-review: a canonical nominal `relinked-rekeyed` vector re-links a frozen-V1-valid package to preimage-derived V2 keys. | FR-201; NFR-074; interface-004; FR-322-AC-12; migration-correspondence.schema.json; migration-correspondence-vectors.json |

## Verification

Native `quoin 0.23.1-87-gb3d02d4 validate --repo . --strict` reports no
findings. The local Rust qualification suite passes all 24 tests, including the
exact definition lock, frozen-V1 byte locks, migration correspondence,
rule-manifest, compound-unit and node-preimage vectors.
Matrix rows remain `planned`.
