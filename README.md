# everything-search-mcp

Filename search through a [Voidtools Everything](https://www.voidtools.com/)
HTTP server, as an MCP server.

> **Rust rewrite.** This branch replaces the earlier JavaScript implementation,
> which remains on `main`.

```bash
everything-search-mcp                                    # localhost:14680
everything-search-mcp --everything-host 192.168.1.10     # remote instance
EVERYTHING_HTTP_PASSWORD=... everything-search-mcp \
  --everything-username blu                              # with auth
```

## Tools

| Tool | Does |
| --- | --- |
| `everything_search` | Searches filenames using Everything query syntax, e.g. `ext:rs router`. Returns full paths with type and size. |

## Requirements

Everything must be running **with its HTTP server enabled** (Options → HTTP
Server). This is a Windows tool; the server here can run anywhere that can reach
it over HTTP.

If Everything is behind HTTP basic auth, pass `--everything-username` and set
`EVERYTHING_HTTP_PASSWORD`. Prefer the environment variable — a password passed
as a flag is visible in the process list. A 401 response says so explicitly
rather than reporting a generic failure.

## Notes

The response is normalised to `{query, total, returned, results}` with full
paths assembled from Everything's separate directory and name fields, rather
than passing its raw JSON through into the model's context.

This searches **filenames only**. For content search, use
[`fs-mcp`](https://github.com/Bluscream/fs-mcp).

## Options

Everything from [`mcp-toolkit`](https://github.com/Bluscream/mcp-toolkit), plus:

| Flag | Env | Default |
| --- | --- | --- |
| `--everything-host` | `EVERYTHING_HTTP_HOST` | 127.0.0.1 |
| `--everything-port` | `EVERYTHING_HTTP_PORT` | 14680 |
| `--everything-username` | `EVERYTHING_HTTP_USERNAME` | none |
| `--everything-password` | `EVERYTHING_HTTP_PASSWORD` | none |

## Development

```bash
./scripts/build.sh --release
```

## License

[Unlicense](LICENSE) (public domain).
