# nu plugin servo

My second attempt at cramming [servo][] into a [nushell][] plugin.  
It works, but is still missing a lot and it might take a while before i finish it.

You are viewing the development-branch. Please use a [release][] instead.  
You can view a changelog [here](./CHANGELOG.md).

## Commands

* `servo html parse`: `string` -> `$format`
* `servo html query <css-query>`: `string` -> `list<$format>`
* `servo xml parse`: `string` -> `$format`
* `servo xml query <css-query>`: `string` -> `list<$format>`
* `servo data-url parse`: `string` -> `record<..>`
* `servo mime parse`: `string` -> `record<..>`

Notes:
* you can `alias 'from html' = servo html parse` - it is not done by default since the format is.. unique
* the `xml` parser is very error resilient. this means you can NOT use it to validate it.

## Data-Formats

almost all commands support the `--format <string>` flag.
possible values:
* `html` (HTML node) (default for `servo html` commands)
* `xml` (XML node) (default for `servo xml` commands)
* `from xml` (same as nu's `from xml`)
* `inner html` (`string`)
* `outer html` (`string`)

### HTML node:

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

### XML node:

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

## Build Flags

* `xml`: the XML commands (adds the `scraper_backend`)
* backends (multiple can be active at once):
  * `scraper_backend`: uses the [scraper][] crate (supports XML)
  * `blitz_backend`: uses the [blitz][] project (will in the future hopefully make it possible to run and render HTML in `nu_plugin_servo`)

## Credits

* [servo][]: all the parsing, etc
* [nushell][]
* [scraper][] (one backend)
* [blitz][] (one backend)

[servo]: https://servo.org
[nushell]: https://nushell.sh
[scraper]: https://crates.io/crates/scraper
[blitz]: https://github.com/DioxusLabs/blitz

[release]: https://github.com/Jan9103/nu_plugin_servo/releases
