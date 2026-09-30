---
id: SR-715
title: "Spec review (failure-domain) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; PR #162 diff against origin/main 0d53cf2: proposals/quire-v1/definitions/native-diagnostics.md new rows and 1-draft.8 paragraph, FR-271 and FR-272 new rows; cross-checked against QSL FR-001 (origin/main), FR-140, FR-148"
review_set: subset
---
# Spec review (failure-domain) — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. This pass looks for unstated failure modes and for identity
confusion in the new cause rows. The single-cause codes are unambiguous. The
three `division_pair_out_of_domain` causes partition the two membership
failures completely. `blank-label` fixes a label order, and `White_Space`
pins what counts as whitespace. Refusals retain no value, which matches
kernel `Refusal`'s "never carry the refused value".

Three gaps remain.

## Verdict

**Changes requested.** One medium finding and two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `invalid_source_identity` does not say which cause wins when a label is blank and the path is also empty. The label order decides between labels, but nothing decides between `blank-label` and `empty-path`, so two conforming producers can report different causes for one input. QSL FR-001 does not settle it either. Fix: state the precedence, for example labels before path. | proposals/quire-v1/definitions/native-diagnostics.md:102; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:84 |
| FND-002 | low | The rows fix the payload keys but not how the values are written. A domain `Decimal[lo, hi; smin, smax]`, `Int[lo, hi]` or `Text[min, max; profile]`, a width `binary32`, and the flag set can each be rendered in more than one form. FR-272 fixes only the order of collections. QSL already renders the `cardinality_out_of_bound` bound as `[min, max]`. Field bytes can therefore differ between producers. | proposals/quire-v1/definitions/native-diagnostics.md:102-112 |
| FND-003 | low | The `ieee_not_exact` `flags` row says "the would-be flag set ... the rounded result would have raised". Strict `exact` selects no rounding direction. FR-148 fixes these as the flags the operation would return under `nearest-even`. The row should say so. | proposals/quire-v1/definitions/native-diagnostics.md:110; spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md:55-57 |

## Dispositions

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-8786dedf

<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-715 date=2026-09-26 -->

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6389506c: both rows now say the label check runs first and the first blank label wins; `empty-path` is reported only when every label is non-blank. No criterion or test pins it (SR-719 FND-002). |
| FND-002 | fixed | 6389506c: native-diagnostics now fixes one ASCII spelling each for domains (`Int[lo, hi]`, `Decimal[lo, hi; smin, smax]`, `Rational[lo, hi; dmin, dmax]`, `Text[min, max; profile]`, FR-038 integers, `, ` inside a group, `; ` between groups, no rounding mode), widths (`binary32`/`binary64`), the flag set (fixed order, `,` with no space) and labels. No criterion or test pins it (SR-719 FND-002). |
| FND-003 | fixed | 6389506c: the `ieee_not_exact` rows in native-diagnostics and FR-272 both define `flags` as the flags the same operation would return under `nearest-even` (FR-148). This matches TC-193 F23. |

+++ [reviewer data]

```yaml
dispositions:
  - fnd: FND-001
    outcome: fixed
    path: proposals/quire-v1/definitions/native-diagnostics.md
    lines: "102"
    after_excerpt: |-
      The label check runs first, over the labels in that order, and the first blank label decides `blank-label`; `empty-path` is reported only when every label is non-blank and the source path is empty.
  - fnd: FND-002
    outcome: fixed
    path: proposals/quire-v1/definitions/native-diagnostics.md
    lines: "226-239"
    after_excerpt: |-
      Each payload value is written as one exact ASCII string:
      - a domain is its type name and bounds, `Int[lo, hi]`,
        `Decimal[lo, hi; smin, smax]`, `Rational[lo, hi; dmin, dmax]` or
        `Text[min, max; profile]`: each bound is written as FR-038 writes an
        integer ..., bounds within a group are separated by `, `, groups by `; `, the
        text profile is its grammar spelling, and no rounding mode is written;
      - an IEEE width is `binary32` or `binary64`;
      - a flag set is its member flag names in the order `invalid`,
        `divide_by_zero`, `overflow`, `underflow`, `inexact`, joined by `,` with no
        space;
  - fnd: FND-003
    outcome: fixed
    path: proposals/quire-v1/definitions/native-diagnostics.md
    lines: "110"
    after_excerpt: |-
      `flags`, the would-be flag set: the flags of `invalid`, `divide_by_zero`, `overflow`, `underflow` and `inexact` the same operation would return under `nearest-even` (FR-148).
```

+++

