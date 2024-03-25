---
title: Usage
position: 3
---

# Usage

## Prerequisites

Before generating documentation, make sure you have the following:

- A `docs` directory where you will execute the `dorcs` 
- **(Optional)** A `dorcs.config.json` file in the root of your project to customize the documentation's look and feel (See [Configuration](./04_configuration) for more details)

## Writing the documentation

- To write documentation, create markdown files (`.md`) in the `docs` directory. 
- Make sure to include all necessary media files in `docs` directory or subdirectories.

> **NOTE:** You must have a `docs/index.md` file as the entry point for the documentation. Otherwise there will be a blank page.

> **NOTE:** When linking documents in the markdown files, avoid using the `.md` extension. The links should be relative to the `docs` directory. For example, use `/another_markdown_file` instead of `/another_markdown_file.md`

## Generating Documentation
To generate documentation, run the following command in your project's documentation folder:

```bash
dorcs
```