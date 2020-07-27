# Error Handling Isn't All About Errors

<span class=author>Jane Lusby</span>

<fab fa-twitter> [@yaahc_] / [yaah.dev]

Notes: Hello and welcome to my talk, error handling isn't all about errors.

<slide-footer>
<left>Jane Lusby</left>
<right>
<fab fa-twitter> <a href="https://twitter.com/yaahc_">@yaahc_</a> / <a href="https://yaah.dev/">yaah.dev</a>
</right>
</slide-footer>

[@yaahc_]: https://twitter.com/yaahc_
[yaah.dev]: https://yaah.dev/

---

<slide class=center>

## About Me

Notes: I am the author of eyre, a fork of anyhow with support for customized
error reports, and color-eyre, which provides a custom panic hook and a
custom eyre report hook and lets you construct error reports like this.

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

Notes: Next slide: we can also filter our backtrace frames, note that here there are 10 frames hidden after main...

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

Notes: We will dig into this example more later...

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


Notes: I'm giving this talk to share what I learned in the process of
developing eyre and how it has changed how I look at error handling and error
reporting.

Next slide: But before we get into the details of reporting lets first cover
error handling at large.

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
what tools they should be using when "handling errors".

Next slide: Okay now talk about fundamentals..

---

<slide class=title-card data-state=purple>

# Recoverable<br> vs<br> Non-Recoverable

Notes: The Rust model for errors distinguishes between two classes of errors.

Recoverable errors are errors you can reasonably expect to occur during execution of..., can be handled, or reported.

Unrecoverable errors are bugs, can’t be handled, only reported before exiting the program / thread

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

Notes: Unrecoverable errors in rust are created via the `panic!` macro. Here we can see an example of an index out of bounds error. NEXT SLIDE

---

## Panic

```rust [4]
// if the index is past the end of the slice
} else if self.end > slice.len() {
    panic!(
        "index {} out of range for slice of length {}",
        self.end,
        slice.len()
    );
}
```

<list fragments>

- Only input is an error message and optional some context
- Reporting and default context gathering done by panic hook
- Cleans up either by unwinding or aborting

Notes: Mention it can be typed but its out of the scope of this talk

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

When you want to return an error you change the return type

The big advantage of using enums is we must handle all errors.

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

Notes: The try trait models fallible operations
Result is a “Try Type”
Abstraction over error propagation

---

## The Error Trait

<list fragments>

- Handling for an open set of errors
- Reporting for all errors
- limited amount of context is accessible

---

<slide class=title-card data-state=purple>

# The error trait provides an interface _for_ reporters.

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
  - `match` and `downcast`
- Discarding
  - explicit drop
- Reporting
  - Reporting types

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

This gets to the goal of this talk, the relationship and difference between
errors and context. Errors describe what went wrong, context helps you figure
out the why, and it's my opinion that keeping these two concepts in mind when
designing your error handling is an important step in designing clean error
reporting.

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

---

## Libraries

<list fragments>

- Defining => thiserror, displaydoc, SNAFU
- Defining ad-hoc errors + Reporting => anyhow, eyre
- Report Hooks => color-eyre, color-backtrace, color-anyhow (soon tm)
- Propagation => fehler
- Context Capture => tracing-error, extracterr

---

## Common Concerns

- Open Set vs Closed Set
- Stack Size
- Unreportable Errors

---

## Reporters

- Reporters usually impl `From<E: Error>` and _don't_ impl `Error`
- Prints report via `Debug` trait

---

# Fin