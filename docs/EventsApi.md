# \EventsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_batch_events**](EventsApi.md#create_batch_events) | **POST** /events/batch | Create batch events
[**create_event**](EventsApi.md#create_event) | **POST** /events | Create a new event
[**event_estimate_fees**](EventsApi.md#event_estimate_fees) | **POST** /events/estimate_fees | Estimate fees for an instant charge
[**find_event**](EventsApi.md#find_event) | **GET** /events/{id} | Find event by transaction ID



## create_batch_events

> create_batch_events(batch_event_input)
Create batch events

Create batch events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_event_input** | [**BatchEventInput**](BatchEventInput.md) | Batch events payload | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_event

> create_event(event_input)
Create a new event

Create a new event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_input** | [**EventInput**](EventInput.md) | Event payload | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_estimate_fees

> crate::models::Fees event_estimate_fees(event_estimate_fees_input)
Estimate fees for an instant charge

Estimate the fees that would be created after reception of an event for a billable metric attached to one or multiple instant charges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_estimate_fees_input** | [**EventEstimateFeesInput**](EventEstimateFeesInput.md) | Event payload for instant fee estimate | [required] |

### Return type

[**crate::models::Fees**](Fees.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_event

> crate::models::Event find_event(id)
Find event by transaction ID

Return a single event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the existing transaction | [required] |

### Return type

[**crate::models::Event**](Event.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

