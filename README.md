# cnx-renvsubst

A Rust implementation of `envsubst`.

# Usage

Substitutes the values of environment variables.

Non-existing env vars will **NOT** (unlike with `envsubst`) be replaced with an empty string.

```
$ export FOO=foooo
$ echo 'I like $FOO' | cargo run
I like foooo

$ echo 'I like $FOO, ${FOO}, \$FOO, \${FOO}, \\${FOO}' | cargo run
I like foooo, foooo, \foooo, \foooo, \foooo
```

# Optimized Build
```bash
export RUSTFLAGS="-C strip=symbols -C target-cpu=native"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
export CARGO_PROFILE_RELEASE_LTO=true
export CARGO_PROFILE_RELEASE_OPT_LEVEL=s
export CARGO_PROFILE_RELEASE_PANIC=abort

cargo build --release
```
