use egg_mode;
use egg_mode::tweet;

use crate::config::Config;

pub struct TwitterApi {
    token: egg_mode::Token,
    user_id: egg_mode::user::UserID,
}

impl TwitterApi {

    pub fn new(config: &Config) -> Self {
        Self {
            token: Self::build_token(config),
            user_id: Self::user_id(config),
        }
    }

    fn build_token(config: &Config) -> egg_mode::Token {
        let consumer = egg_mode::KeyPair::new(config.api.app.key.clone(), config.api.app.secret.clone());
        let access = egg_mode::KeyPair::new(config.api.user.token.clone(), config.api.user.token_secret.clone());
        egg_mode::Token::Access { consumer, access }
    }

    fn user_id(config: &Config) -> egg_mode::user::UserID {
        egg_mode::user::UserID::from(config.api.user.display_name.clone())
    }

    pub async fn delete_tweet(&self, tw_id: u64) -> egg_mode::error::Result<u64> {
        tweet::delete(tw_id, &self.token).await
            .map(|r| r.response.id)
    }

    pub fn get_initial_timeline(&self) -> tweet::Timeline {
        tweet::user_timeline(self.user_id.clone(), true, true, &self.token)
    }

}
