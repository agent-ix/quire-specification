---
id: TC-040
title: "Admit state extensions through exact profile dependencies"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-040
    type: verifies
---
## Description

Planned controls for [FR-040](../functional/FR-040-admit-explicit-state-extensions.md).
No executable test or observed coverage is claimed.

## Test Procedure

Use the exact [candidate definitions](../../proposals/quire-v1/definitions/README.md)
and their selected rule snapshots, once adopted and implemented. Keep a separately
identified historical first-profile control. Exercise this admission table with
the same explicit finite typed model wherever that profile admits its types:

| Construct | Core | Queries | Graph |
| --- | --- | --- | --- |
| Bounded scalar invariant with proved arithmetic | Admit | Admit | Admit |
| Pure named Boolean predicate and correctly typed call | Refuse | Admit | Admit |
| Ordered filter/map/count/sum with proved bounds | Refuse | Admit | Admit |
| Explicit reference identity/navigation/reaches over a closed finite population | Refuse | Refuse | Admit |
| Integer division or recursive value containment | Refuse | Refuse | Refuse |

Supply exact dependency bytes for edition → core → queries → graph; inspect the
resolved graph rather than accepting the root filename. Independently remove a
required file, alter its digest, change bytes under one identity/revision, create
a dependency cycle in a newly identified mutant definition, and replace a
requirement with an incompatible definition. Known supported roots do not grant
support to these mutants. Each control expects its typed selection/dependency
cause and no checked dependent clause.

Import a query-profile predicate into a graph-profile caller, then put graph
navigation inside the predicate without changing its definition selection. The
wider caller cannot repair that callee. Put an otherwise prohibited construct
inside an unreachable branch to confirm admission does not depend on runtime
participation. Preserve exact historical definitions and source bindings.

## Expected Results

Admission follows each declaration's exact closure. No permission leaks from the caller/backend, and no historical source is reinterpreted. Missing/incompatible dependencies refuse their dependents with typed located causes.
