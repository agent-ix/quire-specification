---
id: FR-134
title: "Preserve exact loci through source transformations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-003
    type: references
  - target: ix://agent-ix/quire-specification/FR-036
    type: references
---

# FR-134: Preserve exact loci through source transformations

## Description

When extracting or transforming native source, the tool SHALL map every
retained token and diagnostic locus to the original immutable byte range
without normalizing authored bytes.

## Inputs

Original source bytes, source revision and explicit extraction map.

## Outputs

Lossless token/source correspondence or a typed invalid-map refusal.

## Behavior

Lexing performs no newline conversion. A container extractor may omit only
declared container-owned prefix/suffix or indentation bytes and records each
deletion as a monotone mapping segment. CRLF and final-newline bytes remain
authored bytes unless the container contract proves they are container-owned.
Analysis, tooling, plugin and workflow diagnostics use source spans when a
source occurrence exists and typed runtime paths otherwise.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-134-AC-1 | CRLF, LF and absent-final-newline inputs retain exact original spans through parsing and formatting. | Test (TC-184) |
| FR-134-AC-2 | Declared container-owned indentation deletion maps every retained token monotonically to its original bytes. | Test (TC-184) |
| FR-134-AC-3 | An undeclared deletion, reordered segment or mismatched source revision refuses before a derived artifact is published. | Test (TC-184) |
| FR-134-AC-4 | A declaration carried through linking and lowering reaches every checked node, IR node and result with the same original span it had after parsing; a synthesized node names the authored span it derives from. | Test (TC-184) |
| FR-134-AC-5 | A diagnostic, counterexample or output-mapping record raised at any stage after parsing reports the original source span rather than a derived or reconstructed position. | Test (TC-184) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
