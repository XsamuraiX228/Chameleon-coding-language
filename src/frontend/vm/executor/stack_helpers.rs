use crate::frontend::vm::executor::vrmachine::VirtualMachine;


impl VirtualMachine {
    #[inline(always)]
    pub(super) fn get_arg(&self) -> u16 {
        let low = self.bytecode[self.pc + 1] as u16;
        let high = self.bytecode[self.pc + 2] as u16;
        (high << 8) | low
    }

    #[inline(always)]
    pub(super) fn pop_u64(&mut self) -> Result<u64, String> {
        self.stack
            .pop()
            .ok_or_else(|| "VM Error: Stack underflow".to_string())
    }

    #[inline(always)]
    pub(super) fn get_a_b_i64(&mut self) -> Result<(i64, i64), String> {
        let b = self.pop_u64()?;
        let a = self.pop_u64()?;
        Ok((a as i64, b as i64))
    }

    #[inline(always)]
    pub(super) fn get_a_b_f64(&mut self) -> Result<(f64, f64), String> {
        let b = self.pop_u64()?;
        let a = self.pop_u64()?;
        Ok((f64::from_bits(a), f64::from_bits(b)))
    }

    #[inline(always)]
    pub(super) fn get_a_i64(&mut self) -> Result<i64, String> {
        Ok(self.pop_u64()? as i64)
    }

    #[inline(always)]
    pub(super) fn get_a_f64(&mut self) -> Result<f64, String> {
        Ok(f64::from_bits(self.pop_u64()?))
    }
}