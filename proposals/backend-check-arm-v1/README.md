# Check-arm vectors `backend-check-arm-v1`

End-to-end vectors for the check arm of `value-validity` and
`operation-contract` items
([FR-483](../../spec/functional/ir-proof/FR-483-settle-backend-items-at-the-check-arm.md),
[FR-484](../../spec/functional/ir-proof/FR-484-check-value-validity-items.md),
[FR-485](../../spec/functional/ir-proof/FR-485-check-operation-contract-items.md)).

Each case in `vectors.json` gives a registry of descriptors, one request
item, the FR-290 negotiation disposition, the arm outcome and result when the
item reaches the arm, and the FR-331 outcome stage the provider writes. The
outcome stage validates against `../backend-provider-v1/schema.json`; the
operation-contract counterexample is its `ContractCounterexample`.

Node ids are written `$node:<label>`. The harness derives each NodeId as the
SHA-256 over the domain, a zero byte and the label, so the file holds no digest
and a change of the node-identity scheme changes no vector. The harness is
`tests/backend_check_arm_v1.rs`.
