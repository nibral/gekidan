pub mod api {
    pub mod controllers {
        pub mod root_handlers;
        pub mod user_handlers;
    }

    pub mod dto {
        pub mod user;
    }
}

mod domain {
    pub mod models {
        pub mod user;
    }

    pub mod repositories {
        pub mod user;
    }

    pub mod services {
        pub mod user;
    }

    pub mod constants;
}

mod infrastructure {
    pub mod databases {
        pub mod sqlite3;
    }

    pub mod entities;

    pub mod models {
        pub mod user;
    }

    pub mod repositories {
        pub mod user;
    }
}

pub mod services {
    pub mod user;
}

pub mod container;
pub mod create_app;
