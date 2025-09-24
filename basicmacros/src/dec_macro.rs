macro_rules! vector {
    ($($x:expr),*) => {
        {

            let temp_vec = vec![$($x),*];
            temp_vec
        }
    }

}