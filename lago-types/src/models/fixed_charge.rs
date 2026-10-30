use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::models::PlanTax;

/// Fixed charge model types.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FixedChargeModel {
    /// Standard pricing model.
    Standard,
    /// Graduated pricing model.
    Graduated,
    /// Volume pricing model.
    Volume,
}

/// Represents a fixed charge in the Lago billing system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedCharge {
    /// Unique identifier for the fixed charge in Lago.
    pub lago_id: Uuid,
    /// The Lago ID of the associated add-on.
    pub lago_add_on_id: Option<Uuid>,
    /// Display name for the fixed charge on invoices.
    pub invoice_display_name: Option<String>,
    /// The code of the associated add-on.
    pub add_on_code: Option<String>,
    /// When the fixed charge was created.
    pub created_at: DateTime<Utc>,
    /// Unique code for the fixed charge.
    pub code: Option<String>,
    /// The charge model to use.
    pub charge_model: FixedChargeModel,
    /// Whether the fixed charge is billed in advance.
    pub pay_in_advance: Option<bool>,
    /// Whether the fixed charge is prorated.
    pub prorated: Option<bool>,
    /// Fixed charge properties (model-specific configuration).
    pub properties: Option<serde_json::Value>,
    /// Number of units, as the decimal string the API sends (e.g. "20.0").
    pub units: Option<String>,
    /// Parent fixed charge ID (for overrides).
    pub lago_parent_id: Option<Uuid>,
    /// Taxes applied to this fixed charge.
    pub taxes: Option<Vec<PlanTax>>,
}

#[cfg(test)]
mod tests {
    use super::FixedCharge;
    use crate::models::Plan;
    use crate::responses::plan::GetPlanResponse;
    use crate::responses::plan_charge::{GetPlanFixedChargeResponse, ListPlanFixedChargesResponse};
    use crate::responses::subscription_charge::UpdateSubscriptionFixedChargeResponse;

    fn fixed_charge_json() -> serde_json::Value {
        serde_json::json!({
            "lago_id": "1a901a90-1a90-1a90-1a90-1a901a901a90",
            "lago_add_on_id": "2b012b01-2b01-2b01-2b01-2b012b012b01",
            "invoice_display_name": "Seats",
            "add_on_code": "seats",
            "created_at": "2026-10-01T00:00:00Z",
            "code": "seats",
            "charge_model": "standard",
            "pay_in_advance": true,
            "prorated": false,
            "properties": { "amount": "10" },
            "units": "20.0",
            "lago_parent_id": null,
            "taxes": []
        })
    }

    #[test]
    fn deserializes_units_sent_as_a_decimal_string() {
        let json = serde_json::json!({ "fixed_charge": fixed_charge_json() }).to_string();

        let response: UpdateSubscriptionFixedChargeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.fixed_charge.units, Some("20.0".to_string()));
    }

    #[test]
    fn deserializes_null_units_as_none() {
        let mut json = fixed_charge_json();
        json["units"] = serde_json::Value::Null;

        let fixed_charge: FixedCharge = serde_json::from_value(json).unwrap();

        assert_eq!(fixed_charge.units, None);
    }

    #[test]
    fn deserializes_missing_units_as_none() {
        let mut json = fixed_charge_json();
        json.as_object_mut().unwrap().remove("units");

        let fixed_charge: FixedCharge = serde_json::from_value(json).unwrap();

        assert_eq!(fixed_charge.units, None);
    }

    #[test]
    fn deserializes_plan_with_fixed_charges_having_string_units() {
        let plan_json = serde_json::json!({
            "lago_id": "3c123c12-3c12-3c12-3c12-3c123c123c12",
            "name": "Startup",
            "created_at": "2026-10-01T00:00:00Z",
            "code": "startup",
            "interval": "monthly",
            "amount_cents": 10000,
            "amount_currency": "USD",
            "pay_in_advance": false,
            "fixed_charges": [fixed_charge_json()]
        });
        let json = serde_json::json!({ "plan": plan_json }).to_string();

        let plan: Plan = serde_json::from_value(plan_json).unwrap();
        let response: GetPlanResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(
            plan.fixed_charges.unwrap()[0].units,
            Some("20.0".to_string())
        );
        assert_eq!(
            response.plan.fixed_charges.unwrap()[0].units,
            Some("20.0".to_string())
        );
    }

    #[test]
    fn deserializes_plan_fixed_charge_response_with_string_units() {
        let json = serde_json::json!({ "fixed_charge": fixed_charge_json() }).to_string();

        let response: GetPlanFixedChargeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.fixed_charge.units, Some("20.0".to_string()));
    }

    #[test]
    fn deserializes_plan_fixed_charges_list_response_with_string_units() {
        let mut second = fixed_charge_json();
        second["lago_id"] = serde_json::json!("4d234d23-4d23-4d23-4d23-4d234d234d23");
        second["units"] = serde_json::json!("3.5");
        let json = serde_json::json!({
            "fixed_charges": [fixed_charge_json(), second],
            "meta": {
                "current_page": 1,
                "next_page": null,
                "prev_page": null,
                "total_pages": 1,
                "total_count": 2
            }
        })
        .to_string();

        let response: ListPlanFixedChargesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.fixed_charges.len(), 2);
        assert_eq!(response.fixed_charges[0].units, Some("20.0".to_string()));
        assert_eq!(response.fixed_charges[1].units, Some("3.5".to_string()));
    }

    #[test]
    fn round_trips_units_as_a_json_string() {
        let fixed_charge: FixedCharge = serde_json::from_value(fixed_charge_json()).unwrap();

        let value = serde_json::to_value(&fixed_charge).unwrap();
        let parsed: FixedCharge = serde_json::from_value(value.clone()).unwrap();

        assert_eq!(value["units"], serde_json::json!("20.0"));
        assert_eq!(parsed.units, Some("20.0".to_string()));
    }
}
