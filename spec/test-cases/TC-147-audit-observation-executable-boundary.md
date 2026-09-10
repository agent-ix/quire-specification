---
id: TC-147
title: "Audit the observation executable boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-052
    type: verifies
---
## Description

Planned static control for
[NFR-052](../non-functional/NFR-052-contain-observation-executables.md). No
observation implementation is yet claimed; this control becomes applicable before any first-party observation
binding, replay, incremental assessment, output adapter, fixture audit, or
qualification executable is admitted.

## Test Procedure

Inventory every owned source, generated executable, test, build definition,
workflow, CLI, adapter, reader, loss reporter, and fixture-audit path for the
selected observation scope. Record its language, toolchain, dependencies, and
purpose. Independently introduce a forbidden foreign runtime reference and a
non-1.98.1 Rust toolchain reference into fixture controls.

## Expected Results

Every owned first-party executable path is Rust 1.98.1. Java, JVM, Maven,
Eclipse, Electron, and foreign monitoring runtime references are refused for
the selected path. Foreign output text and non-executable fixture data remain
distinguished from executable dependencies. No hosted workflow is dispatched.
