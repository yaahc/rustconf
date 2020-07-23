# Rust<br> <div class=small>for</div> Non-Systems Programmers

<span class=author>Rebecca Turner</span>

<i class="fab fa-twitter"></i> [@16kbps](https://twitter.com/16kbps)
/ [becca.ooo](https://becca.ooo)

Notes: Hey folks, my name is Rebecca Turner and I'm going to tell you why you
should be writing non-systems code in Rust.

[@16kbps]: https://twitter.com/16kbps
[becca.ooo]: https://becca.ooo/

---

## Why this talk?

Notes: I'm a non-systems programmer, and I really like Rust. But if you
looked at the [rust-lang.org] website before 2019, that might not make
sense to you.

[rust-lang.org]: https://rust-lang.org/

---

<img src=img/rust-2018-11-30.png alt="A screenshot of the rust-lang.org website in late 2018. The headline reads &quot;Rust is a systems programming languaeg that runs blazingly fast, prevents segfaults, and guarantees thread safety.&quot;">

Notes: Here's the [rust-lang.org] website at the end of 2018, right before
they rolled out the new site. The headline emphasizes systems programming,
speed, and memory safety --- all things I don't directly care about that much.

[rust-lang.org]: https://rust-lang.org/

---

<img src=img/rust-2020-07-19.png alt="A screenshot of the rust-lang.org website in mid-2020. The headline reads &quot;A language empowering everyone to build reliable and efficient software.&quot; and sections under &quot;Why Rust?&quot; emphasize performance, reliability, and productivity.">

Notes: And here's the same website today in mid-2020. Now we're "empowering
everyone" to and focusing on reliability and productivity. But a lot of the
documentation has lagged behind and still assumes that new Rust programmers
already know C++ or something similar.

So I want to introduce the rest of us to Rust.

---

## Who is this talk for?

- Programmers comfortable in dynamic languages like Python, JavaScript, Ruby,
  etc.
- Who are tired of some of the problems with those languages:
  - Null/undefined errors
  - Runtime type errors
  - Poor documentation

---

## Why do I like Rust?

Notes: I have ADHD, and it varies from person to person but one area I really
struggle with is [*working memory*][mem], which is roughly how much
information you can hold in your head at once. And as an engineer, that means
that I can't hold much of the program concept in my mind while I work.

It's really important that I have a powerful compiler, linters, and tests
because otherwise I have no way of knowing that the program's correct --- and
I also really need type annotations and autocompletion to remember which
operations are supported on which variables.

Rust *really shines* in all of these areas. I work *with* the compiler to
check my work. And it helps me feel a lot more confident that my programs do
what I think they do.

[mem]: https://en.wikipedia.org/wiki/Working_memory

---

## Tooling

- Documentation: [rustdoc] (API docs) and [mdBook] (long-form guides)
- Language servers: [rls](official) and [rust-analyzer](community)
- Package manager, build system: [Cargo]

[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[mdbook]: https://rust-lang.github.io/mdBook/
[rls]: https://github.com/rust-lang/rls
[rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer
[cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

---

## Documentation

<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/"></iframe>

Notes: Here's the generated documentation for the [`rand`] crate, which you can
find at [docs.rs/rand][`rand`].

[`rand`]: https://docs.rs/rand/

---

## Getting started

```rust no-line-numbers [1-10|1|3-10|3|4|5-9|5|8]
use std::env;

fn main() {
    let user = env::var("USER").unwrap();
    if user == "becca" {
        println!("Hello, Rebecca!");
    } else {
        println!("Hello, {}!", user);
    }
}
```

Notes: Here's a pretty simple rust program, just to show off a bit of syntax.

1. The `use` statement imports names from libraries, here the standard library.
2. Next, we define a function.
3. The function named `main` is the entry point.
4. We call the `var` function in the `env` module, and assign the value it
   returns to `user`; Rust figures out the type for us.
5. Next, we have an `if` statement, which has braces but no parenthesis.
6. Note that we're also comparing strings with `==` --- rust has operator overloading!
7. Finally, we have this `println!` macro --- the `!` means it's a macro, and
   the string literal there is actually turned into a series of formatting
   instructions at compile time so we don't waste time parsing at runtime.

---

```shell-session
$ cargo build
   Compiling rustconf-code v0.1.0 (~/rustconf/rustconf-code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

---

```shell-session
$ ./target/debug/rustconf-code
Hello, Rebecca!
$ env USER=nell ./target/debug/rustconf-code
Hello, nell!
```

---

<pre><code class="shell-session no-line-numbers" data-line-numbers="1-2|3-7" data-trim>
$ env USER= ./target/debug/rustconf-code
Hello, !
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
thread 'main' panicked at 'called `Result::unwrap()` on an
`Err` value: NotUnicode("\xC3(")', src/main.rs:4:16
note: run with `RUST_BACKTRACE=1` environment variable to display
a backtrace
</code></pre>

---

```rust
use std::env;

fn main() {
    let user = env::var("USER");
    match user {
        Ok(user) => {
            if user == "becca" {
                println!("Hello, Rebecca!");
            } else {
                println!("Hello, {}!", user);
            }
        }
        Err(_) => println!("I couldn't figure out who you are!"),
    }
}
```

---

```shell-session
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
I couldn't figure out who you are!
```

---

<img src=img/receipt-printer.jpg alt="A Star TSP100 Eco futurePRNT 72mm receipt printer, powered on with a printed receipt showing the RustConf homepage reading &quot;Beaming to screens across the globe&quot;">

---

```python
import json

import requests

with open("openweather_api.json") as f:
    api_key_obj = json.load(f)
    api_key = api_key_obj["api_key"]
    res = requests.get(
        "https://api.openweathermap.org/data/2.5/weather",
        params={"q": "Waltham,MA,US", "appid": api_key},
    )
    print(res.text)
```

---

```rust
use serde_json::Value;

fn main() {
    let api_key_json = include_str!("../openweather_api.json");
    let api_key_obj: Value =
        serde_json::from_str(api_key_json).unwrap();
    let api_key = api_key_obj
        .as_object()
        .unwrap()
        .get("api_key")
        .unwrap()
        .as_str()
        .unwrap();
    println!("API key is {}", api_key);
}
```

---

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct OpenWeatherConfig {
    api_key: String,
}
```

---

```rust
use serde::Deserialize;

fn main() {
    let config_json = include_str!("../openweather_api.json");
    let config: OpenWeatherConfig =
        serde_json::from_str(config_json).unwrap();
    println!("OpenWeather config: {:?}", config);
}
```

---

```shell-session
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rustconf-code`
OpenWeather config: OpenWeatherConfig {
    api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
}
```

---

```rust
use std::path::PathBuf;

use structopt::StructOpt;

/// A command-line interface to the openweathermap.org API.
#[derive(Debug, StructOpt)]
struct Opt {
    /// Config filename; a JSON file with an `api_key` field.
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // ...
}
```

---

```shell-session
$ ./target/debug/rustconf-code --help
rustconf-code 0.1.0
A command-line interface to the openweathermap.org API

USAGE:
    rustconf-code --config &lt;config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config &lt;config>    Config filename; a JSON file with
                             an `api_key` field
```

---

```shell-session
$ ./target/debug/rustconf-code --confgi my-custom-file.json
error: Found argument '--confgi' which wasn't expected, or isn't
       valid in this context
       Did you mean --config?

USAGE:
    rustconf-code --config &lt;config>

For more information try --help
```

---

```rust
use eyre::WrapErr;

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let config_json = File::open(&opt.config)?;
    let config: OpenWeatherConfig =
        serde_json::from_reader(&config_json)?;
    println!("Config: {:#?}", config);
    Ok(())
}
```

---

```rust
use eyre::WrapErr;

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let config_json =
        File::open(&opt.config).wrap_err_with(|| {
            format!(
                "Failed to open config file {:?}",
                opt.config
            )
        })?;
    let config: OpenWeatherConfig =
        serde_json::from_reader(&config_json)
            .wrap_err("Failed to deserialize JSON")?;
    println!("Config: {:#?}", config);
    Ok(())
}
```

---

```shell-session
$ ./target/debug/rustconf-code --config nonexistent-file.json
Error: Failed to open config file "nonexistent-file.json"

Caused by:
    No such file or directory (os error 2)

$ ./target/debug/rustconf-code --config invalid-file.json
Error: Failed to deserialize JSON

Caused by:
    trailing comma at line 3 column 1
```

---

```rust
use reqwest::blocking::{Client, Response};

fn get_weather(
    api_key: &str,
) -> Result&lt;Response, reqwest::Error> {
    let client = Client::new();
    client
        .get("https://api.openweathermap.org/data/2.5/weather")
        .query(&[("q", "Waltham,MA,US"), ("appid", api_key)])
        .send()
}
```

---

```rust
println!("Response: {:#?}", get_weather(&config.api_key)?);
```

```shell-session
$ cargo run
Response: Response {
    url: "https://api.openweathermap.org/data/2.5/weather?q=...",
    status: 200,
    headers: {
        "server": "openresty",
        "date": "Sun, 19 Jul 2020 19:30:04 GMT",
        "content-type": "application/json; charset=utf-8",
        "content-length": "465",
        "connection": "keep-alive",
        "x-cache-key": "/data/2.5/weather?q=waltham%2cma%2cus",
        "access-control-allow-origin": "*",
        "access-control-allow-credentials": "true",
        "access-control-allow-methods": "GET, POST",
    },
}
```

---

```rust
let res = get_weather(&config.api_key)?;
let bytes = res.bytes()?;
println!("{}", String::from_utf8_lossy(&*bytes));
```

```json
{"coord":{"lon":-71.24,"lat":42.38},"weather":[{"id":802,
"main":"Clouds","description":"scattered clouds","icon":
"03d"}],"base":"stations","main":{"temp":308.71,"feels_like":
307.42,"temp_min":307.59,"temp_max":309.82,"pressure":1010,
"humidity":37},"visibility":10000,"wind":{"speed":6.2, ...
```
