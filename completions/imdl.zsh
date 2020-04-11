#compdef imdl

autoload -U is-at-least

_imdl() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--color=[Print colorful output according to `WHEN`. When `auto`, the default, colored output is only enabled if imdl detects that it is connected to a terminal, the `NO_COLOR` environment variable is not set, and the `TERM` environment variable is not set to `dumb`.]: :(auto always never)' \
'-u[Enable unstable features. To avoid premature stabilization and excessive version churn, unstable features are unavailable unless this flag is set. Unstable features are not bound by semantic versioning stability guarantees, and may be changed or removed at any time.]' \
'--unstable[Enable unstable features. To avoid premature stabilization and excessive version churn, unstable features are unavailable unless this flag is set. Unstable features are not bound by semantic versioning stability guarantees, and may be changed or removed at any time.]' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
":: :_imdl_commands" \
"*::: :->imdl" \
&& ret=0
    case $state in
    (imdl)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:imdl-command-$line[1]:"
        case $line[1] in
            (torrent)
_arguments "${_arguments_options[@]}" \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
":: :_imdl__torrent_commands" \
"*::: :->torrent" \
&& ret=0
case $state in
    (torrent)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:imdl-torrent-command-$line[1]:"
        case $line[1] in
            (create)
_arguments "${_arguments_options[@]}" \
'-a+[Use `URL` as the primary tracker announce URL. To supply multiple announce URLs, also use `--announce-tier`.]' \
'--announce=[Use `URL` as the primary tracker announce URL. To supply multiple announce URLs, also use `--announce-tier`.]' \
'*-A+[Allow `LINT`. Lints check for conditions which, although permitted, are not usually desirable. For example, piece length can be any non-zero value, but probably shouldn'\''t be below 16 KiB. The lint `small-piece-size` checks for this, and `--allow small-piece-size` can be used to disable this check.]: :(private-trackerless small-piece-length uneven-piece-length)' \
'*--allow=[Allow `LINT`. Lints check for conditions which, although permitted, are not usually desirable. For example, piece length can be any non-zero value, but probably shouldn'\''t be below 16 KiB. The lint `small-piece-size` checks for this, and `--allow small-piece-size` can be used to disable this check.]: :(private-trackerless small-piece-length uneven-piece-length)' \
'*-t+[Use `URL-LIST` as a tracker announce tier. Each instance adds a new tier. To add multiple trackers to a given tier, separate their announce URLs with commas:

`--announce-tier udp://example.com:80/announce,https://example.net:443/announce`
            
Announce tiers are stored in the `announce-list` key of the top-level metainfo dictionary as a list of lists of strings, as defined by BEP 12: Multitracker Metadata Extension.
            
Note: Many BitTorrent clients do not implement the behavior described in BEP 12. See the discussion here for more details: https://github.com/bittorrent/bittorrent.org/issues/82]' \
'*--announce-tier=[Use `URL-LIST` as a tracker announce tier. Each instance adds a new tier. To add multiple trackers to a given tier, separate their announce URLs with commas:

`--announce-tier udp://example.com:80/announce,https://example.net:443/announce`
            
Announce tiers are stored in the `announce-list` key of the top-level metainfo dictionary as a list of lists of strings, as defined by BEP 12: Multitracker Metadata Extension.
            
Note: Many BitTorrent clients do not implement the behavior described in BEP 12. See the discussion here for more details: https://github.com/bittorrent/bittorrent.org/issues/82]' \
'-c+[Include `TEXT` as the comment for generated `.torrent` file. Stored under `comment` key of top-level metainfo dictionary.]' \
'--comment=[Include `TEXT` as the comment for generated `.torrent` file. Stored under `comment` key of top-level metainfo dictionary.]' \
'*--node=[Add DHT bootstrap node `NODE` to torrent. `NODE` should be in the form `HOST:PORT`, where `HOST` is a domain name, an IPv4 address, or an IPv6 address surrounded by brackets. May be given more than once to add multiple bootstrap nodes.

Examples:

    --node router.example.com:1337

    --node 203.0.113.0:2290

    --node \[2001:db8:4275:7920:6269:7463:6f69:6e21\]:8832]' \
'*-g+[Include or exclude files that match `GLOB`. Multiple glob may be provided, with the last one taking precedence. Precede a glob with `!` to exclude it.]' \
'*--glob=[Include or exclude files that match `GLOB`. Multiple glob may be provided, with the last one taking precedence. Precede a glob with `!` to exclude it.]' \
'-i+[Read torrent contents from `PATH`. If `PATH` is a file, torrent will be a single-file torrent.  If `PATH` is a directory, torrent will be a multi-file torrent.  If `PATH` is `-`, read from standard input. Piece length defaults to 256KiB when reading from standard input if `--piece-length` is not given.]' \
'--input=[Read torrent contents from `PATH`. If `PATH` is a file, torrent will be a single-file torrent.  If `PATH` is a directory, torrent will be a multi-file torrent.  If `PATH` is `-`, read from standard input. Piece length defaults to 256KiB when reading from standard input if `--piece-length` is not given.]' \
'-N+[Set name of torrent to `TEXT`. Defaults to the filename of the argument to `--input`. Required when `--input -`.]' \
'--name=[Set name of torrent to `TEXT`. Defaults to the filename of the argument to `--input`. Required when `--input -`.]' \
'*--sort-by=[Set the order of files within a torrent. `SPEC` should be of the form `KEY:ORDER`, with `KEY` being one of `path` or `size`, and `ORDER` being `ascending` or `descending`. `:ORDER` defaults to `ascending` if omitted. The `--sort-by` flag may be given more than once, with later values being used to break ties. Ties that remain are broken in ascending path order.

Sort in ascending order by path, the default:

    --sort-by path:ascending

Sort in ascending order by path, more concisely:

    --sort-by path

Sort in ascending order by size, break ties in descending path order:

    --sort-by size:ascending --sort-by path:descending]' \
'-o+[Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is `-`. Defaults to the argument to `--input` with an `.torrent` extension appended. Required when `--input -`.]' \
'--output=[Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is `-`. Defaults to the argument to `--input` with an `.torrent` extension appended. Required when `--input -`.]' \
'*--peer=[Add `PEER` to magnet link.]' \
'-p+[Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib.]' \
'--piece-length=[Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib.]' \
'-s+[Set torrent source to `TEXT`. Stored under `source` key of info dictionary. This is useful for keeping statistics from being mis-reported when participating in swarms with the same contents, but with different trackers. When source is set to a unique value for torrents with the same contents, torrent clients will treat them as distinct torrents, and not share peers between them, and will correctly report download and upload statistics to multiple trackers.]' \
'--source=[Set torrent source to `TEXT`. Stored under `source` key of info dictionary. This is useful for keeping statistics from being mis-reported when participating in swarms with the same contents, but with different trackers. When source is set to a unique value for torrents with the same contents, torrent clients will treat them as distinct torrents, and not share peers between them, and will correctly report download and upload statistics to multiple trackers.]' \
'-n[Skip writing `.torrent` file to disk.]' \
'--dry-run[Skip writing `.torrent` file to disk.]' \
'-F[Follow symlinks in torrent input. By default, symlinks to files and directories are not included in torrent contents.]' \
'--follow-symlinks[Follow symlinks in torrent input. By default, symlinks to files and directories are not included in torrent contents.]' \
'-f[Overwrite the destination `.torrent` file, if it exists.]' \
'--force[Overwrite the destination `.torrent` file, if it exists.]' \
'-h[Include hidden files that would otherwise be skipped, such as files that start with a `.`, and files hidden by file attributes on macOS and Windows.]' \
'--include-hidden[Include hidden files that would otherwise be skipped, such as files that start with a `.`, and files hidden by file attributes on macOS and Windows.]' \
'-j[Include junk files that would otherwise be skipped.]' \
'--include-junk[Include junk files that would otherwise be skipped.]' \
'--link[Print created torrent `magnet:` URL to standard output]' \
'-M[Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and only suitable for checking for accidental corruption.]' \
'--md5[Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically broken and only suitable for checking for accidental corruption.]' \
'--no-created-by[Do not populate `created by` key of generated torrent with imdl version information.]' \
'--no-creation-date[Do not populate `creation date` key of generated torrent with current time.]' \
'-O[Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows]' \
'--open[Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows]' \
'-P[Set the `private` flag. Torrent clients that understand the flag and participate in the swarm of a torrent with the flag set will only announce themselves to the announce URLs included in the torrent, and will not use other peer discovery mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for more information.]' \
'--private[Set the `private` flag. Torrent clients that understand the flag and participate in the swarm of a torrent with the flag set will only announce themselves to the announce URLs included in the torrent, and will not use other peer discovery mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for more information.]' \
'-S[Display information about created torrent file.]' \
'--show[Display information about created torrent file.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(link)
_arguments "${_arguments_options[@]}" \
'-i+[Generate magnet link from metainfo at `PATH`. If `PATH` is `-`, read metainfo from standard input.]' \
'--input=[Generate magnet link from metainfo at `PATH`. If `PATH` is `-`, read metainfo from standard input.]' \
'*-p+[Add `PEER` to magnet link.]' \
'*--peer=[Add `PEER` to magnet link.]' \
'-O[Open generated magnet link. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows]' \
'--open[Open generated magnet link. Uses `xdg-open`, `gnome-open`, or `kde-open` on Linux; `open` on macOS; and `cmd /C start` on Windows]' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(piece-size)
_arguments "${_arguments_options[@]}" \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(piece-length)
_arguments "${_arguments_options[@]}" \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(show)
_arguments "${_arguments_options[@]}" \
'-i+[Show information about torrent at `PATH`. If `Path` is `-`, read torrent metainfo from standard input.]' \
'--input=[Show information about torrent at `PATH`. If `Path` is `-`, read torrent metainfo from standard input.]' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" \
'-l+[Stop after processing `N` torrents. Useful when processing large collections of `.torrent` files.]' \
'--limit=[Stop after processing `N` torrents. Useful when processing large collections of `.torrent` files.]' \
'*-e+[Extract and display values under key paths that match `REGEX`. Subkeys of a bencodeded dictionary are delimited by `/`, and values of a bencoded list are delmited by `*`. For example, given the following bencoded dictionary `{"foo": \[{"bar": {"baz": 2}}\]}`, the value `2`'\''s key path will be `foo*bar/baz`. The value `2` would be displayed if any of `bar`, `foo\[*\]bar/baz`, or `foo.*baz` were passed to `--extract-pattern.]' \
'*--extract-pattern=[Extract and display values under key paths that match `REGEX`. Subkeys of a bencodeded dictionary are delimited by `/`, and values of a bencoded list are delmited by `*`. For example, given the following bencoded dictionary `{"foo": \[{"bar": {"baz": 2}}\]}`, the value `2`'\''s key path will be `foo*bar/baz`. The value `2` would be displayed if any of `bar`, `foo\[*\]bar/baz`, or `foo.*baz` were passed to `--extract-pattern.]' \
'-i+[Search `PATH` for torrents. May be a directory or a single torrent file.]' \
'--input=[Search `PATH` for torrents. May be a directory or a single torrent file.]' \
'-p[Pretty print the contents of each torrent as it is processed.]' \
'--print[Pretty print the contents of each torrent as it is processed.]' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(verify)
_arguments "${_arguments_options[@]}" \
'-i+[Verify torrent contents against torrent metainfo in `METAINFO`. If `METAINFO` is `-`, read metainfo from standard input.]' \
'--input=[Verify torrent contents against torrent metainfo in `METAINFO`. If `METAINFO` is `-`, read metainfo from standard input.]' \
'-c+[Verify torrent content at `PATH` against torrent metainfo. Defaults to `name` field of torrent info dictionary.]' \
'--content=[Verify torrent content at `PATH` against torrent metainfo. Defaults to `name` field of torrent info dictionary.]' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-s+[Print completions for `SHELL`.]: :(zsh bash fish powershell elvish)' \
'--shell=[Print completions for `SHELL`.]: :(zsh bash fish powershell elvish)' \
'-h[Print help message.]' \
'--help[Print help message.]' \
'-V[Print version number.]' \
'--version[Print version number.]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_imdl_commands] )) ||
_imdl_commands() {
    local commands; commands=(
        "torrent:Subcommands related to the BitTorrent protocol." \
"completions:Print shell completion scripts to standard output." \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'imdl commands' commands "$@"
}
(( $+functions[_imdl__completions_commands] )) ||
_imdl__completions_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl completions commands' commands "$@"
}
(( $+functions[_imdl__torrent__create_commands] )) ||
_imdl__torrent__create_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent create commands' commands "$@"
}
(( $+functions[_imdl__help_commands] )) ||
_imdl__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl help commands' commands "$@"
}
(( $+functions[_imdl__torrent__help_commands] )) ||
_imdl__torrent__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent help commands' commands "$@"
}
(( $+functions[_imdl__torrent__link_commands] )) ||
_imdl__torrent__link_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent link commands' commands "$@"
}
(( $+functions[_imdl__torrent__piece-length_commands] )) ||
_imdl__torrent__piece-length_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent piece-length commands' commands "$@"
}
(( $+functions[_imdl__piece-size_commands] )) ||
_imdl__piece-size_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl piece-size commands' commands "$@"
}
(( $+functions[_imdl__torrent__piece-size_commands] )) ||
_imdl__torrent__piece-size_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent piece-size commands' commands "$@"
}
(( $+functions[_imdl__torrent__show_commands] )) ||
_imdl__torrent__show_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent show commands' commands "$@"
}
(( $+functions[_imdl__torrent__stats_commands] )) ||
_imdl__torrent__stats_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent stats commands' commands "$@"
}
(( $+functions[_imdl__torrent_commands] )) ||
_imdl__torrent_commands() {
    local commands; commands=(
        "create:Create a .torrent file." \
"link:Generate a magnet link from a .torrent file." \
"piece-length:Display information about automatic piece length selection." \
"show:Display information about a .torrent file." \
"stats:Show statistics about a collection of .torrent files." \
"verify:Verify files against a .torrent file." \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'imdl torrent commands' commands "$@"
}
(( $+functions[_imdl__torrent__verify_commands] )) ||
_imdl__torrent__verify_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'imdl torrent verify commands' commands "$@"
}

_imdl "$@"
