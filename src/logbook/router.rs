pub mod router {
    use crate::users::auth::auth;
    use crate::users::model::USER;
    use crate::SharedState;
    use std::sync::Arc;

    use crate::images::service::service::ImagesTable;
    use axum::Json;
    use axum::{extract::State, response::IntoResponse, Router};

    use serde_json::json;
    use serde_json::Value;

    use axum::extract::{Extension, Path, Query};
    use axum::middleware;

    use crate::logbook::model::{CreateLogInfo, LogList, RequiredSelectListItems, UpdateLogInfo};
    use crate::logbook::service::service::{
        CREATELogInfoParams, GetLogbookByIdParams, GetLogbookListParams,
        LogInfoTable as log_info_table, SearchLogsParams,
    };
    use http::StatusCode;

    pub fn logbook_routes(shared_state: Arc<SharedState>) -> Router {
        Router::new()
            .route(
                "/log_info",
                axum::routing::get(get_logbook_list).route_layer(middleware::from_fn_with_state(
                    shared_state.connection_pool.clone(),
                    auth,
                )),
            )
            .route("/log_info/:id", axum::routing::get(get_logbook_by_id))
            .route("/log_info/:id", axum::routing::put(update_loginfo_handler))
            .route(
                "/log_info/",
                axum::routing::post(create_loginfo_handler).route_layer(
                    middleware::from_fn_with_state(shared_state.connection_pool.clone(), auth),
                ),
            )
            .with_state(shared_state)
    }

    #[utoipa::path(
        get,
        path = "/log_info",
        params(
            ("page" = Option<i64>, Query, description = "page"),
            ("page_size" = Option<i64>, Query, description = "page_size"),
            ("start_date" = Option<NaiveDateTime>, Query, description = "start_date"),
            ("end_date" = Option<NaiveDateTime>, Query, description = "end_date"),
            ("search_query" = Option<String>, Query, description = "search_query")
        ),
        responses(
            (status = 200, description = "List all todos successfully", body = [model::LogInfo])
        )
    )]
    pub async fn get_logbook_list(
        Extension(user): Extension<USER>,
        State(shared_state): State<Arc<SharedState>>,
        Query(params): Query<SearchLogsParams>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state
            .connection_pool
            .pool
            .get()
            .expect("Failed connection to POOL");

        match log_info_table::new(connection).get_logbook_list(GetLogbookListParams {
            search_params: params,
            user,
        }) {
            Ok(log_info) => {
                let log_list: Vec<LogList> = log_info
                    .iter()
                    .map(
                        |RequiredSelectListItems {
                             id,
                             title,
                             description,
                             start_datetime,
                             image_id: other_image_id,
                             ..
                         }| {
                            let connection = shared_state
                                .connection_pool
                                .pool
                                .get()
                                .expect("Failed connection to POOL");
                            // let &LogInfo {image_id, ..} = x;
                            // let &RequiredSelectListItems {
                            //     id,
                            //     title,
                            //     description,
                            //     start_datetime,
                            //     image_id: other_image_id,
                            //     ..
                            // } = x;

                            let image = other_image_id;

                            if image.is_none() {
                                LogList {
                                    id: id.to_owned(),
                                    title: title.to_owned(),
                                    description: description.to_owned(),
                                    start_datetime: start_datetime.to_owned(),
                                    image_id: None,
                                    image_data: None,
                                }
                            } else {
                                match ImagesTable::new(connection)
                                    .get_log_image_data(image.unwrap())
                                {
                                    Ok(data) => {
                                        //    LogList {
                                        //         ..x
                                        //         image_data: data,
                                        //     }
                                        LogList {
                                            id: id.to_owned(),
                                            title: title.to_owned(),
                                            description: description.to_owned(),
                                            start_datetime: start_datetime.to_owned(),
                                            image_id: None,
                                            image_data: Some(data),
                                        }
                                    }
                                    Err(_error) => LogList {
                                        id: id.to_owned(),
                                        title: title.to_owned(),
                                        description: description.to_owned(),
                                        start_datetime: start_datetime.to_owned(),
                                        image_id: None,
                                        image_data: None,
                                    },
                                }
                            }

                            // if image.is_none() {
                            //     return x;
                            // } else {
                            //     match ImagesTable::new(connection).get_log_image_data(x) {
                            //         Ok(data) => {
                            //             LogList {
                            //                 ..x,
                            //                 image_data: data
                            //             },
                            //             Err((_) => {
                            //                 x
                            //             })
                            //     }
                            // }
                        },
                    )
                    .collect();

                Ok((StatusCode::OK, Json(json!({"data": log_list}))))
            }
            Err(_err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to read empire"})),
            )),
        }
    }

    #[utoipa::path(
        get,
        path = "/log_info/{id}",
        params(
            ("id" = i32, Path, description="Element id")
        ),
        responses(
            (status = 200, description = "todo by id successfully", body= [model:: LogInfo])
        )
    )]
    pub async fn get_logbook_by_id(
        State(shared_state): State<Arc<SharedState>>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state
            .connection_pool
            .pool
            .get()
            .expect("Failed connection to POOL");

        match log_info_table::new(connection).get_loginfo_by_id(GetLogbookByIdParams { id: id }) {
            Ok(log_item) => Ok((StatusCode::OK, Json(json!({"data": log_item})))),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )),
        }
    }

    #[utoipa::path(
        put,
        path = "/log_info/{id}",
        request_body = UpdateLogInfo,
        params(
            ("id" = i32, Path, description="Element id")
        ),
        // responses(
        //     (status = 200, description = "Logbook updated successfully", [model::LogInfo])
        // )
    )]
    pub async fn update_loginfo_handler(
        State(shared_state): State<Arc<SharedState>>,
        Path(id): Path<i32>,
        Json(body): Json<UpdateLogInfo>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let connection = shared_state
            .connection_pool
            .pool
            .get()
            .expect("Failed connection to POOL");

        match log_info_table::new(connection).update_loginfo_by_id(id, body) {
            Ok(updated_id) => Ok((StatusCode::OK, Json(json!(updated_id)))),
            Err(_error) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to read empire"})),
            )),
        }

        // Ok((StatusCode::OK, Json(json!({
        //     "id": params.id,
        //     "body": body,
        // }))))
    }

    #[utoipa::path(
        post,
        path = "/log_info/",
        request_body = CreateLogInfo,

    )]
    pub async fn create_loginfo_handler(
        Extension(user): Extension<USER>,
        State(shared_state): State<Arc<SharedState>>,
        Json(body): Json<CreateLogInfo>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        let conntection = shared_state.connection_pool.pool.get().expect("Failed connection to POOL");

        match log_info_table::new(conntection).create_loginfo(CREATELogInfoParams {
            body,
            user_info: user,
        }) {
            Ok(user_id) => Ok((StatusCode::OK, Json(json!(user_id)))),
            Err(err) => {
                println!("{}", err);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": err.to_string()})),
                ))
            }
        }
    }

    // async fn get_logbook_list(Extension(params): Extension<LogInfoParams>) -> String {
    //     format!("Page size: {}, Page: {}", params.page_size, params.page)
    // }
}
