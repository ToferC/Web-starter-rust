use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::schema::creatures;
use crate::database;

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{QueryDsl};

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Associations, Identifiable, AsChangeset, Clone)]
#[table_name = "creatures"]
pub struct Creature {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_name: String,
    pub found_in: Locales,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dex: u32,
    pub strength: u32,
    pub con: u32,
    pub per: u32,
    pub wil: u32,
    pub cha: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconsciousness_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub slug: String,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Creature {
    pub fn create(creature_data: &InsertableCreature) -> Result<Self> {
        let mut conn = database::connection()?;
        let res = diesel::insert_into(creatures::table)
            .values(creature_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(creature: &InsertableCreature) -> Result<Self> {
        let mut conn = connection()?;
        let res = creatures::table
            .filter(creatures::creature_name.eq(&creature.creature_name))
            .distinct()
            .first(&mut conn);

        let creature = match res {
            Ok(c) => c,
            Err(e) => {
                // creature not found
                println!("{:?}", e);
                let c = Creature::create(creature).expect("Unable to create creature");
                c
            }
        };
        Ok(creature)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::creature_name.ilike(format!("%{}%", name)))
            .load::<Creature>(&mut conn)?;
    }

    pub fn get_by_slug(name: &String) -> Result<Self> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::slug.eq(slug))
            .first::<Creature>(&mut conn)?;
    }

    pub fn update(&mut self) -> Result<Self> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(creatures::table)
            .filter(creatures::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd)]
#[diesel(table_name = rarities)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

#[derive(Serialize, Deserialize, Debug, PartialOrd)]
#[diesel(table_name = locales)]
pub enum Locales {
    Jungle,
    Desert,
    Forest,
    Plains,
    Urban,
    Mountain,
    Cavern,
    Kaer,
    Any,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, InputObject, Queryable)]
/// Referenced by Roles, TeamOwnership, OrgOwnership
#[diesel(table_name = creatures)]
pub struct InsertableCreature {
    pub creator_id: Uuid,
    pub creature_name: String,
    pub found_in: Locales,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dex: u32,
    pub strength: u32,
    pub con: u32,
    pub per: u32,
    pub wil: u32,
    pub cha: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconscious_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub slug: String,
    pub image_url: String,
}

impl InsertableCreature {

    pub fn default(creator_id: Uuid) -> Self {

        let locales = Locales::Jungle;
        let today = chrono::Utc::now().naive_utc();

        Creature {
            creator_id,
            creature_name: "Esparaga".to_string(),
            found_in: locales,
            rarity: Rarity::Rare,
            circle_rank: 5,
            dex: 10,
            strength: 10,
            con: 10,
            per: 10,
            wil: 10,
            cha: 10,
            initiative: 10,
            pd: 9,
            md: 9,
            sd: 9,
            pa: 9,
            ma: 9,
            unconsciousness_rating: 45,
            death_rating: 55,
            wound: 12,
            knockdown: 10,
            actions: 2,
            recovery_rolls: 3,
            image_url: "hdahdksfashf".to_string(),
            slug: "esparaga".to_owned(),
            created_at: today,
            updated_at: today,
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_name: String,
        found_in: Vec<Locales>,
        rarity: Rarity,
        circle_rank: u32,
        dex: u32,
        strength: u32,
        con: u32,
        per: u32,
        wil: u32,
        cha: u32,
        initiative: u32,
        pd: u32,
        md: u32,
        sd: u32,
        pa: u32,
        ma: u32,
        unconscious_rating: u32,
        death_rating: u32,
        wound: u32,
        knockdown: u32,
        actions: u32,
        recovery_rolls: u32,
    ) -> Self {

        let slug = creature_name.trim().to_snake_case();

        InsertableCreature {
            creator_id,
            creature_name,
            found_in,
            rarity,
            circle_rank,
            dex,
            strength,
            con,
            per,
            wil,
            cha,
            initiative,
            pd,
            md,
            sd,
            pa,
            ma,
            unconscious_rating,
            death_rating,
            wound,
            knockdown,
            actions,
            recovery_rolls,
            slug,
            "default_image_url".to_owned(),
        }
    }
}

