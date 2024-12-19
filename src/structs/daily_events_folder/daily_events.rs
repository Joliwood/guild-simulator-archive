use super::{
    discussions::get_random_discussion_indexs,
    spontaneous_applications::get_random_spontaneous_application_indexs,
};
use crate::{
    content::daily_events::{
        discussions::{get_all_daily_discussions, get_daily_discussion},
        spontaneous_applications::{get_all_spontaneous_applications, get_spontaneous_application},
    },
    structs::{equipments::ItemEnum, player_stats::SKIP_TUTO, recruits::RecruitStats},
};
use bevy::prelude::*;
use rand::Rng;

pub fn calculate_total_apparition_chance(list: &[u16]) -> u16 {
    let mut total = 0;
    let mut i = 0;
    while i < list.len() {
        total += list[i];
        i += 1;
    }
    return total;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImpactAction<T> {
    Add(T),
    Remove(T),
}

#[derive(Default, Debug, Component, Resource, Clone, PartialEq)]
pub struct Answer {
    pub equipment_impact: Option<Vec<ImpactAction<ItemEnum>>>,
    pub experience_impact: Option<u32>,
    pub gold_impact: Option<i32>,
    pub id: u16,
    pub message: String,
    pub recruit_impact: Option<RecruitStats>,
    pub reputation_impact: Option<i8>,
    pub toxicity_impact: Option<i8>,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyEventTargets {
    pub daily_discussion_targets: Vec<DiscussionTarget>,
    pub daily_spontaneous_application_targets: Vec<SpontaneousApplicationTarget>,
}

impl Default for DailyEventTargets {
    fn default() -> Self {
        let discussions = get_all_daily_discussions();
        let spontaneous_applications = get_all_spontaneous_applications();
        return Self {
            daily_discussion_targets: discussions
                .into_iter()
                .map(|discussion| DiscussionTarget {
                    percent_chance: discussion.apparition_chance,
                    index: discussion.id,
                    day_system: discussion.day_system,
                })
                .collect(),
            daily_spontaneous_application_targets: spontaneous_applications
                .into_iter()
                .map(|discussion| SpontaneousApplicationTarget {
                    percent_chance: discussion.apparition_chance,
                    index: discussion.id,
                    day_system: discussion.day_system,
                })
                .collect(),
        };
    }
}

impl DailyEventTargets {
    pub fn update_min_day_for_spontaneous_application_by_index(&mut self, index: u16) {
        let target = self
            .daily_spontaneous_application_targets
            .iter_mut()
            .find(|target| target.index == index)
            .unwrap();

        target.day_system.update_min_day();
    }

    pub fn update_min_day_for_discussion_by_index(&mut self, index: u16) {
        let target = self
            .daily_discussion_targets
            .iter_mut()
            .find(|target| target.index == index)
            .unwrap();

        target.day_system.update_min_day();
    }
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DiscussionTarget {
    pub percent_chance: u16,
    pub index: u16,
    pub day_system: DaySystem,
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct SpontaneousApplicationTarget {
    pub percent_chance: u16,
    pub index: u16,
    pub day_system: DaySystem,
}

pub fn get_random_index_from_percent_arr(list: &[u16]) -> usize {
    let total_apparition_chance = calculate_total_apparition_chance(list);

    let random_number = rand::thread_rng().gen_range(0..total_apparition_chance);
    let mut cumulative = 0;

    for (index, &chance) in list.iter().enumerate() {
        cumulative += chance;
        if random_number <= cumulative {
            return index;
        }
    }

    // Default to the last element if none were selected (shouldn't happen)
    return list.len() - 1;
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DaySystem {
    pub cooldown: u16,
    pub max_day: Option<u16>,
    pub min_day: u16,
}

impl DaySystem {
    pub fn update_min_day(&mut self) {
        self.min_day += self.cooldown;
    }
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub enum DailyEventTypeEnum {
    Discussion(u16),
    // EquipmentTrade(u16),
    // MapProposition(u16),
    SpontaneousApplication(u16),
}

#[derive(Debug, Component, Resource, Clone, PartialEq)]
pub struct DailyEvent {
    pub daily_event_type: DailyEventTypeEnum,
    pub day_system: DaySystem,
}

#[derive(Debug, Component, Resource, Clone)]
pub struct DailyEvents(pub Vec<DailyEvent>);

// Contents
impl Default for DailyEvents {
    fn default() -> Self {
        let discussion_ids = [9, 7, 8];
        let spontaneous_application_ids = [2, 1];

        let daily_events = spontaneous_application_ids
            .iter()
            .map(|&id| {
                let application = get_spontaneous_application(&id);
                DailyEvent {
                    daily_event_type: DailyEventTypeEnum::SpontaneousApplication(application.id),
                    day_system: application.day_system,
                }
            })
            .chain(discussion_ids.iter().map(|&id| {
                let discussion = get_daily_discussion(&id);
                DailyEvent {
                    daily_event_type: DailyEventTypeEnum::Discussion(discussion.id),
                    day_system: discussion.day_system,
                }
            }))
            .collect();

        if SKIP_TUTO {
            return Self(vec![]);
        }

        return Self(daily_events);
    }
}

impl DailyEvents {
    pub fn get_last_daily_event(&self) -> Option<&DailyEvent> {
        self.0.last()
    }

    pub fn get_random_number_of_daily_events(
        &self,
        n: usize,
        player_day: u16,
        daily_event_targets: &mut ResMut<DailyEventTargets>,
    ) -> Vec<DailyEvent> {
        let mut daily_events: Vec<DailyEvent> = Vec::new();
        let mut daily_discussion_number = 0;
        let mut daily_spontaneous_application_number = 0;

        // Discussion has a 80% chance of being selected.
        // Spontaneous application has a 20% chance of being selected.

        for _ in 0..n {
            let random_number = rand::thread_rng().gen_range(0..100);
            if random_number <= 80 {
                daily_discussion_number += 1;
            } else {
                daily_spontaneous_application_number += 1;
            }
        }

        let random_discussion_indexs =
            get_random_discussion_indexs(daily_discussion_number, player_day, daily_event_targets);

        for random_discussion_index in random_discussion_indexs {
            let daily_discussion = get_daily_discussion(&random_discussion_index);
            let daily_event = DailyEvent {
                daily_event_type: DailyEventTypeEnum::Discussion(random_discussion_index),
                day_system: daily_discussion.day_system.clone(),
            };
            daily_events.push(daily_event);
        }

        let daily_sponaneous_application_indexs = get_random_spontaneous_application_indexs(
            daily_spontaneous_application_number,
            player_day,
            daily_event_targets,
        );

        for daily_sponaneous_application_index in daily_sponaneous_application_indexs {
            let daily_spontaneous_application =
                get_spontaneous_application(&daily_sponaneous_application_index);
            let daily_event = DailyEvent {
                daily_event_type: DailyEventTypeEnum::SpontaneousApplication(
                    daily_sponaneous_application_index,
                ),
                day_system: daily_spontaneous_application.day_system.clone(),
            };
            daily_events.push(daily_event);
        }

        return daily_events;
    }

    pub fn remove_daily_discussion_by_id(&mut self, discussion_id: u16) {
        self.0.retain(|event| match &event.daily_event_type {
            DailyEventTypeEnum::Discussion(id) => id != &discussion_id,
            _ => true,
        });
    }

    pub fn remove_spontaneous_application_by_id(&mut self, application_id: u16) {
        self.0.retain(|event| match &event.daily_event_type {
            DailyEventTypeEnum::SpontaneousApplication(id) => id != &application_id,
            _ => true,
        });
    }
}
