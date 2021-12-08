//! Definition of twitch record

/* std use */

/* crate use */
use diesel::prelude::*;

/* project use */
use super::schema::twitch;
use super::schema::twitch::dsl::*;

#[derive(
    rocket::serde::Serialize, rocket::serde::Deserialize, Queryable, Insertable, Debug, Clone,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "twitch"]
pub struct Twitch {
    pub id: i32,
    pub token: String,
    pub refresh_token: String,
    pub expire_in: i32,
    pub generation_date: chrono::NaiveDate,
}

impl Twitch {
    pub async fn get(conn: crate::Dbconn) -> diesel::result::QueryResult<Twitch> {
        conn.run(move |c| twitch.filter(twitch::id.eq(1)).first::<Twitch>(c))
            .await
    }

    pub async fn update(
        twitch_: Twitch,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| {
            diesel::update(twitch.filter(id.eq(1)))
                .set((
                    id.eq(twitch_.id),
                    token.eq(twitch_.token),
                    refresh_token.eq(twitch_.refresh_token),
                    expire_in.eq(twitch_.expire_in),
                    generation_date.eq(twitch_.generation_date),
                ))
                .execute(c)
        })
        .await
    }

    pub fn need_update(&self) -> bool {
        self.generation_date
            .and_hms(0, 0, self.expire_in as u32)
            .signed_duration_since(chrono::Utc::now().naive_utc())
            < chrono::Duration::days(1)
    }
}
