pub struct IterColl<'a, T, const N: usize> {
    collection: &'a [T],
    index: usize,
    offset: i32,
    junk: T,
}

impl<'a, T, const N: usize> IterColl<'a, T, N>
where
    T: Copy,
{
    pub fn new(collection: &'a [T], offset: i32, junk: T) -> Self {
        IterColl {
            collection,
            index: 0,
            offset,
            junk,
        }
    }

    pub fn next_arr(&mut self) -> Option<[T; N]> {
        let length = self.collection.len();
        if self.index < length {
            let mut ret = [self.junk; N];
            ret.iter_mut().enumerate().for_each(|(i, p)| {
                let idx = (self.index + i) as i32 + self.offset;
                if idx >= 0 && (idx as usize) < length {
                    *p = self.collection[idx as usize];
                }
            });
            // for (i, p) in ret.iter_mut().enumerate() {
            //     let idx = self.index + i;
            //     if idx < length {
            //         *p = self.collection[idx];
            //     }
            // }
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }

    pub fn next_enum_arr(&mut self) -> Option<(usize, [T; N])> {
        let length = self.collection.len();
        if self.index < length {
            let mut ret = [self.junk; N];
            ret.iter_mut().enumerate().for_each(|(i, p)| {
                let idx = (self.index + i) as i32 + self.offset;
                if idx >= 0 && (idx as usize) < length {
                    *p = self.collection[idx as usize];
                }
            });
            self.index += 1;
            Some((self.index - 1, ret))
        } else {
            None
        }
    }
}

// array or vector already has next for 'for' loop
// impl<'a, T> Iterator for IterColl<'a, T, 1>
// where
//     T: Copy,
// {
//     type Item = T;
//     fn next(&mut self) -> Option<T> {
//         if self.index < self.collection.len() {
//             let item = &self.collection[self.index];
//             self.index += 1;
//             Some(*item)
//         } else {
//             None
//         }
//     }
// }
