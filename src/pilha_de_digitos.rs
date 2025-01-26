use std::num::NonZeroUsize;

use crate::enums::{Digito, Sinal};

pub const MAXIMO_DIGITOS_POR_OPERANDO: u8 = 8;

#[derive(Default, Debug)]
pub struct PilhaDeDigitos {
    pub(in crate::pilha_de_digitos) memoria: [Option<Digito>; MAXIMO_DIGITOS_POR_OPERANDO as usize],
    topo: u8,
}

impl PilhaDeDigitos {
    pub fn receba(&mut self, digito: Digito) {
        if self.topo < MAXIMO_DIGITOS_POR_OPERANDO {
            self.memoria[self.topo as usize] = Some(digito);
            self.topo += 1;
        }
    }

    pub fn resete(&mut self) {
        self.memoria = [const { None }; MAXIMO_DIGITOS_POR_OPERANDO as usize];
        self.topo = 0;
    }

    pub fn largura(&self) -> u8 {
        self.topo
    }

    pub fn get_memory_slice(&self) -> &[Option<Digito>] {
        self.memoria.as_slice()
    }

    pub fn último(&self) -> &Option<Digito> {
        let idx = self.topo.saturating_sub(1) as usize;

        if let Some(option) = self.memoria.get(idx) {
            return option;
        }

        &None
    }

    pub fn remover_último(&mut self) -> Option<Digito> {
        if self.memoria.is_empty() {
            return None;
        }

        let idx = self.topo.saturating_sub(1) as usize;

        let mut item = self.memoria[idx].take();
        self.topo = self.topo.saturating_sub(1);

        item
    }

    pub fn converta_para_números(&self, separator_idx: Option<u8>, sinal: &Sinal) -> f32 {
        let mut num = 0.0_f32;

        for (idx, digito) in self.memoria.iter().flatten().enumerate() {
            if separator_idx.is_some_and(|separator_idx| idx > separator_idx as usize) {
                let decimal_places = 10_i32.pow(idx as u32 - separator_idx.unwrap() as u32);
                num += (digito.to_u8() as f32 / decimal_places as f32);
                continue;
            }

            num = num * 10.0 + digito.to_u8() as f32;
        }

        match sinal {
            Sinal::Negativo => -num,
            Sinal::Positivo => num.abs(),
        }
    }

    pub fn converta_de(num: f32) -> (Self, Option<u8>) {
        let mut stack = Self::default();
        let mut temp_array = [0_u8; MAXIMO_DIGITOS_POR_OPERANDO as usize];
        let mut temp_array_top = 0;

        let mut decimal = num.trunc() as i32;
        let mut fractional = num as f64 - decimal as f64;

        let is_floating = fractional > 0.0;

        while decimal > 0 && temp_array_top < MAXIMO_DIGITOS_POR_OPERANDO {
            temp_array[temp_array_top as usize] = (decimal % 10) as u8;
            temp_array_top += 1;
            decimal /= 10;
        }

        let mut temp_frac_vec = Vec::with_capacity(MAXIMO_DIGITOS_POR_OPERANDO as usize);

        while fractional.fract() > 0.0 {
            fractional *= 10.0;
            temp_frac_vec.push((fractional % 10.0).trunc() as u8);
        }

        for i in (0..temp_array_top).rev() {
            stack.receba(Digito::from_u8(temp_array[i as usize]));
        }

        let separator_idx = if is_floating {
            Some(temp_array_top.saturating_sub(1))
        } else {
            None
        };

        let mut i = 0;
        let temp_vec_len = temp_frac_vec.len();

        while (i < temp_vec_len && stack.largura() < 8) {
            stack.receba(Digito::from_u8(temp_frac_vec[i]));
            i += 1;
        }

        // remove os zeros inúteis do final (adicionados por falta de precisão dos computadores)
        while let Some(digito) = stack.último() {
            if digito != &Digito::Zero {
                break;
            }

            stack.remover_último();
        }

        (stack, separator_idx)
    }
}

impl<'a> IntoIterator for &'a PilhaDeDigitos {
    type Item = Option<Digito>;
    type IntoIter = PilhaDeDigitosIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PilhaDeDigitosIterator {
            pilha: self,
            index: 0,
        }
    }
}

pub struct PilhaDeDigitosIterator<'a> {
    pilha: &'a PilhaDeDigitos,
    index: usize,
}

impl Iterator for PilhaDeDigitosIterator<'_> {
    type Item = Option<Digito>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.pilha.topo as usize {
            return None;
        }

        let item = self.pilha.memoria[self.index].clone();
        self.index += 1;

        Some(item)
    }
}

#[cfg(test)]
mod test {
    use super::PilhaDeDigitos;
    use crate::enums::{Digito, Sinal};

    #[test]
    fn convert_digits_into_decimal_number() {
        let mut digitos = PilhaDeDigitos::default();
        digitos.receba(Digito::Um); // 00
        digitos.receba(Digito::Dois); // 01
                                      // vírgula
        digitos.receba(Digito::Sete); // 02
        digitos.receba(Digito::Oito); // 03
        digitos.receba(Digito::Zero); // 04

        assert_eq!(
            12.780,
            digitos.converta_para_números(Some(1), &Sinal::Positivo)
        );

        assert_eq!(
            -12.780,
            digitos.converta_para_números(Some(1), &Sinal::Negativo)
        );
    }

    #[test]
    fn convert_digits_into_integer() {
        let mut digitos = PilhaDeDigitos::default();
        digitos.receba(Digito::Um); // 00
        digitos.receba(Digito::Dois); // 01
        digitos.receba(Digito::Sete); // 02
        digitos.receba(Digito::Oito); // 03
        digitos.receba(Digito::Zero); // 04

        let num = digitos.converta_para_números(None, &Sinal::Positivo);
        assert_eq!(num, 12780.0);
    }

    #[test]
    fn convert_float_into_digits() {
        // o zero extra é pra garantir que o método irá cortá-lo do vetor final
        #[allow(clippy::excessive_precision)]
        let num = 12_894.950;

        let (digitos, separator) = PilhaDeDigitos::converta_de(num);

        assert_eq!(
            vec![
                &Digito::Um,
                &Digito::Dois,
                &Digito::Oito,
                &Digito::Nove,
                &Digito::Quatro,
                &Digito::Nove,
                &Digito::Cinco
            ],
            digitos
                .get_memory_slice()
                .iter()
                .flatten()
                .collect::<Vec<_>>()
        );

        assert_eq!(4, *separator.as_ref().unwrap());

        assert_eq!(
            num,
            digitos.converta_para_números(separator, &Sinal::Positivo)
        );
    }
}
