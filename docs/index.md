---
title: Introduction
position: 1
---

# Dorcs

Dorcs is a static site generator that transforms markdown files into a website. It is designed to be simple and easy to use, requiring no external dependencies other than a single binary file.

It's currently in development and get more features in the future. For now it's suitable for small non complex projects.

## 🚀 Features

- ✅ Markdown to HTML conversion
- ✅ Table of contents generation
- ✅ Sidebar navigation with automatic links
- ✅ Page metadata like title and position in the sidebar
- [ ] Customizable templates
- [ ] Good table support
- [ ] Syntax highlighting for code blocks
- [ ] Support for images and media files
- [ ] Search functionality
- [ ] Support for multiple languages
- [ ] Plugin system for extending functionality

## Installation

### 📥 Binary

You can download the latest release from the [releases page](https://github.com/padi2312/dorcs/releases).

### 🔨 Build from Source

For those who prefer to build from source, ensure Rust and Cargo are properly set up, then follow these steps:

```sh
git clone https://github.com/padi2312/dorcs
cd dorcs
cargo build --release

./target/release/dorcs
```

## Usage

To generate documentation, run the following command in your project with the documentation folder:

```sh
dorcs
```

This will generate the documentation in the `output` directory.

## Configuration

Dorcs provides some settings that can be configured. The following are the default settings:

| Setting  | Description         | Type   | Default Value |
| -------- | ------------------- | ------ | ------------- |
| `title`  | Title in the navbar | string | Documentation |
| `source` | Source directory    | string | ./docs        |
