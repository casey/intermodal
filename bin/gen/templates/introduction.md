# Intermodal: A 40' shipping container for the Internet

Intermodal is a user-friendly and featureful command-line BitTorrent metainfo utility for Linux, Windows, and macOS.

Project development is hosted on [GitHub](https://github.com/casey/intermodal).
{%- for example in examples -%}
{%- if !example.unstable %}

{{example.text}}

```sh
$ {{example.code}}
```
{%- endif %}
{%- endfor %}

Functionality that is not yet finalized, but still available for preview, can be accessed with the `--unstable` flag:

{%- for example in examples -%}
{%- if example.unstable %}

{{example.text}}

```sh
$ {{example.code}}
```
{%- endif %}
{%- endfor %}
Happy sharing!
