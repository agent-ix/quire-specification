# Qualification fixtures

These are independently authored synthetic inputs. roles.json supplies all six
new exact references and actual payload files. semantic.json separately binds
the pretty-printed core bytes, profile-definition bytes and JCS core identity.
canonical-vectors.json records hand-authored canonical UTF-8 expectations.
adverse.json specifies one-axis substitutions with expected structural and
semantic layers. raw-duplicate.json must fail strict raw intake as invalid-wire.

The core's exact byte digest is
sha256:05f795d53cfa663f7740893165d956087a67d7f17217c761b446a1a73226d682.
Its canonical semantic digest is
sha256-jcs:70a89fec8afa6a435d5a92f63ba9ce1d01e86b17791629bb788628ee7bfb81d5.
The latter hashes exactly the UTF-8 contents of the core vector's canonical
string, with no newline. The textual domain prefix is not hashed.

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

The synthetic core schema and disposition demonstrate hash correspondence only.
They do not define B's VP04 report shape, contain a real human disposition or
prove review independence. B must run the same rule through its actual accepted
report schema and assembler for complete consumer qualification.
