# `imdl torrent stats`
```
imdl-torrent-stats 0.1.3
Show statistics about a collection of `.torrent` files.

USAGE:
    imdl torrent stats [FLAGS] [OPTIONS] --input <PATH>

FLAGS:
    -h, --help       Print help message.
    -p, --print      Pretty print the contents of each torrent as it is processed.
    -V, --version    Print version number.

OPTIONS:
    -e, --extract-pattern <REGEX>...    Extract and display values under key paths that match `REGEX`. Subkeys of a
                                        bencodeded dictionary are delimited by `/`, and values of a bencoded list are
                                        delmited by `*`. For example, given the following bencoded dictionary `{"foo":
                                        [{"bar": {"baz": 2}}]}`, the value `2`'s key path will be `foo*bar/baz`. The
                                        value `2` would be displayed if any of `bar`, `foo[*]bar/baz`, or `foo.*baz`
                                        were passed to `--extract-pattern.
    -i, --input <PATH>                  Search `PATH` for torrents. May be a directory or a single torrent file.
    -l, --limit <N>                     Stop after processing `N` torrents. Useful when processing large collections of
                                        `.torrent` files.

```