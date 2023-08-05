use anyhow::{anyhow, bail, ensure, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::{EventReader, ParserConfig};

use crate::values;

#[derive(Default)]
pub struct Spec {
    pub enum_groups: HashMap<String, Vec<EnumerantId>>,
    pub enums_list: Vec<Enumerant>,
}

pub enum Api {
    OpenGL,
    OpenGLES,
    OpenGLSC,
}

pub struct ApiVersion(pub String);

pub enum ApiProfile {
    Core,
    Compatibility,
}

pub struct SpecOptions {
    pub api: Api,
    pub version: ApiVersion,
    pub profile: ApiProfile,
}

pub fn build_spec(file: File, opts: SpecOptions) -> Result<Spec> {
    let file = BufReader::new(file);
    let reader = EventReader::new_with_config(
        file,
        ParserConfig::new() /* -- */
            .ignore_comments(true),
    );

    let mut spec = Spec::default();

    SpecParse {
        reader,
        opts,
        spec: &mut spec,
    }
    .fill_elements()?;

    Ok(spec)
}

/* =========== */

#[derive(Clone)]
pub struct EnumerantId(pub String);

pub struct Enumerant {
    pub name: EnumerantId,
    pub value: values::Constant,
    pub alias: Option<String>,
}

pub struct EnumerantGroup {
    pub name: Option<String>, /* None for an unnamed group */
    pub enums: Vec<EnumerantId>,
}

struct SpecParse<'a, R: Read> {
    reader: EventReader<R>,
    opts: SpecOptions,
    spec: &'a mut Spec,
}

/* =========== */

impl<'a, R: Read> SpecParse<'a, R> {
    fn fill_elements(&mut self) -> Result<()> {
        // Sanity check: Make sure we are given an un-consumed reader
        ensure!(
            matches!(self.reader.next(), Ok(XmlEvent::StartDocument { .. })),
            "Expected StartDocument"
        );

        // Look for any <registry> root tags, ignoring other things (like comments)
        // along the way
        loop {
            match self.reader.next()? {
                XmlEvent::EndDocument { .. } => bail!("Expected <registry>"),
                XmlEvent::StartElement { name, .. } if match_name(&name, "registry") => break,
                _ => (),
            }
        }

        // ...
        // Here the parser is now inside the root <registry> tag
        // ...

        loop {
            match self.reader.next()? {
                XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                    "enums" => self.elem_enums()?,
                    _ => (),
                },
                XmlEvent::EndDocument => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn api_check(&self, attributes: &Vec<OwnedAttribute>) -> bool {
        // Skip if the element is useless i.e is of a different api
        // If the "api" attibute is present, then we match it with opts.api,
        // otherwise it is implcitly valid for any api.

        getattr(&attributes, "api")
            .map(|api| match (&api.value[..], &self.opts.api) {
                ("", _) => true,
                ("gl", Api::OpenGL) => true,
                ("gles1" | "gles2", Api::OpenGLES) => true,
                ("glsc2", Api::OpenGLSC) => true,
                _ => false,
            })
            .unwrap_or(true)
    }

    fn elem_enums(&mut self) -> Result<()> {
        loop {
            match self.reader.next()? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } if match_name(&name, "enum") => {
                    if !self.api_check(&attributes) {
                        continue;
                    }

                    let name = getattr(&attributes, "name")
                        .map(|v| EnumerantId(v.value.clone()))
                        .ok_or(anyhow!("<enum />: Attribute \"name\" not found."))?;

                    let value = getattr(&attributes, "value")
                        .ok_or(anyhow!("<enum />: Attribute \"value\" not found."))
                        .map(|v| values::make_constant(&v.value))?;

                    let alias = getattr(&attributes, "alias").map(|v| &v.value).cloned();
                    // let enum_ty = getattr(&attributes, "type").map(|v| &v.value).cloned(); // Not needed now

                    let enum_id = name.clone();

                    // Add it to the list
                    self.spec.enums_list.push(Enumerant {
                        name,
                        value,
                        alias,
                        // ty: enum_ty,
                    });

                    if let Some(group_value) = getattr(&attributes, "group").map(|v| &v.value[..]) {
                        let group_list: Vec<&str> = group_value.split(',').map(str::trim).collect();

                        for group in group_list.into_iter() {
                            self.spec
                                .enum_groups
                                .entry(group.into())
                                .or_insert(vec![])
                                .push(enum_id.clone())
                        }
                    };
                }
                XmlEvent::EndElement { name } if match_name(&name, "enums") => return Ok(()),
                _ => (),
            }
        }
    }
}

fn match_name(name: &OwnedName, target: &str) -> bool {
    return name.local_name.as_str() == target;
}

fn getattr<'a>(attributes: &'a Vec<OwnedAttribute>, target: &str) -> Option<&'a OwnedAttribute> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == target)
}
