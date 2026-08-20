use crate::{Error, effect::CounterEffect};
use poise::serenity_prelude as serenity;

pub struct MemberCounter {
    member_count: u32,
    predicate: Box<dyn Fn(&serenity::Member) -> bool + Send + Sync>,
    effects: Vec<CounterEffect>,
}

impl MemberCounter {
    pub fn from_role(role_id: serenity::RoleId, effects: Vec<CounterEffect>) -> Self {
        Self {
            member_count: 0,
            predicate: Box::new(move |member: &serenity::Member| member.roles.contains(&role_id)),
            effects,
        }
    }

    pub fn member_joined(&mut self, ctx: &serenity::Context, new_member: &serenity::Member) {
        if (self.predicate)(new_member) {
            self.member_count += 1;
            self.run_effects(ctx);
        }
    }

    pub fn member_left(&mut self, ctx: &serenity::Context, new_member: &serenity::Member) {
        if (self.predicate)(new_member) {
            self.member_count -= 1;
            self.run_effects(ctx);
        }
    }

    pub fn member_updated(
        &mut self,
        ctx: &serenity::Context,
        member: &serenity::Member,
        old_member: &serenity::Member,
    ) {
        let previous_count = self.member_count;

        if (self.predicate)(member) {
            self.member_count += 1;
        }

        if (self.predicate)(old_member) {
            self.member_count -= 1;
        }

        if self.member_count != previous_count {
            self.run_effects(ctx);
        }
    }

    pub fn refresh_count(&mut self, ctx: &serenity::Context, members: &[serenity::Member]) {
        let previous_count = self.member_count;

        self.member_count = members.iter().fold(0, |acc, member| {
            if (self.predicate)(member) {
                acc + 1
            } else {
                acc
            }
        });

        if self.member_count != previous_count {
            self.run_effects(ctx);
        }
    }

    pub fn get_count(&mut self) -> u32 {
        self.member_count
    }

    pub fn run_effects(&mut self, ctx: &serenity::Context) {
        for effect in &self.effects {
            let effect = effect.clone();
            let count = self.member_count;
            let http = ctx.http.clone();

            tokio::spawn(async move {
                if let Err(error) = effect.handle_update(http, count).await {
                    println!("Error occured during effect: {:?}", error);
                }
            });
        }
    }
}
