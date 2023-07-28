# \AttributeValueApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_attribute_value**](AttributeValueApi.md#get_attribute_value) | **GET** /v2/entities/{entityId}/attrs/{attrName}/value | Get Attribute Value
[**update_attribute_value**](AttributeValueApi.md#update_attribute_value) | **PUT** /v2/entities/{entityId}/attrs/{attrName}/value | Update Attribute Value



## get_attribute_value

> crate::models::GetAttributeValueResponse get_attribute_value(entity_id, attr_name, r#type)
Get Attribute Value

This operation returns the `value` property with the value of the attribute. * If attribute value is JSON Array or Object:   * If `Accept` header can be expanded to `application/json` or `text/plain` return the value as a JSON with a     response type of application/json or text/plain (whichever is the first in `Accept` header or     `application/json` in case of `Accept: *_/_*`).   * Else return a HTTP error \"406 Not Acceptable: accepted MIME types: application/json, text/plain\" * If attribute value is a string, number, null or boolean:   * If `Accept` header can be expanded to text/plain return the value as text. In case of a string, citation     marks are used at the begining and end.   * Else return a HTTP error \"406 Not Acceptable: accepted MIME types: text/plain\" Response: * Successful operation uses 200 OK. * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity in question | [required] |
**attr_name** | **String** | Name of the attribute to be retrieved. | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |

### Return type

[**crate::models::GetAttributeValueResponse**](GetAttributeValueResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_attribute_value

> update_attribute_value(entity_id, attr_name, content_type, body, r#type)
Update Attribute Value

The request payload is the new attribute value. * If the request payload MIME type is `application/json`, then the value of the attribute is set to   the JSON object or array coded in the payload (if the payload is not a valid JSON document,   then an error is returned). * If the request payload MIME type is `text/plain`, then the following algorithm is applied to the   payload:   * If the payload starts and ends with citation-marks (`\"`), the value is taken as a string     (the citation marks themselves are not considered part of the string)   * If `true` or `false`, the value is taken as a boolean.   * If `null`, the value is taken as null.   * If these first three tests 'fail', the text is interpreted as a number.   * If not a valid number, then an error is returned and the attribute's value is unchanged. The payload MIME type in the request is specified in the `Content-Type` HTTP header. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to be updated. | [required] |
**attr_name** | **String** | Attribute name. | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateAttributeValueRequest**](UpdateAttributeValueRequest.md) |  | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

