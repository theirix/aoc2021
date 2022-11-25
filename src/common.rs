pub struct Answer {
    pub a: u64,
    pub b: u64,
    pub path: &'static str,
}

//impl Answer {
//pub const fn new(a: u64, b: u64) -> Answer {
//Answer { a, b, path: std::module_path!() }
//}
//}

#[macro_export]
macro_rules! answer {
    ($a: expr, $b: expr) => {
        Answer {
            a: $a,
            b: $b,
            path: std::module_path!(),
        }
    };
}
