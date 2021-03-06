# Rust <div class=small>for</div> Non-Systems Programmers

<span class=author>Rebecca Turner</span>

<fab fa-twitter> [@16kbps] / [becca.ooo]

Notes: Hey folks, my name is Rebecca Turner and I'm going to tell you why you
should be writing non-systems code in Rust.

[@16kbps]: https://twitter.com/16kbps
[becca.ooo]: https://becca.ooo/

<slide-footer>
<left>Rebecca Turner</left>
<right>
<fab fa-twitter> <a href="https://twitter.com/16kbps">@16kbps</a> / <a href="https://becca.ooo">becca.ooo</a>
</right>
</slide-footer>

---

<slide class=title-card data-state=purple>

## Why this talk?

Notes: I'm a non-systems programmer, and I really like Rust. But if you
looked at the [rust-lang.org] website before 2019, that might not make
sense to you.

[rust-lang.org]: https://rust-lang.org/

---

<slide class=image-slide>

![A screenshot of the rust-lang.org website in late 2018. The headline reads
"Rust is a systems programming languaeg that runs blazingly fast,
prevents segfaults, and guarantees thread
safety."](img/rust-2018-11-30.png)

Notes: Here's the [rust-lang.org] website at the end of 2018, right before
they rolled out the new site. The headline emphasizes systems programming,
speed, and memory safety --- all things I don't directly care about that much.

Next slide: Compare that with the new website.

[rust-lang.org]: https://rust-lang.org/

---

<slide class=image-slide>

![A screenshot of the rust-lang.org website in mid-2020. The headline reads
"A language empowering everyone to build reliable and efficient software."
and sections under "Why Rust?" emphasize performance, reliability, and
productivity.](img/rust-2020-07-19.png)

Notes: And here's the same website today in mid-2020. Now we're "empowering
everyone" to and focusing on reliability and productivity. But a lot of the
documentation has lagged behind and still assumes that new Rust programmers
already know C++ or something similar.

So I want to introduce the rest of us to Rust.

Next slide: Who is this talk for?

---

## Who is this talk for?

<list fragments>

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

Next slide: Tooling.

[mem]: https://en.wikipedia.org/wiki/Working_memory

---

## Tooling

<list fragments>

- Documentation: [rustdoc] (API docs) and [mdBook] (long-form guides)
- Language servers: [rls](official) and [rust-analyzer](community)
- Package manager, build system: [Cargo]


Notes: Before we get started, I want to point out a few of the tools that
making writing Rust easy and fun.

- rustdoc compiles `///` doc comments written in Markdown to webpages ---
  complete with search, links, and more.
- mdBook is used for writing longer-form narrative-style documentation; the
  Rust Book and more are written with mdBook and serve as companions to the
  rustdoc documentation.
- Two very good language servers provide autocompletion, definition-jumping,
  quick fixes, and more.
- Cargo is a package manager and build system, integrating with the crates.io
  package repository.

Next slide: A bit more on documentation.

[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[mdbook]: https://rust-lang.github.io/mdBook/
[rls]: https://github.com/rust-lang/rls
[rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer
[cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

---

<slide class=image-slide>

## Documentation

<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/"></iframe>

Notes: Here's the generated documentation for the [`rand`] crate, which you can
find at [docs.rs/rand][`rand`].

[`rand`]: https://docs.rs/rand/

---

## Getting started

```rust no-line-numbers [1-10|1|3-10|4|5-9|5|8]
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

1. The `use` statement imports names from libraries. `::` is used as a path
   separator / namespacing operator.
2. Next, we define a function with the `fn` keyword; The function named
   `main` is the entry point.
3. We call the `var` function in the `env` module, and assign the value it
   returns to `user`; Rust figures out the type for us.

   `env::var` returns a `Result`, so we have to unwrap it, which will crash
   if there's an error.
4. Next, we have an `if` statement, which has braces but no parenthesis.
5. Note that we're also comparing strings with `==` --- rust has operator overloading!
6. Finally, we have this `println!` macro --- the `!` means it's a macro, and
   the string literal there is actually turned into a series of formatting
   instructions at compile time so we don't waste time parsing at runtime.

Next slide: `cargo build`.

---

<slide class=center>

```shell-session
$ cargo build
   Compiling rustconf-code v0.1.0 (~/rustconf/rustconf-code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

Notes: We can run `cargo build` to compile the program.

Next slide: running the program.

---

<slide class=center>

```shell-session
$ ./target/debug/rustconf-code
Hello, Rebecca!
$ USER=nell ./target/debug/rustconf-code
Hello, nell!
```

Notes: And then we can run it, and it does what we expect.

Next slide: ...although, we might not really expect an empty variable.

---

<slide class=center>

```shell-session no-line-numbers [1-2|3-7]
$ env USER= ./target/debug/rustconf-code
Hello, !
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
thread 'main' panicked at 'called `Result::unwrap()` on an
`Err` value: NotUnicode("\xC3(")', src/main.rs:4:16
note: run with `RUST_BACKTRACE=1` environment variable to display
a backtrace
```

Notes: ...although, we might not really expect an empty variable.

Also, invalid UTF-8 will crash the whole program.

Next slide: `Result`

---

<slide class=center>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Notes: `Result` is an `enum` --- what functional programmers call a sum type.
While a tuple has a value from any number of different types at the same time, a
sum type has a value from *one* of a number of different types.

`Result` is generic, so for any types `T` and `E`, we have a type `Result<T,
E>` which can be *either* an `Ok` value of type `T` or an `Err` value of type
`E`; that's pretty much equivalent to a function returning `T` or throwing an
exception `E`.

Next slide: gracefully handling errors with `match`.

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

Notes: One way we can deal with that error is by matching on it, which is a
little bit like a type-safe `isinstance` check. Here, we just handle an error
by printing a simple message.

Next slide: showing what happens when we run it.

---

<slide class=center>

```shell-session
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
I couldn't figure out who you are!
```

Notes: Now, when we run our program, we print an error message instead of
crashing. We'll talk about some other ways to handle errors as we go, but for the
definitive rundown check out Jane Ludsby's talk ["Error Handling Isn't All
About Errors"][jane-errors].

But this talk is about Rust's value as a practical programming language,
which means doing more than writing "Hello, world!"s. So lets write a program
in Rust and explore some of the ways the language helps us out.

Next slide: receipt printer, weather program overview.

[jane-errors]: https://rustconf.com/schedule/error-handling-isn-t-all-about-errors

---

<slide class=image-slide>

![A Star TSP100 Eco futurePRNT 72mm receipt printer, powered on with a
printed receipt showing the RustConf homepage reading "Beaming to screens
across the globe"](img/receipt-printer.jpg)

Notes: I have this receipt printer hooked up to my computer, and it's super fun
to play with --- there's no ink, so paper is incredibly cheap, and they're
designed for restaurants and retail, so they're incredibly durable.

I always forget to check the weather in the morning, so I want to write a
program I can set to run before I wake up that tells me the weather, and how
it'll feel compared to the previous day.

Next slide: Minimal API call with OpenWeather.

---

## Simple API call with [openweathermap.org]

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

[openweathermap.org]: https://openweathermap.org/

Notes: Weather APIs come and go, but right now
[OpenWeather][openweathermap.org] is providing decent data for free --- even if
the default units are Kelvins.

Here's a simple call of their API; we load the API key from a JSON file, we
make a request, and then we print out the response text.

Let's work on recreating this in Rust.

---

```rust no-line-numbers [1-15|4|5-6|7-13]
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

Notes: Here's a start at a line-by-line conversion of that program.

1. We're using `include_str!` here which actually reads a file as UTF-8 at
   *compile time* --- we'll work on opening files in a bit, but this works well
   enough, end this is very much a "the perfect is the enemy of the good" talk.
2. Next, we use `serde_json` to parse that string into a JSON value.
3. Then, we get the `api_key` key out of the object as a string. Each time
   we assert something about the type of a value in this object, we need to
   unwrap it, because we might *not* have a value of the type we want, so we
   need to deal with that somehow.

   Note that this isn't entirely unique to Rust, though --- our
   Python program would also crash if `api_key_obj` wasn't a JSON object, or if
   it didn't have a key named `api_key`, or if the value to that key wasn't a
   string. But Rust makes us be explicit about it.

   That's not entirely a bad thing --- it helps us figure out where errors
   could happen --- but it is awfully verbose and painful to write.

   Fortunately, there's a better way.

Next slide: `Deserialize` derive.

---

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct OpenWeatherConfig {
    api_key: String,
}
```

Notes: Here, we're declaring a struct, which is roughly a class, in the sense
of a blob of data with named fields and methods. Then, we *derive* some traits
for it. Traits are what Python programmers sometimes call protocols, or what
Java calls interfaces. Here, `Debug` lets us pretty-print the struct's data,
`Clone` lets us deeply copy it, and `Deserialize` lets us deserialize it from
JSON --- or, with other serde libraries, XML, YAML, TOML, Protobufs, and more.

Next slide: Using the `Deserialize` implementation with `serde_json`.

---

```rust
use serde::Deserialize;

fn main() {
    let config_json = include_str!("../openweather_api.json");
    let config: OpenWeatherConfig =
        serde_json::from_str(config_json).unwrap();
    println!("{:?}", config);
}
```

Notes: Here's what deserializing to a value looks like. Note that we don't need
to explicitly construct our `OpenWeatherConfig` object --- that, along with
parsing the JSON, matching up keys to fields, and recursively constructing
other `Deserialize`able values, is handled by `serde` and `serde_json`.

Next slide: Running this example.


---

```shell-session
$ cat openweather_api.json
{
  "api_key": "1b13e6aa173ce14137a50095476e653c"
}

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rustconf-code`
OpenWeatherConfig {
    api_key: "1b13e6aa173ce14137a50095476e653c"
}
```

Notes: Now when we run this, we get some nice pretty-printed debug output by default.

That's not my actual API key, by the way. Don't worry.

Next slide: `structopt`.

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

Notes: The next change I want to make is adding `structopt`, which derives a
command-line interface from a struct definition. Instead of declaring all our
arguments as strings and pulling them out of an untyped hashmap, we just declare
them as struct fields --- which means we get autocompletion for our
command-line options, along with bonuses like detecting that `Option` fields
aren't mandatory and `Vec` fields can have multiple values.

Next slide: Generated help message.

---

```shell-session
$ ./target/debug/rustconf-code --help
rustconf-code 0.1.0
A command-line interface to the openweathermap.org API

USAGE:
    rustconf-code --config <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Config filename; a JSON file with
                             an `api_key` field
```

---

```shell-session
$ ./target/debug/rustconf-code --confgi my-custom-file.json
error: Found argument '--confgi' which wasn't expected, or isn't
       valid in this context
       Did you mean --config?

USAGE:
    rustconf-code --config <config>

For more information try --help
```

---

```rust left
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

```rust no-line-numbers left [6-11,14]
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
) -> Result<Response, reqwest::Error> {
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

---

```rust
#[derive(Deserialize, Debug, Clone)]
pub struct OneCall {
    pub hourly: Vec<Hourly>,
    pub daily: Vec<Daily>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hourly {
    pub dt: UnixUTC,
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub clouds: f64,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
}
```
