# Oxi

![](https://img.shields.io/github/v/release/elliotekj/oxi?sort=semver)
![](https://img.shields.io/github/license/elliotekj/oxi)

Oxi wraps an asynchronous, performant [server][1] around [Mercury
Parser][2], providing a simple API to POST to.

### Quick Start

1. Download the latest [release](https://github.com/elliotekj/oxi/releases)
2. Install [`mercury-parser`][3]
3. `$ oxi`
4. `$ http POST http://127.0.0.1:8080/parse url=https://elliotekj.com/2019/12/20/sqlite-ios-advanced-grdb`

## Dependencies

`mercury-parser` must be in your $PATH.

```bash
$ npm -g install @postlight/mercury-parser
```

## Usage

Oxi only has one endpoint: `/parse`. It expects an `application/json`
Content-Type and accepts the parameters below. If Mercury Parser fails to parse
the given URL, Oxi will return [status code 422][4].

| Key    | Value Type         | Description                                                                          |
|--------|--------------------|--------------------------------------------------------------------------------------|
| url    | String             | The url you want to parse.                                                           |
| format | Optional\<String\> | The output format of the article's content. Options: html (default), markdown, text. |

To start Oxi, add it to your $PATH and run `oxi`. By default it will run on
port 8080, but you can use the `-p` flag to customise that. Example:

```bash
$ oxi -p 8181
```

## License

Oxi is released under the MIT
[`LICENSE`][5].

## About

Oxi was written by [Elliot Jackson][6].

- Blog: [https://elliotekj.com/blog][7]
- Hire: [https://elliotekj.com/hire][8]
- Email: elliot@elliotekj.com

[1]: https://github.com/actix/actix-web
[2]: https://github.com/postlight/mercury-parser
[3]: https://github.com/postlight/mercury-parser#the-command-line-parser
[4]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422
[5]: https://github.com/elliotekj/oxi/blob/master/LICENSE
[6]: https://elliotekj.com
[7]: https://elliotekj.com/blog
[8]: https://elliotekj.com/hire
