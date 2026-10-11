---
id: TC-934
title: "ContractPackage source map is total for nodes, requests and refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---
# TC-934: ContractPackage source map is total for nodes, requests and refusals

## Description

Verify that source correspondence covers each graph node and request outcome,
including a request whose checked-node key is absent.

## Test Procedure

Lower a package with a represented expression that has multiple ordered
FR-322 source occurrences, one unsupported requested item, and one request
whose `CheckedNodeId` is absent from the checked graph. Inspect every
`source_map` collection and join each entry to its graph node, request, or
accounting refusal. Mutate one source region, omit one node entry, omit the
absent-key request entry, and add a fabricated region for that absent key.

## Expected Results

Every semantic node, each of the three requests, and each non-`lowered`
accounting record has exactly one source-map entry. Existing occurrence keys,
source identities, half-open UTF-8 byte regions and ordering match FR-322;
multiple source occurrences are retained in order. The absent `CheckedNodeId`
has an explicit request subject with an empty region list and an `invalid_input`
record, without an invented source span. Every mutation is refused as an
invalid source-map or accounting correspondence before consumption.
