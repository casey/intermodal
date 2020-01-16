# intermodal: a 40' shipping container for the Internet

## Bittorrent

### BEP Support

| BEP | Status | Title                                                            |
|-----|:------:|:-----------------------------------------------------------------|
| 03  |   ??   | The BitTorrent Protocol Specification                            |
| 04  |   ??   | Assigned Numbers                                                 |
| 05  |   ??   | DHT Protocol                                                     |
| 06  |   ??   | Fast Extension                                                   |
| 07  |   ??   | IPv6 Tracker Extension                                           |
| 08  |   ??   | Tracker Peer Obfuscation                                         |
| 09  |   ??   | Extension for Peers to Send Metadata Files                       |
| 10  |   ??   | Extension Protocol                                               |
| 11  |   ??   | Peer Exchange (PEX)                                              |
| 12  |   ??   | Multitracker Metadata Extension                                  |
| 14  |   ??   | Local Service Discovery                                          |
| 15  |   ??   | UDP Tracker Protocol for BitTorrent                              |
| 16  |   ??   | Superseeding                                                     |
| 17  |   ??   | HTTP Seeding                                                     |
| 18  |   ??   | Search Engine Specificiation                                     |
| 19  |   ??   | WebSeed - HTTP/FTP Seeding (GetRight style)                      |
| 20  |   ??   | Peer ID Conventions                                              |
| 21  |   ??   | Extension for partial seeds                                      |
| 22  |   ??   | BitTorrent Local Tracker Discovery Protocol                      |
| 23  |   ??   | Tracker Returns Compact Peer Lists                               |
| 24  |   ??   | Tracker Returns External IP                                      |
| 25  |   ??   | An Alternate BitTorrent Cache Discovery Protocol                 |
| 26  |   ??   | Zeroconf Peer Advertising and Discovery                          |
| 27  |   ??   | Private Torrents                                                 |
| 28  |   ??   | Tracker exchange extension                                       |
| 29  |   ??   | uTorrent transport protocol                                      |
| 30  |   ??   | Merkle hash torrent extension                                    |
| 31  |   ??   | Failure Retry Extension                                          |
| 32  |   ??   | BitTorrent DHT Extensions for IPv6                               |
| 33  |   ??   | DHT Scrapes                                                      |
| 34  |   ??   | DNS Tracker Preferences                                          |
| 35  |   ??   | Torrent Signing                                                  |
| 36  |   ??   | Torrent RSS feeds                                                |
| 37  |   ??   | Anonymous BitTorrent over proxies                                |
| 38  |   ??   | Finding Local Data Via Torrent File Hints                        |
| 39  |   ??   | Updating Torrents Via Feed URL                                   |
| 40  |   ??   | Canonical Peer Priority                                          |
| 41  |   ??   | UDP Tracker Protocol Extensions                                  |
| 42  |   ??   | DHT Security extension                                           |
| 43  |   ??   | Read-only DHT Nodes                                              |
| 44  |   ??   | Storing arbitrary data in the DHT                                |
| 45  |   ??   | Multiple-address operation for the BitTorrent DHT                |
| 46  |   ??   | Updating Torrents Via DHT Mutable Items                          |
| 47  |   ??   | Padding files and extended file attributes                       |
| 48  |   ??   | Tracker Protocol Extension: Scrape                               |
| 49  |   ??   | Distributed Torrent Feeds                                        |
| 50  |   ??   | Publish/Subscribe Protocol                                       |
| 51  |   ??   | DHT Infohash Indexing                                            |
| 52  |   ??   | The BitTorrent Protocol Specification v2                         |
| 53  |   ??   | Magnet URI extension - Select specific file indices for download |
| 54  |   ??   | The lt_donthave extension                                        |

## General Functionality

### Colored Output

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
