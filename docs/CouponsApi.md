# \CouponsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_coupon**](CouponsApi.md#apply_coupon) | **POST** /applied_coupons | Apply a coupon to a customer
[**create_coupon**](CouponsApi.md#create_coupon) | **POST** /coupons | Create a new coupon
[**destroy_coupon**](CouponsApi.md#destroy_coupon) | **DELETE** /coupons/{code} | Delete a coupon
[**find_all_applied_coupons**](CouponsApi.md#find_all_applied_coupons) | **GET** /applied_coupons/ | Find Applied Coupons
[**find_all_coupons**](CouponsApi.md#find_all_coupons) | **GET** /coupons/ | Find Coupons
[**find_coupon**](CouponsApi.md#find_coupon) | **GET** /coupons/{code} | Find coupon by code
[**update_coupon**](CouponsApi.md#update_coupon) | **PUT** /coupons/{code} | Update an existing coupon



## apply_coupon

> crate::models::AppliedCoupon apply_coupon(applied_coupon_input)
Apply a coupon to a customer

Apply a coupon to a customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applied_coupon_input** | [**AppliedCouponInput**](AppliedCouponInput.md) | Apply coupon payload | [required] |

### Return type

[**crate::models::AppliedCoupon**](AppliedCoupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_coupon

> crate::models::Coupon create_coupon(coupon_input)
Create a new coupon

Create a new coupon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_input** | [**CouponInput**](CouponInput.md) | Coupon payload | [required] |

### Return type

[**crate::models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_coupon

> crate::models::Coupon destroy_coupon(code)
Delete a coupon

Delete a coupon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing coupon | [required] |

### Return type

[**crate::models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_applied_coupons

> crate::models::AppliedCoupons find_all_applied_coupons(page, per_page, status, external_customer_id)
Find Applied Coupons

Find all applied coupons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |
**status** | Option<**String**> | Applied coupon status |  |
**external_customer_id** | Option<**String**> | External customer ID |  |

### Return type

[**crate::models::AppliedCoupons**](AppliedCoupons.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_coupons

> crate::models::Coupons find_all_coupons(page, per_page)
Find Coupons

Find all coupons in certain organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::Coupons**](Coupons.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_coupon

> crate::models::Coupon find_coupon(code)
Find coupon by code

Return a single coupon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing coupon | [required] |

### Return type

[**crate::models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_coupon

> crate::models::Coupon update_coupon(code, coupon_input)
Update an existing coupon

Update an existing coupon by code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Code of the existing coupon | [required] |
**coupon_input** | [**CouponInput**](CouponInput.md) | Update an existing coupon | [required] |

### Return type

[**crate::models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

