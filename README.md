# rhep

A tool to easily find / replace text for config preparation purposes.

## Introduction

Basically, say you want to easily do your own form of configuration management, etc. 
in some sort of handlebars-y way, but all that stuff is overkill.  This utility makes it
easy to invent your own configuration management system using whatever sort of prefix
/ postfix you want, and it doesn't come with much other baggage than making that process
fast-ish.

## Install

```
cargo install rhep
```

## Usage

```
rhep 0.1.0
Aaron Roney <twitchax@gmail.com>
A tool to easily find / replace text for config preparation purposes.

USAGE:
    rhep.exe [OPTIONS] --glob <GLOB> --destination <DESTINATION>

OPTIONS:
    -d, --destination <DESTINATION>
            The directory to write the results to

    -e, --end-sentinel <END_SENTINEL>
            The substring to find which indicates the start of a "sentinel" (The starting token for
            a replacement) [default: }}]

    -g, --glob <GLOB>
            The [glob](https://github.com/rust-lang-nursery/glob) of the files you want to interact
            with

    -h, --help
            Print help information

    -q, --quiet
            Suppress log output

    -r, --replacements <REPLACEMENTS>
            The set of replacements to make

    -s, --start-sentinel <START_SENTINEL>
            The substring to find which indicates the start of a "sentinel" (The starting token for
            a replacement) [default: {{]

    -V, --version
            Print version information
```

## Examples

Say you have a `fly.toml` that looks like this (and it is one of a few others since you will likely have a database, etc.).

```toml
app = "twitchax-app"

kill_signal = "SIGINT"
kill_timeout = 5
processes = []

[build]
  image = "repo/the-app"

[env]
  APP_URI="https://app.twitchax.com"
  APP_SERVER_PORT=8082
  APP_STATIC_LOCATION="/static"

[experimental]
  allowed_public_ports = []
  auto_rollback = true

[[services]]
  http_checks = []
  internal_port = 8082
  processes = ["app"]
  protocol = "tcp"
  script_checks = []

  [[services.ports]]
    force_https = true
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

  [[services.tcp_checks]]
    grace_period = "1s"
    interval = "15s"
    restart_limit = 0
    timeout = "2s"
```

But you really want to generalize it without going through the pain of using some sort of heavy
configuration language.  Well, invent whatever replacement you want, and use rhep to do it.

Here is a _new_ TOML in `config/app/fly.toml` (again, you will have another similar one like `config/db/fly.toml`).

```toml
app = "{{username}}-app"

kill_signal = "SIGINT"
kill_timeout = 5
processes = []

[build]
  image = "repo/the-app"

[env]
  APP_URI="https://app.{{username}}.com"
  APP_SERVER_PORT={{internal_port}}
  APP_STATIC_LOCATION="/static"

[experimental]
  allowed_public_ports = []
  auto_rollback = true

[[services]]
  http_checks = []
  internal_port = {{internal_port}}
  processes = ["app"]
  protocol = "tcp"
  script_checks = []

  [[services.ports]]
    force_https = true
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

  [[services.tcp_checks]]
    grace_period = "1s"
    interval = "15s"
    restart_limit = 0
    timeout = "2s"
```

We will assume that your friend that wants to duplicate this configuration happily has a `username.com`.
Well, your friend can get a zip of your configs, and run...

```bash
rhep -g config/**/*.toml -d my_config -s "{{" -e "}}" -r username=foobar -r internal_port=8080
```

Et voilà, they will get (along with replacements in the other TOML files that match that glob)...

```toml
app = "foobar-app"

kill_signal = "SIGINT"
kill_timeout = 5
processes = []

[build]
  image = "repo/the-app"

[env]
  APP_URI="https://app.foobar.com"
  APP_SERVER_PORT=8080
  APP_STATIC_LOCATION="/static"

[experimental]
  allowed_public_ports = []
  auto_rollback = true

[[services]]
  http_checks = []
  internal_port = 8080
  processes = ["app"]
  protocol = "tcp"
  script_checks = []

  [[services.ports]]
    force_https = true
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

  [[services.tcp_checks]]
    grace_period = "1s"
    interval = "15s"
    restart_limit = 0
    timeout = "2s"
```


## Tests

Yeah, not yet.

## License

```
Copyright (c) 2022 Aaron Roney

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```