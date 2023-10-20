use iter_augment::*;

pub struct IterColl2D<'a, T, const W: usize, const H: usize, const N: usize, const M: usize> {
    collection2d: [&'a [T]; H],
    index_x: usize,
    index_y: usize,
    offset_x: i32,
    offset_y: i32,
    junk: T,
    ret_default: [[T; N]; M],
}

impl<'a, T, const W: usize, const H: usize, const N: usize, const M: usize>
    IterColl2D<'a, T, W, H, N, M>
where
    T: Copy + Default,
{
    pub fn new(collection: &'a [T], offset_x: i32, offset_y: i32, junk: T) -> Self {
        IterColl2D {
            collection2d: dim_up_fit::<T, W, H>(collection)
                .expect("cannot convert to 2d (W * H) array"),
            index_x: 0,
            index_y: 0,
            offset_x,
            offset_y,
            junk,
            ret_default: [[junk; N]; M],
        }
    }

    pub fn next_arr(&mut self) -> Option<[[T; N]; M]> {
        if self.index_y < H {
            // for filling...
            let mut ret = self.ret_default;

            let rows: [&'a [T]; M] = self.collection2d[self.index_y..self.index_y + M];
            rows.iter().enumerate().for_each(|(i, row)| {
                // ret[i] = &row[self.index_x..self.index_x + N];
            });

            self.index_y += 1;
            Some(ret)
        } else {
            None
        }

        // let length = self.collection2d.len();
        // if self.index < length {
        //     let mut ret = [[self.junk; N]; M];
        //     ret.iter_mut().enumerate().for_each(|(i, p)| {

        //         // let idx = (self.index + i) as i32 + self.offset_x;
        //         // if idx >= 0 && (idx as usize) < length {
        //         //     *p = self.collection[idx as usize];
        //         // }
        //     });
        //     self.index += 1;
        //     Some(ret)
        // } else {
        //     None
        // }
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
