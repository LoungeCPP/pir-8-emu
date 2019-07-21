use pir_8_emu::binutils::pir_8_as::AssemblerDirective;

mod invalid;
mod valid;


#[test]
fn not_directive() {
    assert_eq!(AssemblerDirective::from_str("origin"), Ok(None));
    assert_eq!(AssemblerDirective::from_str("label save"), Ok(None));
    assert_eq!(AssemblerDirective::from_str("label load"), Ok(None));
}
