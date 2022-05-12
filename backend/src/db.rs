use rusqlite::{params_from_iter, Connection, Row};

pub trait FromRow: Sized {
    fn from_row<'row, 'stmt: 'row>(row: &'row Row<'stmt>) -> Result<Self, rusqlite::Error>;
}

pub trait Scannable: FromRow {
    fn select_all() -> String;

    fn post_load_hook(&mut self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn scan(conn: &mut Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(&Self::select_all())?;
        eprintln!("exec: {:?}", &Self::select_all());
        let res = stmt.query_and_then([], |row| {
            let mut rc = Self::from_row(row)?;
            rc.post_load_hook(conn)?;
            Ok(rc)
        })?;

        res.collect()
    }
}

pub trait Loadable: FromRow {
    type Id;

    fn post_load_hook(&mut self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn select_by() -> String;

    fn select_params(id: &Self::Id) -> Vec<(&str, &dyn rusqlite::ToSql)>;

    fn load(id: Self::Id, conn: &Connection) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(&Self::select_by())?;

        let mut res = stmt.query_and_then::<_, rusqlite::Error, _, _>(
            Self::select_params(&id).as_slice(),
            |row| {
                let mut rc = Self::from_row(row)?;
                rc.post_load_hook(conn)?;
                Ok(rc)
            },
        )?;

        if let Some(Ok(r)) = res.next() {
            if res.next().is_some() {
                panic!("load returned multiple values");
            }
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
}

pub trait LoadableBy<T>: FromRow {
    fn post_load_hook(&mut self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn select_by(by: &T) -> (String, Vec<(&str, &dyn rusqlite::ToSql)>);

    fn load_by(by: &T, conn: &Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let (sql, params) = Self::select_by(by);
        eprintln!("exec: {:?}", sql);
        let mut stmt = conn.prepare(&sql)?;

        let res = stmt.query_and_then(params.as_slice(), |row| {
            let mut rc = Self::from_row(row)?;
            rc.post_load_hook(conn)?;
            Ok(rc)
        })?;

        res.collect()
    }
}

pub trait Saveable: Sized {
    fn table_name() -> String;

    fn key_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;
    fn data_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;

    fn post_save_hook(&self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn save(self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        let sp = conn.savepoint()?;
        self.__save_no_savepoint(&sp)?;
        sp.commit()
    }

    fn __save_no_savepoint(self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut all_columns = self.key_columns();
        all_columns.append(&mut self.data_columns());

        let sql = format!(
            "INSERT INTO {}({}) VALUES({}) ON CONFLICT({}) DO UPDATE SET {}",
            Self::table_name(),
            all_columns
                .iter()
                .map(|(k, _)| *k)
                .collect::<Vec<_>>()
                .join(", "),
            all_columns
                .iter()
                .enumerate()
                .map(|(i, _)| format!("?{}", i + 1))
                .collect::<Vec<_>>()
                .join(", "),
            self.key_columns()
                .iter()
                .map(|(k, _)| *k)
                .collect::<Vec<_>>()
                .join(", "),
            self.data_columns()
                .iter()
                .map(|(k, _)| format!("{k}=excluded.{k}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        eprintln!("exec: {:?}", sql);
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute(params_from_iter(all_columns.iter().map(|(_, v)| v)))?;
        self.post_save_hook(conn)?;

        Ok(())
    }
}

pub trait Obj: FromRow {
    type Id;

    fn post_load_hook(&mut self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn post_save_hook(&self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        Ok(())
    }

    fn table_name() -> String;
    fn where_clause() -> String;

    fn where_params(id: &Self::Id) -> Vec<(&str, &dyn rusqlite::ToSql)>;

    fn key_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;
    fn data_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;
}

impl<T> Loadable for T
where
    T: Obj,
{
    type Id = T::Id;

    fn post_load_hook(&mut self, conn: &Connection) -> Result<(), rusqlite::Error> {
        self.post_load_hook(conn)
    }

    fn select_by() -> String {
        format!(
            "SELECT * FROM {} WHERE {}",
            T::table_name(),
            T::where_clause()
        )
    }

    fn select_params(id: &Self::Id) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        T::where_params(id)
    }
}

impl<T> Scannable for T
where
    T: Obj,
{
    fn post_load_hook(&mut self, conn: &Connection) -> Result<(), rusqlite::Error> {
        self.post_load_hook(conn)
    }

    fn select_all() -> String {
        format!("SELECT * FROM {}", T::table_name())
    }
}

impl<T> Saveable for T
where
    T: Obj,
{
    fn post_save_hook(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        self.post_save_hook(conn)
    }

    fn table_name() -> String {
        T::table_name()
    }

    fn key_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        self.key_columns()
    }

    fn data_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        self.data_columns()
    }
}
