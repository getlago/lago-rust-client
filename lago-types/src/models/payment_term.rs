use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// A structured payment term: how the payment due date of an invoice is
/// computed from its issuing date.
///
/// The fields accepted and returned depend on `term_type`; the API rejects
/// a field that does not belong to the type, so only set the ones it lists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentTerm {
    /// The type of payment term, which decides the other fields.
    pub term_type: PaymentTermType,
    /// Number of days, 0 to 365. Used by `net`, `net_end_of_month` and `days_end_of_month`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// Day of the month the invoice is due on, 1 to 31. Used by `day_of_month`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    /// Months after the issuing month, 0 to 12. Used by `day_of_month`; the API defaults it to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_offset: Option<i32>,
}

impl PaymentTerm {
    /// Creates a payment term of the given type with no other field set.
    pub fn new(term_type: PaymentTermType) -> Self {
        Self {
            term_type,
            days: None,
            day_of_month: None,
            month_offset: None,
        }
    }

    /// Sets the number of days.
    pub fn with_days(mut self, days: i32) -> Self {
        self.days = Some(days);
        self
    }

    /// Sets the day of the month.
    pub fn with_day_of_month(mut self, day_of_month: i32) -> Self {
        self.day_of_month = Some(day_of_month);
        self
    }

    /// Sets the month offset.
    pub fn with_month_offset(mut self, month_offset: i32) -> Self {
        self.month_offset = Some(month_offset);
        self
    }
}

/// Type of a payment term.
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PaymentTermType {
    /// Due on the issuing date.
    DueOnReceipt,
    /// Due `days` days after the issuing date.
    Net,
    /// Due on the last day of the issuing month.
    EndOfMonth,
    /// Due `days` days after the last day of the issuing month.
    NetEndOfMonth,
    /// Due on the last day of the month reached `days` days after the issuing date.
    DaysEndOfMonth,
    /// Due on `day_of_month` of the month `month_offset` months after the issuing month.
    DayOfMonth,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn serializes_every_term_type_in_snake_case() {
        for (term_type, expected) in [
            (PaymentTermType::DueOnReceipt, "due_on_receipt"),
            (PaymentTermType::Net, "net"),
            (PaymentTermType::EndOfMonth, "end_of_month"),
            (PaymentTermType::NetEndOfMonth, "net_end_of_month"),
            (PaymentTermType::DaysEndOfMonth, "days_end_of_month"),
            (PaymentTermType::DayOfMonth, "day_of_month"),
        ] {
            assert_eq!(serde_json::to_value(&term_type).unwrap(), expected);
            assert_eq!(PaymentTermType::from_str(expected).unwrap(), term_type);
        }
    }

    #[test]
    fn serializes_only_the_fields_that_are_set() {
        let term = PaymentTerm::new(PaymentTermType::Net).with_days(30);
        assert_eq!(
            serde_json::to_value(&term).unwrap(),
            serde_json::json!({ "term_type": "net", "days": 30 })
        );

        let term = PaymentTerm::new(PaymentTermType::DueOnReceipt);
        assert_eq!(
            serde_json::to_value(&term).unwrap(),
            serde_json::json!({ "term_type": "due_on_receipt" })
        );

        let term = PaymentTerm::new(PaymentTermType::DayOfMonth)
            .with_day_of_month(15)
            .with_month_offset(0);
        assert_eq!(
            serde_json::to_value(&term).unwrap(),
            serde_json::json!({
                "term_type": "day_of_month",
                "day_of_month": 15,
                "month_offset": 0
            })
        );
    }

    #[test]
    fn deserializes_a_term_as_the_api_returns_it() {
        let term: PaymentTerm =
            serde_json::from_str(r#"{"term_type": "net_end_of_month", "days": 15}"#).unwrap();
        assert_eq!(
            term,
            PaymentTerm::new(PaymentTermType::NetEndOfMonth).with_days(15)
        );

        let term: PaymentTerm = serde_json::from_str(
            r#"{"term_type": "day_of_month", "day_of_month": 31, "month_offset": 1}"#,
        )
        .unwrap();
        assert_eq!(
            term,
            PaymentTerm::new(PaymentTermType::DayOfMonth)
                .with_day_of_month(31)
                .with_month_offset(1)
        );
    }

    #[test]
    fn round_trips_a_term() {
        let json = serde_json::json!({ "term_type": "days_end_of_month", "days": 10 });
        let term: PaymentTerm = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(serde_json::to_value(&term).unwrap(), json);
    }

    #[test]
    fn rejects_an_unknown_term_type() {
        assert!(serde_json::from_str::<PaymentTerm>(r#"{"term_type": "weekly"}"#).is_err());
    }
}
