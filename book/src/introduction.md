# Intermodal

Intermodal is, as the moment, a BitTorrent metainfo utility. The binary is called `imdl`.

Project development is hosted on GitHub at [github.com/casey/intermodal](https://github.com/casey/intermodal).

Intermodal can be used to create `.torrent` files:

```
$ imdl torrent create --input foo
```

Print information about existing torrent files:

```
$ imdl torrent show --input foo.torrent
```

Verify downloaded torrents:

```
$ imdl torrent verify --input foo.torrent --content foo
```

Generate magnet links from torrent files:

```
$ imdl torrent link --input foo.torrent
```

Functionality that is not yet finalized, but still available for preview, can be accessed with the `--unstable` flag:

Print information about a collection of torrents:

```
$ imdl --unstable torrent stats --input dir
```

Happy sharing!
