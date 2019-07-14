use pir_8_emu::isa::GeneralPurposeRegister;


#[test]
fn defaults() {
    assert_eq!(GeneralPurposeRegister::defaults(),
               [GeneralPurposeRegister::new(0b000, 'F').unwrap(),
                GeneralPurposeRegister::new(0b001, 'S').unwrap(),
                GeneralPurposeRegister::new(0b010, 'X').unwrap(),
                GeneralPurposeRegister::new(0b011, 'Y').unwrap(),
                GeneralPurposeRegister::new(0b100, 'A').unwrap(),
                GeneralPurposeRegister::new(0b101, 'B').unwrap(),
                GeneralPurposeRegister::new(0b110, 'C').unwrap(),
                GeneralPurposeRegister::new(0b111, 'D').unwrap()]);
}

#[test]
fn from_letters_ok() {
    assert_eq!(GeneralPurposeRegister::from_letters("01234567"),
               Ok([GeneralPurposeRegister::new(0b000, '0').unwrap(),
                   GeneralPurposeRegister::new(0b001, '1').unwrap(),
                   GeneralPurposeRegister::new(0b010, '2').unwrap(),
                   GeneralPurposeRegister::new(0b011, '3').unwrap(),
                   GeneralPurposeRegister::new(0b100, '4').unwrap(),
                   GeneralPurposeRegister::new(0b101, '5').unwrap(),
                   GeneralPurposeRegister::new(0b110, '6').unwrap(),
                   GeneralPurposeRegister::new(0b111, '7').unwrap()]));

    assert_eq!(GeneralPurposeRegister::from_letters("QwErTyUi"),
               Ok([GeneralPurposeRegister::new(0b000, 'Q').unwrap(),
                   GeneralPurposeRegister::new(0b001, 'w').unwrap(),
                   GeneralPurposeRegister::new(0b010, 'E').unwrap(),
                   GeneralPurposeRegister::new(0b011, 'r').unwrap(),
                   GeneralPurposeRegister::new(0b100, 'T').unwrap(),
                   GeneralPurposeRegister::new(0b101, 'y').unwrap(),
                   GeneralPurposeRegister::new(0b110, 'U').unwrap(),
                   GeneralPurposeRegister::new(0b111, 'i').unwrap()]));
}

#[test]
fn from_letters_err_too_short() {
    for i in 0..0b111 {
        assert_eq!(GeneralPurposeRegister::from_letters(&"01234567"[..i]), Err(-1));
    }
}

#[test]
fn from_letters_err_too_long() {
    let mut s = "ABCDEFGH".to_string();
    for i in 0..100 {
        s.push(s.chars().nth(i % 8).unwrap());
        assert_eq!(GeneralPurposeRegister::from_letters(&s), Err(8));
    }
}
