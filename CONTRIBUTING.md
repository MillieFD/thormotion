# Contributing Guidelines

Thank you for your interest in contributing to Thormotion! We welcome all contributions, whether they are bug
reports, feature suggestions, documentation improvements, or code changes. Please follow the guidelines below to
ensure a smooth collaboration.

# 📖 Documentation

Documentation is an incredibly important part of the Thormotion project. Remember that scientists and researchers
using Thorlabs products may not be avid Rust or Python programmers. It is our responsibility to ensure that
Thormotion is easy-to-use, approachable, and consistent. The following documentation guidelines were inspired by
[RFC 1574](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html).

### English

All documentation should use Oxford English in regard to spelling, grammar, and punctuation conventions. Oxford
commas are permitted if they help to improve the clarity of text or prevent misinterpretation of list elements.
"For example" may be abbreviated to "e.g." and "In other words" may be abbreviated to "i.e." but these should not be
followed by a comma (Do not use "e.g.," or "i.e.,").

### Numbers

As a general rule, use words for single-digit numbers and use numerals for everything else.

- Spell out all single-digit cardinal numbers (e.g. "Six" instead of "6").
- Spell out all single-digit ordinal numbers (e.g. "Sixth" instead of "6th").
- Use numerals for all other numbers (e.g. "76" instead of "seventy-six").
- Use numerals for all negative numbers (e.g. "-6" instead of "minus six").
- Use numerals for all multiple-digit ordinal numbers (e.g. "21st" instead of "twenty-first").
- Use numerals for all ranges, even if the range includes single-digit numbers (e.g. "1-9" instead of "one to nine").

### Markdown

Use Markdown to format your documentation.

Use block-comments (`/**` and `*/`) instead of line-comments (`///`). Do not prefix each line with `*`. Begin and
end your block-comments on new lines with no other text. For example:

```markdown
/**
This is correct!
*/

/** This is NOT correct.
Please begin and end your block-comments on new lines with no other text.
*/

/**

* This is NOT correct.
* Please do not prefix each line with `*`.
  */

/// This is NOT correct. Please use block-comments instead of line-comments.
```

Use top-level headings (#) to indicate sections within the comment. Capitalise each word in the heading. Common
headings include:

- Examples
- Panics
- Errors

Always use plural headings. For example, use "Examples" rather than "Example", even if there is only one example.
This makes future tooling easier.

Use backticks (`) to denote an inline code fragment within a sentence.

Use triple backticks (```) to write longer examples. Always annotate triple backtick blocks with the appropriate
formatting directive (rust). This will highlight syntax in places that do not default to Rust, like GitHub.
For example:

````markdown
```rust
let x = foo();
x.bar();
```
````

Use double quotation marks (") by default. Only use single quotation marks (') if the text is already bounded by
double quotation marks (").

Lines must not exceed 100 characters in length. Prefer inserting single line breaks at punctuation where possible
(e.g. after full-stops, after commas, before opening brackets, after closing brackets, before opening backticks,
or after closing back-ticks).

### Function Doc Comments

The first line should always be a single-line short sentence that explains what the function does. The summary line
should be written in third-person singular present indicative form (i.e. "Returns" instead of "Return").

If the rust function directly relates to commands described in the Thorlabs APT protocol, add the `apt_doc` macro to
automatically generate function documentation. Include the `command names` as they appear in the APT protocol
documentation. If the function returns a `Result<T, E>` enum, list all the possible error types which could occur.
The `apt_doc` macro will automatically include a description for each error type.

# 🧑‍💻Code Style Conventions

A consistent code style helps to ensure that the codebase remains approachable, maintainable, and scalable over time.
It helps all contributors – regardless of experience level – to read, understand, and work within the project more
effectively. The style conventions outlined below are designed to reduce developers' cognitive load by providing
predictable patterns, making it easier to identify and fix bugs, add new features, and review code changes.

### Prefer Asynchronicity

Thormotion uses the `async_std` crate to provide an asynchronous implementation of the standard library. Where
possible, you should use asynchronous operations rather than their synchronous counterparts. For example, prefer
`async_std::sync::RwLock` instead of `std::sync::RwLock`, or `async_std::sync::Arc` instead of `std::sync::Arc`.
Refer to the [async-std documentation](https://docs.rs/async-std/latest/async_std/index.html) for detailed guidance
regarding the `async_std` crate.

### Synchronous and Asynchronous Functions

All Thorlabs APT functions involve exchanging data with an external Thorlabs USB device. IO operations are generally
considered to take an indeterminate length of time, which makes synchronous (blocking) functions and busy-waiting
undesirable. In addition, many Thorlabs APT functions require the connected device to perform operations (such as
movement or a homing sequence) which have no fixed duration and may require significant time. For this reason,
Thormotion is designed with an "async first" approach.

The core logic for all Thorlabs APT functions should be implemented as an `async` function. The function name should
end with `_async` to explicitly indicate its asynchronicity. For example, `home_async()` or `move_absolute_async()`.

A synchronous version of each Thorlabs APT function must also be provided. Internally, the synchronous function
should wrap the asynchronous function in a `async_std::task::block_on(function_name_async)`. This helps to keep the
codebase more maintainable by minimizing boilerplate.

Make sure to document the function correctly using the `apt_doc` macro (check out the **Documentation** section
above). This helps prevent confusion and allows users to choose the most appropriate version based on their needs.

### Prefer Result

Wherever possible, functions should return a `Result<T, Error>` instead of panicking. This improves flexibility
by allowing end-users to handle errors as appropriate for their specific use-case.

Use the `?` operator to automatically propagate errors wherever applicable. This keeps the code concise and clear.

If returning an error is not feasible, prefer using `.expect()` with a helpful and descriptive error message. Use
`&format!()` to dynamically produce error messages that incorporate runtime variables, such as the device's serial
number. This produces clearer error messages that are more helpful for end-users. Only use a string literal `&str` if
the error message does not depend on any runtime variables (this is rare).

```rust
fn get_channel<'a>(message_id: [u8; 2]) -> &'a RwLock<Option<Channel>> {
    CHANNEL_MAP.get(&message_id).expect(&format!(
        "Failed to get channel. {:?} does not correspond to a known message ID",
        message_id
    ))
}
```

### Strings

Use double quotation marks (") by default. Only use single quotation marks (') if text is already bounded by double
quotation marks (").

# 📝 License

This project is licensed under the BSD 3-Clause License. Opening a pull request indicates agreement with these terms.
