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

use crate::extractor::extract;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Style};
use clap::{Args, FromArgMatches, Parser};
use vfs::VfsPath;

#[derive(Parser, Debug)]
#[command(name = "tge", version, about)]
pub struct Cli {
    #[arg(
        required = true,
        help = "Template files from which you want to extract gettext strings"
    )]
    pub input_files: Vec<String>,

    #[arg(
        short,
        long,
        default_value = "template_messages.pot",
        help = "File in which extracted messages should go",
        long_help = "If file already exists, it will empty its content before writing in it"
    )]
    pub output_file: String,
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .error(AnsiColor::Red.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(Style::new().bold())
        .placeholder(Style::new().italic())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
        .context(AnsiColor::Magenta.on_default())
}

fn validate(cli: Cli, current_dir: &VfsPath) -> anyhow::Result<Cli> {
    let input_files: Vec<String> = cli
        .input_files
        .iter()
        .filter_map(|file| {
            let result = file.trim().to_string();
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        })
        .collect();
    if input_files.is_empty() {
        return Err(anyhow::Error::msg(
            "You must provide at least one input file",
        ));
    }

    if !input_files.iter().all(|file| {
        current_dir
            .join(file)
            .is_ok_and(|path| path.is_file().unwrap_or(false))
    }) {
        return Err(anyhow::Error::msg("Some given input files cannot be read"));
    }

    let output_file = cli.output_file.trim().to_string();
    if output_file.is_empty() {
        return Err(anyhow::Error::msg("Please provide a valid output file"));
    }

    Ok(Cli {
        input_files,
        output_file,
    })
}

pub fn run(args: Vec<String>, current_dir: VfsPath) -> anyhow::Result<()> {
    let cli = clap::Command::new("tge")
        .styles(get_styles())
        .arg_required_else_help(true)
        .help_expected(true);
    let cli = Cli::augment_args(cli);
    let matches = cli.get_matches_from(args);

    let cli = Cli::from_arg_matches(&matches)?;
    let cli = validate(cli, &current_dir)?;

    extract(cli, current_dir)
}

#[cfg(test)]
mod tests {
    use crate::cli::{Cli, validate};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use vfs::{MemoryFS, VfsPath};

    fn build_fake_fs() -> VfsPath {
        let fs: VfsPath = MemoryFS::new().into();
        let file = fs.join("existing_file").unwrap();
        file.create_file().unwrap();
        fs
    }

    #[test]
    fn test_validate_returns_ok_when_all_good() {
        assert_eq!(
            true,
            validate(
                Cli {
                    input_files: vec!["existing_file".to_string()],
                    output_file: "something".to_string(),
                },
                &build_fake_fs()
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_returns_err_when_input_files_are_empty() {
        let result = validate(
            Cli {
                input_files: vec!["".to_string()],
                output_file: "something".to_string(),
            },
            &build_fake_fs(),
        );
        assert_eq!(true, result.is_err());
        assert_str_eq!(
            "You must provide at least one input file",
            format!("{}", result.unwrap_err())
        );
    }

    #[test]
    fn test_validate_returns_err_when_input_file_does_not_exist() {
        let result = validate(
            Cli {
                input_files: vec!["non_existing".to_string()],
                output_file: "something".to_string(),
            },
            &build_fake_fs(),
        );
        assert_eq!(true, result.is_err());
        assert_str_eq!(
            "Some given input files cannot be read",
            format!("{}", result.unwrap_err())
        );
    }

    #[test]
    fn test_validate_returns_err_when_output_file_is_empty() {
        let result = validate(
            Cli {
                input_files: vec!["existing_file".to_string()],
                output_file: "".to_string(),
            },
            &build_fake_fs(),
        );
        assert_eq!(true, result.is_err());
        assert_str_eq!(
            "Please provide a valid output file",
            format!("{}", result.unwrap_err())
        );
    }
}
