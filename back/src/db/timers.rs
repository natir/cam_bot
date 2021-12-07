//! Definition of timers record

/* std use */

/* crate use */
use diesel::prelude::*;

/* project use */
use super::schema::timers;
use super::schema::timers::dsl::*;

#[derive(
    rocket::serde::Serialize, rocket::serde::Deserialize, Queryable, Insertable, Debug, Clone,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "timers"]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub time_th: i64,
    pub message_th: i64,
    pub activate: bool,
}

impl Timer {
    pub async fn count(conn: &crate::Dbconn) -> i64 {
        conn.run(|c| timers.count().get_result(c)).await.unwrap()
    }

    pub async fn all(conn: &crate::Dbconn) -> diesel::result::QueryResult<Vec<Timer>> {
        conn.run(|c| timers.select(timers::all_columns).load::<Timer>(c))
            .await
    }

    pub async fn get(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<Timer> {
        conn.run(move |c| timers.filter(timers::id.eq(id_)).first::<Timer>(c))
            .await
    }

    pub async fn insert(
        command: Timer,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(|c| diesel::insert_into(timers).values(command).execute(c))
            .await
    }

    pub async fn update(
        id_: i32,
        command: Timer,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| {
            diesel::update(timers.filter(id.eq(id_)))
                .set((
                    id.eq(command.id),
                    name.eq(command.name),
                    value.eq(command.value),
                    activate.eq(command.activate),
                ))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| diesel::delete(timers.find(id_)).execute(c))
            .await
    }

    pub async fn toggle_activate(
        id_: i32,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| {
            let command = timers.find(id_).get_result::<Timer>(c)?;
            let new_status = !command.activate;
            let updated_command = diesel::update(timers.find(id));

            updated_command
                .set(timers::activate.eq(new_status))
                .execute(c)
        })
        .await
    }
}
