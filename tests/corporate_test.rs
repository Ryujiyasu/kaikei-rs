use kaikei_rs::tax::corporate;
use kaikei_rs::tax::tables::{CapitalTier, FiscalYear};

#[test]
fn test_corporate_tax_under_threshold() {
    let fy = FiscalYear::Reiwa7;
    // 課税所得800万 → 800万 * 15% = 1,200,000
    assert_eq!(corporate::calc_corporate_tax(fy, 8_000_000), 1_200_000);
}

#[test]
fn test_corporate_tax_over_threshold() {
    let fy = FiscalYear::Reiwa7;
    // 課税所得1,000万 → 800万*15% + 200万*23.2% = 1,200,000 + 464,000 = 1,664,000
    assert_eq!(corporate::calc_corporate_tax(fy, 10_000_000), 1_664_000);
}

#[test]
fn test_corporate_tax_zero() {
    assert_eq!(corporate::calc_corporate_tax(FiscalYear::Reiwa7, 0), 0);
}

#[test]
fn test_local_corporate_tax() {
    let fy = FiscalYear::Reiwa7;
    // 法人税100万 → 100万 * 10.3% = 103,000
    assert_eq!(corporate::calc_local_corporate_tax(fy, 1_000_000), 103_000);
}

#[test]
fn test_corporate_enterprise_tax_bracket1() {
    let fy = FiscalYear::Reiwa7;
    // 課税所得400万 → 400万 * 3.5% = 140,000
    assert_eq!(corporate::calc_corporate_enterprise_tax(fy, 4_000_000), 140_000);
}

#[test]
fn test_corporate_enterprise_tax_bracket2() {
    let fy = FiscalYear::Reiwa7;
    // 課税所得800万 → 400万*3.5% + 400万*5.3% = 140,000 + 212,000 = 352,000
    assert_eq!(corporate::calc_corporate_enterprise_tax(fy, 8_000_000), 352_000);
}

#[test]
fn test_corporate_enterprise_tax_bracket3() {
    let fy = FiscalYear::Reiwa7;
    // 課税所得1,000万 → 400万*3.5% + 400万*5.3% + 200万*7.0%
    // = 140,000 + 212,000 + 140,000 = 492,000
    assert_eq!(corporate::calc_corporate_enterprise_tax(fy, 10_000_000), 492_000);
}

#[test]
fn test_special_enterprise_tax() {
    let fy = FiscalYear::Reiwa7;
    // 事業税352,000 → 352,000 * 37% = 130,240
    assert_eq!(corporate::calc_special_enterprise_tax(fy, 352_000), 130_240);
}

#[test]
fn test_corporate_full_calculation() {
    // 売上2,000万、経費1,000万、資本金1,000万以下、従業員50人以下
    let result = corporate::calc_corporate(
        FiscalYear::Reiwa7,
        20_000_000,
        10_000_000,
        CapitalTier::Under10M,
        false,
    );

    // 課税所得 = 2,000万 - 1,000万 = 1,000万
    assert_eq!(result.taxable_income, 10_000_000);

    // 法人税 = 800万*15% + 200万*23.2% = 1,664,000
    assert_eq!(result.corporate_tax, 1_664_000);

    // 地方法人税 = 1,664,000 * 10.3% = 171,392
    assert_eq!(result.local_corporate_tax, 171_392);

    // 法人住民税 = 1,664,000 * 7% + 70,000 = 186,480
    assert_eq!(result.corporate_resident_tax, 186_480);

    // 法人事業税 = 400万*3.5% + 400万*5.3% + 200万*7.0% = 492,000
    assert_eq!(result.enterprise_tax, 492_000);

    // 特別法人事業税 = 492,000 * 37% = 182,040
    assert_eq!(result.special_enterprise_tax, 182_040);

    // 合計
    let total = 1_664_000 + 171_392 + 186_480 + 492_000 + 182_040;
    assert_eq!(result.total(), total);

    // 実効税率 (~27%)
    assert!(result.effective_rate() > 0.26);
    assert!(result.effective_rate() < 0.28);
}

#[test]
fn test_corporate_zero_income() {
    let result = corporate::calc_corporate(
        FiscalYear::Reiwa7,
        0,
        0,
        CapitalTier::Under10M,
        false,
    );
    // 均等割のみかかる
    assert_eq!(result.corporate_tax, 0);
    assert_eq!(result.corporate_resident_tax, 70_000);
    assert_eq!(result.total(), 70_000);
}

#[test]
fn test_corporate_loss() {
    // 赤字でも均等割はかかる
    let result = corporate::calc_corporate(
        FiscalYear::Reiwa7,
        5_000_000,
        8_000_000,
        CapitalTier::Under10M,
        false,
    );
    assert_eq!(result.taxable_income, 0);
    assert_eq!(result.corporate_tax, 0);
    assert_eq!(result.corporate_resident_tax, 70_000); // 均等割のみ
    assert_eq!(result.total(), 70_000);
}
