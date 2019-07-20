use pir_8_emu::micro::{MicroOpPerformError, MicroOp};
use self::super::super::universe;


static VALID_IS_OK_VALUES: &[u8] = &[0, 1];


#[test]
fn jump() {
    for is_ok in 2..=0xFF {
        for start_addr in 0x11..0x22u16 {
            let start_addr = start_addr | ((start_addr + 1) << 8);

            for dest_addr in 0..0x11u16 {
                let dest_addr = dest_addr | ((dest_addr + 1) << 8);

                let mut uni_orig = universe();
                let (mut memory, mut ports, mut registers, mut pc, mut sp, mut adr, mut ins) = universe();

                let mut stack = vec![dest_addr as u8 + 1, dest_addr as u8, is_ok];
                *pc = start_addr;

                assert_eq!(MicroOp::Jump.perform(&mut stack, &mut memory, &mut ports, &mut registers, &mut pc, &mut sp, &mut adr, &mut ins),
                           Err(MicroOpPerformError::InvalidMicrostackTop(is_ok, VALID_IS_OK_VALUES)));

                *uni_orig.3 = start_addr;

                assert_eq!(memory, uni_orig.0);
                assert_eq!(ports, uni_orig.1);
                assert_eq!(registers, uni_orig.2);
                assert_eq!(pc, uni_orig.3);
                assert_eq!(sp, uni_orig.4);
                assert_eq!(adr, uni_orig.5);
                assert_eq!(ins, uni_orig.6);

                assert_eq!(stack, vec![]);
            }
        }
    }
}
