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

use crate::cli::Cli;
use crate::regex_collection::{RegexCollection, get_mustache_regex_collection};
use std::collections::HashSet;
use vfs::VfsPath;

fn extract_from_file_content(
    file_content: String,
    messages: &mut HashSet<String>,
    regex_collection: &RegexCollection,
) -> anyhow::Result<()> {
    for captures in regex_collection.gettext.captures_iter(&file_content) {
        if let Some(value) = captures.name("value") {
            messages.insert(value.as_str().trim().to_string());
        }
    }

    Ok(())
}

fn write_messages(
    messages: HashSet<String>,
    current_dir: VfsPath,
    output_file: String,
) -> anyhow::Result<()> {
    let file_path = current_dir.join(output_file)?;
    let mut file = if !file_path.is_file()? {
        file_path.create_file()?
    } else {
        file_path.append_file()?
    };

    let mut messages_to_write: Vec<String> = messages
        .iter()
        .map(|message| format!("msgid \"{}\"\nmsgstr \"\"\n", message.replace('"', "\\\"")))
        .collect();
    messages_to_write.sort();

    write!(file, "{}", messages_to_write.join("\n").as_str())?;
    Ok(())
}

pub fn extract(cli: Cli, current_dir: VfsPath) -> anyhow::Result<()> {
    let mut messages: HashSet<String> = HashSet::new();

    let regex_collection = get_mustache_regex_collection()?;

    for input_file in cli.input_files {
        let file_path = current_dir.join(input_file)?;
        let file_content = file_path.read_to_string()?;

        extract_from_file_content(file_content, &mut messages, &regex_collection)?;
    }

    write_messages(messages, current_dir, cli.output_file)
}

#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use crate::extractor::{extract, write_messages};
    use pretty_assertions::assert_str_eq;
    use std::collections::HashSet;
    use std::fs;
    use vfs::{MemoryFS, VfsPath};

    #[test]
    fn test_write_messages_0() -> anyhow::Result<()> {
        let mut messages = HashSet::new();
        messages.insert("Some value".to_string());
        let root: VfsPath = MemoryFS::new().into();
        let output_file = root.join("messages.pot")?;
        write_messages(messages, root, "messages.pot".to_string())?;
        assert_str_eq!(
            fs::read_to_string("src/_fixtures/test_write_messages_0.pot")
                .expect("Failed to read fixtures file"),
            output_file.read_to_string()?
        );
        Ok(())
    }

    #[test]
    fn test_write_messages_1() -> anyhow::Result<()> {
        let mut messages = HashSet::new();
        messages.insert("Page not found".to_string());
        messages.insert("You may want to return to <a href=\"/\">home page</a>.".to_string());
        let root: VfsPath = MemoryFS::new().into();
        let output_file = root.join("messages.pot")?;
        write_messages(messages, root, "messages.pot".to_string())?;
        assert_str_eq!(
            fs::read_to_string("src/_fixtures/test_write_messages_1.pot")
                .expect("Failed to read fixtures file"),
            output_file.read_to_string()?
        );
        Ok(())
    }

    #[test]
    fn test_extract() -> anyhow::Result<()> {
        let root: VfsPath = MemoryFS::new().into();
        let output_file = root.join("messages.pot")?;
        write!(
            root.join("test_extract_from_file_content_4.mustache")?
                .create_file()?,
            "{}",
            fs::read_to_string("src/_fixtures/test_extract_from_file_content_4.mustache")
                .expect("Failed to read fixtures file")
        )?;
        write!(
            root.join("test_extract_from_file_content_5.mustache")?
                .create_file()?,
            "{}",
            fs::read_to_string("src/_fixtures/test_extract_from_file_content_5.mustache")
                .expect("Failed to read fixtures file")
        )?;
        write!(
            root.join("some_file.mustache")?.create_file()?,
            "{}",
            "{{#gettext}}Some value{{/gettext}}"
        )?;
        extract(
            Cli {
                input_files: vec![
                    "test_extract_from_file_content_4.mustache".to_string(),
                    "test_extract_from_file_content_4.mustache".to_string(),
                    "test_extract_from_file_content_5.mustache".to_string(),
                    "some_file.mustache".to_string(),
                ],
                output_file: "messages.pot".to_string(),
            },
            root,
        )?;

        assert_str_eq!(
            fs::read_to_string("src/_fixtures/test_extract.pot")
                .expect("Failed to read fixtures file"),
            output_file.read_to_string()?
        );

        Ok(())
    }
}
