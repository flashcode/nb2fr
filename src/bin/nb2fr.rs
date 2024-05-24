//
// Copyright (C) 2020-2024 Sébastien Helleu <flashcode@flashtux.org>
//
// This file is part of nb2fr.
//
// Nb2fr is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// Nb2fr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with nb2fr.  If not, see <https://www.gnu.org/licenses/>.
//

//
// Convert numbers to literal French text.
//
// Example:
//    $ ./nb2fr 123456
//    cent vingt-trois mille quatre cent cinquante-six
//

use nb2fr::nb2fr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Syntax: {} number [number...]", args[0]);
        std::process::exit(1);
    }
    if args[1] == "-v" || args[1] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    for arg in &args[1..] {
        match arg.parse::<i64>() {
            Ok(number) => println!("{}", nb2fr(number)),
            Err(_err) => eprintln!("Invalid integer: {}", arg),
        }
    }
}
