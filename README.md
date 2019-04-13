# Shamrock – a Simple WordPress Plugin Scaffolder

A very simple WordPress plugin scaffolding tool. Mainly for
myself as I learn Rust and continue to use WordPress
(_shutters_).

Shamrock is a baby cousin to [Clover] — a fully featured plugin
scaffolding tool/boilerplate in development by [Roots].

Right now, Shamrock just modifies the plugin header and fills it
with my details. You can use flags to add your details. See the
usage with: `shamrock help new`

## Usage

Create a new plugin (using my details):

```shell
$ shamrock new Example
Created example.php ☘
```

Shamrock automatically converts the name you enter to the
correct format for filenames or plugin names, so you can enter
either (just make sure you’re wrapping with quotes or escaping
with backslashes if you have spaces).

Create a new plugin with a description:

```shell
$ shamrock new example --desc "An example plugin demonstrating Shamrock."
Created example.php ☘
```

Use `shamrock help` and `shamrock help <subcommand>` for more
usage details.

## Installation

You will need [Rust + Cargo](https://rustup.rs/) installed to
install the binary.

You can either install using the URL to the repo:

```shell
$ cargo install --git https://github.com/knowler/shamrock
$ mkdir ~/.config/shamrock
$ curl https://raw.githubusercontent.com/knowler/shamrock/master/src/templates/shamrock.php -o ~/.config/shamrock/shamrock.php
```

Or install after cloning.

```shell
$ git clone https://github.com/knowler/shamrock
$ cd shamrock
$ cargo install --path .
$ mkdir ~/.config/shamrock
$ cp src/templates/shamrock.php ~/.config/shamrock/shamrock.php
```

## Future

It would be great if this didn’t use my details by default, but
it is a personal project. If you’d like to hack away and make it
load author details from the global Git config, PRs are
accepted ([git2] would be a good place to start for this).

[Clover]: https://roots.io/clover/
[Roots]: https://roots.io/
[git2]: https://docs.rs/git2/
