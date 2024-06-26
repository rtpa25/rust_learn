use crate::data::{query, DatabasePool};
use crate::service::ask;
use crate::{Clip, ServiceError};
use std::convert::TryInto;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;

    if clip.password.has_password() && clip.password != user_password {
        return Err(ServiceError::PermissionError(
            "Password not correct".to_string(),
        ));
    }
    Ok(clip)
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}
