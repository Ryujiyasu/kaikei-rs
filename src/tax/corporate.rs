//! Corporate tax calculations for small/medium enterprises (中小法人).
//!
//! Covers:
//! - 法人税 (Corporate income tax)
//! - 地方法人税 (Local corporate tax)
//! - 法人住民税 (Corporate resident tax)
//! - 法人事業税 (Corporate enterprise tax)
//! - 特別法人事業税 (Special corporate enterprise tax)

use super::tables::{self, CapitalTier, FiscalYear};
use super::CorporateResult;

/// Calculate corporate income tax (法人税) for SMEs.
///
/// - 800万円以下: 15% (中小法人特例)
/// - 800万円超: 23.2%
pub fn calc_corporate_tax(fy: FiscalYear, taxable_income: u64) -> u64 {
    if taxable_income == 0 {
        return 0;
    }

    let threshold = tables::corporate_tax_reduced_threshold(fy);
    let reduced_rate = tables::corporate_tax_reduced_rate(fy);
    let standard_rate = tables::corporate_tax_standard_rate(fy);

    if taxable_income <= threshold {
        (taxable_income as f64 * reduced_rate) as u64
    } else {
        let low = (threshold as f64 * reduced_rate) as u64;
        let high = ((taxable_income - threshold) as f64 * standard_rate) as u64;
        low + high
    }
}

/// Calculate local corporate tax (地方法人税).
/// 法人税額 × 10.3%
pub fn calc_local_corporate_tax(fy: FiscalYear, corporate_tax: u64) -> u64 {
    (corporate_tax as f64 * tables::local_corporate_tax_rate(fy)) as u64
}

/// Calculate corporate resident tax (法人住民税).
/// 法人税割 (7% of 法人税額) + 均等割 (flat amount).
pub fn calc_corporate_resident_tax(
    fy: FiscalYear,
    corporate_tax: u64,
    capital_tier: CapitalTier,
    employees_over_50: bool,
) -> u64 {
    let income_portion = (corporate_tax as f64 * tables::corporate_resident_tax_rate(fy)) as u64;
    let flat_portion = tables::corporate_resident_tax_flat(fy, capital_tier, employees_over_50);
    income_portion + flat_portion
}

/// Calculate corporate enterprise tax (法人事業税 所得割).
/// Progressive brackets: 400万以下 3.5%, 400万超〜800万 5.3%, 800万超 7.0%
pub fn calc_corporate_enterprise_tax(fy: FiscalYear, taxable_income: u64) -> u64 {
    if taxable_income == 0 {
        return 0;
    }

    let (t1, t2, r1, r2, r3) = tables::corporate_enterprise_tax_rates(fy);

    if taxable_income <= t1 {
        (taxable_income as f64 * r1) as u64
    } else if taxable_income <= t2 {
        let part1 = (t1 as f64 * r1) as u64;
        let part2 = ((taxable_income - t1) as f64 * r2) as u64;
        part1 + part2
    } else {
        let part1 = (t1 as f64 * r1) as u64;
        let part2 = ((t2 - t1) as f64 * r2) as u64;
        let part3 = ((taxable_income - t2) as f64 * r3) as u64;
        part1 + part2 + part3
    }
}

/// Calculate special corporate enterprise tax (特別法人事業税).
/// 法人事業税所得割額 × 37%
pub fn calc_special_enterprise_tax(fy: FiscalYear, enterprise_tax: u64) -> u64 {
    (enterprise_tax as f64 * tables::special_corporate_enterprise_tax_rate(fy)) as u64
}

/// Calculate all taxes for a small/medium corporation.
///
/// - `revenue`: Gross revenue (売上)
/// - `expenses`: Deductible expenses (経費, including officer compensation)
/// - `capital_tier`: Capital range for resident tax flat portion
/// - `employees_over_50`: Whether employee count exceeds 50
pub fn calc_corporate(
    fy: FiscalYear,
    revenue: u64,
    expenses: u64,
    capital_tier: CapitalTier,
    employees_over_50: bool,
) -> CorporateResult {
    // 課税所得 = 売上 - 経費 (1,000円未満切り捨て)
    let taxable_income = revenue.saturating_sub(expenses) / 1_000 * 1_000;

    // 法人税
    let corporate_tax = calc_corporate_tax(fy, taxable_income);

    // 地方法人税 (法人税額ベース)
    let local_corporate_tax = calc_local_corporate_tax(fy, corporate_tax);

    // 法人住民税 (法人税額ベース + 均等割)
    let corporate_resident_tax =
        calc_corporate_resident_tax(fy, corporate_tax, capital_tier, employees_over_50);

    // 法人事業税 (課税所得ベース)
    let enterprise_tax = calc_corporate_enterprise_tax(fy, taxable_income);

    // 特別法人事業税 (事業税ベース)
    let special_enterprise_tax = calc_special_enterprise_tax(fy, enterprise_tax);

    CorporateResult {
        fiscal_year: fy,
        revenue,
        expenses,
        taxable_income,
        corporate_tax,
        local_corporate_tax,
        corporate_resident_tax,
        enterprise_tax,
        special_enterprise_tax,
    }
}
