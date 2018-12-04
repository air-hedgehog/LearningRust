
mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

// use outermost::inside::inner_function;
// use outermost::inside::secret_function;

//or

// use outermost::inside::{
//     inner_function,
//     secret_function,
// };

//or wishe gor

use outermost::inside::*;

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    inner_function();
    secret_function();
}
