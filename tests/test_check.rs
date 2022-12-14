use flat_chart::{
    check::{all_item_keys, parse_csv, parse_xlsx, transform, write_csv},
    parse_mapping_file, Mapping,
};

use std::collections::HashMap;

#[test]
fn test_csv_reader() {
    let mapping: Mapping = parse_mapping_file("./tests/mapping.toml");
    let data = parse_csv("data/check.csv", mapping);
    assert!(data.is_ok());
    let lines = data.unwrap();
    assert_eq!(lines.len(), 25);
}

#[test]
fn test_transform() {
    let checks: Vec<HashMap<String, String>> = vec![
        HashMap::from([
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査項目名称".to_string(), "  蛋白".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査項目コード".to_string(), "600290".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("採取日".to_string(), "2008/09/05".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査結果値".to_string(), "100(2+)".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("性別".to_string(), "女".to_string()),
        ]),
        HashMap::from([
            ("カナ名".to_string(), "ｾ".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("採取日".to_string(), "2008/09/16".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査項目コード".to_string(), "600290".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "  蛋白".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("検査結果値".to_string(), "（±）".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
        ]),
        HashMap::from([
            ("検査材料名称".to_string(), "000".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("検査項目コード".to_string(), "600290".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査結果値".to_string(), "（±）".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査項目名称".to_string(), "  蛋白".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("採取日".to_string(), "2008/09/30".to_string()),
        ]),
        HashMap::from([
            ("性別".to_string(), "女".to_string()),
            ("検査結果値".to_string(), "（－）".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("採取日".to_string(), "2008/11/07".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査項目名称".to_string(), "  蛋白".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査項目コード".to_string(), "600290".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
        ]),
        HashMap::from([
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("検査結果値".to_string(), "（１＋）".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("採取日".to_string(), "2008/09/05".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "  ケトン体".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査項目コード".to_string(), "600292".to_string()),
        ]),
        HashMap::from([
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("採取日".to_string(), "2008/09/16".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査結果値".to_string(), "（－）".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査項目名称".to_string(), "  ケトン体".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査項目コード".to_string(), "600292".to_string()),
            ("死亡日".to_string(), "".to_string()),
        ]),
        HashMap::from([
            ("カナ名".to_string(), "ｾ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査結果値".to_string(), "（－）".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("採取日".to_string(), "2008/09/30".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "  ケトン体".to_string()),
            ("検査項目コード".to_string(), "600292".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
        ]),
        HashMap::from([
            ("検査項目名称".to_string(), "  ケトン体".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査結果値".to_string(), "（－）".to_string()),
            ("検査項目コード".to_string(), "600292".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("報告書コード".to_string(), "06".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("報告書名称".to_string(), "尿定性".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("採取日".to_string(), "2008/11/07".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
        ]),
        HashMap::from([
            ("基準値外マーク".to_string(), "".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("採取日".to_string(), "2008/09/05".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査項目名称".to_string(), "ｱﾙﾌ\u{ff9e}ﾐﾝ".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査項目コード".to_string(), "600002".to_string()),
            ("検査結果値".to_string(), "4.7".to_string()),
        ]),
        HashMap::from([
            ("検査結果値".to_string(), "1.22".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("採取日".to_string(), "2008/09/05".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
        ]),
        HashMap::from([
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("採取日".to_string(), "2008/09/06".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査結果値".to_string(), "1.32".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
        ]),
        HashMap::from([
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("採取日".to_string(), "2008/09/08".to_string()),
            ("検査結果値".to_string(), "1.08".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
        ]),
        HashMap::from([
            ("漢字姓".to_string(), "中".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査結果値".to_string(), "0.94".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("採取日".to_string(), "2008/09/10".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
        ]),
        HashMap::from([
            ("緊急区分".to_string(), "至急".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("採取日".to_string(), "2008/09/16".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査結果値".to_string(), "0.86".to_string()),
        ]),
        HashMap::from([
            ("検査依頼科名称".to_string(), "消化器内科".to_string()),
            ("基準値外マーク".to_string(), "H".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査項目コード".to_string(), "600042".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査項目名称".to_string(), "ｸﾚｱﾁﾆﾝ".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼科コード".to_string(), "06".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("採取日".to_string(), "2019/02/25".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査結果値".to_string(), "0.90".to_string()),
        ]),
        HashMap::from([
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("採取日".to_string(), "2008/09/05".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査結果値".to_string(), "99".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
        ]),
        HashMap::from([
            ("報告書名称".to_string(), "生化学".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("検査結果値".to_string(), "101".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("採取日".to_string(), "2008/09/06".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
        ]),
        HashMap::from([
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査結果値".to_string(), "101".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("採取日".to_string(), "2008/09/08".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
        ]),
        HashMap::from([
            ("採取日".to_string(), "2008/09/10".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("検査結果値".to_string(), "103".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
        ]),
        HashMap::from([
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("検査依頼科コード".to_string(), "21".to_string()),
            ("検査依頼科名称".to_string(), "泌尿器科".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("採取日".to_string(), "2008/09/16".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("報告書コード".to_string(), "01".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("検査結果値".to_string(), "103".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
        ]),
        HashMap::from([
            ("報告書コード".to_string(), "01".to_string()),
            ("性別".to_string(), "女".to_string()),
            ("緊急区分".to_string(), "至急".to_string()),
            ("漢字名".to_string(), "つせ".to_string()),
            ("検査項目コード".to_string(), "600046".to_string()),
            ("基準値外マーク".to_string(), "".to_string()),
            ("検査依頼科名称".to_string(), "消化器内科".to_string()),
            ("検査材料コード".to_string(), "000".to_string()),
            ("カナ名".to_string(), "ｾ".to_string()),
            ("患者番号".to_string(), "DUMMY034".to_string()),
            ("採取日".to_string(), "2019/02/25".to_string()),
            ("検査材料名称".to_string(), "000".to_string()),
            ("検査依頼科コード".to_string(), "06".to_string()),
            ("報告書名称".to_string(), "生化学".to_string()),
            ("検査項目名称".to_string(), "ｸﾛｰﾙ".to_string()),
            ("漢字姓".to_string(), "中".to_string()),
            ("検査結果値".to_string(), "100".to_string()),
            ("死亡日".to_string(), "".to_string()),
            ("検査依頼入外区分".to_string(), "外来".to_string()),
            ("カナ姓".to_string(), "ﾅﾅｶ".to_string()),
            ("生年月日".to_string(), "1911/01/01".to_string()),
        ]),
    ];
    let mapping: Mapping = parse_mapping_file("./tests/mapping.toml");

    let item_keys = all_item_keys(checks.clone(), mapping.clone());
    let flatted_map = transform(checks, mapping.clone());

    assert_eq!(
        flatted_map,
        HashMap::from([
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/06".to_string(),
                ],
                HashMap::from([
                    ("ｸﾛｰﾙ".to_string(), "101".to_string()),
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "1.32".to_string())
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/05".to_string(),
                ],
                HashMap::from([
                    ("ｸﾛｰﾙ".to_string(), "99".to_string()),
                    ("  蛋白".to_string(), "100(2+)".to_string()),
                    ("ｱﾙﾌ\u{ff9e}ﾐﾝ".to_string(), "4.7".to_string()),
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "1.22".to_string()),
                    ("  ケトン体".to_string(), "（１＋）".to_string()),
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/30".to_string(),
                ],
                HashMap::from([
                    ("  蛋白".to_string(), "（±）".to_string()),
                    ("  ケトン体".to_string(), "（－）".to_string())
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/16".to_string(),
                ],
                HashMap::from([
                    ("  ケトン体".to_string(), "（－）".to_string()),
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "0.86".to_string()),
                    ("  蛋白".to_string(), "（±）".to_string()),
                    ("ｸﾛｰﾙ".to_string(), "103".to_string()),
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2019/02/25".to_string(),
                ],
                HashMap::from([
                    ("ｸﾛｰﾙ".to_string(), "100".to_string()),
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "0.90".to_string())
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/10".to_string(),
                ],
                HashMap::from([
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "0.94".to_string()),
                    ("ｸﾛｰﾙ".to_string(), "103".to_string())
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/11/07".to_string(),
                ],
                HashMap::from([
                    ("  蛋白".to_string(), "（－）".to_string()),
                    ("  ケトン体".to_string(), "（－）".to_string())
                ]),
            ),
            (
                vec![
                    "DUMMY034".to_string(),
                    "ﾅﾅｶ".to_string(),
                    "ｾ".to_string(),
                    "中".to_string(),
                    "つせ".to_string(),
                    "女".to_string(),
                    "1911/01/01".to_string(),
                    "".to_string(),
                    "2008/09/08".to_string(),
                ],
                HashMap::from([
                    ("ｸﾚｱﾁﾆﾝ".to_string(), "1.08".to_string()),
                    ("ｸﾛｰﾙ".to_string(), "101".to_string())
                ]),
            ),
        ])
    );

    let write_to = "foo.csv";
    let result = write_csv(write_to, flatted_map, item_keys, mapping);
    assert!(result.is_ok());
}

#[test]
fn test_all_item_keys() {
    let mapping: Mapping = parse_mapping_file("./tests/mapping.toml");
    let checks = parse_xlsx("data/-20220831　タイチン採血データ.xlsx", mapping.clone()).unwrap();
    assert_eq!(checks.len(), 97307);
    let item_keys = all_item_keys(checks, mapping);
    assert_eq!(523, item_keys.len());
}
