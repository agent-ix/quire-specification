---
id: FR-146
title: "Check total pure functions and recursion"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-033
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
  - target: ix://agent-ix/quire-specification/FR-145
    type: references
---

# FR-146: Check total pure functions and recursion

## Description

When admitting a function declaration, the checker SHALL establish typed
purity, total definedness and a well-founded termination argument for every
reachable call cycle.

## Inputs

Function declarations, parameter/result types, call graph, effects declaration,
termination measure and selected type/model closure.

## Outputs

Checked callable identities or located type, effect, definedness or termination
refusals.

## Behavior

Functions may return any complete-V1 value type. Calls bind exact declaration
identity, arity and ordered arguments. Recursion uses a declared well-founded
measure proven to decrease on every cycle. Reflection, mutation, I/O and ambient
lookups are prohibited. Runtime depth/fuel exhaustion remains incomplete.

## Termination and totality contract

The checker constructs a declaration-identity call graph and checks each
strongly connected component. An acyclic component needs no termination
measure. Every recursive component declares one shared lexicographically
ordered tuple whose elements are nonnegative mathematical integers, finite
collection cardinalities or strict-subvalue depths. Each recursive edge must
prove that the substituted callee tuple is lexicographically smaller than the
caller's tuple; this rule also applies across mutual recursion.

Totality is path-sensitive: every conditional path must return the declared
result, and each partial operation must have a proved precondition. The
Complete-V1 expression grammar has no pattern-matching or destructuring form,
and every `if` has an `else`, so exhaustiveness is structural. Purity and typing are checked before
reachability, so an unreachable impure or ill-typed branch is still rejected.
Evaluation fuel is an execution resource rather than a semantic termination
argument: exhausting it returns `incomplete`, never a value or a proof of
nontermination.

## Calls, measures and definedness facts

Functions are first order. A function is not a value, and a call names a
`function` declaration or FR-307 export by its exact identity, so the call graph
is static. The graph includes calls inside `let`, `if`, collection binders, fold
steps and measure expressions. A call to a reserved `quire::value::...`
intrinsic is not a call-graph edge. A function declaration is
`function f using V (p: T, ...): R pure [decreases(m)] { body }`, where `V` is
the declared value-profile alias.

Only a function in a recursive strongly connected component of the call graph,
including a function that calls itself, needs `decreases`. A non-recursive
function needs no measure. A measure supplied on a non-recursive function is
still type checked and imposes no obligation.

`decreases(m)` supplies the measure. `m` is either one measure element or a call
of a declared `tuple` whose positions are measure elements. A one-element
measure is a one-tuple. A measure element is an `Integer` or `Int[..]`
expression over parameters that is proved nonnegative on entry, or a parameter
of a recursive record type ordered by strict containment. `size(p)` is a
cardinality element. Every function in one strongly connected component
declares a measure with the same arity and the same element kind at each
position. For each recursive edge, the checker substitutes the callee arguments
into the callee measure. It must then prove, under the sound FR-044 guard facts
at that call site, that for some position `i` every earlier element is equal and
element `i` is strictly smaller: an integer by `<`, or a record subvalue by
being a nonempty field projection path of the caller's parameter. A projection
path step is a field projection `.f`, or `value(q)` of an `Option` path `q`
whose `present(q)` is a proved guard fact at that call site; for example
`value(n.next)` under `present(n.next)` is a strict subvalue of `n`. The call-site
nonnegativity of every integer element must also be proved. An omitted or
mismatched measure, or an unproved decrease or nonnegativity obligation, is
`refused { code: undefined_expression, cause: unproved-decrease }`. The
refusal names the component cycle, the call edge and the failed obligation
(`missing-measure`, `measure-arity`, `measure-kind`, `nonnegative` or
`decrease`). `undefined_expression` is the FR-271 code for unproved
definedness, and `unproved-decrease` is its catalogued cause for termination.

The partial operations are closed, each with its `undefined_expression` cause:
a divisor that may be zero in FR-140, FR-142, FR-044 or FR-147
(`unproved-nonzero`); a zero base with a negative integer exponent
(`unproved-nonzero`); `value(e)` without a proved `present(e)`
(`unproved-presence`); `deref` outside the state contract's existence facts
(`unproved-presence`); `reduce` without a proved `size(c) >= 1` under FR-145
(`unproved-range`); and an FR-148 IEEE-to-`Rational` conversion of a value that
may be NaN or infinite (`unproved-range`). Each one needs a proved precondition
on a path that can execute, and an unproved one is
`refused { code: undefined_expression, cause: ... }` at that operation when the
body is linked. Runtime refusals
named by FR-140, FR-142, FR-144 and FR-148 (membership, strict `exact`,
cardinality bound, NaN payload and rational domain) are typed outcomes. They
propagate and do not make a function partial. FR-044 still owns the static
integer and rational result-bound obligations.

Purity, name resolution and typing are checked first, at the offending
expression, even in an unreachable branch. Complete V1 has no source form for
I/O, mutation, reflection or ambient lookup. In a function body, a qualified
call whose name resolves to a declaration other than a checked pure `function`,
tuple constructor or reserved intrinsic, such as a model operation, predicate or
clause, is `refused { code: ill_typed, cause: operator-ineligible }`. A call
whose name resolves to no declaration, such as an undeclared host operation, is
`refused { code: missing_declaration, cause: missing-name }`. Every other typing
failure is `refused { code: ill_typed }` with its FR-272 cause.

Each call charges `function.call` under `quire.value.accounting/v1` before its
arguments are bound. Arguments are evaluated left to right before that charge.
Execution fuel is the `work_units` limit. Its exhaustion is
`incomplete { limit_kind: work_units, ... }` at the denied point, and the
function's static totality verdict is unchanged. Call nesting depth is not a
separate counter, and an implementation must not turn a host stack limit into a
Complete-V1 outcome.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-146-AC-1 | A pure recursive function with a proven decreasing finite measure is admitted and evaluates its declared result. | Test (TC-191) |
| FR-146-AC-2 | A nondecreasing or unproved call cycle refuses with the cycle and failed obligation. | Test (TC-191) |
| FR-146-AC-3 | I/O, mutation, reflection, ambient access or wrong argument/result typing refuses even in an unreachable branch. | Test (TC-191) |
| FR-146-AC-4 | A mutually recursive component is admitted only when every substituted cycle edge decreases the shared lexicographic measure. | Test (TC-191) |
| FR-146-AC-5 | Exhausting execution fuel for an admitted function returns incomplete without changing the function's totality verdict. | Test (TC-191) |
| FR-146-AC-6 | Each call charges `function.call` after argument evaluation and before binding, and an unproved partial-operation precondition refuses as `undefined_expression` while runtime typed refusals propagate. | Test (TC-191) |
| FR-146-AC-7 | A missing, mismatched or undecreasing measure in a recursive component is `undefined_expression` with cause `unproved-decrease`, a non-recursive function needs no measure, and a call to a non-function declaration or an undeclared name refuses with its named code. | Test (TC-191) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
