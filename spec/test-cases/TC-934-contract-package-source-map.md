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

## Source-frame id with an empty output graph

Lower a package whose only requested key is absent from the checked graph, so
the output `semantic_graph` is empty. The requested item keeps the exact
source-indexed id, is recorded `invalid_input`, and the package is admitted:
the id's index segments resolve against the source package's graph, not the
empty output graph.
