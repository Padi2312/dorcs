<p align="center">
  <img src="./docs/dorcs_logo.png" alt="Dorcs Logo" width="200">
</p>

<h1 align="center">Dorcs</h1>

<p align="center">
  <strong>Static site generator in a single file 📄</strong>
</p>

This project is a documentation generator written in Rust. It reads markdown files from a specified directory, converts them to HTML, and saves the generated HTML files to another specified directory.

## 🖥️ Demo

See Dorcs in action [here](https://dorcs.allthing.eu). (same link as the documentation)

## 🚀 Features

- **Markdown to HTML Conversion:** The project reads markdown files and converts them to HTML. This is done in the documents loop in the generate_docs function of the Generator struct.

- **Sidebar navigation:** Dorcs automatically generates links for navigating between the different pages

- **Page Metadata:** Provide a title for a certain page. You can also provide the position of the page in the sidebar.

- **Hot Reload:** In watch mode, Dorcs watches the source directory for changes and regenerates the documentation whenever a change is detected.

- **Built-in Server:** Dorcs starts a local server to preview the generated documentation.

## 📄 Documentation

The documentatoin is available at **[dorcs.allthing.eu](https://dorcs.allthing.eu).**

The website is generated using Dorcs itself, you can find the source code for the documentation inside the `docs` directory.


## 🛠️ Setup

### 📥 Binary

Download pre-built binaries from the [releases section](https://github.com/Padi2312/dorcs/releases) for a quick start.

### 🛠️ Build from Source

To build from source, you need to have Go(lang) installed.

Clone the repository and run the following commands inside:

```sh
go generate ./build/gen.go

go build -o dorcs .
```


### 🧪 Experimental

In  `develop` branch you can find the latest features and changes. Use with caution, there may be not working features or breaking changes.

You can find the nightly builds marked as pre-releases in the [releases section](https://github.com/Padi2312/dorcs/releases)

## 📚 Usage

1. Create a `docs` directory with an `index.md` file in it. This will be the landing page of your documentation.
2. Run Dorcs in the directory with the `docs` folder.

```sh
dorcs
```

This will generate the documentation in the `output` directory.
