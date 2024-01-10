# gtinny - A GTIN Validator

### Install

```console
cargo install gtinny
```

### Usage

```console
A GTIN Validator

Usage: gtinny [OPTIONS] [gtin]

Arguments:
  [gtin]  Validates the GTIN

Options:
  -v, --verbose  Print if it is a valid GTIN
  -h, --help     Print help
  -V, --version  Print version
```

### Examples
```console
> gtinny 97350053850012 # is a valid GTIN
> echo $?
0

> gtinny 12398748906 # is an invalid GTIN
Invalid GTIN: 12398748906
> echo $?
1
```