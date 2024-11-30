use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::runs;
use super::models::{Run, NewRun};

pub struct RunRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> RunRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        RunRepository { conn }
    }

    pub fn create(&mut self, new_run: NewRun) -> Result<Run, diesel::result::Error> {
        diesel::insert_into(runs::table)
            .values(&new_run)
            .get_result(self.conn)
    }

    pub fn mark_completed(&mut self, run_id: i32) -> Result<(), diesel::result::Error> {
        diesel::update(runs::table.find(run_id))
            .set((
                runs::status.eq("completed"),
                runs::end_time.eq(diesel::dsl::now),
            ))
            .execute(self.conn)?;
        Ok(())
    }
} 