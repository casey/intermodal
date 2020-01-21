default: watch

# watch filesystem for changes and rerun tests
watch:
	cargo watch --exec test

# show stats about torrents at `PATH`
stats PATH:
	cargo build --release
	time ./target/release/imdl --unstable torrent stats --input {{PATH}}

# clean up feature branch BRANCH
done BRANCH:
	git checkout master
	git diff --no-ext-diff --quiet --exit-code
	git pull --rebase github master
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
	git branch -D {{BRANCH}}

test:
	cargo test

lint:
	cargo clippy

preview-readme:
	grip -b README.md

dev-deps:
	brew install grip

# update readme table of contents
update-toc:
	cargo run --package update-readme toc

# update readme table of supported BEPs
update-supported-beps:
	cargo run --package update-readme supported-beps

# retrieve large collection of torrents from the Internet Archive
get-torrents:
	aria2c \
		-d dat \
		-x 10 \
		'https://ia802701.us.archive.org/21/items/2014_torrent_archive_organized/torrent_archive_organized.zip'

# download bittorrent.org repository
get-beps:
	git clone git@github.com:bittorrent/bittorrent.org.git tmp/bittorrent.org
