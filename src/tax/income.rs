//! Income tax and related tax calculations for sole proprietors.
//!
//! # Example
//!
//! ```
//! use kaikei_rs::tax::{FiscalYear, income, SolePropResult};
//!
//! // 売上800万、経費200万、青色申告(e-Tax)の場合
//! let result = income::calc_sole_proprietor(
//!     FiscalYear::Reiwa7,
//!     8_000_000,  // revenue
//!     2_000_000,  // expenses
//!     true,       // blue return (e-Tax)
//! );
//!
//! println!("事業所得: {}円", result.business_income);
//! println!("所得税: {}円", result.income_tax);
//! println!("合計税額: {}円", result.total());
//! println!("実効税率: {:.1}%", result.effective_rate() * 100.0);
//! ```

use super::tables::{self, BusinessType, FiscalYear};
use super::SolePropResult;

/// Calculate income tax from taxable income using the bracket table.
pub fn calc_income_tax(fy: FiscalYear, taxable_income: u64) -> u64 {
    if taxable_income == 0 {
        return 0;
    }

    let brackets = tables::income_tax_brackets(fy);
    for &(upper, rate, deduction) in brackets {
        if taxable_income <= upper {
            let tax = (taxable_income as f64 * rate) as u64;
            return tax.saturating_sub(deduction);
        }
    }
    // Should not reach here due to u64::MAX bracket
    0
}

/// Calculate reconstruction special income tax (復興特別所得税).
pub fn calc_reconstruction_tax(fy: FiscalYear, income_tax: u64) -> u64 {
    let rate = tables::reconstruction_tax_rate(fy);
    (income_tax as f64 * rate) as u64
}

/// Calculate resident tax (住民税) - income-proportional + per-capita.
pub fn calc_resident_tax(fy: FiscalYear, taxable_income: u64) -> u64 {
    if taxable_income == 0 {
        return 0;
    }
    let income_portion = (taxable_income as f64 * tables::resident_tax_rate(fy)) as u64;
    income_portion + tables::resident_tax_per_capita(fy)
}

/// Calculate business tax (個人事業税).
///
/// Business income minus 290万 deduction, times the rate for the business type.
pub fn calc_business_tax(fy: FiscalYear, business_income: u64, business_type: BusinessType) -> u64 {
    let deduction = tables::business_tax_deduction(fy);
    if business_income <= deduction {
        return 0;
    }
    let taxable = business_income - deduction;
    (taxable as f64 * business_type.rate()) as u64
}

/// Calculate all taxes for a sole proprietor (default: Type1 business).
///
/// This is the main entry point for a simple tax calculation.
///
/// - `revenue`: Gross revenue (売上)
/// - `expenses`: Deductible expenses (経費)
/// - `blue_return_etax`: Whether filing blue return via e-Tax
pub fn calc_sole_proprietor(
    fy: FiscalYear,
    revenue: u64,
    expenses: u64,
    blue_return_etax: bool,
) -> SolePropResult {
    calc_sole_proprietor_with_options(fy, revenue, expenses, blue_return_etax, BusinessType::Type1)
}

/// Calculate all taxes with full options.
pub fn calc_sole_proprietor_with_options(
    fy: FiscalYear,
    revenue: u64,
    expenses: u64,
    blue_return_etax: bool,
    business_type: BusinessType,
) -> SolePropResult {
    // 事業所得 = 売上 - 経費
    let business_income = revenue.saturating_sub(expenses);

    // 所得控除
    let basic = tables::basic_deduction(fy, business_income);
    let blue = if blue_return_etax {
        tables::blue_return_deduction(fy, true)
    } else {
        0
    };
    let total_deductions = basic + blue;

    // 課税所得 (1,000円未満切り捨て)
    let taxable_income = business_income.saturating_sub(total_deductions) / 1_000 * 1_000;

    // 所得税
    let income_tax = calc_income_tax(fy, taxable_income);

    // 復興特別所得税
    let reconstruction_tax = calc_reconstruction_tax(fy, income_tax);

    // 住民税 (課税所得ベース)
    let resident_tax = calc_resident_tax(fy, taxable_income);

    // 個人事業税 (事業所得ベース、青色控除前)
    let business_tax = calc_business_tax(fy, business_income, business_type);

    SolePropResult {
        fiscal_year: fy,
        revenue,
        expenses,
        business_income,
        total_deductions,
        taxable_income,
        income_tax,
        reconstruction_tax,
        resident_tax,
        business_tax,
    }
}
