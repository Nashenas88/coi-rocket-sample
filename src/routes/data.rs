use crate::dtos::errors::{Error, ErrorId};
use crate::{dtos::data::DataDto, routes::traits::RouteConfig, services::service::IService};
use coi_rocket::inject;
use rocket::serde::json::Json;
use rocket::Build;
use rocket::{get, response::status::NotFound, routes, Rocket};
use std::sync::Arc;

#[inject]
#[get("/<id>")]
async fn get(
    id: i64,
    #[inject] service: Arc<dyn IService>,
) -> Result<Json<DataDto>, NotFound<Json<Error>>> {
    let name = service.get(id).await.map_err(|e| {
        log::error!("{}", e);
        NotFound(Json(Error {
            id: ErrorId::NotFound,
            description: format!("Unable to find data with id {}", id),
        }))
    })?;
    Ok(Json(DataDto::from(name)))
}

#[inject]
#[get("/")]
async fn get_all(#[inject] service: Arc<dyn IService>) -> Result<Json<Vec<DataDto>>, Json<Error>> {
    let data = service.get_all().await.map_err(|e| {
        log::error!("{}", e);
        Json(Error {
            id: ErrorId::InternalError,
            description: "Internal Error".to_owned(),
        })
    })?;
    Ok(Json(
        data.into_iter().map(DataDto::from).collect::<Vec<_>>(),
    ))
}

pub(crate) struct DataRoutes;

impl RouteConfig for DataRoutes {
    fn mount(&self, rocket: Rocket<Build>) -> Rocket<Build> {
        rocket.mount("/data", routes![get, get_all])
    }
}
