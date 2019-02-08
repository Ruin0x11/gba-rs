use args::Args;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process,
};

const PROGRAM_NAME: &'static str = "builder";
const PROGRAM_DESC: &'static str = "Builds the ROM image.";

fn main() {
    let mut args = args();

    if help_arg_present() {
        println!("{}", args.full_usage());
        process::exit(0);
    }

    if let Err(args_err) = args.parse_from_cli() {
        writeln!(io::stderr(), "{}", args_err).expect("Failed to write to stderr");
        process::exit(1);
    };

    // build ROM

    let mut build_args = vec![
        "--manifest-path".into(),
        "../Cargo.toml".into(),
        "--target".into(),
        "../thumbv7-gba-cart.json".into(),
        "--release".into(),
    ];
    if args.value_of("no-default-features").unwrap() {
        build_args.push("--no-default-features".into());
    }
    if args.value_of("all-features").unwrap() {
        build_args.push("--all-features".into());
    }
    if let Some(features) = args.optional_value_of("features").unwrap() {
        build_args.push("--features".into());
        build_args.push(features);
    }

    println!("Running xbuild.");

    let exit_status = run_xbuild(&build_args);
    if !exit_status.map(|s| s.success()).unwrap_or(false) {
        process::exit(1)
    }

    let bootloader_elf_path = Path::new("../target/thumbv7-gba-cart/release/gba");
    let mut bootloader_elf_bytes = Vec::new();
    File::open(bootloader_elf_path)
        .and_then(|mut f| f.read_to_end(&mut bootloader_elf_bytes))
        .expect("failed to read bootloader ELF file");

    // read bootloader section of ELF file

    let elf_file = xmas_elf::ElfFile::new(&bootloader_elf_bytes).unwrap();
    xmas_elf::header::sanity_check(&elf_file).unwrap();

    let init_section = elf_file
        .find_section_by_name(".init")
        .expect("ELF must have .init section");
    let init_offset = init_section.offset() as usize + 0xc0; // add header offset
    let program_bytes = &bootloader_elf_bytes[init_offset..];

    // create output file

    let output_file_path = Path::new("../target/thumbv7-gba-cart/release/image.gba");

    println!("Writing output to {}.", output_file_path.display());

    let mut output_file = File::create(output_file_path).expect("Failed to create output file");
    write_header(&mut output_file).expect("Failed to write header");
    output_file
        .write_all(program_bytes)
        .expect("Failed to write program bytes to output file");
}

fn args() -> Args {
    use getopts::Occur;

    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Prints the help message");
    args.option(
        "",
        "features",
        "Space-separated list of features to activate",
        "FEATURES",
        Occur::Optional,
        None,
    );
    args.flag("", "all-features", "Activate all available features");
    args.flag(
        "",
        "no-default-features",
        "Do not activate the `default` feature",
    );
    args
}

fn help_arg_present() -> bool {
    std::env::args()
        .find(|a| a == "--help" || a == "-h")
        .is_some()
}

fn run_xbuild(args: &[String]) -> io::Result<process::ExitStatus> {
    let mut command = process::Command::new("cargo");
    command.arg("xbuild");
    command.args(args);
    let exit_status = command.status()?;

    if !exit_status.success() {
        let mut help_command = process::Command::new("cargo");
        help_command.arg("xbuild").arg("--help");
        help_command.stdout(process::Stdio::null());
        help_command.stderr(process::Stdio::null());
        if let Ok(help_exit_status) = help_command.status() {
            if !help_exit_status.success() {
                let mut stderr = io::stderr();
                writeln!(
                    stderr,
                    "Failed to run `cargo xbuild`. Perhaps it is not installed?"
                )?;
                writeln!(stderr, "Run `cargo install cargo-xbuild` to install it.")?;
            }
        }
    }

    Ok(exit_status)
}

fn write_header(output: &mut File) -> io::Result<()> {
    let header_bytes = include_bytes!("../header.bin");
    output.write_all(header_bytes)
}
