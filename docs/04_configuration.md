---
title: Configuration
position: 4
---

# Configuration

Dorcs provides some settings that can be configured. The following are the default settings:

| Setting      | Description         | Type   | Default Value |
| ------------ | ------------------- | ------ | ------------- |
| `page_title` | Title in the navbar | string | Documentation |
| `source`     | Source directory    | string | ./docs        |
| `output`     | Output directory    | string | ./output      |


To override the default settings, create a `dorcs.config.json` file in the same directory where you execute the `dorcs` command.


**Example:**

_`dorcs.config.json`_

```json
{
  "page_title": "My Documentation",
  "source": "./my_docs",
  "output": "./my_output"
}
```
