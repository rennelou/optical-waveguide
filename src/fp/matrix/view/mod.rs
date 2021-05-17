use super::*;

pub mod iter_view;
pub struct IterViewOndDimensional<'a, T: 'a + Copy> {
    inner :&'a MatrixView<'a, T, 1usize>,
    depht: usize,
    pos_left: usize,
    pos_right: usize
}

impl<'a, T: 'a + Copy, const D: usize> MatrixView<'a, T, D> {

    pub fn get(&self, p: [usize;D]) -> &T {
        let id = hash(&p, self.matrix.shape());
        let position = list::sum(&unhash(id, &self.shape_mask), &self.position_mask.to_vec());

        self.matrix.get(position)
    }

    pub fn dimension(&self) -> usize {
        dimension(&self.shape())
    }

    pub fn shape(&self) -> Vec<usize> {
        self.shape_mask.to_vec()
    }
}

impl<'a, T: 'a + Copy> MatrixView<'a, T, 1usize> {
    pub fn depht(&self) -> usize {
        self.shape_mask.iter().find(|&&d| d > 1).unwrap().clone()
    }
    
    pub fn iter(&self) -> IterViewOndDimensional<T> {
        let depht = self.depht();
        IterViewOndDimensional {
            inner: &self,
            depht,
            pos_left: 0,
            pos_right: depht
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_test() {
        let matrix = matrix::new(vec![0,1,2,3,4,5], &vec![2usize, 3usize]);

        let sub_matrix = matrix.view::<1usize>(&[Index::Value(0), Index::Free]);
        assert_eq!(sub_matrix.get([0]), &0);
        assert_eq!(sub_matrix.get([1]), &1);
        assert_eq!(sub_matrix.get([2]), &2);

        let sub_matrix = matrix.view::<1usize>(&[Index::Value(1), Index::Free]);
        assert_eq!(sub_matrix.get([0]), &3);
        assert_eq!(sub_matrix.get([1]), &4);
        assert_eq!(sub_matrix.get([2]), &5);

        let sub_matrix = matrix.view::<1usize>(&[Index::Free, Index::Value(0)]);
        assert_eq!(sub_matrix.get([0]), &0);
        assert_eq!(sub_matrix.get([1]), &3);

        let sub_matrix = matrix.view::<1usize>(&[Index::Free, Index::Value(1)]);
        assert_eq!(sub_matrix.get([0]), &1);
        assert_eq!(sub_matrix.get([1]), &4);

        let sub_matrix = matrix.view::<1usize>(&[Index::Free, Index::Value(2)]);
        assert_eq!(sub_matrix.get([0]), &2);
        assert_eq!(sub_matrix.get([1]), &5);
    }
}