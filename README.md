# gtag

The missing git command to rapidly create tags from commit ranges.

## Usage

```
USAGE:
	gtag [FLAGS] --from <from> --to <to> --pattern <pattern>

FLAGS:
    -d, --dryrun     Just prints but doesn't tag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --from <from>          Sets the starting point
        --to <to>              Sets the ending point
        --pattern <pattern>    Sets the pattern for the tag name
```

## Pattern syntax

The pattern is a simple string that can take `##i` as a replacement markers that
will automatically be replaced by the index of the commit in the range. Alternatively
`##ii` uses the index + 1. 
