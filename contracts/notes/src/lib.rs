#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

// ═══════════════════════════════════════════════════════════
//  STORAGE KEYS
// ═══════════════════════════════════════════════════════════

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Initialized,
    Student(Address),
    StudentList,
    Badge(u32),
    BadgeCounter,
    StudentBadge(Address, u32),
    Event(u32),
    EventCounter,
    EventClaim(Address, u32),
    Admin(Address),
}

// ═══════════════════════════════════════════════════════════
//  STRUCTS
// ═══════════════════════════════════════════════════════════

#[contracttype]
#[derive(Clone, Debug)]
pub struct Student {
    pub username: String,
    pub xp: u32,
    pub level: u32,
    pub reputation: u32,
    pub badge_count: u32,
    pub rare_badge_count: u32,
    pub joined_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Badge {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub rarity: String, // "Common" | "Rare" | "Epic" | "Legendary"
    pub xp_required: u32,
}

// ═══════════════════════════════════════════════════════════
//  UTILS
// ═══════════════════════════════════════════════════════════

fn calculate_level(xp: u32) -> u32 {
    let base = xp / 100;
    let sqrt = integer_sqrt(base);
    let level = sqrt + 1;
    if level > 10 { 10 } else { level }
}

fn calculate_reputation(level: u32, badge_count: u32, rare_badge_count: u32) -> u32 {
    let level_pts = level.saturating_mul(10);
    let badge_pts = badge_count.saturating_mul(5);
    let rare_pts  = rare_badge_count.saturating_mul(20);
    level_pts.saturating_add(badge_pts).saturating_add(rare_pts)
}

fn integer_sqrt(n: u32) -> u32 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

// ═══════════════════════════════════════════════════════════
//  CONTRACT
// ═══════════════════════════════════════════════════════════

#[contract]
pub struct SkillQuestContract;

#[contractimpl]
impl SkillQuestContract {

    // ───────────────────────────────────────────────────────
    //  INITIALIZE
    // ───────────────────────────────────────────────────────

    pub fn initialize(env: Env, deployer: Address) {
        deployer.require_auth();
        if env.storage().instance().has(&DataKey::Initialized) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin(deployer.clone()), &true);
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().set(&DataKey::BadgeCounter, &0u32);
        env.storage().instance().set(&DataKey::EventCounter, &0u32);
        env.storage().instance().set(&DataKey::StudentList, &Vec::<Address>::new(&env));
    }

    // ───────────────────────────────────────────────────────
    //  REGISTER
    // ───────────────────────────────────────────────────────

    pub fn register(env: Env, wallet: Address, username: String) {
        wallet.require_auth();
        if env.storage().instance().has(&DataKey::Student(wallet.clone())) {
            panic!("Already registered");
        }
        let student = Student {
            username,
            xp: 0,
            level: 1,
            reputation: 10,
            badge_count: 0,
            rare_badge_count: 0,
            joined_at: env.ledger().timestamp(),
        };
        env.storage().instance().set(&DataKey::Student(wallet.clone()), &student);

        let mut list: Vec<Address> = env
            .storage().instance()
            .get(&DataKey::StudentList)
            .unwrap_or(Vec::new(&env));
        list.push_back(wallet);
        env.storage().instance().set(&DataKey::StudentList, &list);
    }

    // ───────────────────────────────────────────────────────
    //  PROFILE
    // ───────────────────────────────────────────────────────

    pub fn get_student(env: Env, wallet: Address) -> Student {
        env.storage().instance()
            .get(&DataKey::Student(wallet))
            .expect("Student not found")
    }

    pub fn is_registered(env: Env, wallet: Address) -> bool {
        env.storage().instance().has(&DataKey::Student(wallet))
    }

    // ───────────────────────────────────────────────────────
    //  XP & LEVEL
    // ───────────────────────────────────────────────────────

    pub fn add_xp(env: Env, admin: Address, student_wallet: Address, amount: u32) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let mut student: Student = env.storage().instance()
            .get(&DataKey::Student(student_wallet.clone()))
            .expect("Student not found");

        student.xp         = student.xp.saturating_add(amount);
        student.level      = calculate_level(student.xp);
        student.reputation = calculate_reputation(student.level, student.badge_count, student.rare_badge_count);

        env.storage().instance().set(&DataKey::Student(student_wallet), &student);
    }

    // ───────────────────────────────────────────────────────
    //  BADGE SYSTEM
    // ───────────────────────────────────────────────────────

    pub fn create_badge(
        env: Env,
        admin: Address,
        name: String,
        description: String,
        rarity: String,
        xp_required: u32,
    ) -> u32 {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let badge_id: u32 = env.storage().instance()
            .get(&DataKey::BadgeCounter)
            .unwrap_or(0u32) + 1;

        let badge = Badge { id: badge_id, name, description, rarity, xp_required };
        env.storage().instance().set(&DataKey::Badge(badge_id), &badge);
        env.storage().instance().set(&DataKey::BadgeCounter, &badge_id);

        badge_id
    }

    pub fn unlock_badge(env: Env, admin: Address, student_wallet: Address, badge_id: u32) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let badge: Badge = env.storage().instance()
            .get(&DataKey::Badge(badge_id))
            .expect("Badge not found");

        let owned_key = DataKey::StudentBadge(student_wallet.clone(), badge_id);
        if env.storage().instance().has(&owned_key) {
            panic!("Badge already owned");
        }
        env.storage().instance().set(&owned_key, &true);

        let mut student: Student = env.storage().instance()
            .get(&DataKey::Student(student_wallet.clone()))
            .expect("Student not found");

        student.badge_count = student.badge_count.saturating_add(1);

        let is_rare = badge.rarity == String::from_str(&env, "Rare")
            || badge.rarity == String::from_str(&env, "Epic")
            || badge.rarity == String::from_str(&env, "Legendary");

        if is_rare {
            student.rare_badge_count = student.rare_badge_count.saturating_add(1);
        }

        student.reputation = calculate_reputation(student.level, student.badge_count, student.rare_badge_count);
        env.storage().instance().set(&DataKey::Student(student_wallet), &student);
    }

    pub fn has_badge(env: Env, student_wallet: Address, badge_id: u32) -> bool {
        env.storage().instance().has(&DataKey::StudentBadge(student_wallet, badge_id))
    }

    pub fn get_badge(env: Env, badge_id: u32) -> Badge {
        env.storage().instance()
            .get(&DataKey::Badge(badge_id))
            .expect("Badge not found")
    }

    pub fn get_badge_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::BadgeCounter).unwrap_or(0)
    }

    // ───────────────────────────────────────────────────────
    //  EVENT SYSTEM
    // ───────────────────────────────────────────────────────

    pub fn create_event(env: Env, admin: Address, title: String, xp_reward: u32) -> u32 {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let event_id: u32 = env.storage().instance()
            .get(&DataKey::EventCounter)
            .unwrap_or(0u32) + 1;

        env.storage().instance().set(&DataKey::Event(event_id), &(title, xp_reward, true));
        env.storage().instance().set(&DataKey::EventCounter, &event_id);

        event_id
    }

    pub fn complete_event(env: Env, admin: Address, student_wallet: Address, event_id: u32) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let (_, xp_reward, active): (String, u32, bool) = env.storage().instance()
            .get(&DataKey::Event(event_id))
            .expect("Event not found");

        if !active {
            panic!("Event is not active");
        }

        let claim_key = DataKey::EventClaim(student_wallet.clone(), event_id);
        if env.storage().instance().has(&claim_key) {
            panic!("Already completed this event");
        }
        env.storage().instance().set(&claim_key, &true);

        let mut student: Student = env.storage().instance()
            .get(&DataKey::Student(student_wallet.clone()))
            .expect("Student not found");

        student.xp         = student.xp.saturating_add(xp_reward);
        student.level      = calculate_level(student.xp);
        student.reputation = calculate_reputation(student.level, student.badge_count, student.rare_badge_count);

        env.storage().instance().set(&DataKey::Student(student_wallet), &student);
    }

    pub fn get_event_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::EventCounter).unwrap_or(0)
    }

    // ───────────────────────────────────────────────────────
    //  LEADERBOARD
    // ───────────────────────────────────────────────────────

    pub fn get_leaderboard(env: Env, limit: u32) -> Vec<(Address, Student)> {
        let list: Vec<Address> = env.storage().instance()
            .get(&DataKey::StudentList)
            .unwrap_or(Vec::new(&env));

        let mut entries: Vec<(Address, Student)> = Vec::new(&env);

        for addr in list.iter() {
            if let Some(student) = env.storage().instance()
                .get::<DataKey, Student>(&DataKey::Student(addr.clone()))
            {
                entries.push_back((addr.clone(), student));
            }
        }

        // Bubble sort descending by XP
        let len = entries.len();
        for i in 0..len {
            for j in 0..len.saturating_sub(i + 1) {
                let (_, a) = entries.get(j).unwrap();
                let (_, b) = entries.get(j + 1).unwrap();
                if a.xp < b.xp {
                    let tmp_a = entries.get(j).unwrap();
                    let tmp_b = entries.get(j + 1).unwrap();
                    entries.set(j, tmp_b);
                    entries.set(j + 1, tmp_a);
                }
            }
        }

        let cap = if limit as usize > entries.len() as usize { entries.len() } else { limit };
        let mut result: Vec<(Address, Student)> = Vec::new(&env);
        for i in 0..cap {
            result.push_back(entries.get(i).unwrap());
        }
        result
    }

    // ───────────────────────────────────────────────────────
    //  ADMIN
    // ───────────────────────────────────────────────────────

    pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
        caller.require_auth();
        Self::require_admin(&env, &caller);
        env.storage().instance().set(&DataKey::Admin(new_admin), &true);
    }

    pub fn is_admin(env: Env, wallet: Address) -> bool {
        env.storage().instance()
            .get::<DataKey, bool>(&DataKey::Admin(wallet))
            .unwrap_or(false)
    }

    // ───────────────────────────────────────────────────────
    //  INTERNAL
    // ───────────────────────────────────────────────────────

    fn require_admin(env: &Env, caller: &Address) {
        let ok: bool = env.storage().instance()
            .get(&DataKey::Admin(caller.clone()))
            .unwrap_or(false);
        if !ok {
            panic!("Unauthorized: not admin");
        }
    }
}
