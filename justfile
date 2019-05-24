default: watch

# watch filesystem for changes and rerun tests
watch:
	cargo watch --exec test

# show stats about torrents at `PATH`
stats PATH:
	cargo build --release
	time ./target/release/imdl --unstable torrent stats --input {{PATH}}

# retrieve large collection of torrents from the Internet Archive
get-torrents:
	aria2c \
		-d dat \
		-x 10 \
		'https://ia802701.us.archive.org/21/items/2014_torrent_archive_organized/torrent_archive_organized.zip'
