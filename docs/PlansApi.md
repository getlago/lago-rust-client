# \PlansApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_plan**](PlansApi.md#create_plan) | **POST** /plans | Create a new plan
[**destroy_plan**](PlansApi.md#destroy_plan) | **DELETE** /plans/{code} | Delete a plan
[**find_all_plans**](PlansApi.md#find_all_plans) | **GET** /plans | Find plans
[**find_plan**](PlansApi.md#find_plan) | **GET** /plans/{code} | Find plan by code
[**update_plan**](PlansApi.md#update_plan) | **PUT** /plans/{code} | Update an existing plan



## create_plan

> crate::models::Plan create_plan(plan_input)
Create a new plan

Create a new plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_input** | [**PlanInput**](PlanInput.md) | Plan payload | [required] |

### Return type

[**crate::models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_plan

> crate::models::Plan destroy_plan(code)
Delete a plan

Delete a plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing plan | [required] |

### Return type

[**crate::models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_plans

> crate::models::PlansPaginated find_all_plans(page, per_page)
Find plans

Find all plans in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::PlansPaginated**](PlansPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plan

> crate::models::Plan find_plan(code)
Find plan by code

Return a single plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing plan | [required] |

### Return type

[**crate::models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plan

> crate::models::Plan update_plan(code, plan_input)
Update an existing plan

Update an existing plan by code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing plan | [required] |
**plan_input** | [**PlanInput**](PlanInput.md) | Update an existing plan | [required] |

### Return type

[**crate::models::Plan**](Plan.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

