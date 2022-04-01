fn main() {
    let new_vector = sort_vector();
    println!("{:?}", new_vector);
}

fn sort_vector() -> [i8;10] {
    
    let mut vector:[i8;10] = [9,2,5,1,3,7,8,0,4,6];
    
    for i in 0..10 {
        for j in 0..10 {
            if vector[i] < vector[j] {
                let save = vector[i];
                vector[i] = vector[j];
                vector[j] = save;
            }
        }
    }
    
    return vector;
}