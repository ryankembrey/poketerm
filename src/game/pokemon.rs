#[derive(Debug)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(Debug)]
pub enum AttackCategory {
    Physical,
    Special,
}

#[derive(Debug)]
pub enum Effect {
    Damage(u16),
    Heal(u16),
    Stat,
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub pokemon_type: PokemonType,
    pub stats: Stats,
    pub moves: Moves,
}

#[derive(Debug)]
pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defence: u16,
    pub sp_attack: u16,
    pub sp_defence: u16,
    pub speed: u16,
}

#[derive(Debug)]
pub struct Moves {
    pub r#move: Vec<Move>,
}

#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub pokemon_type: PokemonType,
    pub attack_category: AttackCategory,
    pub effect: Effect,
}
