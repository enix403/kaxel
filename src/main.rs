#![allow(unused)]
#![allow(dead_code)]

pub mod emit;
pub mod values;
pub mod spec;

use anyhow::Result;
use emit::Emitter;
use spec::{Api, ApiProfile, ApiVersion, SpecOptions};
use std::path::Path;
use std::fs::File;
use std::io::stdout;

fn main() -> Result<()> {
    /**
     * TEMPORARY
     * TODO: replace with user provided path
     * */
    let file_path = "./refs/assets/latest_gl.xml";
    let file = File::open(file_path)?;

    let spec = spec::build_spec(file, SpecOptions{ 
        api: spec::Api::OpenGL,
        version: ApiVersion("3.3".into()),
        profile: ApiProfile::Core
    })?;

    emit::CppEmitter {
        outdir: Path::new("./output")
    }.emit(&spec);

    Ok(())
}