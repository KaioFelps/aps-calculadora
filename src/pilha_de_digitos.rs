use crate::enums::Digito;

pub const MAXIMO_DIGITOS_POR_OPERANDO: u8 = 8;

#[derive(Default)]
pub struct PilhaDeDigitos {
    pub(in crate::pilha_de_digitos) memoria: [Option<Digito>; MAXIMO_DIGITOS_POR_OPERANDO as usize],
    topo: u8,
}

impl PilhaDeDigitos {
    pub fn receba(&mut self, digito: Digito) {
        if self.topo > MAXIMO_DIGITOS_POR_OPERANDO {
            self.memoria[self.topo as usize] = Some(digito);
            self.topo = self.topo.saturating_add(1);
        }
    }

    pub fn resete(&mut self) {
        self.memoria = [const { None }; MAXIMO_DIGITOS_POR_OPERANDO as usize];
        self.topo = 0;
    }

    pub fn largura(&self) -> u8 {
        self.topo
    }
}

impl<'a> IntoIterator for &'a PilhaDeDigitos {
    type Item = Digito;
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
    type Item = Digito;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.pilha.topo as usize {
            return None;
        }

        let item = self.pilha.memoria[self.index].clone();
        self.index += 1;

        item
    }
}
