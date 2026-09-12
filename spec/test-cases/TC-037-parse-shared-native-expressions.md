---
id: TC-037
title: "Parse shared expressions with explicit precedence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-037
    type: verifies
---
## Description

Planned controls for [FR-037](../functional/foundation/FR-037-parse-shared-native-expressions.md).
These expected cases have not been implemented or executed.

## Test Procedure

Parse symbolic expressions a implies b implies c, a - b - c, a + b * c and not (a = b), retaining explicit groups. Exercise nested conditionals, let bindings, zero-argument predicate calls and a quantified field predicate. Supply a < b < c, mismatched braces, and trailing tokens. Offer a recognized division expression under the candidate restricted arithmetic profile.

Use a model with an admitted bounded sequence member and parse
`self.items[0] = self.items[0]` and `self.items[0..1]` under each composed state
profile. Brackets after a value have no indexing/slicing production in this
edition; require a located syntax refusal, including for an unreachable branch.
Do not accept the `self.items` prefix, infer an operation from an IR index node,
or silently translate indexing into a quantifier. Temporal interval brackets
remain valid in their specified syntactic position.

## Expected Results

Include keyword/member controls: `view.exhausted`, `M::result` and
`M::Payment::retry` retain their model names in qualified positions, while
`let exhausted = true in exhausted` refuses as a reserved native binder.

Syntax trees match the declared associativity and scope, preserving original loci. Invalid/trailing forms refuse as syntax; recognized prohibited integer division parses but refuses profile admission. Rational slash shares product precedence and receives separate typed definedness checks. No temporal/protocol body is treated as arbitrary text.
