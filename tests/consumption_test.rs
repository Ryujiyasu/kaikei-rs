use kaikei_rs::tax::consumption;
use kaikei_rs::tax::FiscalYear;

#[test]
fn test_standard_rate_tax() {
    let fy = FiscalYear::Reiwa7;
    // 10,000円の10% = 1,000円
    assert_eq!(consumption::calc_tax(fy, 10_000, false), 1_000);
}

#[test]
fn test_reduced_rate_tax() {
    let fy = FiscalYear::Reiwa7;
    // 10,000円の8% = 800円
    assert_eq!(consumption::calc_tax(fy, 10_000, true), 800);
}

#[test]
fn test_price_with_tax() {
    let fy = FiscalYear::Reiwa7;
    assert_eq!(consumption::price_with_tax(fy, 10_000, false), 11_000);
    assert_eq!(consumption::price_with_tax(fy, 10_000, true), 10_800);
}

#[test]
fn test_tax_from_inclusive() {
    let fy = FiscalYear::Reiwa7;
    // 11,000円(税込) → 税額1,000円
    assert_eq!(consumption::tax_from_inclusive(fy, 11_000, false), 1_000);
    // 10,800円(税込, 軽減) → 税額800円
    assert_eq!(consumption::tax_from_inclusive(fy, 10_800, true), 800);
}

#[test]
fn test_price_without_tax() {
    let fy = FiscalYear::Reiwa7;
    assert_eq!(consumption::price_without_tax(fy, 11_000, false), 10_000);
    assert_eq!(consumption::price_without_tax(fy, 10_800, true), 10_000);
}

#[test]
fn test_simplified_taxation() {
    let fy = FiscalYear::Reiwa7;

    // サービス業、売上1000万（税抜）
    // 消費税 = 100万, みなし仕入率50% → 納付額 = 50万
    let tax =
        consumption::calc_simplified(fy, 10_000_000, consumption::SimplifiedCategory::Service);
    assert_eq!(tax, 500_000);

    // 卸売業、みなし仕入率90% → 納付額 = 10万
    let tax =
        consumption::calc_simplified(fy, 10_000_000, consumption::SimplifiedCategory::Wholesale);
    assert_eq!(tax, 100_000);
}
