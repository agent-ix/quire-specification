---
id: SR-505
title: "Base review of the runtime value path model"
type: SpecReview
analysis: base
scope: "Branch objects/runtime-value-path at a9368c1: the FR-047 amendment, the new FR-207 object artifact, the FR-274 open-gap narrowing, and the FR-200, spec/spec.md and acceptance-matrix registrations"
review_set: subset
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-047, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-207, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-274, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
---
## Summary

The change closes one confirmed specification gap and nothing else. FR-274
recorded that the `runtime object/value path` location kind had no structure,
key, order, discriminator or serialized form stated by any requirement, which
left a state evaluator unable to report an unavailable nested value without
inventing a model. FR-047 is amended to define that model normatively; FR-207
extracts it; FR-274 narrows its open gap to exactly the portion that remains
unresolved.

Unlike the object layer that preceded it, this change is **not** purely
structural, and does not claim to be. The object layer's discipline (FR-200)
forbids an object artifact from supplying a key, order or vocabulary that no
behavioral requirement establishes. The runtime value path could not be typed
under that discipline because nothing established it. The only two ways forward
were to leave it in the gap list or to amend the owning behavioral requirement.
The owner chose the amendment, so the semantic content lives in FR-047 where a
behavioral requirement can carry it, and FR-207 extracts rather than invents.
That division is the reason the change is admissible.

## Diff shape

Six files, 272 insertions against 11 deletions:

| File | +/- | Nature |
| --- | --- | --- |
| `spec/functional/foundation/FR-047-...md` | 50 / 0 | Purely additive: one frontmatter edge, four Behavior paragraphs, two acceptance criteria, one dependency link |
| `spec/objects/foundation/FR-207-...md` | 189 / 0 | New artifact |
| `spec/objects/foundation/FR-274-...md` | 25 / 10 | Values row, key-order bullet, one AC, open gap narrowed |
| `spec/objects/foundation/FR-200-...md` | 2 / 0 | One `contains` edge |
| `spec/spec.md` | 3 / 0 | One `contains` edge, one Requirements row |
| `spec/composed-foundation/tests.md` | 3 / 1 | Two criterion rows, TC-047 criteria list extended |

`spec/functional` carries **zero deleted lines**. No sentence of existing FR-047
meaning was rewritten, reordered or removed; the amendment is appended. The only
`id:` line in the whole diff is the added `id: FR-207`.

## Constraints

| Constraint | Verified by | Result |
| --- | --- | --- |
| No existing FR id changed | `git diff origin/main HEAD \| grep '^[-+]id:'` | Only `+id: FR-207` |
| No consumer repository touched | Changed-path set is six files, all under `spec/` | Holds |
| `proposals/quire-v1/` not rewritten | Not in the changed-path set | Holds |
| Unrelated findings untouched | No span, diagnostic-code, producer-stage, fingerprint, digest-preimage, delivery, attempt, effect or temporal artifact is in the diff | Holds |
| `quire-spec-language` unmodified | Different repository, not touched | Holds |

## The normative model, as landed

A runtime object or value path is a **root** plus an ordered, possibly empty
sequence of **traversal steps**.

- **Root** = (selected observation identity, subject). The observation is part
  of the root rather than a step because FR-042 states that presence facts
  "never cross observation identity" and FR-045-AC-3 states that a guard cannot
  justify "a same-named field on another object or observation".
- **Subject** = exactly one of a declared object identity (FR-009, FR-204) or a
  named parameter or result binding of one declared invocation (FR-046,
  "Parameters come from one declared invocation"). Subjects of different kinds
  are never equal.
- **Step** = exactly one of a field selection (exact declared field spelling), an
  option-value selection (no name), or a member selection (exact declared member
  spelling). A field and a member selection carrying the same name are never
  equal; an option-value selection is never a field of the option it traverses.
- **Equality** is componentwise over root and steps. Rendered path text, display
  name, message wording, locale and reported source span never participate.
- **Comparison** is root first (observation key, then subject kind with declared
  object identity before invocation binding, then subject key), then step by
  step, with a proper prefix before every path that extends it, differing step
  kinds as field before option-value before member, and equal kinds by exact
  name. Collection comparison only; no causal, severity or precedence meaning.
- **Absence**: an unavailable path is explicitly absent; an empty step sequence
  denotes the subject itself and is never an absent path; a name-search match is
  never a path; an absent observation or subject yields no path at all.
- **Serialization** is explicitly out of scope, in FR-047 and again in FR-207.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The change is not purely structural, unlike the object layer before it. FR-047 gains normative content that no prior requirement stated: the path root, the three step kinds, componentwise equality, the comparison rule, absence, and the exclusion of serialization. This is intended and owner-directed, because FR-200 forbids an object artifact from supplying what no behavioral requirement establishes, so the gap could only be closed by amending a behavioral requirement. It is recorded here so no reader mistakes the change for an extraction. | FR-047 Behavior; FR-200 identity discipline; FR-207 |
| FND-002 | low | Two orderings in the change are authored conventions rather than extractions: the subject-kind order (declared object identity before invocation binding) and the step-kind order (field before option-value before member). Neither is derivable from any requirement, and a total order needs a tie-break between kinds. Both are declared in FR-047 and cited rather than asserted by FR-207. | FR-047 Behavior; FR-207 "Key order" |
| FND-003 | low | FR-207 adds two acceptance criteria to FR-047 whose verification class is Test (TC-047), so TC-047 grows from five planned criteria to seven without any test being written. The acceptance matrix records both as Planned. No coverage is claimed. | spec/composed-foundation/tests.md TC-047; FR-047-AC-6; FR-047-AC-7 |

## Verdict

Admissible. The amendment is minimal, additive, grounded in accepted
requirements rather than in the non-adopted catalog, and it states its own two
limits (order totality, wire spelling) instead of overclaiming. Validation is
349/349 documents grammar-clean with zero findings.
