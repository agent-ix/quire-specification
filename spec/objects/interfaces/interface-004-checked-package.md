---
id: interface-004
title: "I04 immutable checked package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# [interface-004] I04 immutable checked package

## Contract

```yaml
name: CheckedPackageArtifactV1
associated_types:
  - CheckedPackageId
  - CheckedSemanticGraphV1
  - PackageLockV1
  - SourceDocumentRef
  - DefinitionArtifactRef
  - ModelSelectionRef
  - SourceMapV1
  - CapabilityReportV1
  - DiagnosticReportV1
  - CheckedPackageReadLimits
operations:
  - { name: link_and_check, inputs: [ParsedSource, ModelSelection, profile bundle], output: CheckedPackageArtifactV1 or located refusals, semantics: resolves exact definitions types dependencies and capabilities }
  - { name: read, inputs: [canonical CheckedPackageArtifactV1 bytes, CheckedPackageReadLimits], output: immutable checked package or typed refusal/incomplete result, semantics: verifies the selected wire, closed structure, canonical bytes, domain-tagged identities and bounded graph before consumption }
  - { name: inspect, inputs: [CheckedPackageArtifactV1], output: immutable declarations capabilities and diagnostics, semantics: does not execute or lower }
invariants:
  - runtime inputs do not participate in static package identity
  - backend support does not alter admission
  - source bytes, definition bytes, compiled-model documents, checked semantic graph and generated IR occupy distinct typed identity and digest domains
  - the public semantic graph is a closed versioned union owned by this interface; consumers do not receive parser/CST/package implementation objects
  - a reader retains invalid unsupported incomplete and failed dispositions; it cannot replace one with an empty graph or a successful package
```

## Public boundary

`quire.checked-package/v1` is the only I04 interchange accepted by a consumer
outside the native producer. Its normative closed schema and independently
authored vectors are published under
[`proposals/checked-package-v1`](../../../proposals/checked-package-v1/). The
schema defines the transport form; the grammar and the linked normative
requirements retain authority for language meaning. This boundary deliberately
does not expose a parser CST, a filesystem path, a display string, or an
implementation-private native package structure.

The `read` operation is a strict, bounded admission operation. It verifies the
declared wire before graph interpretation and returns a typed refusal for an
unknown version, malformed or duplicate member, cross-domain substitution,
stale dependency, unknown capability, or graph/source-map invariant violation.
Exceeding a selected byte, depth, node, edge, locus, diagnostic, or work limit
returns an incomplete resource disposition and no admitted package.
