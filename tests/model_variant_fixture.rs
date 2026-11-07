// SPDX-License-Identifier: AGPL-3.0-or-later
//! Case-preserving identities in the authored model fixture (FR-154).

use ix_trace_rs::trace;
use serde_json::{Value, json};

const SHAPES_DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-shapes.json");

/// Case names, origins and payloads remain unchanged when their minted slots
/// preserve the declared spelling.
#[trace("TC-234", "FR-154-AC-11")]
#[test]
fn shapes_variant_case_identities_preserve_declared_case() {
    let document: Value = serde_json::from_str(SHAPES_DOCUMENT).expect("the authored model JSON");
    assert_eq!(
        document["canonicalization"],
        json!({"algorithm": "rfc8785-v1"}),
        "the shared Semantic IR reader's canonicalization descriptor is present",
    );
    let types = document["types"]
        .as_array()
        .expect("the document's type definitions");
    let expected = [
        (
            "Shape",
            "Flag",
            "ix://acme/shapes/variant/Shape-Flag",
            Some("ix://quire/native/Boolean"),
        ),
        (
            "Shape",
            "Empty",
            "ix://acme/shapes/variant/Shape-Empty",
            None,
        ),
        (
            "Shape",
            "Both",
            "ix://acme/shapes/variant/Shape-Both",
            Some("ix://acme/shapes/Pair"),
        ),
        ("Color", "Red", "ix://acme/shapes/variant/Color-Red", None),
        (
            "Color",
            "Green",
            "ix://acme/shapes/variant/Color-Green",
            None,
        ),
        ("List", "Nil", "ix://acme/shapes/variant/List-Nil", None),
        (
            "List",
            "Cons",
            "ix://acme/shapes/variant/List-Cons",
            Some("ix://acme/shapes/Cell"),
        ),
    ];
    assert_eq!(
        types
            .iter()
            .filter_map(|declared| declared["variants"].as_array())
            .map(Vec::len)
            .sum::<usize>(),
        expected.len(),
        "every published variant case is checked",
    );
    for (artifact, name, identity, payload) in expected {
        let owner_identity = format!("ix://acme/shapes/{artifact}");
        let owner = types
            .iter()
            .find(|declared| declared["identity"] == owner_identity)
            .unwrap_or_else(|| panic!("the {artifact} declaration"));
        let case = owner["variants"]
            .as_array()
            .expect("the declaration's cases")
            .iter()
            .find(|case| case["name"] == name)
            .unwrap_or_else(|| panic!("the {artifact} case {name}"));
        assert_eq!(case["identity"], identity, "the minted slot for {name}");
        assert_eq!(
            case["origin"], owner["origin"],
            "the source origin of {name}"
        );
        let expected_payload = payload.map(Value::from);
        assert_eq!(
            case.get("payloadType"),
            expected_payload.as_ref(),
            "the payload of {name}"
        );
    }
}
