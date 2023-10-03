use std::fmt::Display;

struct Matrice {
    rows: usize,
    cols: usize,
    data: Vec<Value>,
}

enum Value {
    Float(f64),
    Int(i64),
}

impl Matrice {
    fn new(rows: usize, cols: usize, data: Vec<Value>) -> Self {
        Matrice { rows, cols, data }
    }

    fn zeroes(dim: usize) -> Self {
        let mut vec = Vec::with_capacity(dim * dim);

        for _ in 0..(dim * dim) {
            vec.push(Value::Int(0))
        }

        Matrice::new(dim, dim, vec)
    }

    fn eye(dim: usize) -> Self {
        let mut zeroes = Matrice::zeroes(dim);

        for i in 0..dim {
            zeroes.data[i * (dim + 1)] = Value::Int(1)
        }

        zeroes
    }
}

impl Display for Matrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_float = false;

        for i in self.data {}

        let mut str = String::new();

        for (c, i) in self.data.iter().enumerate() {
            str.push(i.to_string().chars().next().unwrap());

            for _ in 0..3 {
                str.push(' ');
            }

            if (c + 1) % self.cols == 0 {
                str.push('\n')
            }
        }

        write!(f, "{str}")
    }
}

fn main() {
    let mat = Matrice::eye(5);
    println!("{mat}");
}
