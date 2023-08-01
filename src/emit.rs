use anyhow::Result;

use crate::spec::Spec;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::write;
use std::writeln;

pub trait Emitter {
    fn emit(&self, spec: &Spec) -> Result<()>;
}

pub struct CppEmitter<'a> {
    pub outdir: &'a Path,
}

impl<'a> Emitter for CppEmitter<'a> {
    fn emit(&self, spec: &Spec) -> Result<()> {
        if !self.outdir.is_dir() {
            fs::create_dir_all(self.outdir);
        }

        let mut header_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.outdir.join("gl.h"))
            .expect("Failed to open header file \"gl.h\"");

        for enumerant in spec.enums_list.iter() {
            writeln!(
                &mut header_file,
                "constexpr auto {} = 0x{:X};",
                enumerant.name.0, enumerant.value
            )?;
        }

        Ok(())
    }
}
