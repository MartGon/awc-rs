mod areaex;

use areaex::{AreaEx, Alphabet};

impl Alphabet for i32{

}

fn main() {
    let mut area = AreaEx::<i32>::new();
    area.dir(1).dir(2).dir(3).dir(4);

    area.print();
}
