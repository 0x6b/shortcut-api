# Rust API client for Shortcut REST API V3

The API client for [Shortcut](https://www.shortcut.com/) REST API V3, as of 2023-06-14. Swagger/OpenAPI file is available at [their API documentation](https://developer.shortcut.com/api/rest/v3#Swagger-OpenAPI-file).

## Installation

```toml
[dependencies]
shortcut_api = { git = "https://github.com/0x6b/shortcut-api" }
```

## Usage

See [examples/simple.rs](examples/simple.rs) for usage i.e.

```shell
$ export SHORTCUT_API_TOKEN=...
$ cargo run --example simple
```

## Issues

This crate was initially generated by the [OpenAPI Generator](https://openapi-generator.tech) 6.6.0. While there were a few compile errors along the way, I was able to modify it to fit my specific needs. Please report any issues you find.

```console
$ curl -OL https://developer.shortcut.com/api/rest/v3/shortcut.swagger.json
$ npx @openapitools/openapi-generator-cli generate -i shortcut.swagger.json -g rust -o path-to-crate
```

## LICENSE

MIT. See [LICENSE](LICENSE) for details.

## Reference

- [Shortcut Rest API, V3](https://developer.shortcut.com/api/rest/v3) documentation.
- For deprecated [API V2](https://developer.shortcut.com/api/rest/v2), please see
  [JosephLenton/clubhouse-api: An API for Clubhouse. In Rust.](https://github.com/JosephLenton/clubhouse-api).

