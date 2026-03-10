use kaikei_rs::tax::income;
use kaikei_rs::tax::tables::{BusinessType, FiscalYear};
use kaikei_rs::tax::FiscalYear as FY;

#[test]
fn test_income_tax_brackets_reiwa7() {
    let fy = FY::Reiwa7;

    // 課税所得195万 → 5% - 0 = 97,500
    assert_eq!(income::calc_income_tax(fy, 1_950_000), 97_500);

    // 課税所得330万 → 10% - 97,500 = 232,500
    assert_eq!(income::calc_income_tax(fy, 3_300_000), 232_500);

    // 課税所得695万 → 20% - 427,500 = 962,500
    assert_eq!(income::calc_income_tax(fy, 6_950_000), 962_500);

    // 課税所得900万 → 23% - 636,000 = 1,434,000
    assert_eq!(income::calc_income_tax(fy, 9_000_000), 1_434_000);

    // 課税所得0 → 0
    assert_eq!(income::calc_income_tax(fy, 0), 0);

    // 課税所得100万 → 5% = 50,000
    assert_eq!(income::calc_income_tax(fy, 1_000_000), 50_000);
}

#[test]
fn test_reconstruction_tax() {
    let fy = FY::Reiwa7;
    // 所得税100万 → 復興税 = 100万 * 2.1% = 21,000
    assert_eq!(income::calc_reconstruction_tax(fy, 1_000_000), 21_000);
}

#[test]
fn test_resident_tax() {
    let fy = FY::Reiwa7;
    // 課税所得500万 → 所得割 50万 + 均等割 5,000 = 505,000
    assert_eq!(income::calc_resident_tax(fy, 5_000_000), 505_000);

    // 課税所得0 → 0
    assert_eq!(income::calc_resident_tax(fy, 0), 0);
}

#[test]
fn test_business_tax() {
    let fy = FY::Reiwa7;

    // 事業所得500万, 第1種 → (500万 - 290万) * 5% = 105,000
    assert_eq!(
        income::calc_business_tax(fy, 5_000_000, BusinessType::Type1),
        105_000
    );

    // 事業所得200万 → 290万以下なので0
    assert_eq!(
        income::calc_business_tax(fy, 2_000_000, BusinessType::Type1),
        0
    );

    // 非課税業種 → 0
    assert_eq!(
        income::calc_business_tax(fy, 10_000_000, BusinessType::Exempt),
        0
    );
}

#[test]
fn test_sole_proprietor_full_calculation() {
    // 売上800万、経費200万、青色申告(e-Tax)、令和7年
    let result = income::calc_sole_proprietor(FY::Reiwa7, 8_000_000, 2_000_000, true);

    // 事業所得 = 800万 - 200万 = 600万
    assert_eq!(result.business_income, 6_000_000);

    // 所得控除 = 基礎控除58万 + 青色65万 = 123万
    assert_eq!(result.total_deductions, 1_230_000);

    // 課税所得 = 600万 - 123万 = 477万 → 1000円未満切り捨て = 4,770,000
    assert_eq!(result.taxable_income, 4_770_000);

    // 所得税 = 477万 * 20% - 427,500 = 526,500
    assert_eq!(result.income_tax, 526_500);

    // 復興税 = 526,500 * 2.1% = 11,056
    assert_eq!(result.reconstruction_tax, 11_056);

    // 住民税 = 477万 * 10% + 5,000 = 482,000
    assert_eq!(result.resident_tax, 482_000);

    // 事業税 = (600万 - 290万) * 5% = 155,000
    assert_eq!(result.business_tax, 155_000);

    // 合計
    assert_eq!(result.total(), 526_500 + 11_056 + 482_000 + 155_000);

    // 実効税率
    assert!(result.effective_rate() > 0.19);
    assert!(result.effective_rate() < 0.20);
}

#[test]
fn test_sole_proprietor_reiwa6_vs_reiwa7() {
    // 同じ条件で令和6年と令和7年を比較
    // 令和7年は基礎控除が48万→58万に上がっている
    let r6 = income::calc_sole_proprietor(FiscalYear::Reiwa6, 8_000_000, 2_000_000, true);
    let r7 = income::calc_sole_proprietor(FiscalYear::Reiwa7, 8_000_000, 2_000_000, true);

    // 令和7年の方が控除が大きいので税金が安い
    assert!(r7.total() < r6.total());
    assert!(r7.total_deductions > r6.total_deductions);
}

#[test]
fn test_zero_income() {
    let result = income::calc_sole_proprietor(FY::Reiwa7, 0, 0, false);
    assert_eq!(result.total(), 0);
    assert_eq!(result.effective_rate(), 0.0);
}

#[test]
fn test_expenses_exceed_revenue() {
    // 赤字の場合
    let result = income::calc_sole_proprietor(FY::Reiwa7, 1_000_000, 2_000_000, true);
    assert_eq!(result.business_income, 0);
    assert_eq!(result.total(), 0);
}
