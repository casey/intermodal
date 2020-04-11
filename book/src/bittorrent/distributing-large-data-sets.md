Distributing Large Data Sets
============================

Even though BitTorrent is well-suited for distributing large amounts of data,
very large torrents can still cause problems. Here are some of the problems you
might encounter, as well as suggestions for how to avoid or ameliorate those
issues.

Intermodal currently uses a single-threaded piece hashing algorithm. If you're
distributing a large data set and hashing time is a problem, please open an
issue! I'm eager to improve hashing performance, but want to make sure I do it
in such a way that real workloads benefit.


Background
----------

In order to support incremental download and verification, as well as
resumption of partial downloads, the contents of a torrent are broken into
pieces.

The length of pieces varies is configurable, and the ideal choice of piece
length depends on many factors, but values between 16KiB and 256KiB are common.
Very large torrents may use much larger piece lengths, like 16MiB.

Each piece is hashed, and `.torrent` files, also referred to as metainfo,
contain a list of those hashes.

For all the example commands, I'll be using `dir` for the directory containing
the data set you want to share.


Issues
------

### `.torrent` file too large

When the amount of data is large, or the piece length is small, the number of
pieces can make the `.torrent` file very big.

To avoid this, you can either break the data into multiple torrents, or make
the piece length larger, so the `.torrent` file contains fewer pieces.

#### Breaking data into multiple torrents

`imdl torrent create` has a `--glob` option that can be used to control which
files are included in a torrent. If your data set is divided into multiple
files, ideally with a consistent naming scheme, this can be used to easily
create multiple torrents with different subsets of the data.

The name of the created torrent is usually derived from the name of the input,
so the output torrent name should be given manually to avoid conflicts:

    $ imdl torrent create -i dir -o a.torrent --glob 'dir/0*'
    $ imdl torrent create -i dir -o b.torrent --glob 'dir/1*'
    $ imdl torrent create -i dir -o c.torrent --glob 'dir/2*'
    # etc…

#### Making the piece length larger

`imdl` has an automatic piece length picker, which should choose a good piece
length. You can see what choices it makes for different torrent sizes with:

    $ imdl torrrent piece-length

Some torrent clients don't do well with piece lengths over 16 MiB, so the piece
length picker will never pick piece lengths over 16 MiB. This can be
overridden by specifying `--piece-length` manually. `--piece-length` takes
SI units, like `KiB`, `MiB`, and `KiB`:

    $ imdl torrent create -i dir --piece-length 128mib


### Too many files

Torrents containing a large number of separate files can cause performance
issues. It's not clear if these performance issues are due to BitTorrent client
implementations, host OS file system issues, or both.

#### Distributing your data set as an ISO image

By distributing your data set as an ISO image, all the files in your torrent
will be packed into a single `.iso` file. Additionally, recipients of the ISO
won't have to decompress the whole data set to browse or extract individual
files.

You can create an ISO with `genisoimage`, which can be installed on Debian or
Ubuntu with:

    $ sudo apt install genisoimage

To create a compressed ISO containing your data set:

    $ genisoimage                \
        -transparent-compression \ # compress data in the ISO
        -untranslated-filenames  \ # don't mangle filenames
        -verbose                 \ # verbose output
        -output data.iso         \ # output path
        -V DATA_SET_NAME         \ # volume name
        dir                      \ # input path

The same command, but with short flags:

    $ genisoimage -zUvo data.iso -V DATA_SET_NAME dir

A torrent can then be created containing the ISO:

    $ imdl torrent create --input data.iso

Users can mount and unmount the ISO on Linux:

    $ sudo mkdir -p /mnt                   # create mount point
    $ sudo mount --read-only data.iso /mnt # mount ISO
    $ sudo umount /mnt                     # unmount when finished

Or MacOS:

    $ hdiutil mount data.iso                 # mount ISO
    # hdiutil unmount /Volumes/DATA_SET_NAME # unmount when finished

On Windows, MacOS, and some Linux desktop environments, ISOs can also be
mounted by double-clicking the file.


### Torrent Client Issues

Some torrent clients don't do well with torrents with large piece sizes, many
files, or a large amount of data.

#### Switch to a `libtorrent`-based client

If you're experiencing issues downloading a large data set, switching torrent
clients may help.

In my personal experience, torrent clients that use Arvid Norberg's
`libtorrent` have done well with large amounts of data.

`libtorrent`'s [Wikipedia page](https://en.wikipedia.org/wiki/Libtorrent) has a
[list](https://en.wikipedia.org/wiki/Libtorrent#Applications) of torrent
clients that use `libtorrent`.


Conclusion
----------

If you have suggestions for this guide, please don't hesitate to open an
[issue](https://github.com/casey/intermodal/issues).

In particular, if you've found particular torrent clients to be good or bad at
downloading large data sets, or have run into issues or found solutions not
covered by this guide, I would love to know!
