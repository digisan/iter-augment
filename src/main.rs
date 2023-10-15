use iter_augment::util::*;

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let p = &a[0] as *const i32;
    println!("{p:p}");
    let p = &a[1] as *const i32;
    println!("{p:p}");
    let p = &a[2] as *const i32;
    println!("{p:p}");
    let p = &a[3] as *const i32;
    println!("{p:p}");

    // let a2 = dim_up::<i32, 4, 3>(&a);
    // println!("{a2:?}");

    match dim_up_fit::<i32, 4, 3>(&a) {
        Ok(a2) => {
            println!("{a2:?}");
            let p = &a2[0][0] as *const i32;
            println!("{p:p}");
            let p = &a2[0][1] as *const i32;
            println!("{p:p}");
            let p = &a2[1][0] as *const i32;
            println!("{p:p}");
            let p = &a2[1][1] as *const i32;
            println!("{p:p}");
        }
        Err(err) => {
            println!("{err}")
        }
    }
}
