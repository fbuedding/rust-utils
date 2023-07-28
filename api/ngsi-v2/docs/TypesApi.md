# \TypesApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_entity_types**](TypesApi.md#list_entity_types) | **GET** /v2/types/ | List Entity Types
[**retrieve_entity_type**](TypesApi.md#retrieve_entity_type) | **GET** /v2/types/{entityType} | Retrieve entity type



## list_entity_types

> Vec<crate::models::ListEntityTypesResponse> list_entity_types(limit, offset, options)
List Entity Types

If the `values` option is not in use, this operation returns a JSON array with the entity types. Each element is a JSON object with information about the type: * `type` : the entity type name. * `attrs` : the set of attribute names along with all the entities of such type, represented in   a JSON object whose keys are the attribute names and whose values contain information of such   attributes (in particular a list of the types used by attributes with that name along with all the   entities). * `count` : the number of entities belonging to that type. If the `values` option is used, the operation returns a JSON array with a list of entity type names as strings. Results are ordered by entity `type` in alphabetical order. Response code: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f64**> | Limit the number of types to be retrieved. |  |
**offset** | Option<**f64**> | Skip a number of records. |  |
**options** | Option<**String**> | Options dictionary. |  |

### Return type

[**Vec<crate::models::ListEntityTypesResponse>**](ListEntityTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity_type

> crate::models::RetrieveEntityTypeResponse retrieve_entity_type(entity_type)
Retrieve entity type

This operation returns a JSON object with information about the type: * `attrs` : the set of attribute names along with all the entities of such type, represented in   a JSON object whose keys are the attribute names and whose values contain information of such   attributes (in particular a list of the types used by attributes with that name along with all the   entities). * `count` : the number of entities belonging to that type. Response code: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type** | **String** | Entity Type | [required] |

### Return type

[**crate::models::RetrieveEntityTypeResponse**](RetrieveEntityTypeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

