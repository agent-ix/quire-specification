---
id: SR-16204
title: "spec-review of a role on a whole model alias (quire-specification PR 389)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c065714b; spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md; spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md; spec/test-cases/TC-375-protocol-state-key-and-coalescing.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: references
  - target: ix://agent-ix/quire-specification/TC-947
    type: references
  - target: ix://agent-ix/quire-specification/TC-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-466
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
---

# Spec review of a role on a whole model alias (quire-specification PR 389)

## Summary

Ticket: STD-331 (from QSL STD-274, PR #1261). One spec-review pass over
`git diff origin/main...c065714b`. Oracles: QSpec FR-322 ("Model-owned
members" steps 1 to 4, the owner join, FR-322-AC-28, -52, -55), FR-464,
FR-466, FR-361 ("Model subject"), FR-272 (`wrong-role-mapping`), TC-376
(`Fill`), and QSL main FR-218 ("Role on a model alias", FR-218-AC-24) and
FR-383 (the `role` row, FR-383-AC-3 and -20), plus the SR on QSL #1261.

Gates, run at the head: `quire validate --scope . 'spec/**/*.md'` (quire
0.36.2, engine 0.50.2) gives 86 failure lines, the same as main.
`cargo test --test checked_package_v2 --test backend_provider_v1` passes
129 + 23 with 0 failed. origin/main is an ancestor of the head. The PR
changes no JSON fixture or vector, so no QSL conformance run is needed.

Examined, clean:

- The departure from the ticket wording is sound. The ticket's lead default
  says "QSpec adds a model form". The PR puts the form on the role's `model`
  argument (a text, the `model_selections` identity) instead of adding a
  `model` node form. FR-322 step 2 keys every model declaration node by a
  `ModelOwner` whose `node` must name a declaration of the selected document,
  and the owner join (FR-322-AC-52, -55) refuses an owner that names none. A
  selected document declares no node for itself, so a whole-model node form
  would need a new owner kind, preimage and join rule. The text argument
  follows the `replication.population` text precedent of the same body. It
  adds no node, no owner and no dependency edge, so FR-322 node ownership
  is unchanged and needs no edit. The identity text enters the role's node
  key, as the `ModelOwner` identity does, with no version label
  (FR-322-AC-28).
- FR-466. No control rule reads the role's `model` target. An `attempt`
  names a role and an operation anchor, and FR-426 draws the receiver from
  the finite domains, not from the role binding. So a whole-model role
  owning attempts, choices and channels needs no FR-466 change.
- QSL FR-218 and FR-383. QSL main refuses `role w on M` at S3 because QSpec
  had no form for it. This PR supplies one, and the QSL emission half
  (FR-383's `role` row writes the text, FR-218 admits the role) follows
  under STD-274, as the PR body says. FR-383-AC-20's "a model of the wrong
  kind" mutation still applies to a reference `model`.
- FR-465 reader step 1. The text case refuses `missing-selection` at the
  role node, retaining the text, and a non-empty `replication` refuses
  `malformed-declaration`. Any other `model` term is `malformed_wire`. The
  dependencies bullet removes the `model` target for a whole-model role. A
  replicated role keeps a reference, which the binder check in step 3
  needs.
- FR-425 Inputs now ask for a binding only of each static role on a model
  type. The new `wrong-role-mapping` refusals use a cause that FR-272
  already lists.
- Testable ACs. FR-465-AC-9 and TC-947 RL20 to RL24 cover admission, both
  key distinctions and each refusal. FR-425-AC-13 and TC-375 PK-17 cover
  the built subject and its `roles` entry, a supplied binding and an
  unbound role on a model type.
- Ids. FR-465-AC-9, FR-425-AC-13, RL20 to RL24 and PK-17 are not on main
  and are not added by any other open QSpec PR (#390, #267, #207, #59).
- No status, changelog or "previously" wording. The four lead defaults are
  labelled "lead default, not an owner answer" in the PR body.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | [major] The refusal for "a subject whose model subject is over another selection's model" rests on a property that no requirement defines. FR-361 defines a model subject as (checked package, initial states, universes, proof bounds), and FR-425 says a protocol subject is a model subject together with a clause "of its package". A checked package may hold several `model_selections` rows (FR-321, ADR-018 HS-2), and the model subject covers the whole package, so no member of the subject says which selection's "domain model" it is over. Two implementers would decide this refusal differently, and TC-375 PK-17's third case ("over the model subject of a second selection `N`") cannot be built as written. It also leaves QSL #1261 FND-005 (which alias a whole-model role may name) unresolved: a clause with `role w on M` and `role u on N` passes the reader and can then never build a subject. Fix: state the rule in terms the subject has, for example that a whole-model role must name the selection whose model declares the clause's `over` type (a reader refusal in FR-465 step 1, with a TC-947 vector), and drop or restate the FR-425 runtime refusal and PK-17's third case to match. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:73-82, :258; spec/test-cases/TC-375-protocol-state-key-and-coalescing.md:46; spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:52 | missing-requirement |
| FND-002 | low | [minor] FR-465 step 1 orders the two text-model refusals: a text naming no `model_selections` row refuses `missing-selection` before a non-empty `replication` refuses `malformed-declaration`. FR-465-AC-9 and TC-947 RL22 and RL23 test each refusal alone, so a reader that checks `replication` first passes every vector. Fix: add a vector (RL25) with both defects that expects `missing_declaration`/`missing-selection`, and name it in FR-465-AC-9. | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:82-86, :291; spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md:58-59 | correct-requirement-no-evidence |

## Verdict

Not mergeable yet. Putting the form on the role's `model` argument, instead
of adding a `model` node form, is sound against FR-322 and FR-466 node
ownership, and it gives QSL FR-218 and FR-383 something they can emit.
FND-001 must be fixed in this PR: the selection-match refusal cannot be
implemented or tested as written. FND-002 is small and can go in the same
round. The representation, the static-only rule, binding by construction and
the `roles` entry shape are lead defaults, not owner answers, and the PR
body says so.
