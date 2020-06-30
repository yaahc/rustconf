# Rust<br> <div class=small>for</div> Non-Systems Programmers
<span class=author>Rebecca Turner</span>

Notes: Hey folks, my name is Rebecca Turner and I'm going to tell you why you
should be writing non-systems code in Rust.

---

## Systems programming

- Resource-constrained
- Concurrent
- High-performance
- “Low-level”

Notes: By systems programming, I'm talking about programming in
resource-constrained environments (like embedded systems), concurrent or
distributed software with many threads or workers, software that needs to run
extremely quickly, or other so-called “low-level” programs which need to
directly access hardware.

Allegedly, Rust is great for all these use-cases. I say *allegedly*
because I'm not a systems programmer — most of my programs read some files,
maybe make some network calls, do some light parsing, and not much else. Most
of the time, they don't even need to be particularly fast.

---

## What makes a good non-systems language?

- Expressive: write what you mean
- Safe: no use-after-free bugs, no bounds-checking errors...
- ...TODO

---

## Why do I like Rust?

Notes: As much as this is a talk

---

## Tooling

- Documentation: [rustdoc] (API docs) and [mdBook] (long-form guides)
- Language servers: [rls] (official) and [rust-analyzer] (community)
- Package manager, build system: [Cargo]

[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[mdBook]: https://rust-lang.github.io/mdBook/
[rls]: https://github.com/rust-lang/rls
[rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer
[Cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

---

## Documentation

<iframe class=main loading=lazy importance=low src="https://docs.rs/rand/0.7.3/rand/"></iframe>

Notes: Here's the generated documentation for the [`rand`] crate, which you can
find at [docs.rs/rand][`rand`].

[`rand`]: https://docs.rs/rand/

---

## Mutability

```rust
fn main() {
  let var = 2;
  var = 4;
}
```

[playground-1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cbd74684121a3803c2d8451d886d3b78

Notes: One of my favorite Rust features is mutability tracking. Here's the simple
form, which you might've seen in JavaScript as `let`/`const` or Java as
`final`.

---

## Mutability

```rust-compiler
error[E0384]: cannot assign twice to immutable variable `var`
--> src/main.rs:3:5
  |
2 |     let var = 2;
  |         ---
  |         |
  |         first assignment to `var`
  |         help: make this binding mutable: `mut var`
3 |     var = 4;
  |     ^^^^^^^ cannot assign twice to immutable variable
```

Notes: This doesn't compile because we didn't declare `var` as mutable.

---

## Mutability

```rust
fn main() {
  let nums = vec![1, 2, 3];
  nums.push(4);
}
```

[playground-2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b5153b87b7ff53d3dbbe6cb15a761ded

Notes: But Rust actually goes a step further: non-mutable variables aren't
allowed to change *internally,* either.

---

## Mutability

```language-rust-compiler
error[E0596]: cannot borrow `nums` as mutable,
as it is not declared as mutable
--> src/main.rs:3:5
  |
2 |     let nums = vec![1, 2, 3];
  |         ---- help: consider changing this to be mutable:
  |                    `mut nums`
3 |     nums.push(4);
  |     ^^^^ cannot borrow as mutable
```

Notes: This code doesn't compile because adding elements to a [`Vec`] means
changing memory, and that requires having mutable access to the variable.

This gives us a really powerful way to make sure we aren't changing variables
we shouldn't; instead of having to manually verify every line of code and every
function we call, we just make sure a variable isn't declared as mutable, and
Rust makes sure we don't change it.

[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html

---

## Type system
### The Type System is Big and Strong and Your Friend

- Never see `AttributeError: 'NoneType' object has no attribute
  'append'` ever again
- Autocomplete works
- Encode meaning in the types

Notes: Rust's type system is pretty powerful, and it is your friend.

---

## Enums
### AKA: sum / union types

- Types that can be a number of different things, one at a time

---

## You already use enums

- `null` / `nil` / `undefined` / `None` are used for variables that are either
  an object or nothing
- Strings are used to represent a fixed set of constants
- Exceptions are used to represent different kinds of errors

---

## But too much of the time, you *don't* use enums

- This field isn't null *if* this other field is `true`
- Calling this method after this other method is an error
- The response object will either have a `response` key or an `error` key

---


## Goodbye null pointer errors
Hello, `Option`!

```rust
let maybe_name = Some("Rebecca");
let no_name: Option&lt;String> = None;
```

Notes: An `Option` has two states: it can either be `Some` value, or `None`.

You'll hear Rust programmers talk about "zero-cost abstractions" a lot, and
`Option` is a great example --- it gives us a really powerful toolkit for
working with things that *might* not exist, and then most of the time it
represents `None` as a null pointer.

`Option` is a really beautiful type, and it makes dealing with optional values
safer because it makes us *explicit* about accessing the internal type.

---

## Matching

```rust
match some_option {
  Some(val) => println!("Found a value: {}", val),
  None => println!("Didn't get anything :("),
}
```

Notes: Like tuple unpacking in Python and destructuring assignment in
JavaScript, match expressions let us conditionally execute code based on the
"shape" of a value.

---

## Matching

```rust
if let Some(val) = some_option {
  println!("Found a value: {}", val);
}
```

---

## Cloning

---

## Borrowing

---

## Error handling

---

## Iterators

---

## (De)serialization

---

## Shell scripting
