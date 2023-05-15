# FeeObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**lago_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**lago_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**lago_true_up_fee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**lago_true_up_parent_fee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**external_subscription_id** | Option<**String**> |  | [optional]
**amount_cents** | **i32** |  | 
**amount_currency** | **String** |  | 
**vat_amount_cents** | **i32** |  | 
**vat_amount_currency** | **String** |  | 
**units** | **f32** |  | 
**total_amount_cents** | Option<**i32**> |  | [optional]
**total_amount_currency** | Option<**String**> |  | [optional]
**events_count** | Option<**i32**> |  | [optional]
**from_date** | Option<**String**> |  | [optional]
**to_date** | Option<**String**> |  | [optional]
**payment_status** | **String** |  | 
**created_at** | **String** |  | 
**succeeded_at** | Option<**String**> |  | [optional]
**failed_at** | Option<**String**> |  | [optional]
**refunded_at** | Option<**String**> |  | [optional]
**item** | [**crate::models::FeeObjectItem**](FeeObject_item.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


