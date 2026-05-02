mod field;
mod form;

pub use field::*;
pub use form::*;

/// Yeni bir [`Form`] ile bir dikey yerleşim oluşturur.
pub fn v_form() -> Form {
    Form::vertical()
}

/// Yeni bir [`Form`] ile bir yatay yerleşim oluşturur.
pub fn h_form() -> Form {
    Form::horizontal()
}

/// Yeni bir [`Field`] oluşturur.
pub fn field() -> Field {
    Field::new()
}
