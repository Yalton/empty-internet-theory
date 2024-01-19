use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let proto_dir = PathBuf::from("./protocol/");

    let builder = tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true);

    let eit_proto = proto_dir.join("./eit.v1.proto");
    builder.compile(&[eit_proto], &[proto_dir])?;

    Ok(())
}
