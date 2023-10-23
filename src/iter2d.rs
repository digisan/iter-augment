use iter_augment::*;

#[allow(dead_code)]
pub enum IdxFmt {
    XY,
    YX,
}

pub struct IterColl2D<'a, T, const W: usize, const H: usize, const N: usize, const M: usize> {
    collection2d: [&'a [T]; H],
    idx_x: usize,
    idx_y: usize,
    offset_x: i32,
    offset_y: i32,
}

impl<'a, T, const W: usize, const H: usize, const N: usize, const M: usize>
    IterColl2D<'a, T, W, H, N, M>
where
    T: Copy + Default,
{
    pub fn new(collection: &'a [T], offset_x: i32, offset_y: i32) -> Self {
        IterColl2D {
            collection2d: dim_up_fit::<T, W, H>(collection)
                .expect("cannot convert to 2d (W * H) array"),
            idx_x: 0,
            idx_y: 0,
            offset_x,
            offset_y,
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<[&[T]; M]> {
        // println!("Y:{}, X:{}", self.idx_y, self.idx_x);
        if let Some(roi) = subset::<T, N, M>(
            &self.collection2d,
            (self.idx_x as i32 + self.offset_x) as usize,
            (self.idx_y as i32 + self.offset_y) as usize,
        ) {
            self.idx_x += 1;
            return Some(roi);
        } else {
            self.idx_x = 0;
            self.idx_y += 1;
            let below_first = subset::<T, N, M>(
                &self.collection2d,
                (self.idx_x as i32 + self.offset_x) as usize,
                (self.idx_y as i32 + self.offset_y) as usize,
            );
            self.idx_x += 1;
            return below_first;
        }
    }

    #[allow(dead_code)]
    pub fn next_enum(&mut self, idx_fmt: IdxFmt) -> Option<((usize, usize), [&[T]; M])> {
        let ret = subset::<T, N, M>(
            &self.collection2d,
            (self.idx_x as i32 + self.offset_x) as usize,
            (self.idx_y as i32 + self.offset_y) as usize,
        );
        if let Some(roi) = ret {
            let ret = match idx_fmt {
                IdxFmt::XY => Some(((self.idx_x, self.idx_y), roi)),
                IdxFmt::YX => Some(((self.idx_y, self.idx_x), roi)),
            };
            self.idx_x += 1;
            return ret;
        } else {
            // check below first element
            self.idx_x = 0;
            self.idx_y += 1;
            if let Some(roi) = subset::<T, N, M>(
                &self.collection2d,
                (self.idx_x as i32 + self.offset_x) as usize,
                (self.idx_y as i32 + self.offset_y) as usize,
            ) {
                let ret = match idx_fmt {
                    IdxFmt::XY => Some(((self.idx_x, self.idx_y), roi)),
                    IdxFmt::YX => Some(((self.idx_y, self.idx_x), roi)),
                };
                self.idx_x += 1;
                return ret;
            }
            None
        }
    }
}
