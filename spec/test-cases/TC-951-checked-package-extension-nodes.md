---
id: TC-951
title: "Checked-package extension form nodes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-469
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-951: Checked-package extension form nodes

## Description

Show that a checked package carries a selected extension's declaration form
as the `extension`/`extension_form` node FR-469 specifies, that its key binds
the definition, node kind, name, owner and clause subnodes and not its source
span, and that the V2 reader refuses each broken join with its named code,
cause and pointer. Scope: FR-469-AC-1 to FR-469-AC-7.

## Preconditions

- The `retention` extension of FR-467-AC-1 (production `retention`, clauses
  `subject: Text` and `days: Int[1, 3650]`, node kind `retention`) validates
  under FR-467.
- The published V2 package
  `proposals/checked-package-v2/fixtures/positive-extension-nodes.json`
  carries the form `retention Audit { subject: "audit", days: 30 }`, its two
  clause subnodes and a lock whose `definition_selections` holds the
  `retention` definition. Its node keys and `package_id` are recomputed from
  their preimages.
- Both V2 schemas enumerate the `extension` tag and `extension_form` form and
  define `ExtensionFormBody` and the `ExtensionFormNode` preimage.

## Test Procedure

Read the fixture through the strict V2 reader with the `retention` definition
held, then read each vector below. Unless a vector says otherwise, a mutation
keeps every other member, recomputes every changed node key, patches the
`identity_projection` entry and recomputes `package_id`.

| Vector | Input | Expected result |
| --- | --- | --- |
| X01 | The fixture | admitted; one `extension`/`extension_form` node with `declaration` `["Audit"]`, a `SourceOwner`, the `retention` `DefinitionRef`, `node_kind` `retention`, bindings `subject`, `days` in order, each a `reference` to a subnode typed `Text` and `Int[1, 3650]`, `dependencies` those subnodes in digest order, and one `declaration` occurrence whose region spans the whole form (FR-469-AC-1) |
| X02 | Recompute the node key from its `ExtensionFormNode` preimage; then change, one at a time, the definition `identity`, the `node_kind` (to a second node kind of the same definition), the declared name, the owner `identity` (with that source added to `lock.sources` and the region moved to it) and the `days` subnode (to `31`) | the recomputed key equals the retained key; each change yields a different node id and `package_id` (FR-469-AC-2) |
| X03 | Move the form's `source_map` region by one byte, keys untouched | admitted; node id and `package_id` unchanged (FR-469-AC-2) |
| X04 | X02's definition-identity change with the old node key retained | `refused { code: invalid_package, cause: stale-node-key }` at the node's `node_id` (FR-469-AC-2) |
| X05 | The fixture read by a reader that does not hold the `retention` definition | `refused { code: unknown_required_capability }` with no cause, `locus` the node's `node_id`, `document_pointer` `/semantic_graph/nodes/{i}/body/definition`; the lock, source map and node are byte-identical to the input (FR-469-AC-3) |
| X06 | The `retention` entry removed from `lock.definition_selections`, read by a reader that holds it and by one that does not | both `refused { code: missing_declaration, cause: missing-selection }` at `/body/definition` (FR-469-AC-3) |
| X07 | `node_kind` `retentions` | `refused { code: missing_declaration, cause: missing-name }` at `/body/node_kind` (FR-469-AC-4) |
| X08 | Clauses `days`, `subject`; a third clause `owner`; `subject` twice | each `refused { code: invalid_package, cause: invalid-value }` at the first offending `/body/clauses/{c}` (FR-469-AC-4) |
| X09 | Only the `subject` clause | `refused { code: invalid_package, cause: missing-member }` naming `days` at `/body/clauses` (FR-469-AC-4) |
| X10 | The `days` subnode replaced by a `Text` literal node | `refused { code: ill_typed, cause: type-mismatch }` naming `days`, `Int[1, 3650]` and `Text` at `/body/clauses/1/value` (FR-469-AC-5) |
| X11 | The `days` reference re-pointed at a key absent from the graph, `dependencies` patched | `refused { code: missing_declaration, cause: missing-name }` at `/body/clauses/1/value` (FR-469-AC-5) |
| X12 | A second production `policy` with clause `retain: retention` (a node-kind clause); a `policy` form whose `retain` subnode is an extension node of node kind `quota` | `refused { code: ill_typed, cause: type-mismatch }` at `/body/clauses/0/value`; the same form with `retain` referencing the `Audit` node is admitted (FR-469-AC-5) |
| X13 | The `days` value as an inline `literal`; a second occurrence; an `expression` occurrence; `semantic_form` `retention` | each fails the V2 schema and is `refused { code: malformed_wire }` (FR-469-AC-6) |
| X14 | `dependencies` without the `days` subnode | `refused { code: invalid_semantic_graph }` at `/dependencies` (FR-469-AC-6) |
| X15 | A declared record `Audit` added beside the form | `refused { code: ambiguous_declaration, cause: ambiguous-name }` at the later of the two nodes in node-id order (FR-469-AC-6) |
| X16 | A second held, selected extension with node kind `quota` and a form `quota Q { limit: 5 }` | admitted under both unchanged V2 schemas with tag `extension` and form `extension_form` (FR-469-AC-7) |

## Expected Results

Every vector gives exactly its expected result and reports no later refusal.
