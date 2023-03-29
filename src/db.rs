use sea_orm::DbErr;

#[derive(Debug)]
pub struct DbErrWrapper(DbErr);

impl DbErrWrapper {
    pub fn new(err: DbErr) -> Self {
        DbErrWrapper(err)
    }
}
impl From<DbErr> for DbErrWrapper {
    fn from(err: DbErr) -> Self {
        DbErrWrapper::new(err)
    }
}

impl From<DbErrWrapper> for &str {
    fn from(_: DbErrWrapper) -> &'static str {
        "db error ocured"
    }
}
