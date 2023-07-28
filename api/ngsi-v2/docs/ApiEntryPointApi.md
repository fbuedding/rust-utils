# \ApiEntryPointApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_api_resources**](ApiEntryPointApi.md#retrieve_api_resources) | **GET** /v2 | Retrieve API Resources



## retrieve_api_resources

> crate::models::RetrieveApiResourcesResponse retrieve_api_resources()
Retrieve API Resources

This resource does not have any attributes. Instead it offers the initial API affordances in the form of the links in the JSON body. It is recommended to follow the “url” link values, [Link](https://tools.ietf.org/html/rfc5988) or Location headers where applicable to retrieve resources. Instead of constructing your own URLs, to keep your client decoupled from implementation details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RetrieveApiResourcesResponse**](RetrieveApiResourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

