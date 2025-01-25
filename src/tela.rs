use std::{io::Write, rc::Rc, sync::RwLock};

use crate::{
    enums::{Digito, Sinal},
    traits::{DynamicMutable, IntoDynamicMutable, Tela},
};

use crate::pilha_de_digitos::{PilhaDeDigitos, MAXIMO_DIGITOS_POR_OPERANDO};
const TOTAL_DIGITOS_DISPONIVEIS: u8 = 11;
const ALTURA_DIGITO: u8 = 5;
const LARGURA_DIGITO: u8 = 5;

const DIGITOS: [[&str; ALTURA_DIGITO as usize]; TOTAL_DIGITOS_DISPONIVEIS as usize] = [
    ["████", "█  █", "█  █", "█  █", "████"],
    ["  █ ", "  █ ", "  █ ", "  █ ", "████"],
    ["████", "   █", "████", "█   ", "████"],
    ["████", "   █", " ███", "   █", "████"],
    ["█  █", "█  █", "████", "   █", "   █"],
    ["████", "█   ", "████", "   █", "████"],
    ["███ ", "█   ", "████", "█  █", "████"],
    ["████", "   █", "  █ ", " █  ", " █  "],
    ["████", "█  █", "████", "█  █", "████"],
    ["████", "█  █", "████", "   █", " ███"],
    ["", " ", "", "", "◆"],
];

pub struct TelaKaio {
    sinal: Sinal,
    digitos: PilhaDeDigitos,
    posição_cursor: u8,
}

impl TelaKaio {
    pub fn new() -> Self {
        Console::init(0, 0);

        let mut tela = Self {
            digitos: PilhaDeDigitos::default(),
            posição_cursor: 0,
            sinal: Sinal::Positivo,
        };

        tela.limpe();

        tela
    }
}

impl Tela for TelaKaio {
    fn adicione(&mut self, digito: Digito) {
        if self.digitos.largura() == MAXIMO_DIGITOS_POR_OPERANDO {
            return;
        }

        self.digitos.receba(digito);
        self.atualize();
    }

    fn limpe(&mut self) {
        self.sinal = Sinal::Positivo;
        self.digitos.resete();
        self.posição_cursor = u8::MAX;
    }

    fn defina_sinal(&mut self, sinal: Sinal) {
        self.sinal = sinal;
        self.atualize();
    }

    fn defina_separador_decimal(&mut self) {
        self.posição_cursor = self.digitos.largura() - 1;
        self.atualize();
    }
}

impl TelaKaio {
    fn atualize(&mut self) {
        self.limpe();

        for (index, digito) in (&self.digitos).into_iter().enumerate() {
            self.insira_digito(
                digito.clone(),
                MAXIMO_DIGITOS_POR_OPERANDO - self.digitos.largura() + index as u8,
            );
        }
    }

    fn insira_digito(&self, digito: Digito, indice: u8) {
        for i in 0..ALTURA_DIGITO {
            Console::set_cursor((indice * LARGURA_DIGITO + 1) as usize, i as usize);
            print!("{}", DIGITOS[digito.to_u8() as usize][(i - 1) as usize])
        }

        if indice == self.posição_cursor {
            print!("{}", DIGITOS[10][4]);
        }

        println!();
        std::io::stdout().flush().unwrap();
    }
}

impl IntoDynamicMutable<Box<dyn Tela>> for TelaKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Tela>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}

pub struct Console;

impl Console {
    pub fn init(col: usize, row: usize) {
        Self::clear_screen();
        Self::set_cursor(col, row);
    }
    pub fn set_cursor(col: usize, row: usize) {
        print!("{esc}[{row};{col}H", esc = 27 as char);
    }

    pub fn clear_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
