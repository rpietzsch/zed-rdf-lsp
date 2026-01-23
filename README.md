# RDF Language Support for Zed

A Zed extension providing syntax highlighting and language server support for
RDF (Resource Description Framework) languages.

## Supported Languages

| Language   | File Extensions  | Description                            |
| ---------- | ---------------- | -------------------------------------- |
| **SPARQL** | `.rq`, `.sparql` | SPARQL Protocol and RDF Query Language |
| **Turtle** | `.ttl`           | Terse RDF Triple Language              |
| **TriG**   | `.trig`          | Turtle with Named Graphs               |

## Features

- **Syntax highlighting** for all supported languages
- **Language Server Protocol (LSP)** support via language servers:
  - Autocompletion for keywords and syntax
  - Hover tooltips
  - Diagnostics (error checking)
- Bracket matching and auto-closing
- Comment toggling (`#` for line comments)

## Installation

### From Zed Extension Gallery (Recommended)

1. Open Zed
2. Open the Extensions panel (`Cmd+Shift+X` on macOS)
3. Search for "RDF"
4. Click Install

### Development Installation (Requires Rust)

1. Clone this repository
2. Build the extension: `task build`
3. Open Zed
4. Run `Cmd+Shift+P` → `zed: install dev extension`
5. Select the cloned directory

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (via rustup, not homebrew)
- [Task](https://taskfile.dev) runner

```bash
# Install Task
brew install go-task

# Setup Rust and WASM target
task setup
```

### Available Tasks

```bash
task                    # Show all available tasks
task setup              # Install dependencies (Rust, WASM target)
task build              # Build extension (compile to WASM)
task dev:install        # Build and install in Zed for testing
task dev:logs           # Open Zed with verbose logging
task test:all           # Create and open sample files
task publish:check      # Validate extension before publishing
```

### Build & Test Workflow

1. Setup dependencies:

   ```bash
   task setup
   ```

2. Build the extension:

   ```bash
   task build
   ```

3. Install in Zed:

   ```bash
   task dev:install
   ```

4. Test with sample files:

   ```bash
   task test:sparql   # Test SPARQL
   task test:turtle   # Test Turtle
   task test:trig     # Test TriG
   ```

5. Debug with verbose logging:

   ```bash
   task dev:logs
   ```

## Publishing

Zed extensions are published via the
[zed-industries/extensions](https://github.com/zed-industries/extensions)
repository.

### Quick Steps

1. **Build and validate:**

   ```bash
   task build
   task publish:check
   ```

2. **Push your extension to GitHub**

3. **Fork**
   [zed-industries/extensions](https://github.com/zed-industries/extensions)

4. **Add as submodule** in your fork:

   ```bash
   git submodule add \
     https://github.com/YOUR_USERNAME/zed-rdf-lsp \
     extensions/rdf
   ```

5. **Add entry to `extensions.toml`**:

   ```toml
   [rdf]
   submodule = "extensions/rdf"
   version = "0.1.0"
   ```

6. **Format and submit PR**:

   ```bash
   pnpm install && pnpm sort-extensions
   git commit -am "Add RDF extension"
   # Open PR to zed-industries/extensions
   ```

Run `task publish:instructions` for detailed steps.

## Project Structure

```text
zed-rdf-lsp/
├── extension.toml       # Extension metadata + grammars + LSP config
├── Cargo.toml           # Rust dependencies
├── src/
│   └── lib.rs           # Extension implementation (LSP integration)
├── languages/
│   ├── sparql/
│   │   ├── config.toml  # Language config
│   │   ├── highlights.scm
│   │   └── brackets.scm
│   ├── turtle/
│   │   └── ...
│   └── trig/
│       └── ...
├── LICENSE
├── README.md
└── Taskfile.yml
```

## Language Servers

This extension uses the language servers:

| Language | npm Package                                                                    |
| -------- | ------------------------------------------------------------------------------ |
| SPARQL   | [sparql-language-server](https://www.npmjs.com/package/sparql-language-server) |
| Turtle   | [turtle-language-server](https://www.npmjs.com/package/turtle-language-server) |
| TriG     | [trig-language-server](https://www.npmjs.com/package/trig-language-server)     |

The extension automatically downloads and manages these servers.

## Tree-sitter Grammars

- **SPARQL**: [GordianDziwis/tree-sitter-sparql](https://github.com/GordianDziwis/tree-sitter-sparql)
- **Turtle/TriG**: [GordianDziwis/tree-sitter-turtle](https://github.com/GordianDziwis/tree-sitter-turtle)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgements

- Inspired by [stardog-vsc](https://github.com/stardog-union/stardog-vsc)
- Language servers by [Stardog Union](https://github.com/stardog-union)
- Tree-sitter grammars by [Gordian Dziwis](https://github.com/GordianDziwis)
