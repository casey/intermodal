# intermodal: a 40' shipping container for the Internet

## Manual

- [General](#general)
  - [Semantic Versioning](#semantic-versioning)
  - [Unstable Features](#unstable-features)
  - [Colored Output](#colored-output)
- [Bittorrent](#bittorrent)
  - [BEP Support](#bep-support)

## General

### Semantic Versioning

Intermodal follows [semantic versioning](https://semver.org/).

In particular:

- v0.0.X: Breaking changes may be introduced at any time.
- v0.X.Y: Breaking changes may only be introduced with a minor version number
  bump.
- vX.Y.Z: Breaking changes may only be introduced with a major version number
  bump

### Unstable Features

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

### Colored Output

Intermodal features colored help, error, and informational output. Colored
output is disabled if Intermodal detects that it is not printing to a TTY.

To disable colored output, set the `NO_COLOR` environment variable to any
value or pass `--use-color never` on the command line.

To force colored output, pass `--use-color always` on the command line.

## Bittorrent

### BEP Support

| Symbol             | Meaning                               |
|--------------------|---------------------------------------|
| :white_check_mark: | Supported                             |
| :x:                | Unsupported (links to tracking issue) |
| :heavy_minus_sign: | Not Applicable                        |

| BEP                                            | Status                                                | Title                                                            |
|:----------------------------------------------:|:-----------------------------------------------------:|:-----------------------------------------------------------------|
| [00](http://bittorrent.org/beps/bep_0000.html) | :heavy_minus_sign:                                    | Index of BitTorrent Enhancement Proposals                        |
| [01](http://bittorrent.org/beps/bep_0001.html) | :heavy_minus_sign:                                    | The BitTorrent Enhancement Proposal Process                      |
| [02](http://bittorrent.org/beps/bep_0002.html) | :heavy_minus_sign:                                    | Sample reStructured Text BEP Template                            |
| [03](http://bittorrent.org/beps/bep_0003.html) | :white_check_mark:                                    | The BitTorrent Protocol Specification                            |
| [04](http://bittorrent.org/beps/bep_0004.html) | :heavy_minus_sign:                                    | Assigned Numbers                                                 |
| [05](http://bittorrent.org/beps/bep_0005.html) | [:x:](https://github.com/casey/intermodal/issues/90)  | DHT Protocol                                                     |
| [06](http://bittorrent.org/beps/bep_0006.html) | :heavy_minus_sign:                                    | Fast Extension                                                   |
| [07](http://bittorrent.org/beps/bep_0007.html) | :heavy_minus_sign:                                    | IPv6 Tracker Extension                                           |
| [08](http://bittorrent.org/beps/bep_0008.html) | :heavy_minus_sign:                                    | Tracker Peer Obfuscation                                         |
| [09](http://bittorrent.org/beps/bep_0009.html) | [:x:](https://github.com/casey/intermodal/issues/91)  | Extension for Peers to Send Metadata Files                       |
| [10](http://bittorrent.org/beps/bep_0010.html) | :heavy_minus_sign:                                    | Extension Protocol                                               |
| [11](http://bittorrent.org/beps/bep_0011.html) | :heavy_minus_sign:                                    | Peer Exchange (PEX)                                              |
| [12](http://bittorrent.org/beps/bep_0012.html) | :white_check_mark:                                    | Multitracker Metadata Extension                                  |
| [14](http://bittorrent.org/beps/bep_0014.html) | :heavy_minus_sign:                                    | Local Service Discovery                                          |
| [15](http://bittorrent.org/beps/bep_0015.html) | :heavy_minus_sign:                                    | UDP Tracker Protocol for BitTorrent                              |
| [16](http://bittorrent.org/beps/bep_0016.html) | :heavy_minus_sign:                                    | Superseeding                                                     |
| [17](http://bittorrent.org/beps/bep_0017.html) | [:x:](https://github.com/casey/intermodal/issues/92)  | HTTP Seeding                                                     |
| [18](http://bittorrent.org/beps/bep_0018.html) | :heavy_minus_sign:                                    | Search Engine Specificiation                                     |
| [19](http://bittorrent.org/beps/bep_0019.html) | [:x:](https://github.com/casey/intermodal/issues/93)  | WebSeed - HTTP/FTP Seeding (GetRight style)                      |
| [20](http://bittorrent.org/beps/bep_0020.html) | :heavy_minus_sign:                                    | Peer ID Conventions                                              |
| [21](http://bittorrent.org/beps/bep_0021.html) | :heavy_minus_sign:                                    | Extension for partial seeds                                      |
| [22](http://bittorrent.org/beps/bep_0022.html) | :heavy_minus_sign:                                    | BitTorrent Local Tracker Discovery Protocol                      |
| [23](http://bittorrent.org/beps/bep_0023.html) | :heavy_minus_sign:                                    | Tracker Returns Compact Peer Lists                               |
| [24](http://bittorrent.org/beps/bep_0024.html) | :heavy_minus_sign:                                    | Tracker Returns External IP                                      |
| [25](http://bittorrent.org/beps/bep_0025.html) | :heavy_minus_sign:                                    | An Alternate BitTorrent Cache Discovery Protocol                 |
| [26](http://bittorrent.org/beps/bep_0026.html) | :heavy_minus_sign:                                    | Zeroconf Peer Advertising and Discovery                          |
| [27](http://bittorrent.org/beps/bep_0027.html) | :white_check_mark:                                    | Private Torrents                                                 |
| [28](http://bittorrent.org/beps/bep_0028.html) | :heavy_minus_sign:                                    | Tracker exchange extension                                       |
| [29](http://bittorrent.org/beps/bep_0029.html) | :heavy_minus_sign:                                    | uTorrent transport protocol                                      |
| [30](http://bittorrent.org/beps/bep_0030.html) | [:x:](https://github.com/casey/intermodal/issues/94)  | Merkle hash torrent extension                                    |
| [31](http://bittorrent.org/beps/bep_0031.html) | :heavy_minus_sign:                                    | Failure Retry Extension                                          |
| [32](http://bittorrent.org/beps/bep_0032.html) | :heavy_minus_sign:                                    | BitTorrent DHT Extensions for IPv6                               |
| [33](http://bittorrent.org/beps/bep_0033.html) | :heavy_minus_sign:                                    | DHT Scrapes                                                      |
| [34](http://bittorrent.org/beps/bep_0034.html) | :heavy_minus_sign:                                    | DNS Tracker Preferences                                          |
| [35](http://bittorrent.org/beps/bep_0035.html) | [:x:](https://github.com/casey/intermodal/issues/96)  | Torrent Signing                                                  |
| [36](http://bittorrent.org/beps/bep_0036.html) | :heavy_minus_sign:                                    | Torrent RSS feeds                                                |
| [37](http://bittorrent.org/beps/bep_0037.html) | :heavy_minus_sign:                                    | Anonymous BitTorrent over proxies                                |
| [38](http://bittorrent.org/beps/bep_0038.html) | :heavy_minus_sign:                                    | Finding Local Data Via Torrent File Hints                        |
| [39](http://bittorrent.org/beps/bep_0039.html) | [:x:](https://github.com/casey/intermodal/issues/98)  | Updating Torrents Via Feed URL                                   |
| [40](http://bittorrent.org/beps/bep_0040.html) | :heavy_minus_sign:                                    | Canonical Peer Priority                                          |
| [41](http://bittorrent.org/beps/bep_0041.html) | [:x:](https://github.com/casey/intermodal/issues/97)  | UDP Tracker Protocol Extensions                                  |
| [42](http://bittorrent.org/beps/bep_0042.html) | :heavy_minus_sign:                                    | DHT Security extension                                           |
| [43](http://bittorrent.org/beps/bep_0043.html) | :heavy_minus_sign:                                    | Read-only DHT Nodes                                              |
| [44](http://bittorrent.org/beps/bep_0044.html) | :heavy_minus_sign:                                    | Storing arbitrary data in the DHT                                |
| [45](http://bittorrent.org/beps/bep_0045.html) | :heavy_minus_sign:                                    | Multiple-address operation for the BitTorrent DHT                |
| [46](http://bittorrent.org/beps/bep_0046.html) | :heavy_minus_sign:                                    | Updating Torrents Via DHT Mutable Items                          |
| [47](http://bittorrent.org/beps/bep_0047.html) | [:x:](https://github.com/casey/intermodal/issues/99)  | Padding files and extended file attributes                       |
| [48](http://bittorrent.org/beps/bep_0048.html) | :heavy_minus_sign:                                    | Tracker Protocol Extension: Scrape                               |
| [49](http://bittorrent.org/beps/bep_0049.html) | [:x:](https://github.com/casey/intermodal/issues/100) | Distributed Torrent Feeds                                        |
| [50](http://bittorrent.org/beps/bep_0050.html) | :heavy_minus_sign:                                    | Publish/Subscribe Protocol                                       |
| [51](http://bittorrent.org/beps/bep_0051.html) | :heavy_minus_sign:                                    | DHT Infohash Indexing                                            |
| [52](http://bittorrent.org/beps/bep_0052.html) | [:x:](https://github.com/casey/intermodal/issues/101) | The BitTorrent Protocol Specification v2                         |
| [53](http://bittorrent.org/beps/bep_0053.html) | :heavy_minus_sign:                                    | Magnet URI extension - Select specific file indices for download |
| [54](http://bittorrent.org/beps/bep_0054.html) | :heavy_minus_sign:                                    | The lt_donthave extension                                        |
| [55](http://bittorrent.org/beps/bep_0055.html) | :heavy_minus_sign:                                    | Holepunch extension                                              |
