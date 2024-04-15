# mahoney.best

This is my personal website. It's a work in progress.

## Structure

This project isn't so much 1 website as it is a collection of services on a single domain. The website is built together by mixing and matching these services together.

Each service is given it's own top-level url representing a single letter.

| TODO | Path | Meaning    | Description |
| ---- | ---- | ---------- | ----------- |
| xxxx | /m   | "markdown" | markdown renderer |
|      | /t   | "tiny"     | url shortener |
|      | /r   | "redirect" | url redirector |
| xxxx | /s   | "static"   | static files baked into the binary |
|      | /f   | "file"     | simple file upload/download server |
|      | /w   | "who"      | ip geo-locator, ip/user agent logger, ip reflector |

## Append-Only

This project is backed by a custom append-only database. I'm able to keep track of all data changes and fall back to any previous state.
