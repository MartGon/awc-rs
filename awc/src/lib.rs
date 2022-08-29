
mod unit;
mod tile;
mod movement;

pub mod component;
pub mod map;
pub mod weapon;

pub fn test_lib(){
    println!("Testing this lib")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
