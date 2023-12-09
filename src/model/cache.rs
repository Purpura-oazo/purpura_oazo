use crate::DB;
use serde::{Deserialize, Serialize};
use serenity::all::{Member, UserId};
use std::future::IntoFuture;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    pub id: u64,
    pub accepted_terms: bool,
}

impl UserData {
    pub async fn get(u_id: &UserId) -> Self {
        let user: Option<Self> = DB
            .select(("users".to_owned(), u_id.get().to_string()))
            .into_future()
            .await
            .unwrap();

        match user {
            Some(user) => user,
            None => {
                let u = Self {
                    id: u_id.get(),
                    accepted_terms: false,
                };

                DB.create(("users", u_id.get().to_string()))
                    .content(&u)
                    .await
                    .unwrap()
                    .unwrap()
            }
        }
    }

    pub async fn update_metadata(&self, m: &Member) {
        let data = MemberCache::build(m);
        DB.query("UPDATE $user->member_cache->$server SET permissions = $perm, roles = $roles")
            .bind(("user", m.user.id.get()))
            .bind(("server", m.guild_id.get()))
            .bind(("perm", &data.permissions))
            .bind(("roles", &data.roles))
            .await
            .unwrap();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberCache {
    pub roles: Vec<u64>,
    pub permissions: Option<u64>,
    #[serde(rename = "in")]
    pub user_id: u64,
    #[serde(rename = "out")]
    pub guild_id: u64,
}

impl MemberCache {
    pub fn build(m: &Member) -> Self {
        let roles: Vec<u64> = m.roles.iter().map(|r| r.get()).collect();
        let permissions: Option<u64> = m.permissions.map(|p| p.bits());

        Self {
            roles,
            permissions,
            user_id: m.guild_id.get(),
            guild_id: m.user.id.get(),
        }
    }

    pub async fn save(&self) {
        let _: Vec<Self> = DB.create("member_cache").content(self).await.unwrap();
    }
}
