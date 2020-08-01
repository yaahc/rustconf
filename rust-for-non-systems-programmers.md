<slide data-timing=15>

# Rust <div class=small>for</div> Non-Systems Programmers

<span class=author>Rebecca Turner (<a href="https://pronoun.is/she/her/">she/her</a>)</span>

<fab fa-twitter> [@16kbps] / [becca.ooo]

<slide-footer>
  <left>
    Rebecca Turner (<a href="https://pronoun.is/she/her/">she/her</a>)
  </left>
  <right>
    <fab fa-twitter>
    <a href="https://twitter.com/16kbps">@16kbps</a>
    / <a href="https://becca.ooo">becca.ooo</a>
  </right>
</slide-footer>

Notes: Hey folks, my name is Rebecca Turner, I use [she/her] pronouns, and I'm
going to be talking about Rust for non-systems programmers.

Next slide: Why this talk?

[@16kbps]: https://twitter.com/16kbps
[becca.ooo]: https://becca.ooo/
[she/her]: https://pronoun.is/she/her/

---

<slide class=title-card data-timing=15>

## Why this talk?

Notes: I'm a non-systems programmer --- before learning Rust, I mostly wrote
Python --- and now Rust is pretty much my favorite language.

But if you looked at the [rust-lang.org] website before 2019, that might not
make a lot of sense to you.

Next slide: Screenshot of [rust-lang.org] in 2018.

[rust-lang.org]: https://rust-lang.org/

---

<slide class=image-slide data-timing=17>

![A screenshot of the rust-lang.org website in late 2018. The headline reads
"Rust is a systems programming language that runs blazingly fast,
prevents segfaults, and guarantees thread
safety."](img/rust-2018-11-30.png)

Notes: Here's the [rust-lang.org] website at the end of 2018, right before
they rolled out the new site. The headline emphasizes systems programming,
speed, and memory safety --- all things I don't directly care about that much.

Next slide: Compare that with the new website.

[rust-lang.org]: https://rust-lang.org/

---

<slide class=image-slide data-timing=25>

![A screenshot of the rust-lang.org website in mid-2020. The headline reads
"A language empowering everyone to build reliable and efficient software."
and sections under "Why Rust?" emphasize performance, reliability, and
productivity.](img/rust-2020-07-19.png)

Notes: And here's the same website today in mid-2020. Now Rust is about "empowering
everyone to build reliable and efficent software," and the website focuses on
reliability and productivity. But a lot of the documentation has lagged behind
and still assumes that new Rust programmers already know C++ or something
similar.

Next slide: "I don't understand Rust" tweet

---

<slide class=image-slide>

<tweet>
  hi my names rebecca and i still dont understand rust
  <date>10:12 AM · May 1, 2017</date>
  <likes>2</likes>
</tweet>

Notes: That made it really hard for me to learn Rust. I've never really
understood memory management, so a lot of the documentation was pretty
inaccessible to me. I struggled to figure out how Rust would be used to
actually write meaningful programs --- I'd get caught up on error handling.

So we're going to write a non-trivial Rust program together, and see how we can
solve a lot of common problems in a Rust-y way.

<!-- So I want to introduce the rest of us to Rust. -->

Next slide: What can Rust do for you?

---

<slide class=title-card data-state=peach>

## What can Rust <br> do for you?

Notes: Before we start writing code, let's take a quick look at some of the
things Rust makes strikingly easy.

Next slide: Command-line help messages.

---

<slide data-timing=8>

```plain
rustconf-code 0.1.0
A command-line interface to the openweathermap.org API

USAGE:
    rustconf-code [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Config filename; a JSON file with
                             an `api_key` field
                             [default: openweather_api.json]
```

Notes: Rust can do command-line argument parsing generated from a type definition...

---

<slide data-timing=9>

<pre class="term"><span class=hljs-keyword>$</span> <span class=hljs-title>./target/debug/rustconf-code --confiig cfg.json</span>
<span style="color: #CC0000"><b>error:</b></span> Found argument '<span style="color: #C4A000">--confiig</span>' which wasn't expected,
       or isn't valid in this context
       Did you mean <span style="color: #4E9A06">--config</span>?

USAGE:
    rustconf-code --config &lt;config&gt;

For more information try <span style="color: #4E9A06">--help</span>
</pre>

Notes: ...with automatic typo-correction, while generating tab-completion
scripts and man pages at compile-time.

Next slide: JSON deserialization example.

---

<slide class=center data-timing=10>

```shell-session left
$ ./target/debug/rustconf-code --config bad-schema.json
Error: Failed to deserialize configuration JSON

Caused by:
    invalid type: map, expected a string at line 2 column 14
```

Notes: Rust can give you great error reports for complex errors while
automatically deserializing JSON to a custom type.

Next slide: Pretty test diffs.

---

<slide data-timing=8>

<pre class=term>running 1 test
test tests::str_test ... <span style="color: #CC0000">FAILED</span>

failures:
---- tests::str_test stdout ----
thread 'tests::str_test' panicked at
'assertion failed: `(left == right)`

<b>Diff</b> <span style="color: #CC0000">&lt; left</span> / <span style="color: #4E9A06">right &gt;</span> :
<span style="color: #CC0000">&lt;"Hello, RustConf!"</span>
<span style="color: #4E9A06">&gt;"Hello, RustConf</span><span style="background-color:#005F00"><span style="color: #4E9A06"><b> 2020</b></span></span><span style="color: #4E9A06">!"</span>

', src/main.rs:11:9

test result: <span style="color: #CC0000">FAILED</span>. 0 passed; 1 failed; 0 ignored;
0 measured; 0 filtered out
</pre>

Notes: And Rust can output fancy test diffs with a one-line import that
integrates with the default test framework.

Next slide: (And more.)

---

<slide class=title-card data-state=teal data-timing=8>

## (And a lot more)

Notes: Rust can do a whole lot more, too. But I don't want to just list random
Rust features for 30 minutes.

Next slide: What is this talk?

---

<slide class=title-card>

## What is this talk?

Notes: Before I figured out Rust, I was primarily a Python programmer. I found
--- and still find -- a lot of Rust documentation aimed more at C++ programmers
than Python programmers. I've never really understood memory management, which
means that I don't feel comfortable writing C++ and I don't understand that
sort of documentation.

I also used to struggle to figure out how Rust would be used for actually
*doing* the things I wanted to do --- writing user-level applications.

So we're going to write a non-trivial Rust program together, and see how we can
solve a lot of common problems in a Rust-y way.

Next slide: What is this talk *not*?

---

<slide class=title-card data-state=purple>

## What is this talk *not*?

Notes: There's a lot of stuff that isn't a high priority to me as a Python
programmer in Rust, that I'm going to pretty much skip entirely.

We're not going to optimize anything, because the totally naive program we're
going to write takes 1/10th of a second to run, which is almost entirely
waiting on some network requests.

We're not going to talk about macros, or a lot of the fancy type system
features Rust has, or pointers. I'm not even going to *say* the words "heap" or
"stack" or "allocate" --- if it wouldn't matter in Python or JavaScript or
Ruby, it's out of scope here.

Next slide: Who is this talk for?

---

<slide data-timing=30>

## Who is this talk for?

<list fragments>

- Programmers comfortable in dynamic languages like Python, JavaScript, Ruby,
  etc.
- Who are tired of some of the problems with those languages:
  - Null/undefined errors
  - Runtime type errors
  - Poor documentation
  - Fragmented ecosystem / inconsistent tooling

Notes: In particular, I want to talk to programmers who are already comfortable
in dynamic scripting languages who are beginning to feel some of the downsides
of working in those languages.

Next slide: Why do I like Rust?

---

<slide class=title-card data-state=peach data-timing=45>

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

<slide data-timing=60>

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

<slide class=image-slide data-timing=45>

## Documentation

<media fragments>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/"></iframe>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/fn.thread_rng.html"></iframe>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/rngs/struct.ThreadRng.html"></iframe>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/rngs/struct.ThreadRng.html#implementations"></iframe>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/trait.RngCore.html"></iframe>
<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/trait.RngCore.html#required-methods"></iframe>
</media>

Notes: Here's the generated documentation for the <crate rand> crate, which you can
find at [docs.rs/rand][`rand`].

1. We can see the overview they wrote, and we can search the crate's items ---
   with keyboard shortcuts!
2. If we click on the `thread_rng` function, we get to this definition. Let's
   click the return type and check out the documentation for `ThreadRng`.
3. If we scroll down a bit, we can see the traits `ThreadRng` implements. Let's
   check out the documentation for `RngCore`.
4. We can see a description at first...
5. And if we scroll down, we can see the required methods and their documentation.

Next slide: Hello, world!


[`rand`]: https://docs.rs/rand/

---

<slide data-timing=80>

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

<slide class=center data-timing=5>

```shell-session
$ cargo build
   Compiling rustconf-code v0.1.0 (~/rustconf/rustconf-code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

Notes: We can run `cargo build` to compile the program.

<!-- `cargo build` writes output to `target/debug`, so we can run our program from -->
<!-- there. We can also use `cargo run` to compile and immediately run a program. -->

Next slide: running the program.

---

<slide class=center data-timing=5>

```shell-session
$ ./target/debug/rustconf-code
Hello, Rebecca!
$ USER=nell ./target/debug/rustconf-code
Hello, nell!
```

Notes: And then we can run it, and it does what we expect.

Next slide: ...although, we might not really expect an empty variable.

---

<slide class=center data-timing=15>

```shell-session no-line-numbers [1-2|3-7]
$ env USER= ./target/debug/rustconf-code
Hello, !
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
thread 'main' panicked at 'called `Result::unwrap()` on an
`Err` value: NotUnicode("\xC3(")', src/main.rs:4:16
note: run with `RUST_BACKTRACE=1` environment variable to display
a backtrace
```

Notes: ...although, if the `USER` environment variable is empty, it might be a
bit confusing, and if `USER` contains invalid UTF-8, it'll crash the whole
program.

Next slide: The `Result` type.

---

<slide class=center data-timing=25>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Notes: `Result` is an `enum`, which is a type that can be *one* of a number of
different things. It's also a generic type, so we can pick any two types `T`
and `E` and use a `Result` type which can be either an `Ok` variant containing
a `T` value or an `Err` variant containing an `E` value.

Next slide: gracefully handling errors with `match`.

---

<slide data-timing=20>

```rust no-line-numbers [5-14]
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

Notes: One way we can deal with that error is by matching on it, which is a bit
like an `isinstance` check. Here, we'll just handle an error by printing a
simple message.

Next slide: showing what happens when we run it.

---

<slide class=center data-timing=30>

```shell-session
$ env USER="$(printf '\xc3\x28')" ./target/debug/rustconf-code
I couldn't figure out who you are!
```

Notes: Now, when we run our program with invalid data, we print an error
message instead of crashing. We'll talk about some other ways to handle errors
as we go, but for the definitive rundown check out Jane Ludsby's talk ["Error
Handling Isn't All About Errors"][jane-errors].

But this talk is about Rust's value as a practical programming language,
which means doing more than writing "Hello, world!"s. So lets write a program
in Rust and explore some of the ways the language helps us out.

Next slide: receipt printer, weather program overview.

[jane-errors]: https://rustconf.com/schedule/error-handling-isn-t-all-about-errors

---

<slide class=image-slide data-timing=25>

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

<slide data-timing=35>

```python no-line-numbers [1-12|5-7|8-11|12]
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

Here's a simple call of their API; we 1. load the API key from a JSON file, 2.
make a request, and then 3. print out the response text.

Next slide: Running the example.

---

<slide data-timing=10>

```shell-session
$ ./openweather.py
{"coord":{"lon":-71.24,"lat":42.38},"weather":[{"id":801,"main":
"Clouds","description":"few clouds","icon":"02d"}],"base":
"stations","main":{"temp":298.14,"feels_like":297.91,"temp_min":
296.48,"temp_max":299.82,"pressure":1009,"humidity":57}, ...
```

Notes: And when we run it, we get this minified JSON blob as output.

Let's work on recreating this in Rust.

Next slide: Reading the API key from JSON.

---

<slide data-timing=80>

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
2. Next, we use <crate serde_json> to parse that string into a JSON value.
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
`Clone` lets us deeply copy it, and <crate serde>'s `Deserialize` lets us
deserialize it from JSON --- or, with other serde libraries, XML, YAML, TOML,
Protobufs, and more.

Next slide: Using the `Deserialize` implementation with <crate serde_json>.

---

<slide data-timing=20>

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
other `Deserialize`able values, is handled by <crate serde> and <crate serde_json>.

Next slide: Running this example.


---

<slide data-timing=10>

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

Next slide: Adding the <crate structopt> crate.

---

<slide data-timing=30>

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

Notes: The next change I want to make is adding <crate structopt>, which
derives a command-line interface from a struct definition. Instead of declaring
all our arguments as strings and pulling them out of an untyped hashmap, we
just declare them as struct fields --- which means we get autocompletion for
our command-line options, along with bonuses like detecting that `Option`
fields aren't mandatory and `Vec` fields can have multiple values.

Next slide: Generated help message.

---

<slide data-timing=8>

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

Notes: We get a lot of perks from <crate structopt>, including the great
generated help message.

Next slide: Argument typo help.

---

<slide data-timing=8>

```shell-session
$ ./target/debug/rustconf-code --confgi my-custom-file.json
error: Found argument '--confgi' which wasn't expected, or isn't
       valid in this context
       Did you mean --config?

USAGE:
    rustconf-code --config <config>

For more information try --help
```

Notes: <crate structopt> even helps us with typos by default.

Next slide: Adding <crate eyre>.

---

<slide data-timing=25>

```rust left no-line-numbers [3-10|5,7]


fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let config_json = File::open(&opt.config)?;
    let config: OpenWeatherConfig =
        serde_json::from_reader(&config_json)?;
    println!("Config: {:#?}", config);
    Ok(())
}
```

Notes: The next thing I want to do is add some error reporting, so we don't
have to unwrap everything and cause panics when something fails. The <crate
eyre> crate gives us the beautifully-formatted error messages I showed off at
the beginning of the talk, and has a ton of other functionality we won't
explore here.

1. Now, we can handle errors with the `?` operator, which is a pretty simple but
   important bit of syntax sugar.

Next slide: Syntax sugar for `?`.

---

<slide data-timing=35>

```rust left no-line-numbers [6-9,11-14]


fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let config_json =
        match File::open(&opt.config) {
            Ok(file) => file,
            Err(err) => return Err(err.into()),
        };
    let config: OpenWeatherConfig =
        match serde_json::from_reader(&config_json) {
            Ok(config) => config,
            Err(err) => return Err(err.into()),
        };
    println!("Config: {:#?}", config);
    Ok(())
}
```

Notes: The `?`s are transformed into roughly these match statements; if we have
an `Ok` value, we take that value and use it. Otherwise, we return the `Err`
value from the whole function --- we just bubble up the error to the caller.
It's a little bit like throwing an exception, but we don't quit an arbitrary
series of functions --- we only go up one layer, and the type system doesn't
let us ignore it.

Next slide: `WrapErr` and context.

---

<slide data-timing=30>

```rust no-line-numbers left [1,6-11,14]
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

Notes: Using the `?` operator again, we're going to use the `wrap_err` methods
from <crate eyre>'s `WrapErr` trait to attach context to our errors. We just
write a bit about what we were doing that might have caused an error, and then
that string will get displayed if the error report is printed. It's a pretty
simple step --- provided you do it from the start --- and it makes debugging a
*lot* easier.

Next slide: Error report examples.

---

<slide data-timing=25>

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

Notes: Here we can try to use a nonexistent file or an invalid file for our
config file and we can see the error messages we get.

These are pretty simple, but they're especially useful when we have a bunch of
layers of error context to figure out what we did wrong --- and unlike
exceptions in a lot of languages, we don't just get an enormous unreadable
stack trace by default.

Next slide: Using `reqwest` for HTTP requests.

---

<slide data-timing=15>

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

Notes: Now we're going to use the <crate reqwest> library to make a simple call
to the [openweathermap.org] API.

We create an HTTP client object, call the `get` method with the endpoint URL,
add some query parameters, and send the request.

Next slide: Making the call and printing the result.

---

<slide data-timing=10>

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
    },
}
```

Notes: We can see when we pretty-print the `Response` object, we get all the
fields we might expect --- headers, a status code, and so on.

Next slide: Extracting the response text.

---

<slide data-timing=20>

```rust left
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

Notes: We can also print the response text, which is this big minified JSON
blob. We're going to deserialize that, too, but first let's clean up our
interface to the [openweathermap.org] API.

Next slide: Including a `Client` in the deserialized config.

---

<slide data-timing=25>

```rust
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct OpenWeather {
    api_key: String,

    lat: f64,
    lon: f64,

    #[serde(skip)]
    client: Client,
}
```

Notes: While we're at it, let's unify our config file with our API client --
instead of passing an API key into every function call, we can keep it in the
same struct that holds the <crate reqwest> client. And because the `Client` has a
default value, we can tell serde to use that instead of expecting it in our
config file.

Next slide: Deserializing `OpenWeather` from a reader.

---

<slide data-timing=10>

```rust
fn main() -> eyre::Result<()> {
    // ...
    let config: OpenWeather = serde_json::from_reader(
        &config_json,
    )?;
    // ...
}
```

Notes: Now, we can just read our config object from the same JSON file we were
using before, without even a constructor method.

Next slide: `impl OpenWeather`.

---

<slide data-timing=60>

```rust no-line-numbers [1-10|3|2,6]
impl OpenWeather {
    fn get<Response: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> eyre::Result<Response> {
      // I have discovered a truly marvelous implementation
      // of this method, which this slide is too narrow
      // to contain.
    }
}
```

Notes: Now, to make our API a bit cleaner, let's start implementing methods.
This gives us something that looks a lot like the classes we may have used in
other languages, and although Rust doesn't have inheritance or subtyping,
generic functions and traits can get us pretty close.

An `impl` block lets us put methods on types.

1. Like Python, Rust doesn't have an implicit `this` object you can reference
   --- you need to write it explicitly.
2. We also have angle brackets after the function name to indicate a generic
   function. Here, we have one generic parameter named `Response` and the colon
   indicates a *trait bound,* which means that `Response` has to be a type with
   an implementation of `DeserializeOwned` --- which is exactly what
   `#[derive(Deserialize)]` gives us. Essentially, we've copied a type
   parameter from <crate serde_json>`::from_reader` so that we can deserialize
   any type we define.

Next slide: API response struct definitions.

---

<slide data-timing=10>

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

Notes: We can define structs for the API responses --- these are pretty much
copied from the [openweathermap.org] API docs.

Next slide: `OpenWeather::onecall` method and use.

---

<slide data-timing=45>

```rust no-line-numbers [1-9|11-17]
impl OpenWeather {
    fn onecall(&self) -> eyre::Result<OneCall> {
        self.get(
            "onecall",
            &[ ("exclude", "currently,minutely"),
                ("units", "imperial"), ],
        )
    }
}

fn main() -> eyre::Result<()> {
    // ...
    let onecall: OneCall = config
        .onecall()
        .wrap_err("Failed to deserialize hourly weather data")?;
}
```

Notes:

1. Then, we can define a helper method to make that request directly. Note that
   we don't need to annotate the generic types for the `self.get` call, though
   we can if we want --- the compiler is smart enough to figure out what the
   type parameter needs to be from the return type of `self.get` on its own.

2. And then we can use the new method in our `main` function to get the
   forecast data as a richly-typed struct.

Next slide: `TempDifference`

---

<slide data-timing=15>

```rust
#[derive(Debug, PartialEq)]
enum TempDifference {
    MuchColder,
    Colder,
    Same,
    Warmer,
    MuchWarmer,
}
```

Notes: One thing I want from my forecast is to tell me if today is going to be
warmer or colder than yesterday. So I'll create a `TempDifference` enum, and
then a helper method to get the appropriate `TempDifference` for two floats.


---

<slide data-timing=30>

```rust
impl TempDifference {
    fn from(from: f64, to: f64) -> Self {
        let delta = to - from;
        match delta {
            d if d >  10.0 => TempDifference::MuchWarmer,
            d if d >   5.0 => TempDifference::Warmer,
            d if d < -10.0 => TempDifference::MuchColder,
            d if d <  -5.0 => TempDifference::Colder,
            _ => TempDifference::Same,
        }
    }
}
```

Notes: Here's that constructor function, which takes two floats, calculates
their difference, and matches them to the correct `TempDifference` variant.

We're also adding conditional statements to the match patterns, which helps make
it a bit clearer that we're determining which range `delta` is in.

---

<slide data-timing=40>

```rust no-line-numbers [1-13|1|3|5-12|7-10]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tempdiff() {
        assert_eq!(
            TempDifference::from(50.0, 69.0),
            TempDifference::MuchWarmer
        );
        // And so on, but these slides are small...
    }
}
```

Notes: I'm *really bad* at arithmetic stuff like this, so I want to write a few
tests to make sure I got the subtraction order and everything right.

1. The `#[cfg(test)]` attribute means the entire `test` module is conditionally
   compiled, so our tests don't get lumped in to our other builds.
2. We have to import the functions and values from the parent module --- that
   is, the rest of the file --- explicitly.
3. Then, a test is a function annotated with the `#[test]` attribute.
4. Finally, we can write asserts with the `assert!` and `assert_eq!` macros.

---

<slide data-timing=30>

<pre class="term"><span class=hljs-keyword>$</span> <span class=hljs-title>cargo test</span>
<span style="color: #4E9A06"><b>    Finished</b></span> test [unoptimized + debuginfo] target(s)
             in 0.04s
<span style="color: #4E9A06"><b>     Running</b></span> target/debug/deps/rustconf_code-affd1f0e8

running 1 test
test test::test_tempdiff ... <span style="color: #4E9A06">ok</span>

test result: <span style="color: #4E9A06">ok</span>. 1 passed; 0 failed; 0 ignored;
                 0 measured; 0 filtered out
</pre>

Notes: And we can run our tests to make sure that we've written everything
correctly. Another little thing I like about Rust? The type system lets me
describe and check a lot of my code before it compiles correctly, so I end up
writing tests that crash and fail immediately a *lot* less often than I do in
other languages, which is a big boost to my self-esteem.

Next slide: `Stats`.

---

<slide data-timing=20>

```rust
struct Stats {
    min: f64,
    max: f64,
    avg: f64,
    count: usize,
}
```

Notes: I want to be able to state various things about a collection of
temperatures --- like their range and their average, so this `Stats` struct
will handle that computation.

---

<slide data-timing=40>

```rust
impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            avg: 0.0,
            count: 0,
        }
    }
}
```

Notes: Let's implement the `Default` trait for `Stats`, which gives us a way to
construct a default value for a type. It's like Go's concept of a zero-value,
but Rust doesn't require that every type have a default, because that's not
always meaningful (types like file-handles, for instance, don't have a
reasonable default).

We're picking `f64::INFINITY` for the starting minimum value because every
float is less than infinity, and `f64::NEG_INFINITY` for `max` for the same
reason.

---

<slide data-timing=15>

```rust
impl Stats {
    fn from(itr: impl Iterator<Item = f64>) -> Self {
        let mut ret = Self::default();
        let mut sum = 0.0;
        // ...
    }
}
```

Notes: Now we can construct a `Stats` object from an iterator of floats. We
start by initializing a mutable return value and a sum of the iterator's
elements.

---

<slide data-timing=15>

```rust
impl Stats {
    fn from(itr: impl Iterator<Item = f64>) -> Self {
        // ...
        for i in itr {
            if i < ret.min {
                ret.min = i;
            } else if i > ret.max {
                ret.max = i;
            }
            ret.count += 1;
            sum += i;
        }
        // ...
    }
}
```

Notes: Next, we take each value in the iterator and update the return value's
minimum and maximum values, as well as the element count and running total.

---

<slide data-timing=8>

```rust
impl Stats {
    fn from(itr: impl Iterator<Item = f64>) -> Self {
        // ...
        ret.avg = sum / ret.count as f64;
        ret
    }
}
```

Notes: And finally, we compute the average value and return.

---

<slide data-timing=20>

```rust no-line-numbers [1-2|3-4|6]
let yesterday =
    Stats::from(historical.iter().map(|h| h.feels_like));
let today = Stats::from(
    onecall.hourly.iter().map(|h| h.feels_like).take(24),
);

let diff = TempDifference::from(yesterday.avg, today.avg);
```

Notes: Then, we can gather the temperatures for yesterday into a `Stats`
object.

Note that we're using lazy iterators here, so mapping each data-point to the
temperature it "felt like" doesn't require writing a whole new array.

1. We can do the same thing with our forecast, making sure to limit the
   forecast to 24 hourly points.
2. And then we can get a temperature difference between the two days.

Next slide: Printing the result.

---

<slide data-timing=60>

```rust no-line-numbers [1|2|3-13|8-11|12|1-13]
let today_is_warm = 60.0 <= today.avg && today.avg <= 80.0;
print!("Good morning! Today will be about {:.2}°F ", today.avg);
println!(
    "({min} - {max}°F); that's {diff} {than} yesterday{end}",
    min = today.min,
    max = today.max,
    diff = diff,
    than = match diff {
        TempDifference::Same => "as",
        _ => "than",
    },
    end = if today_is_warm { " :)" } else { "." },
);
```

Notes:

1. First, I want to print a smiley face for good weather, so I'll check if the
   average temperature today is between 60 and 80°F.
2. Then, we'll print the first line, truncating today's average temperature to 2
   decimal places.
3. Then, we're going to print the rest of it. There's a bunch going on here, so
   let's break it down.

   First, because `println!` is a macro, it can do weird things with the syntax
   --- like this keyword-argument syntax that's only used for printing and
   formatting macros.
4. Next, we have a `match` statement --- Rust's `if`/`else` and `match`
   statements return a value, so we can use them inline like this.
5. And then we finish with a smiley face if today is going to be warm, or a
   period otherwise.
6. After printing all the information out, our program is done!

Next slide: Running the final program.

---

```shell-session
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

$ ./target/debug/rustconf-code
Good morning! Today will be about 85.16°F (76.42 - 94.96°F);
that's about the same as yesterday.
```

---

<slide data-timing=20>

<video data-src=img/printing.mp4 data-autoplay type=video/mp4>

---

<slide class=image-slide data-timing=15>

![Me holding a printed receipt in my hand, which reads "Good morning! Today will
be about 77.76°F (69.3 - 89.76°F); that's about the same as yesterday
:)](img/printed-receipt.jpg)

---

<slide class=title-card data-state=purple>

## This is only a taste

Notes: Everything I just talked about is just a tiny portion of what you can do
with Rust --- and what Rust can do for you. There's so many features and tools
I wanted to talk about that I didn't have time for --- adding methods to
foreign types, type-safe numbers and unit conversions.

Next slide: **None.**

---

<slide no-footer>

# Rust <div class=small>for</div> Non-Systems Programmers

<span class=author>Rebecca Turner (<a href="https://pronoun.is/she/her/">she/her</a>)</span>

<fab fa-twitter> [@16kbps] / [becca.ooo]

Notes: Thanks so much for listening, and I hope you do some amazing things with
Rust!
