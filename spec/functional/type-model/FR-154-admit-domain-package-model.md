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
---

# FR-154: Admit the domain model from a domain package

## Description

When a package imports a domain package, QSL model intake SHALL admit the
package by identity, version and `sha256-jcs` digest and SHALL build one Quire
model declaration for each IR node, refusing any invalid node with its source
artifact and span.

## Inputs

The source `model` declaration with its domain package identity, version and
digest; the domain package bytes; and the selected `quire.model.complete/v1`
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
up the bytes by the selected digest, recomputes their digest, and only then
compares the package's own identity and version with the selection.

Admission refusals, in order:

| Check | Refusal |
| --- | --- |
| The digest domain is exactly `sha256-jcs` | `stale_dependency`/`digest-domain-mismatch` |
| The package input supplies bytes under the selected digest | `missing_import`/`missing-selection` |
| SHA-256 over the package's JCS bytes equals the selected digest | `stale_dependency`/`byte-digest-mismatch` |
| The package's own identity and version equal the selection | `invalid_model_binding`/`wrong-model-selection` |

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

Every declaration refusal retains the IR node identity, the source artifact id
and the source span `{artifact, start, end}` that the node carries. Intake
reports every declaration refusal in node order, and then no later phase runs.
Within one node, every check of the table that fails reports, in table order,
so the object id check reports first. Several failures of one row within one node report in member-path order: paths compare segment by segment, member names by their UTF-8 bytes and indices numerically, and a proper prefix orders first. A reference that names a node refused by
an earlier row of the table is not reported again as
`missing_declaration`/`missing-name`; that node's own refusal stands for it.
An admission refusal ends intake with no declaration.

A model-declaring artifact is an artifact whose type is an object type of any
spec-objects module (spec-objects-architecture, spec-objects-business, …). Its artifact id is an object id: ASCII letters, digits and underscores,
starting with a letter (`^[A-Za-z][A-Za-z0-9_]*$`). An id such as `entity-001`
is not an object id, and intake refuses the node it declares
`invalid_model_binding`/`malformed-declaration`, naming the IR node identity,
the artifact id and the span. Because every object id is a `member-name`, a
clause names each declaration by qualified name, such as `M::sys_pump`.
Artifact-type ids such as `FR-100` declare no model node and are outside this
rule.

A declaration's IR node identity is its artifact id: a type's identity is
`ix://<package identity>/<artifact id>`, and a member's identity is its owner's
identity, `/` and the member name. The artifact title is the declaration's
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

A declaration key carries no digest. Two versions of one domain package with equal nodes yield
equal declaration keys. The ModelSelection, which binds the digest, separates
them.

Intake charges `normalize.record` under `ModelNormalizationLimitsV1` of
`quire.value.accounting/v1` once per declaration, ascending by declaration key.
Admission checks are decided before the first charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-154-AC-1 | An artifact bundle whose extraction and lift yield a valid domain package is admitted, and intake builds exactly one original declaration per IR node, keyed by package identity and IR node identity. | Test (TC-234) |
| FR-154-AC-2 | A wrong digest domain, a stale digest, a missing package and a package whose identity or version differs each refuse with its named cause before any declaration is built. | Test (TC-234) |
| FR-154-AC-3 | A malformed IR node or a dangling reference refuses with the IR node identity, the artifact id and the span, and every such refusal is reported in node order. | Test (TC-234) |
| FR-154-AC-4 | Publishing a new version of a domain package with equal nodes leaves every declaration key and effective declaration unchanged, while the ModelSelection, the effective view and the object universe change. | Test (TC-234) |
| FR-154-AC-5 | Intake at its exact `normalize.record` bound completes, and the one-less run is incomplete at `normalize.record`. | Test (TC-234) |
| FR-154-AC-6 | Retitling an artifact leaves every declaration key unchanged. Changing an artifact's id changes the declaration key of the type it declares and of that type's members, leaves every other declaration key unchanged, and a reference to the old key refuses `missing_declaration`/`missing-name`. | Test (TC-234) |
| FR-154-AC-7 | A type definition or population node whose kind names no construct entry, or whose construct has no Quire meaning id or an id outside FR-208, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity, the artifact id and the span, and no meaning is inferred from its kind name. | Test (TC-234) |
| FR-154-AC-8 | A type definition or population node declared by an artifact whose id does not match `^[A-Za-z][A-Za-z0-9_]*$`, such as `sys-pump`, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity, the artifact id and the span, a reference to that node reports no `missing_declaration`/`missing-name`, and a second failing check on the same node reports after the id refusal; the artifact id `sys_pump` is admitted and a clause names its declaration `M::sys_pump`. | Test (TC-234) |

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
