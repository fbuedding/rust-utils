# \SubscriptionsApi

All URIs are relative to *http://orion.lab.fiware.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /v2/subscriptions | Create Subscription
[**delete_subscription**](SubscriptionsApi.md#delete_subscription) | **DELETE** /v2/subscriptions/{subscriptionId} | Delete subscription
[**list_subscriptions**](SubscriptionsApi.md#list_subscriptions) | **GET** /v2/subscriptions | List Subscriptions
[**retrieve_subscription**](SubscriptionsApi.md#retrieve_subscription) | **GET** /v2/subscriptions/{subscriptionId} | Retrieve Subscription
[**update_subscription**](SubscriptionsApi.md#update_subscription) | **PATCH** /v2/subscriptions/{subscriptionId} | Update Subscription



## create_subscription

> create_subscription(content_type, body)
Create Subscription

Creates a new subscription. The subscription is represented by a JSON object as described at the beginning of this section. Response: * Successful operation uses 201 Created * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | **String** |  | [required] |
**body** | [**CreateSubscriptionRequest**](CreateSubscriptionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> delete_subscription(subscription_id)
Delete subscription

Cancels subscription. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | subscription Id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriptions

> Vec<crate::models::ListSubscriptionsResponse> list_subscriptions(limit, offset, options)
List Subscriptions

Returns a list of all the subscriptions present in the system. Response: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f64**> | Limit the number of subscriptions to be retrieved |  |
**offset** | Option<**f64**> | Skip a number of subscriptions |  |
**options** | Option<**String**> | Options dictionary |  |

### Return type

[**Vec<crate::models::ListSubscriptionsResponse>**](ListSubscriptionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_subscription

> serde_json::Value retrieve_subscription(subscription_id)
Retrieve Subscription

The response is the subscription represented by a JSON object as described at the beginning of this section. Response: * Successful operation uses 200 OK * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | subscription Id. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> update_subscription(subscription_id, content_type, body)
Update Subscription

Only the fields included in the request are updated in the subscription. Response: * Successful operation uses 204 No Content * Errors use a non-2xx and (optionally) an error payload. See subsection on \"Error Responses\" for   more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | subscription Id. | [required] |
**content_type** | **String** |  | [required] |
**body** | [**UpdateSubscriptionRequest**](UpdateSubscriptionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

