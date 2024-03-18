---
title: Configuration
position: 3
---

# Configuration

Dorcs provides some settings that can be configured. The following are the default settings:

| Setting  | Description         | Type   | Default Value |
| -------- | ------------------- | ------ | ------------- |
| `title`  | Title in the navbar | string | Documentation |
| `source` | Source directory    | string | ./docs        |
| `output` | Output directory    | string | ./output      |

You can override the default settings with the `dorcs.config.json` file. This file should be in the root of your project.

```json
{
  "title": "My Documentation",
  "source": "./my_docs",
  "output": "./my_output"
}
```

