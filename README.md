# kaikei-rs

[![Crates.io](https://img.shields.io/crates/v/kaikei-rs.svg)](https://crates.io/crates/kaikei-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Japanese tax calculation library for sole proprietors (個人事業主向け税金計算ライブラリ).

## Features

- **Fiscal year aware** — specify the year, get the correct rates and deductions
- **Income tax** (所得税) — progressive brackets with quick deduction method
- **Reconstruction tax** (復興特別所得税) — 2.1% surcharge
- **Resident tax** (住民税) — income-proportional + per-capita
- **Business tax** (個人事業税) — by business type with 290万 deduction
- **Consumption tax** (消費税) — standard 10% / reduced 8%, simplified taxation
- **Blue return** (青色申告) — 65万/55万 special deduction

## Supported Fiscal Years

| Fiscal Year | Calendar Year | Notes |
|---|---|---|
| `Reiwa6` | 2024 | 基礎控除48万 |
| `Reiwa7` | 2025 | 基礎控除58万 (改正) |

## Installation

```toml
[dependencies]
kaikei-rs = "0.1"
```

## Usage

### All-in-one calculation

```rust
use kaikei_rs::tax::{FiscalYear, income};

let result = income::calc_sole_proprietor(
    FiscalYear::Reiwa7,
    8_000_000,  // 売上
    2_000_000,  // 経費
    true,       // 青色申告(e-Tax)
);

println!("事業所得: {}円", result.business_income); // 6,000,000
println!("所得税:   {}円", result.income_tax);       // 526,500
println!("住民税:   {}円", result.resident_tax);     // 482,000
println!("事業税:   {}円", result.business_tax);     // 155,000
println!("合計:     {}円", result.total());          // 1,174,556
println!("実効税率: {:.1}%", result.effective_rate() * 100.0);
```

### Consumption tax

```rust
use kaikei_rs::tax::{FiscalYear, consumption};

let fy = FiscalYear::Reiwa7;

// 税込→税抜
let price = consumption::price_without_tax(fy, 11_000, false);
assert_eq!(price, 10_000);

// 簡易課税（サービス業）
let tax = consumption::calc_simplified(
    fy,
    10_000_000,
    consumption::SimplifiedCategory::Service,
);
assert_eq!(tax, 500_000); // 50万円納付
```

## License

MIT
