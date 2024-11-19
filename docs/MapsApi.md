# \MapsApi

All URIs are relative to *https://war-service-live.foxholeservices.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_map_dynamic**](MapsApi.md#get_map_dynamic) | **GET** /worldconquest/maps/{mapName}/dynamic/public | Dynamic map data includes map icons that could change over the lifecycle of a map. This includes static bases and static base build sites.
[**get_map_static**](MapsApi.md#get_map_static) | **GET** /worldconquest/maps/{mapName}/static | Static map data includes things that never change over the lifecycle of a map. This includes map text labels, resource nodes, and world structures.
[**get_maps**](MapsApi.md#get_maps) | **GET** /worldconquest/maps | Returns a list of the active World Conquest map names.
[**get_war_report**](MapsApi.md#get_war_report) | **GET** /worldconquest/warReport/{mapName} | Returns the number of enlistments, casualties, and other map specific information.



## get_map_dynamic

> models::Map get_map_dynamic(map_name, if_none_match)
Dynamic map data includes map icons that could change over the lifecycle of a map. This includes static bases and static base build sites.

<p>Team-specific data and forward bases are excluded.</p><i>This data may update every 3 seconds.</i>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map_name** | **String** | Name of the map | [required] |
**if_none_match** | Option<**String**> | https://datatracker.ietf.org/doc/html/rfc7232 |  |

### Return type

[**models::Map**](Map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_map_static

> models::Map get_map_static(map_name, if_none_match)
Static map data includes things that never change over the lifecycle of a map. This includes map text labels, resource nodes, and world structures.

<p></p><i>You only need to request this once per map between World Conquests.</i>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map_name** | **String** | Name of the map | [required] |
**if_none_match** | Option<**String**> | https://datatracker.ietf.org/doc/html/rfc7232 |  |

### Return type

[**models::Map**](Map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_maps

> Vec<String> get_maps()
Returns a list of the active World Conquest map names.

<p>Note: The maps HomeRegionC and HomeRegionW are returned here, but do not have map data available in this version.</p>

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_war_report

> models::WarReport get_war_report(map_name, if_none_match)
Returns the number of enlistments, casualties, and other map specific information.

<p></p><i>This data may update every 3 seconds.</i>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map_name** | **String** | Name of the map | [required] |
**if_none_match** | Option<**String**> | https://datatracker.ietf.org/doc/html/rfc7232 |  |

### Return type

[**models::WarReport**](WarReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

