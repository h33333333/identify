pub mod create_user;

pub struct UserUseCaseDeps<'a, R> {
    repository: &'a R,
}
