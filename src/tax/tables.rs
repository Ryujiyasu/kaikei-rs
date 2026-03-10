//! Tax rate tables by fiscal year.
//!
//! Each fiscal year defines its own set of tax brackets, deductions, and rates.
//! Add new fiscal years by extending the match arms in each function.

/// Supported fiscal years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum FiscalYear {
    /// 令和6年 (2024)
    Reiwa6,
    /// 令和7年 (2025)
    Reiwa7,
}

impl FiscalYear {
    /// Calendar year corresponding to this fiscal year.
    pub fn calendar_year(self) -> u32 {
        match self {
            FiscalYear::Reiwa6 => 2024,
            FiscalYear::Reiwa7 => 2025,
        }
    }
}

impl std::fmt::Display for FiscalYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiscalYear::Reiwa6 => write!(f, "令和6年 (2024)"),
            FiscalYear::Reiwa7 => write!(f, "令和7年 (2025)"),
        }
    }
}

/// Income tax bracket: (upper_limit, rate, quick_deduction).
/// `upper_limit` is inclusive. Use `u64::MAX` for the highest bracket.
pub type IncomeTaxBracket = (u64, f64, u64);

/// Get income tax brackets for the given fiscal year.
///
/// Returns a slice of (upper_limit, rate, quick_deduction).
/// Formula: tax = taxable_income * rate - quick_deduction
pub fn income_tax_brackets(fy: FiscalYear) -> &'static [IncomeTaxBracket] {
    match fy {
        // 所得税率は令和6年・令和7年とも同じ
        FiscalYear::Reiwa6 | FiscalYear::Reiwa7 => &[
            (1_949_000, 0.05, 0),
            (3_299_000, 0.10, 97_500),
            (6_949_000, 0.20, 427_500),
            (8_999_000, 0.23, 636_000),
            (17_999_000, 0.33, 1_536_000),
            (39_999_000, 0.40, 2_796_000),
            (u64::MAX, 0.45, 4_796_000),
        ],
    }
}

/// Reconstruction special income tax rate (復興特別所得税率).
/// Applied until 2037 (令和19年).
pub fn reconstruction_tax_rate(_fy: FiscalYear) -> f64 {
    0.021 // 2.1% of income tax
}

/// Basic deduction (基礎控除) for sole proprietors.
pub fn basic_deduction(fy: FiscalYear, total_income: u64) -> u64 {
    match fy {
        FiscalYear::Reiwa6 => {
            // 令和6年: 合計所得2,400万以下→48万
            if total_income <= 24_000_000 {
                480_000
            } else if total_income <= 24_500_000 {
                320_000
            } else if total_income <= 25_000_000 {
                160_000
            } else {
                0
            }
        }
        FiscalYear::Reiwa7 => {
            // 令和7年: 合計所得2,350万以下→58万, 2,400万以下→48万
            if total_income <= 23_500_000 {
                580_000
            } else if total_income <= 24_000_000 {
                480_000
            } else if total_income <= 24_500_000 {
                320_000
            } else if total_income <= 25_000_000 {
                160_000
            } else {
                0
            }
        }
    }
}

/// Blue return special deduction (青色申告特別控除).
pub fn blue_return_deduction(_fy: FiscalYear, e_tax: bool) -> u64 {
    if e_tax {
        650_000 // e-Tax or electronic bookkeeping
    } else {
        550_000 // paper filing with double-entry bookkeeping
    }
}

/// Resident tax rate (住民税率).
/// 所得割: 市町村民税6% + 道府県民税4% = 10%
pub fn resident_tax_rate(_fy: FiscalYear) -> f64 {
    0.10
}

/// Resident tax per-capita levy (住民税均等割).
/// 市町村民税3,000円 + 道府県民税1,000円 + 森林環境税1,000円 = 5,000円
pub fn resident_tax_per_capita(fy: FiscalYear) -> u64 {
    match fy {
        FiscalYear::Reiwa6 | FiscalYear::Reiwa7 => 5_000,
    }
}

/// Business tax rate (個人事業税率) for most businesses.
/// Most common rate is 5%. Some professions are 3% or exempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum BusinessType {
    /// 第1種事業 (物品販売業、飲食店業など): 5%
    Type1,
    /// 第2種事業 (畜産業など): 4%
    Type2,
    /// 第3種事業 (医業、弁護士業など): 5%
    Type3,
    /// 第3種事業 (あんま・マッサージ等): 3%
    Type3Low,
    /// 非課税 (農業、林業、作家など)
    Exempt,
}

impl BusinessType {
    /// Tax rate for this business type.
    pub fn rate(self) -> f64 {
        match self {
            BusinessType::Type1 => 0.05,
            BusinessType::Type2 => 0.04,
            BusinessType::Type3 => 0.05,
            BusinessType::Type3Low => 0.03,
            BusinessType::Exempt => 0.0,
        }
    }
}

/// Business tax deduction (事業主控除): 290万円
pub fn business_tax_deduction(_fy: FiscalYear) -> u64 {
    2_900_000
}

/// Consumption tax standard rate (消費税標準税率).
pub fn consumption_tax_rate(_fy: FiscalYear) -> f64 {
    0.10
}

/// Consumption tax reduced rate (消費税軽減税率, food/newspaper).
pub fn consumption_tax_reduced_rate(_fy: FiscalYear) -> f64 {
    0.08
}
