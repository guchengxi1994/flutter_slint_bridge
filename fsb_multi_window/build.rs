fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/runapp.proto")?;
    slint_build::compile("src/ui/tip.slint")?;

    Ok(())
}
