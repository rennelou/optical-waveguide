use super::*;

#[derive(Clone, Copy)]
pub enum Position {
    Free,
    Index(usize)
}

pub struct SliceMask {
    shape_mask: List<usize>,
    position_mask: List<usize>
}

pub fn new<T: Clone + Copy>(values: List<T>, shape: List<usize>) -> Matrix<T> {
    if shape.iter().product::<usize>() != values.len() {
        panic!("shape dosent match with values")
    }

    Matrix { values, shape }
}

pub fn new_single<T: Clone + Copy>(value: T) -> Matrix<T> {
    new(vec![value], vec![1])
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn raw(&self) -> &List<T> {
        &self.values
    }

    pub fn shape(&self) -> &List<usize> {
        &self.shape
    }

    pub fn dimension(&self) -> usize {
        if self.is_single() || self.is_empty() {
            0usize
        } else {
            self.shape().len()
        }
    }

    pub fn is_single(&self) -> bool {
        self.values.len() == 1
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get_at(&self, position: List<usize>) -> T {
        self.values[hash(position, &self.shape)]
    }

    pub fn get(&self, slice_mask: &SliceMask, masked_position: List<usize>) -> T {
        let id = hash(masked_position, &self.shape);
        let position = list::sum(&unhash(id, &slice_mask.shape_mask), &slice_mask.position_mask);

        self.get_at(position)
    }

    pub fn get_slice_mask(&self, slice: List<Position>) -> SliceMask {
        if self.shape.len() > slice.len() {
            panic!("position to get needs has the same matrix dimension")
        }
        
        let (shape_mask, position_mask) = slice.into_iter().zip(self.shape.clone().into_iter()).fold( 
            (vec![],vec![]), 
            |(mut shape_mask, mut position_mask), (position, depht )| {
                match position {
                    Position::Index(index) => {
                        if index >= depht {
                            panic!("position out of the range")
                        }
                        
                        shape_mask.push(1);
                        position_mask.push(index);
    
                        (shape_mask, position_mask)
                    },
                    Position::Free => {
                        shape_mask.push(depht);
                        position_mask.push(0);
    
                        (shape_mask, position_mask)
                    }
                }
            }
        );

        SliceMask { shape_mask, position_mask }
    }
}

fn unhash(id: usize, shape: &List<usize>) -> List<usize> {
    let (position, _) = shape.iter().rev().fold(
        (vec![], id), 
        |(position, quocient), depht| {
            let new_depht = quocient % depht;
            let new_quocient = quocient/depht;
            
            let new_position = list::concat(list::new(new_depht), position);
            (new_position, new_quocient)
        }
    );

    position
}

fn hash(position: List<usize>, shape: &List<usize>) -> usize {
    if position.len() > shape.len() {
        panic!("position to get needs has the same matrix dimension")
    }

    (0..position.len()).fold(0, |id, index| {
        id*shape[index]+position[index]
    })
}

pub fn single<T: Clone + Copy>(m: Matrix<T>) -> Option<T> {
    if m.values.len() == 1 {
        head(m.values.into_iter())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acess_position() {
        let m = new(vec![0,1,2,3,4,5], vec![2usize, 3usize]);

        assert_eq!(m.get_at(vec![0,0]), 0);
        assert_eq!(m.get_at(vec![0,1]), 1);
        assert_eq!(m.get_at(vec![0,2]), 2);
        assert_eq!(m.get_at(vec![1,0]), 3);
        assert_eq!(m.get_at(vec![1,1]), 4);
        assert_eq!(m.get_at(vec![1,2]), 5);
    }

    #[test]
    fn unhash_test() {
        let shape = vec![2usize, 3usize];
        assert_eq!(unhash(0, &shape), vec![0,0]);
        assert_eq!(unhash(1, &shape), vec![0,1]);
        assert_eq!(unhash(2, &shape), vec![0,2]);
        assert_eq!(unhash(3, &shape), vec![1,0]);
        assert_eq!(unhash(4, &shape), vec![1,1]);
        assert_eq!(unhash(5, &shape), vec![1,2]);
    }

    #[test]
    fn hash_test() {
        let shape = vec![2usize, 3usize];
        assert_eq!(hash(vec![0,0], &shape), 0);
        assert_eq!(hash(vec![0,1], &shape), 1);
        assert_eq!(hash(vec![0,2], &shape), 2);
        assert_eq!(hash(vec![1,0], &shape), 3);
        assert_eq!(hash(vec![1,1], &shape), 4);
        assert_eq!(hash(vec![1,2], &shape), 5);
    }

    #[test]
    fn concat() {
        let m = new(vec![0,1,2,3,4,5], vec![2usize, 3usize]);

        let slice_mask = m.get_slice_mask(vec![Position::Index(0), Position::Free]);
        assert_eq!(m.get(&slice_mask, vec![0]), 0);
        assert_eq!(m.get(&slice_mask, vec![1]), 1);
        assert_eq!(m.get(&slice_mask, vec![2]), 2);

        let slice_mask = m.get_slice_mask(vec![Position::Index(1), Position::Free]);
        assert_eq!(m.get(&slice_mask, vec![0]), 3);
        assert_eq!(m.get(&slice_mask, vec![1]), 4);
        assert_eq!(m.get(&slice_mask, vec![2]), 5);

        let slice_mask = m.get_slice_mask(vec![Position::Free, Position::Index(0)]);
        assert_eq!(m.get(&slice_mask, vec![0]), 0);
        assert_eq!(m.get(&slice_mask, vec![1]), 3);

        let slice_mask = m.get_slice_mask(vec![Position::Free, Position::Index(1)]);
        assert_eq!(m.get(&slice_mask, vec![0]), 1);
        assert_eq!(m.get(&slice_mask, vec![1]), 4);

        let slice_mask = m.get_slice_mask(vec![Position::Free, Position::Index(2)]);
        assert_eq!(m.get(&slice_mask, vec![0]), 2);
        assert_eq!(m.get(&slice_mask, vec![1]), 5);
        
    }
}