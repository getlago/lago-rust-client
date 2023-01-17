# \BillableMetricsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billable_metric**](BillableMetricsApi.md#create_billable_metric) | **POST** /billable_metrics | Create a new billable metric
[**destroy_billable_metric**](BillableMetricsApi.md#destroy_billable_metric) | **DELETE** /billable_metrics/{code} | Delete a billable metric
[**find_all_billable_metric_groups**](BillableMetricsApi.md#find_all_billable_metric_groups) | **GET** /billable_metrics/{code}/groups | Find Billable metric groups
[**find_all_billable_metrics**](BillableMetricsApi.md#find_all_billable_metrics) | **GET** /billable_metrics/ | Find Billable metrics
[**find_billable_metric**](BillableMetricsApi.md#find_billable_metric) | **GET** /billable_metrics/{code} | Find billable metric by code
[**update_billable_metric**](BillableMetricsApi.md#update_billable_metric) | **PUT** /billable_metrics/{code} | Update an existing billable metric



## create_billable_metric

> crate::models::BillableMetric create_billable_metric(billable_metric_input)
Create a new billable metric

Create a new billable metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billable_metric_input** | [**BillableMetricInput**](BillableMetricInput.md) | Billable metric payload | [required] |

### Return type

[**crate::models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_billable_metric

> crate::models::BillableMetric destroy_billable_metric(code)
Delete a billable metric

Delete a billable metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric | [required] |

### Return type

[**crate::models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_billable_metric_groups

> crate::models::Groups find_all_billable_metric_groups(code, page, per_page)
Find Billable metric groups

Find all billable metric groups in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric | [required] |
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::Groups**](Groups.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_billable_metrics

> crate::models::BillableMetrics find_all_billable_metrics(page, per_page)
Find Billable metrics

Find all billable metrics in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::BillableMetrics**](BillableMetrics.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_billable_metric

> crate::models::BillableMetric find_billable_metric(code)
Find billable metric by code

Return a single billable metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric | [required] |

### Return type

[**crate::models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_billable_metric

> crate::models::BillableMetric update_billable_metric(code, billable_metric_input)
Update an existing billable metric

Update an existing billable metric by code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing billable metric | [required] |
**billable_metric_input** | [**BillableMetricInput**](BillableMetricInput.md) | Update an existing billable metric | [required] |

### Return type

[**crate::models::BillableMetric**](BillableMetric.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

