# \WalletsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_wallet**](WalletsApi.md#create_wallet) | **POST** /wallets | Create a new wallet
[**create_wallet_transaction**](WalletsApi.md#create_wallet_transaction) | **POST** /wallet_transactions | Create a new wallet transaction
[**destroy_wallet**](WalletsApi.md#destroy_wallet) | **DELETE** /wallets/{id} | Delete a wallet
[**find_all_wallets**](WalletsApi.md#find_all_wallets) | **GET** /wallets/ | Find wallets
[**find_wallet**](WalletsApi.md#find_wallet) | **GET** /wallets/{id} | Find wallet
[**update_wallet**](WalletsApi.md#update_wallet) | **PUT** /wallets/{id} | Update an existing wallet



## create_wallet

> crate::models::Wallet create_wallet(wallet_input)
Create a new wallet

Create a new wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_input** | [**WalletInput**](WalletInput.md) | Wallet payload | [required] |

### Return type

[**crate::models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_wallet_transaction

> crate::models::WalletTransaction create_wallet_transaction(wallet_transaction_input)
Create a new wallet transaction

Create a new wallet transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_transaction_input** | [**WalletTransactionInput**](WalletTransactionInput.md) | Wallet transaction payload | [required] |

### Return type

[**crate::models::WalletTransaction**](WalletTransaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_wallet

> crate::models::Wallet destroy_wallet(id)
Delete a wallet

Delete a wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lago ID of the existing wallet | [required] |

### Return type

[**crate::models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_wallets

> crate::models::Wallets find_all_wallets(external_customer_id, page, per_page)
Find wallets

Find all wallets for certain customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | External customer ID | [required] |
**page** | Option<**i32**> | Number of page |  |
**per_page** | Option<**i32**> | Number of records per page |  |

### Return type

[**crate::models::Wallets**](Wallets.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_wallet

> crate::models::Wallet find_wallet(id)
Find wallet

Return a wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lago ID of the existing wallet | [required] |

### Return type

[**crate::models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wallet

> crate::models::Wallet update_wallet(id, wallet_update_input)
Update an existing wallet

Update an existing wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lago ID of the existing wallet | [required] |
**wallet_update_input** | [**WalletUpdateInput**](WalletUpdateInput.md) | Update an existing wallet | [required] |

### Return type

[**crate::models::Wallet**](Wallet.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

