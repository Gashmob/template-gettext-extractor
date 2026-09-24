/*
 * Template Gettext Extractor
 * Copyright (C) 2026 - Present  Kevin Traini
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use regex::Regex;

pub struct RegexCollection {
    pub gettext: Regex,
}

pub fn get_mustache_regex_collection() -> anyhow::Result<RegexCollection> {
    Ok(RegexCollection {
        gettext: Regex::new(r"\{\{# *gettext *}}(?<value>.*?)\{\{/ *gettext *}}")?,
    })
}

#[cfg(test)]
mod tests {
    use crate::regex_collection::get_mustache_regex_collection;
    use pretty_assertions::assert_str_eq;
    use std::fs;

    #[test]
    fn test_mustache_gettext_0() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for _ in regex_collection.gettext.captures_iter("Hello World!") {
            count += 1;
        }
        assert_eq!(0, count);
    }

    #[test]
    fn test_mustache_gettext_1() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection
            .gettext
            .captures_iter("{{# gettext }} Some value {{/ gettext }}")
        {
            if let Some(value) = captures.name("value") {
                assert_str_eq!(" Some value ", value.as_str());
                count += 1;
            }
        }
        assert_eq!(1, count);
    }

    #[test]
    fn test_mustache_gettext_2() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection
            .gettext
            .captures_iter("{{# gettext }}Some value{{/ gettext }}")
        {
            if let Some(value) = captures.name("value") {
                assert_str_eq!("Some value", value.as_str());
                count += 1;
            }
        }
        assert_eq!(1, count);
    }

    #[test]
    fn test_mustache_gettext_3() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for _ in regex_collection
            .gettext
            .captures_iter("{{# not-gettext }} Some value {{/ not-gettext }}")
        {
            count += 1;
        }
        assert_eq!(0, count);
    }

    #[test]
    fn test_mustache_gettext_4() {
        let expected = vec![
            " Page not found ",
            " You may want to return to <a href=\"/\">home page</a>. ",
        ];
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection.gettext.captures_iter(
            fs::read_to_string("src/_fixtures/test_extract_from_file_content_4.mustache")
                .expect("Failed to read fixtures file")
                .as_str(),
        ) {
            if let Some(value) = captures.name("value") {
                assert_str_eq!(expected[count], value.as_str());
                count += 1;
            }
        }
        assert_eq!(2, count);
    }

    #[test]
    fn test_mustache_gettext_5() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for _ in regex_collection.gettext.captures_iter(
            fs::read_to_string("src/_fixtures/test_extract_from_file_content_5.mustache")
                .expect("Failed to read fixtures file")
                .as_str(),
        ) {
            count += 1;
        }
        assert_eq!(0, count);
    }

    #[test]
    fn test_mustache_gettext_6() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection
            .gettext
            .captures_iter("{{#gettext}}Some value{{/gettext}}")
        {
            if let Some(value) = captures.name("value") {
                assert_str_eq!("Some value", value.as_str());
                count += 1;
            }
        }
        assert_eq!(1, count);
    }

    #[test]
    fn test_mustache_gettext_7() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection
            .gettext
            .captures_iter("{{#gettext }}Some value{{/ gettext}}")
        {
            if let Some(value) = captures.name("value") {
                assert_str_eq!("Some value", value.as_str());
                count += 1;
            }
        }
        assert_eq!(1, count);
    }

    #[test]
    fn test_mustache_gettext_8() {
        let regex_collection = get_mustache_regex_collection().unwrap();
        let mut count = 0;
        for captures in regex_collection
            .gettext
            .captures_iter("{{# gettext}}Some value{{/gettext }}")
        {
            if let Some(value) = captures.name("value") {
                assert_str_eq!("Some value", value.as_str());
                count += 1;
            }
        }
        assert_eq!(1, count);
    }
}
