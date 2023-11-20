fn main() {
    let mut vec: Vec<i32> = vec![11, 5, 2, 8, 10];
    
    let mut iters = 0;

    while iters <= vec.len() {
        for i in 0..vec.len() {
            if i+1 >= vec.len() {
                break;
            }

            if vec.get(i).unwrap() > vec.get(i+1).unwrap() {
                iters = 0;

                let tmp = vec.get(i).unwrap();

                vec.insert(i as usize, *vec.get(i+1).unwrap());

                vec.insert(i+1 as usize, *vec.get(i).unwrap());
            }
            else {
                iters += 1;
            }
        }
    }


    println!("{:?}", vec);
}
