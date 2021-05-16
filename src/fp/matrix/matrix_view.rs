use super::*;

impl<'a, T: Copy> MatrixView<'a, T> {

    pub fn get(&self, masked_position: List<usize>) -> T {
        let id = hash(masked_position, self.matrix.shape());
        let position = list::sum(&unhash(id, &self.shape_mask), &self.position_mask);

        self.matrix.get(position)
    }

    pub fn dimension(&self) -> usize {
        self.shape_mask.iter().copied().fold(
            0, 
            |dim, depht|{ if depht > 1 {dim + 1} else {dim} }
        )
    }

    pub fn to_vec(&self) -> List<T> {
        if self.dimension() != 1 {
            panic!("matrix must have be unidimensional to be converted in list")
        }
    
        let &depht = self.shape_mask.iter().find(|&&d| d > 1).unwrap();
    
        (0..depht).map(|i| self.get(vec![i])).collect()
    }
}