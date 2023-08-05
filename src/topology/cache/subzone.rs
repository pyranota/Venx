use super::table::Block;

#[derive(Clone, Copy)]
pub struct SubZone<const SIZE: usize, T: Default>(pub [[[T; SIZE]; SIZE]; SIZE]);

impl<const SIZE: usize, T: Default + Copy> Default for SubZone<SIZE, T> {
    fn default() -> Self {
        Self([[[T::default(); SIZE]; SIZE]; SIZE])
    }
}
#[derive(Clone)]
pub struct SubZoneFlex<T: Default>(pub Vec<Vec<Vec<T>>>);

impl<T: Default + Clone> SubZoneFlex<T> {
    pub fn init(capacity: usize) -> Self {
        SubZoneFlex(vec![vec![vec![T::default(); capacity]; capacity]; capacity])
    }
}
