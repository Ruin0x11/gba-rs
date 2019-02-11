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

    let mut target = "gba".into();
    let mut config: &str = "debug".into();

    let mut build_args = vec![
        "--manifest-path".into(),
        "../Cargo.toml".into(),
        "--target".into(),
        "../thumbv4t-gba-cart.json".into(),
        "-vv".into(),
    ];
    if args.value_of("no-default-features").unwrap() {
        build_args.push("--no-default-features".into());
    }
    if args.value_of("all-features").unwrap() {
        build_args.push("--all-features".into());
    }
    if args.value_of("release").unwrap() {
        build_args.push("--release".into());
        config = "release".into();
    }
    if let Some(features) = args.optional_value_of("features").unwrap() {
        build_args.push("--features".into());
        build_args.push(features);
    }
    if let Some(example) = args.optional_value_of("example").unwrap() {
        target = format!("examples/{}", example);
        build_args.push("-p".into());
        build_args.push("gba-examples".into());
        build_args.push("--example".into());
        build_args.push(example);
    }

    // Build target with cargo-xbuild.

    println!("Running xbuild for target '{}'.", target);

    let exit_status = run_xbuild(&build_args);
    if !exit_status.map(|s| s.success()).unwrap_or(false) {
        process::exit(1)
    }

    // Strip header section of ELF and extract program data

    let elf_target = format!("../target/thumbv4t-gba-cart/{}/{}", config, target);
    let elf_path = Path::new(&elf_target);
    let mut elf_bytes = Vec::new();
    File::open(elf_path)
        .and_then(|mut f| f.read_to_end(&mut elf_bytes))
        .expect("failed to read bootloader ELF file");

    let elf_file = xmas_elf::ElfFile::new(&elf_bytes).unwrap();
    xmas_elf::header::sanity_check(&elf_file).unwrap();

    let init_section = elf_file
        .find_section_by_name(".init")
        .expect("ELF must have .init section");
    let init_offset = init_section.offset() as usize + 0xc0; // add header offset
    let program_bytes = &elf_bytes[init_offset..];

    // Create output file by writing GBA header, then program data.

    let output_target = format!("../target/thumbv4t-gba-cart/{}/{}.gba", config, target);
    let output_file_path = Path::new(&output_target);

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
    args.option(
        "",
        "example",
        "Example to run",
        "EXAMPLE",
        Occur::Optional,
        None,
    );
    args.flag("", "all-features", "Activate all available features");
    args.flag("", "release", "Build for release");
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
