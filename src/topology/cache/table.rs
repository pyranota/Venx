use std::collections::HashMap;

pub struct TopoCch<'a> {
    map: HashMap<&'a [&'a [&'a [Option<Block>]]], usize>,
}

#[derive(Eq, Clone, Copy, Default, Debug)]
pub struct Block {
    id: usize,
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.id == 0 {
            state.write_u8(0);
        } else {
            state.write_u8(1)
        }
        state.finish();
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        if self.id == 0 && other.id != 0 {
            return false;
        }
        if self.id != 0 && other.id == 0 {
            return false;
        }
        true
    }
}

impl Block {
    pub fn air() -> Self {
        Block { id: 0 }
    }

    pub fn stone() -> Self {
        Block { id: 1 }
    }

    pub fn grass() -> Self {
        Block { id: 2 }
    }
}

#[cfg(test)]
mod tests {
    use crate::topology::cache::genzone::GenSlice;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn core() {
        let mut table2: HashMap<GenSlice, usize> = HashMap::new();

        let arr1 = [
            &[Block::air(), Block::stone()],
            &[Block::air(), Block::stone()],
        ];

        table2.insert(GenSlice(&arr1), 1);

        dbg!(table2.get(&GenSlice(&[
            &[Block::air(), Block::stone()],
            &[Block::air(), Block::grass()],
        ])));
    }
}
