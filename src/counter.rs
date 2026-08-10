use poise::serenity_prelude as serenity;

pub struct MemberCounter {
    member_count: i32,
    predicate: Box<dyn Fn(&serenity::Member) -> bool>,
}

impl MemberCounter {
    pub fn from_role(role_id: serenity::RoleId) -> Self {
        Self {
            member_count: 0,
            predicate: Box::new(move |member: &serenity::Member| member.roles.contains(&role_id)),
        }
    }

    pub fn member_joined(&mut self, new_member: &serenity::Member) {
        if (self.predicate)(new_member) {
            self.member_count += 1;
        }
    }

    pub fn member_left(&mut self, new_member: &serenity::Member) {
        if (self.predicate)(new_member) {
            self.member_count -= 1;
        }
    }

    pub fn member_updated(&mut self, member: &serenity::Member) {
        if !(self.predicate)(member) {
            self.member_count -= 1;
        }
    }
}
