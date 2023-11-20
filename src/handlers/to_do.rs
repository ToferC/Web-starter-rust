use actix_web::{web, get, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};
use uuid::Uuid;

use crate::models::ToDo;

#[get("/{lang}/toggle_status/{todo_id}")]
pub async fn toggle_status(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,

    _id: Option<Identity>,
    _req: HttpRequest,
) -> impl Responder {

    let (lang, todo_id) = path.into_inner();

    let todo = ToDo::get(todo_id).expect("Unable to retrieve todo");

    let _result = todo.toggle_status().expect("Unable to toggle status");

    return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()

}