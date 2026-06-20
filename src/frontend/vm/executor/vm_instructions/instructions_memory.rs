use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_memory(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // 0x01: LOAD_CONST
            0x01 => {
                let arg = self.get_arg();
                // SAFETY: arg всегда в пределах constants.len(), т.к. индекс
                // выставляется парсером при компиляции и не может быть некорректным
                // для валидного байткода, сгенерированного Bparser'ом.
                let value = unsafe { *self.constants.get_unchecked(arg as usize) };
                self.push_u64(value)?;
                self.pc += 3;
            }

            // 0x02: LOAD_VAR
            0x02 => {
                let arg = self.get_arg();
                // SAFETY: arg ограничен var_count при компиляции
                let value = unsafe { *self.globals.get_unchecked(arg as usize) };
                self.push_u64(value)?;
                self.pc += 3;
            }

            // 0x03: STORE_VAR
            0x03 => {
                let arg = self.get_arg();
                let value = self.pop_u64()?;
                // SAFETY: то же самое — arg всегда валиден для скомпилированного байткода
                unsafe {
                    *self.globals.get_unchecked_mut(arg as usize) = value;
                }
                self.pc += 3;
            }

            _ => {
                println!("Error in execution memory opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
