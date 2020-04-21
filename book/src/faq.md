FAQ
===


- [Can intermodal be used to preview torrents with `fzf`?](#fzf-preview)

- [Can intermodal be used to create a torrent from a Git repo?](#git-repo)



<a name="fzf-preview"></a>
## Can intermodal be used to preview torrents with `fzf`?
</h2>

Yes! [@mustaqimM](https://github.com/mustaqimM) came up with the following:

    fzf --preview='imdl --color always --terminal torrent show --input {}

_Note the use of `--color always` and `--terminal` to force colored, human readable output._

This can be used to, for example, preview the torrents in a directory:

    find . -name '*.torrent' | fzf --preview='imdl -c always -t torrent show -i {}'

<a name="git-repo"></a>
## Can intermodal be used to create a torrent from a Git repo?
</h2>

Yes! The `--ignore` flag, contributed by [@Celeo](https://github.com/Celeo), can be used
to make `imdl torrent create` respect `.gitignore` files:

    imdl torrent create --ignore --include-hidden --include-junk --glob '!.git/*' --input .

In addition to `--ignore`, `--include-hidden`, `--include-junk`, and `--glob '!.git/*'`
are used to include files, like `.gitignore`, that are present in the repo but would
otherwise be skipped, and to skip the contents of the `.git` directory.

Equivalently, with short flags:

    imdl torrent create --ignore -hjg '!.git/*' -i .
