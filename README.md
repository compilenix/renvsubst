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
