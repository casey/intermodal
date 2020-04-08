default: watch

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

bt := '0'

export RUST_BACKTRACE := bt

log := 'warn'

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
done BRANCH=`git rev-parse --abbrev-ref HEAD`: check
	git diff --no-ext-diff --quiet --exit-code
	git push github {{BRANCH}}:master
	git checkout master
	git rebase {{BRANCH}}
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}} --
	git branch -d {{BRANCH}}

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
	mdbook serve book --open --dest-dir ../www/book

# add git log messages to changelog
changes:
	git log --pretty=format:%s >> CHANGELOG.md

dev-deps:
	brew install grip
	cargo install mdbook
	cargo install cargo-watch
	npm install --global asciicast2gif
	brew install imagemagick
	brew install gifsicle

# update readme table of contents
update-toc:
	cargo run --package update-readme toc

generate-completions:
	./bin/generate-completions

man:
	cargo build
	help2man \
		--name 'BitTorrent metainfo utility' \
		--manual 'IMDL MANUAL' \
		--no-info \
		target/debug/imdl \
		> man/imdl.1
	sd '📦 ' "\n" man/imdl.1

check-man: man
	git diff --no-ext-diff --quiet --exit-code

check-minimal-versions:
	./bin/check-minimal-versions

check: test clippy lint check-minimal-versions changelog-update
	git diff --no-ext-diff --quiet --exit-code
	cargo +nightly fmt --all -- --check
	cargo run --package update-readme toc
	git diff --no-ext-diff --quiet --exit-code

draft: push
	hub pull-request -o --draft

pr: push
	hub pull-request -o

merge: check
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! hub ci-status --verbose; do
		sleep 5
	done
	just done

publish-check: check check-man
	cargo outdated --exit-code 1
	grep {{version}} CHANGELOG.md

publish: publish-check
	git branch | grep '* master'
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}
	cargo publish

changelog-update:
	cargo run --package changelog update

changelog-types:
	cargo run --package changelog types

changelog-issue-template:
	cargo run --package changelog issue-template

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
	asciicast2gif tmp/demo.json www/demo.gif

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
