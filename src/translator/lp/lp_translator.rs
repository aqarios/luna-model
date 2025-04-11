use super::{
    sections::{Section, SectionsHolder},
    util::{is_comment, is_end},
};
use crate::translator::base::{BackTranslator, Translator};
use crate::{
    core::{
        expression::{BiasConstraints, IndexConstraints},
        Model,
    },
    errors::TranslationErr,
};
use std::{fs::File, io::Read, marker::PhantomData, path::PathBuf};

pub struct LPTranslator<Index, Bias> {
    _phantom_index: PhantomData<Index>,
    _phantom_bias: PhantomData<Bias>,
}

impl<Index, Bias> LPTranslator<Index, Bias>
where
    Self: Translator,
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn read_file(filepath: PathBuf) -> Result<String, TranslationErr> {
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

    fn parse_sections(contents: String) -> Result<SectionsHolder<Index, Bias>, TranslationErr> {
        let mut sections: SectionsHolder<Index, Bias> = SectionsHolder::new();
        let mut last_section = Section::Placeholder;
        for line in contents.lines() {
            if is_comment(line) {
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
                (None, Some(content)) => {
                    sections.push(&last_section, content.to_string());
                }
                _ => {
                    return Err(TranslationErr::new(String::from(
                        "unknown section detected",
                    )))
                }
            }
        }
        Ok(sections)
    }

    fn build_model(
        sections: SectionsHolder<Index, Bias>,
    ) -> Result<Model<Index, Bias>, TranslationErr> {
        let model_name = None;
        let mut model = Model::new(model_name);
        let vl = sections.make_variables(&mut model)?;
        sections.make_objective(&mut model, &vl)?;
        sections.make_constraints(&mut model, &vl)?;
        Ok(model)
    }
}

impl<Index, Bias> Translator for LPTranslator<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type TranslateIn = PathBuf;
    type TranslateOut = Result<Model<Index, Bias>, TranslationErr>;

    fn translate(filepath: Self::TranslateIn) -> Self::TranslateOut {
        let contents = Self::read_file(filepath)?;
        let sections = Self::parse_sections(contents)?;
        Self::build_model(sections)
    }
}

impl<Index, Bias> BackTranslator for LPTranslator<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type BackTranslateIn = (Model<Index, Bias>, PathBuf);
    type BackTranslateOut = Result<(), TranslationErr>;

    fn back_translate(_data: Self::BackTranslateIn) -> Self::BackTranslateOut {
        // let (model, pathbuf) = data;
        todo!()
    }
}
