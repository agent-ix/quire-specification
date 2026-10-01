---
id: FR-450
title: "Spell and check an abstraction relation declaration"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-353
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-339
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---

# FR-450: Spell and check an abstraction relation declaration

## Description

When a source unit that selects `quire.model.complete/v1` declares an
`abstraction-decl`, the compiler SHALL read each of its bindings as one
FR-353 binding, keyed as FR-353 states, and SHALL refuse each binding whose
model name, Rust spelling or map entries are not admissible, naming the
binding's key.

## Inputs

- The source unit's `abstraction-decl` declarations, recognized by the
  [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md)
  "Abstraction relation declaration" production.
- The admitted domain packages the unit's `model` declarations select, with
  their object types, fields, populations and operations.

## Outputs

One checked abstraction relation per declaration, each binding keyed and
valued as below, or a located refusal per inadmissible binding.

## Behavior

### Binding forms

Each binding form denotes one FR-353 binding:

| Form | FR-353 element | Key | Value |
| --- | --- | --- | --- |
| `type M::T to "p" { f to "x"; ... }` | model object | the AD-006 effective-declaration key of object type `T` | `rust_type`: the Rust path `p`; `fields`: each field `f` to its Rust field `x` |
| `population M::P to "p";` | population | the AD-006 effective-declaration key of population `P` | `collection`: the Rust path `p`, from the implementation state root to the collection |
| `operation M::T::o to "f" receiver "r" (a to "x", ...);` | operation frame | the operation key: the AD-006 effective-declaration key of `T` and the operation identifier `o` | `function`: the Rust path `f`; `receiver`: the function input `r` that carries the operation's context object; `parameters`: each operation parameter `a` to the Rust parameter `x` |

`M` is a model alias the unit declares. An object binding with no braces
binds no field. The operation's result is the bound function's return value,
and the framed state is read through the receiver at the function's entry
and exit, so neither has a binding position.

### Rust spellings

A Rust string's content is its source bytes between the quotes. A content
byte sequence that holds a backslash is malformed, so the content has no
escape sequence and each segment's byte range inside it is a source span.

- A **Rust identifier** is `IDENTIFIER` of the Rust Reference for the 2021
  edition: a non-keyword identifier, or a raw identifier `r#` followed by an
  identifier or keyword other than `crate`, `self`, `super`, `Self` and `_`.
  Its content is in Unicode Normalization Form C, so two spellings of one
  Rust identifier are equal bytes.
- A `rust-path` is one or more Rust identifiers separated by `::`.
- A `rust-field` is one Rust identifier, or a tuple-field index: `0` or a
  decimal integer with no leading zero.
- A `rust-receiver` is exactly `self`, the bound function's method receiver,
  or one Rust identifier, which names one of the function's parameters.
- A `rust-name` is one Rust identifier.

These are lexical keys and compare by bytes. The compiler checks their
syntax only. It resolves none of them against Rust code; the generator that
emits implementation code resolves them.

### Refusals

The compiler SHALL refuse, in source order of the bindings, naming the
binding's key:

1. a `type` name that resolves to no object type of a selected domain
   package, a `population` name that resolves to no population, or an
   `operation` whose type does not itself declare an operation of that name:
   `missing_declaration`/`missing-name`, naming the name and its owning
   domain package identity;
2. a `type` name that resolves to a declaration other than an object type,
   or a `population` name that resolves to a declaration other than a
   population: `invalid_model_binding`/`malformed-declaration`;
3. a Rust string whose content is not of its position's syntax:
   `invalid_model_binding`/`malformed-declaration`, naming the first
   malformed segment and its span;
4. a field binding naming a name that is not an effective field member of
   the object type (own or inherited), a field named twice in one object
   binding, a parameter binding naming a name the operation does not declare,
   an operation binding that omits a declared parameter, names one twice,
   maps two parameters to one Rust parameter, or maps a parameter to the
   receiver's spelling: `invalid_model_binding`/`malformed-declaration`,
   naming the entry;
5. a second binding for a key an earlier binding in the package covers,
   with an equal or a different value: `invalid_model_binding`/
   `conflicting-binding`, naming both bindings and the key (FR-353-AC-4).
   Key uniqueness spans every `abstraction-decl` of the package. Two values
   are equal when every member is equal by bytes.

An operation a subtype inherits is keyed at the object type that declares
it, as its `state`/`operation_anchor` is
([FR-342](../../objects/interfaces/FR-342-checked-package-operation-anchor-body.md)),
so a binding keyed at the inheriting subtype refuses by rule 1.

A declaration may bind any subset of the model's elements. The unbound
refusal belongs to the item that references an unbound element
([FR-353](FR-353-declare-abstraction-relation.md)), not to the declaration.

### What the declaration is not

The declaration requests no capability kind
([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)); it is a
premise of the claims that reference its elements. It is not a clause, has
no evaluation and is carried in the checked package as one node
([FR-451](../../objects/interfaces/FR-451-checked-package-abstraction-relation-body.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-450-AC-1 | A declaration with one `type` binding naming two fields, one `population` binding and one `operation` binding parses losslessly and checks into a relation of exactly three bindings, keyed by the object type's and the population's effective-declaration keys and by the operation key. | Test (TC-410) |
| FR-450-AC-2 | A declaration binding only an object type, with no braces, checks with that one binding and an empty field map. | Test (TC-410) |
| FR-450-AC-3 | An `abstraction-decl` in a package that does not select `quire.model.complete/v1` refuses `unsupported_construct` at the declaration's span; a declaration with no binding, and a binding with an unquoted Rust path, refuse `invalid_syntax` at the offending token. | Test (TC-410) |
| FR-450-AC-4 | A `type` name absent from the domain package, a `population` name absent from it, and an `operation` its type only inherits each refuse `missing_declaration`/`missing-name` naming the name and its owning domain package identity; the same operation keyed at its declaring type checks. | Test (TC-411) |
| FR-450-AC-5 | The Rust paths `"type"`, `"9lives"`, `"a::"`, `"r#crate"` and `"a-b"`, the field index `"01"` and a non-NFC identifier each refuse `invalid_model_binding`/`malformed-declaration` naming the segment and its source span; `"r#type"`, `"config_store::Store"`, the field index `"0"` and the receiver `"self"` are admitted. | Test (TC-411) |
| FR-450-AC-6 | A field binding naming an undeclared field, a field named twice, and an operation binding that omits a parameter, names an undeclared one, maps two parameters to one Rust parameter or maps a parameter to the receiver's spelling each refuse `invalid_model_binding`/`malformed-declaration` naming the entry; a field the object type inherits is admitted. | Test (TC-411) |
| FR-450-AC-7 | Two equal bindings for one key and two different bindings for one key, in one declaration and across two declarations of one package, each refuse `invalid_model_binding`/`conflicting-binding` naming both bindings and the key. | Test (TC-411) |

## Dependencies

- [FR-353](FR-353-declare-abstraction-relation.md) owns the relation's
  meaning, its keys and its unbound and conflicting refusals.
- [FR-339](../source-package/FR-339-define-complete-native-grammar.md)
  incorporates the grammar production.
- [FR-342](../../objects/interfaces/FR-342-checked-package-operation-anchor-body.md)
  fixes the declaring-type anchoring of an operation.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns the
  effective-declaration key.
- Every requirement linked in frontmatter is a normative prerequisite.

## References

- quire-spec-language ADR-017 §3 AR-1 to AR-3 and §6 Q-1; ADR-020 §7 (the
  abstraction relation is a declaration separate from model refinement).
- Specification ticket STD-116.

## Status

Proposed complete-V1 source contract for the abstraction relation.
Implementation and qualification remain tracked separately.
