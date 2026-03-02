pub mod create_user;
pub mod get_user;

pub struct UserUseCaseDeps<'a, R> {
    pub repository: &'a R,
}
