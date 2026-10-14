---
id: SR-9111
title: "spec review of quire-specification PR 253 (protocol-reference production)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cc9316e7b2a02995e9a0011e8248c23c3b0464d7; proposals/quire-v1/shared-grammar.md, spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md, spec/test-cases/TC-943-a-protocol-reference-parses-resolves-and-binds-by-key.md"
review_set: subset
---

## Summary

Ticket: QSL-770. Base spec review of PR 253, which adds `protocol-reference` to
the complete-V1 `primary`, FR-463 and TC-943. Compared against the paired QSL
spec PR (quire-spec-language #840 at 26e53f967945ec66aa288c24fe69ac33b6a43fa4:
FR-308, FR-218, FR-092-AC-16..18, TC-1021), against QSpec FR-322 (carrier text
and FR-322-AC-66), FR-330-AC-4, TC-931 and the positive-all-families fixture,
and against the Contract IR reader on main (e0bee055b6677cc511a303cc380138915ec3905d,
`validate_protocol_references`).

Units examined: the `primary` alternative and the `protocol-reference`
production; the "Protocol reference expression" subsection; FR-463 statement,
Surface syntax, Resolution, Type, Carrier and Refusals paragraphs; FR-463-AC-1
to AC-5; TC-943 steps 1 to 5 and expected results; FR-463's relationships
(US-011, FR-322, FR-330). Context only: FR-322 carrier paragraph, FR-322-AC-66,
FR-330-AC-4, TC-931, the bounded grammar, the facet rule, and the reserved-word
lists.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-463 gives the reference the target protocol's semantic type, and says a consumer that accepts no "protocol-typed operand" refuses it. The only target form specified (the `protocol_clause` node of FR-322's worked fixture, TC-931, and QSL FR-092-AC-18) has the Boolean scalar type as its semantic type. So the reference is Boolean-typed, no "protocol-typed operand" exists, and every Boolean consumer (`and`, `not`, an `if` guard, an invariant block) accepts `protocol Fill` and the unit checks into a package. FR-463 gives the reference no value meaning, so that package holds a Boolean expression whose evaluation nothing defines. State which positions admit a protocol reference (and refuse every other operand position with a named code and cause), or give the reference a type no Boolean consumer accepts. QSL FR-308 and FR-218 carry the same sentence, so fix both. | spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md:56-58; spec/objects/interfaces/FR-322-checked-package-artifact.md:141,167 | wrong-requirement |

## Verdict

One finding (FND-001). Everything else examined is clean:

- Grammar matches QSL FR-308 exactly: `protocol-reference = 'protocol', qualified-name ;`
  as a `primary` alternative, admitted wherever a `primary` is, no package
  selector, alias or key, non-`qualified-name` target is `invalid_syntax`, and
  `protocol` never changes how a `protocol-clause` parses. No difference found.
- No grammar conflict. `protocol` is a common reserved word, so it never starts
  an `ident` or a `qualified-name`. It is a distinct first token among the
  `primary` alternatives (LL(1)). No production places an `expr` directly before
  a declaration without a `;` or `}`, so the expression/declaration boundary
  stays unambiguous. `protocol-step-source` uses `qualified-name`, not `primary`.
  Leaving the bounded `primary` unchanged is correct: the production belongs to
  the complete-V1 protocol facet.
- FR-463-AC-1 to AC-5 each have a TC-943 step. FR-463 and TC-943 are unused on
  origin/main and on every remote branch. US-011 exists and 17 other protocol
  FRs implement it.
- The carrier text agrees with FR-322 (body exactly `{term: "reference", target}`,
  same semantic type as target, sole dependency, one `expression` occurrence) and
  with the IR reader's join (absent target `missing_declaration`/`missing-name`,
  non-protocol or unequal type `ill_typed`/`operator-ineligible`).
- Both copies validate with `quire validate` (quire 0.36.2, engine 0.50.2).

Recorded, not blocking:

- Facet gating: the general rule (an unselected facet's recognized construct
  receives `unsupported_construct`) covers the unselected case. No explicit row
  is needed.
- QSpec does not state QSL's duplicate-reference refusal
  (`unsupported_construct`/`expression-form`). It follows from the carrier key
  and the one-occurrence rule, and IR-785 decides whether repeats become allowed.
  Leaving it out is right.
- `protocol M::Fill` parses, but it can never resolve, because a local protocol
  declares a single `ident`. It always refuses `missing_declaration`.
- QSL FR-308 says QSpec's shared grammar has no `protocol-reference`
  production. That sentence becomes stale when this PR merges.

## Dispositions

Round 1 at agent-ix/quire-specification@2ef84742d1a92d6524f38779874926b15e6fefa1,
paired with quire-spec-language #840 at 05373eedb07d9c1f72b3294f8673d55d9751b874.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2ef84742: FR-463's "Type and value" paragraph removes the "protocol-typed operand" sentence and gives the reference the target protocol clause's value. The reference keeps the target's semantic type, so FR-322-AC-66 and the IR reader's equal-type join are unchanged. QSL FR-308 and FR-218 carry the same rule. No text that was removed survives in either repo. |

Recorded, not blocking: a protocol clause's value is a verdict over a run (its
`over` and activation), so a reference inside a state-point expression takes
that whole-run verdict. shared-grammar.md:237 says common value-expression
productions carry no protocol semantics. `protocol-reference` is a
complete-V1 production, so the two do not formally conflict. The other
recorded difference is wording only: QSL FR-308 says "a pure reference that
denotes" where QSpec says "a pure reference denotes".
