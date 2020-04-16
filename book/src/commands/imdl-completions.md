# `imdl completions`
```
imdl-completions 0.1.5
Print shell completion scripts to standard output.

USAGE:
    imdl completions [OPTIONS] --shell <SHELL>

FLAGS:
    -h, --help       Print help message.
    -V, --version    Print version number.

OPTIONS:
    -d, --dir <DIR>        Write completion script to `DIR` with an appropriate
                           filename. If `--shell` is not given, write all
                           completion scripts.
    -s, --shell <SHELL>    Print completion script for `SHELL`. [possible
                           values: zsh, bash, fish, powershell, elvish]

```