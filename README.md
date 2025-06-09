<!--
SPDX-FileCopyrightText: 2020-2025 Sébastien Helleu <flashcode@flashtux.org>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# Convert integer numbers to literal French text

[![Crates.io](https://img.shields.io/crates/v/nb2fr.svg)](https://crates.io/crates/nb2fr)
[![Build status](https://github.com/flashcode/nb2fr/workflows/CI/badge.svg)](https://github.com/flashcode/nb2fr/actions?query=workflow%3A%22CI%22)
[![REUSE status](https://api.reuse.software/badge/github.com/flashcode/nb2fr)](https://api.reuse.software/info/github.com/flashcode/nb2fr)

Integer numbers from `-9 223 372 036 854 775 808` to `9 223 372 036 854 775 807` are accepted.

## Usage

In Rust:

```rust
use nb2fr::nb2fr;

fn main() {
    assert_eq!(nb2fr(123), "cent vingt-trois");
}
```

From command line:

```
$ nb2fr 0 -15 123456 6824718
zéro
moins quinze
cent vingt-trois mille quatre cent cinquante-six
six millions huit cent vingt-quatre mille sept cent dix-huit
```

## Copyright

<!-- REUSE-IgnoreStart -->
Copyright © 2020-2025 [Sébastien Helleu](https://github.com/flashcode)

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
<!-- REUSE-IgnoreEnd -->
