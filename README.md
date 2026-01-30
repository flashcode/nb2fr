<!--
SPDX-FileCopyrightText: 2020-2026 Sébastien Helleu <flashcode@flashtux.org>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# Nb2fr

[![Crates.io](https://img.shields.io/crates/v/nb2fr.svg)](https://crates.io/crates/nb2fr)
[![Build status](https://github.com/flashcode/nb2fr/workflows/CI/badge.svg)](https://github.com/flashcode/nb2fr/actions?query=workflow%3A%22CI%22)
[![REUSE status](https://api.reuse.software/badge/github.com/flashcode/nb2fr)](https://api.reuse.software/info/github.com/flashcode/nb2fr)

Nb2fr converts integer numbers to literal French text.

Signed 128-bit integers are supported:

- from: `-170 141 183 460 469 231 731 687 303 715 884 105 728` (`i128::MIN` in Rust)
- to: `170 141 183 460 469 231 731 687 303 715 884 105 727` (`i128::MAX` in Rust).

## Usage

In Rust:

```rust
use nb2fr::nb2fr;

fn main() {
    assert_eq!(nb2fr(123), "cent vingt-trois");
}
```

From command line:

```shell
$ nb2fr 0 -15 123456 6824718 715884105727
zéro
moins quinze
cent vingt-trois mille quatre cent cinquante-six
six millions huit cent vingt-quatre mille sept cent dix-huit
sept cent quinze milliards huit cent quatre-vingt-quatre millions cent cinq mille sept cent vingt-sept
```

## Copyright

<!-- REUSE-IgnoreStart -->
Copyright © 2020-2026 [Sébastien Helleu](https://github.com/flashcode)

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
