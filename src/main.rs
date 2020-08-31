mod compiler;
mod cythanenc;
mod decompiler;
mod execute;

use clap::*;

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("compile")
            .about("Compile a Cythan file | use .cct extension for Compressed Executable or .rct for raw format")
            .version("1.0")
            .author("Laurent Gaucheron <gaucheron.laurent@gmail.com>")
            .arg(Arg::with_name("INPUT")
                .required(true)
                .help("Set input file"))
            .arg(Arg::with_name("OUTPUT")
                .help("Set output file")))
        .subcommand(SubCommand::with_name("decompile")
            .about("Decompile a Cythan file | This require a compressed cythan file in .cct to work")
            .version("1.0")
            .author("Laurent Gaucheron <gaucheron.laurent@gmail.com>")
            .arg(Arg::with_name("INPUT")
                .required(true)
                .help("Set input file"))
            .arg(Arg::with_name("OUTPUT")
                .help("Set output file")))
        .subcommand(SubCommand::with_name("execute")
            .about("Execute compiled Cythan file | Support raw and compressed format (.rct and .cct)")
            .version("1.0")
            .author("Laurent Gaucheron <gaucheron.laurent@gmail.com>")
            .arg(Arg::with_name("INPUT")
                .required(true)
                .help("Set input file"))
            .arg(Arg::with_name("ITERATIONS")
                .required(true)
                .help("Number of iterations to compute"))
            .arg(Arg::with_name("OUTPUT")
                .help("Set output file if none will print to the console the result")))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("compile") {
        let output = matches.value_of("OUTPUT").unwrap_or("compiled.cct");
        let input = matches.value_of("INPUT").unwrap();
        let instant = std::time::Instant::now();
        compiler::compile(input, output).unwrap();
        println!(
            "Compiled `{}` to `{}` in {}",
            input,
            output,
            ms_to_str(instant.elapsed().as_millis())
        );
    } else if let Some(matches) = matches.subcommand_matches("execute") {
        let output = matches.value_of("OUTPUT");
        let input = matches.value_of("INPUT").unwrap();
        let iterations: usize = matches
            .value_of("ITERATIONS")
            .map(|x| x.parse().ok())
            .flatten()
            .expect("<ITERATIONS> must be a positive integer");

        let instant = std::time::Instant::now();
        execute::execute(input, iterations, output.map(|x| x.to_owned())).unwrap();
        if let Some(e) = output {
            println!(
                "Executed `{}` and written the result to `{}` in {}",
                input,
                e,
                ms_to_str(instant.elapsed().as_millis())
            );
        } else {
            println!(
                "Executed `{}` in {}",
                input,
                ms_to_str(instant.elapsed().as_millis())
            );
        }
    } else if let Some(matches) = matches.subcommand_matches("decompile") {
        let output = matches.value_of("OUTPUT").unwrap_or("decompiled.rct");
        let input = matches.value_of("INPUT").unwrap();

        let instant = std::time::Instant::now();
        decompiler::decompile(input, output);
        println!(
            "Decompiled `{}` to `{}` in {}",
            input,
            output,
            ms_to_str(instant.elapsed().as_millis())
        );
    } else {
        println!("Invalid use: cythan-cli compile <FILE> [OUTPUT]")
    }
}

fn ms_to_str(ms: u128) -> String {
    if ms < 100 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        format!("{}s", (ms / 100) as f32 / 10.0)
    } else {
        format!(
            "{}m {}s",
            (ms / 60_000) as f32,
            ((ms % 60_000) / 100) as f32 / 10.0
        )
    }
}
