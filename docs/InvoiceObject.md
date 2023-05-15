# InvoiceObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**sequential_id** | **i32** |  | 
**number** | **String** |  | 
**issuing_date** | **String** |  | 
**invoice_type** | **String** |  | 
**status** | **String** |  | 
**payment_status** | **String** |  | 
**currency** | **String** |  | 
**fees_amount_cents** | **i32** |  | 
**coupons_amount_cents** | **i32** |  | 
**credit_notes_amount_cents** | **i32** |  | 
**sub_total_vat_excluded_amount_cents** | **i32** |  | 
**vat_amount_cents** | **i32** |  | 
**sub_total_vat_included_amount_cents** | **i32** |  | 
**prepaid_credit_amount_cents** | **i32** |  | 
**total_amount_cents** | **i32** |  | 
**version_number** | **i32** |  | 
**amount_cents** | **i32** |  | 
**amount_currency** | **String** |  | 
**vat_amount_currency** | **String** |  | 
**credit_amount_cents** | **i32** |  | 
**credit_amount_currency** | **String** |  | 
**total_amount_currency** | **String** |  | 
**legacy** | **bool** |  | 
**file_url** | Option<**String**> |  | [optional]
**customer** | [**crate::models::CustomerObject**](CustomerObject.md) |  | 
**metadata** | Option<[**Vec<crate::models::InvoiceMetadataObject>**](InvoiceMetadataObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


