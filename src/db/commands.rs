//! Definition of commands record

/* std use */

/* crate use */
use diesel::prelude::*;

/* project use */
use super::schema::commands;
use super::schema::commands::dsl::*;

#[derive(
    rocket::serde::Serialize, rocket::serde::Deserialize, Queryable, Insertable, Debug, Clone,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "commands"]
pub struct Command {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub activate: bool,
}

impl Command {
    pub async fn count(conn: &crate::Dbconn) -> i64 {
        conn.run(|c| commands.count().get_result(c)).await.unwrap()
    }

    pub async fn all(conn: &crate::Dbconn) -> diesel::result::QueryResult<Vec<Command>> {
        conn.run(|c| commands.select(commands::all_columns).load::<Command>(c))
            .await
    }

    pub async fn get(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<Command> {
        conn.run(move |c| commands.filter(commands::id.eq(id_)).first::<Command>(c))
            .await
    }

    pub async fn insert(
        command: Command,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(|c| diesel::insert_into(commands).values(command).execute(c))
            .await
    }

    pub async fn update(
	id_: i32,
        command: Command,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| diesel::update(commands.filter(id.eq(id_))).set((id.eq(command.id), name.eq(command.name), value.eq(command.value), activate.eq(command.activate))

	).execute(c))
            .await
    }
    
    pub async fn delete(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| diesel::delete(commands.find(id_)).execute(c))
            .await
    }

    pub async fn toggle_activate(
        id_: i32,
        conn: &crate::Dbconn,
    ) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| {
            let command = commands.find(id_).get_result::<Command>(c)?;
            let new_status = !command.activate;
            let updated_command = diesel::update(commands.find(id));

            updated_command
                .set(commands::activate.eq(new_status))
                .execute(c)
        })
        .await
    }
}
