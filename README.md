# nu plugin servo

My second attempt at cramming [servo][] into a [nushell][] plugin.  
It works, but is still missing a lot and it might take a while before i finish it.

## Commands

* `servo html parse`: `string` -> `html_node`
* `servo html query <css-query>`: `string` -> `list<string>`
* `servo html query_parse <css-query>`: `string` -> `list<html_node>`
* `servo xml parse`: `string` -> `xml_node`
* `servo xml query <css-query>`: `string` -> `list<string>`
* `servo xml query_parse <css-query>`: `string` -> `list<xml_node>`

Notes:
* you can `alias 'from html' = servo html parse` - it is not done by default since the format is.. unique
* the `xml` parser is very error resilient. this means you can NOT use it to validate it.

## Data-Formats

**html node:** (example)

```nushell
{
  'tag': 'div'
  'attributes': {}  # string -> string map
  'id': null  # or string
  'classes': ['navbar_item']
  'content': [
    'foo'  # string (text content)
    $html_node  # another node
  ]
}
```

**xml node:** (example)

if you pass `--from-xml-compat` it will have the same format as `from xml` instead.

```nushell
{
  'tag': 'article'
  'attributes': {}  # string -> string map
  'content': [
    'foo'  # text node
    $xml_node  # another node
  ]
}
```

## Credits

* [servo][]: all the parsing, etc
* [nushell][]
* [scraper][]: internal [DOM](https://en.wikipedia.org/wiki/Document_Object_Model), css-query

[servo]: https://servo.org
[nushell]: https://nushell.sh
[scraper]: https://crates.io/crates/scraper
