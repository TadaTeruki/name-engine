pub struct SortedVecBuilder<C, V>
where
    C: PartialOrd,
{
    vec: Vec<(C, V)>,
}

impl<C, V> SortedVecBuilder<C, V>
where
    C: PartialOrd,
{
    pub fn new() -> Self {
        Self { vec: vec![] }
    }

    pub fn push(&mut self, cmp: C, value: V) {
        self.vec.push((cmp, value));
    }

    pub fn build(self) -> SortedVec<C, V> {
        let mut vec = self.vec;
        vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        SortedVec { vec }
    }
}

#[derive(Debug)]
pub struct SortedVec<C, V>
where
    C: PartialOrd,
{
    vec: Vec<(C, V)>,
}

impl<C, V> SortedVec<C, V>
where
    C: PartialOrd,
{
    pub fn find(&self, cmp: C) -> &(C, V) {
        let mut left: i32 = -1;
        let mut right: i32 = self.vec.len() as i32;

        while right - left > 1 {
            let mid = left + (right - left) / 2;
            match self
                .vec
                .get(mid as usize)
                .unwrap()
                .0
                .partial_cmp(&cmp)
                .unwrap()
            {
                std::cmp::Ordering::Less => left = mid,
                std::cmp::Ordering::Greater => right = mid,
                std::cmp::Ordering::Equal => right = mid,
            }
        }

        &self.vec[right as usize]
    }
}
