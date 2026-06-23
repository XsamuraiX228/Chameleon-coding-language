use crate::frontend::vm::executor::vrmachine::{CallFrame, VmState};

impl VmState {
    #[inline(always)]
    pub fn check_reg(&self) -> u8 {
        match (self.tos, self.stos) {
            (Some(_), Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
            (None, Some(_)) => unreachable!("VM Error: STOS isn't empty, while TOS is empty!"),
        }
    }

    #[inline(always)]
    pub fn push_u64(&mut self, value: u64) -> Result<(), String> {
        if self.tos.is_none() {
            self.tos = Some(value);
        } else if self.stos.is_none() {
            self.stos = self.tos;
            self.tos = Some(value);
        } else {
            self.stack.push(self.stos.unwrap());
            self.stos = self.tos;
            self.tos = Some(value);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn pop_u64(&mut self) -> Result<u64, String> {
        let res = self.check_reg();
        let value = match res {
            2 => {
                let val = self.tos.unwrap();
                self.tos = self.stos;
                self.stos = None;
                val
            }
            1 => {
                let val = self.tos.unwrap();
                self.tos = self.stack.pop();
                val
            }
            0 => self
                .stack
                .pop()
                .ok_or_else(|| "Stack underflow".to_string())?,
            _ => unreachable!(),
        };
        Ok(value)
    }

    #[inline(always)]
    pub fn get_a_b_i64(&mut self) -> Result<(i64, i64), String> {
        let res = self.check_reg();
        let (a, b) = match res {
            2 => {
                let val_2 = self.tos.unwrap();
                let val_1 = self.stos.unwrap();
                (self.tos, self.stos) = (None, None);
                (val_1, val_2)
            }
            1 => {
                let val_2 = self.tos.unwrap();
                let val_1 = self
                    .stack
                    .pop()
                    .ok_or_else(|| "Stack underflow in a".to_string())?;
                self.tos = self.stack.pop();
                (val_1, val_2)
            }
            0 => {
                let val_2 = self
                    .stack
                    .pop()
                    .ok_or_else(|| "Stack underflow in b".to_string())?;
                let val_1 = self
                    .stack
                    .pop()
                    .ok_or_else(|| "Stack underflow in a".to_string())?;
                (val_1, val_2)
            }
            _ => unreachable!(),
        };
        Ok((a as i64, b as i64))
    }

    #[inline(always)]
    pub(super) fn get_a_b_f64(&mut self) -> Result<(f64, f64), String> {
        let res = self.check_reg();
        let (a, b) =
            match res {
                2 => {
                    let val_2 = self.tos.unwrap();
                    let val_1 = self.stos.unwrap();
                    (self.tos, self.stos) = (None, None);
                    (val_1, val_2)
                }
                1 => {
                    let val_2 = self.tos.unwrap();
                    let val_1 = self.stack.pop().ok_or_else(|| {
                        "Stack underflow in argumnet a, func get_a_b_f64".to_string()
                    })?;
                    self.tos = self.stack.pop();
                    (val_1, val_2)
                }
                0 => {
                    let val_2 = self.stack.pop().ok_or_else(|| {
                        "Stack underflow in argumnet b, func get_a_b_f64".to_string()
                    })?;
                    let val_1 = self.stack.pop().ok_or_else(|| {
                        "Stack underflow in argumnet a, func get_a_b_f64".to_string()
                    })?;
                    (val_1, val_2)
                }
                _ => unreachable!(),
            };
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

impl CallFrame {
    #[inline(always)]
    pub(super) fn get_arg(&self) -> u16 {
        let low = self.bytecode[self.pc + 1] as u16;
        let high = self.bytecode[self.pc + 2] as u16;
        (high << 8) | low
    }
}
