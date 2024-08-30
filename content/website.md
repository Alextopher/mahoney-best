---
hidden: true
---

# Website

On the surface this site is very simple with markdown content, but it has a significant [deep web](https://en.wikipedia.org/wiki/Deep_web) presence.

## Markdown rendering pipeline

### Overview

Markdown is rendered with a pretty deep pipeline of transformations.

1. Markdown is parsed into an AST [`comrak`](https://crates.io/crates/comrak)
2. Custom transformations are made to the AST (e.g. syntax highlighting with `syntect`)
3. The transformed AST is rendered into HTML [`comrak`](https://crates.io/crates/comrak)
4. Individual pages are built together now with [`maud`](https://crates.io/crates/maud) components (e.g. header, footer, etc.)
5. A page is compiled as a [`tera`](https://crates.io/crates/tera) template. At this point the HTML is 95% complete are only simple context changes are needed (e.g. page-views).
6. On each request a context is built and the HTML is finalized.

The portfolio/blog focused part of the site ships zero javascript and zero cookies.
