# Confirm Before
> Sanity check for your shell commands

## Motivation

I have recently started working with a lot of DevOps CLIs that interact with production servers, but also with staging servers. On my setup I have aliases like:

```
ks -> kubectl --context staging
kp -> kubectl --context production
```

Frequently, I just use my command history to find old commands, and `ks`/`kp` is very easy to get confused by.

### Goal

### Sanity Check

Declare aliases

```
alias kp=`confirm-before kubectl --context production`
```

Use

```
$ kp apply -f *
Are you sure? (y/n)
n
```
