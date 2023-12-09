use super::gcd::Gcd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mod {
    pub value: usize,
    pub modulo: usize
}

impl Mod {
    pub fn try_mul(self, rhs: Self) -> Option<Mod> {
        let result = self.modulo.gcd(rhs.modulo);
        let lcm = (self.modulo / result.gcd) * rhs.modulo;

        if self.value == rhs.value {
            Some(Mod {
                value: self.value,
                modulo: lcm
            })
        } else if self.value > rhs.value {
            let diff = self.value - rhs.value;
            if diff % result.gcd == 0 {
                let value = ((diff / result.gcd) * result.rhs_inverse) % lcm;
                let value = (value * rhs.modulo) % lcm;
                let value = (value + rhs.value) % lcm;
                Some(Mod {
                    value,
                    modulo: lcm
                })
            } else {
                None
            }
        } else {
            let diff = rhs.value - self.value;
            if diff % result.gcd == 0 {
                let value = ((diff / result.gcd) * result.lhs_inverse) % lcm;
                let value = (value * self.modulo) % lcm;
                let value = (value + self.value) % lcm;
                Some(Mod {
                    value,
                    modulo: lcm
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_correct() {
        let lhs = Mod { value: 4, modulo: 7 };
        let rhs = Mod { value: 2, modulo: 9 };

        let result = lhs.try_mul(rhs);

        assert_eq!(result, Some(Mod { value: 11, modulo: 63 }))
    }
}
