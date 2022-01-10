//
// Copyright (C) 2020-2022 Sébastien Helleu <flashcode@flashtux.org>
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

/// Converts an integer to literal French text.
///
/// # Examples:
///
///     assert_eq!(nb2fr::nb2fr(0), "zéro");
///     assert_eq!(nb2fr::nb2fr(-42), "moins quarante-deux");
///     assert_eq!(nb2fr::nb2fr(123), "cent vingt-trois");
///     assert_eq!(nb2fr::nb2fr(2_000_000_000), "deux milliards");
pub fn nb2fr(number: i64) -> String {
    let number_1_99 = [
        "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
        "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize",
        "dix-sept", "dix-huit", "dix-neuf", "vingt", "vingt et un",
        "vingt-deux", "vingt-trois", "vingt-quatre", "vingt-cinq", "vingt-six",
        "vingt-sept", "vingt-huit", "vingt-neuf", "trente", "trente et un",
        "trente-deux", "trente-trois", "trente-quatre", "trente-cinq",
        "trente-six", "trente-sept", "trente-huit", "trente-neuf", "quarante",
        "quarante et un", "quarante-deux", "quarante-trois", "quarante-quatre",
        "quarante-cinq", "quarante-six", "quarante-sept", "quarante-huit",
        "quarante-neuf", "cinquante", "cinquante et un", "cinquante-deux",
        "cinquante-trois", "cinquante-quatre", "cinquante-cinq",
        "cinquante-six", "cinquante-sept", "cinquante-huit", "cinquante-neuf",
        "soixante", "soixante et un", "soixante-deux", "soixante-trois",
        "soixante-quatre", "soixante-cinq", "soixante-six", "soixante-sept",
        "soixante-huit", "soixante-neuf", "soixante-dix", "soixante et onze",
        "soixante-douze", "soixante-treize", "soixante-quatorze",
        "soixante-quinze", "soixante-seize", "soixante-dix-sept",
        "soixante-dix-huit", "soixante-dix-neuf", "quatre-vingts",
        "quatre-vingt-un", "quatre-vingt-deux", "quatre-vingt-trois",
        "quatre-vingt-quatre", "quatre-vingt-cinq", "quatre-vingt-six",
        "quatre-vingt-sept", "quatre-vingt-huit", "quatre-vingt-neuf",
        "quatre-vingt-dix", "quatre-vingt-onze", "quatre-vingt-douze",
        "quatre-vingt-treize", "quatre-vingt-quatorze", "quatre-vingt-quinze",
        "quatre-vingt-seize", "quatre-vingt-dix-sept", "quatre-vingt-dix-huit",
        "quatre-vingt-dix-neuf",
    ];
    let number_1000 = [
        "mille", "million", "milliard", "billion", "billiard", "trillion",
    ];
    if number == 0 {
        return String::from("zéro");
    }
    let mut result = String::new();
    if number < 0 {
        result.push_str("moins");
    }
    let mut str_number = number.to_string().replace("-", "");
    let zeroes_to_add = (3 - (str_number.len() % 3)) % 3;
    str_number.insert_str(0, &"0".repeat(zeroes_to_add));
    let mut nb3 = ((str_number.len() / 3) - 1) as i64;
    let mut index = 0;
    while nb3 >= 0 {
        let grp3 = &str_number[index..index + 3];
        let num_grp3 = grp3.parse::<i64>().unwrap();
        if num_grp3 > 0 {
            if num_grp3 != 1 || nb3 != 1 {
                if num_grp3 > 99 {
                    if num_grp3 / 100 > 1 {
                        if !result.is_empty() {
                            result.push(' ');
                        }
                        let index_num = (num_grp3 / 100) - 1;
                        result.push_str(number_1_99[index_num as usize]);
                    }
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str("cent");
                    if num_grp3 / 100 > 1 && num_grp3 % 100 == 0 {
                        result.push('s')
                    }
                }
                if num_grp3 % 100 != 0 {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    let index_num = (num_grp3 % 100) - 1;
                    result.push_str(number_1_99[index_num as usize]);
                }
            }
            if nb3 > 0 {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(number_1000[(nb3 - 1) as usize]);
                if num_grp3 > 1 && nb3 > 1 {
                    result.push('s');
                }
            }
        }
        index += 3;
        nb3 -= 1;
    }
    result
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
        assert_eq!(nb2fr(123_456_789),
                   "cent vingt-trois millions \
                    quatre cent cinquante-six mille \
                    sept cent quatre-vingt-neuf");
        assert_eq!(nb2fr(123_456_789_012),
                   "cent vingt-trois milliards \
                    quatre cent cinquante-six millions \
                    sept cent quatre-vingt-neuf mille \
                    douze");
        assert_eq!(nb2fr(2_000), "deux mille");
        assert_eq!(nb2fr(2_000_000), "deux millions");
        assert_eq!(nb2fr(2_000_000_000), "deux milliards");
        assert_eq!(nb2fr(2_000_000_000_000), "deux billions");
        assert_eq!(nb2fr(2_000_000_000_000_000), "deux billiards");
        assert_eq!(nb2fr(2_000_000_000_000_000_000), "deux trillions");
        assert_eq!(nb2fr(i64::MIN), // -9_223_372_036_854_775_808
                   "moins neuf trillions \
                    deux cent vingt-trois billiards \
                    trois cent soixante-douze billions \
                    trente-six milliards \
                    huit cent cinquante-quatre millions \
                    sept cent soixante-quinze mille \
                    huit cent huit");
        assert_eq!(nb2fr(i64::MAX), // 9_223_372_036_854_775_807
                   "neuf trillions \
                    deux cent vingt-trois billiards \
                    trois cent soixante-douze billions \
                    trente-six milliards \
                    huit cent cinquante-quatre millions \
                    sept cent soixante-quinze mille \
                    huit cent sept");
    }
}
