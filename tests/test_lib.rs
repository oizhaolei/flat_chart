use flat_chart::{parse_mapping_file, Mapping};

#[test]
fn test_mapping_file() {
    let mapping = parse_mapping_file("./tests/mapping.toml");
    assert_eq!(
        Mapping {
            fixed_header: vec![
                "患者番号".to_string(),
                "カナ姓".to_string(),
                "カナ名".to_string(),
                "漢字姓".to_string(),
                "漢字名".to_string(),
                "性別".to_string(),
                "生年月日".to_string(),
                "死亡日".to_string(),
                "採取日".to_string(),
            ],
            flat_item_key: "検査項目名称".to_string(),
            flat_item_value: "検査結果値".to_string(),
        },
        mapping
    );
}
