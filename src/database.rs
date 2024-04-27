// database.rs
use rusqlite::{Connection, Result};

use serde_json::{from_str, to_string};
use spider::{ImgDetail, Picture};

// use crate::{ImgDetail, Picture};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_file: &str) -> Result<Self> {
        let conn = Connection::open(db_file)?;
        // 创建 picture 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pictures (
                  id              INTEGER PRIMARY KEY,
                  title             TEXT NOT NULL,
                  url              TEXT NOT NULL,
                  srcs             TEXT NOT NULL,
                  star             INTEGER NOT NULL,
                  collect          BOOLEAN NOT NULL,
                  download         BOOLEAN NOT NULL,
                  deleted           BOOLEAN NOT NULL
                  )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn insert_picture(&self, picture: Picture) -> Result<()> {
        let serialized_srcs = to_string(&picture.srcs).unwrap_or_default();
        self.conn.execute(
            "INSERT INTO pictures (id, title, url, srcs, star, collect, download, deleted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&picture.id as &dyn rusqlite::ToSql, &picture.title, &picture.url, &serialized_srcs, &picture.star, &picture.collect, &picture.download, &picture.deleted],
        )?;
        Ok(())
    }

    pub fn get_all_pictures(&self) -> Result<Vec<Picture>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, url, srcs, star, collect, download, deleted FROM pictures",
        )?;
        let picture_iter = stmt.query_map([], |row| {
            let srcs_str: String = row.get(3)?;
            let srcs: Vec<ImgDetail> = from_str(&srcs_str).unwrap_or_default();

            Ok(Picture {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                srcs,
                star: row.get(4)?,
                collect: row.get(5)?,
                download: row.get(6)?,
                deleted: row.get(7)?,
            })
        })?;

        let mut pictures = Vec::new();
        for picture in picture_iter {
            pictures.push(picture?);
        }
        Ok(pictures)
    }
}
