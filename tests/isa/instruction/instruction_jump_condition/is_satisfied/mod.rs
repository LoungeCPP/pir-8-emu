use pir_8_emu::isa::instruction::InstructionJumpCondition;

mod yes;
mod no;


fn satisfy(cond: InstructionJumpCondition, constant: u8, variable: u8, exp: bool) {
	for var in 0..=0b11111u8 {
		let f = constant | (var & variable);

		assert_eq!(cond.is_satisfied(f), exp, "{:05b}", f);
	}
}
