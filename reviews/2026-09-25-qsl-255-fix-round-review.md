---
id: SR-636
title: "Spec review — QSL-255 SR-634 fix round"
type: SpecReview
analysis: failure-domain
scope: "git diff 88b67e3..e60521d on qsl-255-dep-design: FR-307 Behavior, export paragraph and FR-307-AC-6; FR-322 dependency_reference, family sentence and FR-322-AC-37; FR-323 package dependencies order and FR-323-AC-7; native-diagnostics step 7; TC-227 L01-L04, L09; TC-233 procedure and expected results; TC-277 BP-07, BP-08, BP-11. Checked against QSL ADR-015 D-4, D-5, FR-098 and FR-099 in qsl-dep-design at 620ed0b4."
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-307
    type: reviews
---
# SR-636: Spec review of the QSL-255 SR-634 fix round

## Summary

Review set: the base checklist and failure-domain, over the fix commit
e60521d only. The commit applies SR-634's six findings.

SR-634 findings, re-measured:

- FND-001 (transitive package-independence): fixed. FR-322 now refuses a
  signature any of whose reachable type nodes carries a `declaration`.
  FR-322-AC-37 and TC-233 add `Set<R>` and a tuple holding `R`. The list of
  edges the rule follows is not defined anywhere (FND-002 below).
- FND-002 (FR-307 use rule): fixed. FR-307's Behavior and export paragraph
  state the callee-only use rule. TC-227 L01 to L04 use an exported function
  `f`. FR-307-AC-6 and L09 add the `g::R` refusal. FR-307-AC-6 has a
  punctuation slip (FND-004 below).
- FND-003 (callee-only placement): fixed in FR-322's statements, in the
  family sentence ("a `dependency_reference` callee has family `function`")
  and in native-diagnostics. But the fix deleted the only placement rule for
  a term outside an application argument, which leaves a gap (FND-001
  below).
- FND-004 (first-refusal order): fixed. FR-323 lists five rules, applies
  each rule over all entries before the next, and refuses a repeated
  identity under rule 1. Rules 1 and 3 to 5 match QSL D-4 rules 2 and 5 to 7,
  with the same codes. D-4 rule 3 (dependency input) has no FR-323
  counterpart (FND-003 below).
- FND-005 (step 7 wording): fixed. The sentence is now inside the argument
  clause, in left-to-right argument order.
- FND-006 (TC-277 BP-11): fixed. BP-11 changes only the entry's
  `package_id`, and FR-323 rule 5 is the first rule that fires for it. The
  entry's sources are intact, so rules 2 to 4 pass. FR-323-AC-7 cites it.

Every code the diff uses exists in native-diagnostics.md: `missing_import`,
`missing_declaration`, `ill_typed`, `invalid_package`, `stale_dependency`,
`missing-selection`, `missing-name`, `operator-ineligible`, `invalid-value`,
`byte-digest-mismatch`, `revision-mismatch` and `conflicting-definition`.
TC-233's "five times" matches the five `ill_typed` mutations in
FR-322-AC-37. The complete-v1 matrix rows for FR-307-AC-6, FR-322-AC-37 and
FR-323-AC-7 still name the right test cases.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | A `dependency_reference` outside an application argument has no place in the first-refusal order. The V2 `SemanticTerm` union admits the term in every term position: node body root, `binding` value and `aggregate` member. FR-322:232 refuses those positions `ill_typed`/`operator-ineligible`. But the fix removed the "outside an application, at its node" clause. FR-322:233-236 and native-diagnostics step 7 now place the term only "among its application's arguments", and step 7's argument clause covers only a direct argument, not a binder value. Two readers can therefore report different first refusals for a package whose body root is a `dependency_reference`. **Fix:** in FR-322:233-236 and native-diagnostics step 7, check a `dependency_reference` at its place in the node's pre-order term walk wherever it stands (application argument, binder value, aggregate member or body root), by `missing-selection`, `missing-name`, `operator-ineligible`. Add a TC-233 / FR-322-AC-37 mutation with the term as a node's body root, refusing `ill_typed`/`operator-ineligible`. | FR-322:209-210, 232-236; native-diagnostics:139-150; schema.json SemanticTerm; FR-322-AC-37 |
| FND-002 | medium | The reachability rule follows an open list of edges, "element, member, component, argument or unit-term references" (FR-322:222-223), and FR-322 defines no edge under those names. It is unclear whether an `option` inner type, an `alias` target, a `reference` target, a `collection_bounds` base or a `model_population` model counts. So `Option<R>`, or a bounded domain over a declared type, could be read as admitted. QSL ADR-015 D-5 copies the same list. **Fix:** in FR-322:221-224 and QSL D-5, define the rule over the graph's own edges: "no node in the transitive closure of the signature type nodes' `dependencies` carries a `declaration`". | FR-322:219-236; QSL ADR-015 D-5 |
| FND-003 | medium | The refusal code sets of FR-323 and QSL D-4 differ. D-4 rule 3 and QSL FR-098 build the dependency input before recompiling and refuse `invalid_package`/`conflicting-definition` when two entries' sources share one authority and identity. D-4 says it uses "the refusal codes QSpec FR-323 gives". FR-323's rule 2 names only the recompile's refusals, and no FR-323 rule yields `conflicting-definition`. **Fix:** in FR-323 rule 2 (FR-323:177-182), first state that building the dependency input refuses an entry whose source has the authority and identity of the proved package's source or another entry's source, as `invalid_package`/`conflicting-definition` naming both. Add a TC-277 BP-12 vector and cite it from FR-323-AC-7. D-4 rule 1 (`NotASource`/`SourceCount`) also has no FR-323 code. D-4 should call that rule a QSL precondition rather than an FR-323 code. | FR-323:170-189, FR-323-AC-7; QSL ADR-015 D-4 rules 1, 3; QSL FR-098:73-77 |
| FND-004 | low | FR-307-AC-6 joins its new clause with a stray ".;" ("`byte-digest-mismatch`.; with `test/geometry` exporting …") and ends without a period. **Fix:** replace ".; with" with "; and with", and end the row's criterion with ".". | FR-307-AC-6 |

## Verdict

**Changes required.** SR-634's six findings are fixed as its dispositions
say. The fix leaves three medium defects. FND-001 is a hole the FND-003 fix
opened. FND-002 is a precision gap in the FND-001 rule. FND-003 is a
code-set mismatch with QSL D-4. FND-004 is a punctuation fix. FND-002 and
FND-003 touch QSL ADR-015, so land them in both PRs together.

## Dispositions

All findings are fixed in the commit that follows this review.

- FND-001: FR-322 and native-diagnostics step 7 check a
  `dependency_reference` at its place in the node's pre-order term walk,
  wherever it stands; FR-322-AC-37 and TC-233 add a body-root mutation.
- FND-002: FR-322 and QSL ADR-015 D-5 use the transitive closure of the
  signature type nodes' `dependencies`.
- FND-003: FR-323 rule 2 opens with the dependency-input refusal
  (`invalid_package`/`conflicting-definition`), with TC-277 BP-12 cited
  from FR-323-AC-7; QSL D-4 calls its source-count rule a QSL precondition.
- FND-004: FR-307-AC-6's punctuation is fixed.
- QSL SR-635 FND-003, carried here for alignment: FR-322 also refuses a
  signature reaching a `ModelOwner` node, such as `Reference<M::T>`, with an
  FR-322-AC-37 case and a TC-233 mutation.
