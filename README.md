# Web Screen

very simple web screen shot with Rust.

## How Download

Go to [release](https://github.com/NoobforAl/webScreen/releases) page and download binary file.  
Or clone project and compile it.

Note: for run this browser need chrome browser.

## How Run Docker

> docker pull noobforal/webscreen  
> docker run -d -p 8080:8080 noobforal/webscreen

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

```webscreen -u https://example.com/```

And screen saved screenShot.png/jpeg.

Note: If you run server mode request to this endpoint:

```http://127.0.0.1:8080/?url=http://google.com&quality=100&format=PNG&timeout=10```

> url : query string  
> quality : query int  
> format : query (PNG or jpeg)  
> timeout : query int  
