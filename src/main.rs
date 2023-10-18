use iter_augment::util::*;

mod iter;
use iter::IterColl;

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

    ////////////////////////////////////////////////////////////////

    let junk = -99999;

    // let mut my_iter: IterColl<'_, i32, 3> = IterColl::new(&vec[..5], -1, junk);
    // while let Some(e) = my_iter.next_arr() {
    //     if matches!(e.first(), Some(j) if *j == junk) {
    //         continue;
    //     }
    //     if matches!(e.last(), Some(j) if *j == junk) {
    //         break;
    //     }
    //     println!("main_thread while: {:?}", e);
    // }

    let mut my_iter: IterColl<'_, i32, 3> = IterColl::new(&a, -1, junk);
    while let Some(e) = my_iter.next_enum_arr() {
        if matches!(e.1.first(), Some(j) if *j == junk) {
            continue;
        }
        if matches!(e.1.last(), Some(j) if *j == junk) {
            break;
        }
        println!("main_thread while: {:?}", e);
    }

    // for e in vec[5..].iter() {
    //     println!("main_thread for: {}", e);
    // }
}
