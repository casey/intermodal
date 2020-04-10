# `imdl torrent create`
```
imdl-torrent-create 0.1.4
Create a `.torrent` file.

USAGE:
    imdl torrent create [FLAGS] [OPTIONS] --input <PATH>

FLAGS:
    -n, --dry-run             Skip writing `.torrent` file to disk.
    -F, --follow-symlinks     Follow symlinks in torrent input. By default, symlinks to files and directories are not
                              included in torrent contents.
    -f, --force               Overwrite the destination `.torrent` file, if it exists.
        --help                Print help message.
    -h, --include-hidden      Include hidden files that would otherwise be skipped, such as files that start with a `.`,
                              and files hidden by file attributes on macOS and Windows.
    -j, --include-junk        Include junk files that would otherwise be skipped.
    -M, --md5                 Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and
                              only suitable for checking for accidental corruption.
        --no-created-by       Do not populate `created by` key of generated torrent with imdl version information.
        --no-creation-date    Do not populate `creation date` key of generated torrent with current time.
    -O, --open                Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on
                              Linux; `open` on macOS; and `cmd /C start` on Windows
        --link                Print created torrent `magnet:` URL to standard output
    -P, --private             Set the `private` flag. Torrent clients that understand the flag and participate in the
                              swarm of a torrent with the flag set will only announce themselves to the announce URLs
                              included in the torrent, and will not use other peer discovery mechanisms, such as the DHT
                              or local peer discovery. See BEP 27: Private Torrents for more information.
    -S, --show                Display information about created torrent file.
    -V, --version             Print version number.

OPTIONS:
    -A, --allow <LINT>...                Allow `LINT`. Lints check for conditions which, although permitted, are not
                                         usually desirable. For example, piece length can be any non-zero value, but
                                         probably shouldn't be below 16 KiB. The lint `small-piece-size` checks for
                                         this, and `--allow small-piece-size` can be used to disable this check.
                                         [possible values: private-trackerless, small-piece-length, uneven-piece-length]
    -a, --announce <URL>                 Use `URL` as the primary tracker announce URL. To supply multiple announce
                                         URLs, also use `--announce-tier`.
    -t, --announce-tier <URL-LIST>...    Use `URL-LIST` as a tracker announce tier. Each instance adds a new tier. To
                                         add multiple trackers to a given tier, separate their announce URLs with
                                         commas:
                                         
                                         `--announce-tier
                                         udp://example.com:80/announce,https://example.net:443/announce`
                                                     
                                         Announce tiers are stored in the `announce-list` key of the top-level metainfo
                                         dictionary as a list of lists of strings, as defined by BEP 12: Multitracker
                                         Metadata Extension.
                                                     
                                         Note: Many BitTorrent clients do not implement the behavior described in BEP
                                         12. See the discussion here for more details:
                                         https://github.com/bittorrent/bittorrent.org/issues/82
    -c, --comment <TEXT>                 Include `TEXT` as the comment for generated `.torrent` file. Stored under
                                         `comment` key of top-level metainfo dictionary.
        --node <NODE>...                 Add DHT bootstrap node `NODE` to torrent. `NODE` should be in the form
                                         `HOST:PORT`, where `HOST` is a domain name, an IPv4 address, or an IPv6 address
                                         surrounded by brackets. May be given more than once to add multiple bootstrap
                                         nodes.
                                         
                                         Examples:
                                         
                                             --node router.example.com:1337
                                         
                                             --node 203.0.113.0:2290
                                         
                                             --node [2001:db8:4275:7920:6269:7463:6f69:6e21]:8832
    -g, --glob <GLOB>...                 Include or exclude files that match `GLOB`. Multiple glob may be provided, with
                                         the last one taking precedence. Precede a glob with `!` to exclude it.
    -i, --input <PATH>                   Read torrent contents from `PATH`. If `PATH` is a file, torrent will be a
                                         single-file torrent.  If `PATH` is a directory, torrent will be a multi-file
                                         torrent.  If `PATH` is `-`, read from standard input. Piece length defaults to
                                         256KiB when reading from standard input if `--piece-length` is not given.
    -N, --name <TEXT>                    Set name of torrent to `TEXT`. Defaults to the filename of the argument to
                                         `--input`. Required when `--input -`.
    -o, --output <TARGET>                Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is
                                         `-`. Defaults to the argument to `--input` with an `.torrent` extension
                                         appended. Required when `--input -`.
        --peer <PEER>...                 Add `PEER` to magnet link.
    -p, --piece-length <BYTES>           Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib.
        --sort-by <SPEC>...              Set the order of files within a torrent. `SPEC` should be of the form
                                         `KEY:ORDER`, with `KEY` being one of `path` or `size`, and `ORDER` being
                                         `ascending` or `descending`. `:ORDER` defaults to `ascending` if omitted. The
                                         `--sort-by` flag may be given more than once, with later values being used to
                                         break ties. Ties that remain are broken in ascending path order.
                                         
                                         Sort in ascending order by path, the default:
                                         
                                             --sort-by path:ascending
                                         
                                         Sort in ascending order by path, more concisely:
                                         
                                             --sort-by path
                                         
                                         Sort in ascending order by size, break ties in descending path order:
                                         
                                             --sort-by size:ascending --sort-by path:descending
    -s, --source <TEXT>                  Set torrent source to `TEXT`. Stored under `source` key of info dictionary.
                                         This is useful for keeping statistics from being mis-reported when
                                         participating in swarms with the same contents, but with different trackers.
                                         When source is set to a unique value for torrents with the same contents,
                                         torrent clients will treat them as distinct torrents, and not share peers
                                         between them, and will correctly report download and upload statistics to
                                         multiple trackers.

```