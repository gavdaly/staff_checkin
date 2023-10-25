use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub provider: Option<i32>,
    pub phone_number: String,
    pub display_name: Option<String>,
    pub api_id: Option<i32>,
    pub state: State,
    pub role: Role,
    pub settings: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Role {
    Unauthorized = 0,
    NoPermissions = 1,
    Management = 2,
    Admin = 3,
    BusinessStaff = 4,
    Provider = 5,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Inactive = 0,
    Salary = 1,
    Hourly = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub state: Option<i32>,
    pub role: Option<i32>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::*;

    impl UserPublic {
        pub async fn get_all_hourly() -> Result<Vec<Self>, sqlx::Error>  {
            use crate::database;
            let db = database::get_db();
            query_as!(UserPublic, "SELECT id, last_name, first_name, phone_number, role, state From users
                            WHERE state = 2
                            ORDER BY last_name, first_name;").fetch_all(db).await
        }


        pub async fn get(id: Uuid) -> Result<Self, sqlx::Error> {
            use crate::database;
            let db = database::get_db();
            query_as!(UserPublic, "SELECT id, last_name, first_name, phone_number, role, state From users
                            WHERE id = $1
                            ORDER BY last_name, first_name;", id).fetch_one(db).await
        }
    }
}
}

/*
From rails app

has_one :setting, dependent: :destroy
has_one :hour, dependent: :destroy
has_one :pin, dependent: :destroy
has_many :assignations, dependent: :destroy
has_many :corrections, through: :assignations
has_many :adjustments, dependent: :destroy
has_many :messages, dependent: :destroy

validates :phone_number, uniqueness: true, presence: true

after_initialize :set_defaults, if: :new_record?
after_save :set_default_associations, if: :new_record?

scope :filtered, -> { select(%i[first_name id  last_name phone_number role state]) }

scope :timesheet_all_dates, -> { includes(:adjustments, :assignations, :corrections) }
scope :ass, ->(date_range) { joins(:assignations).merge(Assignation.for(date_range)) }
scope :adjust, ->(date_range) { joins(:adjustments).merge(Adjustment.for(date_range)) }

def instantiate_pin
  if self.pin
    self.pin = nil
    self.save
  end
  self.create_pin
  self.pin.save
end

private

def set_defaults
  set_default_role
  set_default_state
end

def set_default_associations
  set_default_settings
  set_default_hours
end
*/
