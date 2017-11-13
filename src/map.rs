use std;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 100;

pub struct Map
{
    values: [[f32; WIDTH]; HEIGHT],
}

impl Map
{
    fn new() -> Self
    {
        Self{values: [[0.0; WIDTH]; HEIGHT]}
    }
}

impl std::ops::Index<[usize; 2]> for Map
{
    type Output = f32;
    fn index(&self, index: [usize; 2]) -> &Self::Output
    {
        &self.values[index[1]][index[0]]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Map
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output
    {
        &mut self.values[index[1]][index[0]]
    }
}
