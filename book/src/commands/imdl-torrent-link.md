# `imdl torrent link`
```
imdl-torrent-link 0.1.4
Generate a magnet link from a .torrent file.

USAGE:
    imdl torrent link [FLAGS] [OPTIONS] --input <METAINFO>

FLAGS:
    -h, --help       Print help message.
    -O, --open       Open generated magnet link. Uses `xdg-open`, `gnome-open`,
                     or `kde-open` on Linux; `open` on macOS; and `cmd /C start`
                     on Windows
    -V, --version    Print version number.

OPTIONS:
    -i, --input <METAINFO>    Generate magnet link from metainfo at `PATH`. If
                              `PATH` is `-`, read metainfo from standard input.
    -p, --peer <PEER>...      Add `PEER` to magnet link.

```