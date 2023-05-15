# \OrganizationsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_organization**](OrganizationsApi.md#update_organization) | **PUT** /organizations | Update an existing Organization



## update_organization

> crate::models::Organization update_organization(organization_input)
Update an existing Organization

Update an existing organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_input** | [**OrganizationInput**](OrganizationInput.md) | Update an existing organization | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

