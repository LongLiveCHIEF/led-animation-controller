
pub fn new_controller(colors: &[str], num_leds: usize){
    for (i, &item) in colors.iter().enumerate() {
        println!("{}", &item);
    }
}


#[cfg(test)]
mod tests {
    use heapless::Vec;
    use super::*;

    #[test]
    fn it_works() {

        assert_eq!(2 + 2, 4);
    }
}

