---
id: SR-16255
title: "Spec-review of the hybrid model node fix-forward (quire-specification PR 414)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@59cad4f44e50976bbded2bb934e23a63959481c1 (origin/main...HEAD); spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md; proposals/checked-package-v2/fixtures/positive-all-families.json; proposals/checked-package-v2/dependency-selection-vectors.json; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/TC-966
    type: references
  - target: ix://agent-ix/quire-specification/TC-233
    type: references
---

# Spec-review of the hybrid model node fix-forward (quire-specification PR 414)

## Summary

Ticket: STD-385. This PR fixes forward SR-16260, the post-merge review of
#400. The companion QSL PR is quire-spec-language #1364 (SR-16256).
Reviewed `git diff origin/main...HEAD` at head 59cad4f4 (10 files,
+1101/-95). Main is an ancestor of the head.

Gates run by the reviewer:

- `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2):
  86 failure lines, the baseline count; no new failure in a changed file.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 134
  and 23 passed.
- QSL conformance, `cargo test -p qsl-package --lib checked_v2` on QSL
  #1364 head 419f1ce7 (QSL main 7b2726bf plus spec only) with `QSPEC_DIR`
  at this head: 41 passed, 6 failed. All six fail with `UnsupportedNodeTag`
  at `/semantic_graph/nodes/39/node_tag`, the first `hybrid` node of
  `positive-all-families.json`: `conformance_c14`, `conformance_i2`,
  `conformance_fr340_frame_mutations`, `extension_definition_vectors_xd_01_to_04`,
  `conformance_shipped_projection_cases` and `conformance_dependency_selection_vectors`.
  The PR body reports four. Current QSL main has two more tests that read
  this fixture. QSL's missing `hybrid` tag is the only cause.

Re-derived by hand:

- The keys of every `Tank` node (rational scalar, `x` and `y` parameters,
  literal `1`, transition, mode, model) recompute from their
  `quire.structural-node/v1` preimages.
- The fixture `package_id` and the leaf id `0e1900eb…` recompute. The
  `test/a` id `2f5e94bc…` and the top-level id `6bf5813f…` in
  `dependency-selection-vectors.json` recompute from the package-id
  preimage, with the lock's `dependency_selections` replaced.
- `x' = y` and the reset `x = y` reference `y`'s parameter node. Checks 2
  to 5 admit both, and `y` (classed `hybrid`, rate `1`) passes check 5.
- HY-03: check 1 refuses `modes` listing `Moving` twice at the second
  entry, before any other model check.
- HY-10: with every variable classed `clock`, `v`'s rate `-a` fails check 5
  first at `v`'s class literal. `Timer` passes the clock check and refuses
  at `variables`.
- HY-13: body steps run before operation refusals (FR-322 "Body steps"),
  so a law naming event-position passes FR-370's profile check and fit
  (integer `[0, 10]`, empty fairness) and is refused by hybrid check 6 at
  `/body/operation/laws/0`. Under `timed/v1`, the integer interval is
  refused `operation-member-mismatch` in FR-370's temporal step, which
  comes first.
- HY-16: `Moving`'s one transition, index 0, has target 1 (`Stopped`), so
  the first trace is kept and `Halted` is refused at its `mode`.
- RL-02: `2/2` and `1/1` are refused. Each has a denominator of 1 after
  reduction, so the canonical spelling is `1`.

Prior findings: FND-002, FND-003, FND-004, FND-005, FND-006, FND-007,
FND-009 and FND-010 are fixed. The schema enforces each layout claim in
the new "Reader order" preamble (prefixItems with min/max counts, the
class enum, integer `target`/`variable` literals, at least one mode and one
rate, occurrence roles). FND-001 is fixed in text, but the fixture it adds
breaks FR-481 (FND-001 below). FND-008 is fixed by a profile restriction
that is a scope narrowing (FND-002 below).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [soundness] `Tank`'s invariant and guard are a text literal, so a reader that follows FR-481 refuses the all-families fixture. Mode `Filling`'s `invariant` and its transition's `guard` both reference node `b1a66d7f…`. That node is the extension's `value`/`literal` `"audit"`, typed `scalar_type`/`text`. FR-481 check 3.2 refuses an invariant that is not typed `scalar_type`/`boolean` (`ill_typed`/`non-boolean-root`), and check 3.3 refuses such a guard. FR-481-AC-9, FR-447-AC-6 and FR-322-AC-13 all say the full reader admits this package, and HN-11 says the same. The PR body's lead default 3 says the fixture uses "the existing Boolean literal `true`". That is node `8a2ac579…` (`Example.enabled`, typed `Example.Flag`), which the fixture does not reference. No QSpec test asserts the invariant or guard type, and QSL's reader cannot read the tag yet, so no gate catches this. Fix: point both bindings at `8a2ac579…` (or at a Boolean term over `x`/`y`); rekey the mode, the transition and the model; recompute the fixture `package_id` and the three dependency-selection ids; and assert in `tc_966_the_all_families_fixture_carries_a_hybrid_model_with_bare_variable_terms` that the invariant and guard are Boolean-typed. | proposals/checked-package-v2/fixtures/positive-all-families.json:1795; proposals/checked-package-v2/fixtures/positive-all-families.json:1878; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:219; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:304 | correct-requirement-no-evidence |
| FND-002 | high | [scope-deferral] A hybrid claim is restricted to `quire.temporal.infinite-trace/v1`, which is a permanent refusal that removes a capability. FR-481 "Hybrid claims" and check 6 refuse any other `temporal_profile` law (`invalid_package`/`invalid-value` at `/body/operation/laws/0`), and AC-8 pins this for event-position and `timed/v1`. The companion QSL #1364 adds the matching S3 refusal. So a hybrid claim cannot be written under `quire.temporal.timed/v1`, the dense-time profile with exact rational bounds that fits model time, or under the bounded profiles. The PR lists this as lead default 2. Under the standing rule a lead default does not excuse a narrowing. Fix: admit `timed/v1` hybrid claims in the timed interval form (rational seconds) in FR-481 check 6 and 7, with an admitted vector. Admit or refuse the three bounded profiles only on Peter's explicit answer. Land this together with SR-16256 FND-001. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:154; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:255; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:303 | wrong-requirement |
| FND-003 | low | [ambiguous] The TC-233 edit leaves a broken sentence that contradicts itself. It now reads "Only a node of a kind QSL never writes (FR-093-AC-19), kept so that the all-families fixture carries all 15 node tags (FR-322-AC-13), and the fixture's `hybrid` nodes (FR-481) are hand authored, and it is keyed by…". QSL does write `hybrid` nodes (FR-447 S4), so "only a node of a kind QSL never writes" no longer holds, and the sentence has no main verb for its first subject. Fix: rewrite it as two sentences that name both hand-authored sets and the key rule each one follows. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:226-229 | wrong-requirement |

## Merge order

#414 cannot merge before QSL's reader code admits the `hybrid` tag. QSL's
conformance reads the sibling QSpec checkout (Makefile `QSPEC_DIR`
defaults to `$(SIBLINGS)/quire-specification`). Merging #414 first would
turn six QSL conformance tests red on main. Land FND-001 first, then merge
#414 together with the QSL reader-code PR. With FND-001 open, a reader that
follows FR-481 still refuses the fixture after the code lands, at `Filling`'s
invariant.

## Verdict

Not mergeable. There are two high findings: the all-families fixture
breaks FR-481's own term rules (FND-001), and the profile restriction
narrows hybrid claims (FND-002). There is one low finding (FND-003). Eight
of the ten SR-16260 findings are fixed soundly. Both QSpec gates are green.
QSL conformance fails only because QSL's reader lacks the `hybrid` tag.
