use cythan::{BasicCythan, Cythan};

pub fn execute(file: &str, iterations: usize, outfile: Option<String>) -> anyhow::Result<()> {
    let mut vec = BasicCythan::new(
        super::cythanenc::read_file_and_decompress(file)?
            .into_iter()
            .map(|x| x as usize)
            .collect(),
    );
    for _ in 0..iterations {
        vec.next();
    }
    if let Some(e) = outfile {
        super::cythanenc::write_file_and_compress(
            &e,
            &vec.cases.into_iter().map(|x| x as u32).collect(),
        )?;
    } else {
        println!("{:?}", vec.cases);
    };
    Ok(())
}
