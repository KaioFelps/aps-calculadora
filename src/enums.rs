#[derive(Clone, Debug)]
pub enum Sinal {
    Positivo,
    Negativo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn from_u8(num: u8) -> Self {
        match num {
            0 => Digito::Zero,
            1 => Digito::Um,
            2 => Digito::Dois,
            3 => Digito::Três,
            4 => Digito::Quatro,
            5 => Digito::Cinco,
            6 => Digito::Seis,
            7 => Digito::Sete,
            8 => Digito::Oito,
            9 => Digito::Nove,
            _ => panic!("Houve uma tentativa de serializar um digito inválido: {num}."),
        }
    }
}

#[derive(Clone, Debug)]
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
#[derive(Debug)]
pub enum Controle {
    LigaLimpaErro,
    Desliga,
    MemóriaLeituraEscrita,
    MemóriaSoma,
    MemóriaSubtração,
    Igual,
    SeparadorDecimal,
}

#[derive(Debug)]
pub enum Ação {
    Ctrl(Controle),
    Op(Operação),
    Di(Digito),
    Si(Sinal),
}
