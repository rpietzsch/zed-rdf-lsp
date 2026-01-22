# RDF Language Support for Zed

A Zed extension providing syntax highlighting for RDF (Resource Description Framework) languages.

## Supported Languages

| Language | File Extensions | Description |
|----------|----------------|-------------|
| **SPARQL** | `.rq`, `.sparql` | SPARQL Protocol and RDF Query Language |
| **Turtle** | `.ttl` | Terse RDF Triple Language |
| **TriG** | `.trig` | Turtle with Named Graphs |

## Features

- Syntax highlighting for all supported languages
- Bracket matching and auto-closing
- Comment toggling (`#` for line comments)

## Installation

### From Zed Extension Gallery

1. Open Zed
2. Open the Extensions panel (`Cmd+Shift+X` on macOS)
3. Search for "RDF"
4. Click Install

### Manual Installation (Development)

1. Clone this repository
2. Open Zed
3. Run `Cmd+Shift+P` → `zed: install dev extension`
4. Select the cloned directory

## Development

This project uses [Task](https://taskfile.dev) for automation. Install it with:

```bash
brew install go-task
```

### Available Tasks

```bash
task                    # Show all available tasks
task dev:install        # Install extension in Zed for development
task dev:logs           # Open Zed with verbose logging
task test:all           # Create and open sample files for testing
task publish:check      # Validate extension before publishing
task publish:instructions  # Show publishing instructions
```

### Testing Locally

1. Install the dev extension:

   ```bash
   task dev:install
   ```

2. Create test files:

   ```bash
   task test:sparql   # Test SPARQL highlighting
   task test:turtle   # Test Turtle highlighting
   task test:trig     # Test TriG highlighting
   ```

3. Debug with verbose logging:

   ```bash
   task dev:logs
   ```

## Publishing

Zed extensions are published via the
[zed-industries/extensions](https://github.com/zed-industries/extensions) repository.

### Quick Steps

1. **Push your extension to GitHub**

2. **Fork** [zed-industries/extensions](https://github.com/zed-industries/extensions)

3. **Add as submodule** in your fork:

   ```bash
   git submodule add https://github.com/YOUR_USERNAME/zed-rdf-lsp extensions/rdf
   ```

4. **Add entry to `extensions.toml`**:

   ```toml
   [rdf]
   submodule = "extensions/rdf"
   version = "0.1.0"
   ```

5. **Format and submit PR**:

   ```bash
   pnpm install && pnpm sort-extensions
   git commit -am "Add RDF extension"
   # Open PR to zed-industries/extensions
   ```

Run `task publish:instructions` for detailed steps.

## Project Structure

```text
zed-rdf-lsp/
├── extension.toml           # Extension metadata + grammar sources
├── LICENSE                   # MIT license
├── README.md
├── Taskfile.yml             # Task automation
└── languages/
    ├── sparql/
    │   ├── config.toml      # Language config
    │   ├── highlights.scm   # Syntax highlighting
    │   └── brackets.scm     # Bracket matching
    ├── turtle/
    │   └── ...
    └── trig/
        └── ...
```

## Tree-sitter Grammars

This extension uses the following tree-sitter grammars:

- **SPARQL**: [GordianDziwis/tree-sitter-sparql](https://github.com/GordianDziwis/tree-sitter-sparql)
- **Turtle/TriG**: [GordianDziwis/tree-sitter-turtle](https://github.com/GordianDziwis/tree-sitter-turtle)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgements

- Inspired by [stardog-vsc](https://github.com/stardog-union/stardog-vsc) for VSCode
- Tree-sitter grammars by [Gordian Dziwis](https://github.com/GordianDziwis)
