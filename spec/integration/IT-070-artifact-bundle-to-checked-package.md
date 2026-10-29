---
id: IT-070
title: "Artifact bundle to checked package"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-150
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-152
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
---

# IT-070: Artifact bundle to checked package

> BLOCKED-BY QSpec #517: this file still cites the removed `sha256-jcs` / `quire.verification.jcs` identity domain (FR-201); delete that wording when #517 lands.

## Objective

Verify that a spec's artifact bundle becomes a checked package through the real
chain: quire-rs extraction, FCD extraction-frontend lift, domain package, QSL
model intake and linking. The chain must agree on declaration keys, effective
declarations, systems-model relations, identity domains and refusal behavior.

## Target Integration

The quire-rs semantic extraction API reads an artifact bundle. The FCD
extraction-frontend lifts the result into a domain package. QSL
model intake, normalization and package linking consume that domain package in
one local process.

## Preconditions

quire-rs, FCD and quire-spec-language build locally with their published Rust interfaces; `resources/native-v1/` is
read-only.

## Inputs

An artifact bundle declaring object types with inheritance and redefinition,
parts, ports, a connection and an allocation; plus digest-domain, stale-digest,
malformed-node, dangling-reference, cycle and ambiguity mutants of that bundle
or its domain package.

## Test Procedure

1. Extract and lift the valid bundle, then admit its domain package.
   - IT-070-SC-01: every IR node yields one original declaration keyed by
     package identity and IR node identity, and every effective declaration and
     relation matches.
2. Link a complete native package against the admitted model view.
   - IT-070-SC-02: all typed references resolve with exact provenance, and the
     checked package's `model_root` nodes name the domain package by identity
     and `sha256-jcs` digest.
3. Run each mutant through the same chain.
   - IT-070-SC-03: each refuses at its exact identity, rule or locus, and each
     declaration refusal retains the node's IR identity and `origin`.

## Expected Results

The valid bundle links reproducibly; no mutant is repaired by local-name,
shape, digest-domain or ambient lookup substitution.
