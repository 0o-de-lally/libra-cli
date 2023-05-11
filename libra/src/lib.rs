pub mod common {
    pub mod types {
        pub use zapatos::common::types::*;
    }
    pub mod utils {
        pub use zapatos::common::utils::*;
    }
    pub mod network {
        pub use zapatos::common::init::*;
    }
}

pub mod account {
    pub mod key_rotation {
        pub use zapatos::account::key_rotation::*;
    }
}
