use std::{any::Any, rc::Rc, sync::RwLock};

use crate::{
    enums::{Ação, Controle, Digito, Operação, Sinal},
    pilha_de_digitos::PilhaDeDigitos,
    traits::{
        DynamicMutable, IntoDynamicMutable, Recebedor, Tela, Ucp, UcpRecebedor, UnwrapAndWrite,
    },
};

pub struct UcpKaio {
    tela: Option<DynamicMutable<Box<dyn Tela>>>,
    digitos_operando_1: PilhaDeDigitos,
    digitos_operando_2: PilhaDeDigitos,
    sinais: [Sinal; 2],
    posições_separator: [Option<u8>; 2],
    operação: Option<Operação>,
}

impl UcpKaio {
    pub fn new() -> Self {
        Self {
            tela: None,
            digitos_operando_1: PilhaDeDigitos::default(),
            digitos_operando_2: PilhaDeDigitos::default(),
            sinais: [Sinal::Positivo, Sinal::Positivo],
            posições_separator: [None, None],
            operação: None,
        }
    }
}

impl Ucp for UcpKaio {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>) {
        self.tela = Some(tela)
    }

    fn obtenha_tela(&self) -> &Option<DynamicMutable<Box<dyn Tela>>> {
        &self.tela
    }

    fn armazene_digito(&mut self, digito: Digito) {
        match self.operação {
            Some(_) => self.digitos_operando_2.receba(digito),
            None => self.digitos_operando_1.receba(digito),
        };
    }

    fn calcule(&mut self) {
        let operação = match &self.operação {
            None => return,
            Some(operação) => operação,
        };

        let operando_1: f32 = self
            .digitos_operando_1
            .converta_para_números(self.posições_separator[0], &self.sinais[0]);

        let operando_2 = self
            .digitos_operando_2
            .converta_para_números(self.posições_separator[1], &self.sinais[1]);

        let mut result = 0.0;

        match operação {
            Operação::Soma => result = operando_1 + operando_2,
            Operação::Divisão => result = operando_1 / operando_2,
            Operação::Multiplicação => result = operando_1 * operando_2,
            Operação::Porcentagem => result = operando_1 * operando_2 / 100.0,
            Operação::Radiciação => result = operando_1.powf(operando_2),
            Operação::Subtração => result = operando_1 - operando_2,
            Operação::Subtração => result = operando_1 - operando_2,
            Operação::Noop => return,
        }

        let (digitos, indice_separador) = PilhaDeDigitos::converta_de(result);

        let tela = (&self.tela);
        let mut tela = tela.get_write_ref();

        tela.limpe();

        for (idx, digito) in (&digitos).into_iter().flatten().enumerate() {
            tela.adicione(digito);

            if indice_separador.is_some() && idx == indice_separador.unwrap() as usize {
                tela.defina_separador_decimal();
            }
        }
    }
}

impl Recebedor for UcpKaio {
    fn receba_digito(&mut self, digito: &Digito) {
        self.armazene_digito(digito.clone());
        (&self.tela).get_write_ref().adicione(digito.clone());
    }

    fn receba_operação(&mut self, operação: &Operação) {
        self.operação = Some(operação.clone());
        (&self.tela).get_write_ref().limpe();
    }

    fn receba_controle(&mut self, controle: &Controle) {
        match controle {
            Controle::Igual => self.calcule(),
            Controle::Desliga => std::process::exit(0),
            Controle::SeparadorDecimal => {
                let (indice, pilha) = self.obter_operando_corrente();
                let coluna = pilha.largura().saturating_sub(1);

                // é o índice (coluna) do número mais significante da parte decimal
                self.posições_separator[indice as usize] = Some(coluna);
                (&self.tela).get_write_ref().defina_separador_decimal();
            }
            _ => todo!(),
        };
    }

    fn interprete_ação(&mut self, ação: &Ação) {
        match ação {
            Ação::Ctrl(controle) => self.receba_controle(controle),
            Ação::Di(digito) => self.receba_digito(digito),
            Ação::Si(sinal) => {
                self.sinais[self.obter_operando_corrente().0 as usize] = sinal.clone();
                (&self.tela).get_write_ref().defina_sinal(sinal.clone());
            }
            Ação::Op(operação) => self.receba_operação(operação),
        };
    }
}

impl UcpKaio {
    fn obter_operando_corrente(&self) -> (u8, &PilhaDeDigitos) {
        if self.operação.is_some() {
            return (0, &self.digitos_operando_1);
        }

        (1, &self.digitos_operando_2)
    }
}

impl UcpRecebedor for UcpKaio {}

impl IntoDynamicMutable<Box<dyn UcpRecebedor>> for UcpKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn UcpRecebedor>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
