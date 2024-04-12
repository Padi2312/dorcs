---
title: Usage
position: 3
---

# Usage

## Prerequisites

Before generating documentation, make sure you have the following:

- A `docs` directory in the same folder where you execute the `dorcs` command
- _**(Optional)**_ A `dorcs.config.json` file in the root of your project to customize the documentation's look and feel (See [Configuration](./04_configuration) for more details)

## Modes of Operation

### Build Mode
In build mode, Dorcs generates the documentation and saves it to the output directory. This is the default mode of operation.

```sh
dorcs
```

### Watch Mode
In watch mode, Dorcs watches the source directory for changes and regenerates the documentation whenever a change is detected. \
It also starts a local server to preview the generated documentation.

```sh
dorcs --watch
```

### Preview Mode
In preview mode, Dorcs starts a local server to preview the generated documentation.

```sh
dorcs --preview
```

## Structure of Documentation

### Special Files
- `[file].md`: Markdown files in the `docs` directory will be converted to HTML pages.
- `docs/index.md`: The entry point for the documentation. This file will be the landing page of your documentation.
- `docs/[...anysubdirs]/index.md` in subdirectories: The landing page for the respective section.

### Directory Structure
The following shows a sample directory structure for your documentation:
``` 
docs
├── index.md 
├── section1
│   ├── index.md
│   ├── file_in_section_one.md
│   
├── section2
│   ├── index.md
│   ├── another_markdown_file.md
```

### Metadata
You can add metadata to your markdown files to specify the title and position of the page in the sidebar. \
The metadata should be at the top of the markdown file and should be in the following format:

```markdown
---
title: Page Title
position: 1
---

# Your Content here
```

#### Tips
In order to have a document always at the top of the sidebar, set the `position` to a negative value. \
For exmaple you have a `Home` page, you can set the position to `-1` and other pages to `1`, `2`, `3`, `...`. \
The `Home` page is now always at the top of the sidebar.

