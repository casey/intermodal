default: watch

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

bt := "0"

export RUST_BACKTRACE := bt

log := "warn"

export RUST_LOG := log

# watch filesystem for changes and rerun tests
watch +ARGS='':
	cargo watch --clear --exec 'test {{ARGS}}'

# show stats about torrents at `PATH`
stats PATH:
	cargo build --release
	time ./target/release/imdl --unstable torrent stats --input {{PATH}}

push: check
	! git branch | grep '* master'
	git push github

# clean up feature branch BRANCH
done BRANCH=`git rev-parse --abbrev-ref HEAD`:
	git checkout master
	git diff --no-ext-diff --quiet --exit-code
	git pull --rebase github master
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
	git branch -D {{BRANCH}}

test:
	cargo test --all

clippy:
	cargo clippy --all

fmt:
	cargo +nightly fmt --all

lint:
	./bin/lint

preview-readme:
	grip -b README.md

# build and serve the book
book:
	mdbook serve book --open

# add git log messages to changelog
changes:
	git log --pretty=format:%s >> CHANGELOG.md

dev-deps:
	brew install grip
	cargo install mdbook

# update readme table of contents
update-toc:
	cargo run --package update-readme toc

check-minimal-versions:
	./bin/check-minimal-versions

check: test clippy lint check-minimal-versions
	git diff --no-ext-diff --quiet --exit-code
	cargo +nightly fmt --all -- --check
	cargo run --package update-readme toc
	git diff --no-ext-diff --quiet --exit-code

pr: push
	hub pull-request -o

publish-check: check
	cargo outdated --exit-code 1
	git branch | grep '* master'
	grep {{version}} CHANGELOG.md

publish: publish-check
	cargo publish
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}

# open github pages index
open-pages:
	open docs/index.html

# retrieve large collection of torrents from the Internet Archive
get-torrents:
	aria2c \
		-d dat \
		-x 10 \
		'https://ia802701.us.archive.org/21/items/2014_torrent_archive_organized/torrent_archive_organized.zip'

# download bittorrent.org repository
get-beps:
	git clone git@github.com:bittorrent/bittorrent.org.git tmp/bittorrent.org
