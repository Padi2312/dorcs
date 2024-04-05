---
title: Configuration
position: 4
---

# Configuration

Dorcs provides some settings that can be configured. The following are the default settings:

## General Settings
| Property        | Type             | Description                                                                                    | Default Value                       |
| --------------- | ---------------- | ---------------------------------------------------------------------------------------------- | ----------------------------------- |
| `output`        | `string`, `null` | Path to the output directory. If not provided, no output is specified.                         | `output`                            |
| `page_settings` | `object`, `null` | Optional settings for the page, detailed below. If not provided, no page settings are applied. | see [Page Settings](#page-settings) |
| `source`        | `string`, `null` | Path to the source directory. If not provided, no source is specified.                         | `docs`                              |

## Page Settings

The `page_settings` object, when provided, should conform to the following structure:

| Property     | Type             | Description                                                | Default Value   |
| ------------ | ---------------- | ---------------------------------------------------------- | --------------- |
| `icon`       | `string`, `null` | Path to the page's icon. If not provided, no icon is used. | `null`          |
| `page_title` | `string`, `null` | Title of the page. If not provided, no title is set.       | `Documentation` |


To override the default settings, create a `dorcs.config.json` file in the same directory where you execute the `dorcs` command.

## IntelliSense

Dorcs provides IntelliSense for the `dorcs.config.json` file. To enable IntelliSense, add the following line at the top of your `dorcs.config.json`:

```json
{
  "$schema": "https://dorcs.allthing.eu/dorcs.config.schema.json"
}
```

This line tells your editor to use the schema provided by Dorcs for IntelliSense. You should now get suggestions for the properties and their types.


## Full Example Configuration

_`dorcs.config.json`_

```json
{
  "$schema": "https://dorcs.allthing.eu/dorcs.config.schema.json",
  "source": "./my_docs",
  "output": "./my_output",
  "page_settings": {
    "page_title": "My Documentation",
    "icon": "./my_icon.png"
  }
}
```
