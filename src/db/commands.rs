//! Definition of commands record

/* std use */

/* crate use */
use diesel::prelude::*;

/* project use */
use super::schema::commands;
use super::schema::commands::dsl::*;

#[derive(rocket::serde::Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "commands"]
pub struct Command {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub activate: Option<bool>,
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
	conn.run(move |c| commands.filter(commands::id.eq(id_)).first::<Command>(c)).await
    }

    pub async fn delete(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<usize> {
	conn.run(move |c| diesel::delete(commands.find(id_)).execute(c)).await
    }
    
    pub async fn toggle_activate(id_: i32, conn: &crate::Dbconn) -> diesel::result::QueryResult<usize> {
        conn.run(move |c| {
            let command = commands.find(id_).get_result::<Command>(c)?;
            let new_status = !command.activate.unwrap();
            let updated_command = diesel::update(commands.find(id));

            updated_command
                .set(commands::activate.eq(new_status))
                .execute(c)
        })
        .await
    }
}
