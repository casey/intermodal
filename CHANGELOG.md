Changelog
=========

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
