pub enum Sinal {
    Positivo,
    Negativo,
}

#[derive(Clone)]
pub enum Digito {
    Zero,
    Um,
    Dois,
    Três,
    Quatro,
    Cinco,
    Seis,
    Sete,
    Oito,
    Nove,
}

impl Digito {
    pub fn to_u8(&self) -> u8 {
        match self {
            Digito::Zero => 0,
            Digito::Um => 1,
            Digito::Dois => 2,
            Digito::Três => 3,
            Digito::Quatro => 4,
            Digito::Cinco => 5,
            Digito::Seis => 6,
            Digito::Sete => 7,
            Digito::Oito => 8,
            Digito::Nove => 9,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Digito::Zero => '0',
            Digito::Um => '1',
            Digito::Dois => '2',
            Digito::Três => '3',
            Digito::Quatro => '4',
            Digito::Cinco => '5',
            Digito::Seis => '6',
            Digito::Sete => '7',
            Digito::Oito => '8',
            Digito::Nove => '9',
        }
    }
}

pub enum Operação {
    Soma,
    Subtração,
    Multiplicação,
    Divisão,
    Radiciação,
    Porcentagem,
    Noop,
}

impl Default for Operação {
    fn default() -> Self {
        Self::Noop
    }
}

pub enum Controle {
    LigaLimpaErro,
    Desliga,
    MemóriaLeituraEscrita,
    MemóriaSoma,
    MemóriaSubtração,
    Igual,
    SeparadorDecimal,
}
