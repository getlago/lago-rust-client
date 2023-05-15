# \FeesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_all_fees**](FeesApi.md#find_all_fees) | **GET** /fees | Find all fees
[**find_fee**](FeesApi.md#find_fee) | **GET** /fees/{id} | Find fee by ID
[**update_fee**](FeesApi.md#update_fee) | **PUT** /fees/{id} | Update an existing fee



## find_all_fees

> crate::models::FeesPaginated find_all_fees(page, per_page, external_customer_id, external_subscription_id, currency, fee_type, billable_metric_code, payment_status, created_at_from, created_at_to, succeeded_at_from, succeeded_at_to, failed_at_from, failed_at_to, refunded_at_from, refunded_at_to)
Find all fees

Find all fees of an organization and filter them

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |
**external_customer_id** | Option<**String**> | External customer ID |  |
**external_subscription_id** | Option<**String**> | External subscription ID |  |
**currency** | Option<**String**> | Amount currency |  |
**fee_type** | Option<**String**> | Fee type |  |
**billable_metric_code** | Option<**String**> | Code of the source billable metric |  |
**payment_status** | Option<**String**> | Payment status |  |
**created_at_from** | Option<**String**> | Creation datetime from |  |
**created_at_to** | Option<**String**> | Creation date to |  |
**succeeded_at_from** | Option<**String**> | Payment succees date from |  |
**succeeded_at_to** | Option<**String**> | Payment succees date to |  |
**failed_at_from** | Option<**String**> | Payment failed date from |  |
**failed_at_to** | Option<**String**> | Payment failed date to |  |
**refunded_at_from** | Option<**String**> | Payment refund date from |  |
**refunded_at_to** | Option<**String**> | Payment refund date to |  |

### Return type

[**crate::models::FeesPaginated**](FeesPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_fee

> crate::models::FeeObject find_fee(id)
Find fee by ID

Return a single fee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the existing Lago Fee | [required] |

### Return type

[**crate::models::FeeObject**](FeeObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_fee

> crate::models::FeeObject update_fee(id, fee_update_input)
Update an existing fee

Update an existing fee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the existing Lago Fee | [required] |
**fee_update_input** | Option<[**FeeUpdateInput**](FeeUpdateInput.md)> | Payload to update a fee |  |

### Return type

[**crate::models::FeeObject**](FeeObject.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

