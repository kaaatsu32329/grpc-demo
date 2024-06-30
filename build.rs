fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure().compile(&["proto/**.proto"], &["proto"])?;
    Ok(())
}
