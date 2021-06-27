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
        self.matrix.get(self.convert(p).as_slice())
    }

    pub fn get_transposed(&self, p: [usize;D]) -> &T {
        self.matrix.get_transposed(self.convert(p).as_slice())
    }

    fn convert(&self, p: [usize;D]) -> Vec<usize> {
        let id = position_to_id(&p, self.matrix.shape());
        let mut position = id_to_position(id, &self.shape_mask);
        (0..position.len()).for_each(|i| position[i] += self.position_mask[i]);

        position
    }

    pub fn dimension(&self) -> usize {
        dimension(&self.shape())
    }

    pub fn shape(&self) -> Vec<usize> {
        self.shape_mask.to_vec()
    }

    pub fn view<const C: usize>(&self, slice: &[Idx]) -> MatrixView<T, C> {
        let slice_dimension = slice_dimension(slice);
        
        if slice_dimension != C {
            panic!("slice dosent match with dimension of view")
        }

        if self.dimension() < slice_dimension {
            panic!("slice dimension must be less or equal than dimension matrix")
        }
        
        let mut shape_mask = self.shape_mask.clone();
        let mut position_mask = self.position_mask.clone();
        
        let mut free_indexes = self.shape_mask.iter().enumerate().filter(|(_i, &d)| d > 1);
        for &position in slice {
            let (index, &depht) = free_indexes.next().unwrap();
            match position {
                Idx::Value(value) => {
                    if value >= depht {
                        panic!("position out of the range")
                    }
                    
                    shape_mask[index] = 1;
                    position_mask[index] = value;
                },
                Idx::Free => {
                    shape_mask[index] = depht;
                    position_mask[index] = 0;
                }
            }
        }
        
    
        MatrixView { matrix: &self.matrix, shape_mask, position_mask }
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

        let sub_matrix = matrix.view::<1usize>(&[Idx::Value(0), Idx::Free]);
        assert_eq!(sub_matrix.get([0]), &0);
        assert_eq!(sub_matrix.get([1]), &1);
        assert_eq!(sub_matrix.get([2]), &2);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Value(1), Idx::Free]);
        assert_eq!(sub_matrix.get([0]), &3);
        assert_eq!(sub_matrix.get([1]), &4);
        assert_eq!(sub_matrix.get([2]), &5);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Free, Idx::Value(0)]);
        assert_eq!(sub_matrix.get([0]), &0);
        assert_eq!(sub_matrix.get([1]), &3);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Free, Idx::Value(1)]);
        assert_eq!(sub_matrix.get([0]), &1);
        assert_eq!(sub_matrix.get([1]), &4);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Free, Idx::Value(2)]);
        assert_eq!(sub_matrix.get([0]), &2);
        assert_eq!(sub_matrix.get([1]), &5);
    }

    #[test]
    fn viwe_of_view_test() {
        let matrix = matrix::new(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17], &vec![2usize, 3usize, 3usize]);

        let sub_matrix = matrix.view::<2usize>(&[Idx::Value(0), Idx::Free, Idx::Free]);
        
        let sub_matrix1 = sub_matrix.view::<1usize>(&[Idx::Value(1), Idx::Free]);
        assert_eq!(sub_matrix1.get([0]), &3);
        assert_eq!(sub_matrix1.get([1]), &4);
        assert_eq!(sub_matrix1.get([2]), &5);

        let sub_matrix1 = sub_matrix.view::<1usize>(&[Idx::Value(2), Idx::Free]);
        assert_eq!(sub_matrix1.get([0]), &6);
        assert_eq!(sub_matrix1.get([1]), &7);
        assert_eq!(sub_matrix1.get([2]), &8);

        let sub_matrix = matrix.view::<2usize>(&[Idx::Value(1), Idx::Free, Idx::Free]);
        
        let sub_matrix1 = sub_matrix.view::<1usize>(&[Idx::Free, Idx::Value(0)]);
        assert_eq!(sub_matrix1.get([0]), &9);
        assert_eq!(sub_matrix1.get([1]), &12);
        assert_eq!(sub_matrix1.get([2]), &15);

        let sub_matrix1 = sub_matrix.view::<1usize>(&[Idx::Free, Idx::Value(2)]);
        assert_eq!(sub_matrix1.get([0]), &11);
        assert_eq!(sub_matrix1.get([1]), &14);
        assert_eq!(sub_matrix1.get([2]), &17);
    }
}