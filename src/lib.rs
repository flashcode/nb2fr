//
// SPDX-FileCopyrightText: 2020-2026 Sébastien Helleu <flashcode@flashtux.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later
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

//! Convert numbers to literal French text.
//!
//! 128-bit signed integers are supported:
//! - from [`i128::MIN`] (`-170 141 183 460 469 231 731 687 303 715 884 105 728`)
//! - to [`i128::MAX`] (`170 141 183 460 469 231 731 687 303 715 884 105 727`).

pub const NUMBER_FR_1_99: [&str; 99] = [
    "un",
    "deux",
    "trois",
    "quatre",
    "cinq",
    "six",
    "sept",
    "huit",
    "neuf",
    "dix",
    "onze",
    "douze",
    "treize",
    "quatorze",
    "quinze",
    "seize",
    "dix-sept",
    "dix-huit",
    "dix-neuf",
    "vingt",
    "vingt et un",
    "vingt-deux",
    "vingt-trois",
    "vingt-quatre",
    "vingt-cinq",
    "vingt-six",
    "vingt-sept",
    "vingt-huit",
    "vingt-neuf",
    "trente",
    "trente et un",
    "trente-deux",
    "trente-trois",
    "trente-quatre",
    "trente-cinq",
    "trente-six",
    "trente-sept",
    "trente-huit",
    "trente-neuf",
    "quarante",
    "quarante et un",
    "quarante-deux",
    "quarante-trois",
    "quarante-quatre",
    "quarante-cinq",
    "quarante-six",
    "quarante-sept",
    "quarante-huit",
    "quarante-neuf",
    "cinquante",
    "cinquante et un",
    "cinquante-deux",
    "cinquante-trois",
    "cinquante-quatre",
    "cinquante-cinq",
    "cinquante-six",
    "cinquante-sept",
    "cinquante-huit",
    "cinquante-neuf",
    "soixante",
    "soixante et un",
    "soixante-deux",
    "soixante-trois",
    "soixante-quatre",
    "soixante-cinq",
    "soixante-six",
    "soixante-sept",
    "soixante-huit",
    "soixante-neuf",
    "soixante-dix",
    "soixante et onze",
    "soixante-douze",
    "soixante-treize",
    "soixante-quatorze",
    "soixante-quinze",
    "soixante-seize",
    "soixante-dix-sept",
    "soixante-dix-huit",
    "soixante-dix-neuf",
    "quatre-vingts",
    "quatre-vingt-un",
    "quatre-vingt-deux",
    "quatre-vingt-trois",
    "quatre-vingt-quatre",
    "quatre-vingt-cinq",
    "quatre-vingt-six",
    "quatre-vingt-sept",
    "quatre-vingt-huit",
    "quatre-vingt-neuf",
    "quatre-vingt-dix",
    "quatre-vingt-onze",
    "quatre-vingt-douze",
    "quatre-vingt-treize",
    "quatre-vingt-quatorze",
    "quatre-vingt-quinze",
    "quatre-vingt-seize",
    "quatre-vingt-dix-sept",
    "quatre-vingt-dix-huit",
    "quatre-vingt-dix-neuf",
];
pub const NUMBER_FR_1000: [&str; 12] = [
    "mille",
    "million",
    "milliard",
    "billion",
    "billiard",
    "trillion",
    "trilliard",
    "quadrillion",
    "quadrilliard",
    "quintillion",
    "quintilliard",
    "sextillion",
];

/// Convert a signed 128-bit integer to literal French text.
///
/// # Examples
///
/// ```
/// assert_eq!(nb2fr::nb2fr(0), "zéro");
/// assert_eq!(nb2fr::nb2fr(-42), "moins quarante-deux");
/// assert_eq!(nb2fr::nb2fr(123), "cent vingt-trois");
/// assert_eq!(nb2fr::nb2fr(2_000_000_000), "deux milliards");
/// ```
#[must_use]
pub fn nb2fr(number: i128) -> String {
    if number == 0 {
        return String::from("zéro");
    }
    let mut result: Vec<String> = Vec::new();
    if number < 0 {
        result.push(String::from("moins"));
    }
    let mut str_number = number.unsigned_abs().to_string();
    // Pad with leading zeroes to have a multiple of 3 digits,
    // for example: "1234" -> "001234".
    let zeroes_to_add = (3 - (str_number.len() % 3)) % 3;
    str_number.insert_str(0, &"0".repeat(zeroes_to_add));
    let mut nb3 = str_number.len() / 3;
    let mut index = 0;
    while nb3 > 0 {
        let num_grp3 = str_number[index..index + 3]
            .parse::<u32>()
            .unwrap_or_default();
        if num_grp3 > 0 {
            if num_grp3 != 1 || nb3 != 2 {
                if num_grp3 > 99 {
                    if num_grp3 / 100 > 1 {
                        result.push(String::from(
                            NUMBER_FR_1_99[((num_grp3 as usize) / 100) - 1],
                        ));
                    }
                    let plural = if num_grp3 / 100 > 1 && num_grp3 % 100 == 0 {
                        "s"
                    } else {
                        ""
                    };
                    result.push(String::from("cent") + plural);
                }
                if num_grp3 % 100 != 0 {
                    result.push(String::from(
                        NUMBER_FR_1_99[((num_grp3 as usize) % 100) - 1],
                    ));
                }
            }
            if nb3 > 1 {
                let plural = if num_grp3 > 1 && nb3 > 2 { "s" } else { "" };
                result.push(String::from(NUMBER_FR_1000[nb3 - 2]) + plural);
            }
        }
        index += 3;
        nb3 -= 1;
    }
    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb2fr() {
        assert_eq!(nb2fr(0), "zéro");
        assert_eq!(nb2fr(1), "un");
        assert_eq!(nb2fr(5), "cinq");
        assert_eq!(nb2fr(10), "dix");
        assert_eq!(nb2fr(100), "cent");
        assert_eq!(nb2fr(123), "cent vingt-trois");
        assert_eq!(nb2fr(200), "deux cents");
        assert_eq!(
            nb2fr(123_456_789),
            "cent vingt-trois millions \
            quatre cent cinquante-six mille \
            sept cent quatre-vingt-neuf"
        );
        assert_eq!(
            nb2fr(123_456_789_012),
            "cent vingt-trois milliards \
            quatre cent cinquante-six millions \
            sept cent quatre-vingt-neuf mille \
            douze"
        );
        assert_eq!(nb2fr(2_000), "deux mille");
        assert_eq!(nb2fr(2_000_000), "deux millions");
        assert_eq!(nb2fr(2_000_000_000), "deux milliards");
        assert_eq!(nb2fr(2_000_000_000_000), "deux billions");
        assert_eq!(nb2fr(2_000_000_000_000_000), "deux billiards");
        assert_eq!(nb2fr(2_000_000_000_000_000_000), "deux trillions");
        assert_eq!(nb2fr(2_000_000_000_000_000_000_000), "deux trilliards");
        assert_eq!(
            nb2fr(2_000_000_000_000_000_000_000_000),
            "deux quadrillions"
        );
        assert_eq!(
            nb2fr(2_000_000_000_000_000_000_000_000_000),
            "deux quadrilliards"
        );
        assert_eq!(
            nb2fr(2_000_000_000_000_000_000_000_000_000_000),
            "deux quintillions"
        );
        assert_eq!(
            nb2fr(2_000_000_000_000_000_000_000_000_000_000_000),
            "deux quintilliards"
        );
        assert_eq!(
            nb2fr(2_000_000_000_000_000_000_000_000_000_000_000_000),
            "deux sextillions"
        );
        assert_eq!(
            nb2fr(i128::MIN), // -170_141_183_460_469_231_731_687_303_715_884_105_728
            "moins cent soixante-dix sextillions \
            cent quarante et un quintilliards \
            cent quatre-vingt-trois quintillions \
            quatre cent soixante quadrilliards \
            quatre cent soixante-neuf quadrillions \
            deux cent trente et un trilliards \
            sept cent trente et un trillions \
            six cent quatre-vingt-sept billiards \
            trois cent trois billions \
            sept cent quinze milliards \
            huit cent quatre-vingt-quatre millions \
            cent cinq mille \
            sept cent vingt-huit"
        );
        assert_eq!(
            nb2fr(i128::MAX), // 170_141_183_460_469_231_731_687_303_715_884_105_727
            "cent soixante-dix sextillions \
            cent quarante et un quintilliards \
            cent quatre-vingt-trois quintillions \
            quatre cent soixante quadrilliards \
            quatre cent soixante-neuf quadrillions \
            deux cent trente et un trilliards \
            sept cent trente et un trillions \
            six cent quatre-vingt-sept billiards \
            trois cent trois billions \
            sept cent quinze milliards \
            huit cent quatre-vingt-quatre millions \
            cent cinq mille \
            sept cent vingt-sept"
        );
    }
}
