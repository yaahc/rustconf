# Error Handling Isn't All About Errors

<span class=author>Jane Lusby</span>

<fab fa-twitter> [@yaahc_] / [yaah.dev]

Notes: Hello and welcome to my talk, error handling isn't all about errors.

[@yaahc_]: https://twitter.com/yaahc_
[yaah.dev]: https://yaah.dev/

---

## About Me

Notes: I am the author of eyre, a fork of anyhow with support for customized
error reports and color-eyre which customizes the behaviour of eyre and lets
you construct error reports like this.

Show the various usage examples from `color-eyre`.

I'm giving this talk to share what I learned in the process of developing
eyre and how it has changed how I look at error handling.

But before we get into the details of reporting lets first cover error handling at large.

---

## What Is Error Handling?

<ul fragments>

- <del>Annoying</del>
- Defining errors
- Propagating errors and gathering context
- Reacting to specific errors
- Discarding errors
- Reporting errors and gathered context

</ul>


Notes: A lot of things, when you zoom in close

The breakdown here gets to the goal of my talk. I have a theory that
error handling is made more confusing by people try to simplify it, because,
among other things, error handling is annoying. I worry that the fuzziness
between these different responsibilities makes it hard for people to infer
what tools they should be using when "handling errors".

*Next slide: For the rest of this talk I’m going to treat error reporting and error handling as mutually exclusive activities...*

---

# You either handle an error or you report it.

Notes: So what is "handling" now?

---

## What Is Error Handling?

- <del>Annoying</del>
- Defining errors
- Propagating errors and gathering context
- Reacting to specific errors
- Discarding errors
- Reporting errors and gathered context

*Next Slide: Okay now talk about fundamentals..*

---

## Recoverable vs Non-Recoverable

<ul fragments>

- panic => unrecoverable
- Result => recoverable
- Most languages dont distinguish between these kinds of errors

Notes: The Rust model for errors distinguishes between two classes of errors.
Recoverable errors are errors you can reasonably expect to occur during execution of..., can be handled, or reported.
Unrecoverable errors are bugs, can’t be handled, only reported before exiting the program / thread.
C++ has exceptions
Rust doesnt
Rust has panic for unrecoverable errors and result recoverable errors

NEXT SLIDE


</ul>


---

## Panic

```rust
// if the index is past the end of the slice
} else if self.end > slice.len() {
    panic!(
        "index {} out of range for slice of length {}",
  self.end,
  slice.len()
    );
}
```

<ul fragments>

- Only input is an error message
- Reporting and Context gathering done by panic hook
- Cleans up either by unwinding or aborting

Notes: Unrecoverable errors in rust are created via the `panic!` macro. Here we can see an example of an index out of bounds error. NEXT SLIDE

BIGGER CODE FONT

Mention it can be typed but its out of the scope of this talk

---

## Result

```rust
enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
```

Notes: Rust models recoverable errors with `Result<T, E>`. There’s very little syntax sugar involved for recoverable errors in rust, they’re just values and they’re implemented as a library feature.

You use result for functions where you want to return either a value or an error

NEXT SLIDE

---

## Result

```rust
fn get_config() -> Config {
    // ...
}
```

Notes: When you want to return an error you change the return type


---

## Result

```rust
fn get_config(path: &Path) -> Config {
    // ...
}
```

---

## Result

```rust
fn get_config(path: &Path) -> Result<Config, Error> {
    /// ...
}
```

- We must handle errors
- Any type can be an error

Notes: When you want to return an error you change the return type

By leveraging enums we gain a couple of advantages

---

## Try and `?`

```rust
let config = get_config()?;

// vs

let config = match get_config() {
    Ok(success_value) => success_value,
    Err(error_value) => return Err(E::from(error_value)),
};
```

Notes: The try trait models fallible operations
Result is a “Try Type”
Abstraction over error propagation
Separates fallibility and failures

---

## Try and `?`

```rust
fn configure() -> Result<(), ConfigureError> {
    let config = get_config()?;

    // ...

    Ok(())
}
```

---

## Recoverable

- Defining Errors
  - types
- Propagating Errors and Gathering Context
  - `?`, more types and functions
- Matching and Reacting to specific errors
  - `match` and other control flow primitives
- Discarding Errors
  - explicit drop e.g. `let _ =`
- Reporting Errors and Gathered Context
  - error reporting type or function that operate on the Error Trait


----

## Unrecoverable


- Defining Errors
  - panic!
- Propagating Errors and Gathering Context
  - panic hook, intrinsics
- Matching and Reacting to specific errors
  - pls don’t
- Discarding Errors
  - catch_unwind, thread/task boundaries
- Reporting Errors and Gathered Context
  - panic hook


Display incrementally

---

## The Error Trait

- Handling for an open set of errors
- Reporting for all errors

Notes: Okay so now we’ve covered the fundamentals, you know how to handle errors of both types, so let’s get back and start digging into the differences between errors, context, and reports.


---

# The error trait is an interface _for_ reporters.

---

## Definitions

- **Error**: A description of why an operation failed
- **Context**: Any information relevant to an error or an error report that is not itself an error
- **Error Report**: Printed representation of an error and all of its associated context

Notes: This is in the context of reporting, we will no longer talk about handling.

What isn’t an error? NEXT SLIDE

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

```rust [8|10]
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


Notes: What can I say, I like cats.

This is context, and the whole thing is a report.

---

```rust
use color_eyre::{eyre, Help, SectionExt};
use eyre::eyre;

fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;

    let _ = std::process::Command::new("git")
        .arg("cat")
        .output2()?;

    Ok(())
}
```

---

```rust
use color_eyre::{eyre, Help, SectionExt};
use eyre::{eyre, WrapErr};

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
<font color="#4E9A06"><b>   Compiling</b></font> playground v0.1.0 (/home/jlusby/playground)
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 1.34s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/playground`
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


---

## Reporters

- Reporters usually impl `From<E: Error>` and _don't_ impl `Error`
- Prints report via `Debug` trait

---

## Libraries

- Defining => thiserror, SNAFU
- Reporters => anyhow, eyre
- Reporting => color-eyre, color-anyhow (soon tm)
- Propagation => fehler
- Context Capture => tracing-error, extracterr

---

## Common Concerns When Defining Errors

- Open Set vs Closed Set
- Stack Size
- Unreportable Errors