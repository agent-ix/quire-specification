---
id: SR-202611101500001
title: "Base review of the STD-628 VV-08 reachability ruling"
type: SpecReview
analysis: base
scope: "quire-specification branch std-628-vv08 at a51cd2db08c370caf895cfeb9af483594d74ea9a; diff origin/main..HEAD: FR-484-AC-6, TC-973 VV-08 row and Expected Results"
review_set: subset
review_date: "2026-11-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-484, type: references }
  - { target: ix://agent-ix/quire-specification/TC-973, type: references }
---

## Summary

PASS with low findings. The ruling is correct. QSL FR-495 defines a
package-level `CheckMode::Kernel` that records no nonzero obligation for an
integer `div`, `rem` or `mod` (FR-495 Inputs and Behavior; FR-562-AC-6 uses it
at package level). So `10 mod a` over `a: Int[0, 9]` is admitted under Kernel,
and its zero-modulus refutation reaches the ValueValidity arm from real QSL
source. Linked admission is unchanged: FR-495-AC-3 refuses the same body
`unproved-nonzero`, and the QSL FR-062 sentence on the companion branch keeps
that guarantee for `Linked` only. The route needs no synthetic
`CheckedGraph`/`CheckedPackage`. It weakens no admission, because it uses an
existing mode and leaves the default unchanged. VV-08 is still a refutation
vector, so nothing is cut.

## Review set

`git diff origin/main..HEAD`, 2 files and 3 lines. The ruling was checked
against QSL FR-495, FR-068, FR-062 and FR-562, the QSL checker code, and QSL
open PR #2630. `quire validate` (0.38.3) passes on both files. The diff adds no
new IDs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-6 is the narrowing criterion, and the new sentence attaches VV-08 (a `mod` occurrence with no narrowing) to it, so one AC now covers two behaviours. The same AC says "a host that proves narrowing ranges at admission, as QSL's checker does". That is true only of QSL's `Linked` check: under `Kernel`, FR-562-AC-6 and PR #2630 leave unproved result bounds and ranges to the kernel. Fix: move the VV-08 sentence to AC-4, the undefined-occurrence criterion VV-08 already cites, and say "as QSL's `Linked` check does". | spec/functional/ir-proof/FR-484-check-value-validity-items.md:95 |
| FND-002 | low | A QSpec criterion and TC now name a QSL Rust enum variant (`CheckMode::Kernel`), which couples the language spec to one host's API. Fix: describe it as host-neutral behaviour ("a host check mode that leaves the nonzero obligation to evaluation; QSL: `CheckMode::Kernel`, FR-495") or keep the QSL name only as a parenthetical example. | spec/test-cases/TC-973-value-validity-check-input.md:37; spec/test-cases/TC-973-value-validity-check-input.md:51 |
| FND-003 | low | TC-973 Expected Results says "the unguarded `a / b` of VV-05". QSL has no integer `/`: QSL FR-495-AC-6 refuses it, and the QSL provider test spells the vector's `/` as `div`. Fix: write "the unguarded integer division of VV-05 (QSL `a div b`)". | spec/test-cases/TC-973-value-validity-check-input.md:51 |

## Notes for the coding lanes

The ruling needs QSL code that is not on main yet. Open PR #2630 adds a
`CheckMode` parameter to `PackageDeclarations::check`. The Linked nonzero
obligation for integer `div`/`rem`/`mod` (QSL FR-495-AC-3) is also missing on
main, which is the only reason QSL's `value_validity.rs` passes VV-05 and VV-08
today under a plain check. When that obligation lands, QSL-1200's VV-05 and
VV-08 fixtures must check under Kernel. This is recorded in the companion QSL
review (SR-2026111015000000000001, FND-005).
