pub trait Translator {
    type TranslateIn;
    type TranslateOut;

    fn translate(data: Self::TranslateIn) -> Self::TranslateOut;
}

pub trait BackTranslator {
    type BackTranslateIn;
    type BackTranslateOut;

    fn back_translate(data: Self::BackTranslateIn) -> Self::BackTranslateOut;
}
