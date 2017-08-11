# renvsubst

Text substitution tool like envsubst, but it can escape vars.

# Install

```
$ cargo install renvsubst
```

# Usage

Substitutes the values of environment variables.

```
$ export FOO=foooo
$ echo 'I like $FOO' | renvsubst
I like foooo
```

It can escape variable name.

```
$ echo 'I like $FOO, ${FOO}, \$FOO, \${FOO}' | renvsubst
I like foooo, foooo, $FOO, ${FOO}
```
