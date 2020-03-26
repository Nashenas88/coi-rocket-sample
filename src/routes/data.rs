use crate::{dtos::data::DataDto, routes::traits::RouteConfig, services::service::IService};
use coi_rocket::inject;
use rocket::{get, response::status::NotFound, routes, Rocket};
use rocket_contrib::json::Json;
use std::sync::Arc;

#[inject]
#[get("/<id>")]
fn get(
    id: i64,
    #[inject] service: Arc<dyn IService>,
) -> Result<Json<DataDto>, NotFound<String>> {
    futures::executor::block_on(async move {
        let name = service.get(id).await.map_err(|e| {
            log::error!("{}", e);
            NotFound(e.to_string())
        })?;
        Ok(Json(DataDto::from(name)))
    })
}

#[inject]
#[get("/")]
fn get_all(#[inject] service: Arc<dyn IService>) -> Result<Json<Vec<DataDto>>, String> {
    futures::executor::block_on(async move {
        let data = service.get_all().await.map_err(|e| {
            log::error!("{}", e);
            e.to_string()
        })?;
        Ok(Json(data.into_iter().map(DataDto::from).collect::<Vec<_>>()))
    })
}

pub struct DataRoutes;

impl RouteConfig for DataRoutes {
    fn mount(&self, rocket: Rocket) -> Rocket {
        rocket.mount("/data", routes![get, get_all])
    }
}
