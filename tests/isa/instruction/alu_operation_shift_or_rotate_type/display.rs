use pir_8_emu::isa::instruction::AluOperationShiftOrRotateType;


#[test]
fn lsf() {
    assert_eq!(AluOperationShiftOrRotateType::Lsf.to_string(), "LSF");
}

#[test]
fn asf() {
    assert_eq!(AluOperationShiftOrRotateType::Asf.to_string(), "ASF");
}

#[test]
fn rtc() {
    assert_eq!(AluOperationShiftOrRotateType::Rtc.to_string(), "RTC");
}

#[test]
fn rtw() {
    assert_eq!(AluOperationShiftOrRotateType::Rtw.to_string(), "RTW");
}
