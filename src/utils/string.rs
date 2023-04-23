pub trait Tabulate {
    fn tabulate(self) -> Self;

    fn tabulate_n(self, n: usize) -> Self;
}

impl Tabulate for String {
    fn tabulate(self) -> Self {
        format!("\t{self}")
    }

    fn tabulate_n(self, n: usize) -> Self {
        format!("{}{self}", "\t".repeat(n))
    }
}