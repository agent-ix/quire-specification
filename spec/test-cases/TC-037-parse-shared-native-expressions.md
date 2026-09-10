---
id: TC-037
title: "Parse shared expressions with explicit precedence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-037
    type: verifies
---
## Description

Planned controls for [FR-037](../functional/FR-037-parse-shared-native-expressions.md).
These expected cases have not been implemented or executed.

## Test Procedure

Parse symbolic expressions a implies b implies c, a - b - c, a + b * c and not (a = b), retaining explicit groups. Exercise nested conditionals, let bindings, zero-argument predicate calls and a quantified field predicate. Supply a < b < c, mismatched braces, and trailing tokens. Offer a recognized division expression under the candidate restricted arithmetic profile.

## Expected Results

Syntax trees match the declared associativity and scope, preserving original loci. Invalid/trailing forms refuse as syntax; recognized prohibited division parses but refuses profile admission. No temporal/protocol body is treated as arbitrary text while its grammar is still missing.

