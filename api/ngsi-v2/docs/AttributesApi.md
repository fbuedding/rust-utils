# \AttributesApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_attribute_data**](AttributesApi.md#get_attribute_data) | **GET** /v2/entities/{entityId}/attrs/{attrName} | Get attribute data
[**remove_a_single_attribute**](AttributesApi.md#remove_a_single_attribute) | **DELETE** /v2/entities/{entityId}/attrs/{attrName} | Remove a Single Attribute
[**update_attribute_data**](AttributesApi.md#update_attribute_data) | **PUT** /v2/entities/{entityId}/attrs/{attrName} | Update Attribute Data



## get_attribute_data

> crate::models::GetAttributeDataResponse get_attribute_data(entity_id, attr_name, r#type, metadata)
Get attribute data

Returns a JSON object with the attribute data of the attribute. The object follows the JSON representation for attributes (described in \"JSON Attribute Representation\" section). Response: * Successful operation uses 200 OK. * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity | [required] |
**attr_name** | **String** | Name of the attribute to be retrieved. | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**metadata** | Option<**String**> | A list of metadata names to include in the response. See \"Filtering out attributes and metadata\" section for more detail. |  |

### Return type

[**crate::models::GetAttributeDataResponse**](GetAttributeDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_a_single_attribute

> remove_a_single_attribute(entity_id, attr_name, r#type)
Remove a Single Attribute

Removes an entity attribute. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity. | [required] |
**attr_name** | **String** | Attribute name. | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_attribute_data

> update_attribute_data(entity_id, attr_name, content_type, body, r#type)
Update Attribute Data

The request payload is an object representing the new attribute data. Previous attribute data is replaced by the one in the request. The object follows the JSON representation for attributes (described in \"JSON Attribute Representation\" section). Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to update | [required] |
**attr_name** | **String** | Attribute name | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateAttributeDataRequest**](UpdateAttributeDataRequest.md) |  | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

