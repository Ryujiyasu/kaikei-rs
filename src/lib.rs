//! # kaikei-rs
//!
//! Japanese tax calculation library for sole proprietors (個人事業主向け税金計算ライブラリ).
//!
//! Supports fiscal year selection so tax rates and deductions stay accurate
//! as legislation changes.
//!
//! ## Quick Start
//!
//! ```
//! use kaikei_rs::tax::{FiscalYear, income, SolePropResult};
//!
//! // 売上800万、経費200万、青色申告(e-Tax)の場合
//! let result = income::calc_sole_proprietor(
//!     FiscalYear::Reiwa7,
//!     8_000_000,
//!     2_000_000,
//!     true,
//! );
//!
//! println!("所得税: {}円", result.income_tax);
//! println!("住民税: {}円", result.resident_tax);
//! println!("合計: {}円", result.total());
//! ```

pub mod tax;
