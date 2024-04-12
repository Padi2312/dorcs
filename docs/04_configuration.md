---
title: Configuration
position: 4
---

# Configuration

Dorcs provides some settings that can be configured. The following are the default settings:

To override the default settings, create a `dorcs.config.json` file in the same directory where you execute the `dorcs` command.

## General Settings
| Property          | Type             | Description                                       | Default Value                           |
| ----------------- | ---------------- | ------------------------------------------------- | --------------------------------------- |
| `source`          | `string`, `null` | Path to the source directory.                     | `docs`                                  |
| `output`          | `string`, `null` | Path to the output directory.                     | `output`                                |
| `page_settings`   | `object`, `null` | Optional settings for the page, detailed below.   | see [Page Settings](#page-settings)     |
| `server_settings` | `object`, `null` | Optional settings for the server, detailed below. | see [Server Settings](#server-settings) |

## Page Settings

| Property     | Type             | Description                                                | Default Value   |
| ------------ | ---------------- | ---------------------------------------------------------- | --------------- |
| `icon`       | `string`, `null` | Path to the page's icon. If not provided, no icon is used. | `null`          |
| `page_title` | `string`, `null` | Title of the page. If not provided, no title is set.       | `Documentation` |

## Server Settings

| Property | Type             | Description                          | Default Value |
| -------- | ---------------- | ------------------------------------ | ------------- |
| `port`   | `number`, `null` | Port on which the server should run. | `8080`        |


# JSON Schema 

Dorcs provides a JSON Schema for the `dorcs.config.json` file. To enable suggestions, add the following line at the top of your `dorcs.config.json`:

```json
{
  "$schema": "https://dorcs.allthing.eu/dorcs.config.schema.json"
}
```

This line tells your editor to use the schema provided by Dorcs for IntelliSense. You should now get suggestions for the properties and their types.


# Full Example Configuration

_`dorcs.config.json`_

```json
{
  "$schema": "https://dorcs.allthing.eu/dorcs.config.schema.json",
  "source": "docs",
  "output": "output",
  "page_settings": {
    "page_title": "Documentation",
    "icon": ""
  },
  "server_settings": {
    "port": 8080 
  }
}
```
