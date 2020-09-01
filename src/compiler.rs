pub fn compile(file: &str, out: &str) -> anyhow::Result<()> {
    let vec = match cythan_compiler::Context::default().compute(
        &match cythan_compiler::generate_tokens(&std::fs::read_to_string(file)?) {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                panic!("");
            }
        },
    ) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            panic!("");
        }
    };
    super::cythanenc::write_file_and_compress(out, &vec)
}
