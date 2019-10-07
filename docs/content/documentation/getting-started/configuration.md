+++
title = "Configuration"
weight = 4
+++

The default configuration will be enough to get Zola running locally but not more than that.
It follows the philosophy of only paying for what you need: almost everything is turned off by default.

To change the config, edit the `config.toml` file.
If you are not familiar with TOML, have a look at [the TOML Spec](https://github.com/toml-lang/toml)
to learn about it.

Only one variable - `base_url` - is mandatory, everything else is optional. You can find all variables
used by Zola config as well as their default values below:


```toml
# Base URL of the site, the only required config argument
base_url = "mywebsite.com"

# Used in feeds by default
title = ""
description = ""
# Can be a name as a string, or an object with keys "name",
# "uri" and "email" (all optional), and string values.
author = ""
# The default language, used in feeds
default_language = "en"

# Theme name to use
theme = ""

# Highlight all code blocks found
highlight_code = false

# Which theme to use for the code highlighting.
# See below for list of accepted values
highlight_theme = "base16-ocean-dark"

# Whether to generate a feed automatically
generate_feed = false

# The filename to use for the feed. Used as the template filename, too.
# Defaults to "atom.xml", which has a builtin template that renders an Atom 1.0 feed.
# There is also a builtin template "rss.xml" that renders an RSS 2.0 feed.
# feed_filename = "atom.xml"

# The number of articles to include in the feed. Will include all items if
# not set (the default).
# feed_limit = 20

# Whether to copy or hardlink files in static/ directory. Useful for sites
# whose static files are large. Note that for this to work, both static/ and
# output directory need to be on the same filesystem. Also, theme's static/
# files are always copies, regardles of this setting. False by default.
# hard_link_static = false

# The taxonomies to be rendered for that site and their configuration
# Example:
#     taxonomies = [
#       {name = "tags", feed = true}, # each tag will have its own feed
#       {name = "tags", lang = "fr"}, # you can have taxonomies with the same name in multiple languages
#       {name = "categories", paginate_by = 5},  # 5 items per page for a term
#       {name = "authors"}, # Basic definition: no feed or pagination
#     ]
#
taxonomies = []

# The additional languages for that site
# Example:
#     languages = [
#       {code = "fr", feed = true}, # there will be a feed for French content
#       {code = "fr", search = true}, # there will be a Search Index for French content
#       {code = "it"}, # there won't be a feed for Italian content
#     ]
#
languages = []

# Whether to compile the Sass files found in the `sass` directory
compile_sass = false

# Whether to build a search index out of the pages and section
# content for the `default_language`
build_search_index = false

# A list of glob patterns specifying asset files to ignore when
# processing the content directory.
# Defaults to none, which means all asset files are copied over to the public folder.
# Example:
#     ignored_content = ["*.{graphml,xlsx}", "temp.*"]
ignored_content = []

# A list of directories to search for additional `.sublime-syntax` files in.
extra_syntaxes = []

# Optional translation object. The key if present should be a language code
[translations]

# You can put any kind of data in there and it
# will be accessible in all templates
[extra]
```

## Syntax highlighting

Zola currently has the following highlight themes available:

- [1337](https://tmtheme-editor.herokuapp.com/#!/editor/theme/1337)
- [agola-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Agola%20Dark)
- [ascetic-white](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Ascetic%20White)
- [axar](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Axar)
- [base16-ocean-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Base16%20Ocean%20Dark)
- [base16-ocean-light](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Base16%20Ocean%20Light)
- [bbedit](https://tmtheme-editor.herokuapp.com/#!/editor/theme/BBEdit)
- [boron](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Boron)
- [charcoal](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Charcoal)
- [cheerfully-light](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Cheerfully%20Light)
- [classic-modified](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Classic%20Modified)
- [demain](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Demain)
- [dimmed-fluid](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Dimmed%20Fluid)
- [dracula](https://draculatheme.com/)
- [gray-matter-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Gray%20Matter%20Dark)
- [gruvbox-dark](https://github.com/morhetz/gruvbox)
- [gruvbox-light](https://github.com/morhetz/gruvbox)
- [idle](https://tmtheme-editor.herokuapp.com/#!/editor/theme/IDLE)
- [inspired-github](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Inspiredgithub)
- [ir-white](https://tmtheme-editor.herokuapp.com/#!/editor/theme/IR_White)
- [kronuz](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Kronuz)
- [material-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Material%20Dark)
- [material-light](https://github.com/morhetz/gruvbox)
- [monokai](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Monokai)
- [solarized-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Solarized%20(dark))
- [solarized-light](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Solarized%20(light))
- [subway-madrid](https://github.com/idleberg/Subway.tmTheme)
- [subway-moscow](https://github.com/idleberg/Subway.tmTheme)
- [visual-studio-dark](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Visual%20Studio%20Dark)
- [ayu-light](https://github.com/dempfi/ayu)
- [ayu-dark](https://github.com/dempfi/ayu)
- [ayu-mirage](https://github.com/dempfi/ayu)
- [Tomorrow](https://tmtheme-editor.herokuapp.com/#!/editor/theme/Tomorrow)
- [one-dark](https://github.com/andresmichel/one-dark-theme)

Zola uses the Sublime Text themes, making it very easy to add more.
If you want a theme not on that list, please open an issue or a pull request on the [Zola repo](https://github.com/getzola/zola).
