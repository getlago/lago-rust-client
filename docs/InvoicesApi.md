# \InvoicesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_invoice**](InvoicesApi.md#download_invoice) | **POST** /invoices/{id}/download | Download an existing invoice
[**finalize_invoice**](InvoicesApi.md#finalize_invoice) | **PUT** /invoices/{id}/finalize | Finalize a draft invoice
[**find_all_invoices**](InvoicesApi.md#find_all_invoices) | **GET** /invoices/ | Find all invoices
[**find_invoice**](InvoicesApi.md#find_invoice) | **GET** /invoices/{id} | Find invoice by ID
[**refresh_invoice**](InvoicesApi.md#refresh_invoice) | **PUT** /invoices/{id}/refresh | Refresh a draft invoice
[**retry_payment**](InvoicesApi.md#retry_payment) | **POST** /invoices/{id}/retry_payment | Retry invoice payment
[**update_invoice**](InvoicesApi.md#update_invoice) | **PUT** /invoices/{id} | Update an existing invoice status



## download_invoice

> crate::models::Invoice download_invoice(id)
Download an existing invoice

Download an existing invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Invoice | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finalize_invoice

> crate::models::Invoice finalize_invoice(id)
Finalize a draft invoice

Finalize a draft invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the draft Lago Invoice | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_invoices

> crate::models::Invoices find_all_invoices(page, per_page, external_customer_id, issuing_date_from, issuing_date_to, status)
Find all invoices

Find all invoices in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |
**external_customer_id** | Option<**String**> | External customer ID |  |
**issuing_date_from** | Option<**String**> | Date from |  |
**issuing_date_to** | Option<**String**> | Date to |  |
**status** | Option<**String**> | Status (draft or finalized) |  |

### Return type

[**crate::models::Invoices**](Invoices.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_invoice

> crate::models::Invoice find_invoice(id)
Find invoice by ID

Return a single invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Invoice | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_invoice

> crate::models::Invoice refresh_invoice(id)
Refresh a draft invoice

Refresh a draft invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Invoice | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_payment

> retry_payment(id)
Retry invoice payment

Retry invoice payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Invoice | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_invoice

> crate::models::Invoice update_invoice(id, invoice_input)
Update an existing invoice status

Update an existing invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Invoice | [required] |
**invoice_input** | [**InvoiceInput**](InvoiceInput.md) | Update an existing invoice | [required] |

### Return type

[**crate::models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

