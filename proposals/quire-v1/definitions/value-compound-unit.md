# Compound-unit value identity definition

Definition identity: `quire.value.compound-unit/v1`.
This evaluator-owned value identity is selected by `quire.value.complete/v1`.
This value identity is not an I04 checked-semantic node key and has no source
occurrence. The `scalar_type`/`compound_unit` node that a checked source type
forms is a separate I04 node, keyed by FR-322's structural node preimage; its
terms follow this identity's term order, and neither identity is derived from
the other.

The normative preimage shape is
[value-compound-unit.schema.json](value-compound-unit.schema.json).
The normative examples and mutations are
[value-compound-unit-vectors.json](value-compound-unit-vectors.json).

A value identity is the schema-valid preimage itself, compared componentwise (terms in order, each term's node id and exponent); nothing is hashed.
Terms reference admitted canonical root-unit I04 node IDs, use nonzero
mathematical integer exponents written as canonical decimal strings (never
`-0`), and are strictly ascending by canonical node
key. Duplicate terms refuse internal construction. The empty list is the sole
dimensionless identity. An evaluator constructs only the canonical preimage and
does not accept a caller-supplied identity assertion.
