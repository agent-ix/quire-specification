---
id: SR-202611141351001
title: "Base review of the STD-663 reaches_field population link"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #652 at 1a2b4659, diff origin/main(f268d62f)...HEAD: spec/objects/interfaces/FR-322-checked-package-artifact.md (Population link of a reaches, FR-322-AC-398)"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-233, type: references }
---

## Summary

Ticket: STD-663. Base checklist over the FR-322 amendment that admits `population` on the `field`
member of `quire.op.model.reaches_field`, for QSL FR-380 "Field-edge reachability"
(quire-spec-language origin/main). The text says what FR-380 needs. The population is over the
field's owner `T` (steps 2 and 3), not the member's `declaration` `D`. The same `dependencies` rule
and the same refusals at `member.population` apply. The three field forms are covered, and
optionality and `[l, u]` are not bounds. The PR changes only the spec text. The normative closed
schema and the repository's reader oracle both still refuse what the new text admits. FR-322-AC-398
has no test, no vectors and no TC-233 entry.

## Verdict

FAIL on FND-001 and FND-002.

Examined and clean:
- AC id: FR-322-AC-398 is unused elsewhere (the highest id on main is 397). It is a row of the
  acceptance-criteria table, and `quire matrix` mints it.
- `quire validate --scope . spec/objects/interfaces/FR-322-checked-package-artifact.md` (quire
  0.45.0) reports only the pre-existing line-104 EARS warnings and no new diagnostic.
- QSL FR-380-AC-8 to AC-11 and AC-15: `Reference<T>`, `Option<Reference<T>>` and
  `Sequence<Reference<T>>[l, u]` are named. A node over `Order` with `D` = `Rush` is admitted, and
  one over `D` is refused. The key is part of the structural path, and an absent key keeps the
  pre-existing member and node id (QSL FR-380-AC-13).
- QSL FR-380-AC-12: least-ordinal `Type` occurrence selection is the unchanged general rule. That
  AC also reads through IR-1533, the IR reader, as FR-380's own Dependencies state. Merging this
  PR alone does not un-ignore it.
- No hash, digest, pin or tracking record anywhere in the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322's Description makes the "normative closed schema and independent vectors" the public boundary. The published `schema.json` `OperationMember` `field` branch is `{kind, declaration, name, read_form}` with `additionalProperties: false`. A `reaches_field` field member carrying `population` therefore fails the V2 schema (`malformed_wire`), and FR-322-AC-398's "validates against both V2 schemas" is false at this head. The new rule that the schema refuses `population` on a field projection's `field` member needs an identity-conditional branch, which the schema also lacks. Precedent: #268 (FR-322-AC-73) and #651 (FR-322-AC-397) each landed the schema change in the same PR as the spec text. | spec/objects/interfaces/FR-322-checked-package-artifact.md:2670-2681,4560; proposals/checked-package-v2/schema.json:4679-4707 |
| FND-002 | high | FR-322-AC-398 is `untagged` in `quire matrix`; FR-322-AC-73, AC-74 and AC-397 are each tagged. The repository's reader oracle `population_link_holds` admits `population` only where `member["kind"] == "relationship_end"`, so it refuses the AC's positive cases. `adverse.json` has no field-edge population case, although the STD-663 ask lists them. TC-233 names no case for FR-322-AC-398. Fix in this PR: extend the oracle, add the cases as their own adverse array or a new base so the existing 7-case `reaches_population_cases` test is untouched, tag a test FR-322-AC-398, and list the cases in TC-233. | tests/checked_package_v2.rs:1616-1647,10378-10387; proposals/checked-package-v2/fixtures/adverse.json; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:342-348 |
