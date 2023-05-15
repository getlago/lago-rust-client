# \SubscriptionsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /subscriptions | Assign a plan to a customer
[**destroy_subscription**](SubscriptionsApi.md#destroy_subscription) | **DELETE** /subscriptions/{external_id} | Terminate a subscription
[**find_all_subscriptions**](SubscriptionsApi.md#find_all_subscriptions) | **GET** /subscriptions | Find subscriptions
[**update_subscription**](SubscriptionsApi.md#update_subscription) | **PUT** /subscriptions/{external_id} | Update an existing subscription



## create_subscription

> crate::models::Subscription create_subscription(subscription_create_input)
Assign a plan to a customer

Assign a plan to a customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_create_input** | [**SubscriptionCreateInput**](SubscriptionCreateInput.md) | Subscription payload | [required] |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_subscription

> crate::models::Subscription destroy_subscription(external_id)
Terminate a subscription

Terminate a subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_subscriptions

> crate::models::SubscriptionsPaginated find_all_subscriptions(page, per_page, external_customer_id, plan_code)
Find subscriptions

Find all suscriptions for certain customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |
**external_customer_id** | Option<**String**> | External customer ID |  |
**plan_code** | Option<**String**> | Code of the plan attached to the subscription |  |

### Return type

[**crate::models::SubscriptionsPaginated**](SubscriptionsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> crate::models::Subscription update_subscription(external_id, subscription_update_input)
Update an existing subscription

Update an existing subscription by external ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing subscription | [required] |
**subscription_update_input** | [**SubscriptionUpdateInput**](SubscriptionUpdateInput.md) | Update an existing subscription | [required] |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

