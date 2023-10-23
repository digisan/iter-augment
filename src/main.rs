use iter_augment::*;

// mod iter;
// use iter::IterColl;

// mod iter2d;
// use iter2d::IdxFmt;
// use iter2d::IterColl2D;

const JUNK: i32 = -99999;

fn main() {
    let a = [
        01, 02, 03, 04, 05, 
        06, 07, 08, 09, 10, 
        11, 12, 13, 14, 15, 
        16, 17, 18, 19, 20,
    ];

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

    match dim_up_fit::<i32, 5, 4>(&a) {
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

            let roi = subset::<i32, 4, 4>(&a2, -1, -1);
            if let Some(roi) = roi {
                println!("roi 1:   {:?}", roi);

                let my2d: [[i32; 4]; 4] = make_owned_2d(&roi, JUNK);
                println!("owned 1: {:?}", my2d);
            }

            // let roi = subset::<i32, 3, 3>(&a2, 3, 3);
            // if let Some(roi) = roi {
            //     println!("roi 2:   {:?}", roi);

            //     let my2d: [[i32; 3]; 3] = make_owned_2d(&roi, JUNK);
            //     println!("owned 2: {:?}", my2d);
            // }
        }
        Err(err) => {
            println!("{err}")
        }
    }

    ////////////////////////////////////////////////////////////////

    // let vec = a.to_vec();
    // let mut my_iter: IterColl<'_, i32, 3> = IterColl::new(&vec[..5], -1, JUNK);
    // while let Some(e) = my_iter.next() {
    //     if matches!(e.first(), Some(j) if *j == JUNK) {
    //         continue;
    //     }
    //     if matches!(e.last(), Some(j) if *j == JUNK) {
    //         break;
    //     }
    //     println!("main_thread while 1: {:?}", e);
    // }

    // let mut my_iter: IterColl<'_, i32, 3> = IterColl::new(&a, -1, JUNK);
    // while let Some(e) = my_iter.next_enum() {
    //     if matches!(e.1.first(), Some(j) if *j == JUNK) {
    //         continue;
    //     }
    //     if matches!(e.1.last(), Some(j) if *j == JUNK) {
    //         break;
    //     }
    //     println!("main_thread while 2: {:?}", e);
    // }

    // for e in vec[5..].iter() {
    //     println!("main_thread for: {}", e);
    // }

    ////////////////////////////////////////////////////////////////

    // let mut my_iter: IterColl2D<'_, i32, 5, 4, 3, 3> = IterColl2D::new(&a, 1, 1);
    // while let Some(e) = my_iter.next_enum(IdxFmt::YX) {
    //     println!("main_thread while 2d 1: {:?}", e);
    // }
}
