// #![allow(unused)]
// #![allow(dead_code)]

pub mod emit;
pub mod spec;
pub mod values;

use anyhow::Result;
use emit::Emitter;
use spec::{ApiProfile, ApiVersion, SpecOptions};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    /*
     * TEMPORARY
     * TODO: replace with user provided path
     * */
    let file_path = "./refs/api_specs_latest/gl.xml";
    let file = File::open(file_path).expect("Failed to open gl.xml file");

    let spec = spec::build_spec(
        file,
        SpecOptions {
            api: spec::Api::OpenGL,
            version: ApiVersion("3.3".into()),
            profile: ApiProfile::Core,
        },
    )?;

    emit::CppEmitter {
        outdir: Path::new("./output"),
    }
    .emit(&spec)?;

    Ok(())
}
