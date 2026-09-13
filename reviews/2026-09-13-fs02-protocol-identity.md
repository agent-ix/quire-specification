---
id: SR-530
title: "Gap analysis — FS02 protocol identity rulings"
type: SpecReview
analysis: gap-analysis
scope: "quire-specification issue #28; TM-003 and TM-004 changed rows; no executable implementation"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-specification/TM-003", type: references }
  - { target: "ix://agent-ix/quire-specification/TM-004", type: references }
---
# Gap analysis — FS02 protocol identity rulings

## Summary

Issue #28's specification scope is complete and internally traced. The packet
defines exact static and runtime keys without claiming that downstream protocol
or observation consumers already implement them. All affected evidence remains
honestly planned.

## Verdict

**PASS** — every issue question has one owning requirement/object rule, all 50
changed acceptance criteria map to a concrete planned case and matrix row, and
the change introduces no untraced code or unsupported semantic approximation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved at PR time: 21 changed object-level criteria described planned mutations but lacked individual TM-004 rows and test-case relationship edges. The corrected matrix now traces all 50 affected criteria, with every row still `Planned`. | TM-004; TC-050; TC-051; TC-057; TC-058; TC-171 |
| FND-002 | low | No remaining gap exists in issue #28's specification-authoring scope. Executable FR-287/288 adoption remains explicit downstream work, not completion evidence for this PR. | FR-287; FR-288; SR-528 |

## Ticket completion

| Live issue #28 item | Closing evidence |
| --- | --- |
| Effect duplicate distinguishability | FR-051/FR-226 require one producer key per effect, keep the establishing attempt as a relationship, treat identical-key replay as one effect and retain a second valid key at a singleton occurrence as the duplicate violation. |
| Delivery identity and order | FR-051/FR-224 select an authority-qualified `delivery` key, keep receipt distinct and introduce no semantic total delivery order. |
| Forward/compensation attempt identity | FR-051/FR-058/FR-225/FR-235 select distinct authority-qualified attempt keys; retry is a relationship, never derived from a maximum, key, timestamp or arrival position. |
| Commit boundary | FR-057/FR-237 separate the static boundary, concrete occurrence and concrete observation keys and represent `never` without an empty/default identity. |
| Recovery relation | FR-057/FR-238 key the relation by FR-288 plus declaration-local compensation and typed recovery-expression handles; operands and dependency facts do not become guessed key members. |
| Static role/channel/definition | FR-050/FR-211/FR-215/FR-288 supply the exact linked-definition qualifier and admitted local handles; an unqualified index, name or span refuses. |
| Workflow/role-instance/message and occurrence | FR-050/FR-287 supply authority-qualified opaque runtime keys; FR-049/FR-221 supply the complete occurrence dimensions and explicit absent/surplus/out-of-range refusals. |

Issue #31 retains only the FIFO key-scope, graph identity, admitted-observation,
endpoint/outcome, activation/completeness, registration, retry-counting,
forbidding-commit and other non-duplicate decisions. This review does not mark
those complete.

## Coverage

- Ticket decision groups specified: 7 / 7.
- Affected acceptance criteria with matrix rows: 50 / 50; missing: 0.
- Planned cases: TC-035, TC-050, TC-051, TC-057, TC-058 and TC-171.
- Executable Rust tracking tags added: 0. This is a specification-only ticket;
  all affected rows remain `Planned`, so no downstream implementation coverage
  is implied.
- Untraced behaviors or source stubs introduced: 0 / 0. The diff contains no
  `.rs`, Cargo, workflow or `resources/native-v1/` change.
- Semantic review: SR-527 through SR-529 record base, implemented-dependency and
  EARS checks. Undefined global definition/predicate identities found during
  review were replaced by exact admitted records and local handles.

## Rust review and local gates

`/rust-review` is not applicable because PR #41 changes no Rust, Cargo or
workflow file. No hosted CI was dispatched.

The three self spec-review artifacts validate 3/3. The scoped matrices and six
planned case artifacts validate 8/8. Full repository validation reports every
357 specification document grammar-clean; its only structural failure is the
unchanged `Status` versus `Coverage Status` header in
`spec/native-source/tests.md`, outside issue #28. `git diff --check` passes.
