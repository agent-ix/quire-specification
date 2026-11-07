// SPDX-License-Identifier: AGPL-3.0-or-later
//! Case-preserving identities in the authored model fixture (FR-154).

use ix_trace_rs::trace;
use quire_specification::ACME_COLORS_DOMAIN_PACKAGE;
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

/// The standalone comparison model is a complete independent declaration
/// document with exactly the two nullary cases its native source names.
#[trace("TC-234", "FR-154-AC-11")]
#[test]
fn colors_comparison_model_exposes_two_original_nullary_cases() {
    let document: Value =
        serde_json::from_str(ACME_COLORS_DOMAIN_PACKAGE).expect("the authored enum model JSON");
    assert_eq!(document["contractVersion"], "2.0.0");
    assert_eq!(document["package"]["identity"], "acme/colors");
    assert_eq!(document["source"]["identity"], "ix://acme/colors/spec");
    assert_eq!(
        document["canonicalization"],
        json!({"algorithm": "rfc8785-v1"}),
    );
    let constructs = document["constructs"]
        .as_array()
        .expect("the construct declaration");
    assert_eq!(constructs.len(), 1);
    assert_eq!(constructs[0]["construct"]["shape"], "enumeration");
    assert_eq!(
        constructs[0]["construct"]["meaning"],
        "quire.meaning.model.variant-type/v1",
    );
    let types = document["types"]
        .as_array()
        .expect("the original declarations");
    assert_eq!(types.len(), 1, "the comparison model declares only Color");
    let color = &types[0];
    assert_eq!(color["identity"], "ix://acme/colors/Color");
    assert_eq!(color["kind"], constructs[0]["kind"]);
    let cases = color["variants"].as_array().expect("the Color cases");
    assert_eq!(cases.len(), 2);
    for (name, identity) in [
        ("Red", "ix://acme/colors/variant/Color-Red"),
        ("Green", "ix://acme/colors/variant/Color-Green"),
    ] {
        let case = cases
            .iter()
            .find(|case| case["name"] == name)
            .expect("the named case");
        assert_eq!(case["identity"], identity);
        assert!(case.get("payloadType").is_none(), "{name} remains nullary");
        assert_eq!(case["origin"], color["origin"]);
        assert_eq!(
            case["origin"]["source"]["sourceIdentity"],
            document["source"]["identity"]
        );
    }
}
