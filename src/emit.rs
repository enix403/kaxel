use crate::spec::Spec;
use anyhow::Result;
use std::format;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
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
            fs::create_dir_all(&root_folder).unwrap_or_else(|e| {
                panic!(
                    "Failed to create missing output directory \"{:?}\": {}",
                    root_folder.as_path(),
                    e
                )
            });
        }

        let mut header_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(root_folder.join("gl.h"))
            .expect("Failed to open header file \"gl.h\"");

        writeln!(&mut header_file, "#include <cstdint>\n").expect("write!() failed");

        self.emit_types(&mut header_file);
        self.emit_enums(spec, &mut header_file)?;

        Ok(())
    }
}

impl<'a> CppEmitter<'a> {
    fn emit_types<W: Write>(&self, writer: &mut W) {
        writeln!(
            writer,
            r#"
#if defined(_WIN64)
typedef signed long long int khronos_ssize_t;
typedef unsigned long long int khronos_usize_t;
#else
typedef signed long int khronos_ssize_t;
typedef unsigned long int khronos_usize_t;
#endif

typedef unsigned int GLenum;
typedef unsigned char GLboolean;
typedef unsigned int GLbitfield;
typedef void GLvoid;
typedef int8_t GLbyte;
typedef uint8_t GLubyte;
typedef int16_t GLshort;
typedef uint16_t GLushort;
typedef int GLint;
typedef unsigned int GLuint;
typedef int32_t GLclampx;
typedef int GLsizei;
typedef float GLfloat;
typedef float GLclampf;
typedef double GLdouble;
typedef double GLclampd;
typedef void *GLeglClientBufferEXT;
typedef void *GLeglImageOES;
typedef char GLchar;
typedef char GLcharARB;
#ifdef __APPLE__
typedef void *GLhandleARB;
#else
typedef unsigned int GLhandleARB;
#endif
typedef uint16_t GLhalf;
typedef uint16_t GLhalfARB;
typedef int32_t GLfixed;
typedef intptr_t GLintptr;
typedef intptr_t GLintptrARB;
typedef khronos_ssize_t GLsizeiptr;
typedef khronos_ssize_t GLsizeiptrARB;
typedef int64_t GLint64;
typedef int64_t GLint64EXT;
typedef uint64_t GLuint64;
typedef uint64_t GLuint64EXT;

typedef struct __GLsync *GLsync;

struct _cl_context;
struct _cl_event;

typedef void (*GLDEBUGPROC)(
    GLenum source,
    GLenum type,
    GLuint id,
    GLenum severity,
    GLsizei length,
    const GLchar *message,
    const void *userParam
);
typedef void (*GLDEBUGPROCARB)(
    GLenum source,
    GLenum type,
    GLuint id,
    GLenum severity,
    GLsizei length,
    const GLchar *message,
    const void *userParam
);
typedef void (*GLDEBUGPROCKHR)(
    GLenum source,
    GLenum type,
    GLuint id,
    GLenum severity,
    GLsizei length,
    const GLchar *message,
    const void *userParam
);
typedef void (*GLDEBUGPROCAMD)(
    GLuint id,
    GLenum category,
    GLenum severity,
    GLsizei length,
    const GLchar *message,
    void *userParam
);

typedef unsigned short GLhalfNV;
typedef GLintptr GLvdpauSurfaceNV;

typedef void (*GLVULKANPROCNV)(void);
            "#
        )
        .expect("write!() failed");
    }

    fn emit_enums<W: Write>(&self, spec: &Spec, writer: &mut W) -> Result<()> {
        for enumerant in spec.enums_list.iter() {
            let cnt = &enumerant.value;

            let datatype = match (cnt.ty.signed, cnt.ty.bitwidth) {
                (false, 8) => "uint8_t",
                (false, 32) => "uint32_t",
                (false, 64) => "uint64_t",

                (true, 8) => "int8_t",
                (true, 32) => "int32_t",
                (true, 64) => "int64_t",

                _ => "auto",
            };

            let value = if cnt.ty.signed {
                format!("-{}", cnt.value)
            } else {
                format!("0x{:X}", cnt.value)
            };

            writeln!(
                writer,
                "constexpr {} {} = {};",
                datatype, enumerant.name.0, value
            )?;
        }

        Ok(())
    }
}
