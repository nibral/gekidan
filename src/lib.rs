pub mod api {
    pub mod controllers {
        pub mod root_handlers;
        pub mod user_handlers;
        pub mod well_known_handlers;
    }

    pub mod dto {
        pub mod user;
    }
}

mod domain {
    pub mod models {
        pub mod app_config;
        pub mod user;
        pub mod user_rsa_key;
    }

    pub mod repositories {
        pub mod user;
    }

    pub mod services {
        pub mod activity_pub;
        pub mod app_config;
        pub mod rsa_key;
        pub mod user;
    }

    pub mod constants;
    pub mod error;
}

mod infrastructure {
    pub mod databases {
        pub mod sqlite3;
    }

    pub mod entities;

    pub mod models {
        pub mod user;
        pub mod user_rsa_key;
    }

    pub mod repositories {
        pub mod user;
    }

    pub mod error;
}

pub mod services {
    pub mod activity_pub;
    pub mod app_config;
    pub mod user;
}

pub mod container;
pub mod create_app;
