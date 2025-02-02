# RBMSTable-Parser

A simple library helps you to parse BMS difficult table content from an URL

The repository name was stolen from jbmstable-parser

## Usage

This library only provides one function(with some modal definitions):

```rust
fn parse(url: String) -> Result<DifficultTable, ParseError>
```

See `examples` for basic usage.