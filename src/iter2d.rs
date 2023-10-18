use iter_augment::util::*;

pub struct IterColl2D<'a, T, const M: usize, const N: usize> {
    collection: &'a [T],
    index: usize,
    stride: i32,
    offset_x: i32,
    offset_y: i32,
    junk: T,
}

impl<'a, T, const M: usize, const N: usize> IterColl2D<'a, T, M, N>
where
    T: Copy,
{
    pub fn new(collection: &'a [T], stride: i32, offset_x: i32, offset_y: i32, junk: T) -> Self {
        IterColl2D {
            collection,
            index: 0,
            stride,
            offset_x,
            offset_y,
            junk,
        }
    }

    pub fn next_arr(&mut self) -> Option<[[T; N]; M]> {
        let length = self.collection.len();

        // util::util::dim_up_fit::<T,  >(self.collection);

        if self.index < length {
            let mut ret = [[self.junk; N]; M];
            ret.iter_mut().enumerate().for_each(|(i, p)| {

                // let idx = (self.index + i) as i32 + self.offset_x;
                // if idx >= 0 && (idx as usize) < length {
                //     *p = self.collection[idx as usize];
                // }
            });
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }

    // pub fn next_enum_arr(&mut self) -> Option<(usize, [T; M * N])> {
    //     let length = self.collection.len();
    //     if self.index < length {
    //         let mut ret = [self.junk; N];
    //         ret.iter_mut().enumerate().for_each(|(i, p)| {
    //             let idx = (self.index + i) as i32 + self.offset;
    //             if idx >= 0 && (idx as usize) < length {
    //                 *p = self.collection[idx as usize];
    //             }
    //         });
    //         self.index += 1;
    //         Some((self.index - 1, ret))
    //     } else {
    //         None
    //     }
    // }
}
