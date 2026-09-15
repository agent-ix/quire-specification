---
id: IT-070
title: "Authoritative model export links into a native package"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-150
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-152
    type: verifies
---

# IT-070: Authoritative model export links into a native package

## Objective

Verify that the exact Filament Rust producer contract and QSL Rust linker agree
on original/effective declarations, systems-model relations, identity domains
and refusal behavior without editing producer resources.

## Target Integration

The real pinned Filament model-export Rust API/fixtures feed QSL's model
correspondence and package-linking APIs in one local process.

## Preconditions

Both repositories are pinned by one ecosystem lock and build locally with their
published Rust interfaces; `resources/native-v1/` is read-only.

## Inputs

Valid inheritance/redefinition/part-port-connection exports plus digest-domain,
closure, cycle and ambiguity mutants.

## Test Procedure

1. Produce and bind the valid exact export.
   - IT-070-SC-01: every original/effective declaration and relation matches.
2. Link a complete native package against the bound view.
   - IT-070-SC-02: all typed references resolve with exact provenance.
3. Run each mutant through the same boundary.
   - IT-070-SC-03: each refuses at its exact identity/closure/rule locus.

## Expected Results

The valid package links reproducibly; no mutant is repaired by local-name,
shape, digest-domain or ambient lookup substitution.
