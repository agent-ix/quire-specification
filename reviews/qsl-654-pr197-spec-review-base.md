---
id: SR-1311
title: "Spec review of PR #197 (QSL-654): the domains refusal rule for every registration"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@46dec81487e5c9964ad6cb0f61ed3523e3c94730; PR #197 diff against origin/main: FR-290 registration paragraph and FR-290-AC-13, FR-331 manifest domains paragraph and FR-331-AC-22, FR-462 hello paragraph, schema BackendDescriptor domains description, TC-271 NG-20, TC-428 PW-09, TC-431 BP-09; with every other place that names invalid-domains or the hello registration refusal as context"
review_set: subset
---
# Spec review of PR #197 (QSL-654)

## Summary

Ticket: QSL-654. The plan-lead rule has two parts. A `domains` that is
present and malformed (empty, a non-boundable or unknown kind, or a repeated
kind) refuses any registration with `invalid_capability`/`invalid-domains`,
an unbounded-only one included. An absent `domains` refuses only a
registration that advertises a `bounded` pair. FR-290 stays the one owner.

Examined:
- FR-290 registration paragraph, cases 1 and 2 (examined)
- FR-290-AC-13, final two sentences (examined)
- FR-331 manifest `domains` paragraph and FR-331-AC-22 (examined)
- FR-462 hello registration-refusal paragraph (examined)
- schema `BackendDescriptor.domains` description (examined)
- TC-271 NG-20, TC-428 PW-09, TC-431 BP-09 (examined)
- FR-272 and native-diagnostics `invalid_capability` rows (context_only)
- proposals/backend-provider-v1/README.md hello paragraph (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The README's hello sentence still lists only the earlier registration causes. It says a `hello` the manifest stage refuses "only for an absent or unknown kind or mode, or for missing or unknown evidence kinds" is an FR-290 registration refusal. A `hello` refused only for its `domains`, whether absent on a bounded descriptor or present and malformed on any descriptor, is also an FR-290 registration refusal (`invalid-domains`). Read literally, the README makes that hello an FR-462 `malformed-body` failure. FR-462's hello paragraph already includes it. Add `domains` to the README sentence, or point the sentence to FR-290 instead of listing causes. | proposals/backend-provider-v1/README.md:77-79 |

## Verdict

Every normative place now states the plan-lead rule the same way:
- FR-290's paragraph splits it into case 1 (present and malformed refuses any
  registration) and case 2 (absent refuses only with a `bounded` pair).
- FR-290-AC-13 states both cases and admits an unbounded-only registration
  that omits `domains`.
- FR-331 now only points to FR-290, and AC-22 states the schema's
  application of the same rule.
- FR-462's hello paragraph says "whatever modes it advertises".
- The schema description says "on every descriptor".
- NG-20, PW-09 and BP-09 each exercise the malformed cases on both bounded
  and unbounded-only descriptors, and an unbounded-only descriptor with no
  `domains` that is admitted.

No other place restates the old rule. FR-272 and native-diagnostics list
the cause name only. The PR's spec prose contains no SHAs.

Mergeable once FND-001 is fixed. It is a one-line README edit.
