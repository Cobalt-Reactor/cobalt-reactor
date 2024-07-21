pub mod shared;
use reactor_serial::prelude::*;
use shared::*;

#[test]
fn save_format_can_from_previous() {
    let v1 = TestSaveFormatV1 {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2 {
        name: "test".to_string(),
        value: 0,
    };
    let v3 = TestSaveFormatV3 {
        name: "test".to_string(),
        value: 0.0,
    };
    assert_eq!(TestSaveFormatV3::from_previous(&v2), Some(v3));
    assert_eq!(TestSaveFormatV2::from_previous(&v1), Some(v2));
}

#[test]
fn errors_on_bad_convert_type() {
    let v1 = TestSaveFormatV1 {
        name: "test".to_string(),
    };

    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1::VERSION;

    let result = convert_save::<TestSaveFormatV2Json>(v1_version, "test", v1_data.as_bytes());

    assert!(result.is_err());
}

#[test]
fn save_format_can_convert_ron() {
    let v1 = TestSaveFormatV1 {
        name: "test".to_string(),
    };
    let v3 = TestSaveFormatV3 {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1::VERSION;

    let result = convert_save::<TestSaveFormatV3>(v1_version, "test", v1_data.as_bytes());

    assert_eq!(result.unwrap(), v3);
}

#[test]
fn save_format_can_convert_json() {
    let v1 = TestSaveFormatV1Json {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2Json {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = serde_json::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1Json::VERSION;

    let result = convert_save::<TestSaveFormatV2Json>(v1_version, "test", v1_data.as_bytes());

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn save_format_can_convert_yaml() {
    let v1 = TestSaveFormatV1Yaml {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2Yaml {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = serde_yaml::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1Yaml::VERSION;

    let result = convert_save::<TestSaveFormatV2Yaml>(v1_version, "test", v1_data.as_bytes());

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn save_format_can_convert_toml() {
    let v1 = TestSaveFormatV1Toml {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2Toml {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = toml::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1Toml::VERSION;

    let result = convert_save::<TestSaveFormatV2Toml>(v1_version, "test", v1_data.as_bytes());

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn save_format_can_convert_xml() {
    let v1 = TestSaveFormatV1Xml {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2Xml {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = quick_xml::se::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1Xml::VERSION;

    let result = convert_save::<TestSaveFormatV2Xml>(v1_version, "test", v1_data.as_bytes());

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn save_format_can_convert_csv() {
    let v1 = TestSaveFormatV1Csv {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2Csv {
        name: "test".to_string(),
        value: 0.0,
    };

    let mut builder = csv::WriterBuilder::new().from_writer(Vec::new());
    builder.serialize(&v1).unwrap();
    // Should never fail because we're writing to a vec not a file
    let v1_data = builder.into_inner().unwrap();
    let v1_version = TestSaveFormatV1Csv::VERSION;

    let result = convert_save::<TestSaveFormatV2Csv>(v1_version, "test", &v1_data);

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn save_format_can_convert_msgpack() {
    let v1 = TestSaveFormatV1MsgPack {
        name: "test".to_string(),
    };
    let v2 = TestSaveFormatV2MsgPack {
        name: "test".to_string(),
        value: 0.0,
    };

    let v1_data = rmp_serde::to_vec(&v1).unwrap();
    let v1_version = TestSaveFormatV1MsgPack::VERSION;

    let result = convert_save::<TestSaveFormatV2MsgPack>(v1_version, "test", &v1_data);

    assert_eq!(result.unwrap(), v2);
}

#[test]
fn cyclical_reference() {
    let v1 = CyclicalReferenceV1;
    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = CyclicalReferenceV1::VERSION;

    let result = convert_save::<CyclicalReferenceV2>(v1_version, "test", v1_data.as_bytes());

    assert!(result.is_err());
}

#[test]
fn matching_versions() {
    let v1 = MatchingSaveVersionsV1;
    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = MatchingSaveVersionsV1::VERSION;

    let result = convert_save::<MatchingSaveVersionsV2>(v1_version, "test", v1_data.as_bytes());

    assert!(result.is_err());
}

#[test]
fn decrementing_versions() {
    let v1 = DecrementedSaveVersionsV1;
    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = DecrementedSaveVersionsV1::VERSION;

    let result = convert_save::<DecrementedSaveVersionsV2>(v1_version, "test", v1_data.as_bytes());

    assert!(result.is_err());
}

#[test]
fn bad_format() {
    let v1 = BadFormat { bad: -1 };
    let v1_data = ron::to_string(&v1).unwrap();
    let v1_version = TestSaveFormatV1Toml::VERSION;

    let result = convert_save::<TestSaveFormatV3>(v1_version, "test", v1_data.as_bytes());

    assert!(result.is_err());
}
