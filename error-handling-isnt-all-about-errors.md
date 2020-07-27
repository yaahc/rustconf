# Error Handling Isn't All About Errors

<span class=author>Jane Lusby</span>

<fab fa-twitter> [@yaahc_] / [yaah.dev]

Slide Template by Rebecca Turner <fab fa-twitter> [@16kbps] / [becca.ooo]

Notes: Hello and welcome to my talk, error handling isn't all about errors.

Next slide: Let me start by introducing myself...

<slide-footer>
<left>Jane Lusby</left>
<right>
<fab fa-twitter> <a href="https://twitter.com/yaahc_">@yaahc_</a> / <a href="https://yaah.dev/">yaah.dev</a>
</right>
</slide-footer>

[@yaahc_]: https://twitter.com/yaahc_
[yaah.dev]: https://yaah.dev/
[@16kbps]: https://twitter.com/16kbps
[becca.ooo]: https://becca.ooo/

---

<slide class=center>

## About Me

Notes: My name is Jane Lusby. On the internet I go by Yaah or Yaahc. I've
been writing rust for two and a half years though I was only recently hired
to do so professionally, by The Zcash Foundation. I got into error handling
on accident, it started as a yak shave when I wanted to open source a library
I wrote for work but I wasn't happy with the error handling and decided to
fix it up first.

That yak shave ended with me writing eyre, a fork of anyhow with support for
customized error report hooks, and color-eyre, which provides a custom panic
hook and a custom eyre report hook and lets you construct error reports like
this.

Next slide: Show the various usage examples from `color-eyre`.

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run --example usage
Error:
   0: <font color="#F15D22">Unable to read config</font>
   1: <font color="#F15D22">No such file or directory (os error 2)</font>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">32</font>
   1: <font color="#F15D22">usage::read_config</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">38</font>

<font color="#34E2E2">Suggestion</font>: try using a file that exists next time

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</pre>

Notes: this is the basic usage example, with an error section, a spantrace section which, if you're not familiar with tracing is this extremely cool backtrace-like type of tracing spans..., a suggestion, and an env setting section.

---


<pre class=term><font color="#CC0000"><b>❯</b></font> RUST_BACKTRACE=1 cargo run --example usage
// ...
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  <font color="#34E2E2">                       ⋮ 5 frames hidden ⋮                       </font>
   6: <font color="#F15D22">usage::read_file</font><font color="#88807C">::h10b2389c2b814452</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">35</font>
   7: <font color="#F15D22">usage::read_config</font><font color="#88807C">::hf7150b146edb25d9</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">40</font>
   8: <font color="#F15D22">usage::main</font><font color="#88807C">::hc3df11a6ea0d044d</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">11</font>
  <font color="#34E2E2">                      ⋮ 10 frames hidden ⋮                       </font>
// ...
Run with RUST_BACKTRACE=full to include source snippets.</pre>

Notes: we can pretty print backtraces and hide unimportant frames, here you can see...

Next slide: we can also filter our backtrace frames, note that here there are 10 frames hidden after main...

---

<pre class=term><font color="#CC0000"><b>❯</b></font> RUST_BACKTRACE=1 cargo run --example custom_filter
// ...
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  <font color="#34E2E2">                       ⋮ 5 frames hidden ⋮                       </font>
   6: <font color="#F15D22">custom_filter::read_file</font><font color="#88807C">::h0afee8fe0960bf02</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/custom_filter.rs</font>:<font color="#75507B">53</font>
   7: <font color="#F15D22">custom_filter::read_config</font><font color="#88807C">::h6622065848c69b29</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/custom_filter.rs</font>:<font color="#75507B">58</font>
  <font color="#34E2E2">                      ⋮ 11 frames hidden ⋮                       </font>
// ...
Run with RUST_BACKTRACE=full to include source snippets.</pre>

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run --example custom_section
Error:
   0: <font color="#F15D22">Unable to read config</font>
   1: <font color="#F15D22">cmd exited with non-zero status code</font>

Stderr:
   cat: fake_file: No such file or directory

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <font color="#F15D22">custom_section::output2</font> with <font color="#34E2E2">self=&quot;cat&quot; &quot;fake_file&quot;</font>
      at <font color="#75507B">examples/custom_section.rs</font>:<font color="#75507B">14</font>
   1: <font color="#F15D22">custom_section::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
      at <font color="#75507B">examples/custom_section.rs</font>:<font color="#75507B">58</font>
   2: <font color="#F15D22">custom_section::read_config</font>
      at <font color="#75507B">examples/custom_section.rs</font>:<font color="#75507B">63</font></pre>

Notes: We can add custom sections, here you can see I've added the section
for Stderr

Next slide: We will dig into this example more later...

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run --example multiple_errors
Error:
   0: <font color="#F15D22">encountered multiple errors</font>

Error:
   0: <font color="#F15D22">The task could not be completed</font>
   1: <font color="#F15D22">The task you ran encountered an error</font>

Error:
   0: <font color="#F15D22">The machine is unreachable</font>
   1: <font color="#F15D22">The machine you&apos;re connecting to is actively on fire</font>

Error:
   0: <font color="#F15D22">The file could not be parsed</font>
   1: <font color="#F15D22">The file you&apos;re parsing is literally written in c++ instead of rust, what the hell</font></pre>

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run --example panic_hook
<font color="#CC0000">The application panicked (crashed).</font>
Message:  <font color="#06989A">some real basic stuff went wrong</font>
Location: <font color="#75507B">examples/panic_hook.rs</font>:<font color="#75507B">14</font>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <font color="#F15D22">panic_hook::do_thing</font> with <font color="#34E2E2">thing=42</font>
      at <font color="#75507B">examples/panic_hook.rs</font>:<font color="#75507B">12</font>

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</pre>

Notes: And we can be consistent when reporting, here you can see a panic
that produces almost identical output to our Eyre Reports.

Now, I'm not giving this talk to talk about eyre.

Next slide: I'm giving this talk to share what I learned in that yak shave to
fix the error handling in my library, and how it has changed how I look at
error handling and error reporting.

---

## What Is Error Handling?

<list fragments>

- <del>Annoying</del>
- Defining errors
- Propagating errors and gathering context
- Reacting to specific errors
- Discarding errors
- Reporting errors and gathered context


Notes: A lot of things, when you zoom in close

don't mention the annoying, just keep going

The breakdown here gets to the goal of my talk. I have a theory that
error handling is made more confusing by people try to simplify it, because,
among other things, error handling is annoying. I worry that the fuzziness
between these different responsibilities makes it hard for people to infer
what tools they should be using when "handling errors". My hope is that by breaking error handling into it's component parts we can make it easier to understand and explain.

Next slide: So let's start with the fundamentals. Note, this first bit is taken almost word for word from The Rust Book's chapter on error handling.


---

<slide class=title-card data-state=purple>

# Recoverable<br> vs<br> Non-Recoverable

Notes: The Rust model for errors distinguishes between two classes of errors.

Recoverable errors are errors you can reasonably expect to occur during execution of..., can be reacted to, or reported.

Unrecoverable errors are bugs, like index out of bounds. can’t be reacted to, only reported before exiting the program / thread

Most languages dont distinguish between these kinds of errors

C++ has exceptions

Rust doesnt

Rust has panic for unrecoverable errors and result recoverable errors

---

## Panic

```rust []
// if the index is past the end of the slice
} else if self.end > slice.len() {
    panic!(
        "index {} out of range for slice of length {}",
        self.end,
        slice.len()
    );
}
```

Notes: Unrecoverable errors in rust are created via the `panic!` macro. Here we can see an example of an index out of bounds error.

Next slide: Only input is an error message and optional some context

---

## Panic

```rust [4|5-6]
// if the index is past the end of the slice
} else if self.end > slice.len() {
    panic!(
        "index {} out of range for slice of length {}",
        self.end,
        slice.len()
    );
}
```

Notes: Reporting and default context gathering done by panic hook

once its done printing the report the panic handler cleans up either by unwinding or aborting

Next slide: Recoverable errors are modeled in rust with the enum `Result<T, E>`.

---

## Result

```rust [1-6|2-3|4-5]
enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
```

Notes: This enum has two variants,

This is used to combine two return types in one and assign meaning to each possibility.

This enum concisely describes whether and how errors are returned.

Next slide: The big advantage of using enums is we must handle all errors.

---

## Result

```rust [1-4]
match result {
    Ok(success) => println!("we got the value {}!", success),
    Err(error) => println!("uh oh we got an error: {}", error),
}
```

Notes: Next slide: next is the try trait and operator...

---

## Try and `?`

```rust left [1-4|8]
let config = match get_config() {
    Ok(success_value) => success_value,
    Err(error_value) => return Err(Error::from(error_value)),
};

// vs

let config = get_config()?;
```

Notes: The unstable try trait models fallible operations and is currently used to define how to convert a type to and from a Result.

Indeed, Result is type that implements the Try trait, as does Option, and other some combinations thereof.

Abstraction over error propagation

Next slide: Next up is the error trait.

---

## The Error Trait

<list fragments>

- Representing an open set of errors
- Reacting to specific errors in an open set.
- Reporting Interface for all errors

Notes: fills three roles

Represent them by converting them to a trait object.

You can then later react to specific errors by downcasting back to the original type, rather than using match as you would with enums.

Next slide: Now, what do I mean by this?

---

## The Error Trait

<list fragments>

- Backtrace via `backtrace()`
- Cause via `source()`
- Error Message via `Display`

Notes: The error trait is how reporters access context that was captured for them.

This includes...

---

<slide class=title-card data-state=purple>

# The error trait provides an interface _for_ reporters.

Next slide: In other languages there is no distinction between errors and
reporters, and this is largely due the lack of an equivalent to the Error
Trait.

---

## The Error Trait

```rust []
trait GoError {
    fn msg(&self) -> String;
}

trait CppError {
    fn msg(&self) -> &'spooky str;
}
```

Notes: The error trait equivalent in other languages is often quite simple, just a single fn to grab the error message.

These interfaces force you to either include your source error, your
backtrace, and any other information you care about in your error message or
to avoid using the provided interface all together.

In rust we don't have to combine our messages all into one, in fact, you're
encouraged not to. Including a source error's message in your `Display`
implementation and returning it as your source via the Error trait is
essentially a bug, and it forces reporters to duplicate information when they
print out the chain of error messages.

Next slide: By separating the source and the error message we move the
responsibility of formatting away from the errors themselves, making it
easier to get fancy.

---

## The Error Trait

<pre class=term><font color="#CC0000">ERROR</font> <b>read_config</b>:<b>read_file{</b>path=&quot;fake_file&quot;<b>}</b>: Error: Unable
to read config: No such file or directory (os error 2)

// vs

Error:
   0: <font color="#F15D22">Unable to read config</font>
   1: <font color="#F15D22">No such file or directory (os error 2)</font>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">52</font>
   1: <font color="#F15D22">usage::read_config</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">58</font></pre>

Notes: In rust we can have the same error print to a log as one line, but the
screen as many.

This wouldn't be possible if the error trait didn't separate context from errors.

Next slide: However, despite the fact that the error trait in rust is more flexible than most other languages, it is still restrictive in some ways.

---

## The Error Trait is restrictive

<list fragments>

- Can only represent errors with a single source
- Can only access 3 forms of context

Notes: Can't return types like SpanTrace without using hacks based on downcast to work around the error trait.

Error return traces

---

<style>
.container{
    display: flex;
}
.col{
    flex: 1;
}
</style>

<div class="container">

<div class="col">
 Recoverable

<list fragments>

- Defining
  - types and traits
- Propagating
  - `?`
- Matching and Reacting
  - `match` or `downcast`
- Discarding
  - `drop` or `unwrap`
- Reporting
  - Reporting types and hook

</div>

<div class="col">
Unrecoverable

<list fragments>

- Defining
  - `panic!`
- Propagating
  - builtin
- Matching and Reacting
  - pls don’t
- Discarding
  - `catch_unwind`
- Reporting
  - panic hook

</div>

</div>

Notes: Okay so now we’ve covered the fundamentals, you know how to handle errors of both types, so let’s get back and start digging into the differences between errors, context, and reports.

---

## Definitions

<list fragments>

- **Error**: A description of why an operation failed
- **Context**: Any information relevant to an error or an error report that is not itself an error
- **Error Report**: Printed representation of an error and all of its associated context

Notes: This is in the context of reporting, we will no longer talk about handling.

This gets to the other goal of this talk, clarifying the relationship and
difference between errors and context. Errors describe what went wrong,
context helps you figure out the why, and it's my opinion that keeping these
two concepts in mind is very important when designing your error reporting.

How about an example? Let's dig into error reporting by recreating the
custom_section example from the beginning of the talk. NEXT SLIDE

---

```rust [1-3|5|7|9-10|12|13|15]
trait CommandExt {
    fn output2(&mut self) -> Result<String, eyre::Report>;
}

impl CommandExt for std::process::Command {
    fn output2(&mut self) -> Result<String, eyre::Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout)
            .into_owned();

        if !output.status.success() {
            Err(eyre!("command exited unsuccessfully"))
        } else {
            Ok(stdout)
        }
    }
}
```

Notes: lets start with a customized version of `Command::output` that reports
better errors and converts stdout to a String on success.


---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">command exited unsuccessfully</font></pre>

Notes: This is an Error.

okay, thats not very helpful, also I didn't even tell you what command I was running

---

```rust [8|11]
impl CommandExt for std::process::Command {
    fn output2(&mut self) -> Result<String, eyre::Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        if !output.status.success() {
            let cmd = format!("{:?}", self);

            Err(eyre!("command exited unsuccessfully"))
                .section(cmd.header("Command:"))
        } else {
            Ok(stdout)
        }
    }
}
```

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">command exited unsuccessfully</font>

Command:
   &quot;git&quot; &quot;cat&quot;</pre>


Notes: Here we can see why the command failed, cat isn't a real git command! It would be cool if it was though.

This is context, and the whole thing is a report.

---

```rust left []
fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;

    let _ = std::process::Command::new("git")
        .arg("cat")
        .output2()?;


    Ok(())
}
```

---

```rust left [7]
fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;

    let _ = std::process::Command::new("git")
        .arg("cat")
        .output2()?
        .wrap_err("cat could not be got")?;

    Ok(())
}
```

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">cat could not be got</font>
   1: <font color="#F15D22">command exited unsuccessfully</font>

Command:
   &quot;git&quot; &quot;cat&quot;

</pre>

---

```rust [9|13-14]
impl CommandExt for std::process::Command {
    fn output2(&mut self) -> Result<String, eyre::Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        if !output.status.success() {
            let cmd = format!("{:?}", self);
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            Err(eyre!("command exited unsuccessfully"))
                .section(cmd.header("Command:"))
                .section(stdout.header("Stdout:"))
                .section(stderr.header("Stderr:"))
        } else {
            Ok(stdout)
        }
    }
}
```

---

<pre class=term><font color="#4E9A06"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">cat could not be got</font>
   1: <font color="#F15D22">command exited unsuccessfully</font>

Command:
   &quot;git&quot; &quot;cat&quot;

Stderr:
   git: &apos;cat&apos; is not a git command. See &apos;git --help&apos;.

   The most similar commands are
   	clean
   	mktag
   	stage
   	stash
   	tag
   	var</pre>

Notes: And finally we have an error report including all the context we need
to pinpoint what went wrong.

Hopefully this makes it clear how benefitial it can be to keep errors and context separate.

---

## Libraries

- Defining
- Propagating
- Matching and Reacting
- Discarding
- Reporting

Notes: Defining => thiserror, displaydoc, SNAFU
Defining ad-hoc errors + Reporting => anyhow, eyre
Report Hooks => color-eyre, color-backtrace, color-anyhow (soon tm)
Propagation => fehler
Context Capture => tracing-error, extracterr

---

## Defining - thiserror

``` rust []
#[derive(thiserror::Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

---

## Defining - displaydoc

```rust []
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum DataStoreError {
    /// data store disconnected
    Disconnect(#[from] io::Error),
    /// the data for key `{0}` is not available
    Redaction(String),
    /// invalid header (expected {expected:?}, found {found:?})
    InvalidHeader {
        expected: String,
        found: String,
    },
    /// unknown data store error
    Unknown,
}
```

---

## Defining - SNAFU


```rust [1-13|10-11]
#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: io::Error, path: PathBuf },
}

fn process_data() -> Result<(), Error> {
    let path = "config.toml";
    let configuration = fs::read_to_string(path)
        // wrap error while capturing `path` as context
        .context(ReadConfiguration { path })?;
    Ok(())
}
```

---

## Defining - anyhow/eyre

```rust [1-2|4-6]
// Construct an ad-hoc error
Err(eyre!("file not found"))?

// Constructing an ad-hoc wrapping error
fallible_fn()
    .wrap_err("failed operation")?;
```

---

## Common Concerns - Defining

- Open Set vs Closed Set
- Stack Size
- Unreportable Errors

---

## Propagating - fehler

```rust
#[fehler::throws(i32)]
fn foo(x: bool) -> i32 {
    if x {
        0
    } else {
        fehler::throw!(1);
    }
}
```

---

## Gathering Context - tracing-error

```rust [2|3|5-7]
let error = std::fs::read_to_string("myfile.txt")
    .in_current_span();
let error: &(dyn std::error::Error + 'static) = &error;

if let Some(spantrace) = error.span_trace() {
    eprintln!("found a spantrace:\n{}", spantrace);
}
```

---

## Gathering Context - extracterr

```rust
type Error = extracter::Bundled<ExampleError, backtrace::Backtrace>;

fn foo() -> Result<(), Error> {
    Err(ExampleError)?
}
```

---

## Matching and Reacting

Notes: mostly handled by builtin features, match, downcast.

---

## Matching and Reacting - anyhow/eyre

```rust
use eyre::WrapErr;

#[derive(Debug, displaydoc::Display)]
/// Foo error
struct FooError;

let report = fallible_fn()
    .wrap_err(FooError)
    .unwrap_err();

let foo_error = report.downcast_ref::<FooError>().unwrap();
```

---

## Discarding

---

## Reporting

- Reporters: anyhow/eyre
- Hooks: color-eyre, jane-eyre, color-anyhow (soon), color-backtrace

---

## Common Concerns - Reporters

- Reporters usually impl `From<E: Error>`
- if they do they _cannot_ impl `Error`
- Prints report via `Debug` trait

---

## Library vs Application

- Library => error defining libraries
- Application => adhoc error defining + error reporting libraries

---

# Fin