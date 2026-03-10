//! Consumption tax (消費税) calculations.
//!
//! Supports standard rate (10%) and reduced rate (8%) for food/newspapers.
//!
//! # Example
//!
//! ```
//! use kaikei_rs::tax::{FiscalYear, consumption};
//!
//! let fy = FiscalYear::Reiwa7;
//!
//! // 税込価格から税額を計算
//! let tax = consumption::tax_from_inclusive(fy, 11_000, false);
//! assert_eq!(tax, 1_000);
//!
//! // 税抜価格から税込価格を計算
//! let inclusive = consumption::price_with_tax(fy, 10_000, false);
//! assert_eq!(inclusive, 11_000);
//! ```

use super::tables::{self, FiscalYear};

/// Calculate consumption tax amount from a tax-exclusive price (税抜→税額).
pub fn calc_tax(fy: FiscalYear, price_exclusive: u64, reduced_rate: bool) -> u64 {
    let rate = if reduced_rate {
        tables::consumption_tax_reduced_rate(fy)
    } else {
        tables::consumption_tax_rate(fy)
    };
    (price_exclusive as f64 * rate) as u64
}

/// Calculate tax-inclusive price from a tax-exclusive price (税抜→税込).
pub fn price_with_tax(fy: FiscalYear, price_exclusive: u64, reduced_rate: bool) -> u64 {
    price_exclusive + calc_tax(fy, price_exclusive, reduced_rate)
}

/// Extract consumption tax from a tax-inclusive price (税込→税額).
pub fn tax_from_inclusive(fy: FiscalYear, price_inclusive: u64, reduced_rate: bool) -> u64 {
    let rate = if reduced_rate {
        tables::consumption_tax_reduced_rate(fy)
    } else {
        tables::consumption_tax_rate(fy)
    };
    // tax = inclusive * rate / (1 + rate)
    (price_inclusive as f64 * rate / (1.0 + rate)).round() as u64
}

/// Extract tax-exclusive price from a tax-inclusive price (税込→税抜).
pub fn price_without_tax(fy: FiscalYear, price_inclusive: u64, reduced_rate: bool) -> u64 {
    price_inclusive - tax_from_inclusive(fy, price_inclusive, reduced_rate)
}

/// Simplified tax calculation for small businesses (簡易課税制度).
///
/// For businesses with taxable sales ≤ 50M yen in base period.
/// Uses deemed purchase rate (みなし仕入率) by business category.
#[derive(Debug, Clone, Copy)]
pub enum SimplifiedCategory {
    /// 第1種: 卸売業 (90%)
    Wholesale,
    /// 第2種: 小売業 (80%)
    Retail,
    /// 第3種: 製造業等 (70%)
    Manufacturing,
    /// 第4種: その他 (60%)
    Other,
    /// 第5種: サービス業等 (50%)
    Service,
    /// 第6種: 不動産業 (40%)
    RealEstate,
}

impl SimplifiedCategory {
    /// Deemed purchase rate (みなし仕入率).
    pub fn deemed_rate(self) -> f64 {
        match self {
            SimplifiedCategory::Wholesale => 0.90,
            SimplifiedCategory::Retail => 0.80,
            SimplifiedCategory::Manufacturing => 0.70,
            SimplifiedCategory::Other => 0.60,
            SimplifiedCategory::Service => 0.50,
            SimplifiedCategory::RealEstate => 0.40,
        }
    }
}

/// Calculate consumption tax payable under simplified taxation (簡易課税).
///
/// - `taxable_sales`: Tax-exclusive taxable sales amount
/// - `category`: Business category for deemed purchase rate
pub fn calc_simplified(fy: FiscalYear, taxable_sales: u64, category: SimplifiedCategory) -> u64 {
    let sales_tax = calc_tax(fy, taxable_sales, false);
    let deemed_purchase_tax = (sales_tax as f64 * category.deemed_rate()) as u64;
    sales_tax - deemed_purchase_tax
}
