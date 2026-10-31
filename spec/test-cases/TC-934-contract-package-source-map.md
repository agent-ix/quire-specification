---
id: TC-934
title: "ContractPackage source map is total for nodes, requests and refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-2
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-7
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-14
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-16
    type: verifies
---
# TC-934: ContractPackage source map is total for nodes, requests and refusals

## Description

Verify that source correspondence covers each graph node and request outcome,
including a request whose node_id is absent.

## Test Procedure

Lower a package with three requested items in this order: a represented
expression with multiple ordered FR-322 source occurrences, one unsupported
item, and one request whose node_id is absent from the checked graph.
Inspect every `source_map` collection and join each entry to its graph node,
request, or accounting refusal. Apply one mutation per case: select the `nodes`
entry whose `node_id` is the represented expression's node, record its array
index `i`, and change `start` to a value beyond `end` at
`/source_map/nodes/i/occurrences/0/regions/0/start`; omit node entry 0; omit
absent-key request entry 2; or add a fully well-formed but non-corresponding
`SourceMapEntry` at `/source_map/requested_items/2/occurrences/0`.

## Expected Results

Every semantic node, each of the three requests, and each non-`lowered`
accounting record has exactly one source-map entry. Existing occurrence keys,
source identities, half-open UTF-8 byte regions and ordering match FR-322;
multiple source occurrences are retained in order. The absent node_id
has an explicit request subject with an empty region list and an `invalid_input`
record, without an invented source span. The invalid interval is refused as
`invalid_package`/`invalid-value` at the exact selected path
`/source_map/nodes/i/occurrences/0/regions/0/start`; the fabricated entry is
refused as `invalid_package`/`invalid-value` at
`/source_map/requested_items/2/occurrences/0`. The omitted node and request
entries are refused as `invalid_package`/`invalid-value` for source-map
totality at `/source_map` before consumption. Encode each valid-JSON mutation
as canonical RFC 8785 bytes before reading.

## Source-frame ids and the empty output graph (FR-330-AC-14, AC-16)

Lower a package whose only requested key is absent from the checked graph and
derived: its structural key `{package, path, ordinal}` holds at least one
operand, which the source wrote as an index segment. The output `semantic_graph`
is empty. Require `source_keys` to carry the operand's key and the requested
key exactly as the source wrote them, the requested key's index segment
`{"node": i}` naming another `source_keys` entry, and the requested item,
accounting `node_id` and source-map `requested_items` entry to be the source-key
reference to the requested key. Require the package to be admitted with
`invalid_input`, an empty output graph and an empty `occurrences` array. Then
permute `source_keys` (rewriting every `source_key` reference and index
segment) and, for a `lowered` item, the output graph: every accounting, bijection and refusal-pointer comparison is unchanged, and the
`lowered` record's source key rebuilds to its output root's structural key.
