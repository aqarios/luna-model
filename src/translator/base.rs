pub trait Translator {
    type TranslateIn;
    type TranslateOut;

    fn translate(data: Self::TranslateIn) -> Self::TranslateOut;
}

pub trait BackTranslator: Translator {
    fn back_translate(data: Self::TranslateOut) -> Self::TranslateIn;
}
