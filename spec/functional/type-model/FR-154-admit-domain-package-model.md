---
id: FR-154
title: "Admit the domain model from a domain package"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
  - target: ix://agent-ix/filament-core-data/FR-095
    type: references
  - target: ix://agent-ix/filament-core-data/FR-091
    type: references
---

# FR-154: Admit the domain model from a domain package

## Description

When a package imports a domain package, QSL model intake SHALL admit the
package by identity and `sha256-jcs` digest and SHALL build one Quire
model declaration for each IR node, refusing any invalid node with its IR node identity and
`origin`.

## Inputs

The source `model` declaration with its domain package identity and digest; the domain package bytes; and the selected `quire.model.complete/v1`
definition.

## Outputs

The original declarations of one ModelSelection (FR-321), each keyed by
`(domain package identity, IR node identity, sha256-jcs)`, or located refusals.

## Behavior

The domain model is declared in the spec's artifacts (AD-006). The model intake
chain is:

1. **Artifact bundle.** The spec package's artifacts declare the domain
   objects. Each domain object is one artifact whose object type, such as
   `entity`, comes from a module.
2. **Extraction.** quire-rs reads each artifact under its module manifest and
   yields field, relationship, operation, frame, clause, population and systems
   declarations with source spans.
3. **Lift.** The FCD extraction-frontend lowers the extracted declarations into
   one semantic IR 2.0.0 domain package, serialized with RFC 8785 JCS and
   fingerprinted with `sha256-jcs`.
4. **Admission.** Rule `quire.model.normalize.intake/v1` checks the selection
   and the package bytes in the order the selected definition fixes.
5. **Declarations.** Intake builds one original declaration per IR node,
   ascending by declaration key, and rule `quire.model.features/v1` checks
   each declaration.

The package input maps each `sha256-jcs` digest to package bytes. Intake looks
up the bytes by the selected digest, recomputes the canonical `sha256-jcs`
digest of the supplied document, compares it with the selected digest, and
then compares the package's own identity with the selection. The selected
digest is the key of that lookup and the content identity member of the
ModelSelection (FR-321).

Admission refusals, in order:

| Check | Refusal |
| --- | --- |
| The digest domain is exactly `sha256-jcs` | `stale_dependency`/`digest-domain-mismatch` |
| The package input supplies bytes under the selected digest | `missing_import`/`missing-selection` |
| The canonical `sha256-jcs` digest of the supplied document equals the selected digest | `stale_dependency`/`content-mismatch` with the selected and the recomputed digest |
| The package's own identity equals the selection's | `invalid_model_binding`/`wrong-model-selection` |

Declaration refusals:

| Failure | Refusal |
| --- | --- |
| A type definition's or population node's artifact id does not match `^[A-Za-z][A-Za-z0-9_]*$` | `invalid_model_binding`/`malformed-declaration` |
| A type definition's or population node's `kind` names no entry of the `constructs` table, or its construct has no `meaning` or a `meaning` that is not a Quire meaning id (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| An IR node is not valid for its construct's meaning, including a reference that names a node of the wrong meaning and a `supertypes` entry that names a node of another meaning (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| A supertype, field type, subsets, redefines, relationship end, frame entry, population member, `occurrenceField`, transition `from`, `to`, `trigger`, `guard` or `emits` entry, step `consumes` or `emits` entry, `persists` entry or namespace `members` entry names no node or member of the package | `missing_declaration`/`missing-name` |
| A node of a business model meaning (FR-208) declares a form that meaning does not support: a non-empty `supertypes` or a `quire` clause of a persistence interface or namespace, an operation of a record value type, or an operation contract of a persistence interface | `unsupported_construct`/`declaration-form` |
| Two nodes share one identity, including two states or two steps of one owner with one name, or two transitions of one state machine with equal `from`, `to` and `trigger` | `invalid_model_binding`/`conflicting-binding` |
| A multiplicity has `lower > upper` | `invalid_model_binding`/`unpreserved-model-meaning` |

When FCD extraction reports one of the following semantic diagnostics, the
FR-154 intake projection SHALL preserve the affected IR node and `origin` and
map it to the QSL refusal pair used by TC-235:

| FCD diagnostic | QSL refusal pair | Scope |
| --- | --- | --- |
| `UNRESOLVED_CONSTRUCT_REF` | `missing_declaration`/`missing-name` | A construct member reference names no declared node or member, including an occurrence field, transition state or trigger, or emitted/consumed type. |
| `DANGLING_CLAUSE_REF` | `missing_declaration`/`missing-name` | A transition guard names no clause of its owning type. |
| `INVALID_OCCURRENCE_FIELD` | `invalid_model_binding`/`malformed-declaration` | An occurrence field resolves to a field whose type, including aliases, is not scalar `datetime`; a missing target remains `UNRESOLVED_CONSTRUCT_REF`. |

Every declaration refusal retains the IR node identity and the node's FCD
`origin` verbatim (filament-core-data `schema/semantic/v1/common.schema.json`,
`$defs.origin`): a source origin gives `sourceIdentity`, `path`, `startLine`,
`startColumn` and, when present, `endLine` and `endColumn`, as given; a
generated origin gives `generatorIdentity`, `generatorVersion` and
`inputIdentities`, as given, with no span. The artifact id the object id rule checks is the `<Name>` segment of the IR node
identity; a FCD document carries no separate artifact id member. Nothing is synthesized: a node whose `origin` is missing or malformed
retains no origin. Intake
reports every declaration refusal in node order, and then no later phase runs.
Within one node, every check of the table that fails reports, in table order,
so the object id check reports first. Several failures of one row within one node report in member-path order: a member path is a sequence of segments, each a member name or a numeric index, where `[]` denotes the index-free segment of a list member and sorts as an index before every numeric index; paths compare segment by segment, a member name before a numeric index, member names by their UTF-8 bytes and indices numerically, and a proper prefix orders first. A reference that names a node refused by
an earlier row of the table is not reported again as
`missing_declaration`/`missing-name`; that node's own refusal stands for it.
An admission refusal ends intake with no declaration.

A model-declaring artifact is an artifact whose type is an object type of any
spec-objects module (spec-objects-architecture, spec-objects-business, …). Its artifact id is an object id: ASCII letters, digits and underscores,
starting with a letter (`^[A-Za-z][A-Za-z0-9_]*$`). An id such as `entity-001`
is not an object id, and intake refuses the node it declares
`invalid_model_binding`/`malformed-declaration`, naming the IR node identity
and its `origin`. Because every object id is a `member-name`, a
clause names each declaration by qualified name, such as `M::sys_pump`.
Artifact-type ids such as `FR-100` declare no model node and are outside this
rule.

A declaration's IR node identity is the identity filament-core-data FR-095
mints (the closed slot list of its "Node identities" section), rooted at
`ix://<package identity>/`, with `<Name>` the declaring artifact's id passed
through verbatim and every other part slugged. A type's identity is
`ix://<package identity>/<artifact id>`, and the alias of a constrained
field is `ix://<package identity>/<Name><Field>` (`<Field>` slugged with its
first character upper-cased), which names no slot segment of its own. A field, an operation and an operation
parameter nest under their owner: `<owner identity>/<field>`,
`<owner identity>/<operation>` and `<operation identity>/<param>`. Every other
node kind mints under its own slot and is not owner-nested: a relationship is
`ix://<package identity>/relationship/<Name>-<verb>-<TargetName>` with
`<TargetName>` the target artifact's id, a clause is
`ix://<package identity>/clause/<Name>-<clauseId>`, a constraint is
`ix://<package identity>/constraint/<Name>-<field>-<keyword>`, an enumeration
variant is `ix://<package identity>/variant/<Name>-<value>`, and a state,
transition and step are `state/<Name>-<state>`,
`transition/<Name>-<from>-<to>-<trigger>` and `step/<Name>-<step>` under the
same root. A relationship is declared in its source type's `relationships`
list (with `sourceEnd` and `targetEnd`), not under the source type's identity. The artifact title is the declaration's
`displayName`, used for presentation only and never part of a key or a name
lookup. Retitling an artifact keeps the declaration keys of the type it declares
and of that type's members. Changing an artifact's id declares a new type: its
declaration key and its members' keys change, and a reference to the old key
refuses `missing_declaration`/`missing-name`. Renaming a member changes that
member's key.

Intake binds each IR type definition and population node by the `meaning` of
its construct. The node's `kind` is `{module, name}` and names one entry of the package's
embedded `constructs` table; that entry's `meaning` is a Quire meaning id
(FR-208). The kind name and module select the construct entry only and never
select a meaning.

A declaration key carries no digest. Two domain package documents with equal nodes yield
equal declaration keys. The ModelSelection, which binds the digest, separates
them.

Intake charges `normalize.record` under `ModelNormalizationLimitsV1` of
`quire.value.accounting/v1` once per declaration, ascending by declaration key.
Admission checks are decided before the first charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-154-AC-1 | An artifact bundle whose extraction and lift yield a valid domain package is admitted, and intake builds exactly one original declaration per IR node, keyed by package identity and IR node identity. | Test (TC-234) |
| FR-154-AC-2 | A wrong digest domain, a missing package, a document whose recomputed canonical digest differs from the selected digest and a package whose identity differs each refuse with its named cause before any declaration is built, and a package supplied under the selected digest with the selection's identity is admitted. | Test (TC-234) |
| FR-154-AC-3 | A malformed IR node or a dangling reference refuses with the IR node identity and its `origin`, and every such refusal is reported in node order. | Test (TC-234) |
| FR-154-AC-4 | Publishing a domain package document with equal nodes and a different digest leaves every declaration key and effective declaration unchanged, while the ModelSelection, the effective view and the object universe change. | Test (TC-234) |
| FR-154-AC-5 | Intake at its exact `normalize.record` bound completes, and the one-less run is incomplete at `normalize.record`. | Test (TC-234) |
| FR-154-AC-6 | Retitling an artifact leaves every declaration key unchanged. Changing an artifact's id changes the declaration key of the type it declares and of that type's members, leaves every other declaration key unchanged, and a reference to the old key refuses `missing_declaration`/`missing-name`. | Test (TC-234) |
| FR-154-AC-7 | A type definition or population node whose kind names no construct entry, or whose construct has no Quire meaning id or an id outside FR-208, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, and no meaning is inferred from its kind name. | Test (TC-234) |
| FR-154-AC-8 | A type definition or population node declared by an artifact whose id does not match `^[A-Za-z][A-Za-z0-9_]*$`, such as `sys-pump`, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, a reference to that node reports no `missing_declaration`/`missing-name`, and a second failing check on the same node reports after the id refusal; the artifact id `sys_pump` is admitted and a clause names its declaration `M::sys_pump`. | Test (TC-234) |
| FR-154-AC-9 | The published domain package document of `acme/orders` (`domain-package-acme-orders.json`) carries exactly FR-095's identity forms: types `ix://acme/orders/<artifact id>`, fields, operations and parameters nested under their owner, and relationships `ix://acme/orders/relationship/<Name>-<verb>-<TargetName>` (`Order-order-Order`, `Order-invoiced-Invoice`, `Order-follows-Order`, `Order-twin-Order`), the constrained-field alias `ix://acme/orders/OrderTotal`, none of them owner-nested. | Test (TC-280) |
| FR-154-AC-10 | A relationship declaration of the selected document refuses at the selection row (`/lock/model_selections/<i>`), in node order and, within a node, in the order malformed declaration, unresolved end type, duplicate identity, reversed multiplicity: an identity outside the package's global `relationship` slot (owner-nested, foreign-package or wrong-slot), a missing or invalid `direction`, `category`, `composite` or `origin`, a missing, null, empty or non-string `sourceEnd.role`, a present null, empty or non-string `targetEnd.role` (an absent `targetEnd.role` is valid), a source end naming another owning type, a malformed end or multiplicity, and an end type naming a relationship or another declaration of the wrong meaning each refuse `invalid_model_binding`/`malformed-declaration`; an end type naming no declaration refuses `missing_declaration`/`missing-name`; a repeated relationship identity, including under two owners, refuses `invalid_model_binding`/`conflicting-binding`, and two missing identities are malformed declarations, not conflicting ones; `lower > upper` alone refuses `invalid_model_binding`/`unpreserved-model-meaning`. The published `adverse.json` `relationship_document_cases` hold one entry per refusal and three admitted entries. Each refusal retains the relationship's IR identity and its `origin` verbatim: `declaration_origin_cases` `refusal-retains-source-origin` (a source origin with its `sourceIdentity`, `path` and span) `refusal-retains-generated-origin` (a generated origin with its members and no span) and `refusal-retains-no-malformed-origin` (a malformed origin, refused `malformed-declaration`, retained as none). | Test (TC-280) |

## Dependencies

- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns the
  model intake design.
- [FR-208](../../objects/foundation/FR-208-quire-meaning-vocabulary.md) owns
  the Quire meaning ids intake binds.
- [Complete model definition](../../../proposals/quire-v1/definitions/model-complete.md)
  owns the intake and feature rules.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `model` declaration.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
