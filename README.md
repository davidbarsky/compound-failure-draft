# compound-failure-draft

This is a quick sketch to illustrate how I'd handle errors that _don't_ short-circuit. In applications or servers, most errors are nicely handled by the default behavior defined in the `?` operator. However, for a certain class of programs, it's preferential to accumulate errors and display them all at once. For instance, if we wanted to validate a JSON blob, this crate would provide the following output:

```bash
compound-failure:master* >> cargo run
   Compiling compound-failure v0.1.0 (file:///Users/dbarsky/Developer/Rust/compound-failure)
    Finished dev [unoptimized + debuginfo] target(s) in 0.81 secs
     Running `target/debug/compound-failure`
JSON was missing field: author
JSON was missing field: email
JSON was missing field: git
```

One can imagine more advanced diagnostics and formatting that includes colors, line numbers, and inline errors.
