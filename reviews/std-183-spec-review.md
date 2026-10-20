---
id: SR-13501
title: "base review of STD-183 shared-grammar note on an unqualified Clock type-ref (PR #298)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@1272d8527643317180570f0d6117342a5b289e78; proposals/quire-v1/shared-grammar.md"
review_set: subset
---
# SR-13501: base review of STD-183 (PR #298)

## Summary

Ticket: STD-183. Base checklist over
`git diff refs/remotes/origin/main...1272d8527` (1 file, +7). The diff adds a
note after `type-ref =/ 'Clock' ;` (shared grammar line 886). The note says
that an unqualified `Clock` in a type-ref is always the clock type keyword,
whether or not the temporal facet is selected, and that a unit-declared type
named `Clock` is referenced as `M::Clock`. Spec-only diff, so code-review and
gap-analysis do not apply. No requirement statement changed, so no EARS
sub-analysis ran.

Checks made by hand (scratch, uncommitted):

- Grammar. The keyword reading agrees with the base `type-ref` (line 304),
  which has `qualified-name` as an alternative. `Clock` is not in the
  reserved-word lists (lines 55-82), so the note's "keyword of the type
  forms, not a reserved word" agrees with them. The facet clause ("receives
  `unsupported_construct` for the form") agrees with line 944 and with the
  general rule for an unselected facet.
- QSL. FR-393 says an unqualified `Clock` in a type position is always the
  built-in and `M::Clock` is a named type. That matches the note. FR-230's
  new "Timed facet gate" (QSL PR #1088, 9cdbed126) refuses a `Clock` field
  `unsupported_construct`/`declaration-form` at S3 when the unit does not
  select `quire.temporal.complete/v1`. That matches the note's facet clause.
  FR-361 item 8 now points to that gate.
- Engine probe (`qsl_cst::parse`, QSL main 469f8b49c). `record R { x: Clock; }`
  and `record R { x: M::Clock; }` are admissible. `record Clock { a: Integer; }`
  and `type Clock = Integer;` refuse at S1 with `invalid_syntax`/unexpected
  token at the name. A control, `record Clock2 …`, is admissible. This
  refusal is FR-393 Behavior bullet 2 and FR-393-AC-7, implemented by
  qsl-cst's `TypeIdentifier { except: "Clock" }`. Because S1 refuses the
  declaration, S3 never sees a declared `Clock`.
- Gates. `quire validate` on `shared-grammar.md` fails the same way on main
  and at the head (the file has no frontmatter `type:`; it is an untyped
  proposal). `cargo test --test checked_package_v2 --test
  backend_provider_v1` passes (11 + 109). `quire matrix --format tsv` is
  byte-identical to main. Tools: quire 0.36.2 (engine 0.50.2), quoin 0.28.3.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The note is inside the ```` ```ebnf ```` fence (opened at line 818, closed at 908). It sits between `type-ref =/ 'Clock' ;` and `timed-interval = …`. The normative grammar block now holds six lines of prose, which are not EBNF and render as code. No other block in the file mixes prose into a production list. The facet notes for this block live in the prose after the fence (lines 910-944). Move the note there. | proposals/quire-v1/shared-grammar.md:886-894 |
| FND-002 | medium | "A unit-declared type named `Clock` is therefore referenced by a qualified name (`M::Clock`)" does not say which qualifier a unit's own declaration takes. In this grammar, `M::` qualifies a model or domain-package declaration through its alias (lines 1233-1234). A native `record`, `type`, `enum`, `tuple` or `union` is referenced by its bare `ident`, and nothing gives a qualified path to a native declaration of the same unit (lines 966-972). As written, a unit that declares `record Clock` cannot reference it. The note also never says whether such a declaration is admitted. The owner decision is that it is not refused, but the note only implies it. State both: that the declaration is admitted, and how it is referenced (or that only a domain-package `Clock` is reachable, as `M::Clock`). | proposals/quire-v1/shared-grammar.md:891-893 |
| FND-003 | medium | QSL disagrees with the owner decision that the note records. QSL FR-393 ("S1 SHALL refuse a unit declaration that would introduce an unqualified type named `Clock` with a parse diagnostic at the name"), FR-393-AC-7 and qsl-cst's `TypeIdentifier { except: "Clock" }` refuse `record Clock`/`type Clock`/`enum Clock`/`tuple Clock`/`union Clock` at S1. My probe confirms this (`invalid_syntax` at the name). QSL PR #1088, the other half of STD-183, does not change it. After both PRs, QSpec says the declaration is admitted, and QSL refuses it as a reserved word. Record a QSL follow-up to drop the S1 refusal and replace FR-393-AC-7, or record that the owner accepts the divergence. | proposals/quire-v1/shared-grammar.md:888-893 |

## Verdict

Not mergeable as is: three medium findings. The note's main rule is right.
An unqualified `Clock` in a type-ref is the clock type keyword under every
facet selection, a `qualified-name` `M::Clock` is a named type, and an
unselected facet gives `unsupported_construct`. That agrees with the base
type-ref, the reserved-word lists, QSL FR-393 and QSL FR-230's new gate
(PR #1088). FND-001 is placement: move the note out of the EBNF fence.
FND-002 is a wording gap about which qualifier a native declaration takes.
FND-003 is a cross-repo divergence. It can be deferred to a QSL follow-up,
but it has to be recorded, because STD-183 otherwise lands with the two
repos disagreeing.

Examined and clean: the keyword reading, the facet clause, and agreement
with FR-230's "Timed facet gate" and FR-361 item 8.
