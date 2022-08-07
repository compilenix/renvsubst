# renvsubst

Text substitution tool like envsubst, but it can escape vars.

# Usage

Substitutes the values of environment variables.

```
$ export FOO=foooo
$ echo 'I like $FOO' | cargo run
I like foooo
```

It can escape variable name.

```
$ echo 'I like $FOO, ${FOO}, \$FOO, \${FOO}' | cargo run
I like foooo, foooo, $FOO, ${FOO}
```
