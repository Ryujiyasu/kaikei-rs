//! Japanese tax calculation modules.
//!
//! All calculations support fiscal year selection via [`FiscalYear`].
//!
//! # Example
//!
//! ```
//! use kaikei_rs::tax::{FiscalYear, income, SolePropResult};
//!
//! let fy = FiscalYear::Reiwa7; // 令和7年 (2025)
//! let result = income::calc_sole_proprietor(fy, 8_000_000, 2_000_000, true);
//!
//! println!("所得税: {}円", result.income_tax);
//! println!("住民税: {}円", result.resident_tax);
//! println!("個人事業税: {}円", result.business_tax);
//! println!("合計: {}円", result.total());
//! ```

pub mod consumption;
pub mod income;
pub mod tables;

pub use tables::FiscalYear;

/// Summary of all taxes for a sole proprietor.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SolePropResult {
    /// Fiscal year used for calculation.
    pub fiscal_year: FiscalYear,
    /// Gross revenue (売上).
    pub revenue: u64,
    /// Total deductible expenses (経費).
    pub expenses: u64,
    /// Business income before deductions (事業所得).
    pub business_income: u64,
    /// Total personal deductions applied (所得控除合計).
    pub total_deductions: u64,
    /// Taxable income (課税所得).
    pub taxable_income: u64,
    /// Income tax (所得税).
    pub income_tax: u64,
    /// Reconstruction special income tax (復興特別所得税).
    pub reconstruction_tax: u64,
    /// Resident tax (住民税).
    pub resident_tax: u64,
    /// Business tax (個人事業税).
    pub business_tax: u64,
}

impl SolePropResult {
    /// Total tax burden (税金合計).
    pub fn total(&self) -> u64 {
        self.income_tax + self.reconstruction_tax + self.resident_tax + self.business_tax
    }

    /// Effective tax rate (実効税率).
    pub fn effective_rate(&self) -> f64 {
        if self.business_income == 0 {
            return 0.0;
        }
        self.total() as f64 / self.business_income as f64
    }
}
