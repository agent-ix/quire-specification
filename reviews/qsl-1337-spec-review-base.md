---
id: SR-1337001
title: "Spec review of quire-specification PR #546 (QSL-1337): FR-322 operand index segments"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@81b2a6af (PR #546, fr322-operand-node-reference) vs origin/main; FR-322 Structural node identity and Operand index segments, FR-322-AC-36, AC-95, AC-278..AC-281, FR-201 NodeId row, TC-233 NI-01..NI-13, proposals/checked-package-v2 schema, fixtures and vectors, proposals/backend-provider-v1/vectors.json; cross-checked FR-336 and proposals/conformance-corpus-v1/schema.json, FR-375 RefinementKey, FR-418, FR-479, TC-952"
review_set: base
date: "2026-10-31"
---

## Summary

Ticket: QSL-1337. The index-segment design keeps node identity exactly
structural inside one checked package. The structural key is well-defined for
any acyclic, non-topological graph. The refusal order (out-of-range in
canonical byte order, then a cycle by holding node and path position) is
total, and it sits as a wire check ahead of the join stages. The mechanical
conversion is faithful: for every converted fixture, rebuilding the keys
reproduces the origin/main document exactly. Every vector file maps each index
to exactly one original nested id, and the ids agree with the base fixture, or
with the node a patch inserts. The four new TC-233 tests pass at the head.

Two cross-document gaps remain. First, the conformance corpus wire still nests
node ids. Second, FR-375's external `{package, node}` refinement keys were not
given the dependency-reference treatment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `proposals/conformance-corpus-v1/schema.json` `$defs/NodeId` still allows only nested operand node ids and rejects `{"node": i}`. FR-336 says every corpus node id is an FR-322 node id: replay `parameter` keys, `occurrence_key`, `obligation_identity`, `DomainKey`. FR-322 now says such an external document holds the id "as the package writes it", with index segments, and that a path never holds a node_id object. A real `value`/`parameter` or `bounded_domain` id is derived and has operands, so it has no conformant corpus encoding. | proposals/conformance-corpus-v1/schema.json:1730; spec/objects/interfaces/FR-336-conformance-corpus.md:164; spec/objects/interfaces/FR-322-checked-package-artifact.md:1082-1095 |
| FND-002 | high | FR-375's external `{package, node}` `RefinementKey` (`abstract`, `protocol`, and the abstract operation target `operation` naming a dependency's no-point `operation_anchor`) carries a full `node_id` of a node in a dependency graph. Dependency nodes join through the dependency package and are not in the importing graph. The new Scope bullet says every index segment in the document names the importing graph. Only `dependency_reference` got string-only `DeclaredNodeId`. An `operation_anchor` is derived, with operand index segments, as in every published anchor. So this key is either inexpressible or resolves against the wrong graph. | spec/functional/analysis/FR-375-declare-model-refinement.md:418-423,514-515; spec/objects/interfaces/FR-322-checked-package-artifact.md:1082-1099 |
| FND-003 | medium | Leftover text: the `dependency_reference` paragraph still says "`node` has the `NodeId` shape". The new "Dependency references" bullet and the schema make it `DeclaredNodeId`, with string-only paths, so FR-322 states two shapes for one member. | spec/objects/interfaces/FR-322-checked-package-artifact.md:716,1096-1100 |
| FND-004 | medium | FR-322-AC-95 is newly traced by `tc_233_a_reordered_graph_keeps_every_structural_key`. That test only runs the index checks and compares rebuilt keys on the reversed graph. It never runs the full reader on the reordered package, and it never builds the "sorted ascending by node-id order" variant that the AC names. The AC's admission claim stays untested while the matrix reports it as covered. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3278; tests/checked_package_v2.rs:18094-18140 |

## Verdict

Not mergeable until FND-001 and FND-002 are resolved. Each needs a stated rule
for how an id that names a node outside this document is written: string-only
`DeclaredNodeId` where the target is always declared, or an explicit
graph-scoped encoding. The corpus schema also has to adopt the checked-package
`NodeId` shape. FND-003 is a one-line fix. FND-004 needs the AC-95 test to run
the reader on both reorders, or the trace narrowed to the clause it covers.

Clean units: FR-201 NodeId row; Operand index segments Order, Reader checks,
Rebuilding the key, Producing and Notation bullets; FR-322-AC-278..AC-281 and
TC-233 NI-01..NI-13; and the Body grammar and resource-limit depth statements.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The fix for FND-001/FND-002 added a rule to the FR-322 Scope bullet: "A node id outside its own package document resolves its index segments against the `semantic_graph.nodes` of the package its `package` member names." That rule keys on `node_id.package`, which is the node's owner, not the document that holds it. An imported derived node sits in the importing graph and carries the dependency's `package`. In `positive-dependency-call.json`, node 2 is `{package: "test/predicates", path: [..., {"node": 0}, ...]}`, and its `{"node": 0}` is the importer's index. The bullet's own earlier sentence says such segments name the importing graph. So a request or report about `example` that copies node 2's id as the package writes it would be resolved against the `test/predicates` graph. It is correct in the fixture only because Integer sits at index 0 in both graphs. FR-375-AC-35 keys on the key's `package`, not the node's, so the two rules also disagree for a domain-owned abstract declaration. The rule should name the graph by document context: the checked package the document is about, or the FR-375 key's `package`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1085-1099; spec/functional/analysis/FR-375-declare-model-refinement.md:421-431; proposals/checked-package-v2/fixtures/positive-dependency-call.json |
| FND-006 | medium | FR-322-AC-95 claims "each published positive fixture whose graph holds no `recursion_group`". The reworked test walks `positive_fixtures()`, which holds 11 such fixtures of the 18. It skips `positive-dependency-call`, `positive-integer-literal-references`, `positive-model-tuple-union`, `positive-tick-timed-model-nodes`, `positive-timed-clause-nodes`, `positive-timed-model-nodes` and `positive-timed-slot-nodes`. These include the imported-derived-node fixture and the largest graphs, so the AC's universal claim is still partly untested. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3278; tests/checked_package_v2.rs:8860,18099-18168 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Corpus `$defs/NodeId` now `$ref`s `urn:quire:checked-package:v2#/$defs/NodeId`; FR-336 states the shape and the graph it resolves against; `conformance_corpus_v1` admits `{"node": 2}` and refuses a nested id (passes at a538b816). |
| FND-002 | fixed | FR-375 now resolves an external key's index segments against the dependency its `package` names, adds an out-of-range `malformed_wire` refusal at `.../node/path/{s}/node`, and adds FR-375-AC-35 with TC-310 RW-09 (importing graph longer than `N`, so it discriminates). |
| FND-003 | fixed | FR-322:716 now reads "`node` has the `DeclaredNodeId` shape, a `node_id` whose `path` holds only strings". |
| FND-004 | fixed | The AC-95 test now runs `read_package_semantics` on each fixture as published, reversed and sorted ascending by structural key, with keys unchanged (passes at a538b816); the fixture-set gap is FND-006. |
| FND-005 | fixed | Round 2 (7da9fe7a): FR-322 Scope now resolves an index segment against the graph of the checked-package document the id belongs to and states that `node_id.package` selects no graph. It names the selecting member for each outside document (request `package`, `ContractPackage` source identity, corpus replay `package` or compiled `input`), and FR-375 keys on the key's own `package`. FR-322-AC-282/NI-14 discriminates: the library resolution would be a self-cycle. `tc_233_an_imported_node_id_resolves_against_the_importer_graph` passes. |
| FND-006 | fixed | Round 2 (7da9fe7a): the AC-95 test now reads all 13 non-recursive positive fixtures except the two timed fixtures, and asserts the count of 13. The dependency call is read with its library supplied. FR-322-AC-95 now names the two timed exclusions and the reason (TC-367 owns their checks), so the claim and the test agree. Measured: 15 fixtures have no recursion_group; minus 2 is 13. |
