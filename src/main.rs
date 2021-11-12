use std::env;
use std::fs;
use std::slice::IterMut;

fn main() {
    let file_name = env::args().nth(1).unwrap();

    let mut file_buf = fs::read(file_name).unwrap();

    let mut code = file_buf.iter_mut();

    decode_instruction(get_instruction(&mut code).unwrap());

    while let Some(instruction_byte) = get_instruction(&mut code) {
        if decode_instruction(instruction_byte) { // 16-bit instruction

        } else { // 32-bit instruction
            println!("THIS IS A 32-BIT INSTRUCTION. NEXT BYTE IGNORED!");
            let _ = get_instruction(&mut code); // ignore the other bit
        }
    }
}

fn get_instruction(code_iterator: &mut IterMut<u8>) -> Option<u16> {
    let first_byte = code_iterator.next();
    if first_byte.is_none() {
        return None;
    }

    let second_byte = code_iterator.next();
    if second_byte.is_none() {
        return None;
    }

    let hi = *first_byte.unwrap();
    let lo = *second_byte.unwrap();

    Some(((hi as u16) << 8) | (lo as u16))
}

fn decode_instruction(instruction: u16) -> bool {
    let bits15to11 = instruction & 0xf800u16;
    
    if bits15to11 == 0xf800u16 || bits15to11 == 0xf000u16 || bits15to11 == 0xe800u16 {
        return false;
    }

    // this should be 16 bit instructions!

    print!("{:#018b} ", instruction);

    let opcode = (instruction & 0xfc00u16) >> 10;

    if opcode & 0x0030u16 == 0x0000 {
        decode_arithmetic_instruction(instruction);
    } else if opcode == 0x0010 {
        decode_data_handling_instruction(instruction);
    } else if opcode == 0x0011 {
        decode_special_instructions(instruction);
    } else if opcode & 0x003e == 0x0012 {
        handle_ldr_literal(instruction);
    } else if opcode & 0x003e == 0x0028 {
        handle_adr(instruction);
    } else if opcode & 0x003e == 0x002a {
        handle_add_sp_plus_immediate(instruction);
    } else if opcode & 0x003e == 0x0030 {
        handle_store_multiple_registers(instruction);
    } else if opcode & 0x003e == 0x0032 {
        handle_load_multiple_register(instruction);
    } else if opcode & 0x003e == 0x0038 {
        handle_unconditional_branch_t2(instruction);
    } else if opcode & 0x003c == 0x002c {
        decode_misc_16_bit(instruction);
    } else if opcode & 0x003c == 0x0034 {
        decode_cond_branch_supv_call(instruction);
    } else if opcode & 0x003c == 0x0014 ||
        opcode & 0x0038 == 0x0018 ||
        opcode & 0x0038 == 0x0020 {
        decode_load_store_single_data(instruction);
    } else {
        println!("????");
    }

    true
}

fn decode_arithmetic_instruction(instruction: u16) {
    let opcode = (instruction & 0x3e00) >> 9;

    if opcode == 0x000c {
        handle_add_register_t1(instruction);
    } else if opcode == 0x000d {
        handle_sub_register(instruction);
    } else if opcode == 0x000e {
        handle_add_three_bit_imm(instruction);
    } else if opcode == 0x000f {
        handle_sub_three_bit_imm(instruction);
    } else {
        let opcode = opcode & 0x001c;

        if opcode == 0x0000 {
            handle_lsl_imm(instruction);
        } else if opcode == 0x0004 {
            handle_lsr_imm(instruction);
        } else if opcode == 0x0008 {
            handle_asr_imm(instruction);
        } else if opcode == 0x0010 {
            handle_move_imm(instruction);
        } else if opcode == 0x0014 {
            handle_compare_imm(instruction);
        } else if opcode == 0x0018 {
            handle_add_eight_bit_imm(instruction);
        } else if opcode == 0x001c {
            handle_sub_eight_bit_imm(instruction);
        } else {
            println!("ARITHMETIC: ???");
        }
    }

}

fn handle_add_register_t1(instruction: u16) {
    let rm = (instruction & 0x01c0) >> 6;
    let rn = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;
    
    println!("ADDS r{rd}, r{rn}, r{rm}", rd = rd, rn = rn, rm = rm);
}

fn handle_sub_register(instruction: u16) {
    let rm = (instruction & 0x01c0) >> 6;
    let rn = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;
    
    println!("SUBS r{rd}, r{rn}, r{rm}", rd = rd, rn = rn, rm = rm);
}

fn handle_add_three_bit_imm(instruction: u16) {
    let imm3 = (instruction & 0x01c0) >> 6;
    let rn = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;
    
    println!("ADDS r{rd}, r{rn}, #{imm3}", rd = rd, rn = rn, imm3 = imm3);
}

fn handle_sub_three_bit_imm(instruction: u16) {
    let imm3 = (instruction & 0x01c0) >> 6;
    let rn = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;
    
    println!("SUBS r{rd}, r{rn}, #{imm3}", rd = rd, rn = rn, imm3 = imm3);
}

fn handle_lsl_imm(instruction: u16) {
    let imm5 = (instruction & 0x07c0) >> 6;
    let rm = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;

    if imm5 != 0 {
        println!("LSLS r{}, r{}, #{}", rd, rm, imm5);
    } else {
        println!("MOVS r{}, r{}", rd, rm);
    }
}

fn handle_lsr_imm(instruction: u16) {
    let imm5 = (instruction & 0x07c0) >> 6;
    let rm = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;

    println!("LSRS r{}, r{}, #{}", rd, rm, imm5);
}

fn handle_asr_imm(instruction: u16) {
    let imm5 = (instruction & 0x07c0) >> 6;
    let rm = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;

    println!("ASRS r{}, r{}, #{}", rd, rm, imm5);
}

fn handle_move_imm(instruction: u16) {
    let rd = (instruction & 0x0700) >> 8;
    let imm8 = instruction & 0x00ff;

    println!("MOVS r{}, #{}", rd, imm8);
}

fn handle_compare_imm(instruction: u16) {
    let rn = (instruction & 0x0700) >> 8;
    let imm8 = instruction & 0x00ff;

    println!("CMP r{}, #{}", rn, imm8);
}

fn handle_add_eight_bit_imm(instruction: u16) {
    let rdn = (instruction & 0x0700) >> 8;
    let imm8 = instruction & 0x00ff;

    println!("ADDS r{}, #{}", rdn, imm8);
}

fn handle_sub_eight_bit_imm(instruction: u16) {
    let rdn = (instruction & 0x0700) >> 8;
    let imm8 = instruction & 0x00ff;

    println!("SUBS r{}, #{}", rdn, imm8);
}

fn decode_data_handling_instruction(instruction: u16) {
    let opcode = (instruction & 0x03c0) >> 6;

    match opcode {
        0x0000 => handle_and_register(instruction),
        0x0001 => handle_eor_register(instruction),
        0x0002 => handle_lsl_register(instruction),
        0x0003 => handle_lsr_register(instruction),
        0x0004 => handle_asr_register(instruction),
        0x0005 => handle_adc_register(instruction),
        0x0006 => handle_sbc_register(instruction),
        0x0007 => handle_ror_register(instruction),
        0x0008 => handle_tst_register(instruction),
        0x0009 => handle_rsb_imm(instruction),
        0x000a => handle_cmp_register_t1(instruction),
        0x000b => handle_cmn_register(instruction),
        0x000c => handle_orr_register(instruction),
        0x000d => handle_mul_register(instruction),
        0x000e => handle_bic_register(instruction),
        0x000f => handle_mvn_register(instruction),
        _ => println!("??? FROM DATA HANDLING"),
    }
}

fn handle_and_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("ANDS r{}, r{}", rdn, rm);
}

fn handle_eor_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("EORS r{}, r{}", rdn, rm);
}

fn handle_lsl_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("LSLS r{}, r{}", rdn, rm);
}

fn handle_lsr_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("LSRS r{}, r{}", rdn, rm);
}

fn handle_asr_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("ASRS r{}, r{}", rdn, rm);
}

fn handle_adc_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("ADCS r{}, r{}", rdn, rm);
}

fn handle_sbc_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("SBCS r{}, r{}", rdn, rm);
}

fn handle_ror_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("RORS r{}, r{}", rdn, rm);
}

fn handle_tst_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rn = instruction & 0x0007;

    println!("TST r{}, r{}", rn, rm);
}

fn handle_rsb_imm(instruction: u16) {
    let rn = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;

    if rn == rd {
        println!("RSBS r{}, #0", rn);
    } else {
        println!("RSBS r{}, r{}, #0", rd, rn);
    }
}

fn handle_cmp_register_t1(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rn = instruction & 0x0007;

    println!("CMP r{}, r{}", rn, rm);
}

fn handle_cmn_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rn = instruction & 0x0007;

    println!("CMN r{}, r{}", rn, rm);
}

fn handle_orr_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("ORRS r{}, r{}", rdn, rm);
}

fn handle_mul_register(instruction: u16) {
    let rn = (instruction & 0x0038) >> 3;
    let rdm = instruction & 0x0007;

    if rdm == rn {
        println!("MULS r{}, r{}", rn, rdm);
    } else {
        println!("MULS r{rd}, r{rn}, r{rm}", rd = rdm, rn = rn, rm = rdm);
    }
}

fn handle_bic_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rdn = instruction & 0x0007;

    println!("BICS r{}, r{}", rdn, rm);
}

fn handle_mvn_register(instruction: u16) {
    let rm = (instruction & 0x0038) >> 3;
    let rd = instruction & 0x0007;

    println!("MVNS r{}, r{}", rd, rm);
}

fn decode_special_instructions(instruction: u16) {
    let opcode = (instruction & 0x03c0) >> 6;

    if opcode & 0x000c == 0x0000 {
        handle_add_register_t2(instruction);
    } else if opcode == 0x0005 || opcode & 0x000e == 0x0006 {
        handle_cmp_register_t2(instruction);
    } else if opcode & 0x000c == 0x0008 {
        handle_mov_register(instruction);
    } else if opcode & 0x000e == 0x000c {
        handle_bx(instruction);
    } else if opcode & 0x000e == 0x000e {
        handle_blx_register(instruction);
    } else {
        println!("??? -> FROM SPECIAL INSTRUCTIONS");
    }
}

fn handle_add_register_t2(instruction: u16) {
    let rm = (instruction & 0x0078) >> 3;
    let rdn = (instruction & 0x0080) >> 4 | instruction & 0x0007;

    if rdn == 0x000d && rm == 0x000d {
        println!("ADD SP, SP, SP");
    } else if rdn == 0x000d {
        println!("ADD SP, r{}", rm);
    } else if rm == 0x000d {
        println!("ADD r{rdn}, SP, r{rdn}", rdn = rdn);
    } else {
        println!("ADD r{}, r{}", rdn, rm);
    }
}

fn handle_cmp_register_t2(instruction: u16) {
    let rm = (instruction & 0x0078) >> 3;
    let rn = (instruction & 0x0080) >> 4 | (instruction & 0x0007);

    println!("CMP r{}, r{}", rn, rm);
}

fn handle_mov_register(instruction: u16) {
    let rm = (instruction & 0x0078) >> 3;
    let rd = (instruction & 0x0080) >> 4 | (instruction & 0x0007);

    println!("MOV r{}, r{}", rd, rm);
}

fn handle_bx(instruction: u16) {
    let rm = (instruction & 0x0078) >> 3;

    println!("BX r{}", rm);
}

fn handle_blx_register(instruction: u16) {
    let rm = (instruction & 0x0078) >> 3;

    println!("BLX r{}", rm);
}

fn handle_ldr_literal(instruction: u16) {
    let rt = (instruction & 0x0700) >> 8;
    let imm8 = ((instruction & 0x00ff) as u32) << 2;

    println!("LDR r{}, [PC, #{}]", rt, imm8);
}

fn handle_adr(instruction: u16) {
    let rd = (instruction & 0x0700) >> 8;
    let imm8 = ((instruction & 0x00ff) as u32) << 2;

    println!("ADD r{}, PC, #{}", rd, imm8);
}

fn handle_add_sp_plus_immediate(instruction: u16) {
    let rd = (instruction & 0x0700) >> 8;
    let imm8 = ((instruction & 0x00ff) as u32) << 2;

    println!("ADD r{}, SP, #{}", rd, imm8);
}

fn handle_store_multiple_registers(instruction: u16) {
    let rn = (instruction & 0x0700) >> 8;
    let registers = instruction & 0x00ff;

    let mut register_list = String::from("");

    for i in 0..=7 {
        if registers & (1 << i) == 1 {
            if !register_list.is_empty() {
                register_list += ",";
            }

            register_list += format!("r{}", i).as_str();
        }
    }

    println!("STM r{}!, {{{}}}", rn, register_list);
}

fn handle_load_multiple_register(instruction: u16) {
    let rn = (instruction & 0x0700) >> 8;
    let registers = instruction & 0x00ff;

    let mut is_rn_in_register_list = false;
    let mut register_list = String::from("");

    for i in 0..=7 {
        if registers & (1 << i) == 1 {
            if i == rn {
                is_rn_in_register_list = true;
            }
            if !register_list.is_empty() {
                register_list += ",";
            }

            register_list += format!("r{}", i).as_str();
        }
    }

    if is_rn_in_register_list {
        println!("LDM r{}, {{{}}}", rn, register_list);
    } else {
        println!("LDM r{}!, {{{}}}", rn, register_list);
    }
}

fn handle_unconditional_branch_t2(instruction: u16) {
    todo!("IMPLEMENT UNCONDITIONAL BRANCHES");
}

fn decode_misc_16_bit(instruction: u16) {
    todo!("IMPLEMENT MISCELLANEOUS 16 BIT INSTRUCTIONS");
}

fn decode_cond_branch_supv_call(instruction: u16) {
    todo!("IMPLEMENT CONDITIONAL BRANCHES AND SUPERVISOR CALLS");
}

fn decode_load_store_single_data(instruction: u16) {
    todo!("IMPLEMENT LOAD AND STORING OF SINGLE DATA");
}