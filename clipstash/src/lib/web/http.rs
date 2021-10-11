use crate::data::AppDatabase;
use crate::web::ctx;
use crate::web::ctx::Home;
use crate::web::renderer::Renderer;
use crate::{service, ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

use super::forms::NewClip;
use super::page_error::PageError;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> Html<String> {
    let context = ctx::Home::default();
    Html(renderer.render(context, &[]))
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<Html<String>>, PageError> {
    //
    // An inner function, reused just here.
    fn render_with_status<T: ctx::PageCtx + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<Html<String>>, PageError> {
        Ok(status::Custom(status, Html(renderer.render(context, &[]))))
    }

    match service::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(err) => match err {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".into())),
            _ => Err(PageError::Internal("server error".into())),
        },
    }
}

#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, Html<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::NewClip {
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };
        match service::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internal error: {}", e);
                Err((
                    Status::InternalServerError,
                    Html(renderer.render(
                        ctx::Home::default(),
                        &["A server error occurred. Please try again."],
                    )),
                ))
            }
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;
                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {}", err);
                    "An error occurred. Please try again."
                }
            })
            .collect::<Vec<_>>();
        Err((
            Status::BadRequest,
            Html(renderer.render_with_data(ctx::Home::default(), ("clip", &form.context), &errors)),
        ))
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong"
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}
