# kaikei-rs

[![Crates.io](https://img.shields.io/crates/v/kaikei-rs.svg)](https://crates.io/crates/kaikei-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

日本の税金計算ライブラリ for Rust（個人事業主・法人対応）

年度ごとの税率・控除額に対応しており、税制改正があっても正確な計算が可能です。

## 機能

- **所得税** (income tax) — 累進課税の速算表方式
- **復興特別所得税** — 所得税額の2.1%
- **住民税** — 所得割＋均等割
- **個人事業税** — 業種別税率、290万円の事業主控除
- **法人税** — 資本金規模別・所得区分別の税率
- **消費税** — 標準税率10%・軽減税率8%、簡易課税対応
- **青色申告特別控除** — 65万円控除（e-Tax）/ 55万円控除

## 対応年度

| 年度 | 西暦 | 主な変更点 |
|---|---|---|
| 令和6年 (`Reiwa6`) | 2024年 | 基礎控除48万円 |
| 令和7年 (`Reiwa7`) | 2025年 | 基礎控除58万円（税制改正） |

## インストール

```toml
[dependencies]
kaikei-rs = "0.2"
```

## 使い方

### 個人事業主の税金を一括計算

```rust
use kaikei_rs::tax::{FiscalYear, income};

// 売上800万円、経費200万円、青色申告(e-Tax)の場合
let result = income::calc_sole_proprietor(
    FiscalYear::Reiwa7,
    8_000_000,  // 売上
    2_000_000,  // 経費
    true,       // 青色申告(e-Tax)
);

println!("事業所得: {}円", result.business_income); // 5,420,000円
println!("所得税:   {}円", result.income_tax);       // 526,500円
println!("住民税:   {}円", result.resident_tax);     // 482,000円
println!("事業税:   {}円", result.business_tax);     // 155,000円
println!("合計:     {}円", result.total());          // 1,174,556円
println!("実効税率: {:.1}%", result.effective_rate() * 100.0);
```

### 法人税の計算

```rust
use kaikei_rs::tax::{FiscalYear, corporate};
use kaikei_rs::tax::tables::CapitalTier;

let result = corporate::calc_corporate(
    FiscalYear::Reiwa7,
    50_000_000,  // 売上
    30_000_000,  // 経費
    CapitalTier::Under10M,  // 資本金1,000万円以下
    false,                   // 従業員50人以下
);

println!("法人税:       {}円", result.corporate_tax);
println!("地方法人税:   {}円", result.local_corporate_tax);
println!("法人住民税:   {}円", result.corporate_resident_tax);
println!("法人事業税:   {}円", result.enterprise_tax);
println!("特別法人事業税: {}円", result.special_enterprise_tax);
println!("合計:         {}円", result.total());
```

### 消費税の計算

```rust
use kaikei_rs::tax::{FiscalYear, consumption};

let fy = FiscalYear::Reiwa7;

// 税込→税抜
let price = consumption::price_without_tax(fy, 11_000, false);
assert_eq!(price, 10_000);

// 税込→消費税額
let tax = consumption::tax_from_inclusive(fy, 11_000, false);
assert_eq!(tax, 1_000);

// 簡易課税（サービス業）
let tax = consumption::calc_simplified(
    fy,
    10_000_000,  // 課税売上（税抜）
    consumption::SimplifiedCategory::Service,
);
assert_eq!(tax, 500_000); // 50万円納付
```

## プロジェクトで使われている場所

- [みんなの経理](https://github.com/Ryujiyasu/minnano-keiri) — 個人事業主・一人法人向け無料会計Webアプリ（WebAssembly経由で利用）

## ライセンス

[MIT](LICENSE)
