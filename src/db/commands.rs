//! Definition of commands record

/* std use */

/* crate use */
use diesel::prelude::*;

/* project use */
use crate::Dbconn;
use super::schema::commands;
use super::schema::commands::dsl::{activate as commands_activate, commands as all_commands};

#[derive(rocket::serde::Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "commands"]
pub struct Command {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub activate: bool,
}

impl Command {
    pub async fn all(conn: &Dbconn) -> diesel::result::QueryResult<Vec<Command>> {
        conn.run(|c| {
            all_commands.order(commands::id.desc()).load::<Command>(c)
        }).await
    }

    pub async fn toggle_activate(id: i32, conn: &Dbconn) -> diesel::result::QueryResult<usize> {
	conn.run(move |c| {
            let command = all_commands.find(id).get_result::<Command>(c)?;
            let new_status = !command.activate;
            let updated_command = diesel::update(all_commands.find(id));
            updated_command.set(commands_activate.eq(new_status)).execute(c)
	}).await
    }

}
