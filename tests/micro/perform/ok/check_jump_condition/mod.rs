use pir_8_emu::isa::instruction::InstructionJumpCondition;
use pir_8_emu::micro::MicroOp;
use self::super::super::universe;

mod unsatisfied;
mod satisfied;


fn satisfy(cond: InstructionJumpCondition, constant: u8, variable: u8, exp: bool) {
    for var in 0..=0b11111u8 {
        let flags = constant | (var & variable);

        let uni_orig = universe();
        let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();
        let mut stack = vec![flags];

        assert_eq!(MicroOp::CheckJumpCondition(cond).perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                   Ok(true));

        assert_eq!((memory, ports, registers, pc, sp, adr, ins), uni_orig);

        assert_eq!(stack, vec![exp as u8]);
    }
}
