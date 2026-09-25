---
id: SR-634
title: "Pair review of the QSL-255 dependency design with QSL ADR-015"
type: SpecReview
analysis: integrity
scope: "QSpec PR 161 head 8bdf48a4 (git diff origin/main...HEAD at main bfbc1af): FR-307, FR-307-AC-6, FR-322 (library identity, dependency_reference, FR-322-AC-36/37), FR-323 (package sources/dependencies, FR-323-AC-7), native-diagnostics step 7, schema.json and node-identity-preimage.schema.json, proposals/checked-package-v2/README.md, TC-227 L09, TC-233, TC-277 BP-06..BP-10, complete-v1 matrix rows, tests/checked_package_v2.rs; reviewed as one design with QSL PR 447 head 30fa18f3 (ADR-015, FR-099, FR-098-AC-6/7, FR-071-AC-9, FR-027-AC-10)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-307
    type: reviews
---
# SR-634: Pair review of the QSL-255 dependency design with QSL ADR-015

## Summary

Review set: the base checklist, failure-domain and cross-repo integrity,
over the diff only. QSL PR 447 (ADR-015) was reviewed together with this
PR. Its findings are in QSL `spec/reviews/qsl-255-dependency-design/pair-review.md`
(SR-633). This review follows SR-632, the author's self-review, and does
not repeat the findings SR-632 fixed.

What holds up:

- Both v2 schemas carry the same closed
  `{term: "dependency_reference", package: PackageId, node: NodeId}` arm. The
  preimage schema's new `PackageId` def matches `schema.json`'s.
  `cargo test --test checked_package_v2` on this branch passes 43 of 43,
  including `tc_233_a_dependency_reference_enters_the_referencing_node_id`.
  That test shows `package` and `node` each re-key the call's
  application-node id, and that the two changes key apart.
- The node-identity design is sound. The term is spelled identically in the
  body and the preimage, so the dependency's id and the node id enter the
  referencing node's id and nothing else changes. The dependency's
  `package_id` is fixed before the importer's, so the preimage stays
  acyclic.
- QSL ADR-015 agrees with FR-307 on the bare-hex digest, `invalid-digest`,
  library identity as an unstructured string, one supplied library per
  identity, and the `missing_import`/`stale_dependency`/diamond/cycle
  causes. It agrees with FR-322-AC-36/37 on `dependency_reference` never
  appearing in `dependencies`. It agrees with FR-323 and FR-323-AC-7 on the
  request shape, the removed entry, the changed version and swapped or
  unreached entries.
- The style is clean. No roadmap wording and no unsupported alternatives
  appear in the added text.
- Measured: `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/2026-*.md'`
  reports 142 failed documents on both this branch and origin/main
  (bfbc1af). The FR-322 and FR-323 `[value_object]` and `record-invalid`
  errors are pre-existing.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|----|----------|---------|------|--------------|
| FND-001 | high | The FR-322 admission rule for a `dependency_reference` target is not transitive. It admits a function whose parameter and result types "carry no `declaration` (builtin, bounded-domain and anonymous structural types), whose node ids are the same in every package". An anonymous structural type can reach a declared node, and then its id depends on the dependency's owner. Examples are a set or tuple over a declared record, or a compound-unit quantity whose unit terms name declared unit nodes. The importing graph cannot carry it "under the same id" without the declared node, and FR-322 gives no way to hold that node. **Fix:** say "types none of whose reachable type nodes carries a `declaration`" at FR-322:219-222. Add a FR-322-AC-37 mutation and a TC-233 vector: a function over an anonymous type that reaches a declared record refuses `ill_typed`/`operator-ineligible`. The same fix is SR-633 FND-001 in QSL ADR-015 D-5. | FR-322:219-227, FR-322-AC-37 (:629); TC-233 procedure | wrong-requirement |
| FND-002 | high | FR-307 contradicts the new FR-322 rule. FR-307's Behavior says libraries export "types, functions, predicates, protocols, plans and view definitions". TC-227 L01 (edited in this PR) and L04 admit a use `l::R` of an exported `R` "with export identity". Under FR-322 a cross-package reference is only a `dependency_reference`, and one that names anything but a declaration-free function refuses `ill_typed`/`operator-ineligible`. A conforming producer therefore cannot admit L01's first case. QSL ADR-015 D-5 follows FR-322 and would refuse it. **Fix:** in FR-307 "Import binding and export identity" (from :50), state that a use of an imported name is admitted as FR-322's `dependency_reference` rule states. Rewrite L01 and L04 around an exported function `f` over builtin types, used as `l::f(x)`. Add a `l::R` use refusing `ill_typed`/`operator-ineligible` to FR-307-AC-6 and L09. | FR-307:33-38, 50-70; TC-227 L01 (:33), L04 (:36); FR-322:219-227 | wrong-requirement |
| FND-003 | medium | Where a `dependency_reference` may stand is stated three incompatible ways. FR-322:209-210 says "It stands wherever a `reference` term may stand". FR-322:223-224 says "It stands as the callee of a `quire.op.function.call` application". native-diagnostics:162-163 and FR-322:231-232 check one "outside any application", and the family sentence at FR-322:490 gives it a general family. A reader cannot tell whether a `dependency_reference` as a non-callee argument, a binding value or a node body root is admitted. **Fix:** pick one. For the QSL-255 scope: it stands only as the callee argument of `quire.op.function.call`, and any other position refuses `ill_typed`/`operator-ineligible`. Then delete "wherever a `reference` term may stand" and the "outside any application" clause. | FR-322:209-210, 223-224, 230-232, 490; native-diagnostics:158-163 | wrong-requirement |
| FND-004 | medium | FR-323 gives no first-refusal order for the replay dependency checks. It also classes a repeated identity as "not in strictly ascending UTF-8 byte order" (`invalid_package`/`invalid-value`). QSL ADR-015 D-1 refuses a repeated identity `invalid_package`/`conflicting-definition` when it builds the dependency input, so the two repos give different codes for one request. **Fix:** in FR-323:40-48, order the checks as follows. First, the entry list is strictly ascending by identity, repeats included (`invalid-value` at `/package/dependencies`), before any recompile. Then the recompile's own refusals, then the proved `package_id`, then an unreached entry, then an entry `package_id` mismatch, each rule over all entries before the next. QSL adopts the same order (SR-633 FND-004, FND-005). | FR-323:40-48, FR-323-AC-7 (:101); QSL ADR-015:184-207 | missing-requirement |
| FND-005 | low | The step 7 sentence for `dependency_reference` is appended after the `leaves` clause, so "before its `operation-mode-type-mismatch`" can be read as the leaves' check rather than the application's. It does not say whether the term is checked in its left-to-right argument place. FR-322:230-232 says it comes before the application's `operation-mode-type-mismatch` and `operator-ineligible`. **Fix:** move the sentence into the argument clause at native-diagnostics:145-147: "then every argument, left to right: an application under this same order, a `dependency_reference` by `missing-selection`, `missing-name`, `operator-ineligible`". | native-diagnostics:139-163; FR-322:230-232 | wrong-requirement |
| FND-006 | low | FR-323 refuses "a dependency whose recompiled `package_id` differs from its entry's `package_id`". TC-277 exercises that only through BP-07, a source edit, which QSL refuses inside the recompile against the import's recorded digest. Nothing tests an entry whose `package_id` alone is changed. That is the case QSL tests as `ReplayRefusal::DependencyIdentityMismatch` (QSL FR-098-AC-6). **Fix:** add BP-11 (BP-06 with one entry's `package_id` changed and its sources intact) expecting `stale_dependency`/`byte-digest-mismatch` naming that entry's identity, and cite it from FR-323-AC-7. | FR-323:40-43, FR-323-AC-7; TC-277 BP-07 (:44) | correct-requirement-no-evidence |

## Verdict

**Changes required.** FND-001 and FND-002 must be fixed before merge.
FND-001 is shared with QSL PR 447, so land the fix in both PRs together.
FND-003 and FND-004 should be fixed in this PR, because QSL cites these
lines as its authority. FND-005 and FND-006 are small and can go in the
same commit.

## Dispositions

All findings are fixed in the commit that follows this review.

- FND-001: FR-322 admits only signatures none of whose reachable type nodes
  carries a `declaration`; FR-322-AC-37 and TC-233 add `Set<R>` and a tuple
  holding `R`.
- FND-002: FR-307's Behavior and export paragraph state the use rule; TC-227
  L01 to L04 use an exported function `f`; FR-307-AC-6 and L09 add the
  `g::R` refusal.
- FND-003: the term stands only as the `quire.op.function.call` callee; any
  other place refuses `ill_typed`/`operator-ineligible`. The "wherever a
  `reference` may stand" and "outside any application" text is gone, and the
  family sentence gives the callee family `function`.
- FND-004: FR-323 lists a five-rule first-refusal order, each rule over all
  entries before the next, with a repeated identity refused by rule 1.
- FND-005: the step 7 sentence is in the argument clause.
- FND-006: TC-277 BP-11 is added and FR-323-AC-7 cites it.
