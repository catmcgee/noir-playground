fn add(x: u36, y: u36) -> u36 {
    let z = x+y;
    z
}

fn main(x: u36, y: u36) {
    let z = add(x,y);
    assert(z == x+y);
}