# InvoiceObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | Option<**String**> |  | [optional]
**sequential_id** | Option<**i32**> |  | [optional]
**number** | Option<**String**> |  | [optional]
**issuing_date** | Option<**String**> |  | [optional]
**invoice_type** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**payment_status** | Option<**String**> |  | [optional]
**amount_cents** | Option<**i32**> |  | [optional]
**amount_currency** | Option<**String**> |  | [optional]
**vat_amount_cents** | Option<**i32**> |  | [optional]
**vat_amount_currency** | Option<**String**> |  | [optional]
**credit_amount_cents** | Option<**i32**> |  | [optional]
**credit_amount_currency** | Option<**String**> |  | [optional]
**total_amount_cents** | Option<**i32**> |  | [optional]
**total_amount_currency** | Option<**String**> |  | [optional]
**legacy** | Option<**bool**> |  | [optional]
**file_url** | Option<**String**> |  | [optional]
**customer** | Option<[**crate::models::CustomerObject**](CustomerObject.md)> |  | [optional]
**subscriptions** | Option<[**Vec<crate::models::SubscriptionObject>**](SubscriptionObject.md)> |  | [optional]
**fees** | Option<[**Vec<crate::models::FeeObject>**](FeeObject.md)> |  | [optional]
**credits** | Option<[**Vec<crate::models::CreditObject>**](CreditObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


