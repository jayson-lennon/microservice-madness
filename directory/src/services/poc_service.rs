pub mod response {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Capitalized {
        pub output: String,
    }
}

pub mod params {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Capitalize {
        pub input: String,
    }
}
