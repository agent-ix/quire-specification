---
id: interface_004
title: "I04 immutable checked package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# [interface_004] I04 immutable checked package

## Contract

```yaml
name: CheckedPackageArtifactV2
associated_types:
  - CheckedPackageId
  - CheckedSemanticGraphV2
  - CheckedSemanticNodePreimageV1
  - PackageLockV2
  - SourceDocumentRef
  - DefinitionArtifactRef
  - ModelSelectionRef
  - SourceMapV2
  - CapabilityReportV2
  - DiagnosticReportV2
  - CheckedPackageReadLimits
operations:
  - { name: link_and_check, inputs: [ParsedSource, ModelSelection, profile bundle], output: CheckedPackageArtifactV2 or located refusals, semantics: resolves exact definitions types dependencies nominal identities and capabilities }
  - { name: read, inputs: [canonical CheckedPackageArtifactV2 bytes, CheckedPackageReadLimits], output: immutable checked package or typed refusal/incomplete result, semantics: verifies the selected wire, closed structure, canonical bytes, domain-tagged identities and bounded graph before consumption }
  - { name: inspect, inputs: [CheckedPackageArtifactV2], output: immutable declarations capabilities and diagnostics, semantics: does not execute or lower }
invariants:
  - runtime inputs do not participate in static package identity
  - backend support does not alter admission
  - source bytes, definition bytes, domain packages, checked semantic graph and generated IR occupy distinct typed identity and digest domains
  - the public semantic graph is a closed versioned union owned by this interface; consumers do not receive parser/CST/package implementation objects
  - complete-V1 nominal scalar node keys use the closed node-identity preimages and known vectors published with the I04 schema
  - a reader retains invalid unsupported incomplete and failed dispositions; it cannot replace one with an empty graph or a successful package
```

## Public boundary

`quire.checked-package/v2` is the complete-V1 I04 interchange emitted for new
packages. Its normative closed schema and independently authored vectors are
published under
[`proposals/checked-package-v2`](../../../proposals/checked-package-v2/). The
schema defines the transport form; the grammar and the linked normative
requirements retain authority for language meaning. This boundary deliberately
does not expose a parser CST, a filesystem path, a display string, or an
implementation-private native package structure.

The adjacent `node-identity-preimage.schema.json` and
`node-identity-vectors.json` are normative parts of this version. They close the
JCS preimages for enum declarations/members, dimensions and declared units
without making a whole package identity part of a nominal type identity.

The `read` operation is a strict, bounded admission operation. It verifies the
declared wire before graph interpretation and returns a typed refusal for an
unknown version, malformed or duplicate member, cross-domain substitution,
stale dependency, unknown capability, or graph/source-map invariant violation.
Exceeding a selected byte, depth, node, edge, locus, diagnostic, or work limit
returns an incomplete resource disposition and no admitted package.
