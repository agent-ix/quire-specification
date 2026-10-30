---
id: FR-141
title: "Evaluate text and declaration-qualified enumerations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-141: Evaluate text and declaration-qualified enumerations

## Description

When checking text or enumeration values, the semantic kernel SHALL apply the
selected text normalization and declaration-qualified member identity rules.

## Inputs

Text profile, text bounds, enum declarations, literals and comparison requests.

## Outputs

Completed typed text/enum values or comparison results, or an undefined,
refused or incomplete evaluator outcome. I13 dispositions are not evaluator
outputs.

## Behavior

Text equality and ordering operate on the selected Unicode scalar/normalization
profile and enforce declared length bounds. Enumeration members are identified
by declaration identity plus member identity; source order defines ordering
only when the declaration explicitly selects ordered enumeration semantics.
Evolution never reuses a removed member identity for different meaning.

## Text and enumeration profiles

`unicode-scalars` compares the decoded scalar sequence without normalization;
`nfc`, `nfd`, `nfkc` and `nfkd` apply exactly the named Unicode normalization
form using the `quire.value.text.unicode-17.0.0/v1` tables; `binary-utf8`
compares the admitted UTF-8 payload bytes. A source string's payload is the
canonical UTF-8 encoding of its decoded scalar sequence, so raw `é` and
`\u00e9` have one binary value while their different source spellings remain
provenance. A runtime text input supplies its validated payload bytes directly.
Length is the scalar count of the retained sequence under the selected
profile, which is the normalized sequence for `nfc`, `nfd`, `nfkc` and `nfkd`
and the decoded sequence for `unicode-scalars`, and the payload byte count for
`binary-utf8`; declared text bounds apply to that length, and a length outside
them refuses as
`refused { code: text_length_out_of_domain, cause: outside-domain }` with no
value. Every applicable
text charge up to the bound check is made first, so an exhausted counter returns
incomplete before any bound refusal: under `nfc`, `nfd`, `nfkc` and `nfkd` the
length-bound refusal follows every `text.normalize-output` charge, under
`unicode-scalars` it follows `text.decode-scalars`, and under `binary-utf8` it
follows `text.input-bytes`; in each case it precedes `text.result-retain`. Ordering is lexicographic over normalized
scalar values or unsigned bytes respectively. No locale, case folding,
collation table or grapheme segmentation is implicit.

An enumeration value is (`declaration identity`, `member identity`). Unordered
enumerations admit only equality/inequality. An ordering request on an
unordered enumeration, or any comparison of members of different declarations,
is refused by type checking with `refused { code: ill_typed }` before any
evaluation or `enum.*` charge. An `ordered enum` additionally
uses declaration order for comparison; insertion or reordering therefore
creates a new declaration revision. Optional display strings never participate
in equality or ordering.

Declaration identity is the structural `node_id` of an I04 `scalar_type`/`enum`
node, `{package, path, ordinal}`, where `path` is the declaration path of the
enum and `package` is the identity of the exact admitted source, DefinitionRef
or selected domain package that owns it (it must join that lock or model_root
selection). The declaration-ordered list of semantic case names (sorted by case
name for an unordered enum) is the enum node's body. Member identity is the
`node_id` of an I04 `value`/`enum_value` node whose `semantic_type` references
that declaration; its `path` is `["value", "enum_value", declaration_node_id,
case]`. For an ordinary `scalar_type`/`enum` node `case` is an identifier
(`^[A-Za-z_][A-Za-z0-9_]*$`). For a member of a `model`/`variant_type` node,
`case` is the declared case name, any non-empty string (`Gold Plus`, `2x`,
`Café`), compared byte for byte as UTF-8 and never altered to fit an
identifier. Display text and source loci are excluded from both ids. The
enum-value node has that declaration as its sole dependency and its enum
literal body carries the same case; the strict reader joins all three fields.

An enumeration declared by a model package is a `model`/`variant_type` node
(FR-322), not a `scalar_type`/`enum` node: its declaration identity is that
node's `node_id`, a function of the selected document's identity and the
declaration path alone, never of a model alias. Its member identity is the
`node_id` of the `value`/`enum_value` node whose `semantic_type` references it,
over that node id and the case name. Two units that select one package under
different aliases therefore assign equal ids, and the same case built in
either is equal.

The strict I04 reader joins the enum nodes of `scalar_type`/`enum` nodes with
the admitted semantic graph. Changing owner, declaration membership, case
meaning or ordered position of such a node is `invalid_semantic_graph` unless
its body and its enum-value members agree. For a `model`/`variant_type` node
the declaration id names no case, so adding, renaming or removing a case leaves
it unchanged (FR-322). The same exact imported owner and declaration retain the
same ids across otherwise different packages; an unrelated package edit cannot
rename them. Neither id is a local name,
source position or caller-computed assertion. Under
`quire.value.accounting/v1`, text normalization charges the named input,
normalization and result points before publishing a value; exhaustion is
incomplete, not a truncated string.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-141-AC-1 | Canonically equivalent text compares according to the selected normalization profile and distinct text remains distinct. | Test (TC-186) |
| FR-141-AC-2 | Comparing members of different enum declarations refuses `ill_typed`; equal member spellings are never equal across declarations. | Test (TC-186) |
| FR-141-AC-3 | Missing text bounds, invalid scalars, unordered enum ordering or an enum node whose body disagrees with its enum-value members refuses. | Test (TC-186) |
| FR-141-AC-4 | Each text profile applies its exact length and lexicographic comparison domain, with scalar length measured after the profile's normalization and the length-bound refusal, `text_length_out_of_domain`/`outside-domain`, after that profile's last size charge and before `text.result-retain`; equal display text under different profiles remains differently typed. | Test (TC-186) |
| FR-141-AC-5 | An unordered enumeration comparison other than equality, or any cross-declaration enumeration comparison, refuses with `ill_typed` before any charge, while an ordered enumeration follows declaration order only. | Test (TC-186) |
| FR-141-AC-6 | Exact-bound accounting succeeds and denial of a named next text or enum charge returns incomplete without a truncated value or Boolean. | Test (TC-186) |
| FR-141-AC-7 | The `enum_value` node of a case of a model variant type, `Tier` with `Gold`, has the member id over `declaration_node_id` the `model`/`variant_type` node id of `Tier` and `case` `Gold`; two units that select the package as `Config` and as `Cfg` derive equal ids for `Tier` and for `Gold`, and `Gold` of one equals `Gold` of the other. A case name changed in the package yields a new member id, and `Tier::Gold` and `Level::Gold` are unequal. | Test (TC-940) |
| FR-141-AC-8 | A variant type with cases `Gold`, `Gold Plus`, `2x` and `Café` is admitted, each case has its own member key over its exact name, the four keys are pairwise distinct, and `Gold` and `Gold Plus` are unequal. A `Set` of those cases orders `2x`, `Café`, `Gold`, `Gold Plus` (ascending UTF-8 bytes). An `enum_value` node of an ordinary `scalar_type`/`enum` whose case is not an identifier refuses `invalid_semantic_graph` at that node, since the case is not among the declaration's members. | Test (TC-942) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
