use crate::num_parse::Constant;
use crate::spec::Spec;
use anyhow::Result;
use std::fs::{self, File, OpenOptions};
use std::format;
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
        let root_folder = self.outdir.join("kaxel");

        if !root_folder.is_dir() {
            fs::create_dir_all(&root_folder);
        }

        let mut header_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(root_folder.join("gl.h"))
            .expect("Failed to open header file \"gl.h\"");

        writeln!(&mut header_file, "#include <cstdint>\n");

        self.emit_enums(spec, &mut header_file)?;

        Ok(())
    }
}

impl<'a> CppEmitter<'a> {
    fn emit_enums<W: Write>(&self, spec: &Spec, writer: &mut W) -> Result<()> {
        for enumerant in spec.enums_list.iter() {
            let cnt = &enumerant.value;

            let datatype = match (cnt.signed, cnt.bitwidth) {
                (false, 8) => "uint8_t",
                (false, 32) => "uint32_t",
                (false, 64) => "uint64_t",

                (true, 8) => "int8_t",
                (true, 32) => "int32_t",
                (true, 64) => "int64_t",

                _ => "auto",
            };

            let value = if cnt.signed {
                format!("-{}", cnt.value)
            } else {
                format!("0x{:X}", cnt.value)
            };

            writeln!(
                writer,
                "constexpr {} {} = {};",
                datatype,
                enumerant.name.0,
                value
            )?;
        }

        Ok(())
    }
}
