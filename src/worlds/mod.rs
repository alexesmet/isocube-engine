pub mod blocks;

pub enum AddError { BlockOccupied }

pub struct World<'a> {
    pub blocks: Vec<blocks::Block<'a>>
}

impl<'a> World<'a> {
    pub fn new() -> Self { Self { blocks: Vec::new() } }

    pub fn add(&mut self, block: blocks::Block<'a>) -> Result<(), AddError> {
        for i in 0..(self.blocks.len()) {
            if block < self.blocks[i] {
                self.blocks.insert(i, block);
                return Ok(())
            }
            if block == self.blocks[i] {
                return Err(AddError::BlockOccupied)
            }
        }
        self.blocks.push(block);
        return Ok(());
    }
}

