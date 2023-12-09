# metabrowser

metabrowser delegates opening URLs in other browsers, based on a set of rules.

## Installation

On Windows, metabrowser can be installed with the `install` subcommand. If you
specify the `--set-default` flag, it will then open the Control Panel so that
you may set it as your default browser.

## Configuration

The configuration is located in a platform-specific location:

* Linux: `~/.config/metabrowser/metabrowser.yml`
* macOS: `~/Library/Application Support/ca.brennie.metabrowser/metabrowser.yml`
* Windows: `%APPDATA%\brennie\metabrowser\config\metabrowser.yml`

An example configuration is provided in
[`contrib/metabrowser.example.yml`](contrib/metabrowser.example.yml).
