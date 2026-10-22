---
id: SR-12821
title: "Integrity review of PR #275: FR-375 history-row literal set"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@5f88120f77768e54c518426c6a2396a3bf006875; PR #275 diff against origin/main fd7a16b: FR-375 surface syntax and history rows; checked against proposals/quire-v1/shared-grammar.md (type-ref, primary, exact-number, float-value, collection-value), FR-378, quire-spec-language FR-138, ADR-020 AX-1 and qsl-semantics LiteralValue"
review_set: subset
---
# Integrity review of PR #275 (STD-168)

## Summary

Ticket: STD-168. This pass checks whether the closed `history-literal` set
agrees with the shared grammar, with FR-378 and with the QSL contracts that
consume it (FR-138, ADR-020 AX-1, QSL-557 parser, QSL-543 checker).

The author's exclusions are mostly sound. QSL's `LiteralValue` has no
collection, record or float variant, and every ADR-020/FR-138/FR-378 history
example is a scalar (`Int[0, 1]`, `Int[0, 4]`). So leaving out
`collection-value`, `record-value` and `float-value` matches the consumer. A
bounded `Set`/`Sequence` or a `Float32` declared type is still admitted by
`type-ref`, but every literal fails against it with `type-mismatch`, so the
behaviour is deterministic. It is only less expressive. The one omission that
breaks a real use is `none`.

## Verdict

One medium finding (`none`) and two lows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `history-literal` leaves out `none`. The shared grammar's complete `primary` has `'none'`, QSL `LiteralValue::None` exists, and `Option<T>` over a bounded `T` is a finite-domain type that `type-ref` admits. Without `none`, an `Option` history field cannot be declared. One example is "the previous write, absent before the first write", which is the register-prev pattern of FR-138 with no fake initial value. Every such row refuses `type-mismatch`. Add `'none'` (it conforms to any `Option<T>`), or state that `Option` history types are excluded and why. | spec/functional/analysis/FR-375-declare-model-refinement.md:69-70; spec/functional/analysis/FR-375-declare-model-refinement.md:79-83; proposals/quire-v1/shared-grammar.md:350 |
| FND-002 | low | "The declared `type-ref` is checked against the bounded-type rule below" points at a rule that does not exist. The History rows paragraph says only "an unbounded type refuses". Either define bounded there (finite value domain, as QSL FR-138 says), or drop the forward reference. | spec/functional/analysis/FR-375-declare-model-refinement.md:83-84 |
| FND-003 | low | `qualified-name` also matches a bare `ident` and any non-enum path, such as `Impl::Counter`. The prose says it "names an enum member" but gives no refusal for a name that resolves to something else or to nothing. Presumably that is `missing_declaration`/`missing-name` or `type-mismatch`; name it. The AC's `Type::member` example may also read as two segments where the shared grammar resolves a case as `Model::Type::Member` (three segments); say which. | spec/functional/analysis/FR-375-declare-model-refinement.md:70; spec/functional/analysis/FR-375-declare-model-refinement.md:82; spec/functional/analysis/FR-375-declare-model-refinement.md:188 |

## Dispositions

Round 1, reviewed at ffabc35ab74cccd2837bfb9fff5b1da310be72cc. The enum
code pairing is sound. A name that does not resolve to a member gives
`missing-name`; a member that resolves but has the wrong type gives
`type-mismatch`. This matches FR-138's resolution-then-conformance order.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
