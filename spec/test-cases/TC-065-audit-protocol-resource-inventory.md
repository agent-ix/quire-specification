---
id: TC-065
title: "Audit the complete protocol resource inventory"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-020
    type: verifies
---
## Description

Inspect the complete source-bound inventory of caller-controlled protocol
collections and traversals independently of their runtime boundary tests.

## Test Procedure

Build the inventory from every reviewed Rust request/result field and production
path that can allocate, retain, queue, iterate or traverse caller-controlled
protocol structure, roles, messages, branches, repetitions, active instances,
observations, result items or derived artifacts. For each source site, require
one unique inventory entry naming its owning interface, exact limit authority,
pre-work charge point, finite termination argument, typed resource-incomplete
code and TC-062 boundary case. Reconcile source sites to entries in both
directions. Delete each entry in turn, add one otherwise valid unlisted source
site to the audit fixture, duplicate an entry under another name and remove its
TC-062 binding.

## Expected Results

The complete inventory reports zero unbounded or unmatched source sites and
every entry names a discriminating TC-062 boundary case. Each deleted, unlisted,
duplicated or boundary-unbound mutation fails the inspection and identifies the
exact source site or inventory entry; no dynamic passing case can conceal an
omitted caller-controlled collection or traversal.
