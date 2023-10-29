
use chrono::NaiveDate;
use std::collections::BTreeMap;
use uuid::Uuid;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use crate::models::{
    adjustments::Adjustment, sessions::Session};

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeSheet {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
    pub entries: BTreeMap<NaiveDate, Vec<Entry>>,
    pub summary: BTreeMap<NaiveDate, (i64, i64, i64, i64)>
}

#[derive(Clone, Deserialize, Serialize)]
        pub enum Entry {
            Session(Session),
            Adjustment(Adjustment),
        }

impl TimeSheet {
    
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::{
             corrections::Correction,  user::UserPublic, sessions::get_sessions_for, adjustments::get_adjustments_for
        };
        use chrono::{Weekday, Duration, Utc, DateTime, NaiveTime};

        #[derive(Clone,  Deserialize, Serialize)]
        struct InputValues {
            user: UserPublic,
            sessions: Vec<Session>,
            corrections: Vec<Correction>,
            adjustments: Vec<Adjustment>,
        }
        
        
        impl TimeSheet {
            pub async fn generate_for(user_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self, sqlx::Error> {
                let midnitght = NaiveTime::default();
                let values = InputValues {
                    user: UserPublic::get(user_id).await?,
                    sessions: get_sessions_for(&user_id, DateTime::from_naive_utc_and_offset(start_date.and_time(midnitght), Utc), DateTime::from_naive_utc_and_offset(end_date.and_time(midnitght), Utc)).await?,
                    corrections: vec![],
                    adjustments: get_adjustments_for(&user_id, start_date, end_date).await?
                };
                Ok(Self::from(values))
            }

            fn from(vals: InputValues) -> Self {
                let UserPublic { id, first_name, last_name, phone_number, state} = vals.user;
                let entries = generate_entries(vals.adjustments, vals.sessions);
                let summary = generate_summary(&entries);
                Self {
                    id,first_name, last_name, phone_number, state,
                    entries,
                    summary,
                }
            }
        }


        fn calculate_statuatory_hours(number_of_days: i64, entries: &BTreeMap<NaiveDate, Vec<Entry>>) -> Duration {
            let mut total = Duration::zero();
            entries.iter().for_each(|(date, entries)| {
                for entry in entries {
                    match entry {
                        Entry::Session(s) => {
                            if let Some(end_time) = s.end_time {
                                total =  end_time - s.start_time + total;
                        }
                        }
                        Entry::Adjustment(a) => {
                            match a.category {
                                1 => {total = Duration::milliseconds(a.duration as i64) + total;},
                                _ => {}
                            }
                        }
                    }
                }
            });
            Duration::milliseconds(total.num_milliseconds() / number_of_days)
        }
        
        /// Generates a summary from entries summary is (Entries, Adjustment, Vacation, Statuatory)
        fn generate_summary(entries: &BTreeMap<NaiveDate, Vec<Entry>>) -> BTreeMap<NaiveDate, (i64, i64, i64, i64)> {
            let mut map: BTreeMap<NaiveDate, (i64, i64, i64, i64)> = BTreeMap::new();
        
            entries.iter().for_each(|(date, entries)| {
                let week = date.week(Weekday::Mon).first_day();
                if !map.contains_key(&week) {
                    map.insert(week, (0, 0, 0, 0));
                }
                let Some(totals) = map.get_mut(&week) else {return};
                for entry in entries {
                    match entry {
                        Entry::Session(s) => {
                            if let Some(end_time) = s.end_time {
                                totals.0 =  (end_time - s.start_time).num_milliseconds() + totals.0;
                        }
                        }
                        Entry::Adjustment(a) => {
                            match a.category {
                                1 => {totals.1 += a.duration as i64 ;},
                                2 => {totals.2 += a.duration as i64;},
                                3 => {totals.3 += a.duration as i64;},
                                _ => {}
                            }
                        }
                    }
                }
            });
        
            map
        }
        
        fn generate_entries(adjustments: Vec<Adjustment>, sessions: Vec<Session>) -> BTreeMap<NaiveDate, Vec<Entry>> {
            let mut map: BTreeMap<NaiveDate, Vec<Entry>> = BTreeMap::new();
            adjustments.into_iter().for_each(|adj| {
                let date: NaiveDate = adj.start_date;
                match map.get_mut(&date) {
                    Some(e) => {e.push(Entry::Adjustment(adj));},
                    None => {map.insert(date, vec![Entry::Adjustment(adj)]);}
                };
            });
            sessions.into_iter().for_each(|sess| {
                let date: NaiveDate = sess.start_time.date_naive();
                match map.get_mut(&date) {
                    Some(e) => {e.push(Entry::Session(sess));},
                    None => {map.insert(date, vec![Entry::Session(sess)]);}
                }
            });
            map
        }
}}

