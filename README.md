<p align="center">
  <img src="./docs/dorcs_logo.png" alt="Dorcs Logo" width="200">
</p>

<h1 align="center">Dorcs</h1>

<p align="center">
  <strong>Static site generator in a single file 📄</strong>
</p>

This project is a documentation generator written in Rust. It reads markdown files from a specified directory, converts them to HTML, and saves the generated HTML files to another specified directory.

## 🚀 Features

- **Markdown to HTML Conversion:** The project reads markdown files and converts them to HTML. This is done in the documents loop in the generate_docs function of the Generator struct.

- **Sidebar navigation:** Dorcs automatically generates links for navigating between the different pages

- **Page Metadata:** Provide a title for a certain page. You can also provide the position of the page in the sidebar.

## 🛠️ Setup

### 📥 Binary

Download pre-built binaries from the releases section for a quick start.

### 🔨 Build from Source

For those who prefer to build from source, ensure Rust and Cargo are properly set up, then follow these steps:

```sh
git clone https://github.com/padi2312/dorcs.git
cd dorcs
cargo build --release

./target/release/dorcs
```

## 📚 Usage

To generate documentation, run the following command in your project with the documentation folder:

```sh
dorcs
```

This will generate the documentation in the `output` directory.


## ⚙️ Configuration

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

