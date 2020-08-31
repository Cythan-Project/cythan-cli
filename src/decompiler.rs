pub fn decompile(file: &str, out: &str) -> anyhow::Result<()> {
    let i = super::cythanenc::read_file_and_decompress(file)?;

    std::fs::write(
        out,
        &i.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "),
    )?;
    Ok(())
}
