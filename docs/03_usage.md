---
title: Usage
position: 3
---

# Usage

## Prerequisites

- You need to have a `docs` directory where you will execute the `dorcs` command
- Make sure all necessary markdown files **and** images used in the markdown files are present in the `docs` directory.
- (Optional) If you want to customize the look and feel of the documentation, you can create a `dorcs.config.json` file in the root of your project. (See [Configuration](./03_configuration) for more details)

> **NOTE:** If you link documents in the markdown files make sure to not use the `.md` extension. The links should be relative to the `docs` directory. 
>
> 
> For example: `/another_markdown_file` instead of `/another_markdown_file.md`

## Generating Documentation

To generate documentation, run the following command in your project with the documentation folder:

```sh
dorcs
```

This will generate the documentation in the `output` directory or at the location specified in the `output` property of the `dorcs.config.json` file.
