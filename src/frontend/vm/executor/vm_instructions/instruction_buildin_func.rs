use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl<'a> VirtualMachine<'a> {
    #[inline(always)]
    pub fn execute_buildin(&mut self, func_id: u8) -> Result<(), String> {
        // Заранее проверяем наличие активного кадра выполнения
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;

        match func_id {
            0 => {
                // Sin
                let val = self.state.get_a_f64()?;
                let res = f64::sin(val);
                self.state.push_u64(res.to_bits())?;
            }

            1 => {
                // Cos
                let val = self.state.get_a_f64()?;
                let res = f64::cos(val);
                self.state.push_u64(res.to_bits())?;
            }

            8 => {
                // Sqrt
                let val = self.state.get_a_f64()?;
                let res = f64::sqrt(val);
                self.state.push_u64(res.to_bits())?;
            }

            2 => {
                // AbsInt
                let val = self.state.get_a_i64()?;
                let res = val.abs();
                self.state.push_u64(res as u64)?;
            }

            3 => {
                // AbsFloat
                let val = self.state.get_a_f64()?;
                let res = val.abs();
                self.state.push_u64(res.to_bits())?;
            }

            4 => {
                // MinInt
                let (a, b) = self.state.get_a_b_i64()?;
                let res = std::cmp::min(a, b);
                self.state.push_u64(res as u64)?;
            }

            5 => {
                // MinFloat
                let (a, b) = self.state.get_a_b_f64()?;
                let res = f64::min(a, b);
                self.state.push_u64(res.to_bits())?;
            }

            6 => {
                // MaxInt
                let (a, b) = self.state.get_a_b_i64()?;
                let res = std::cmp::max(a, b);
                self.state.push_u64(res as u64)?;
            }

            7 => {
                // MaxFloat
                let (a, b) = self.state.get_a_b_f64()?;
                let res = f64::max(a, b);
                self.state.push_u64(res.to_bits())?;
            }

            9 => {
                // RandomInt
                let (min, max) = self.state.get_a_b_i64()?;
                let res = rand::Rng::gen_range(&mut rand::thread_rng(), min..=max);
                self.state.push_u64(res as u64)?;
            }

            10 => {
                // RandomFloat
                let (min, max) = self.state.get_a_b_f64()?;
                let res = rand::Rng::gen_range(&mut rand::thread_rng(), min..=max);
                self.state.push_u64(res.to_bits())?;
            }

            _ => return Err(format!("Unknown native function ID: {}", func_id)),
        }

        // Продвигаем PC текущего кадра на 2 байта
        frame.pc += 2;

        Ok(())
    }
}
