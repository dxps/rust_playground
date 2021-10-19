use crate::data::AppDatabase;
use crate::web::forms::GetPasswordProtectedClip;
use crate::web::renderer::Renderer;
use crate::web::{ctx, PASSWORD_COOKIE};
use crate::{service, ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::Html;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

use super::forms::NewClip;
use super::page_error::PageError;
use super::HitCounter;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> Html<String> {
    let context = ctx::Home::default();
    Html(renderer.render(context, &[]))
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
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
            hit_counter.hit(shortcode, 1);
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

#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, GetPasswordProtectedClip>>,
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
    renderer: &State<Renderer<'_>>,
) -> Result<Html<String>, PageError> {
    if let Some(form) = &form.value {
        let req = service::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };
        match service::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                hit_counter.hit(shortcode, 1);
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(Html(renderer.render(context, &[])))
            }
            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(Html(renderer.render(context, &[e.as_str()])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("Clip not found.".to_owned())),
                _ => Err(PageError::Internal("server error".to_owned())),
            },
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(Html(renderer.render(
            context,
            &["A password is required to view this clip."],
        )))
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;
    let req = service::GetClip {
        shortcode: shortcode.clone().into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_passwd| Password::new(raw_passwd.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };
    match service::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode, 1);
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        }
        Err(e) => match e {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password, get_raw_clip]
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

#[cfg(test)]
pub mod test {
    use crate::data::AppDatabase;
    use crate::test::async_runtime;
    use crate::web::test::client;
    use rocket::http::Status;

    #[test]
    fn get_home() {
        let client = client();
        let resp = client.get("/").dispatch();
        assert_eq!(resp.status(), Status::Ok);
    }

    #[test]
    fn missing_clip() {
        let client = client();
        let resp = client.get("/clip/unknown-fake").dispatch();
        assert_eq!(resp.status(), Status::NotFound);
    }

    #[test]
    fn requires_password_when_applicable() {
        use crate::domain::clip::field::{Content, Expires, Password, Title};
        use crate::service;
        use rocket::http::{ContentType, Cookie};

        let rt = async_runtime();
        let client = client();
        let db = client.rocket().state::<AppDatabase>().unwrap();

        let req = service::NewClip {
            content: Content::new("content").unwrap(),
            title: Title::default(),
            expires: Expires::default(),
            password: Password::new("123".to_owned()).unwrap(),
        };

        let clip = rt.block_on(async move { service::new_clip(req, db.get_pool()).await.unwrap() });

        // Unauthorized access to clip when no password is provided.
        let resp = client
            .get(format!("/clip/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(resp.status(), Status::Unauthorized);

        // Unauthorized access to raw clip when no password cookie is provided.
        let resp = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(resp.status(), Status::Unauthorized);

        // Granted access to clip when password is provided.
        let resp = client
            .post(format!("/clip/{}", clip.shortcode.as_str()))
            .header(ContentType::Form)
            .body("password=123")
            .dispatch();
        assert_eq!(resp.status(), Status::Ok);

        // Granted access to raw clip when password cookie is provided.
        let resp = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "123"))
            .dispatch();
        assert_eq!(resp.status(), Status::Ok);

        // Unauthorized access to raw clip when password cookie is incorrect.
        let resp = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "456"))
            .dispatch();
        assert_eq!(resp.status(), Status::Unauthorized);
    }
}
