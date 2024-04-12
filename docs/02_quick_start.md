---
title: Quick Start
position: 2
---

# Quick Start

I promise you, we'll get over this very quickly. -  _Although you should read the rest of the documentation😉_

## Installation

### 📥 Binary

You can download the latest release from the [releases page](https://github.com/padi2312/dorcs/releases).

### 🔨 Build from Source

For those who prefer to build from source, ensure Go(lang) is properly set up, then follow these steps:

```sh
git clone https://github.com/padi2312/dorcs
cd dorcs

go generate ./build/gen.go
go build -o dorcs .  # or on Windows: go build -o dorcs.exe .
```

---

## 💫 Usage

1. Create a `docs` directory 
2. Add markdown files to the `docs` directory
3. Run the following command:

```sh
dorcs
```

This will generate the documentation by default in the `output` directory.

\
**For more information on how to use Dorcs, check out the [Usage Page](./03_usage).**