use std::collections::HashMap;
extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

enum Gender{
    Female,
    Male,
}

enum RelationshipTypes{
    Father,
    Mother,
    Mate,
    Son,
    Daughter,
    Grandpa,
    Grandma,
    Grandson,
    Grandondaughter,
}

enum Skills{
    Construction,
    Healing,
    Cooking,
    Shooting,
    Combat,
    Farming,
    Mining,
    Manufacturing,
    Sociability,
}

enum Actions{
    Shoot,
    Combat,
    Cutdown,
    Talk,
    Construct,
    Heal,
    Cook,
    Farm,
    Manufacture,
    Move,
}

enum Equipment{
    ax,
    revolver,
    shotgun,
    assault_rifle,
    snipe_rifle,
}

enum Health{
    healthy,
    ill,
}
struct Location{
    x: i8,
    y: i8,
} 
impl Location{
    pub fn new(x:i8, y:i8)->Self{
        Self { x: (x), y: (y) }
    }
}

pub struct Person{
    age: i32,
    health: Health,
    sex: Gender,
    //social_relationship: HashMap<RelationshipTypes,String>,
    skills: HashMap<Skills, usize>,
    location: Location,
    equipment: Equipment,
}

// Actions

impl Person{
    fn new() -> Self{
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(0..80);
        let health_condition = rng.gen::<bool>();
        let health = match health_condition {
            TRUE => Health::healthy,
            FALSE => Health::ill,
        };
        let sex_condition = rng.gen::<bool>();
        let sex = match sex_condition {
            TRUE => Gender::Male,
            FALSE => Gender::Female,
        };
        let mut skills = HashMap::new();
        let mut location = Location { x: (0), y: (0) };
        let mut equipment = Equipment::revolver;
        Self{
            age,
            health,
            sex,
            skills,
            location,
            equipment
        }
    }
    
}

pub struct Player{
    age: i32,
    health: Health,
    sex: Gender,
    skills: HashMap<Skills, usize>,
    location: Location,
    equipment: Equipment,
    inventory: Vec<Items>,  //TODO 用dict存储更合理
}

impl Player{
    fn new() -> Self{
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(0..80);
        let health_condition = rng.gen::<bool>();
        let health = match health_condition {
            TRUE => Health::healthy,
            FALSE => Health::ill,
        };
        let sex_condition = rng.gen::<bool>();
        let sex = match sex_condition {
            TRUE => Gender::Male,
            FALSE => Gender::Female,
        };
        let mut skills = HashMap::new();
        let mut location = Location { x: (0), y: (0) };
        let mut equipment = Equipment::revolver;
        let mut inventory = vec![Items::Food,Items::Medicine];
        Self{
            age,
            health,
            sex,
            skills,
            location,
            equipment,
            inventory
        }
    }
    
}


enum Items{
    Food,
    Medicine,
    Arms,
    Money,
}