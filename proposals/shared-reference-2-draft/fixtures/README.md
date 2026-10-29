# Qualification fixtures

These are independently authored synthetic inputs. roles.json supplies all six
new exact references and actual payload files. semantic.json selects the
profile.
adverse.json specifies one-axis substitutions with expected structural and
semantic layers. raw-duplicate.json must fail strict raw intake as invalid-wire.

The Rust qualification tests validate schema shape and the located fixture
files. They deliberately do not implement expected-role matching, semantic/profile
adoption, feature support, immutable identity joins or B's report assembler.
Those remain IT-003-SC-03. A nearby role can be structurally valid and still
be a required semantic refusal; adverse.json preserves that distinction.

An initial test incorrectly assumed B's existing parse_bounded_json already
rejected duplicate decoded keys. The actual test failed: the helper accepted
the two decoded a keys as one map entry. The producer test was scoped back to its actual schema
boundary; raw-duplicate.json remains a required, unexecuted strict-reader
control. No duplicate-key intake qualification or complete reader is claimed.
