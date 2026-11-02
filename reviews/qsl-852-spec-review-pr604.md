---
id: SR-202611022120
title: 'Spec review of PR #604: FR-322 protocol slot expressions are located under type_declaration site origins (AC-313)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@b306f04; git diff origin/main...HEAD (PR #604, branch peter/qspec-slot-site-origin): FR-322 protocol_locations origin clause and site-name registry, FR-322-AC-313, TC-233, checked-term-data-vectors.json PL-01/PL-02/PL-R2/PL-R3, tests/checked_package_v2.rs; checker source agent-ix/quire-spec-language main qsl-semantics/src/check; ticket QSL-852'
review_set: subset
date: '2026-11-02'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: QSL-852. PR: quire-specification#604, head b306f04. The site-name registry was compared form by form with qsl-semantics protocol_site, protocol_attempt_operation_site and register_protocol_sites. All nineteen forms match exactly: declaration, finish, finish body, compensate, the five compensation stages (registration-capture, trigger-guard, activation-capture, retry, recovery) with a zero-based index, role, role until, channel, channel key, terminal, terminal body, activation guard, capture, capture initializer, run path, run body, related start end, decision start..end and the attempt operation reference. The statement of what protocol_attempt identifies agrees with the checker (Origin::ProtocolAttempt at depth 0 is the attempt root; the operation reference is a separate type_declaration site). The reviewed leaf-rule vectors from #603 (PL-R3-leaf-*) keep their rows, graphs and refusal pointers; only the origin changed. The IR repo tests/conformance_qspec/protocol_locations.rs compares round-tripped locations against the vector rows and asserts no origin kind or name literal, so the consumer claim holds. Open PRs #600 and #602 touch no FR-322 or TC-233 file and neither names AC-313. quire validate on FR-322 and TC-233 gives output identical to main apart from line numbers. cargo test --test checked_package_v2 passes 162 of 162. Mutations: renaming PL-01 or setting PL-R3-leaf-literal back to protocol_attempt turns the TC-233 vector test red; renaming PL-R3-leaf-local to Handshake stays green. That last result is FND-001.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 protocol_locations origin clause and site-name registry | spec/objects/interfaces/FR-322-checked-package-artifact.md:1113 | examined |
| FR-322-AC-313 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4139 | examined |
| FR-322-AC-310..312 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4136 | context_only |
| TC-233 relationship and closing paragraph | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:28 | examined |
| PL-01-root, PL-02-child, PL-03-declaration | proposals/checked-package-v2/checked-term-data-vectors.json:881 | examined |
| PL-R2 and PL-R3 reader refusals | proposals/checked-package-v2/checked-term-data-vectors.json:2505 | examined |
| tc_233_checked_term_data_vectors_cover_the_shared_carrier | tests/checked_package_v2.rs:20297 | examined |
| qsl-semantics protocol_site and register_protocol_sites | agent-ix/quire-spec-language@main qsl-semantics/src/check/lowering/protocol.rs:128; check/mod.rs:7924 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-313 states that each PL-R2 and PL-R3 row names the site, but the test enforces it only for PL-R2 (through an older equality) and not for PL-R3 | tests/checked_package_v2.rs:20297 |
| FND-002 | medium | PL-03-declaration keeps origin name Handshake, which the new prose and the checker both exclude for a protocol site, and the test carries an escape clause for it | proposals/checked-package-v2/checked-term-data-vectors.json:1 |
| FND-003 | low | AC-313 makes several claims in one criterion, and its path clause conflicts with PL-02-child | spec/objects/interfaces/FR-322-checked-package-artifact.md:4139 |

## Verdict

Mergeable once FND-001 and FND-002 are fixed or explicitly dispositioned; no high finding. The registry and the protocol_attempt statement are accurate against the checker.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | AC-316 says a protocol_attempt origin appears in the two rows of PL-R1-duplicate-term, but the test asserts only the vector id; AC-317 bundles the PL-03 origin name with a no-bare-type-name rule | tests/checked_package_v2.rs:20339 |

## Dispositions

Round 1 reviewed head 2ffa802. Mutations rerun: PL-R3-leaf-local origin renamed to Handshake fails at the AC-315 assertion; PL-03-declaration renamed to Handshake fails at the AC-313/AC-317 assertion. cargo test --test checked_package_v2 passes 162 of 162. quire validate on FR-322, TC-233 and this SR file matches main apart from line numbers.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AC-315 asserts every PL-R2 and PL-R3 row origin equals PL-02-child's origin; the PL-R3 mutation now fails. |
| FND-002 | fixed | PL-03-declaration now reads "protocol Handshake" (AC-317); the Handshake escape clause is removed and the test asserts exact names. |
| FND-003 | fixed | AC-313 split into AC-313..317, each one claim; path rule is AC-314 and states root versus child. |
| FND-004 | deferred | Low nit, not merge-blocking; the lead may fold it into the next edit of AC-316/AC-317. |

Note (not a defect): PL-03-declaration keeps child path [1]. The checker records a protocol declaration only at the location root, and region resolution returns no region for a non-root path on a name-only site. The vector is a carrier-shape example of a named type origin with a non-root path, which the schema admits and the IR consumer round-trips without source-region inference, so it is acceptable. A root path would mirror the checker more closely but is not required.
