# \EntitiesApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_entity**](EntitiesApi.md#create_entity) | **POST** /v2/entities | Create Entity
[**list_entities**](EntitiesApi.md#list_entities) | **GET** /v2/entities | List Entities
[**remove_entity**](EntitiesApi.md#remove_entity) | **DELETE** /v2/entities/{entityId} | Remove Entity
[**replace_all_entity_attributes**](EntitiesApi.md#replace_all_entity_attributes) | **PUT** /v2/entities/{entityId}/attrs | Replace all entity attributes
[**retrieve_entity**](EntitiesApi.md#retrieve_entity) | **GET** /v2/entities/{entityId} | Retrieve Entity
[**retrieve_entity_attributes**](EntitiesApi.md#retrieve_entity_attributes) | **GET** /v2/entities/{entityId}/attrs | Retrieve Entity Attributes
[**update_existing_entity_attributes**](EntitiesApi.md#update_existing_entity_attributes) | **PATCH** /v2/entities/{entityId}/attrs | Update Existing Entity Attributes
[**update_or_append_entity_attributes**](EntitiesApi.md#update_or_append_entity_attributes) | **POST** /v2/entities/{entityId}/attrs | Update or Append Entity Attributes



## create_entity

> create_entity(content_type, body, options)
Create Entity

The payload is an object representing the entity to be created. The object follows the JSON entity representation format (described in a \"JSON Entity Representation\" section). Response: * Successful operation uses 201 Created (if upsert option is not used) or 204 No Content (if   upsert option is used). Response includes a `Location` header with the URL of the   created entity. * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** |  | [required] |
**body** | [**CreateEntityRequest**](CreateEntityRequest.md) |  | [required] |
**options** | Option<**String**> | Options dictionary |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_entities

> Vec<crate::models::ListEntitiesResponse> list_entities(id, r#type, id_pattern, type_pattern, q, mq, georel, geometry, coords, limit, offset, attrs, metadata, order_by, options)
List Entities

Retrieves a list of entities that match different criteria by id, type, pattern matching (either id or type) and/or those which match a query or geographical query (see [Simple Query Language](#simple_query_language) and  [Geographical Queries](#geographical_queries)). A given entity has to match all the criteria to be retrieved (i.e., the criteria is combined in a logical AND way). Note that pattern matching query parameters are incompatible (i.e. mutually exclusive) with their corresponding exact matching parameters, i.e. `idPattern` with `id` and `typePattern` with `type`. The response payload is an array containing one object per matching entity. Each entity follows the JSON entity representation format (described in \"JSON Entity Representation\" section). Response code: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | A comma-separated list of elements. Retrieve entities whose ID matches one of the elements in the list. Incompatible with `idPattern`. |  |
**r#type** | Option<**String**> | comma-separated list of elements. Retrieve entities whose type matches one of the elements in the list. Incompatible with `typePattern`. |  |
**id_pattern** | Option<**String**> | A correctly formated regular expression. Retrieve entities whose ID matches the regular expression. Incompatible with `id`. |  |
**type_pattern** | Option<**String**> | A correctly formated regular expression. Retrieve entities whose type matches the regular expression. Incompatible with `type`. |  |
**q** | Option<**String**> | A query expression, composed of a list of statements separated by `;`, i.e., q=statement1;statement2;statement3. See [Simple Query Language specification](#simple_query_language). |  |
**mq** | Option<**String**> | A query expression for attribute metadata, composed of a list of statements separated by `;`, i.e., mq=statement1;statement2;statement3. See [Simple Query Language specification](#simple_query_language). |  |
**georel** | Option<**String**> | Spatial relationship between matching entities and a reference shape. See [Geographical Queries](#geographical_queries). |  |
**geometry** | Option<**String**> | Geografical area to which the query is restricted. See [Geographical Queries](#geographical_queries). |  |
**coords** | Option<**String**> | List of latitude-longitude pairs of coordinates separated by ';'. See [Geographical Queries](#geographical_queries). |  |
**limit** | Option<**f64**> | Limits the number of entities to be retrieved |  |
**offset** | Option<**f64**> | Establishes the offset from where entities are retrieved |  |
**attrs** | Option<**String**> | Comma-separated list of attribute names whose data are to be included in the response. The attributes are retrieved in the order specified by this parameter. If this parameter is not included, the attributes are retrieved in arbitrary order. See \"Filtering out attributes and metadata\" section for more detail. |  |
**metadata** | Option<**String**> | A list of metadata names to include in the response. See \"Filtering out attributes and metadata\" section for more detail. |  |
**order_by** | Option<**String**> | Criteria for ordering results. See \"Ordering Results\" section for details. |  |
**options** | Option<**String**> | Options dictionary |  |

### Return type

[**Vec<crate::models::ListEntitiesResponse>**](ListEntitiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_entity

> remove_entity(entity_id, r#type)
Remove Entity

Delete the entity. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to be deleted | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_entity_attributes

> replace_all_entity_attributes(entity_id, content_type, body, r#type, options)
Replace all entity attributes

The request payload is an object representing the new entity attributes. The object follows the JSON entity representation format (described in a \"JSON Entity Representation\" above), except that `id` and `type` are not allowed. The attributes previously existing in the entity are removed and replaced by the ones in the request. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity in question. | [required] |
**content_type** | **String** |  | [required] |
**body** | [**ReplaceAllEntityAttributesRequest**](ReplaceAllEntityAttributesRequest.md) |  | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**options** | Option<**String**> | Operations options |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity

> crate::models::RetrieveEntityResponse retrieve_entity(entity_id, r#type, attrs, metadata, options)
Retrieve Entity

The response is an object representing the entity identified by the ID. The object follows the JSON entity representation format (described in \"JSON Entity Representation\" section). This operation must return one entity element only, but there may be more than one entity with the same ID (e.g. entities with same ID but different types). In such case, an error message is returned, with the HTTP status code set to 409 Conflict. Response: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to be retrieved | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**attrs** | Option<**String**> | Comma-separated list of attribute names whose data must be included in the response. The attributes are retrieved in the order specified by this parameter. See \"Filtering out attributes and metadata\" section for more detail. If this parameter is not included, the attributes are retrieved in arbitrary order, and all the attributes of the entity are included in the response. |  |
**metadata** | Option<**String**> | A list of metadata names to include in the response. See \"Filtering out attributes and metadata\" section for more detail. |  |
**options** | Option<**String**> | Options dictionary |  |

### Return type

[**crate::models::RetrieveEntityResponse**](RetrieveEntityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity_attributes

> crate::models::RetrieveEntityAttributesResponse retrieve_entity_attributes(entity_id, r#type, attrs, metadata, options)
Retrieve Entity Attributes

This request is similar to retreiving the whole entity, however this one omits the `id` and `type` fields. Just like the general request of getting an entire entity, this operation must return only one entity element. If more than one entity with the same ID is found (e.g. entities with same ID but different type), an error message is returned, with the HTTP status code set to 409 Conflict. Response: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to be retrieved | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**attrs** | Option<**String**> | Comma-separated list of attribute names whose data are to be included in the response. The attributes are retrieved in the order specified by this parameter. If this parameter is not included, the attributes are retrieved in arbitrary order, and all the attributes of the entity are included in the response. See \"Filtering out attributes and metadata\" section for more detail. |  |
**metadata** | Option<**String**> | A list of metadata names to include in the response. See \"Filtering out attributes and metadata\" section for more detail. |  |
**options** | Option<**String**> | Options dictionary |  |

### Return type

[**crate::models::RetrieveEntityAttributesResponse**](RetrieveEntityAttributesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_existing_entity_attributes

> update_existing_entity_attributes(entity_id, content_type, body, r#type, options)
Update Existing Entity Attributes

The request payload is an object representing the attributes to update. The object follows the JSON entity representation format (described in \"JSON Entity Representation\" section), except that `id` and `type` are not allowed. The entity attributes are updated with the ones in the payload. In addition to that, if one or more attributes in the payload doesn't exist in the entity, an error is returned. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Id of the entity to be updated | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateExistingEntityAttributesRequest**](UpdateExistingEntityAttributesRequest.md) |  | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**options** | Option<**String**> | Operations options |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_or_append_entity_attributes

> update_or_append_entity_attributes(entity_id, content_type, body, r#type, options)
Update or Append Entity Attributes

The request payload is an object representing the attributes to append or update. The object follows the JSON entity representation format (described in \"JSON Entity Representation\" section), except that `id` and `type` are not allowed. The entity attributes are updated with the ones in the payload, depending on whether the `append` operation option is used or not. * If `append` is not used: the entity attributes are updated (if they previously exist) or appended   (if they don't previously exist) with the ones in the payload. * If `append` is used (i.e. strict append semantics): all the attributes in the payload not   previously existing in the entity are appended. In addition to that, in case some of the   attributes in the payload already exist in the entity, an error is returned. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | Entity id to be updated | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateOrAppendEntityAttributesRequest**](UpdateOrAppendEntityAttributesRequest.md) |  | [required] |
**r#type** | Option<**String**> | Entity type, to avoid ambiguity in case there are several entities with the same entity id. |  |
**options** | Option<**String**> | Operations options |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

