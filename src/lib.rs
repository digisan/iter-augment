pub mod util {

    pub fn vec_dim_up<T, const N: usize>(a: &[T]) -> Vec<&[T]> {
        let cap = a.len() / N;
        let mut rt = Vec::with_capacity(cap);
        for i in 0..cap {
            rt.push(&a[(i * N)..((i + 1) * N)]);
        }
        rt
    }

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
}
