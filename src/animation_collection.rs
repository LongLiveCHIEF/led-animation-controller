use smart_leds::RGB8;

pub struct AnimationCollection<O> {
    animations: O,
};

impl<O> Iterator for AnimationCollection<O>
where
    O: Iterator,
    O::Item: ,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Opsion<Self::Item> {
        None
    }
}
