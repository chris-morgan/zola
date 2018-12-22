# zola (né Gutenberg)
[![Build Status](https://travis-ci.com/getzola/zola.svg?branch=master)](https://travis-ci.com/getzola/zola)
[![Build status](https://ci.appveyor.com/api/projects/status/i0ufvx2sdm2cmawo/branch/master?svg=true)](https://ci.appveyor.com/project/Keats/zola/branch/master)

A fast static site generator in a single binary with everything built-in.

Documentation is available on [its site](https://www.getzola.org/documentation/getting-started/installation/) or
in the `docs/content` folder of the repository and the community can use [its forum](https://zola.discourse.group).

## Comparisons with other static site generators

<table>
	<thead>
		<tr>
			<th align="center"></th>
			<th align="center">Zola</th>
			<th align="center">Cobalt</th>
			<th align="center">Hugo</th>
			<th align="center">Pelican</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<th align="left">Single binary</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
		</tr>
		<tr>
			<th align="left">Language</th>
			<td align="center">Rust</td>
			<td align="center">Rust</td>
			<td align="center">Go</td>
			<td align="center">Python</td>
		</tr>
		<tr>
			<th align="left">Supported content formats</th>
			<td align="left" valign="top">Markdown</td>
			<td align="left" valign="top">Markdown</td>
			<td align="left" valign="top">Markdown<br>HTML<br>Org-mode<br>More via helpers</td>
			<td align="left" valign="top">Markdown<br>HTML<br>reStructuredText<br>More via plugins</td>
		</tr>
		<tr>
			<th align="left">Syntax highlighting</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Sass compilation</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Assets co-location</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">i18n</th>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Image processing</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Sane & powerful template engine</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/partial.svg" alt="Partial" title="Partial (see note below)"></td>
			<td align="center"><img src="./docs/static/assets/partial.svg" alt="Subjective" title="Subjective (see note below)"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Themes</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Shortcodes</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Internal links</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Link checker</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Table of contents</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Automatic header anchors</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Aliases</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Pagination</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Custom taxonomies</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
		</tr>
		<tr>
			<th align="left">Search</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Data files</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
		</tr>
		<tr>
			<th align="left">LiveReload</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Netlify support</th>
			<td align="center"><img src="./docs/static/assets/partial.svg" alt="Partial" title="Partial (see note below)"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
		</tr>
		<tr>
			<th align="left">Breadcrumbs</th>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
		</tr>
		<tr>
			<th align="left">Custom output formats</th>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/no.svg" alt="No" title="No"></td>
			<td align="center"><img src="./docs/static/assets/yes.svg" alt="Yes" title="Yes"></td>
			<td align="center"><img src="./docs/static/assets/unknown.svg" alt="Unknown" title="Unknown"></td>
		</tr>
	</tbody>
</table>

### Template engine explanation

Cobalt gets <img src="./docs/static/assets/partial.svg" alt="Partial" title="Partial"> as, while based on [Liquid](https://shopify.github.io/liquid/), the Rust library doesn't implement all its features but there is no documentation on what is and isn't implemented. The errors are also cryptic. Liquid itself is not powerful enough to do some of things you can do in Jinja2, Go templates or Tera.

Hugo gets <img src="./docs/static/assets/partial.svg" alt="Partial" title="Partial">. It is probably the most powerful template engine in the list after Jinja2 (hard to beat python code in templates) but personally drives me insane, to the point of writing my own template engine and static site generator. Yes, this is a bit biased.

### Pelican notes
Many features of Pelican are coming from plugins, which might be tricky
to use because of version mismatch or lacking documentation. Netlify supports Python
and Pipenv but you still need to install your dependencies manually.
