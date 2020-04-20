# `imdl torrent verify`
```
imdl-torrent-verify 0.1.6
Verify files against a .torrent file.

USAGE:
    imdl torrent verify [OPTIONS] --input <METAINFO>

FLAGS:
    -h, --help       Print help message.
    -V, --version    Print version number.

OPTIONS:
    -c, --content <PATH>      Verify torrent content at `PATH` against torrent
                              metainfo. Defaults to `name` field of torrent info
                              dictionary.
    -i, --input <METAINFO>    Verify torrent contents against torrent metainfo
                              in `METAINFO`. If `METAINFO` is `-`, read metainfo
                              from standard input.

```