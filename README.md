# リリース

- flat_chart.exe
- mapping.toml

## 実行方法

```shell
flat_chart data/xxxx.xlsx
flat_chart -m ./mapping.xml data/xxxx.xlsx
flat_chart --help
```

# データ xlsx ファイル制約

- パスワードばし
- データは１シート目にある
- 1 行目はヘッダー

# mapping.toml 定義

```toml
fixed_header = [
        "患者番号",
        "カナ姓",
        "カナ名",
        "漢字姓",
        "漢字名",
        "採取日",
]
flat_item_key = "検査項目名称"
flat_item_value = "検査結果値"

```

## fixed_header

固定列、少なく一つ項目が必要

## flat_item_key

検査項目名称

## flat_item_value

検査結果値

# 出力結果ファイル

```csv
患者番号, カナ姓, カナ名, 漢字姓, 漢字名, 採取日,検査項目名称1,検査項目名称2,...
03716172,ｵｶﾞﾜ,ﾖｼﾐﾂ,小川,義充,2006/11/25,23.2,344,...
```

## tips

- ヘッダー行を固定行にする
- 固定列をソート
