# Qualification fixtures

These are independently authored synthetic inputs. roles.json supplies all six
new exact references and actual payload files. semantic.json selects the
profile and the JCS canonicalization of the core.
canonical-vectors.json records hand-authored canonical UTF-8 expectations.
adverse.json specifies one-axis substitutions with expected structural and
semantic layers. raw-duplicate.json must fail strict raw intake as invalid-wire.

No fixture stores a digest. The tests derive the core's JCS identity at run
time from core.json and compare run-time-derived values with each other.

The Rust qualification tests call B's existing JCS functions. They validate
schema shape and exact bytes and exercise the selected canonical preimages.
They deliberately do not implement expected-role matching, semantic/profile
adoption, feature support, immutable identity joins or B's report assembler.
Those remain IT-003-SC-03. A nearby role can be structurally valid and still
be a required semantic refusal; adverse.json preserves that distinction.

An initial test incorrectly assumed B's existing parse_bounded_json already
rejected duplicate decoded keys. The actual test failed: the helper accepted
the two decoded a keys as one map entry. The producer test was scoped back to its actual schema/JCS
boundary; raw-duplicate.json remains a required, unexecuted strict-reader
control. No duplicate-key intake qualification or complete reader is claimed.

The synthetic core schema and disposition demonstrate correspondence only.
They do not define B's VP04 report shape, contain a real human disposition or
prove review independence. B must run the same rule through its actual accepted
report schema and assembler for complete consumer qualification.
