//! Integration tests for plan queries
//!
//! This module contains integration tests that verify the plan queries functionality
//! works correctly across the entire lago-client crate. These tests focus on:
//!
//! - Type compatibility between lago-types and lago-client
//! - Request/response object construction and validation
//! - Method signature correctness
//! - Builder pattern functionality
//! - Complex pricing model support
//!
//! **Note:** These tests do not make actual HTTP requests to the Lago API.
//! They focus on testing the client interface, type safety, and ensuring
//! all components work together correctly.

use lago_client::LagoClient;
use lago_types::{
    models::{ChargeModel, PlanInterval},
    requests::plan::{
        CreateChargeInput, CreateChargeProperties, CreatePlanInput, CreatePlanRequest,
        DeletePlanRequest, GetPlanRequest, ListPlansRequest, UpdatePlanInput, UpdatePlanRequest,
    },
};

#[test]
fn test_plan_request_creation() {
    let list_request = ListPlansRequest::new();
    assert!(list_request.pagination.page.is_none());
    assert!(list_request.pagination.per_page.is_none());

    let get_request = GetPlanRequest::new("test_plan".to_string());
    assert_eq!(get_request.code, "test_plan");

    let delete_request = DeletePlanRequest::new("old_plan".to_string());
    assert_eq!(delete_request.code, "old_plan");
}

#[test]
fn test_plan_creation_types() {
    let charge_properties = CreateChargeProperties::Standard {
        amount: "0.10".to_string(),
    };

    let charge = CreateChargeInput::new(
        uuid::Uuid::new_v4(),
        ChargeModel::Standard,
        charge_properties,
    )
    .with_invoice_display_name("API Calls".to_string())
    .with_invoiceable(true);

    assert!(matches!(charge.charge_model, ChargeModel::Standard));
    assert_eq!(charge.invoice_display_name, Some("API Calls".to_string()));
    assert_eq!(charge.invoiceable, Some(true));

    let plan_input = CreatePlanInput::new(
        "Test Plan".to_string(),
        "test_plan_v1".to_string(),
        PlanInterval::Monthly,
        2999,
        "USD".to_string(),
    )
    .with_description("A test plan".to_string())
    .with_trial_period(14.0)
    .with_pay_in_advance(true)
    .with_charges(vec![charge]);

    assert_eq!(plan_input.name, "Test Plan");
    assert_eq!(plan_input.code, "test_plan_v1");
    assert!(matches!(plan_input.interval, PlanInterval::Monthly));
    assert_eq!(plan_input.amount_cents, 2999);
    assert_eq!(plan_input.amount_currency, "USD");
    assert_eq!(plan_input.description, Some("A test plan".to_string()));
    assert_eq!(plan_input.trial_period, Some(14.0));
    assert_eq!(plan_input.pay_in_advance, Some(true));
    assert_eq!(plan_input.charges.as_ref().unwrap().len(), 1);

    let create_request = CreatePlanRequest::new(plan_input);
    assert_eq!(create_request.plan.name, "Test Plan");
}

#[test]
fn test_plan_update_types() {
    let update_input = UpdatePlanInput::new()
        .with_name("Updated Plan".to_string())
        .with_description("Updated description".to_string())
        .with_cascade_updates(true);

    assert_eq!(update_input.name, Some("Updated Plan".to_string()));
    assert_eq!(
        update_input.description,
        Some("Updated description".to_string())
    );
    assert_eq!(update_input.cascade_updates, Some(true));

    let update_request = UpdatePlanRequest::new(update_input);
    assert_eq!(update_request.plan.name, Some("Updated Plan".to_string()));
}

#[test]
fn test_graduated_pricing_types() {
    use lago_types::requests::plan::CreateGraduatedRange;

    let graduated_ranges = vec![
        CreateGraduatedRange {
            from_value: 0,
            to_value: Some(100),
            flat_amount: "0".to_string(),
            per_unit_amount: "0.05".to_string(),
        },
        CreateGraduatedRange {
            from_value: 101,
            to_value: None,
            flat_amount: "0".to_string(),
            per_unit_amount: "0.03".to_string(),
        },
    ];

    assert_eq!(graduated_ranges.len(), 2);
    assert_eq!(graduated_ranges[0].from_value, 0);
    assert_eq!(graduated_ranges[0].to_value, Some(100));
    assert_eq!(graduated_ranges[1].from_value, 101);
    assert_eq!(graduated_ranges[1].to_value, None);

    let graduated_properties = CreateChargeProperties::Graduated { graduated_ranges };

    let graduated_charge = CreateChargeInput::new(
        uuid::Uuid::new_v4(),
        ChargeModel::Graduated,
        graduated_properties,
    );

    assert!(matches!(
        graduated_charge.charge_model,
        ChargeModel::Graduated
    ));
}

#[test]
fn test_package_pricing_types() {
    let package_properties = CreateChargeProperties::Package {
        amount: "10.00".to_string(),
        free_units: 1000,
        package_size: 5000,
    };

    let package_charge = CreateChargeInput::new(
        uuid::Uuid::new_v4(),
        ChargeModel::Package,
        package_properties,
    )
    .with_min_amount_cents(500);

    assert!(matches!(package_charge.charge_model, ChargeModel::Package));
    assert_eq!(package_charge.min_amount_cents, Some(500));
}

#[test]
fn test_client_method_signatures() {
    let config = lago_client::Config::default();
    let client = LagoClient::new(config);

    let list_request = ListPlansRequest::new();
    let get_request = GetPlanRequest::new("test".to_string());
    let delete_request = DeletePlanRequest::new("test".to_string());

    let plan_input = CreatePlanInput::new(
        "Test".to_string(),
        "test".to_string(),
        PlanInterval::Monthly,
        1000,
        "USD".to_string(),
    );
    let create_request = CreatePlanRequest::new(plan_input);

    let update_input = UpdatePlanInput::new().with_name("Updated".to_string());
    let update_request = UpdatePlanRequest::new(update_input);

    let _list_future = client.list_plans(Some(list_request));
    let _list_future_none = client.list_plans(None);
    let _get_future = client.get_plan(get_request);
    let _create_future = client.create_plan(create_request);
    let _update_future = client.update_plan("test", update_request);
    let _delete_future = client.delete_plan(delete_request);

    // If we reach this point, all method signatures are correct
    assert!(true);
}

#[test]
fn test_plan_intervals() {
    let intervals = vec![
        PlanInterval::Weekly,
        PlanInterval::Monthly,
        PlanInterval::Quarterly,
        PlanInterval::Semiannual,
        PlanInterval::Yearly,
    ];

    for interval in intervals {
        let _plan = CreatePlanInput::new(
            "Test Plan".to_string(),
            "test_plan".to_string(),
            interval,
            1000,
            "USD".to_string(),
        );
    }
}

#[test]
fn test_charge_models() {
    let models = vec![
        ChargeModel::Standard,
        ChargeModel::Graduated,
        ChargeModel::GraduatedPercentage,
        ChargeModel::Package,
        ChargeModel::Percentage,
        ChargeModel::Volume,
        ChargeModel::Dynamic,
    ];

    for model in models {
        let properties = match model {
            ChargeModel::Standard => CreateChargeProperties::Standard {
                amount: "1.00".to_string(),
            },
            ChargeModel::Graduated => CreateChargeProperties::Graduated {
                graduated_ranges: vec![],
            },
            ChargeModel::GraduatedPercentage => CreateChargeProperties::GraduatedPercentage {
                graduated_percentage_ranges: vec![],
            },
            ChargeModel::Package => CreateChargeProperties::Package {
                amount: "10.00".to_string(),
                free_units: 100,
                package_size: 1000,
            },
            ChargeModel::Percentage => CreateChargeProperties::Percentage {
                rate: "5.0".to_string(),
                fixed_amount: None,
                free_units_per_events: None,
                free_units_per_total_aggregation: None,
            },
            ChargeModel::Volume => CreateChargeProperties::Volume {
                volume_ranges: vec![],
            },
            ChargeModel::Dynamic => CreateChargeProperties::Dynamic {
                custom_properties: std::collections::HashMap::new(),
            },
        };

        let _charge = CreateChargeInput::new(uuid::Uuid::new_v4(), model, properties);
    }
}
