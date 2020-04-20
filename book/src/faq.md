FAQ
===


- [Can intermodal be used to preview torrents with `fzf`?](#fzf-preview)



<a name="fzf-preview"></a>
## Can intermodal be used to preview torrents with `fzf`?
</h2>

Yes! [@mustaqimM](https://github.com/mustaqimM) came up with the following:

    fzf --preview='imdl --color always --terminal torrent show --input {}

_Note the use of `--color always` and `--terminal` to force colored, human readable output._

This can be used to, for example, preview the torrents in a directory:

    find . -name '*.torrent' | fzf --preview='imdl -c always -t torrent show -i {}'
