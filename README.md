# confluence_scraper

This application downloads all pages from a Confluence space and saves them as one big HTML file. Run `make download` to do this.

Using [pandoc](https://pandoc.org/), you can then convert the HTML file to a single plain text file by running `make convert`.

The intent is that you can now throw your confluence conten into text processing tools, like NLP tools, to analyze and search the content.

This application expects three environment variables:

- `CONFLUENCE_BASE_URL`: The base URL of your Confluence server, e.g., https://example.atlassian.net/wiki.
- `CONFLUENCE_SPACE_KEY`: The space key of the Confluence space you want to download pages from, e.g., `DEMO`.
- `CONFLUENCE_AUTH`: A base64-encoded string of your Confluence username and API token (or password) separated by a colon, e.g., `dXNlcm5hbWU6YXBpdG9rZW4=`.
