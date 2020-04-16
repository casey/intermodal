# Intermodal: A 40' shipping container for the Internet

Intermodal is a user-friendly and featureful command-line BitTorrent metainfo utility for Linux, Windows, and macOS.

Project development is hosted on [GitHub](https://github.com/casey/intermodal).

The binary is called `imdl`:

```sh
$ imdl --help
```

BitTorrent metainfo related functionality is under the `torrent` subcommand:

```sh
$ imdl torrent --help
```

Intermodal can be used to create `.torrent` files:

```sh
$ imdl torrent create --input foo
```

Print information about existing `.torrent` files:

```sh
$ imdl torrent show --input foo.torrent
```

Verify downloaded torrents:

```sh
$ imdl torrent verify --input foo.torrent --content foo
```

Generate magnet links from `.torrent` files:

```sh
$ imdl torrent link --input foo.torrent
```

Show infromation about the piece length picker:

```sh
$ imdl torrent piece-length
```

Print completion scripts for the `imdl` binary:

```sh
$ imdl completions --shell zsh
```

Functionality that is not yet finalized, but still available for preview, can be accessed with the `--unstable` flag:

Print information about a collection of torrents:

```sh
$ imdl --unstable torrent stats --input dir
```
Happy sharing!
