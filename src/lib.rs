pub mod app {
    pub mod container;
    pub mod factory;
}

pub mod domain {
    pub mod activity_pub {
        pub mod activity_pub;
        pub mod activity_pub_service;
    }

    pub mod follower {
        pub mod follower;
        pub mod follower_repository;
    }

    pub mod note {
        pub mod note;
        pub mod note_repository;
        pub mod paging;
    }

    pub mod user {
        pub mod user;
        pub mod user_repository;
        pub mod user_service;
    }

    pub mod app_config;
    pub mod constants;
    pub mod error;
    pub mod id_generator;
}

pub mod infrastructure {
    pub mod config {
        pub mod env_file;
    }

    pub mod databases {
        pub mod converters {
            pub mod follower;
            pub mod note;
            pub mod user;
        }

        pub mod entities;
    }

    pub mod repositories {
        pub mod follower;
        pub mod note;
        pub mod user;
    }
}

pub mod presentation {
    pub mod controllers {
        pub mod activity_pub;
        pub mod echo;
        pub mod user_note;
        pub mod user_management;
    }

    pub mod errors {
        pub mod api;
    }

    pub mod extractors {
        pub mod admin_claim;
    }
}

pub mod usecase {
    pub mod activity_pub;
    pub mod user_note;
    pub mod user_management;
}
