# stdselect - Standard Select

A tui that allows you to make a selection from stdin

# Installation

Requirements:
* cargo

`/bin/bash install.sh`

# Usage

We have:
* `/dev/stdin`
* `/dev/stdout`

There are times where we need a way to select.

For example:

`kjoedicker@arch ~ % echo "Option 1\nOption 2\nOption 3\nOption 4\nOption 5 | stdselect | rev > final_destination.txt`

```sh
| Option 1
| Option 2
> Option 3
| Option 4
| Option 5
3/5
```

### final_destination.txt
```
3 noitpO
```
