Changelog
=========

[v0.1.14](https://github.com/casey/intermodal/releases/tag/v0.1.14) - 2024-09-03
--------------------------------------------------------------------------------

### Added
- Add command to dump bencode ([#530](https://github.com/casey/intermodal/pull/530) by [casey](https://github.com/casey))

### Misc
- Build releases for more targets ([#540](https://github.com/casey/intermodal/pull/540) by [casey](https://github.com/casey))
- Add dockerfile ([#533](https://github.com/casey/intermodal/pull/533) by [rare-magma](https://github.com/rare-magma))
- Fix warnings ([#537](https://github.com/casey/intermodal/pull/537) by [casey](https://github.com/casey))
- Typo: infromation → information ([#532](https://github.com/casey/intermodal/pull/532) by [casey](https://github.com/casey))
- Fix `imdl torrent dump` dictionary formatting ([#531](https://github.com/casey/intermodal/pull/531) by [casey](https://github.com/casey))
- Fix piece selection algorithm in docs ([#524](https://github.com/casey/intermodal/pull/524) by [schnerring](https://github.com/schnerring))
- Un-align metainfo utilities table ([#529](https://github.com/casey/intermodal/pull/529) by [casey](https://github.com/casey))
- Install correct toolchain on CI ([#520](https://github.com/casey/intermodal/pull/520) by [casey](https://github.com/casey))
- Don't auto-generate changelog ([#523](https://github.com/casey/intermodal/pull/523) by [casey](https://github.com/casey))

[v0.1.13](https://github.com/casey/intermodal/releases/tag/v0.1.13) - 2023-08-20
--------------------------------------------------------------------------------
- :bookmark: [`800d082ac9e3`](https://github.com/casey/intermodal/commit/800d082ac9e3699d15425f36b6d2680744eff2c6) Release v0.1.13 - _[atomgardner](mailto:tom@faff.stream)_
- :sparkles: [`a88f0eed688a`](https://github.com/casey/intermodal/commit/a88f0eed688a8b7fc5b599ea6bcc7beef5bc9c8c) Add a `torrent from-link` subcommand - _[atomgardner](mailto:tom@faff.stream)_
- :sparkles: [`50d5a9352dd0`](https://github.com/casey/intermodal/commit/50d5a9352dd03ce329a9e7a238fc15321e6c8bdb) Add a `torrent announce` subcommand - _[atomgardner](mailto:tmg@fastmail.com)_
- :bug: [`660c63101cd0`](https://github.com/casey/intermodal/commit/660c63101cd0857931c9434b101379b058255801) Update dependency (openssl-sys) - _[atomgardner](mailto:tmg@fastmail.com)_
- :package: [`9b434039e1af`](https://github.com/casey/intermodal/commit/9b434039e1af378b137cc7c3632a358892fb1c00) Fix install script - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`984543fcb9ad`](https://github.com/casey/intermodal/commit/984543fcb9ad9a461e041e7a2667436fe4440dca) Don't export rustflags - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`6c4805890b49`](https://github.com/casey/intermodal/commit/6c4805890b49b524974d00b29bcf27fe4498b5e6) Pass rustflags consistently - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`b82ccf1882a4`](https://github.com/casey/intermodal/commit/b82ccf1882a470972e6842ed02436867727ac758) Add `--base-directory` to `imdl torrent verify` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`379a001f47ec`](https://github.com/casey/intermodal/commit/379a001f47eca374dd8cd98b8f7d028fbba8aeac) Fix warnings and clippy errors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`2346c30fec91`](https://github.com/casey/intermodal/commit/2346c30fec91633444accb83837a132379e0ea00) Add Scoop package to README - _[Pika](mailto:15848969+ThatNerdyPikachu@users.noreply.github.com)_
- :art: [`452486a7823d`](https://github.com/casey/intermodal/commit/452486a7823d6515ac82b6a473fa1bfc84ef820e) Placate clippy - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`61bbd3bad521`](https://github.com/casey/intermodal/commit/61bbd3bad521b38da896de15c88ee41b72a8a7b0) Skip generating changelog in tests - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`76ea6e5ed712`](https://github.com/casey/intermodal/commit/76ea6e5ed7127a62017601b073924d349b82af17) Tweak MetainfoDecode error message - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`a7e0995c7d25`](https://github.com/casey/intermodal/commit/a7e0995c7d2505fc0730944fa32f7564a6ebcaea) Replace deprecated add path command - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`05ba87039b6b`](https://github.com/casey/intermodal/commit/05ba87039b6bd0044a4c6f3e19afc0843022e4a6) Add Void Linux package to README - _[Pika](mailto:15848969+ThatNerdyPikachu@users.noreply.github.com)_
- :sparkles: [`97ab785b7c22`](https://github.com/casey/intermodal/commit/97ab785b7c229013217fc5c2534120b951caebe7) Implement FromStr for MagnetLink - _[Thomas Gardner](mailto:tmg@fastmail.com)_
- :zap: [`a787d6a964ed`](https://github.com/casey/intermodal/commit/a787d6a964ede1e049f8861c7758c9d9c8a609f3) Update clippy restrictions - _[Thomas Gardner](mailto:tmg@fastmail.com)_
- :wrench: [`fa78cba0a559`](https://github.com/casey/intermodal/commit/fa78cba0a559eeaa9a3478065d8c88cc8f22a97f) Fix build fix - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`1e4d224dcbbe`](https://github.com/casey/intermodal/commit/1e4d224dcbbe6c807f46e832c11b47d5f858baf8) Fix publish recipe - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.12](https://github.com/casey/intermodal/releases/tag/v0.1.12) - 2020-10-03
--------------------------------------------------------------------------------
- :bookmark: [`164a87d7ad45`](https://github.com/casey/intermodal/commit/164a87d7ad45f507b9dc0922c161c551ce623e49) Release v0.1.12 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`0c17c3da4973`](https://github.com/casey/intermodal/commit/0c17c3da49736b9c7a96ce8190785ecf1040d31f) Optionally print torrent details as JSON - _[Celeo](mailto:mattboulanger@fastmail.com)_
- :books: [`70c1f4e57ccd`](https://github.com/casey/intermodal/commit/70c1f4e57ccd6281ed9c7a563b871d35eb0d0047) Center README header items - _[Celeo](mailto:mattboulanger@fastmail.com)_
- :wrench: [`06b6d2200535`](https://github.com/casey/intermodal/commit/06b6d22005355ea0e15060ab2606bf68a1e6a102) Switch mdbook version back to latest - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`42e20a4a6ad3`](https://github.com/casey/intermodal/commit/42e20a4a6ad3497d85c1886e93240afafb88faa3) Suppress stderr output if --quiet is passed - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`39dcb5e18348`](https://github.com/casey/intermodal/commit/39dcb5e183489da514c219a18bd15f9fec86f8fb) Don't treat head as special in changelog - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`2d226cf01675`](https://github.com/casey/intermodal/commit/2d226cf0167584c4ae8913e675a1f37a5ff4a402) Make changelog author more concise - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.11](https://github.com/casey/intermodal/releases/tag/v0.1.11) - 2020-09-07
--------------------------------------------------------------------------------
- :bookmark: [`8ca27652591d`](https://github.com/casey/intermodal/commit/8ca27652591d4aeaf4b5795cb6e08fcb259e81b3) Release v0.1.11 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`7ca9ed62d5c9`](https://github.com/casey/intermodal/commit/7ca9ed62d5c902d93cc45d1a5c6b3b26868353b3) Mark `Info::infohash` as potentially lossy - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`dbb0eac96daa`](https://github.com/casey/intermodal/commit/dbb0eac96daac509e06644ea4b5a333060c046f8) Update clippy lint names - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`a2a4ea143008`](https://github.com/casey/intermodal/commit/a2a4ea143008d72f9faabac81e0992e35152e8dc) Mark tags that start with `v` as releases - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`a53c2bc5dbfd`](https://github.com/casey/intermodal/commit/a53c2bc5dbfd834cefdeb25fb7b8c5ba8712c1e1) Remove Keybase and IRC links from readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.10](https://github.com/casey/intermodal/releases/tag/v0.1.10) - 2020-06-24
--------------------------------------------------------------------------------
- :bookmark: [`705014c87ea4`](https://github.com/casey/intermodal/commit/705014c87ea42947ca59a168a6622f7a819dd52f) Release v0.1.10 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`04b58464c98c`](https://github.com/casey/intermodal/commit/04b58464c98c5638bdac3f77dbaf666bccbfaea6) Add `--no-git` flag to `gen book` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.9](https://github.com/casey/intermodal/releases/tag/v0.1.9) - 2020-06-24
------------------------------------------------------------------------------
- :bookmark: [`5951139fdfb5`](https://github.com/casey/intermodal/commit/5951139fdfb59df69c5392835bc3432c5f8fa720) Release v0.1.9 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`047b36639db7`](https://github.com/casey/intermodal/commit/047b36639db7d6ed2e8c124618f8070987c3e5a6) Add `--no-git` flag to `gen all` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :racehorse: [`6f4f8e759541`](https://github.com/casey/intermodal/commit/6f4f8e7595413bf04e4ff85cac37950d9d232968) Improve verification performance - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.8](https://github.com/casey/intermodal/releases/tag/v0.1.8) - 2020-05-26
------------------------------------------------------------------------------
- :bookmark: [`9dea195694b4`](https://github.com/casey/intermodal/commit/9dea195694b4cafed5ed341e8b972720614bc650) Release v0.1.8 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`f832605d8139`](https://github.com/casey/intermodal/commit/f832605d813908c593803b16f8632a7b92ed4a5d) Document benchmarks in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :racehorse: [`4e6b475470d0`](https://github.com/casey/intermodal/commit/4e6b475470d00c49feff6623e745a8bab5718394) Benchmark and improve hashing performance - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`1b2d79b4a519`](https://github.com/casey/intermodal/commit/1b2d79b4a519a3645f9cf5ab3d6b25f38e38c807) Split crate into a binary and a library - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :heavy_minus_sign: [`c1c8d3cb89c3`](https://github.com/casey/intermodal/commit/c1c8d3cb89c31a477a9b15cbc7192f6b7af2d37a) Move `data` crate to its own repo - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`173c0e5ac5c7`](https://github.com/casey/intermodal/commit/173c0e5ac5c7542ef80a130b0883ee6e9ece271f) Initial commit of the `data` crate - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`213624cf8eca`](https://github.com/casey/intermodal/commit/213624cf8ecaccb832e14499165767e779a2799c) Metainfo refactoring - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`f4f7a69069b4`](https://github.com/casey/intermodal/commit/f4f7a69069b4c69987f58f031ae5c3b10429e8a8) Add `co-authored-by` field to commit metadata - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`e7872f56f227`](https://github.com/casey/intermodal/commit/e7872f56f22708d8c901f1c733b4ec1d23e978a3) Move all output from `bin/gen` to `target/gen` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`bf29d74b3eeb`](https://github.com/casey/intermodal/commit/bf29d74b3eeb96d37b4bb61a350f205b79ffb1f5) Add changelog to book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`60a72cf0576d`](https://github.com/casey/intermodal/commit/60a72cf0576d44ed67c4530f2839c96b5fbaa96f) Add color to `bin/gen` error messages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`8dfdbe43dff2`](https://github.com/casey/intermodal/commit/8dfdbe43dff2df128dddcd4f29e4a7149d6ce122) Add `bin/gen` command to diff generated content - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`342266853e08`](https://github.com/casey/intermodal/commit/342266853e08f23570ecac253819161cb6f0347f) Improve bin/gen error messages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`e396b7f07106`](https://github.com/casey/intermodal/commit/e396b7f071060647baae49ea40a674264ed6b331) Don't commit changelog - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`f1514dd30194`](https://github.com/casey/intermodal/commit/f1514dd30194c0060653e07c6f8830a9cb90f0fa) Add support for BEP 39. - Fixes [#98](https://github.com/casey/intermodal/issues/98) - _[Annie Cherkaev](mailto:annie.cherk@gmail.com)_
- :books: [`d077da405e1f`](https://github.com/casey/intermodal/commit/d077da405e1f50e4b1f2dc78cf821105640c9110) Improve documentation - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`f8711a79a3d9`](https://github.com/casey/intermodal/commit/f8711a79a3d92da5b30fd3e8b7bba7a5e2d73766) Improve the done and merge recipes - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`97ee5684f8e0`](https://github.com/casey/intermodal/commit/97ee5684f8e0f8de7dbdb4f930018c8df0f012b1) Don't invalid build cache when `Cargo.lock` changes - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`c75ec39b14bb`](https://github.com/casey/intermodal/commit/c75ec39b14bb4375a875d2d2c73718b44eb54e12) Remove watch dropdown image from readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`8fbe39726fe6`](https://github.com/casey/intermodal/commit/8fbe39726fe6c5bdc342dbd0764f8052ddd03597) Add notes for packagers to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`43788cac9a0c`](https://github.com/casey/intermodal/commit/43788cac9a0cb957605fbd16299d04d36a7e0411) Fix `bin/package` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.7](https://github.com/casey/intermodal/releases/tag/v0.1.7) - 2020-04-22
------------------------------------------------------------------------------
- :bookmark: [`f217ac659a14`](https://github.com/casey/intermodal/commit/f217ac659a145f4385b68f20a86b610a02f679f5) Release v0.1.7 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`25ac072985f8`](https://github.com/casey/intermodal/commit/25ac072985f8799474298e6922043746888c0c06) Allow positional input to `imdl torrent show` - Fixes [#375](https://github.com/casey/intermodal/issues/375) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`cecd2f66a5d6`](https://github.com/casey/intermodal/commit/cecd2f66a5d6a6b44f27f8ca499e359d82d29ab7) Fix help strings - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`ebec2d591a7a`](https://github.com/casey/intermodal/commit/ebec2d591a7a0e2a2c4cd55217db4ba46b5dd9ed) Allow positional shell to `imdl completions` - Fixes [#375](https://github.com/casey/intermodal/issues/375) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`134c241ae7f8`](https://github.com/casey/intermodal/commit/134c241ae7f8e374d8a9266e7eb0c4a9c3844c30) Use `lexiclean` crate for lexical path cleaning - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`323434d0aa21`](https://github.com/casey/intermodal/commit/323434d0aa21ebfda5be85ecd4a38a55ed3fec0a) Allow positional input to `imdl torrent verify` - Fixes [#375](https://github.com/casey/intermodal/issues/375) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`5ba885dbc4f2`](https://github.com/casey/intermodal/commit/5ba885dbc4f24781d6a3240ddfc0c03177b12f1e) Take input to `imdl torrent create` as positional - Fixes [#375](https://github.com/casey/intermodal/issues/375) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`c22df5a08326`](https://github.com/casey/intermodal/commit/c22df5a083265b03abd5531b1f5b2aad60aa68cd) Don't commit man pages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`4d67d3a10d17`](https://github.com/casey/intermodal/commit/4d67d3a10d17db3c63af092a936eb5994ee107b1) Don't commit the book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`28114c3d64dd`](https://github.com/casey/intermodal/commit/28114c3d64dde5e0275c936b0019eaf4760ba559) Don't commit shell completion scripts - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`4f4464e3a2a7`](https://github.com/casey/intermodal/commit/4f4464e3a2a7f4aaffea8dbe38dd110ad9be4393) Get `st_flags` from `MetadataExt` on MacOS - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`deca555ac3b3`](https://github.com/casey/intermodal/commit/deca555ac3b3b8f665ee6415f80e05b2bb5e4af7) Allow suppressing output with `--quiet` - Fixes [#174](https://github.com/casey/intermodal/issues/174) - _[Celeo](mailto:celeodor@gmail.com)_
- :books: [`838167c4d3bc`](https://github.com/casey/intermodal/commit/838167c4d3bcbe2fa28f27a00bd94b959ad31e15) Describe in FAQ creating torrent from git repo - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`9b72873ed13e`](https://github.com/casey/intermodal/commit/9b72873ed13e8f0ae747714545c48c6e37c67dd0) Optionally respect `.gitignore` in `imdl torrent create` - Fixes [#378](https://github.com/casey/intermodal/issues/378) - _[Celeo](mailto:celeodor@gmail.com)_
- :books: [`9f480624616b`](https://github.com/casey/intermodal/commit/9f480624616b77995befec722effda22cc2d06ad) Improve FAQ template - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`1380290eb8e2`](https://github.com/casey/intermodal/commit/1380290eb8e222605f368bc8346a1e63c83d9af7) Make `publish-check` recipe stricter - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.6](https://github.com/casey/intermodal/releases/tag/v0.1.6) - 2020-04-20
------------------------------------------------------------------------------
- :bookmark: [`85246d816c93`](https://github.com/casey/intermodal/commit/85246d816c93bef9973d6ab2c0e5aac07ba8c88d) Release v0.1.6 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`1cb11786724a`](https://github.com/casey/intermodal/commit/1cb11786724acef844b41e7ab3b339841f5d920e) Create FAQ - Fixes [#397](https://github.com/casey/intermodal/issues/397) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`65597c98510b`](https://github.com/casey/intermodal/commit/65597c98510b0536247c58a4cead559d3b3817be) Add `--terminal` override terminal autodetection - Fixes [#398](https://github.com/casey/intermodal/issues/398) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`70dbe93c6ab0`](https://github.com/casey/intermodal/commit/70dbe93c6ab01408dfc42e1b75664de13de16a51) Note install script only works on Linux and MacOS - Fixes [#371](https://github.com/casey/intermodal/issues/371) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`b67a2f1885c9`](https://github.com/casey/intermodal/commit/b67a2f1885c9445c08411457809f8893ebfa2045) Fix 404.css link - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`87687f4decbb`](https://github.com/casey/intermodal/commit/87687f4decbbd216b32ea3c9001122c56d5a93fc) Add custom 404 page to site - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`09b0ee316c03`](https://github.com/casey/intermodal/commit/09b0ee316c034848c3b50966e7b5e3ed720aef2b) Document piece length selection algorithm ([#392](https://github.com/casey/intermodal/pull/392)) - Fixes [#367](https://github.com/casey/intermodal/issues/367) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`3ed449ce9325`](https://github.com/casey/intermodal/commit/3ed449ce932509ac88bd4837d74c9cbbb0729da9) Generate reference sections with `bin/gen` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`a6bf75279181`](https://github.com/casey/intermodal/commit/a6bf7527918178821e080db10e65b057f427200d) Use `invariant` instead of `unwrap` and `expect` - Fixes [#167](https://github.com/casey/intermodal/issues/167) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`faf46c0f0e6f`](https://github.com/casey/intermodal/commit/faf46c0f0e6fd4e4f8b504d414a3bf02d7d68e4a) Test that globs match torrent contents - Fixes [#377](https://github.com/casey/intermodal/issues/377) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`0a754d0bcfcf`](https://github.com/casey/intermodal/commit/0a754d0bcfcfd65127d7b6e78d41852df78d3ea2) Add manual Arch install link - Fixes [#373](https://github.com/casey/intermodal/issues/373) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`0a870ed2ee2c`](https://github.com/casey/intermodal/commit/0a870ed2ee2cca79fddb9940fb879354468deb4d) Get current time early when creating torrents - Fixes [#207](https://github.com/casey/intermodal/issues/207) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`9098d3684032`](https://github.com/casey/intermodal/commit/9098d368403232a07684cae8c0b9b1f1383dd2ce) Readme improvements - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`04338e3501af`](https://github.com/casey/intermodal/commit/04338e3501afd155af47d0c4bda2c680d2a7a519) Merge documentation and changelog generation - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`1f8023d13a39`](https://github.com/casey/intermodal/commit/1f8023d13a399e381176c20bbb6a71763b7c352a) Fix directory link in README - _[Matt Boulanger](mailto:Celeo@users.noreply.github.com)_
- :sparkles: [`cb8b5a691945`](https://github.com/casey/intermodal/commit/cb8b5a691945b8108676f95d2888774263be8cc8) Partially implement BEP 53 - Fixes [#245](https://github.com/casey/intermodal/issues/245) - _[strickinato](mailto:aaronstrick@gmail.com)_
- :books: [`6185d6c8a27c`](https://github.com/casey/intermodal/commit/6185d6c8a27c0d603f0434e98000c8e4a868dcc8) Add table of packages to readme ([#372](https://github.com/casey/intermodal/pull/372)) - Fixes [#369](https://github.com/casey/intermodal/issues/369) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`ddf097c83690`](https://github.com/casey/intermodal/commit/ddf097c8369002748992165f81e9a1bdbe6eff98) Fix `publish` recipe ([#368](https://github.com/casey/intermodal/pull/368)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.5](https://github.com/casey/intermodal/releases/tag/v0.1.5) - 2020-04-11
------------------------------------------------------------------------------
- :bookmark: [`707fdea21d61`](https://github.com/casey/intermodal/commit/707fdea21d61b776640950fc84228a2271da02e5) Release v0.1.5 ([#366](https://github.com/casey/intermodal/pull/366)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`727d5c102840`](https://github.com/casey/intermodal/commit/727d5c102840de552822afb82de7475a5183d1f5) Fix Z Shell completions ([#365](https://github.com/casey/intermodal/pull/365)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`f05807290b31`](https://github.com/casey/intermodal/commit/f05807290b314cd68e8679a9aba92e5cd3c5403d) Render command help text in book to avoid wrapping ([#364](https://github.com/casey/intermodal/pull/364)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`ed34ff48a740`](https://github.com/casey/intermodal/commit/ed34ff48a7406adb8b4cdb523b5dc1bf9435e1bc) Add `fuchsi/maketorrent` to prior art table ([#362](https://github.com/casey/intermodal/pull/362)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`a3f46187229f`](https://github.com/casey/intermodal/commit/a3f46187229f499e7a13ec5ce656408ba95d1dcc) Rename distributing large datasets ([#361](https://github.com/casey/intermodal/pull/361)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`1c9ff0cde491`](https://github.com/casey/intermodal/commit/1c9ff0cde4910f369e75930257ce92a8cf4c6cd5) Add suggestions for distributing large datasets to book ([#360](https://github.com/casey/intermodal/pull/360)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`ff6f6d4c3de1`](https://github.com/casey/intermodal/commit/ff6f6d4c3de1a14c6b2ebef270c0ec542300f0de) Test that `--glob`s match entire file paths ([#357](https://github.com/casey/intermodal/pull/357)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`b914c175949f`](https://github.com/casey/intermodal/commit/b914c175949fa6063b6fb0428f4ebd66a51fdda3) Add buildtorretn to prior art section of book ([#355](https://github.com/casey/intermodal/pull/355)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.4](https://github.com/casey/intermodal/releases/tag/v0.1.4) - 2020-04-10
------------------------------------------------------------------------------
- :bookmark: [`f070c62b12f5`](https://github.com/casey/intermodal/commit/f070c62b12f55909c62d461095605f096715b475) Release v0.1.4 ([#354](https://github.com/casey/intermodal/pull/354)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`4dfe537fa515`](https://github.com/casey/intermodal/commit/4dfe537fa515186a6fc65485c6bea16ccd611231) Prevent progress bar from overflowing ([#353](https://github.com/casey/intermodal/pull/353)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`a67eb72848c9`](https://github.com/casey/intermodal/commit/a67eb72848c9f30513fde2849e1f07a332931e6c) Improve install.sh and documentation ([#352](https://github.com/casey/intermodal/pull/352)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`e54bdeb95d93`](https://github.com/casey/intermodal/commit/e54bdeb95d932bd5f81870f34999de37b615a69d) Remove use of unreachable in favor of internal errors ([#351](https://github.com/casey/intermodal/pull/351)) - Fixes [#188](https://github.com/casey/intermodal/issues/188) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`52b78b90f675`](https://github.com/casey/intermodal/commit/52b78b90f6751a72a64074619fbf19df2988ac14) Improve badges ([#350](https://github.com/casey/intermodal/pull/350)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.3](https://github.com/casey/intermodal/releases/tag/v0.1.3) - 2020-04-10
------------------------------------------------------------------------------
- :bookmark: [`8e6a2b0034de`](https://github.com/casey/intermodal/commit/8e6a2b0034debaf60a13f1cea3fa75d92f5d9b50) Release v0.1.3 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`268c62b66d46`](https://github.com/casey/intermodal/commit/268c62b66d46e033786612ce1e85c3c8ee21933a) Add `bin/man` command for generating man pages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`14fff1d888a3`](https://github.com/casey/intermodal/commit/14fff1d888a3d4aebd88059feacde5c665019f30) Make smaller demo for readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`4f8b6a212e80`](https://github.com/casey/intermodal/commit/4f8b6a212e8099ebfcf14600ce92863583758231) Improve demo GIF - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.2](https://github.com/casey/intermodal/releases/tag/v0.1.2) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`685883f02698`](https://github.com/casey/intermodal/commit/685883f02698733e5c8270d52ddf53837b146b0d) Release v0.1.2 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`03a993516481`](https://github.com/casey/intermodal/commit/03a99351648173aa12def1a8f9b9d9ddfe45bbc9) Skip fixup commits in changelog - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`a395052f7c22`](https://github.com/casey/intermodal/commit/a395052f7c226a934cf1b0d75294b1a3146cbeae) Deduplicate progress style string - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`db2a2ac4f556`](https://github.com/casey/intermodal/commit/db2a2ac4f556e7a1a450f9a18d212f082d18fb9d) Refactor demo recipe in justfile into multiple recipes - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`92748f9fd4e6`](https://github.com/casey/intermodal/commit/92748f9fd4e6e65c25e82f2a6e41a0b9b82cf4dd) Make changelog generator strict ([#341](https://github.com/casey/intermodal/pull/341)) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`c33446b48135`](https://github.com/casey/intermodal/commit/c33446b481351009fc16335cbcd66ff2c2b7985e) Generate changelog from git history ([#337](https://github.com/casey/intermodal/pull/337)) - Fixes [#336](https://github.com/casey/intermodal/issues/336) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`6edab1fa3fa8`](https://github.com/casey/intermodal/commit/6edab1fa3fa8461ac4ca02466a04b0f14e42dfc0) Use `TestEnv::assert_ok` everywhere - Fixes [#330](https://github.com/casey/intermodal/issues/330) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`8e3f5516aff8`](https://github.com/casey/intermodal/commit/8e3f5516aff8c89289203a2bc1b46505410c5f1f) Use attractive paths in user-facing messages - Fixes [#252](https://github.com/casey/intermodal/issues/252), [#332](https://github.com/casey/intermodal/issues/332) - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`1cfc0214536c`](https://github.com/casey/intermodal/commit/1cfc0214536c607fff7c29d9e878cbcd7f3a9ffc) Forbid empty input, output, and path targets - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`c23b0635ee25`](https://github.com/casey/intermodal/commit/c23b0635ee25b9d081ec1d5848ee166b1ea225d8) Add ability to create single-file torrents from stdin - _[Eric Siegel](mailto:siegel.eric@gmail.com)_
- :wrench: [`796024bec9a0`](https://github.com/casey/intermodal/commit/796024bec9a0e94e9de2b1f876c99b151e107ead) Split automerge functionality into `merge` recipe - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`4aa8f47da5d8`](https://github.com/casey/intermodal/commit/4aa8f47da5d8c4d997db71b3df3c93567a03e09d) Add "Metadata/Prior Art" section to book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`4c90bc41c3a2`](https://github.com/casey/intermodal/commit/4c90bc41c3a2ab9d8854759dd74beb048b4dfe8d) Rewrite `pr` recipe to merge when CI passes - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`b16da8da5a85`](https://github.com/casey/intermodal/commit/b16da8da5a8589433bd2a70d575cfa5c7cf6c1a1) Rewrite `done` recipe for new merge workflow - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`9d43fa427597`](https://github.com/casey/intermodal/commit/9d43fa42759731f2e35994b8987a634797e16ee2) Fix link to rodarmor's PGP key in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`543524b96de4`](https://github.com/casey/intermodal/commit/543524b96de47704a701f04d2be47448231a72f3) Mention signature policy in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`f17527e58ee6`](https://github.com/casey/intermodal/commit/f17527e58ee6f3718576929e2a9abbeca1b984ab) Use single-quoted strings in justfile - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`25153990817f`](https://github.com/casey/intermodal/commit/25153990817f93a65147125c659703164323ca39) Cache build artifacts - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`83ee172b72cf`](https://github.com/casey/intermodal/commit/83ee172b72cf21c6c9e692888e0304ff88cf42e0) Create section for bittorrent book chapters - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`2ba24bb98528`](https://github.com/casey/intermodal/commit/2ba24bb98528e86be722a09950016908987fccfa) Add description of UDP tracker protocol to book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`57e482f4b32b`](https://github.com/casey/intermodal/commit/57e482f4b32bba52e701d8631272db83f2c23f2c) Record demo for readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`a5e127318747`](https://github.com/casey/intermodal/commit/a5e1273187472f5762b11339d46cebffcf211168) Update man page with new version number - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.1](https://github.com/casey/intermodal/releases/tag/v0.1.1) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`93c23d29f24b`](https://github.com/casey/intermodal/commit/93c23d29f24be5750744b1a65d93f96d86e73862) Release v0.1.1 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`3131b0d56c7f`](https://github.com/casey/intermodal/commit/3131b0d56c7f472916d3a5e34402f94adddbf4d0) Generate man page with from `--help` with `help2man` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`8c30205b0263`](https://github.com/casey/intermodal/commit/8c30205b02634c713f734f016b3ea8ca2f4b9555) Add shell completion scripts - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`e69c65a38b6a`](https://github.com/casey/intermodal/commit/e69c65a38b6a6b8683123976731cebb98125946a) Remove errant torrent file - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.1.0](https://github.com/casey/intermodal/releases/tag/v0.1.0) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`328a3adeafe3`](https://github.com/casey/intermodal/commit/328a3adeafe385fb281a580cf585ee9a6ebbeaf3) Release v0.1.0 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`97018031c1ab`](https://github.com/casey/intermodal/commit/97018031c1abaaf12c1cdc8f645aa9417c1937c8) Introduce "sort specs" to allow fine-grained sorting of files in torrents - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`362a81d42fb9`](https://github.com/casey/intermodal/commit/362a81d42fb9e703f70330b007b701812a22aef5) Use `strum` crate to derive enum↔string conversions - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`6996d1a3eac8`](https://github.com/casey/intermodal/commit/6996d1a3eac88dc3ba1a2e73063d468867ac5b76) List supported OSs in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`4f7eead16a9a`](https://github.com/casey/intermodal/commit/4f7eead16a9ac0b659930322cc43c752ec91d74b) Link to blog post in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`1cd6c276fdf7`](https://github.com/casey/intermodal/commit/1cd6c276fdf795d916ba78d09c4d9e5e3ff992d0) Allow sorting files in torrents - _[Eric](mailto:siegel.eric@gmail.com)_
- :wrench: [`687a863b45a5`](https://github.com/casey/intermodal/commit/687a863b45a53a01aff431b49b288b88420e92fb) Add `cargo install cargo-watch` to dev-deps recipe - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`4b6191773aab`](https://github.com/casey/intermodal/commit/4b6191773aabd445551820ed338c77d907d723c2) Fix condition in GHA workflow - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`b27ecf1c09d6`](https://github.com/casey/intermodal/commit/b27ecf1c09d686c81893ddead727dd15eec5b143) Only deploy site during linux run of CI build - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`b6cb3d389c98`](https://github.com/casey/intermodal/commit/b6cb3d389c987dd60d4234a8004a2d68d89a120f) Go back to sans-serif `I` in site logo - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`ef1acd58c105`](https://github.com/casey/intermodal/commit/ef1acd58c105b2c2e16cf394695f8699b28ec46f) Use serifed `I` for intermodal on site - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`4fc0f508e6a0`](https://github.com/casey/intermodal/commit/4fc0f508e6a0d6b781a770a4e4b5be146e951929) Add glow to `intermodal` letters on site - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`89e97144e958`](https://github.com/casey/intermodal/commit/89e97144e958dd7277dcda0185e08e52e22548e8) Improve readme intro - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`6d5f489f1934`](https://github.com/casey/intermodal/commit/6d5f489f19347c56edc9562855215e7a2ed413ae) Add links to site - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`6328118c0052`](https://github.com/casey/intermodal/commit/6328118c0052ffbc40a6f300cb2f18e315680558) Use `open` crate to open files and URLs - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`35d90adab4fd`](https://github.com/casey/intermodal/commit/35d90adab4fdc1e931e9a0a6c44cad36f8df2e6b) Rename `www/head` to `www/head.txt` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`0469f7fecd29`](https://github.com/casey/intermodal/commit/0469f7fecd296c9e0ac2360a71c81ae7c3462f1e) Record current git revision in `www/head` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`972a1ec6288c`](https://github.com/casey/intermodal/commit/972a1ec6288c94ca851eb0d83b509b3bc4d200b7) Merge Github Actions workflows - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`151d912156c7`](https://github.com/casey/intermodal/commit/151d912156c76dbda271a1c2b6e1a1861f32be04) Deny warnings in GitHub Actions main workflow build - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`e91b419af532`](https://github.com/casey/intermodal/commit/e91b419af5323c7b7d5ed3c0b9a180d91b74b502) Improve book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`1e5c19d02b5d`](https://github.com/casey/intermodal/commit/1e5c19d02b5d770a3270f850420fbebc5a95dbf6) Add GitHub Actions workflow to build book and push to github pages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`b96c27e9b123`](https://github.com/casey/intermodal/commit/b96c27e9b123f8182355e9f0451857d594871ce6) Fix build errors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`4cbeca802ac2`](https://github.com/casey/intermodal/commit/4cbeca802ac207a13ce8099e3521d2cc15e878bd) Add additional documents to release archive - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.0.3](https://github.com/casey/intermodal/releases/tag/v0.0.3) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`5d4baa22eacb`](https://github.com/casey/intermodal/commit/5d4baa22eacb953941d9ad3ba937ea854f15015d) Release v0.0.3 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`86aeec3ce90f`](https://github.com/casey/intermodal/commit/86aeec3ce90f9e62fde842cf3a2f71e8e972842d) Fix release process issues - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.0.2](https://github.com/casey/intermodal/releases/tag/v0.0.2) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`8b3954ff9965`](https://github.com/casey/intermodal/commit/8b3954ff996590fd8b30c6bbcd734ec94f07d644) Release v0.0.2 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`498549b35c2f`](https://github.com/casey/intermodal/commit/498549b35c2fcc6cc7f10019cad15aaea2785dfb) Allow reading torrent metainfo from stdin - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`1c84172ad48a`](https://github.com/casey/intermodal/commit/1c84172ad48a7b9a8a59e91d28594026c520440c) Skip torrent file creation with `imdl torrent create --dry-run` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`7e3a53ce52de`](https://github.com/casey/intermodal/commit/7e3a53ce52deaf51eeb755b86034a4f171380dc5) Make `just done` default to current branch - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`0d7c1c0c279e`](https://github.com/casey/intermodal/commit/0d7c1c0c279e0bca1e1f68a2ef653b5151b7516a) Print magnet link to stdout with `--link` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`901fa150ff54`](https://github.com/casey/intermodal/commit/901fa150ff545c67e230f89ca6a66ae5867005c9) Indicate BEP 9 support in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`d8055c6e6a6c`](https://github.com/casey/intermodal/commit/d8055c6e6a6ca84b87e504a5cf88e9c84c869191) Allow opening magnet links after creation - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`aeb9d8b31721`](https://github.com/casey/intermodal/commit/aeb9d8b31721ad5b4bfe042d3779be7d5007dd6c) Add name and peers to magnet links - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`57a358e4580f`](https://github.com/casey/intermodal/commit/57a358e4580f6e2c07e10bf095e7b14c6a50ea70) Allow creating magnet links with `imdl torrent link` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`0b486cc6817c`](https://github.com/casey/intermodal/commit/0b486cc6817ccb4993e67cdb565dd24cca09d9f9) Update BEP list in readme with new issue links - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`3257614c4fff`](https://github.com/casey/intermodal/commit/3257614c4fff99470839ac503c544a3e0f6bb197) Print correct and incorrect MD5 checksums in color - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`1cac9ab9246f`](https://github.com/casey/intermodal/commit/1cac9ab9246faf66419c74e1085d08baa4ffc435) Use imdl-indicatif - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`0033e8381f59`](https://github.com/casey/intermodal/commit/0033e8381f597a349fa551d3f1a96bdfcb544bba) Test `imdl torrent verify` output - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`2ea5e0b3840d`](https://github.com/casey/intermodal/commit/2ea5e0b3840df8239ae31a4ad5715df292091d7b) Deny `clippy::too_many_arguments` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`1532113782ec`](https://github.com/casey/intermodal/commit/1532113782ecb9fb5ca00d9fd0b0bb18c49acd5e) Print individual file torrent verification errors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`f8e3fd594b50`](https://github.com/casey/intermodal/commit/f8e3fd594b505b42c7ff5c72aa67ff9987dc3934) Add explanation paragraph to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`f1cc12858cee`](https://github.com/casey/intermodal/commit/f1cc12858cee387d27c63b6cec101a16af44b62f) Use bendy dep from crates.io instead of GitHub - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`1daa18ef9a7a`](https://github.com/casey/intermodal/commit/1daa18ef9a7abee47dd60ab7e098667e1f134d89) Add progress messages and bar to `imdl torrent verify` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`5a0bd2dda75b`](https://github.com/casey/intermodal/commit/5a0bd2dda75b3d02765dd3db7cea0cca37fcb9d8) Add braille spinner char explanation and legend - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`2edf8a4fab1e`](https://github.com/casey/intermodal/commit/2edf8a4fab1e49b82c8c7473d33f6494401ea7af) Style `imdl torrent create` progress messages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`8c0d9186079b`](https://github.com/casey/intermodal/commit/8c0d9186079b1ac57b7aedb0a1a83c2f037102d1) Use `concat!(...)` to format braille tick chars - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`c3879db06f82`](https://github.com/casey/intermodal/commit/c3879db06f82b7c59e027df918a01899672ef843) Remove `matches` macro - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`bb34936c2ffd`](https://github.com/casey/intermodal/commit/bb34936c2ffd5e4caaea92a91bc132ee117021ed) Only write spinner and progress bar when connected to terminal - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`2cfdad2597d6`](https://github.com/casey/intermodal/commit/2cfdad2597d6f2265c5e7da647138add7cd170be) Fail early if destination .torrent file exists - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`4371bb14023b`](https://github.com/casey/intermodal/commit/4371bb14023bb4f439ab9b10e6c4f9584d62d6f1) Improve spinner and progress bar - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`b25b389ae66b`](https://github.com/casey/intermodal/commit/b25b389ae66b38c00f013353a5a0ca97ee08d499) Rename `Target` to `OutputTarget` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`0de2b1719e55`](https://github.com/casey/intermodal/commit/0de2b1719e5546fa8e23b3c0678fa13f6e658573) Only show torrent summary on create with `--show` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`06947fd63e87`](https://github.com/casey/intermodal/commit/06947fd63e87e747d26efc3f85580eafb3a10cfa) Make table names more greppable - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`bdaec27cafd4`](https://github.com/casey/intermodal/commit/bdaec27cafd49d9d2cecbffdfdbe72a0544d5ffd) Add piece hashing progress bar - _[RJ Rybarczyk](mailto:rj@rybar.tech)_
- :sparkles: [`c6cd78f56594`](https://github.com/casey/intermodal/commit/c6cd78f56594c2c101e678f03e84007cc4b352a6) Add progress messages to `imdl torrent create` - _[RJ Rybarczyk](mailto:rj@rybar.tech)_
- :books: [`2415d88d9256`](https://github.com/casey/intermodal/commit/2415d88d92567ee2a863bb9db96ebdac80769c4e) Add empty book - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`21a87a46f330`](https://github.com/casey/intermodal/commit/21a87a46f3301bb5bc737adc321198b790ca4161) Improve `imdl torrent create` flags - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`3276f2d043d9`](https://github.com/casey/intermodal/commit/3276f2d043d93d4c478c3187cc2e179f781df049) Add short flags to `imdl torrent {show,verify}` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`027b229df112`](https://github.com/casey/intermodal/commit/027b229df112380481d118d6f695796cd483a923) Test piece-hashing edge cases - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`2b19a6213456`](https://github.com/casey/intermodal/commit/2b19a6213456ac6e93bee389b32ee6991ac6e31b) Test creating torrents from `.` and `..` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`fa6d4e6ad013`](https://github.com/casey/intermodal/commit/fa6d4e6ad0130740359e633b2e59435e366cc13e) Revise command line value names - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`cba238470d3c`](https://github.com/casey/intermodal/commit/cba238470d3c615d49284f4015875ff60361d83f) Remove `long_help` in favor of `help` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`4fffa777b4af`](https://github.com/casey/intermodal/commit/4fffa777b4af5afee208cfffb7d6de5b4972aaf6) Refactor Opt into Arguments, Options, and Subcommand - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`eb8efaf52843`](https://github.com/casey/intermodal/commit/eb8efaf52843a41abc40d889c9e4147aa7de5a72) Fix hidden method unused arg warning - _[RJ Rybarczyk](mailto:rj@rybar.tech)_
- :wrench: [`1c22623df48f`](https://github.com/casey/intermodal/commit/1c22623df48f039b2e3fd984371688e3226e6f1e) Trigger GitHub actions build on either push and PR to master - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`f2a5f137294e`](https://github.com/casey/intermodal/commit/f2a5f137294ef4ffc49efd4feb164d6349dbd2f7) Format with unstable rustfmt options - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`d71bdffda15b`](https://github.com/casey/intermodal/commit/d71bdffda15b2de30313e692661420698c026a36) Refactor tests and improve verification - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`2fb5bdb93344`](https://github.com/casey/intermodal/commit/2fb5bdb93344ba554ef1ecefb5706f380cc2bcee) Test that metainfo bencode representation is correct - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`fdb18bb8d807`](https://github.com/casey/intermodal/commit/fdb18bb8d8077c2b8c88bcd5a4e5a9118bd001a1) Update discord invite link - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`55f2fcc73874`](https://github.com/casey/intermodal/commit/55f2fcc738741f914ea799026b63831b28f2a57b) Add discord badge to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`355f76b86d3a`](https://github.com/casey/intermodal/commit/355f76b86d3ad69cad7b09b2a92c1419922148ec) Update badges - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`b0962722c7db`](https://github.com/casey/intermodal/commit/b0962722c7dbcccc0779a2743486c0b1a9d1619c) Add Github Actions build badge to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`1227628306f0`](https://github.com/casey/intermodal/commit/1227628306f046ef7154ad3a514b1746b41e59d9) Use list of SHA1 digests for piece list - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :bug: [`9787344d9eee`](https://github.com/casey/intermodal/commit/9787344d9eeed0251e963be2d8e9d2777d023b55) Fix torrent verify about message - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`165a7ea444b0`](https://github.com/casey/intermodal/commit/165a7ea444b00376c17ac7275381311b5bf7dd23) Support adding DHT bootstrap nodes to created torrents - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`6549850dac2e`](https://github.com/casey/intermodal/commit/6549850dac2eb57f7ea2e67a1a4b50bd4a35b1d4) Add initial implementation of `imdl torrent verify` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`b0f449b6aedb`](https://github.com/casey/intermodal/commit/b0f449b6aedb78f185f8c7ecb451391b40de11e3) Drop `serde_bencode` in favor of `bendy` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`9b696f78a7df`](https://github.com/casey/intermodal/commit/9b696f78a7dfb19fde1d6eb5d2805af450e0d335) Don't display tier labels if tiers only contain a single entry - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`f7e9e80f972d`](https://github.com/casey/intermodal/commit/f7e9e80f972deae401ab40e7afdf6ddd107b4d4c) Show files in single-file mode - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`43d87c06b6b5`](https://github.com/casey/intermodal/commit/43d87c06b6b5a3b1c6b21f088a56da1892279700) Display torrent file tree - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`a574368ffca6`](https://github.com/casey/intermodal/commit/a574368ffca6738393950bfe2412a26d66a41d17) Allow including and excluding files from torrent with globs - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`9158c230dfd0`](https://github.com/casey/intermodal/commit/9158c230dfd0a7ec90847b05399137012501a613) Skip hidden files, symlinks, and junk in created torrents - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`3739a92857fc`](https://github.com/casey/intermodal/commit/3739a92857fcc402c5063361ff1637fd7fb0b87e) Support creating multi-file torrents - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`551617de4f09`](https://github.com/casey/intermodal/commit/551617de4f09370e34a279884a1b38f5fb7f702c) Don't overwrite destination torrent without `--force` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`df3326510c4b`](https://github.com/casey/intermodal/commit/df3326510c4bd924a7c4cba5520ba156a533924e) Write torrent to stdout if `-` is passed to `--output` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`5a1de1acd219`](https://github.com/casey/intermodal/commit/5a1de1acd219bc5e83c678fccaf862aee40713be) Select piece length when none is provided - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`35a0e8f9b73f`](https://github.com/casey/intermodal/commit/35a0e8f9b73f2271a372e46b5cc72db952e02ae0) Improve torrent display formatting - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`5c5dac1fe532`](https://github.com/casey/intermodal/commit/5c5dac1fe53247ea5893953e503aa074d19e9a38) Add source to generated torrent with `--source` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`b9ca02fbaabc`](https://github.com/casey/intermodal/commit/b9ca02fbaabcfb601fc91ee8762ddea2f41dff9e) Show information about torrents after creation - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`3971854eaa44`](https://github.com/casey/intermodal/commit/3971854eaa44eda7ebc20dbfe8e6c6659b9bad98) Check for outdated dependencies before publishing - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`99a069a02184`](https://github.com/casey/intermodal/commit/99a069a02184c451eb81a632bae1cbda2f6195ea) Add `imdl torrent show` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`6df45e024463`](https://github.com/casey/intermodal/commit/6df45e024463e0d10cd391e8758f797692a6f762) Restrict piece length - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`85f02d9f2992`](https://github.com/casey/intermodal/commit/85f02d9f2992f64208d35f4d2776c27e33861b89) Add pyrocore to prior art section of readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`635692fdfac1`](https://github.com/casey/intermodal/commit/635692fdfac195f0eb0f8184d97280788e5cd8ee) Accept `--piece-length` arguments with SI units - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`eb6556ae6aa2`](https://github.com/casey/intermodal/commit/eb6556ae6aa2106faf0f87043e4dcbcdc3fba35e) Replace favicon with pixel art rainbow text version - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`5611f359686d`](https://github.com/casey/intermodal/commit/5611f359686d67f738b355dd3612b1febe751ed9) Add favicon to github pages - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`3f91f042c771`](https://github.com/casey/intermodal/commit/3f91f042c7717cef93350782617286df18c4f335) Fix github pages table jitter - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`9ff627c29fd9`](https://github.com/casey/intermodal/commit/9ff627c29fd9b761be997370fa950da92417b76c) Make homepage A E S T H E T I C - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`460cf9ee0e79`](https://github.com/casey/intermodal/commit/460cf9ee0e79f2551eacdbb1299c2fd366a49a4c) Disable bors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`2512b05ece5e`](https://github.com/casey/intermodal/commit/2512b05ece5ee679691dde9172a7d69213ed8576) Fix Bors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`e2c1f291605c`](https://github.com/casey/intermodal/commit/e2c1f291605cb54685d86ce9eb3afb4d383fc9fe) Center symbols in BEP support table key in readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`48ca86c67ce7`](https://github.com/casey/intermodal/commit/48ca86c67ce75204f13a48d3373f35e09fea7a5e) Add Alternatives & Prior Art section to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`ef5be39f9b61`](https://github.com/casey/intermodal/commit/ef5be39f9b61033468cbe64b5340be45e56b5fc7) Update install instructions to new domain - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`6461a00ba37a`](https://github.com/casey/intermodal/commit/6461a00ba37ab29770637e172303be83a282ecaa) Test that `wss://` trackers are accepted - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`a6745e2a4187`](https://github.com/casey/intermodal/commit/a6745e2a4187e151354ae1e0dced6210abb2cba5) Configure Bors - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`dc95bc53b27b`](https://github.com/casey/intermodal/commit/dc95bc53b27b2e0f3f6f2f29c66d59cd05590d44) Add github pages homepage - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`e2cf93efb054`](https://github.com/casey/intermodal/commit/e2cf93efb054629f14d2a837494b2790fc5a36a1) Add `install` script and installation section to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_


[v0.0.1](https://github.com/casey/intermodal/releases/tag/v0.0.1) - 2020-04-08
------------------------------------------------------------------------------
- :bookmark: [`9dd8a5629814`](https://github.com/casey/intermodal/commit/9dd8a56298143f80e6ecc85311bdbdba0b914ef5) Release v0.0.1 - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`e8ab0e1c4f68`](https://github.com/casey/intermodal/commit/e8ab0e1c4f687d2f98a90add9d954efb44282058) Open torrents with `imdl create --open ...` - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`495316e82110`](https://github.com/casey/intermodal/commit/495316e8211038b55ec0a12daaa6d70a31d7eaee) Add table of references to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`a8446c82db9e`](https://github.com/casey/intermodal/commit/a8446c82db9e54a96d967f4028b68fc8fce39653) Test UDP tracker URLs parse - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`0560181a099f`](https://github.com/casey/intermodal/commit/0560181a099f5f1e47c030256c0715df5f036e14) Remove redundant information from the readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`1a20f0d0b35d`](https://github.com/casey/intermodal/commit/1a20f0d0b35dd8aa05f783d0660e87a014000dc8) Link to tracking issues from BEP support table - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`ff3a30ae2138`](https://github.com/casey/intermodal/commit/ff3a30ae21388988076e4c892565b0e679076152) Add package script - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :package: [`cf59a58c6733`](https://github.com/casey/intermodal/commit/cf59a58c67331fef371011d562abb9f1f1bf6437) Build and upload release artifacts from CI - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`70e0091ec03c`](https://github.com/casey/intermodal/commit/70e0091ec03ca0014974ebf98673f3e37a1fea75) Add `help` messages to CLI flags and options - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`b83d8d7ef53b`](https://github.com/casey/intermodal/commit/b83d8d7ef53bac73e7066cfd3f9497a62fc9233b) [torrent stats] Pretty print torrents if `--print` is passed - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :zap: [`7d5e68f1941a`](https://github.com/casey/intermodal/commit/7d5e68f1941a63f3ce24218d43d03cafa6e65a44) Enable `--help` text wrapping - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`87e9b982b6be`](https://github.com/casey/intermodal/commit/87e9b982b6be4cfc4b362863e4110770f88bf4b5) Sort `Create` opt struct fields - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`801a51926564`](https://github.com/casey/intermodal/commit/801a51926564918ab0b4d08a185dc799859e73e2) BEP 3 is supported - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`1749fce2b227`](https://github.com/casey/intermodal/commit/1749fce2b227efc446ca478891ea5f63b62f5c2d) Slighly improve readability of Hasher::hash_root - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`1f5b829742f4`](https://github.com/casey/intermodal/commit/1f5b829742f4b0a355587ecc8f33d2a8bfe13118) Add table of contents to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :books: [`66d44155f05a`](https://github.com/casey/intermodal/commit/66d44155f05aab5d9af315831df2f3a68e257d51) Add BEP support table to readme - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`d1f8f24d8e18`](https://github.com/casey/intermodal/commit/d1f8f24d8e184bb79e3e8bd17c024a70d545d2a2) Add colored output - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`b334fa49b2d8`](https://github.com/casey/intermodal/commit/b334fa49b2d84905197533354afb302a61f3499b) Redirect stdin and stdout and capture for tests - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`fd06943726cb`](https://github.com/casey/intermodal/commit/fd06943726cbc864b32401213d6958c7164b7851) Rename: Environment -> Env - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`7420c91553c6`](https://github.com/casey/intermodal/commit/7420c91553c622849fd2de4aaa1d4c1a6e7def3b) Rename bencode::Error::ExtraData -> TrailingData - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`7de1c0477623`](https://github.com/casey/intermodal/commit/7de1c047762318722c09b9099890996894b82b92) Fail CI if code isn't formatted - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :art: [`408735621e51`](https://github.com/casey/intermodal/commit/408735621e51a5b3a67652cc602f1f429bfe7ca1) Delete extraneous comment in workflow file - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`074a3b3d42dd`](https://github.com/casey/intermodal/commit/074a3b3d42dd6b4037cd31383751e3f2e048366d) Run CI tests on windows, macos, and linux - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`9f8366137477`](https://github.com/casey/intermodal/commit/9f83661374776715108fe3876153bdd0a001894b) Configure clippy and lint on push - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :wrench: [`7f630a1bd937`](https://github.com/casey/intermodal/commit/7f630a1bd9376e2461b55ac66126147277ab5dfe) Add `done` recipe to justfile - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :white_check_mark: [`3affa8c5e14b`](https://github.com/casey/intermodal/commit/3affa8c5e14b40a7be88705e5ab7d7fc492a3a69) Add build and test github action workflow - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
- :sparkles: [`98335f435de2`](https://github.com/casey/intermodal/commit/98335f435de2bc0cc80abafdea9536dc945fb11c) Initial commit - _[Casey Rodarmor](mailto:casey@rodarmor.com)_
