use actix_session::Session;
use serde::{Serialize, Deserialize};
use failure::Error;

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    id: u32,
    name: String,
    role: String,
}

impl UserAuth {
    pub fn new(id:u32, name: String, role: String) -> UserAuth {
        UserAuth{
            id,
            name,
            role,
        }
    }

    pub fn get_role(sess:Session, role: &str) -> Result<UserAuth, Error> {
        match sess.get::<UserAuth>(role) {
            Ok(Some(admin)) => Ok(admin),
            Err(e) => {
                e.as_error()
            },
            _ => failure::err_msg(format!("fail get session: {}", role)),
        }
    }

    pub fn save(&self, sess:Session) -> Result<(), Error> {
        match sess.set(self.role.as_str(), self) {
            Ok(_) => Ok(()),
            Err(e) => e.as_error(),
        }
    }
}