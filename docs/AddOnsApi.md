# \AddOnsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_add_on**](AddOnsApi.md#apply_add_on) | **POST** /applied_add_ons | Apply an add-on to a customer
[**create_add_on**](AddOnsApi.md#create_add_on) | **POST** /add_ons | Create a new add-on
[**destroy_add_on**](AddOnsApi.md#destroy_add_on) | **DELETE** /add_ons/{code} | Delete an add-on
[**find_add_on**](AddOnsApi.md#find_add_on) | **GET** /add_ons/{code} | Find add-on by code
[**find_all_add_ons**](AddOnsApi.md#find_all_add_ons) | **GET** /add_ons | Find add-ons
[**update_add_on**](AddOnsApi.md#update_add_on) | **PUT** /add_ons/{code} | Update an existing add-on



## apply_add_on

> crate::models::AppliedAddOn apply_add_on(applied_add_on_input)
Apply an add-on to a customer

Apply an add-on to a customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applied_add_on_input** | [**AppliedAddOnInput**](AppliedAddOnInput.md) | Apply add-on payload | [required] |

### Return type

[**crate::models::AppliedAddOn**](AppliedAddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_add_on

> crate::models::AddOn create_add_on(add_on_input)
Create a new add-on

Create a new add-on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_on_input** | [**AddOnInput**](AddOnInput.md) | Add-on payload | [required] |

### Return type

[**crate::models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_add_on

> crate::models::AddOn destroy_add_on(code)
Delete an add-on

Delete an add-on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing add-on | [required] |

### Return type

[**crate::models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_add_on

> crate::models::AddOn find_add_on(code)
Find add-on by code

Return a single add-on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing add-on | [required] |

### Return type

[**crate::models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_add_ons

> crate::models::AddOnsPaginated find_all_add_ons(page, per_page)
Find add-ons

Find all add-ons in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::AddOnsPaginated**](AddOnsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_add_on

> crate::models::AddOn update_add_on(code, add_on_input)
Update an existing add-on

Update an existing add-on by code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing add-on | [required] |
**add_on_input** | [**AddOnInput**](AddOnInput.md) | Update an existing add-on | [required] |

### Return type

[**crate::models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

