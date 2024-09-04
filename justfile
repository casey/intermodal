default: watch

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

bt := '0'

export RUST_BACKTRACE := bt

log := 'warn'

export RUST_LOG := log

# watch filesystem for changes and rerun tests
watch +ARGS='test':
	cargo watch --clear --exec '{{ARGS}}'

# show stats about torrents at `PATH`
stats PATH:
	cargo build --release
	time ./target/release/imdl --unstable torrent stats --input {{PATH}}

push:
	! git branch | grep '* master'
	git push github

# clean up feature branch BRANCH
done BRANCH=`git rev-parse --abbrev-ref HEAD`:
	git push github {{BRANCH}}:master
	git rebase github/master master
	git branch -d {{BRANCH}}

test:
	cargo test --all

clippy:
	cargo clippy --all-targets --all-features

fmt:
	cargo +nightly fmt --all

forbid:
	./bin/forbid

preview-readme:
	grip -b README.md

# build and serve the book
book:
	mdbook serve book --open --dest-dir ../www/book

dev-deps:
	brew install grip
	cargo install mdbook
	cargo install cargo-watch
	npm install --global asciicast2gif
	brew install imagemagick
	brew install gifsicle

# update generated documentation
gen:
	cargo build
	cargo run --package gen -- --bin target/debug/imdl all

check-minimal-versions:
	./bin/check-minimal-versions

check: test clippy forbid check-minimal-versions gen
	git diff --no-ext-diff --quiet --exit-code
	cargo +nightly fmt --all -- --check

draft: push
	hub pull-request -o --draft

pr: check push
	hub pull-request -o

merge BRANCH=`git rev-parse --abbrev-ref HEAD`:
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! hub ci-status --verbose {{BRANCH}}; do
		sleep 5
	done
	just done {{BRANCH}}

publish-check: check
	cargo outdated --exit-code 1
	grep '^\[{{version}}\]' target/gen/CHANGELOG.md

publish BRANCH=`git rev-parse --abbrev-ref HEAD`: publish-check (merge BRANCH)
	#!/usr/bin/env bash
	set -euxo pipefail
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}
	while ! hub ci-status --verbose {{BRANCH}}; do
		sleep 5
	done
	cargo publish

# record, upload, and render demo animation
demo: demo-record demo-upload demo-render

demo-record:
	#!/usr/bin/env bash
	set -euxo pipefail
	cargo build --release --all
	rm -f tmp/9front.torrent
	asciinema rec \
		--title "Intermodal {{version}} Demo" \
		--command ./target/release/demo \
		--overwrite \
		tmp/demo.json

demo-upload:
	asciinema upload tmp/demo.json

demo-render:
	../asciicast2gif/asciicast2gif -S4 tmp/demo.json www/demo.gif

# print commit metadata types
commit-types:
	cargo run --package gen -- --bin target/debug/imdl commit-types

# open site index
www:
	open www/index.html

# retrieve large collection of torrents from the Internet Archive
get-torrents:
	aria2c \
		-d dat \
		-x 10 \
		'https://ia802701.us.archive.org/21/items/2014_torrent_archive_organized/torrent_archive_organized.zip'

# download bittorrent.org repository
get-beps:
	git clone git@github.com:bittorrent/bittorrent.org.git tmp/bittorrent.org

build-image:
  podman build -t imdl .
  podman run imdl

test-release:
  -git tag -d test-release
  -git push origin :test-release
  git tag test-release
  git push origin test-release

outdated:
  cargo outdated --workspace --root-deps-only

unused:
  cargo +nightly udeps --workspace

coverage:
  cargo llvm-cov --html
  open target/llvm-cov/html/index.html

update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

update-contributors:
  cargo run --release --package update-contributors
