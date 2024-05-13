trait CloneReal {
    fn reset_map(&self) {
        println!("hello");
    }
}

trait CloneReal2 {
    fn calls(&self) {}
}

#[allow(dead_code)]
struct Hello {
    ay: bool,
}

impl CloneReal2 for Hello {
    fn calls(&self) {
        self.reset_map()
    }
}
impl CloneReal for Hello {}

pub fn main() {
    let ay = Hello { ay: true };
    ay.calls()
}
