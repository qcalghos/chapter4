use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form,pool),
    fields(
        // request_id=%Uuid::new_v4(),
        subscribe_email=%form.email,
        subscribe_name=%form.name
    )
)]
pub async fn subscribe(form:web::Form<FormData>,pool:web::Data<PgPool>)->HttpResponse{
    match insert_subscriber(&pool,&form).await{
        Ok(_)=>HttpResponse::Ok().finish(),
        Err(_)=>HttpResponse::InternalServerError().finish()
    }
}
#[tracing::instrument(name="Saving new subscriber details in the database",
skip(form,pool))]
pub async fn insert_subscriber(pool:&PgPool,form:&FormData)->Result<(),sqlx::Error>{
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email,name,subscribed_at)
        VALUES($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e|{
        tracing::error!("Failed to execute query:{:?}",e);
        e
    })?;
    Ok(())
}
/*pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    //生成随机uid，将请求与日志关联
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber.",%request_id,subscriber_email=%form.email,subscriber_name=%form.name);
    let request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    // tracing::info!("request_id {} - Adding '{}' '{}' as a new subscriber.",request_id,form.email,form.name);
    // tracing::info!("Saving new subscriber details in the database");//插入数据库之前记录日志
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
            // tracing::info!("request_id {} -  New subscriber details in the database",request_id);//成功插入到数据库
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // println!("Failed to execute insert:{}", e);
            tracing::error!(
                "request_id {} - Failed to execute query:{:?}",
                request_id,
                e
            ); //插入失败提示error级别日志
            HttpResponse::InternalServerError().finish()
        }
    }
}*/
