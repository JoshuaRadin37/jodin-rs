//! The type-safe api for the compiler.



use jodin_common::ast::JodinNode;
use jodin_common::compilation::Target;
use crate::safe_api::translation_object::{EmitTo, TranslationObject};

pub mod error;
pub mod translation_object;

pub trait CompileTo<T : Target> {
    type Output : EmitTo<T>;


}

pub trait IntoTranslationObject {

    fn into_translation_object(self) -> TranslationObject;
}

impl<T: IntoTranslationObject> From<T> for TranslationObject {
    fn from(obj: T) -> Self {
        obj.into_translation_object()
    }
}
