# \RegistrationsApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_registration**](RegistrationsApi.md#create_registration) | **POST** /v2/registrations | Create Registration
[**delete_registration**](RegistrationsApi.md#delete_registration) | **DELETE** /v2/registrations/{registrationId} | Delete Registration
[**list_registrations**](RegistrationsApi.md#list_registrations) | **GET** /v2/registrations | List Registrations
[**retrieve_registration**](RegistrationsApi.md#retrieve_registration) | **GET** /v2/registrations/{registrationId} | Retrieve Registration
[**update_registration**](RegistrationsApi.md#update_registration) | **PATCH** /v2/registrations/{registrationId} | Update Registration



## create_registration

> create_registration(content_type, body)
Create Registration

Creates a new context provider registration. This is typically used for binding context sources as providers of certain data. The registration is represented by a JSON object as described at the beginning of this section. Response: * Successful operation uses 201 Created * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** |  | [required] |
**body** | [**CreateRegistrationRequest**](CreateRegistrationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_registration

> delete_registration(registration_id)
Delete Registration

Cancels a context provider registration. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | registration Id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_registrations

> Vec<crate::models::ListRegistrationsResponse> list_registrations(limit, offset, options)
List Registrations

Lists all the context provider registrations present in the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f64**> | Limit the number of registrations to be retrieved |  |
**offset** | Option<**f64**> | Skip a number of registrations |  |
**options** | Option<**String**> | Options dictionary |  |

### Return type

[**Vec<crate::models::ListRegistrationsResponse>**](ListRegistrationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_registration

> crate::models::RetrieveRegistrationResponse retrieve_registration(registration_id)
Retrieve Registration

The response is the registration represented by a JSON object as described at the beginning of this section. Response: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | registration Id. | [required] |

### Return type

[**crate::models::RetrieveRegistrationResponse**](RetrieveRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_registration

> update_registration(registration_id, content_type, body)
Update Registration

Only the fields included in the request are updated in the registration. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registration_id** | **String** | registration Id. | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateRegistrationRequest**](UpdateRegistrationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

