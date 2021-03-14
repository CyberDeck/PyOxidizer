// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(unused)]
mod code_hash;
#[allow(unused)]
mod macho;
#[allow(unused)]
mod specification;

use {
    crate::macho::{find_signature_data, parse_signature_data, HashType},
    clap::{App, AppSettings, Arg, ArgMatches, SubCommand},
    goblin::mach::{Mach, MachO},
    std::{io::Write, str::FromStr},
};

const EXTRACT_ABOUT: &str = "\
Extract code signature data from a Mach-O binary.

Given the path to a Mach-O binary (including fat/universal) binaries, this
command will parse and print requested data to stdout.

The --data argument controls which data to extract and how to print it.
Possible values are:

blobs
   Low-level information on the records in the embedded code signature.
cms-ber
   BER encoded ASN.1 of the CMS SignedObject message containing a
   cryptographic signature over content. (This will print binary
   to stdout.)
cms-pem
   Like cms-ber except it prints PEM encoded data, which is ASCII and
   safe to print to terminals.
code-directory
   Information on the main code directory data structure.
linkededit-segment-raw
   Complete content of the __LINKEDIT Mach-O segment as binary.
requirements
   Parsed code requirement statement/expression.
segment-info
   Information about Mach-O segments in the binary and where the
   __LINKEDIT is in relationship to the binary.
superblob
   The SuperBlob record and high-level details of embedded Blob
   records, including digests of every Blob.
";

#[derive(Debug)]
enum AppError {
    UnknownCommand,
    BadArgument,
    Io(std::io::Error),
    Goblin(goblin::error::Error),
    MachOError(crate::macho::MachOParseError),
    NoCodeSignature,
    NoCmsData,
    Digest(crate::macho::DigestError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadArgument => f.write_str("bad argument"),
            Self::UnknownCommand => f.write_str("unknown command"),
            Self::Io(e) => f.write_fmt(format_args!("I/O error: {:?}", e)),
            Self::Goblin(e) => f.write_fmt(format_args!("error parsing binary: {:?}", e)),
            Self::MachOError(e) => f.write_fmt(format_args!("Mach-O parsing error: {:?}", e)),
            Self::NoCodeSignature => f.write_str("code signature data not found"),
            Self::NoCmsData => f.write_str("CMS data structure not found"),
            Self::Digest(e) => f.write_fmt(format_args!("digest error: {}", e)),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<goblin::error::Error> for AppError {
    fn from(e: goblin::error::Error) -> Self {
        Self::Goblin(e)
    }
}

impl From<crate::macho::MachOParseError> for AppError {
    fn from(e: crate::macho::MachOParseError) -> Self {
        Self::MachOError(e)
    }
}

impl From<crate::macho::DigestError> for AppError {
    fn from(e: crate::macho::DigestError) -> Self {
        Self::Digest(e)
    }
}

fn get_macho_from_data(data: &[u8], universal_index: usize) -> Result<MachO, AppError> {
    let mach = Mach::parse(data)?;

    match mach {
        Mach::Binary(macho) => Ok(macho),
        Mach::Fat(multiarch) => {
            eprintln!(
                "found fat/universal Mach-O binary with {} architectures; examining binary at index {}",
                multiarch.narches, universal_index
            );

            Ok(multiarch.get(universal_index)?)
        }
    }
}

fn command_extract(args: &ArgMatches) -> Result<(), AppError> {
    let path = args.value_of("path").ok_or(AppError::BadArgument)?;
    let format = args.value_of("data").ok_or(AppError::BadArgument)?;
    let index = args.value_of("universal_index").unwrap();
    let index = usize::from_str(index).map_err(|_| AppError::BadArgument)?;

    let data = std::fs::read(path)?;

    let macho = get_macho_from_data(&data, index)?;

    let sig = find_signature_data(&macho)?.ok_or(AppError::NoCodeSignature)?;

    match format {
        "blobs" => {
            let embedded = parse_signature_data(&sig.signature_data)?;

            for blob in embedded.blobs {
                let parsed = blob.into_parsed_blob()?;
                println!("{:#?}", parsed);
            }
        }
        "cms-ber" => {
            let embedded = parse_signature_data(&sig.signature_data)?;
            if let Some(cms) = embedded.signature_data()? {
                std::io::stdout().write_all(cms)?;
            } else {
                eprintln!("no CMS data");
            }
        }
        "cms-pem" => {
            let embedded = parse_signature_data(&sig.signature_data)?;
            if let Some(cms) = embedded.signature_data()? {
                print!(
                    "{}",
                    pem::encode(&pem::Pem {
                        tag: "PKCS7".to_string(),
                        contents: cms.to_vec(),
                    })
                );
            } else {
                eprintln!("no CMS data");
            }
        }
        "code-directory" => {
            let embedded = parse_signature_data(&sig.signature_data)?;

            if let Some(cd) = embedded.code_directory()? {
                println!("{:#?}", cd);
            } else {
                eprintln!("no code directory");
            }
        }
        "linkedit-segment-raw" => {
            std::io::stdout().write_all(sig.linkedit_segment_data)?;
        }
        "requirements" => {
            let embedded = parse_signature_data(&sig.signature_data)?;

            if let Some(reqs) = embedded.requirements()? {
                println!("{:#?}", reqs)
            } else {
                eprintln!("no requirements");
            }
        }
        "segment-info" => {
            println!("segments count: {}", sig.segments_count);
            println!("__LINKEDIT segment index: {}", sig.linkedit_segment_index);
            println!(
                "__LINKEDIT segment size: {}",
                sig.linkedit_segment_data.len()
            );
            println!(
                "__LINKEDIT signature start offset: {}",
                sig.signature_start_offset
            );
            println!(
                "__LINKEDIT signature end offset: {}",
                sig.signature_end_offset
            );
            println!("__LINKEDIT signature size: {}", sig.signature_data.len());
        }
        "superblob" => {
            let embedded = parse_signature_data(&sig.signature_data)?;

            println!("length: {}", embedded.length);
            println!("blob count: {}", embedded.count);
            println!("blobs:");
            for blob in embedded.blobs {
                println!("- index: {}", blob.index);
                println!("  offset: {}", blob.offset);
                println!("  length: {}", blob.length);
                println!("  end offset: {}", blob.offset + blob.length - 1);
                println!("  magic: {:?}", blob.magic);
                println!("  sha1: {}", hex::encode(blob.digest_with(HashType::Sha1)?));
                println!(
                    "  sha256: {}",
                    hex::encode(blob.digest_with(HashType::Sha256)?)
                );
                println!(
                    "  sha384: {}",
                    hex::encode(blob.digest_with(HashType::Sha384)?)
                );
            }
        }
        _ => panic!("unhandled format: {}", format),
    }

    Ok(())
}

fn main_impl() -> Result<(), AppError> {
    let matches = App::new("Oxidized Apple Codesigning")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Gregory Szorc <gregory.szorc@gmail.com>")
        .about("Do things related to code signing of Apple binaries")
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts code signature data from a Mach-O binary")
                .long_about(EXTRACT_ABOUT)
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .help("Path to Mach-O binary to examine"),
                )
                .arg(
                    Arg::with_name("data")
                        .long("data")
                        .takes_value(true)
                        .possible_values(&[
                            "blobs",
                            "cms-ber",
                            "cms-pem",
                            "code-directory",
                            "linkedit-segment-raw",
                            "requirements",
                            "segment-info",
                            "superblob",
                        ])
                        .default_value("segment-info")
                        .help("Which data to extract and how to format it"),
                )
                .arg(
                    Arg::with_name("universal_index")
                        .long("universal-index")
                        .takes_value(true)
                        .default_value("0")
                        .help("Index of Mach-O binary to operate on within a universal/fat binary"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("extract", Some(args)) => command_extract(args),
        _ => Err(AppError::UnknownCommand),
    }
}

fn main() {
    let exit_code = match main_impl() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    };

    std::process::exit(exit_code)
}
