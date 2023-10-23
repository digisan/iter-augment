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

pub fn subset<'a, T, const SN: usize, const SM: usize>(
    a2d: &'a [&[T]],
    x: usize,
    y: usize,
) -> Option<[&'a [T]; SM]>
where
    T: Copy,
{
    if y + SM > a2d.len() {
        return None;
    }
    let mut ret: [&[T]; SM] = [&[]; SM];
    let rows = &a2d[y..y + SM];
    rows.iter().enumerate().for_each(|(i, ln)| {
        if x + SN <= ln.len() {
            ret[i] = &ln[x..x + SN];
        }
    });
    if ret.len() > 0 && ret[0].len() > 0 {
        Some(ret)
    } else {
        None
    }
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
