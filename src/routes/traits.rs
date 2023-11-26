use rocket::{Build, Rocket};

pub(crate) trait RocketExt {
    fn route(self, config: impl RouteConfig) -> Self;
}

pub(crate) trait RouteConfig {
    fn mount(&self, rocket: Rocket<Build>) -> Rocket<Build>;
}

impl RocketExt for Rocket<Build> {
    fn route(self, config: impl RouteConfig) -> Self {
        config.mount(self)
    }
}
