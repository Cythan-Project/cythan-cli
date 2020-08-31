pub fn compile(file: &str, out: &str) -> anyhow::Result<()> {
    let vec = cythan_compiler::Context::new().compute(&cythan_compiler::generate_tokens(
        &std::fs::read_to_string(file)?,
    ));
    super::cythanenc::write_file_and_compress(out, &vec)
}
