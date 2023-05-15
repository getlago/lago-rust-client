# \CustomersApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer**](CustomersApi.md#create_customer) | **POST** /customers | Create a customer
[**delete_applied_coupon**](CustomersApi.md#delete_applied_coupon) | **DELETE** /customers/{customer_external_id}/applied_coupons/{applied_coupon_id} | Delete customer's appplied coupon
[**destroy_customer**](CustomersApi.md#destroy_customer) | **DELETE** /customers/{external_id} | Delete a customer
[**find_all_customers**](CustomersApi.md#find_all_customers) | **GET** /customers | Find customers
[**find_customer**](CustomersApi.md#find_customer) | **GET** /customers/{external_id} | Find customer by external ID
[**find_customer_current_usage**](CustomersApi.md#find_customer_current_usage) | **GET** /customers/{customer_external_id}/current_usage | Find customer current usage
[**get_customer_portal_url**](CustomersApi.md#get_customer_portal_url) | **GET** /customers/{customer_external_id}/portal_url | Get customer portal URL



## create_customer

> crate::models::Customer create_customer(customer_input)
Create a customer

Create a new customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_input** | [**CustomerInput**](CustomerInput.md) | Customer payload | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_applied_coupon

> crate::models::AppliedCoupon delete_applied_coupon(customer_external_id, applied_coupon_id)
Delete customer's appplied coupon

Delete customer's appplied coupon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_external_id** | **String** | External ID of the existing customer | [required] |
**applied_coupon_id** | **String** | Applied Coupon Lago ID | [required] |

### Return type

[**crate::models::AppliedCoupon**](AppliedCoupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_customer

> crate::models::Customer destroy_customer(external_id)
Delete a customer

Return the deleted customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_customers

> crate::models::CustomersPaginated find_all_customers(page, per_page)
Find customers

Find all customers in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::CustomersPaginated**](CustomersPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_customer

> crate::models::Customer find_customer(external_id)
Find customer by external ID

Return a single customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_customer_current_usage

> crate::models::CustomerUsage find_customer_current_usage(customer_external_id, external_subscription_id)
Find customer current usage

Return a customer current usage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_external_id** | **String** | External ID of the existing customer | [required] |
**external_subscription_id** | **String** | External subscription ID | [required] |

### Return type

[**crate::models::CustomerUsage**](CustomerUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customer_portal_url

> crate::models::GetCustomerPortalUrl200Response get_customer_portal_url(customer_external_id)
Get customer portal URL

Get customer portal URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_external_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**crate::models::GetCustomerPortalUrl200Response**](getCustomerPortalUrl_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

