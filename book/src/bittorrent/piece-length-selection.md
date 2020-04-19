BitTorrent Piece Length Selection
=================================

BitTorrent `.torrent` files contain so-called metainfo that allows BitTorrent
peers to locate, download, and verify the contents of a torrent.

This metainfo includes the piece list, a list of SHA-1 hashes of fixed-size
pieces of the torrent data. The size of these pieces is chosen by the torrent
creator.

Intermodal has a simple algorithm that attempts to pick a reasonable piece
length for a torrent given the size of the contents.

For compatibility with the
[BitTorrent v2 specification](http://bittorrent.org/beps/bep_0052.html), the
algorithm chooses piece lengths that are powers of two, and that are at least
16KiB.

The maximum automatically chosen piece length is 16MiB, as piece lengths larger
than 16MiB have been reported to cause issues for some clients.

In addition to the above constraints, there are a number of additional factors
to consider.


Factors favoring smaller piece length
-------------------------------------

- To avoid uploading bad data, peers only upload data from full pieces, which
  can be verified by hash. Decreasing the piece size allows peers to more
  quickly obtain a full piece, which decreases the time before they begin
  uploading, and receiving data in return.

- Decreasing the piece size decreases the amount of data that must be thrown
  away in case of corruption.


Factors favoring larger piece length
------------------------------------

- Increasing the piece size decreases the protocol overhead from requesting
  many pieces.

- Increasing the piece size decreases the number of pieces, decreasing the
  size of the metainfo.

- Increasing piece length increases the proportion of disk seeks to disk
  reads, which can be beneficial for spinning disks.


Intermodal's Algorithm
----------------------

In Python, the algorithm used by intermodal is:

```python
MIN = 16 * 1024
MAX = 16 * 1024 * 1024

def piece_length(content_length):
  exponent = math.log2(content_length)
  length = 1 << int((exponent / 2 + 4))
  return min(max(length, MIN), MAX)
```

Which gives the following piece lengths:

```
Content -> Piece Length x Count    = Piece List Size
16 KiB  -> 16 KiB       x 1        = 20 bytes
32 KiB  -> 16 KiB       x 2        = 40 bytes
64 KiB  -> 16 KiB       x 4        = 80 bytes
128 KiB -> 16 KiB       x 8        = 160 bytes
256 KiB -> 16 KiB       x 16       = 320 bytes
512 KiB -> 16 KiB       x 32       = 640 bytes
1 MiB   -> 16 KiB       x 64       = 1.25 KiB
2 MiB   -> 16 KiB       x 128      = 2.5 KiB
4 MiB   -> 32 KiB       x 128      = 2.5 KiB
8 MiB   -> 32 KiB       x 256      = 5 KiB
16 MiB  -> 64 KiB       x 256      = 5 KiB
32 MiB  -> 64 KiB       x 512      = 10 KiB
64 MiB  -> 128 KiB      x 512      = 10 KiB
128 MiB -> 128 KiB      x 1024     = 20 KiB
256 MiB -> 256 KiB      x 1024     = 20 KiB
512 MiB -> 256 KiB      x 2048     = 40 KiB
1 GiB   -> 512 KiB      x 2048     = 40 KiB
2 GiB   -> 512 KiB      x 4096     = 80 KiB
4 GiB   -> 1 MiB        x 4096     = 80 KiB
8 GiB   -> 1 MiB        x 8192     = 160 KiB
16 GiB  -> 2 MiB        x 8192     = 160 KiB
32 GiB  -> 2 MiB        x 16384    = 320 KiB
64 GiB  -> 4 MiB        x 16384    = 320 KiB
128 GiB -> 4 MiB        x 32768    = 640 KiB
256 GiB -> 8 MiB        x 32768    = 640 KiB
512 GiB -> 8 MiB        x 65536    = 1.25 MiB
1 TiB   -> 16 MiB       x 65536    = 1.25 MiB
2 TiB   -> 16 MiB       x 131072   = 2.5 MiB
4 TiB   -> 16 MiB       x 262144   = 5 MiB
8 TiB   -> 16 MiB       x 524288   = 10 MiB
16 TiB  -> 16 MiB       x 1048576  = 20 MiB
32 TiB  -> 16 MiB       x 2097152  = 40 MiB
64 TiB  -> 16 MiB       x 4194304  = 80 MiB
128 TiB -> 16 MiB       x 8388608  = 160 MiB
256 TiB -> 16 MiB       x 16777216 = 320 MiB
512 TiB -> 16 MiB       x 33554432 = 640 MiB
1 PiB   -> 16 MiB       x 67108864 = 1.25 GiB
```


References
----------

### Articles

- [Vuze Wiki](https://wiki.vuze.com/w/Torrent_Piece_Size)

- [TorrentFreak](https://torrentfreak.com/how-to-make-the-best-torrents-081121/)

### Implementations

- [libtorrent](https://github.com/arvidn/libtorrent/blob/a3440e54bb7f65ac6100c3d993c53f887025d660/src/create_torrent.cpp#L367)

- [libtransmission](https://github.com/transmission/transmission/blob/a482100f0cbae8050fd7e954af2cb1311205916e/libtransmission/makemeta.c#L89)

- [dottorrent](https://github.com/kz26/dottorrent/blob/fea5714efe0cde2a55eabfb387295781a78d84bb/dottorrent/__init__.py#L154)

- [Torrent File Editor](https://github.com/torrent-file-editor/torrent-file-editor/blob/811e401b38f26b6d94c4808c54ae2dcc7bbc27dd/mainwindow.cpp#L1210)
