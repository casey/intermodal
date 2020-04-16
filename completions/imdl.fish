complete -c imdl -n "__fish_use_subcommand" -s c -l color -d 'Print colorful output according to `WHEN`. When `auto`, the default, colored output is only enabled if imdl detects that it is connected to a terminal, the `NO_COLOR` environment variable is not set, and the `TERM` environment variable is not set to `dumb`.' -r -f -a "auto always never"
complete -c imdl -n "__fish_use_subcommand" -s u -l unstable -d 'Enable unstable features. To avoid premature stabilization and excessive version churn, unstable features are unavailable unless this flag is set. Unstable features are not bound by semantic versioning stability guarantees, and may be changed or removed at any time.'
complete -c imdl -n "__fish_use_subcommand" -s t -l terminal -d 'Disable automatic terminal detection and behave as if both standard output and standard error are connected to a terminal.'
complete -c imdl -n "__fish_use_subcommand" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_use_subcommand" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_use_subcommand" -f -a "torrent" -d 'Subcommands related to the BitTorrent protocol.'
complete -c imdl -n "__fish_use_subcommand" -f -a "completions" -d 'Print shell completion scripts to standard output.'
complete -c imdl -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "create" -d 'Create a .torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "link" -d 'Generate a magnet link from a .torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "piece-length" -d 'Display information about automatic piece length selection.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "show" -d 'Display information about a .torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "stats" -d 'Show statistics about a collection of .torrent files.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "verify" -d 'Verify files against a .torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from torrent" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c imdl -n "__fish_seen_subcommand_from create" -s a -l announce -d 'Use `URL` as the primary tracker announce URL. To supply multiple announce URLs, also use `--announce-tier`.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s A -l allow -d 'Allow `LINT`. Lints check for conditions which, although permitted, are not usually desirable. For example, piece length can be any non-zero value, but probably shouldn\'t be below 16 KiB. The lint `small-piece-size` checks for this, and `--allow small-piece-size` can be used to disable this check.' -r -f -a "private-trackerless small-piece-length uneven-piece-length"
complete -c imdl -n "__fish_seen_subcommand_from create" -s t -l announce-tier -d 'Use `URL-LIST` as a tracker announce tier. Each instance adds a new tier. To add multiple trackers to a given tier, separate their announce URLs with commas:

`--announce-tier udp://example.com:80/announce,https://example.net:443/announce`
            
Announce tiers are stored in the `announce-list` key of the top-level metainfo dictionary as a list of lists of strings, as defined by BEP 12: Multitracker Metadata Extension.
            
Note: Many BitTorrent clients do not implement the behavior described in BEP 12. See the discussion here for more details: https://github.com/bittorrent/bittorrent.org/issues/82'
complete -c imdl -n "__fish_seen_subcommand_from create" -s c -l comment -d 'Include `TEXT` as the comment for generated `.torrent` file. Stored under `comment` key of top-level metainfo dictionary.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l node -d 'Add DHT bootstrap node `NODE` to torrent. `NODE` should be in the form `HOST:PORT`, where `HOST` is a domain name, an IPv4 address, or an IPv6 address surrounded by brackets. May be given more than once to add multiple bootstrap nodes.

Examples:

    --node router.example.com:1337

    --node 203.0.113.0:2290

    --node [2001:db8:4275:7920:6269:7463:6f69:6e21]:8832'
complete -c imdl -n "__fish_seen_subcommand_from create" -s g -l glob -d 'Include or exclude files that match `GLOB`. Multiple glob may be provided, with the last one taking precedence. Precede a glob with `!` to exclude it.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s i -l input -d 'Read torrent contents from `PATH`. If `PATH` is a file, torrent will be a single-file torrent.  If `PATH` is a directory, torrent will be a multi-file torrent.  If `PATH` is `-`, read from standard input. Piece length defaults to 256KiB when reading from standard input if `--piece-length` is not given.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s N -l name -d 'Set name of torrent to `TEXT`. Defaults to the filename of the argument to `--input`. Required when `--input -`.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l sort-by -d 'Set the order of files within a torrent. `SPEC` should be of the form `KEY:ORDER`, with `KEY` being one of `path` or `size`, and `ORDER` being `ascending` or `descending`. `:ORDER` defaults to `ascending` if omitted. The `--sort-by` flag may be given more than once, with later values being used to break ties. Ties that remain are broken in ascending path order.

Sort in ascending order by path, the default:

    --sort-by path:ascending

Sort in ascending order by path, more concisely:

    --sort-by path

Sort in ascending order by size, break ties in descending path order:

    --sort-by size:ascending --sort-by path:descending'
complete -c imdl -n "__fish_seen_subcommand_from create" -s o -l output -d 'Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is `-`. Defaults to the argument to `--input` with an `.torrent` extension appended. Required when `--input -`.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l peer -d 'Add `PEER` to magnet link.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s p -l piece-length -d 'Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s s -l source -d 'Set torrent source to `TEXT`. Stored under `source` key of info dictionary. This is useful for keeping statistics from being mis-reported when participating in swarms with the same contents, but with different trackers. When source is set to a unique value for torrents with the same contents, torrent clients will treat them as distinct torrents, and not share peers between them, and will correctly report download and upload statistics to multiple trackers.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s n -l dry-run -d 'Skip writing `.torrent` file to disk.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s F -l follow-symlinks -d 'Follow symlinks in torrent input. By default, symlinks to files and directories are not included in torrent contents.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s f -l force -d 'Overwrite the destination `.torrent` file, if it exists.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s h -l include-hidden -d 'Include hidden files that would otherwise be skipped, such as files that start with a `.`, and files hidden by file attributes on macOS and Windows.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s j -l include-junk -d 'Include junk files that would otherwise be skipped.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l link -d 'Print created torrent `magnet:` URL to standard output'
complete -c imdl -n "__fish_seen_subcommand_from create" -s M -l md5 -d 'Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and only suitable for checking for accidental corruption.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l no-created-by -d 'Do not populate `created by` key of generated torrent with imdl version information.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l no-creation-date -d 'Do not populate `creation date` key of generated torrent with current time.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s O -l open -d 'Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows'
complete -c imdl -n "__fish_seen_subcommand_from create" -s P -l private -d 'Set the `private` flag. Torrent clients that understand the flag and participate in the swarm of a torrent with the flag set will only announce themselves to the announce URLs included in the torrent, and will not use other peer discovery mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for more information.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s S -l show -d 'Display information about created torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l ignore -d 'Skip files listed in `.gitignore`, `.ignore`, `.git/info/exclude`, and `git config --get core.excludesFile`.'
complete -c imdl -n "__fish_seen_subcommand_from create" -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from create" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s i -l input -d 'Generate magnet link from metainfo at `PATH`. If `PATH` is `-`, read metainfo from standard input.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s p -l peer -d 'Add `PEER` to magnet link.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s s -l select-only -d 'Select files to download. Values are indices into the `info.files` list, e.g. `--select-only 1,2,3`.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s O -l open -d 'Open generated magnet link. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from link" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from piece-length" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from piece-length" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from show" -s i -l input -d 'Show information about torrent at `PATH`. If `Path` is `-`, read torrent metainfo from standard input.'
complete -c imdl -n "__fish_seen_subcommand_from show" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from show" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s l -l limit -d 'Stop after processing `N` torrents. Useful when processing large collections of `.torrent` files.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s e -l extract-pattern -d 'Extract and display values under key paths that match `REGEX`. Subkeys of a bencodeded dictionary are delimited by `/`, and values of a bencoded list are delmited by `*`. For example, given the following bencoded dictionary `{"foo": [{"bar": {"baz": 2}}]}`, the value `2`\'s key path will be `foo*bar/baz`. The value `2` would be displayed if any of `bar`, `foo[*]bar/baz`, or `foo.*baz` were passed to `--extract-pattern.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s i -l input -d 'Search `PATH` for torrents. May be a directory or a single torrent file.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s p -l print -d 'Pretty print the contents of each torrent as it is processed.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from stats" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from verify" -s i -l input -d 'Verify torrent contents against torrent metainfo in `METAINFO`. If `METAINFO` is `-`, read metainfo from standard input.'
complete -c imdl -n "__fish_seen_subcommand_from verify" -s c -l content -d 'Verify torrent content at `PATH` against torrent metainfo. Defaults to `name` field of torrent info dictionary.'
complete -c imdl -n "__fish_seen_subcommand_from verify" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from verify" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c imdl -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c imdl -n "__fish_seen_subcommand_from completions" -s s -l shell -d 'Print completion script for `SHELL`.' -r -f -a "zsh bash fish powershell elvish"
complete -c imdl -n "__fish_seen_subcommand_from completions" -s d -l dir -d 'Write completion script to `DIR` with an appropriate filename. If `--shell` is not given, write all completion scripts.'
complete -c imdl -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Print help message.'
complete -c imdl -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Print version number.'
complete -c imdl -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c imdl -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
