use super::{
    keywords::{BoundsKeywords, ConstraintsKeywords, EndKeywords, VariableType},
    sections::{Section, SectionsHolder},
    util::{chunks, is_comment, is_end},
};
use crate::{core::Model, errors::TranslationErr};
use crate::{
    core::Sense,
    translator::base::{BackTranslator, Translator},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

static MAX_LINE_LENGTH: usize = 88;
static INDENT: &str = " ";

pub struct LPTranslator
where
    Self: Translator;

impl LPTranslator {
    pub fn read_file(filepath: PathBuf) -> Result<String, TranslationErr> {
        let display = filepath.display();
        let mut file = match File::open(&filepath) {
            Err(why) => {
                return Err(TranslationErr::new(format!(
                    "couldn't open {}: {}",
                    display, why
                )))
            }
            Ok(file) => file,
        };
        let mut s = String::new();
        file.read_to_string(&mut s)
            .map_err(|why| TranslationErr::new(format!("couldn't read {}: {}", display, why)))?;
        Ok(s)
    }

    fn write_file(data: String, filepath: &PathBuf) -> Result<(), TranslationErr> {
        let mut file =
            File::create(filepath).map_err(|why| TranslationErr::new(why.to_string()))?;
        file.write_all(data.as_bytes())
            .map_err(|why| TranslationErr::new(why.to_string()))?;
        Ok(())
    }

    fn parse_sections(contents: String) -> Result<SectionsHolder, TranslationErr> {
        let mut sections: SectionsHolder = SectionsHolder::new();
        let mut last_section = Section::Placeholder;
        for (_i, line) in contents.lines().enumerate() {
            // println!("{}: {}", i, line);
            if is_comment(line) {
                // Check if the Comment contains "Model" and use the rest after as the model name.
                if line.contains("Model") {
                    let name = line.split_once("Model").unwrap().1.trim();
                    sections.model_name = Some(name.to_string());
                }
                // Check if the Comment contains "Problem name:" and use the rest after as the model name.
                else if line.contains("Problem name:") {
                    let name = line.split_once("Problem name:").unwrap().1.trim();
                    sections.model_name = Some(name.to_string());
                }
                // Skip empty or commented lines.
                continue;
            }
            if is_end(line) {
                // Excape after `End` keyword is reached.
                break;
            }
            match Section::detect(line) {
                // Header Section
                (Some(sec), None) => {
                    sections.put(&sec);
                    last_section = sec;
                }
                // Content Section
                (None, Some(content)) => match &last_section {
                    Section::Objective(Sense::Min) | Section::Objective(Sense::Max) => {
                        if let Some((_name, rest)) = content.split_once(":") {
                            // sections.model_name = Some(name.to_string());
                            sections.push(&last_section, rest.to_string())
                        } else {
                            sections.push(&last_section, content.to_string())
                        }
                    }
                    _ => sections.push(&last_section, content.to_string()),
                },
                _ => {
                    return Err(TranslationErr::new(String::from(
                        "unknown section detected",
                    )))
                }
            }
        }
        // println!("{:#?}", sections);
        Ok(sections)
    }

    fn build_model(sections: SectionsHolder) -> Result<Model, TranslationErr> {
        let model_name = &sections.model_name;
        let mut model = Model::new(model_name.clone(), None);
        // ATTENTION: The sense will be set correctly in the `.make_objective` call.
        let vl = sections.make_variables(&mut model)?;
        sections.make_objective(&mut model, &vl)?;
        sections.make_constraints(&mut model, &vl)?;
        Ok(model)
    }

    fn build_string(model: &Model) -> Result<String, TranslationErr> {
        let sections = SectionsHolder::from_model(&model)?;
        let mut out = String::new();

        out.push_str(&format!("\\ Model {}\n", model.name));
        out.push_str(&format!("\\ Problem name: {}\n", model.name));
        out.push_str("\n");
        let (keyword, data) = sections.get_objective_str()?;
        out.push_str(&format!("{keyword}\n"));
        // the obj: prefix
        out.push_str(&format!("{INDENT}obj:"));
        for row in data.iter() {
            let chunks = chunks(row, MAX_LINE_LENGTH);
            for chunk in chunks {
                out.push_str(&format!("{INDENT}{chunk}\n"));
            }
        }
        if let Some(data) = sections.get(Section::Constraints) {
            out.push_str(&format!("{}\n", ConstraintsKeywords::SubjectTo));
            for constraint in data {
                out.push_str(&format!("{INDENT}{constraint}\n"));
                // let chunks = chunks(constraint, MAX_LINE_LENGTH);
                // for chunk in chunks {
                //     out.push_str(&format!("{INDENT}{chunk}\n"));
                // }
            }
        }
        out.push_str(&format!("{}\n", BoundsKeywords::Bounds));
        if let Some(data) = sections.get(Section::Bounds) {
            for bound in data {
                let chunks = chunks(bound, MAX_LINE_LENGTH);
                for chunk in chunks {
                    out.push_str(&format!("{INDENT}{chunk}\n"));
                }
            }
        }
        for (vt, data) in sections.iter_variables() {
            if *vt == VariableType::Continuous {
                continue;
            }
            out.push_str(&format!("{}\n", vt.to_string()));
            let data_str = data.join(" ");
            let chunks = chunks(&data_str, MAX_LINE_LENGTH);
            for chunk in chunks {
                out.push_str(&format!("{INDENT}{chunk}\n"));
            }
        }
        out.push_str(&EndKeywords::End.to_string());
        Ok(out)
    }
}

impl Translator for LPTranslator {
    type TranslateIn = String;
    type TranslateOut = Result<Model, TranslationErr>;

    fn translate(file: Self::TranslateIn) -> Self::TranslateOut {
        Self::build_model(Self::parse_sections(file)?)
    }
}

impl<'a> BackTranslator<'a> for LPTranslator {
    type BackTranslateIn = (&'a Model, Option<PathBuf>);
    type BackTranslateOut = Result<Option<String>, TranslationErr>;

    fn back_translate(data: Self::BackTranslateIn) -> Self::BackTranslateOut {
        let (model, pathbuf) = data;
        let lpstr = Self::build_string(model)?;
        if let Some(pb) = pathbuf {
            Self::write_file(lpstr, &pb)?;
            Ok(None)
        } else {
            Ok(Some(lpstr))
        }
    }
}
