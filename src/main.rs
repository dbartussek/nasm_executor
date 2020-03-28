#[link(name = "assembly", kind = "static")]
extern "sysv64" {
    pub fn echo(arg: usize) -> usize;
}

fn main() {
    let result = unsafe { echo(42) };

    println!("Result: {}", result);
}
