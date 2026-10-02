# Compound-unit value identity definition

Definition identity: `quire.value.compound-unit/v1`.
This evaluator-owned value identity is selected by `quire.value.complete/v1`.
It is not an I04 checked-semantic node and has no source occurrence.

The normative preimage shape is
[value-compound-unit.schema.json](value-compound-unit.schema.json).
The normative examples and mutations are
[value-compound-unit-vectors.json](value-compound-unit-vectors.json).

A value identity is SHA-256 over RFC 8785 JCS of the schema-valid preimage.
Terms reference admitted canonical root-unit I04 node IDs, use nonzero
mathematical integer exponents written as canonical decimal strings (never
`-0`), and are strictly ascending by canonical node
key. Duplicate terms refuse internal construction. The empty list is the sole
dimensionless identity. An evaluator constructs only the canonical preimage and
does not accept a caller-supplied digest assertion.
