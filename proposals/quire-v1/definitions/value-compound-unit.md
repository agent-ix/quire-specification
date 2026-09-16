# Compound-unit value identity definition

Definition identity: `quire.value.compound-unit/v1`; revision: `1-draft.1`.
This evaluator-owned value identity is selected by `quire.value.complete/v1`.
It is not an I04 checked-semantic node and has no source occurrence.

The normative preimage shape is
[value-compound-unit.schema.json](value-compound-unit.schema.json), SHA-256
`59d9d01073511d12211447837a560eec18489fa462b8dc9b4a14e4dd0b8b3134`.
The normative examples and mutations are
[value-compound-unit-vectors.json](value-compound-unit-vectors.json), SHA-256
`f12a662b58239a29e87625b59855cd9e351b1e61ee639d84956276cb7c0deecd`.
Hashes use the `quire.definition.bytes/v1` raw-byte domain without rewriting.

A value identity is SHA-256 over RFC 8785 JCS of the schema-valid preimage.
Terms reference admitted canonical root-unit I04 node IDs, use nonzero
mathematical integer exponents, and are strictly ascending by canonical node
key. Duplicate terms refuse internal construction. The empty list is the sole
dimensionless identity. An evaluator constructs only the canonical preimage and
does not accept a caller-supplied digest assertion.
