# Partners

Partners is a convenient command line tool for switching between different git
users, and combining them when multiple people are working together. It is a
similar alternative to gitswitch.

## Why?

Because switching between different user settings on shared workstations,
especially when pair programming can happen quite frequently, and the existing
tools don't address the problem very nicely.

We've used gitswitch for a long time, but it has a lot of problems:

- Requires cartesian join of all people who are meant to be pair-programming
- Defaults to repository-local switch, which is usually not what you want
- Slow to start, slow to execute
- Dependent on Ruby toolchain, since we primarily work in Ruby and switch Ruby
  version and gemset frequently, this is especially annoying.

All other alternatives we could find seem to be incomplete or unmaintained.

## Installation

First, install Rust and Cargo. You can use [rustup](https://www.rust-lang.org/install.html),
or on OS X you can simply install them via Homebrew like this:

``` sh
$ brew install rust
```

Now install Partners via Cargo:

``` sh
$ cargo install partners
```

It might prompt you to add Cargo's install directory to your path.

Now run partners' interactive setup:

``` sh
$ partners setup
```

This will prompt you to create the configuration file if it doesn't exist.

## Usage

Partners maintains a list of authors it knows about, you can inspect this list by running:

``` sh
$ partners list
```

You can add a new author by running:

``` sh
$ partners add
```

This will prompt you for their nickname, name and email address. You can use
the nickname to quickly change your git author information like this:

``` sh
$ partners set jonas
jonas:
  Name:  Jonas Nicklas
  Email: jonas.nicklas@gmail.com
```

Where `jonas` is the nickname. You can also use multiple nicknames:

``` sh
$ partners set jonas kim
jonas+kim:
  Name:  Jonas Nicklas, Kim Burgestrand
  Email: dev+jonas+kim@elabs.se
```

In the case of multiple authors, partners constructs the email address based
on the authors' nicknames, and the domain, prefix and separator configuration
parameters specified during setup. You can change these any time by running:

``` sh
partners setup
```