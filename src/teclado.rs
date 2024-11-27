use crate::{tecla::Tecla, ucp::Ucp};

pub struct Teclado {
    ucp: Ucp,
    teclas: Vec<Tecla>,
}

impl Teclado {
    pub fn adicioneTecla(&mut self, tecla: Tecla) {
        self.teclas.push(tecla);
    }
}
