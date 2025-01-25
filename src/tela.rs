use std::{
    rc::Rc,
    sync::{Mutex, RwLock},
};

use crate::{
    enums::{Digito, Sinal},
    traits::{DynamicMutable, IntoDynamicMutable, Tela},
};

pub struct TelaKaio;

impl TelaKaio {
    pub fn new() -> Self {
        Self
    }
}

impl Tela for TelaKaio {
    fn adicione(&self, digito: Digito) {
        print!("{}", digito.to_char());
    }

    fn limpe(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn defina_sinal(&self, sinal: Sinal) {
        todo!()
    }

    fn defina_separador_decimal(&self) {
        todo!()
    }
}

impl IntoDynamicMutable<Box<dyn Tela>> for TelaKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Tela>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
