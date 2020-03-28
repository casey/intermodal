Changelog
=========


[v0.1.0] - 2020-3-27
--------------------

### Added
- Allow sorting files in torrents (#287)

### Changed
- Introduce "sort specs" to allow fine-grained sorting of files in torrents (#292)
- Use `open` crate to open files and URLs (#270)

### Misc
- Use `strum` crate to derive enum↔string conversions (#291)
- List supported OSs in readme (#290)
- Link to blog post in readme (#289)
- Add `cargo install cargo-watch` to dev-deps recipe (#285)
- Fix condition in GHA workflow (#284)
- Only deploy site during linux run of CI build (#282)
- Go back to sans-serif `I` in site logo (#283)
- Use serifed `I` for intermodal on site (#281)
- Add glow to `intermodal` letters on site (#279)
- Improve readme intro (#278)
- Add links to site (#277)
- Rename `www/head` to `www/head.txt` (#275)
- Record current git revision in `www/head` (#274)
- Merge Github Actions workflows (#273)
- Deny warnings in GitHub Actions main workflow build (#272)
- Improve book (#271)
- Add GitHub Actions workflow to build book and push to github pages (#269)
- Fix build errors (#267)
- Add additional documents to release archive (#266)


[v0.0.3] - 2020-3-18
--------------------

### Misc
- Fix release process issues (#264)


[v0.0.2] - 2020-3-18
--------------------

### Added
- Allow reading torrent metainfo from stdin (#262)
- Skip torrent file creation with `imdl torrent create --dry-run` (#260)
- Print magnet link to stdout with `--link` (#258)
- Allow opening magnet links after creation (#250)
- Allow creating magnet links with `imdl torrent link` (#248)
- Add piece hashing progress bar (#214)
- Add progress messages to `imdl torrent create` (#213)
- Support adding DHT bootstrap nodes to created torrents (#169)
- Add initial implementation of `imdl torrent verify`
- Display torrent file tree (#153)
- Allow including and excluding files from torrent with globs (#151)
- Support creating multi-file torrents (#148)
- Don't overwrite destination torrent without `--force` (#146)
- Select piece length when none is provided (#144)
- Add source to generated torrent with `--source` (#141)
- Add `imdl torrent show` (#138)

### Changed
- Add name and peers to magnet links (#249)
- Print correct and incorrect MD5 checksums in color (#243)
- Print individual file torrent verification errors (#236)
- Add progress messages and bar to `imdl torrent verify` (#230)
- Style `imdl torrent create` progress messages (#227)
- Only write spinner and progress bar when connected to terminal (#219)
- Fail early if destination .torrent file exists (#220)
- Improve spinner and progress bar (#218)
- Only show torrent summary on create with `--show` (#216)
- Make table names more greppable (#215)
- Improve `imdl torrent create` flags (#208)
- Add short flags to `imdl torrent {show,verify}` (#205)
- Remove `long_help` in favor of `help` (#201)
- Fix torrent verify about message (#172)
- Don't display tier labels if tiers only contain a single entry (#156)
- Show files in single-file mode (#154)
- Skip hidden files, symlinks, and junk in created torrents  (#149)
- Write torrent to stdout if `-` is passed to `--output` (#145)
- Improve torrent display formatting (#142)
- Show information about torrents after creation (#140)
- Restrict piece length (#136)
- Accept `--piece-length` arguments with SI units (#134)
- Update install instructions to new domain (#122)

### Misc
- Make `just done` default to current branch (#259)
- Indicate BEP 9 support in readme (#254)
- Update BEP list in readme with new issue links (#246)
- Use imdl-indicatif (#240)
- Test `imdl torrent verify` output (#239)
- Deny `clippy::too_many_arguments` (#237)
- Add explanation paragraph to readme (#232)
- Use bendy dep from crates.io instead of GitHub (#231)
- Add braille spinner char explanation and legend (#229)
- Use `concat!(...)` to format braille tick chars (#226)
- Remove `matches` macro (#224)
- Rename `Target` to `OutputTarget` (#217)
- Add empty book (#212)
- Test piece-hashing edge cases (#204)
- Test creating torrents from `.` and `..` (#203)
- Revise command line value names (#202)
- Refactor Opt into Arguments, Options, and Subcommand (#200)
- Fix hidden method unused arg warning (#198)
- Trigger GitHub actions build on either push and PR to master (#199)
- Format with unstable rustfmt options (#191)
- Refactor tests and improve verification (#189)
- Test that metainfo bencode representation is correct (#184)
- Update discord invite link (#181)
- Add discord badge to readme (#180)
- Update badges (#179)
- Add Github Actions build badge to readme (#178)
- Use list of SHA1 digests for piece list (#173)
- Drop `serde_bencode` in favor of `bendy` (#160)
- Check for outdated dependencies before publishing (#139)
- Add pyrocore to prior art section of readme (#135)
- Replace favicon with pixel art rainbow text version (#133)
- Add favicon to github pages (#132)
- Fix github pages table jitter (#131)
- Make homepage A E S T H E T I C (#130)
- Disable bors (#129)
- Fix Bors (#128)
- Center symbols in BEP support table key in readme (#127)
- Add Alternatives & Prior Art section to readme (#126)
- Test that `wss://` trackers are accepted (#121)
- Configure Bors (#117)
- Add github pages homepage (#118)
- Add `install` script and installation section to readme (#116)

[v0.0.1] - 2020-1-31
--------------------

### Added
- Open torrents with `imdl create --open ...` (#112)
- [torrent stats] Pretty print torrents if `--print` is passed (#84)
- Add colored output (#66)
- Initial commit

### Misc
- Add table of references to readme (#111)
- Test UDP tracker URLs parse (#110)
- Remove redundant information from the readme (#104)
- Link to tracking issues from BEP support table (#102)
- Add package script (#89)
- Build and upload release artifacts from CI (#88)
- Add `help` messages to CLI flags and options (#87)
- Enable `--help` text wrapping (#76)
- Sort `Create` opt struct fields (#75)
- BEP 3 is supported (#74)
- Slighly improve readability of Hasher::hash_root (#73)
- Add table of contents to readme (#72)
- Add BEP support table to readme (#67)
- Redirect stdin and stdout and capture for tests (#64)
- Rename: Environment -> Env (#63)
- Rename bencode::Error::ExtraData -> TrailingData (#60)
- Fail CI if code isn't formatted (#55)
- Delete extraneous comment in workflow file (#54)
- Run CI tests on windows, macos, and linux (#53)
- Configure clippy and lint on push (#52)
- Add `done` recipe to justfile (#51)
- Add build and test github action workflow (#50)
