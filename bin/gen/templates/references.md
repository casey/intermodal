# {{title}}

{% for entry in entries -%}
- [{{entry.title.clone().unwrap_or(entry.url.to_string())}}]({{entry.url}}) — {{entry.description}}

{% endfor -%}
