---
id: TC-147
title: "Audit the observation executable boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-052
    type: verifies
---
## Description

Accepted FS04 portable static control for
[NFR-052](../non-functional/NFR-052-contain-observation-executables.md). Implementation-specific execution and evidence stay separate.

## Test Procedure

Inventory every owned source, generated executable, test, build definition,
workflow, CLI, adapter, reader, loss reporter, and fixture-audit path for the
selected observation scope. Record its language and purpose. Independently
introduce a forbidden foreign runtime reference into fixture controls.

## Expected Results

Every owned first-party executable path is Rust. Java, JVM, Maven,
Eclipse, Electron, and foreign monitoring runtime references are refused for
the selected path. Foreign output text and non-executable fixture data remain
distinguished from executable dependencies. Existing retained Quoin tooling does
not exempt a newly owned path. No hosted workflow is dispatched.
