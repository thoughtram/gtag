# gtag

The missing git command to rapidly create tags from commit ranges.

## Usage

```
USAGE:
	gtag [FLAGS] <range> <pattern>

FLAGS:
    -d, --delete     Deletes generated tags
        --dryrun     Just prints what it would do
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    range      Sets the commit range
    pattern    Sets the pattern for the tag name
```

## Specifying a range

Ranges can be specified following the standard Git range syntax (e.g. `sha1..sha2`).
There's one catch though: Git treats the first commit as exclusive whereas `gtag`
always treats them as inclusive. This deliberate choice made for convenience.

## Pattern syntax

The pattern is a simple string that can take `##i` as a replacement markers that
will automatically be replaced by the index of the commit in the range. Alternatively
`##ii` uses the index + 1.

## Installation

The easiest way to get the `gtag` command is through cargo. Just run `cargo install gtag`.
