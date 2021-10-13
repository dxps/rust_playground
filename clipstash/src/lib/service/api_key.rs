use crate::{
    data::{self, DatabasePool, RevokationStatus},
    web::api::ApiKey,
    ServiceError,
};

pub async fn generate_api_key(pool: &DatabasePool) -> Result<ApiKey, ServiceError> {
    let api_key = ApiKey::default();
    Ok(data::save_api_key(api_key, pool).await?)
}

pub async fn revoke_api_key(
    api_key: ApiKey,
    pool: &DatabasePool,
) -> Result<RevokationStatus, ServiceError> {
    Ok(data::revoke_api_key(api_key, pool).await?)
}

pub async fn is_api_key_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool, ServiceError> {
    Ok(data::is_api_key_valid(api_key, pool).await?)
}
