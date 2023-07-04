# Web Screen

very simple web screen shot with Rust.

## Help Cli

```txt
Usage: webscreen [OPTIONS]

Options:
  -u, --url <URL>          send url website [default: ]
      --silent             Run silent Mode App
      --debug              Run Debug Mode App
  -t, --timeout <TIMEOUT>  Set Timeout for requests [default: 10]
  -f, --format <FORMAT>    [default: png]
  -q, --quality <QUALITY>  quality of image 10 - 100 [default: 100]
  -r, --run-server         run tools server
  -p, --port <PORT>        port by default is 8080 [default: 8080]
  -h, --help               Print help
  -V, --version            Print version
```

Example:

> webscreen -u https://example.com/

And screen saved screenShot.png/jpeg.
