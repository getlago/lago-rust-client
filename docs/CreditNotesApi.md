# \CreditNotesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_credit_note**](CreditNotesApi.md#create_credit_note) | **POST** /credit_notes | Create a new Credit note
[**download_credit_note**](CreditNotesApi.md#download_credit_note) | **POST** /credit_notes/{id}/download | Download an existing credit note
[**find_all_credit_notes**](CreditNotesApi.md#find_all_credit_notes) | **GET** /credit_notes/ | Find Credit notes
[**find_credit_note**](CreditNotesApi.md#find_credit_note) | **GET** /credit_notes/{id} | Find credit note
[**update_credit_note**](CreditNotesApi.md#update_credit_note) | **PUT** /credit_notes/{id} | Update an existing credit note
[**void_credit_note**](CreditNotesApi.md#void_credit_note) | **PUT** /credit_notes/{id}/void | Void existing credit note



## create_credit_note

> crate::models::CreditNote create_credit_note(credit_note_input)
Create a new Credit note

Create a new credit note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_note_input** | [**CreditNoteInput**](CreditNoteInput.md) | Credit note payload | [required] |

### Return type

[**crate::models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_credit_note

> crate::models::CreditNote download_credit_note(id)
Download an existing credit note

Download an existing credit note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Credit note | [required] |

### Return type

[**crate::models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_credit_notes

> crate::models::CreditNotes find_all_credit_notes(page, per_page, external_customer_id)
Find Credit notes

Find all credit notes in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |
**external_customer_id** | Option<**String**> | External customer ID |  |

### Return type

[**crate::models::CreditNotes**](CreditNotes.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_credit_note

> crate::models::CreditNote find_credit_note(id)
Find credit note

Return a single credit note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the existing credit note | [required] |

### Return type

[**crate::models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_credit_note

> crate::models::CreditNote update_credit_note(id, credit_note_update_input)
Update an existing credit note

Update an existing credit note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the existing credit note | [required] |
**credit_note_update_input** | [**CreditNoteUpdateInput**](CreditNoteUpdateInput.md) | Update an existing credit note | [required] |

### Return type

[**crate::models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## void_credit_note

> crate::models::CreditNote void_credit_note(id)
Void existing credit note

Void an existing credit note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the existing Lago Credit note | [required] |

### Return type

[**crate::models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

