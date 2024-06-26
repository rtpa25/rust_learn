use super::model::{self, GetClip};
use crate::data::{DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: GetClip = model.into();
    let shortcode = model.shortcode.as_str();

    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: model::NewClip = model.into();

    Ok(sqlx::query_as!(
        model::Clip,
        "INSERT INTO clips (clip_id, shortcode, content, title, posted, expires, password, hits) VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: model::UpdateClip = model.into();

    Ok(sqlx::query_as!(
        model::Clip,
        "UPDATE clips SET content = ?, title = ?, expires = ?, password = ? WHERE shortcode = ? RETURNING *",
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode
    )
    .fetch_one(pool)
    .await?)
}
