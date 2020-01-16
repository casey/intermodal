# intermodal: a 40' shipping container for the Internet

## Colored Output

Intermodal features colored help, error, and informational output. Colored
output is disabled if Intermodal detects that it is not printing to a TTY.

To disable colored output, set the `NO_COLOR` environment variable to any
valu or pass `--use-color never` on the command line.

To force colored output, pass `--use-color always` on the command line.

## Semantic Versioning and Unstable Features

Intermodal follows [semantic versioning](https://semver.org/).

In particular:

- v0.0.X: Breaking changes may be introduced at any time.
- v0.X.Y: Breaking changes may only be introduced with a minor version number
  bump.
- vX.Y.Z: Breaking changes may only be introduced with a major version number
  bump

To avoid premature stabilization and excessive version churn, unstable features
are unavailable unless the `--unstable` / `-u` flag is passed. Unstable
features may be changed or removed at any time.

```
$ imdl torrent stats --input tmp
error: Feature `torrent stats subcommand` cannot be used without passing the `--unstable` flag
$ imdl --unstable torrent stats tmp
Torrents processed: 0
Read failed:        0
Decode failed:      0
```
