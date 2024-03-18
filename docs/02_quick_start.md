---
title: Quick Start
position: 2
---

# Quick Start

So you want to get started with Dorcs? Here's a quick guide to help you get started. You should also have a look at the rest of the documentation to get a better understanding of the features and limitations of Dorcs.

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