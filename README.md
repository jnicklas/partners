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

``` sh
git clone https://github.com/elabs/partners.git
cd partners
curl -L https://static.rust-lang.org/rustup.sh | sudo sh
cargo build --release
cp target/release/partners /usr/local/bin/partners
```

This will be easier once a stable version of Rust is released.

Partners is unfortunately still lacking an interactive setup method. Copy the
example into your home directory:

``` sh
cp default.cfg ~/.partners.cfg
```

You can examine this file, you'll probably want to change the domain.

## Usage



