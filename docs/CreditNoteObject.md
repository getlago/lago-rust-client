# CreditNoteObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**sequential_id** | **i32** |  | 
**number** | **String** |  | 
**lago_invoice_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**invoice_number** | **String** |  | 
**issuing_date** | **String** |  | 
**credit_status** | Option<**String**> | Credit status | [optional]
**refund_status** | Option<**String**> | Refund status | [optional]
**reason** | **String** | Reason | 
**description** | Option<**String**> |  | [optional]
**currency** | **String** |  | 
**total_amount_cents** | **i32** |  | 
**total_amount_currency** | **String** |  | 
**vat_amount_cents** | **i32** |  | 
**vat_amount_currency** | **String** |  | 
**sub_total_vat_excluded_amount_cents** | **i32** |  | 
**sub_total_vat_excluded_amount_currency** | **String** |  | 
**balance_amount_cents** | **i32** |  | 
**balance_amount_currency** | **String** |  | 
**credit_amount_cents** | **i32** |  | 
**credit_amount_currency** | **String** |  | 
**refund_amount_cents** | **i32** |  | 
**refund_amount_currency** | **String** |  | 
**coupons_adjustement_amount_cents** | **i32** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**file_url** | Option<**String**> |  | [optional]
**items** | Option<[**Vec<crate::models::CreditNoteItemObject>**](CreditNoteItemObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


