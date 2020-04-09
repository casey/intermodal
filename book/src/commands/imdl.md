# `imdl`
```
imdl v0.1.3
Casey Rodarmor <casey@rodarmor.com>
📦 A 40' shipping container for the internet - https://github.com/casey/intermodal

USAGE:
    imdl [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help        Print help message.
    -u, --unstable    Enable unstable features. To avoid premature stabilization and excessive version churn, unstable
                      features are unavailable unless this flag is set. Unstable features are not bound by semantic
                      versioning stability guarantees, and may be changed or removed at any time.
    -V, --version     Print version number.

OPTIONS:
        --color <WHEN>    Print colorful output according to `WHEN`. When `auto`, the default, colored output is only
                          enabled if imdl detects that it is connected to a terminal, the `NO_COLOR` environment
                          variable is not set, and the `TERM` environment variable is not set to `dumb`. [default: auto]
                          [possible values: auto, always, never]

SUBCOMMANDS:
    completions    Print shell completion scripts to standard output.
    help           Prints this message or the help of the given subcommand(s)
    torrent        Subcommands related to the BitTorrent protocol.

```