use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    global: Global,
    storage: Storage,
    hideout: Hideout,
    player: Player,
    scav: Scav,
    trader: Trader,
    items: Items,
    raid: Raid,
    events: Events
}

#[derive(Serialize, Deserialize)]
struct Global {
    disable_mod: bool,
    enable_debug_mode: bool
}

#[derive(Serialize, Deserialize)]
struct Storage {
    case: Case,
    secure: Secure
}

#[derive(Serialize, Deserialize)]
struct Case {
    enable: bool,
    ammo_case_rows: u32,
    ammo_case_columns: u32,
    doc_case_rows: u32,
    doc_case_columns: u32,
    dogtag_case_rows: u32,
    dogtag_case_columns: u32,
    gingy_keychain_rows: u32,
    gingy_keychain_columns: u32,
    grenade_case_rows: u32,
    grenade_case_columns: u32,
    injector_case_rows: u32,
    injector_case_columns: u32,
    key_tool_rows: u32,
    key_tool_columns: u32,
    lucky_scav_case_rows: u32,
    lucky_scav_case_columns: u32,
    medicine_case_rows: u32,
    medicine_case_columns: u32,
    mr_holodilnick_rows: u32,
    mr_holodilnick_columns: u32,
    money_case_rows: u32,
    money_case_columns: u32,
    sicc_case_rows: u32,
    sicc_case_columns: u32,
    simple_wallet_rows: u32,
    simple_wallet_columns: u32,
    thicc_item_case_rows: u32,
    thicc_item_case_columns: u32,
    thicc_weapon_case_rows: u32,
    thicc_weapon_case_columns: u32,
    wz_wallet_rows: u32,
    wz_wallet_columns: u32,
}

#[derive(Serialize, Deserialize)]
struct Secure {
    remove_filters: bool,
    size_change_enabled: bool,
    alpha_rows: u32,
    alhpa_columns: u32,
    beta_rows: u32,
    betalpha_columns: u32,
    epsilon_rows: u32,
    epsilon_columns: u32,
    gamma_rows: u32,
    gamma_columns: u32,
    kappa_rows: u32,
    kappa_columns: u32
}

#[derive(Serialize, Deserialize)]
struct Hideout {
    production: Production,
    scav_case: ScavCase,
    areas: Areas,
    misc_modifier: MiscModifier,
}

#[derive(Serialize, Deserialize)]
struct Production {
    enabled: bool,
    production_time_multiplier: f64,
    minutes_for_btc: f64,
    btc_limit: u32,
    water_filter_rate: u32,
    turn_off_dsp_transmitter_fuel_requirement: bool,
}

#[derive(Serialize, Deserialize)]
struct ScavCase {
    enabled: bool,
    return_time_multiplier: f64,
    rouble_cost_multiplier: f64,
}

#[derive(Serialize, Deserialize)]
struct Areas {
    enabled: bool,
    no_construction_requirements: bool,
    construction_time_multiplier: f64,
}

#[derive(Serialize, Deserialize)]
struct MiscModifier {
    enabled: bool,
    generator_speed_without_fuel: f64,
    generator_fuel_flow_rate: f64,
    air_filter_unit_flow_rate: f64,
}

#[derive(Serialize, Deserialize)]
struct Player {
    health: Health,
    xp: Experience,
}

#[derive(Serialize, Deserialize)]
struct Health {
    enable: bool,
    unlimited_stamina: bool,
    no_fall_damage: bool,
    starvation_rate: f64,
    dehydration_rate: f64,
    max_energy: u32,
    max_hydration: u32,
    max_stamina: u32,
    enable_hit_point_modifier: bool,
    head: u32,
    thorax: u32,
    stomach: u32,
    l_arm: u32,
    r_arm: u32,
    l_leg: u32,
    r_leg: u32,
}

#[derive(Serialize, Deserialize)]
struct Experience {
    enable: bool,
    skill_multiplier: f64,
    weapon_skill_multiplier: f64,
}

#[derive(Serialize, Deserialize)]
struct Scav {
    health: Health,
    rep: Reputation,
    hostility: Hostility,
}

#[derive(Serialize, Deserialize)]
struct Reputation {
    enabled: bool,
    friendly_scav_kill: f64,
    boss_scav_kill: f64,
    follower_scav_kill: f64,
    pmc_scav_kill: f64,
    car_coop_exfil_standing: f64,
}

#[derive(Serialize, Deserialize)]
struct Hostility {
    enabled: bool,
    scav: String,
    boss: String,
}

#[derive(Serialize, Deserialize)]
struct Items {
    modifier: Modifier,
}

#[derive(Serialize, Deserialize)]
struct Modifier {
    enabled: bool,
    weapon_malfunction_chance: f64,
    weapon_misfire_chance: f64,
    mag_packing_speed_multiplier: f64,
    examine_time_multiplier: f64,
    max_money_stack: u32,
}

#[derive(Serialize, Deserialize)]
struct Trader {
    level4_traders: bool,
    stock: Stock,
    clothes: Clothes,
    repair: Repair,
    insurance: Insurance,
}

#[derive(Serialize, Deserialize)]
struct Stock {
    enable: bool,
    restock_timer: u32,
    no_purchase_limit: bool,
    min_durability_to_sell: u32,
}

#[derive(Serialize, Deserialize)]
struct Clothes {
    enable: bool,
    free_clothes: bool,
    all_clothes_available: bool,
}

#[derive(Serialize, Deserialize)]
struct Repair {
    enable: bool,
    cost_multiplier: f64,
    no_random_dmg: bool,
    no_wep_dmg: bool,
    no_armor_dmg: bool,
}

#[derive(Serialize, Deserialize)]
struct Insurance {
    enable: bool,
    prapor: InsuranceDetails,
    therapist: InsuranceDetails,
}

#[derive(Serialize, Deserialize)]
struct InsuranceDetails {
    cost_multiplier: f64,
    return_chance_percentage: u32,
    hours_until_return_min: u32,
    hours_until_return_max: u32,
}

#[derive(Serialize, Deserialize)]
struct Raid {
    free_lab_entry: bool,
    always_survive: bool,
    extraction: Extraction,
}

#[derive(Serialize, Deserialize)]
struct Extraction {
    enable: bool,
    ez_coop_extract: bool,
    paid_coop_extract: PaidCoopExtract,
    car_extract_price: u32,
    remove_backpack_check: bool,
    remove_armor_vest_check: bool,
    rng_extracts_always_on: bool,
}

#[derive(Serialize, Deserialize)]
struct PaidCoopExtract {
    enabled: bool,
    extract_price: u32,
}

#[derive(Serialize, Deserialize)]
struct Events {
    enabled: bool,
    halloween_enabled: bool,
    christmas_enabled: bool,
}