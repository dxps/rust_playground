#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: &str,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    _api_key: ApiKey,
) -> Result<Json<Clip>, ApiError> {
    let req = service::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_pwd| Password::new(raw_pwd.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };

    let clip = service::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode.into(), 1);
    Ok(Json(clip))
}

#[rocket::post("/", data = "<req>")]
pub async fn new_clip(
    req: Json<service::NewClip>,
    db: &State<AppDatabase>,
) -> Result<Json<Clip>, ApiError> {
    let clip = service::new_clip(req.into_inner(), db.get_pool()).await?;
    Ok(Json(clip))
}

#[rocket::put("/", data = "<req>")]
pub async fn update_clip(
    req: Json<service::UpdateClip>,
    db: &State<AppDatabase>,
) -> Result<Json<Clip>, ApiError> {
    let clip = service::update_clip(req.into_inner(), db.get_pool()).await?;
    Ok(Json(clip))
}
