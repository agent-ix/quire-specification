---
id: interface_004
title: "I04 immutable checked package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# [interface_004] I04 immutable checked package

## Features

| Feature | Kind |
| --- | --- |
| link_and_check | operation |
| read | operation |
| inspect | operation |

## Contract

```yaml
name: CheckedPackageArtifactV2
associated_types:
  - CheckedSemanticGraphV2
  - CheckedOperationCatalogV1
  - PackageLockV2
  - SourceDocumentRef
  - DefinitionArtifactRef
  - ModelRootNode
  - SourceMapV2
  - CapabilityReportV2
  - DiagnosticReportV2
  - CheckedPackageReadLimits
operations:
  - { name: link_and_check, inputs: [ParsedSource, ModelSelection, profile bundle], output: CheckedPackageArtifactV2 or located refusals, semantics: resolves exact definitions types dependencies nominal identities and capabilities }
  - { name: read, inputs: [canonical CheckedPackageArtifactV2 bytes, CheckedPackageReadLimits], output: immutable checked package or typed refusal/incomplete result, semantics: verifies the selected wire, closed structure, canonical bytes, structural node identities and bounded graph before consumption }
  - { name: inspect, inputs: [CheckedPackageArtifactV2], output: immutable declarations capabilities and diagnostics, semantics: does not execute or lower }
invariants:
  - runtime inputs do not participate in static package identity
  - backend support does not alter admission
  - a checked package is identified by its `name` and its declared contents compared structurally
  - the public semantic graph is a closed union owned by this interface; consumers do not receive parser/CST/package implementation objects
  - every node id is the structural `{package, path, ordinal}` of FR-322, compared componentwise
  - every V2 application term carries one closed catalogued operation identity with exactly its required lock-selected laws (a clause its lock-selected profile law and one profile operator member), type-pinned mode, member, structural or keyed-collection leaves and declared result type, bound into its application node
  - every literal term carries its declared type; no application result or literal type is inferred
  - every named, source-declared node carries exactly one package-local declaration path, bound into its node identity; builtin and anonymous nodes carry none; library exports resolve only through it
  - a reader retains invalid unsupported incomplete and failed dispositions; it cannot replace one with an empty graph or a successful package
```

## Public boundary

`quire.checked-package/v2` is the complete-V1 I04 interchange emitted for
checked packages. Its normative closed schema and independently authored
vectors are published under
[`proposals/checked-package-v2`](../../../proposals/checked-package-v2/). The
schema defines the transport form; the grammar and the linked normative
requirements retain authority for language meaning. This boundary deliberately
does not expose a parser CST, a filesystem path, a display string, or an
implementation-private native package structure.

The `read` operation is a strict, bounded admission operation. It verifies the
declared wire before graph interpretation and returns a typed refusal for a
document that is not a checked package, a malformed or duplicate member, unknown capability, or graph/source-map invariant violation.
Exceeding a selected byte, node, edge, locus, diagnostic, or work limit
returns an incomplete resource disposition naming the limit, its value and its
setting, and no admitted package. A node body follows FR-322's closed
five-stratum body grammar, so the wire's JSON depth is fixed by its schema.

## Operation identity

A V2 `application` term carries its operator class, its exact
`operation: {identity, laws, mode, member, leaves}`, its declared
`result_type` and its arguments: a closed
`quire.checked-operation-catalog/v1` identity from the
`contracts/checked-operation-catalog-v1.json` of `agent-ix/quire-verification-contracts`, the ordered `{role, definition}` laws the catalog
requires for that identity, its `rounding`, `text_profile` or `absence` mode,
its member (record field, tuple position, element, relationship end, dispatched
operation or type argument, each naming its declaring node, or a clause's
`profile_operator`) and one `{path, laws, mode}` leaf for every text leaf of a
structurally compared type or of a keyed collection result's element type. A
`literal` term carries its declared `type`. The operator class alone never
selects floor, truncating or Euclidean division, an IEEE rounding attribute, a
text comparison profile, an absence behaviour, a temporal or protocol operator
or a member, and no result or literal type is inferred. Types are
authoritative: a `rounding` or `text_profile` mode equals the value the operand
and result types pin. Each value law definition joins the package lock's
`definition_selections`; a clause's profile law (`temporal_profile` or
`protocol_profile`) equals the lock's `profile_selections` row of that role, and
its operator is one that profile defines. The families, groups, constraints and
FR-149 conversion classes that decide eligibility are the normative table of
[FR-322](./FR-322-checked-package-artifact.md#operation-families-groups-and-constraints):
range and rational narrowing are checked obligations, and only a decimal scale
reduction rounds.

Every node whose body contains an application has a structural `node_id`
over its derived form (FR-322), so a changed operation, law, mode, member,
leaf, result type or declaration yields a different node. `read` refuses with the causes and order owned by
`quire.native.diagnostics/v1`:

| Order | Cause | Code |
| --- | --- | --- |
| 1 | `unknown-operation` | `invalid_package` |
| 2 | `operation-class-mismatch` | `invalid_package` |
| 3 | `operation-law-missing` | `invalid_package` |
| 4 | `operation-law-mismatch` | `invalid_package` |
| 5 | `operation-law-unselected` | `invalid_package` |
| 6 | `operation-mode-mismatch` | `invalid_package` |
| 7 | `operation-member-mismatch` | `invalid_package` |
| 8 | `operation-mode-type-mismatch` | `invalid_package` |
| 9 | `operator-ineligible` | `ill_typed` |

Causes 1 to 7 of an application precede its argument applications' refusals,
which precede its causes 8 and 9 and then its leaves. Nodes are checked in
ascending node-id order and applications in pre-order. Before any
operation refusal the reader checks, in order, the graph shape
(`invalid_semantic_graph`), `declaration-nominal-mismatch`
(`invalid_package`) and then `ambiguous-name` (`ambiguous_declaration`) over
the nodes' `declaration` members, and then, at the first `state`/`frame` node
in ascending node-id order carrying a defect
([FR-340](./FR-340-checked-package-frame-location.md)), that node's own
defect: a `missing-name`/`missing_declaration` or `malformed-declaration`/
`invalid_model_binding` meaning-join defect, or a
`conflicting-binding`/`invalid_model_binding` defect, outranks that same frame
node's own `invalid_semantic_graph` canonical-order defect; and then the state
step ([FR-341](./FR-341-checked-package-state-clause-body.md)): the
placement of every `quire.op.state.clause` application, then each
`state`/`operation_anchor` node's joins
([FR-342](./FR-342-checked-package-operation-anchor-body.md)), then each
`state`/`state_clause` node's joins, each in ascending node-id order.

## Declaration

A named, source-declared `scalar_type`, `composite_type`, `bounded_domain`,
`function`, `model`, `protocol` or `claim` node, and a non-enum `value` node,
carries `declaration: {qualified_name}` exactly when it has a `declaration`
source occurrence; builtin and anonymous nodes, expressions and every other node
do not, and no name is invented. `qualified_name` is the package-local
declaration path. It is part of the node identity. It equals a nominal node's
`qualified_declaration`, and no two nodes share it. A qualified reference
`a::Name` resolves `a` to exactly one locked package and then matches `Name`
byte for byte against that package's declaration paths. Library exports
([FR-307](../../functional/tooling/FR-307-package-reusable-semantic-libraries.md))
resolve only through this member.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface_004-AC-1 | Every application term of each published V2 package names exactly one catalogued operation identity and explicitly states its laws, mode, member and leaves, all admitted by the catalog and the package lock. | Test (TC-233) |
| interface_004-AC-2 | Changing an application's operation identity, law or absence mode yields a different node; the changed package is admitted, refused as `operation-law-unselected` for an unlocked law, or refused as `operator-ineligible` when the mode disagrees with the retained result type. | Test (TC-233) |
