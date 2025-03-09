use crate::dataclasses::{AllowedMention, AllowedMentionType, Snowflake};

macro_rules! switch {
    ($name1:ident, $name2:ident => $at:ident) => {
        pub fn $name1(&mut self) -> &mut Self {
            if !self
                .allowed_mentions
                .parse
                .contains(&AllowedMentionType::$at)
            {
                self.allowed_mentions.parse.push(AllowedMentionType::$at);
            }
            self
        }

        pub fn $name2(&mut self) -> &mut Self {
            if self
                .allowed_mentions
                .parse
                .contains(&AllowedMentionType::$at)
            {
                self.allowed_mentions
                    .parse
                    .retain(|x| *x != AllowedMentionType::$at);
            }
            self
        }
    };
}

pub struct AllowedMentionsBuilder {
    allowed_mentions: AllowedMention,
}

impl AllowedMentionsBuilder {
    pub const fn empty() -> Self {
        Self {
            allowed_mentions: AllowedMention {
                parse: vec![],
                roles: None,
                users: None,
            },
        }
    }

    pub fn roles(roles: Vec<Snowflake>) -> Self {
        Self {
            allowed_mentions: AllowedMention {
                parse: vec![AllowedMentionType::Roles],
                roles: Some(roles),
                users: None,
            },
        }
    }

    pub fn users(users: Vec<Snowflake>) -> Self {
        Self {
            allowed_mentions: AllowedMention {
                parse: vec![AllowedMentionType::Users],
                roles: None,
                users: Some(users),
            },
        }
    }
}

impl AllowedMentionsBuilder {
    pub fn build(self) -> AllowedMention {
        self.allowed_mentions
    }

    switch!(enable_everyone, disable_everyone => Everyone);
    switch!(enable_roles, disable_roles => Roles);
    switch!(enable_users, disable_users => Users);

    pub fn with_users(mut self, users: Vec<Snowflake>) -> Self {
        self.allowed_mentions.users = Some(users);
        self
    }

    pub fn with_roles(mut self, roles: Vec<Snowflake>) -> Self {
        self.allowed_mentions.roles = Some(roles);
        self
    }
}

impl From<AllowedMention> for AllowedMentionsBuilder {
    fn from(value: AllowedMention) -> Self {
        AllowedMentionsBuilder {
            allowed_mentions: value,
        }
    }
}
