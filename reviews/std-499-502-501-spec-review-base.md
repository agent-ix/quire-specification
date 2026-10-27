---
id: SR-499001
title: 'Spec review of quire-specification PR #459 (STD-499, STD-502, STD-501): class_catalog key and order, activate association, inherited field order'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@48efe279 (PR #459, spec/std-499-502-501-catalog-and-field-order, base e3d48d76); FR-375 checked class catalog paragraph, FR-375-AC-26, FR-375-AC-27, reader step 2; FR-432 binder record table, effective field order section, FR-432-AC-16..18; FR-322-AC-133; FR-464 step 7; FR-208 record value type bullet; TC-310 CC-12, CC-15..17; TC-233 CCW-08/09; TC-386 FO-01..03; proposals/checked-package-v2/schema.json and node-identity-preimage.schema.json RefinementMemoryClass and declared classes'
review_set: subset
date: '2026-10-27'
relationships:
- target: ix://agent-ix/quire-specification/FR-375
  type: reviews
- target: ix://agent-ix/quire-specification/FR-432
  type: reviews
---

## Summary

Ticket: STD-499, STD-502, STD-501. PR: quire-specification#459 (head 48efe279). Spec-only; no fixture or schema changed (confirmed: only the two schema files mention class_catalog under proposals/, no fixture carries one).

Examined (all `examined` unless noted):
- FR-375 checked class catalog paragraph, reader step 2, AC-23, AC-26, AC-27
- FR-432 binder record table, new effective field order section, AC-14, AC-15, AC-16, AC-17, AC-18
- FR-322-AC-130, AC-132, AC-133; FR-464 step 7 and AC-13 (context_only)
- FR-208 record value type bullet; FR-150 and FR-151 and FR-322 member resolution step 3 (context_only)
- TC-310 CC-12, CC-15, CC-16, CC-17; TC-233 CCW-08, CCW-09; TC-386 FO-01..03
- both V2 schemas: RefinementMemoryClass oneOf, declared class `not ^memory ` pattern, no uniqueItems or order keyword
- criterion ids on origin/main and on every open PR branch (#432, #433, #440, #444) and the local STD-503/504/508 worktrees: FR-375-AC-26/27, FR-432-AC-16..18, FR-322-AC-133, CC-15..17, CCW-08/09, FO-01..03 are unused; no id renumbered; no edit outside the eight listed files. `quire validate` on the eight changed files before and after: identical diagnostics (pre-existing FR-208 missing id and FR-322 warnings), modulo path.

Rulings checked: STD-499 memory key (class, memory_model) with class-then-model byte order is a total order (equal keys are duplicates), keeps CC-12, and one entry per model for two controls follows from "for each of those models selected". STD-502 activate as an ordinary declared class on the clause node with FR-432's event binder rule agrees with the existing event rule and with the schemas (`activate P` does not match the `^memory ` exclusion). STD-501 is sound for acyclic supertypes (cycles are already refused `specialization-cycle` by FR-150 and FR-208).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375 checked class catalog paragraph | spec/functional/analysis/FR-375-declare-model-refinement.md:469 | examined |
| FR-375-AC-26 | spec/functional/analysis/FR-375-declare-model-refinement.md:711 | examined |
| FR-375-AC-27 | spec/functional/analysis/FR-375-declare-model-refinement.md:712 | examined |
| FR-432 binder record and effective field order | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:93 | examined |
| FR-432-AC-16..18 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:278 | examined |
| FR-322-AC-133 | spec/objects/interfaces/FR-322-checked-package-artifact.md:2447 | examined |
| FR-464 step 7 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:481 | examined |
| FR-208 record value type bullet | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:246 | examined |
| TC-310 CC-12, CC-15..17 | spec/test-cases/TC-310-refinement-declaration-admission.md:375 | examined |
| TC-233 CCW-08, CCW-09 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:525 | examined |
| TC-386 FO-01..03 | spec/test-cases/TC-386-explicit-protocol-step-rows.md:50 | examined |
| both V2 schemas | proposals/checked-package-v2/schema.json:181 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-432-AC-16, AC-17 and AC-18 are added after the blank line that ends the criteria table, so they are a headerless second block, not table rows. `quire matrix` lists FR-432-AC-14 and AC-15 but none of AC-16..18: three new criteria are invisible to the matrix, trace and TC-386 binding. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:278 |
| FND-002 | medium | FR-375-AC-27 and TC-310 CC-17 refuse a dropped `activate` entry and an added `origin` entry as `invalid_model_binding`/`malformed-declaration`, but FR-375 already says an omitted or extra catalog class refuses `invalid_semantic_graph` at the first offending entry (or the array for an omission); only a wrong node or binder signature is `invalid_model_binding`/`malformed-declaration`. "protocol step 7" is also ambiguous against the refinement reader steps; it means FR-464 step 7. | spec/functional/analysis/FR-375-declare-model-refinement.md:712 |
| FND-003 | medium | The effective field order is not well defined for a diamond where one branch redefines the shared field (Top{t}; L{t redefines Top.t}; R; Bot(L, R)). Step 1 has each supertype contribute its own effective order, and R's contains Top.t, which Bot's effective view hides (FR-322 step 3); the section never says hiding is applied once at T over the whole walk, nor that Top.t via R and L.t via L are not an ambiguous pair (a `redefines` exists between them). No AC or FO row covers redefinition combined with a diamond or multi-level redefinition. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:118 |
| FND-004 | medium | Same-name unrelated fields "refuse `ambiguous_declaration`/`ambiguous-name` and have no binder record", but in FR-322 step 3 that cause is raised only when a member is looked up by name; the type itself is admitted. The ruling names no locus (which node, which pointer, which step) for a binder record request over such a T, so two readers cannot agree where the refusal lands. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:278 |
| FND-005 | medium | The order rests on the domain-package `supertypes` list and each type's `fields` array preserving source order, and no spec says so: FR-154 and FR-150 are silent, and FR-150 sorts the effective view by identity. The PR asks whether FCD owns the supertypes order; the answer must be a stated intake guarantee, not an assumption. | spec/functional/type-model/FR-154-admit-domain-package-model.md:89 |
| FND-006 | low | CC-12 is retained unchanged and still lists the defaults as `sc`, `tso` and `ra` and says "supply a `memory drain` entry for each", while the only admitted order is `ra`, `sc`, `tso`; a vector author following CC-12 literally supplies a refused catalog. | spec/test-cases/TC-310-refinement-declaration-admission.md:375 |
| FND-007 | low | Wording and placement: the FR-432 sentence inserted before "So a binder of a scalar..." breaks that sentence's referent; FR-208 (foundation) now points at a protocol FR for a rule it does not own, and states the order only for record value types; FR-432 does not state the acyclic precondition (FR-150 and FR-208 refuse `specialization-cycle`) that makes the recursion total. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:102 |

## Dispositions

(none yet)

## Verdict

Request changes. One high (FND-001, a one-line blank-line removal) and three mediums are fixable inside this PR; FND-005 may be a ticketed follow-up if the lead prefers, but a one-sentence FR-154 statement is cheaper. The rulings themselves are sound and cheapest-consistent: memory key and order are a total order, CC-12 is admitted exactly in canonical order and refused when reordered or repeated, `activate` reuses FR-432's event rule, and the diamond and redefine rule is well defined once FND-003 is fixed. IR follow-ups, not defects: IR FieldTables (model_fields.rs) orders by identity and must follow FR-432's effective order; IR FR-061-AC-12 / FR-064 should cite FR-375-AC-26/27. STD-503's PR (schema/FR-341/TC) has no spec text yet; no id collision today, but FR-322-AC-133 and TC-310 CC-15.. are claimed here, so STD-503 must take the next free ids.
