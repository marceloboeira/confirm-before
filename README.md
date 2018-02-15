<p align="center">
  <img src="https://memecrunch.com/meme/5RNOG/are-you-sure/image.jpg?w=552&c=1" width="400">
  <h3 align="center">No? Confirm Before</h3>
  <p align="center">Sanity check for your shell commands<p>
</p>

## Installing

```
brew tap marceloboeira/formulas
brew install marceloboeira/formulas/confirm-before
```

## Motivation

I have recently started working with a lot of DevOps CLIs that interact with staging and production servers.

My shell setup includes aliases like:

```
alias ks=`kubectl --context staging`
alias kp=`kubectl --context production`
```

However, I frequently find myself almost running something wrong by not paying attention if its production/staging. Therefore, I decided to create this small 'command wrapper' for me to make sure if I really want to run that command against production. e.g.:

```
alias kp=`confirm-before kubectl --context production`
```

Then, ...

```
$ kp apply -F *
Are you sure? (y/n)
```
