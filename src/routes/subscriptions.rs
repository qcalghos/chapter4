use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    //生成随机uid，将请求与日志关联
    let request_id=Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber.",request_id,form.email,form.name);
    log::info!("Saving new subscriber details in the database");//插入数据库之前记录日志
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email,name,subscribed_at)
        VALUES($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {} -  New subscriber details in the database",request_id);//成功插入到数据库
            HttpResponse::Ok().finish()}
            ,
        Err(e) => {
            // println!("Failed to execute insert:{}", e);
            log::error!("request_id {} - Failed to execute query:{:?}",request_id,e);//插入失败提示error级别日志
            HttpResponse::InternalServerError().finish()
        }
    }
}
