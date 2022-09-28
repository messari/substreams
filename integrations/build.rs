fn main() {}

// NOTE: The follow method is currently not used, but is kept here
// for future reference in case we want to use the substreams grpc
// blocks endpoint directly, instead of the substreams cli.
//
//
// fn download_and_compile_substream_v1() {
//     use std::fs;
//     use std::path::Path;
//     const SUBSTREAMS_GITHUB_REV: &str = "https://raw.githubusercontent.com/streamingfast/substreams/73ec39f882638be99960b59feab8e23f2ea76c50";
//     const SUBSTREAMS_PROTO_SRC: &str = "/proto/sf/substreams/v1/substreams.proto";
//     const CLOCK_PROTO_SRC: &str = "/proto/sf/substreams/v1/clock.proto";
//     const MODULE_PROTO_SRC: &str = "/proto/sf/substreams/v1/modules.proto";

//     const DEFAULT_TARGET_DIR: &str = "./target/sf/substreams/v1";

//     let out_dir = Path::new(DEFAULT_TARGET_DIR);

//     fs::create_dir_all(out_dir).ok();

//     let substreams_proto = out_dir.join("substreams.proto");
//     let clock_proto = out_dir.join("clock.proto");
//     let module_proto = out_dir.join("modules.proto");

//     let substreams_proto_url = format!("{}/{}", SUBSTREAMS_GITHUB_REV, SUBSTREAMS_PROTO_SRC);
//     let clock_proto_url = format!("{}/{}", SUBSTREAMS_GITHUB_REV, CLOCK_PROTO_SRC);
//     let module_proto_url = format!("{}/{}", SUBSTREAMS_GITHUB_REV, MODULE_PROTO_SRC);

//     let mut substreams_proto_file = fs::File::create(&substreams_proto).unwrap();
//     let mut clock_proto_file = fs::File::create(&clock_proto).unwrap();
//     let mut module_proto_file = fs::File::create(&module_proto).unwrap();

//     reqwest::blocking::get(&substreams_proto_url)
//         .unwrap()
//         .copy_to(&mut substreams_proto_file)
//         .unwrap();
//     reqwest::blocking::get(&clock_proto_url)
//         .unwrap()
//         .copy_to(&mut clock_proto_file)
//         .unwrap();
//     reqwest::blocking::get(&module_proto_url)
//         .unwrap()
//         .copy_to(&mut module_proto_file)
//         .unwrap();

//     tonic_build::configure()
//         .build_client(true)
//         .build_server(true)
//         .out_dir(out_dir)
//         .compile(
//             &[
//                 substreams_proto.as_path(),
//                 clock_proto.as_path(),
//                 module_proto.as_path(),
//             ],
//             &["./target"],
//         )
//         .unwrap();
// }
