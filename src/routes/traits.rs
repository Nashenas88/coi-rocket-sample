use rocket::Rocket;

pub trait RocketExt {
    fn route(self, config: impl RouteConfig) -> Self;
}

pub trait RouteConfig {
    fn mount(&self, rocket: Rocket) -> Rocket;
}

impl RocketExt for Rocket {
    fn route(self, config: impl RouteConfig) -> Self {
        config.mount(self)
    }
}
