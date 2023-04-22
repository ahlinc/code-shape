# Code-shape

[![crates.io badge]][crates.io]

[crates.io]: https://crates.io/crates/code-shape
[crates.io badge]: https://img.shields.io/crates/v/code-shape.svg?color=%23B48723

Code-shape is a tool that uses [Tree-sitter] to extract a shape of code definitions from a source code file.
The tool uses the same language parsers that are installed for [Tree-sitter CLI][tree-sitter-cli].

## Installation

To install the `code-shape` CLI it's possible to use Rust's Cargo package manager:

```sh
cargo install code-shape
```

## Usage

To start using the tool it's needed to do some preparation.

### Prerequsites

1. Install [Tree-sitter CLI][tree-sitter-cli].
1. Run `tree-sitter init-config` that creates a config file like `~/.config/tree-sitter/config.json` in Tree-sitter's [config dir].
1. Create a directory where installed parsers would be located and add it in `"parser-directories"` list in Tree-sitter's config file.
1. Clone Tree-sitter [parsers][tree-sitter parsers] for required languages to the parsers directory.

### Define extraction query

To make it possible to extract a shape of definitions from some source code file for some language, it's needed to define a [query]. To define a new query create a file in a Code-shape's languages [config dir] `~/.config/code-shape/languages/` with an `.scm` suffix like `~/.config/code-shape/languages/c.scm` and put there a set of Tree-sitter [query] patterns like:

```scheme
; C language function declarations
(declaration
    [
        (function_declarator
            declarator: (identifier) @fn.name
        )
        (init_declarator
            (_
                (function_declarator
                    (_ (_ declarator: (identifier) @fn.name))
                )
            )
        )
    ]
)

; C language function definitions
(function_definition
    [
        (function_declarator
            declarator: (identifier) @fn.name
        )
        (_
            (function_declarator
                declarator: (identifier) @fn.name
            )
        )
    ]
    body: (_) @fn.scope
)
```

It's needed to define captures with special names:

* `<type>.name` is a capture where the `type` may be, e.g., `fn`, `class` or anything else to match a code entity name.
* `<type>.scope` is a special capture that allow for the tool to capture a context of entities and usually are tokens that defines a body of the the entity, e.g., a _function body_.

An example of the tool output:

```sh
# code-shape --scope source.c tree-sitter/lib/src/alloc.c
fn ts_malloc_default
fn ts_calloc_default
fn ts_realloc_default
fn ts_current_malloc
fn ts_current_calloc
fn ts_current_realloc
fn ts_set_allocator
```

## Embedded shape queries

For now the tool has [builtin][builtin queries] shape queries for the following language parsers:

* [C](https://github.com/tree-sitter/tree-sitter-c)

[Tree-sitter]: https://github.com/tree-sitter/tree-sitter
[tree-sitter-cli]: https://crates.io/crates/tree-sitter-cli
[tree-sitter parsers]: https://tree-sitter.github.io/tree-sitter/#parsers
[builtin queries]: https://github.com/ahlinc/code-shape/tree/main/queries/languages
[config dir]: https://docs.rs/dirs/latest/dirs/fn.config_dir.html
[query]: https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries
