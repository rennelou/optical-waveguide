use super::*;

impl<'a, T: Copy> Iterator for IterViewOndDimensional<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_left >= self.depht || self.pos_left >= self.pos_right {
            None
        } else {
            let value = self.inner.get([self.pos_left]);
            self.pos_left += 1;
            Some(value)
        }
    }
}

impl<'a, T: Copy> DoubleEndedIterator for IterViewOndDimensional<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos_right == 0usize || self.pos_right <= self.pos_left {
            None
        } else {
            let value = self.inner.get([self.pos_right-1]);
            self.pos_right -= 1;
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let matrix = matrix::new(vec![0,1,2,3,4,5,6,7,8,9,10,11], &vec![2usize, 6usize]);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Value(0), Idx::Free]);
        let mut iter = sub_matrix.iter();
        
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), Some(&3));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let sub_matrix = matrix.view::<1usize>(&[Idx::Value(1), Idx::Free]);
        let mut iter = sub_matrix.iter();
        
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next_back(), Some(&11));
        assert_eq!(iter.next_back(), Some(&10));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next_back(), Some(&9));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}