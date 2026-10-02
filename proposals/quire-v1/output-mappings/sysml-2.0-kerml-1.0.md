# SysML 2.0 / KerML 1.0 output profile

Selected identity `quire.output.sysml2-kerml1/v1`.
Targets OMG SysML 2.0 Language `formal/26-03-02` and its KerML 1.0 foundation
`formal/26-03-01` as one inseparable profile selection.

## Admitted fragment

The profile generates a SysML `requirement def` with exact references or local
typed attributes corresponding to model declarations from spec artifacts and a
`require constraint { ... }` expression. The expression fragment contains total
Boolean literals/connectives, equality and ordered comparison of equal types,
bounded integer literals and checked `+`/`-`/`*`, and exact attribute references.

The generated requirement retains the native requirement/clause identity in its
manifest and a non-authoritative display-safe short name in text. The display
name is never the native identity. Any generated local attribute is a target
binding to a spec-declared model declaration, never a new model declaration.

## Conditional and unsupported meaning

- Scalar expression preservation is conditional on an exact authoritative
  native-to-SysML/KerML type, unit, attribute and library correspondence.
- Requirement assumptions and required constraints remain distinct. A native
  implication is not split into `assume`/`require` unless that exact structural
  correspondence is selected; otherwise it stays a Boolean expression or is
  unrepresented.
- Pre/post operation anchors, collection queries, object/relationship traversal,
  protocol execution, temporal/history semantics and target library units with
  no exact source correspondence are unrepresented by this profile.
- Parts, ports, actions, state machines, allocations and model interchange are
  outside this expression profile. The exporter does not generate a replacement
  system model or infer a feature from a display name.

## Output and qualification boundary

Generated text is output-only and does not claim full SysML/KerML conformance.
The normative standard revisions are distinct from any pilot implementation or
library bundle. Parser/typechecker acceptance is structural evidence only and
requires a separately qualified exact tool/library/license selection.
