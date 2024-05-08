# Rah - Rusty AUR Helper
Some time ago, I tried to do my own [AUR helper](https://github.com/kStor2poche/yaah/) using bash just for fun, but I quickly abandonned the project because it was made in quite a convoluted way (plus I found out later that the name was already taken by another AUR helper 🥲).

So here I am, trying to make an AUR helper that
- is fast
- interferes as little as possible with pacman
- has pacman-like syntax

and most importantly 
- provides interactive mecanisms when things go bad during an install in order to fix everything, rather than trying to do its own thing and ending up breaking everything

## Structure
- main.rs &rarr; control flow
- [subcommand].rs &rarr; functions related to each of these subcommands that will be launched from main
- database.rs &rarr; helper functions for managing the package database
