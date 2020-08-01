# Error Handling Isn't All About Errors

<span class=author>Jane Lusby</span>

<fab fa-twitter> [@yaahc_] / [yaah.dev]

Slide Template by Rebecca Turner <fab fa-twitter> [@16kbps] / [becca.ooo]

Notes: Hello and welcome to my talk, error handling isn't all about errors.

Next slide: Let me start by introducing myself...

<slide-footer>
<left>Jane Lusby (she/her)</left>
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

Notes: My name is Jane Lusby, my pronouns are she/her. On the internet I go
by Yaah or Yaahc. I've been writing rust for two and a half years though I
was only recently hired to do so professionally, by The Zcash Foundation.

Next slide: Quick shameless plug, I also maintain `awesome-rust-mentors`...

---

<slide no-footer>

## Awesome Rust Mentors

![](img/mentors.png)

Notes: Awesome-rust-mentors is a list of projects and people who are willing
to provide mentorship to anyone who asks. If you're interested in finding a
mentor, finding a project to get involved in, being a mentor, or getting
people involved in your project you should check it out.

---

## Why Error Handling?

Notes: I actually got into error handling on accident, it started as a yak
shave when I wanted to open source a library I wrote for work but I wasn't
happy with the error handling so I decided to fix it up first.

Next slide: That yak shave ended with me writing eyre...

---

## eyre



Notes: Eyre is a fork of anyhow with support for customized error reports via a
global hook, similar to a panic hook.

Next slide: I also ended up writing color-eyre.

---

## color-eyre

Notes: color-eyre is a library which provides a custom panic report hook and
a custom error report hook for eyre.

Next slide: With these libraries I'm now able to construct error reports like
this.

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run --example usage
<font color="#333333">Error:
   0: Unable to read config
   1: No such file or directory (os error 2)

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: usage::read_file with path=&quot;fake_file&quot;
      at examples/usage.rs:32
   1: usage::read_config
      at examples/usage.rs:38

Suggestion: try using a file that exists next time

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: Here we can see the basic usage example.

Next slide: In it we have an error section.

---

<pre class=term><font color="#333333"><b>❯</b> cargo run --example usage</font>
Error:
   0: <font color="#F15D22">Unable to read config</font>
   1: <font color="#F15D22">No such file or directory (os error 2)</font>
<font color="#333333">
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: usage::read_file with path=&quot;fake_file&quot;
      at examples/usage.rs:32
   1: usage::read_config
      at examples/usage.rs:38

Suggestion: try using a file that exists next time

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: Next slide: followed by a span trace section...


---

<pre class=term><font color="#333333"><b>❯</b> cargo run --example usage
Error:
   0: Unable to read config
   1: No such file or directory (os error 2)
</font>
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">32</font>
   1: <font color="#F15D22">usage::read_config</font>
      at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">38</font>
<font color="#333333">
Suggestion: try using a file that exists next time

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: If you're not familiar with tracing is this extremely cool
backtrace-like type of tracing spans.

Next slide: then after that we have a suggestion section


---

<pre class=term><font color="#333333"><b>❯</b> cargo run --example usage
Error:
   0: Unable to read config
   1: No such file or directory (os error 2)

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: usage::read_file with path=&quot;fake_file&quot;
      at examples/usage.rs:32
   1: usage::read_config
      at examples/usage.rs:38
</font>
<font color="#34E2E2">Suggestion</font>: try using a file that exists next time
<font color="#333333">
Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: Next slide: Followed finally by a omitted backtrace section

---

<pre class=term><font color="#333333"><b>❯</b> cargo run --example usage
Error:
   0: Unable to read config
   1: No such file or directory (os error 2)

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: usage::read_file with path=&quot;fake_file&quot;
      at examples/usage.rs:32
   1: usage::read_config
      at examples/usage.rs:38

Suggestion: try using a file that exists next time
</font>
Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.</pre>

Notes: Next slide: I can also enable backtrace capture...

---

<pre class=term><b>❯</b> RUST_BACKTRACE=1 <font color="#333333">cargo run --example usage
// ...
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
                         ⋮ 5 frames hidden ⋮
   6: usage::read_file::h10b2389c2b814452
      at /home/jlusby/git/yaahc/color-eyre/examples/usage.rs:35
   7: usage::read_config::hf7150b146edb25d9
      at /home/jlusby/git/yaahc/color-eyre/examples/usage.rs:40
   8: usage::main::hc3df11a6ea0d044d
      at /home/jlusby/git/yaahc/color-eyre/examples/usage.rs:11
                        ⋮ 10 frames hidden ⋮
// ...
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: Next slide: Here we have a backtrace section...

---

<pre class=term><b>❯</b> <font color="#333333">RUST_BACKTRACE=1 cargo run --example usage
// ...</font>
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  <font color="#34E2E2">                       ⋮ 5 frames hidden ⋮                       </font>
   6: <font color="#F15D22">usage::read_file</font><font color="#88807C">::h10b2389c2b814452</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">35</font>
   7: <font color="#F15D22">usage::read_config</font><font color="#88807C">::hf7150b146edb25d9</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">40</font>
   8: <font color="#F15D22">usage::main</font><font color="#88807C">::hc3df11a6ea0d044d</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">11</font>
  <font color="#34E2E2">                      ⋮ 10 frames hidden ⋮                       </font>
<font color="#333333">// ...
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: We can control the formatting of the backtrace section, here you can
see a pretty printed backtrace based on `color-backtrace`, which hides
irrelevant frames, in this case it's hidden 5 frames before read_file and 10
frames after main.

Next slide: we can also further customize this by applying custom filters...

---

<pre class=term><font color="#333333"><b>❯</b> RUST_BACKTRACE=1 cargo run --example custom_filter
// ...</font>
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  <font color="#34E2E2">                       ⋮ 5 frames hidden ⋮                       </font>
   6: <font color="#F15D22">custom_filter::read_file</font><font color="#88807C">::h0afee8fe0960bf02</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/custom_filter.rs</font>:<font color="#75507B">53</font>
   7: <font color="#F15D22">custom_filter::read_config</font><font color="#88807C">::h6622065848c69b29</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/custom_filter.rs</font>:<font color="#75507B">58</font>
  <font color="#34E2E2">                      ⋮ 11 frames hidden ⋮                       </font>
<font color="#333333">// ...
Run with RUST_BACKTRACE=full to include source snippets.</font></pre>

Notes: Here you can see we've got 11 frames hidden after read_config instead
of 10 frames hidden after main because I've configured it to hide the main
frame.

Next slide: And we can apply this custom filtering consistently to all of our
errors.

---

<pre class=term><font color="#CC0000"><b>❯</b></font> RUST_BACKTRACE=1 cargo run --example panic_hook --no-default-features
<font color="#CC0000">The application panicked (crashed).</font>
Message:  <font color="#06989A">No such file or directory (os error 2)</font>
Location: <font color="#75507B">examples/panic_hook.rs</font>:<font color="#75507B">37</font>

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━
  <font color="#34E2E2">                      ⋮ 13 frames hidden ⋮                       </font>
  14: <font color="#F15D22">panic_hook::read_file</font><font color="#88807C">::h1a2c1d2710c16ca9</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/panic_hook.rs</font>:<font color="#75507B">37</font>
  15: <font color="#F15D22">panic_hook::read_config</font><font color="#88807C">::h2751dcca3305a9a3</font>
      at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/panic_hook.rs</font>:<font color="#75507B">43</font>
  <font color="#34E2E2">                      ⋮ 11 frames hidden ⋮                       </font>

Run with COLORBT_SHOW_HIDDEN=1 environment variable to disable frame filtering.
Run with RUST_BACKTRACE=full to include source snippets.</pre>

Notes: So here you can see we can have the same report format for panics as
we did for our errors.

Next slide: with our error report hook we can also bundle arbitrary data with
our errors.

---

<slide no-footer>

<pre class=term><font color="#333333"><b>❯</b> cargo run --example custom_section
Error:
   0: the cat could not be got
   1: cmd exited unsuccessfully
</font>
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
   	var

<font color="#333333">Suggestion: Maybe that isn&apos;t what git is for...</font></pre>

Notes: We can use this to make custom sections in our error reports. Here you
can see that in addition to the error section we also have a Command section
showing which command we tried to run and a stderr section, showing the
output of the command when it failed. And we'll will dig into this example
more later, so look forward to that...

Next slide: and finally, we can also add errors as sections

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

Notes: Which we can use to aggregate multiple errors into a single report and
format them all consistently.


Now, I'm not giving this talk to talk about eyre and color-eyre.

Next slide: I'm giving this talk to share what I learned in that yak shave to
fix the error handling in my library that resulted in eyre and color-eyre,
and how the process has changed how I look at error handling and error
reporting.


---

## What Is Error Handling?

<list fragments>

- <del>Annoying</del>
- Defining errors
- Propagating errors and gathering context
- Reacting to specific errors
- Discarding errors
- Reporting errors and gathered context


Notes: Show the `annoying` bullet at the end of saying "What is error handling?".

don't mention the annoying, just keep going

It's lot of things, when you zoom in close. Error handling is defining
errors, with types and traits. It's propagating errors and gathering context,
and by context I mean stuff like the path you tried to open or a backtrace
showing where your error came from. It's reacting to specific errors, if the
file isn't found, create the file. It's discarding errors, and doing so
intentionally and visibly. And last but not least, it's reporting errors and
their associated context.

Now, this breakdown gets to the goal of my talk. I have a theory that error
handling is made more confusing by people try to simplify it, because, among
other things, error handling is annoying. I worry that the fuzziness between
these different responsibilities makes it hard for people to infer what tools
they should be using when "handling errors". My hope is that by breaking
error handling into it's component parts we can make it easier to understand
and explain.

Next slide: So let's start with the fundamentals. Note, this first bit is
taken almost word for word from The Rust Book's chapter on error handling.

---

<slide class=title-card data-state=purple>

# Recoverable<br> vs<br> Non-Recoverable

Notes: The Rust model for errors distinguishes between two classes of errors,
recoverable and non-recoverable errors.

Recoverable errors are errors that can reasonablly be reacted to, or reported
when encountered. These are errors like file not found, or connection closed.

Non-recoverable errors are errors that cannot reasonably be reacted to, only
reported, before exiting the thread or program. These are errors are usually
caused by bugs such as index out of bounds or integer overflow.

Now, most languages dont distinguish between these kinds of errors.

For example, C++ has historically used exceptions for both.

Rust doesnt have exceptions, instead Rust has panic for non-recoverable
errors and result recoverable errors

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

Notes: non-recoverable errors in rust are created via the `panic!` macro. Here
we can see an example of an index out of bounds error.

Next slide: Only input for the panic macro is an error message and optionally
some context to include in that error message.

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

Notes: Reporting and default context gathering is done by panic hook, and by
default context gathering I mean capturing the caller location or capturing
the backtrace if it's enabled.

Once the panic hook is done printing the report the panic handler takes over
and cleans up either by unwinding the thread's stack or aborting the
application all together.

Next slide: Result

---

## Result

```rust [1-7|3-4|5-6|1-7]
#[must_use]
enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
```

Notes: Recoverable errors in rust are modeled with the enum `Result<T, E>`.

This enum has two variants, Ok, which contains the value of an
operation when it completes successfully, and Err, which contains the error
value of an operation when it could _not_ be completed successfully.

We use Result to combine two return types in one and assign meaning to each
possibility.

Next slide: The big advantage of using enums for recoverable errors is that
we must react to all errors.

---

## Result

```rust [1-4]
match result {
    Ok(success) => println!("we got the value {}!", success),
    Err(error) => println!("uh oh we got an error: {}", error),
}
```

Notes: Here you can see we have to use match (or anything equivalent to
match) to access the values inside of either variant.

With an enum, we cannot access the inner values without first accounting for
all the variants that enum could possibly be. In addition to this, Rust has
marked the Result enum as `#[must_use]`, which makes the compiler emit a
warning whenever a result is discarded implicitly. This helps us avoid
ignoring errors accidentally and makes discarded errors visible to later
readers.

Next slide: For recoverable errors rust also provides the currently unstable
Try trait and the already stable try operator...

---

## Try and `?`

Notes: The try trait is used to model fallible return types in rust. Indeed,
Result is type that implements the Try trait, as does Option, and other some
combinations thereof.

Next slide: With the try trait rust is able to abstract the "propogation of
errors" with the try operator.

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

Notes: Here we see two equivalent code snippets. The first manually
propagates the error using match and return. The second does the same by
simply using the try operator to propagate the error.

Next slide: Finally, for recoverable errors rust also provides the error trait.

---

## The Error Trait

<list fragments>

- Representing an open set of errors
- Reacting to specific errors in an open set
- Reporting Interface for all errors

Notes: The error trait fills three roles in rust.

First, it lets us represent an open set of errors by converting any type that
implements the error trait into an error trait object. This is useful for
composing errors, and it is what lets us expose source errors via the error
trait regardless of their actual type.

Second, the error trait lets us then react to specific errors by letting us
try to downcast them back to their original type safely, rather than using
match as we would with enums.

Finally, it provides an interface for reporters.

Next slide: The concept of error reporters isn't a concept that is common in
the rust ecosystem today or any language's error handling ecosystem as far as
I know. However it is vocabulary that I find particularly compelling in the
context of rust error handling, and this is largely because of how rust has
defined it's error trait.

---

## The Error Trait

```rust [1-9|1|2-4|6-9]
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }
}
```

Notes: Here is a simplified version of the error trait. Here you can see
we've got two super traits, debug and display, which we must impl to
implement the error trait. Then below that we have two functions, source and
backtrace, both with default impls that we can override when needed.

Next slide: Next lets look at a simple error

---

## The Error Trait - An Error

```rust [1-2|4|9|13]
#[derive(Debug)]
struct DeserializeError;

impl std::fmt::Display for DeserializeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "unable to deserialize type")
    }
}

impl std::error::Error for DeserializeError {}
```

Notes: Here we've got an error with no members. We derive debug on it, then
implement display, where we just write our error message and finally we impl
the error trait.

We don't have a source or a backtrace, so we don't need to override any
functions here, which is why the block on the trait implementation is empty.
If we did have a source though we would need to override the `source`
function to explicitly return a reference to our source when that function is
called by an error reporter.

Next slide: finally lets look at a simple error reporter.

---

## The Error Trait - A Reporter

```rust []
fn report(error: &(dyn std::error::Error + 'static)) {
    print!("Error:");

    let errors =
        std::iter::successors(Some(error), |e| e.source());

    for (ind, error) in errors.enumerate() {
        print!("\n   {}: {}", ind, error);
    }

    if let Some(backtrace) = error.backtrace() {
        print!("\n\nBacktrace: {}", backtrace);
    }
}
```

Notes: Here we've implemented our reporter as a short free function. This
function takes an error and prints that error and all of its sources,
followed by a backtrace if our error captured one. A more complex error
reporter might also try to check all errors for a backtrace, or if it were a
type and was storing its own context in addition to the error it might print
that as well.

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

Notes: The error trait equivalent in other languages is often quite simple,
just a single fn to grab the error message.

These interfaces force you to either include your source error, your
backtrace, and any other information you care about in your error message or
to avoid using the provided interface all together.

In rust we don't have to combine our messages all into one, in fact, you're
encouraged not to. Including a source error's message in your `Display`
implementation and returning it as your source via the Error trait is
essentially a bug, as it forces reporters to duplicate information when they
print out the chain of error messages.

Next slide: By separating the source and the error message we move the
responsibility of formatting away from the errors themselves, making it
possible to get fancy.

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

This wouldn't be possible if the error trait didn't separate the error
message from other context such as the source error.

Next slide: However, despite the fact that the error trait in rust is more
flexible than most other languages, it is still restrictive in some ways.

---

## The Error Trait is restrictive

<list fragments>

- Can only represent errors with a single source
- Can only access 3 forms of context

Notes: If you've ever written a parser you might have run into this, where
you have multiple syntax errors at once, but you can only return one via
`source`. The error trait can only represent a chain of errors as a singly
linked list, not as a tree, and so it doesn't work for every situation.

Can't return types like SpanTrace without using hacks based on downcast to
work around the error trait.

This prevents us from having things like Error return traces in zig, though I
do have plans on how we can fix these problems so in the future this may no
longer be an issue.

---

## TIPS - Reporters

- Reporters usually impl `From<E: Error>`
- if they do they _cannot_ impl `Error`
- don't compose well
- They often carry extra context
- Prints report via `Debug` trait

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
Non-Recoverable

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

Notes: Okay so now we’ve covered the fundamentals, you know all the tools the
language and standard library give you to work with different kinds of
errors.

Next slide: So next let’s dig into my favorite part of error handling, error
reporting, and how it fits into the bigger picture, starting with some
definitions...

---

## Definitions

<list fragments>

- **Error**: A description of why an operation failed
- **Context**: Any information relevant to an error or an error report that
  is not itself an error
- **Error Report**: Printed representation of an error and all of its
  associated context

Notes: In the context of error reporting an error is ..., context is ..., and
an error report is the ...

This gets to the other goal of this talk, clarifying the relationship between
errors and context. Errors describe what went wrong, context helps you figure
out why, and it's my opinion that keeping these two concepts separate leads
to more readable error reports and that adding just a little context can take
your error reports from somewhat servicable to oddly satisfying.

Next slide: I think the best way to explain what I mean will be with an
example, so let's dig into error reporting real quick by recreating the
custom_section example from the beginning of the talk.

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

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(eyre!("command exited unsuccessfully"))
        }
    }
}
```

Notes: lets start with a customized version of `Command::output` that reports
better errors and converts stdout to a String on success.

Next slide: lets run it...

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">command exited unsuccessfully</font></pre>

Notes: We got our error report with our error message, cool! But also not
very helpful, I didn't even tell you what command I was running. Lets figure
that out next.

---

```rust [10|13]
impl CommandExt for std::process::Command {
    fn output2(&mut self) -> Result<String, eyre::Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

        if output.status.success() {
            Ok(stdout)
        } else {
            let cmd = format!("{:?}", self);

            Err(eyre!("command exited unsuccessfully"))
                .section(cmd.header("Command:"))
        }
    }
}
```

Notes: Header takes a type that implements display and prefixes the header
before printing it, and section takes a type that implements display and
prints it after the chain of errors.

Next slide: so lets see how this changes things...

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">command exited unsuccessfully</font>

Command:
   &quot;git&quot; &quot;cat&quot;</pre>


Notes: Here we can see why the command failed, cat isn't a real git command!
It would be cool if it was though.

This is context, and the whole thing is a report.

Next slide: Now, this error isn't very descriptive. Sure, it describes what
went wrong, but it is far too generic. Let's go ahead and define a new error
with a more descriptive error message to wrap our source error.

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
        .output2()
        .wrap_err("the cat could not be got")?;

    Ok(())
}
```

Notes: This function takes a Result and an arg that implements Display, and
if the Result is the `Err` variant it creates a new error, using the arg as
the error message and the old error as the source. It then returns this new
error as an eyre::Report.

---

<pre class=term><font color="#CC0000"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">the cat could not be got</font>
   1: <font color="#F15D22">command exited unsuccessfully</font>

Command:
   &quot;git&quot; &quot;cat&quot;

</pre>

---

```rust [7-9|13-14]
        let stdout = String...

        if output.status.success() {
            Ok(stdout)
        } else {
            let cmd = format!("{:?}", self);
            let stderr =
                String::from_utf8_lossy(&output.stderr)
                    .into_owned();

            Err(eyre!("command exited unsuccessfully"))
                .section(cmd.header("Command:"))
                .section(stdout.header("Stdout:"))
                .section(stderr.header("Stderr:"))
        }
    }
}
```

---

<pre class=term><font color="#4E9A06"><b>❯</b></font> cargo run
Error:
   0: <font color="#F15D22">the cat could not be got</font>
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

Notes: And finally we have an error report including all the information we
need. With it we can pinpoint what went wrong, why it went wrong, and, as an
added bonus, how we can fix it.

Hopefully this makes it clear how benefitial just a little context can be for
error reports.

Next slide: By now you should know all the tools built into the language,
how they fit into the various pieces of error handling, and have an
understanding of how they can be combined to write error reports. So lets
look at the ecosystem at large to see what open source libraries we can use
to help us with our five parts of error handling.

---

## Libraries

- Defining
- Propagating and Gathering Context
- Matching and Reacting
- Discarding
- Reporting

Notes: I'm going to introduce these libraries by how they fit into our error
handling breakdown, not every part will have libraries to help and some will
be disproportionately represented.

Next slide: First I'd like to introduce thiserror.

---

## Defining - thiserror

``` rust []
#[derive(Debug)]
pub enum DataStoreError {

    Disconnect(io::Error),

    Redaction(String),

    InvalidHeader {
        expected: String,
        found: String,
    },

    Unknown,
}
```

Notes: This error is an error derive macro, and it exists to reduce boiler
plate by implementing commonly used traits for you, such as Error, Display,
and From.

---

## Defining - thiserror

``` rust []
#[derive(Debug, thiserror::Error)]
pub enum DataStoreError {

    Disconnect(io::Error),

    Redaction(String),

    InvalidHeader {
        expected: String,
        found: String,
    },

    Unknown,
}
```

Notes: To use it, we start by adding the macro's identifier to our derive
attribute.

---

## Defining - thiserror

``` rust [1-14|3|5|7|4]
#[derive(Debug, thiserror::Error)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(io::Error),
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

## Defining - thiserror

``` rust [4]
#[derive(Debug, thiserror::Error)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[source] io::Error),
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

## Defining - thiserror

``` rust [4|1-14]
#[derive(Debug, thiserror::Error)]
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

Notes: Next slide: Next I'd like to introduce `displaydoc`

---

## Defining - displaydoc

```rust []
#[derive(Debug, thiserror::Error, displaydoc::Display)]
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

Notes: Display doc is a fork of thiserror that provides only the display
derive portion of this error, but uses doc comments instead of custom
attributes to input the format strings.

---

## Defining Errors & Gathering Context - SNAFU


```rust [1-13|1|2-4|10-11]
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

Notes: The context function takes a result and a "Context Selector" struct
which is autogenerated by the derive macro. This struct implicitly passes
along context like the source and backtrace, making it so you only have to
capture additional context that is unique that your error variant. It then
internally creates the correct wrapping error variant. You can think of it as
syntax sugar for `map_err`.

---

## Defining - anyhow/eyre

```rust [1-2|4-6]
// Construct an ad-hoc error
Err(eyre!("file not found"))?

// Constructing an ad-hoc wrapping error
fallible_fn()
    .wrap_err("failed operation")?;
```

Notes: anyhow and eyre also have helpers for defining new errors. However,
these functions don't help you define new error types, instead they use
private types to create the new errors and then they immediately wrap those
types in the main reporting type e.g. `eyre::Report`. This is mostly useful
for when you want to create errors exclusively to report them, though these
crates do also provide some helpers for then later reacting to these adhoc
error types.

---

## TIPS - Defining

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

## Library vs Application

- Library => error defining libraries
- Application => adhoc error defining + error reporting libraries

Notes: What parts of error handling do you need as a library developer? You
probably don't know, because you're not the one handling errors as a library
developer. Libraries usually create and return errors for applications to
react to or report. This means we need errors that are maximally flexible, so
we need something users can match and react too, that means either an enum or
a dyn Error with downcast. We also want an error that can be wrapped by other
errors easily, which means we want to impl the Error trait on our error type,
so that rules out `Box<dyn Error>`, finally we want to be able to report it,
so we should impl the Error trait. All of this means we need to define
errors, which we can do with types and traits, either by hand or with helper
libraries like `thiserror` or `SNAFU`.

I tend to go with the implemented by hand option in libraries that I expect
other ppl to use, because proc macros can add a lot of time to your build,
but in my applications I tend to use thiserror for convenience.

So what does that mean for applications? We need to be able to handle lots of
error types, so we probably want open set error handling, that means box<dyn
Error> or a reporter like `eyre` or `anyhow`. We also need to be able to
create errors that we handle, if they're handled immediately we probably
don't need to introduce much at all, we could just impl an enum and handle it
and not bother with the error trait or anything. If we need to create an
error that will just be reported we don't need types or handling, so we can
use ad-hoc error construction, which makes reporters a better option than a
simple Box. And we probably need more tools for debugging our errors, so we
want a featureful error reporter that captures enough context to help us
debug our applications, so we might want a custom report hook like
`color-eyre` and maybe some tracing integration with `tracing-error`.

---

# Fin