use std::io;
use std::fmt;


#[derive(Debug, PartialEq)]
enum Pieza {
    Cruz,
    Circulo,
    Vacio,
}

#[derive(Debug)]
struct Tablero {
    matriz: Vec<Vec<Pieza>>,
}

impl Tablero {
    fn new() -> Tablero {
        let mut matriz = vec![];

        for _i in 0..3 {
            let fila = vec![Pieza::Vacio, Pieza::Vacio, Pieza::Vacio];
            matriz.push(fila);
        }

        Tablero { matriz }
    }

    fn poner(mut self, n: u32, pieza: Pieza) -> Tablero {
        let (i, j) = self.traducir(n as usize);
        self.matriz[i][j] = pieza;
        self
    }

    fn traducir(&self, n: usize) -> (usize, usize) {
        let c = self.matriz[0].len();
        (n / c, n % c)
    }

    fn ia(&mut self, pieza: Pieza) -> bool {
        let n = self.matriz.len();
        let m = self.matriz[0].len();

        let mut pos: Option<(usize, usize)> = None;

        'outer: for i in 0..n {
            for j in 0..m {
                if let Pieza::Vacio = self.matriz[i][j] {
                    pos = Some((i, j));
                    break 'outer;
                }
            }
        }

        match pos {
            Some((fila, columna)) => {
                self.matriz[fila][columna] = pieza;
                true
            },
            None => false
        }
    }

    fn valida(&self, n: u32) -> bool {
        let (i, j) = self.traducir(n as usize);
        match self.matriz[i][j] {
            Pieza::Vacio => true,
            _ => false,
        }
    }

    fn fin(&self) -> bool {
        let n = self.matriz.len();
        let m = self.matriz[0].len();

        // Fila horizontal
        for fila in self.matriz.iter() {
            let circulos = fila
                .iter()
                .filter(|x| match x { Pieza::Circulo => true, _ => false })
                .count();
            let cruces = fila
                .iter()
                .filter(|x| match x { Pieza::Cruz => true, _ => false })
                .count();
            if circulos == m {
                return true;
            }

            if cruces == m {
                return true;
            }
        }

        // Fila vertical
        let mut columna = true;
        for j in 0..m {
            columna = true;
            let pieza = &self.matriz[0][j];
            if let Pieza::Vacio = pieza {
                columna = false;
                continue;
            }
            for i in 1..n {
                if pieza != &self.matriz[i][j] {
                    columna = false;
                    break;
                }
            }

            if columna {
                return true;
            }
        }

        // Fila diagonal
        let mut diagonal1 = true;
        let mut diagonal2 = true;
        let pd1 = &self.matriz[0][0];
        let pd2 = &self.matriz[0][m-1];
        for (i, fila) in self.matriz.iter().enumerate() {
            if let Pieza::Vacio = fila[i] {
                diagonal1 = false;
            }
            if let Pieza::Vacio = fila[m - i - 1] {
                diagonal2 = false;
            }

            if &fila[i] != pd1 {
                diagonal1 = false;
            }

            if &fila[m - i - 1] != pd2 {
                diagonal2 = false;
            }
        }

        if diagonal1 || diagonal2 {
            return true;
        }

        // Tablero completo
        for fila in self.matriz.iter() {
            for celda in fila.iter() {
                if let Pieza::Vacio = celda {
                    return false;
                }
            }
        }
        false
    }

}

impl fmt::Display for Tablero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, fila) in self.matriz.iter().enumerate() {
            write!(f, "+---+---+---+\n");
            for (j, valor) in fila.iter().enumerate() {
                match valor {
                    Pieza::Cruz => {
                        write!(f, "| X ");
                    },
                    Pieza::Circulo => {
                        write!(f, "| o ");
                    }
                    _ => {
                        write!(f, "| {} ", i * 3 + j);
                    }
                }
            }
            write!(f, "|\n");
        }

        write!(f, "+---+---+---+\n")
    }
}

fn main() {
    let mut tablero = Tablero::new();

    loop {
        let mut input: String = String::new();

        println!("Tablero: \n{}", tablero);

        println!("Selecciona una casilla");
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("Error de lectura: {}", error);
        }

        match input.trim().parse() {
            Ok(number) if 0 <= number && number < 9 && tablero.valida(number) => {
                tablero = tablero.poner(number, Pieza::Circulo);
            }
            _ => {
                println!("Casilla incorrecta");
                continue;
            }
        }

        if tablero.fin() {
            println!("FINAL, Has ganado");
            println!("{}", tablero);
            break;
        }

        // Coloca la pieza la IA
        tablero.ia(Pieza::Cruz);
        if tablero.fin() {
            println!("FINAL, has perdido");
            println!("{}", tablero);
            break;
        }
    }
}
