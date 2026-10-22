---
id: SR-14745
title: "gap analysis of STD-188 new and changed ACs against TC rows (QSpec PR #350)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@a8be66647bfb9a32fd2b546d467a1cbe54bc121c; spec/functional/analysis/FR-405-declare-probabilistic-models.md, spec/functional/analysis/FR-407-state-probabilistic-claims.md, spec/test-cases/TC-350-probabilistic-declarations.md, spec/test-cases/TC-353-probabilistic-claim-forms-and-routing.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-405
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-407
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-350
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-353
    type: reviews
---
# SR-14745: gap analysis of STD-188 (QSpec PR #350)

## Summary

Ticket: STD-188. Plan completion: not assessed. This PR changes spec only,
and the QSL implementation is listed as remaining work. Mapping of each new
or changed AC to its TC row:

- FR-405-AC-9 maps to TC-350 PD-11.
- FR-407-AC-4 (changed) maps to PF-05 and PF-13.
- FR-407-AC-6 maps to PF-02, whose second vector cannot be built (SR-14743
  FND-001).
- FR-407-AC-10 maps to PF-11, AC-11 to PF-12, AC-12 to PF-13, AC-13 to
  PF-14, AC-14 to PF-15 and AC-15 to PF-16.

Every new or changed AC has a TC row. TC-350 and TC-353 have no Scope line to
update. `cargo test --test checked_package_v2 --test backend_provider_v1`
passes (117 and 14).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | PD-11's admitted half says "`reward duration` on `Server::attempt` and on `Server::request`, both `Quantity<ms>`" but gives no expression for the `Server::request` reward. `Server::request` has no parameter `d`, and TC-350 defines no `Service` model. Give the full second declaration, for example `reward duration on Server::request = 0 ms;`, so the admitted case is one exact input. | spec/test-cases/TC-350-probabilistic-declarations.md:36 |
| FND-002 | low | PF-16's admitted half `under every scheduler fair weak Server::tick` names no claim. `Server::tick` is an operation of `Avail` only, and `NoFault`, which PF-11, PF-12 and PF-15 use, would refuse `missing_declaration`/`missing-name` for it. Name the claim, for example `LongRun` with its scheduler clause replaced. | spec/test-cases/TC-353-probabilistic-claim-forms-and-routing.md:53 |

## Verdict

Every new or changed AC has a TC row. Two vectors leave part of their input
unstated (FND-001, FND-002), which is minor. The PF-02 buildability defect is
recorded in SR-14743.

## Dispositions

Disposition pass 1, against head `85fc4212aca2a5622de140243505f916ab110526`
(fix commit `85fc421`).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | PD-11 now reads "then `reward duration on Server::attempt = d;` followed by `reward duration on Server::request = 0 ms;`, both `Quantity<ms>`", so the admitted case is one exact input. |
| FND-002 | fixed | PF-16 now reads "`LongRun` with its scheduler clause written `under Steady fair weak Server::tick`; the same with `under every scheduler fair weak Server::tick`". `LongRun` is defined over `Avail`, which has `Server::tick`, and FR-407-AC-15 says the same thing. |

Verdict, pass 1: both findings are fixed, and none remain open. The changed
rows still map to FR-405-AC-9, FR-407-AC-6, AC-11 and AC-15. PF-02 now tests
only `foreign-workload`, which matches the fixed AC-6.
