//! WASM bindings for browser usage.

use wasm_bindgen::prelude::*;

use crate::tax::income;
use crate::tax::tables::{BusinessType, FiscalYear};

fn parse_fiscal_year(year: &str) -> Result<FiscalYear, JsValue> {
    match year {
        "reiwa6" | "2024" => Ok(FiscalYear::Reiwa6),
        "reiwa7" | "2025" => Ok(FiscalYear::Reiwa7),
        _ => Err(JsValue::from_str("Unsupported fiscal year")),
    }
}

fn parse_business_type(btype: &str) -> BusinessType {
    match btype {
        "type1" => BusinessType::Type1,
        "type2" => BusinessType::Type2,
        "type3" => BusinessType::Type3,
        "type3low" => BusinessType::Type3Low,
        "exempt" => BusinessType::Exempt,
        _ => BusinessType::Type1,
    }
}

/// Calculate all taxes for a sole proprietor. Returns JSON string.
#[wasm_bindgen]
pub fn calc_sole_proprietor(
    fiscal_year: &str,
    revenue: u64,
    expenses: u64,
    blue_return: bool,
    business_type: &str,
) -> Result<String, JsValue> {
    let fy = parse_fiscal_year(fiscal_year)?;
    let btype = parse_business_type(business_type);
    let result = income::calc_sole_proprietor_with_options(fy, revenue, expenses, blue_return, btype);

    serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Calculate consumption tax from inclusive price. Returns tax amount.
#[wasm_bindgen]
pub fn consumption_tax_from_inclusive(
    fiscal_year: &str,
    price_inclusive: u64,
    reduced_rate: bool,
) -> Result<u64, JsValue> {
    let fy = parse_fiscal_year(fiscal_year)?;
    Ok(crate::tax::consumption::tax_from_inclusive(
        fy,
        price_inclusive,
        reduced_rate,
    ))
}

/// Calculate price without tax from inclusive price.
#[wasm_bindgen]
pub fn price_without_tax(
    fiscal_year: &str,
    price_inclusive: u64,
    reduced_rate: bool,
) -> Result<u64, JsValue> {
    let fy = parse_fiscal_year(fiscal_year)?;
    Ok(crate::tax::consumption::price_without_tax(
        fy,
        price_inclusive,
        reduced_rate,
    ))
}
