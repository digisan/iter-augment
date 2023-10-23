pub fn as_arr<T, const N: usize>(v: &[T]) -> [T; N]
where
    T: Copy + Default,
{
    let mut arr = [T::default(); N];
    v.iter().enumerate().for_each(|(i, e)| {
        if i < N {
            arr[i] = *e
        }
    });
    arr
}

pub fn vec_dim_up<T, const N: usize>(a: &[T]) -> Vec<&[T]> {
    let cap = a.len() / N;
    let mut rt = Vec::with_capacity(cap);
    for i in 0..cap {
        rt.push(&a[(i * N)..((i + 1) * N)]);
    }
    rt
}

pub fn dim_up<T, const N: usize, const M: usize>(a: &[T]) -> [&[T]; M] {
    let v = vec_dim_up::<T, N>(a);
    as_arr::<&[T], M>(&v)
}

pub fn dim_up_fit<T, const N: usize, const M: usize>(a: &[T]) -> Result<[&[T]; M], &str> {
    let a2 = dim_up::<T, N, M>(a);
    if a2.len() > 0 && (a.len() == a2.len() * a2[0].len()) {
        Ok(a2)
    } else {
        Err("result dim2 array is not exactly fit to original dim1 vec/arr")
    }
}

pub fn subset<'a, T, const SN_MAX: usize, const SM: usize>(
    a2d: &'a [&[T]],
    x: i32,
    y: i32,
) -> Option<[&'a [T]; SM]>
where
    T: Copy,
{
    let mut ret: [&[T]; SM] = [&[]; SM];

    let min_y = if y < 0 { 0 } else { y as usize };
    let min_y = if min_y >= a2d.len() { a2d.len() } else { min_y };

    let max_y = y + SM as i32;
    let max_y = if max_y < 0 { 0 } else { max_y as usize };
    let max_y = if max_y > a2d.len() { a2d.len() } else { max_y };

    println!("{} {}", min_y, max_y);

    let rows = &a2d[min_y..max_y];

    rows.iter().enumerate().for_each(|(i, ln)| {
        let min_x = if x < 0 { 0 } else { x as usize };
        let min_x = if min_x >= ln.len() { ln.len() } else { min_x };

        let max_x = x + SN_MAX as i32;
        let max_x = if max_x < 0 { 0 } else { max_x as usize };
        let max_x = if max_x > ln.len() { ln.len() } else { max_x };

        let i = if y < 0 { (i as i32 - y) as usize } else { i };
        ret[i] = &ln[min_x..max_x];
    });

    for rr in ret {
        if rr.len() != 0 {
            return Some(ret);
        }
    }
    None
}

pub fn make_owned_2d<T, const N: usize, const M: usize>(arr: &[&[T]], junk: T) -> [[T; N]; M]
where
    T: Copy,
{
    let mut ret = [[junk; N]; M];
    arr.iter().enumerate().for_each(|(i, r)| {
        if i < M {
            (*r).iter().enumerate().for_each(|(j, e)| {
                if j < N {
                    ret[i][j] = *e;
                }
            })
        }
    });
    ret
}
