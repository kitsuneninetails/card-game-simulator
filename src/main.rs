#![feature(let_chains)]

use card_game_simulator;
use card_game_simulator::game::GameOutcome;
use card_game_simulator::{
    enemy::Enemy,
    fp_vec::FpVec,
    game::Game,
    game_effects::GameEffect,
    player::{Player, PlayerCard},
    Damage, DamageAdjustment, DefenseProps, EffectCondition, EffectTrigger, EffectType,
    ElementType, Enchantment,
};

pub fn init_game() -> Game {
    let enemy = Enemy {
        defense_props: DefenseProps {
            wind: DamageAdjustment::Normal,
            water: DamageAdjustment::Absolute(1),
            land: DamageAdjustment::Absolute(-1),
            any: DamageAdjustment::Normal,
        },
        start_turn_effects: FpVec::new(),
        end_turn_effects: FpVec::from_vec(vec![GameEffect::player(
            "Enemy Attack",
            EffectTrigger::Always(EffectType::Damage(Damage::raw(3))),
        )]),
        hit_points: 10,
    };

    let water_poss_cost_down = EffectTrigger::Condition(
        EffectCondition::PlayerHasCardWithElement(ElementType::Water),
        EffectType::Enchantment(Enchantment::SpellCostAdjust(ElementType::Water, -1)),
    );
    let water_damage = EffectTrigger::Always(EffectType::Damage(Damage {
        amount: 3,
        element_type: ElementType::Water,
    }));
    let player_card_1 = PlayerCard {
        element: ElementType::Water,
        power_cost: 3,
        can_play: true,
        name: "Waterspout".to_string(),
        description: "Play to cause 3 Water Damage.  Enchant: ALl Water spells cost 1 less."
            .to_string(),
        enchantments: FpVec::from_vec(vec![GameEffect::player(
            "Water Spell Cost Down",
            water_poss_cost_down,
        )]),
        start_turn_effects: FpVec::new(),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy("Attack", water_damage)]),
    };

    let water_damage = EffectTrigger::Always(EffectType::Damage(Damage {
        amount: 3,
        element_type: ElementType::Water,
    }));
    let player_card_2 = PlayerCard {
        element: ElementType::Water,
        power_cost: 1,
        can_play: true,
        name: "Spill Net".to_string(),
        description: "Play to cause 3 Water Damage.".to_string(),
        enchantments: FpVec::new(),
        start_turn_effects: FpVec::new(),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy("Attack", water_damage)]),
    };

    let land_damage = EffectTrigger::Always(EffectType::Damage(Damage {
        amount: 6,
        element_type: ElementType::Land,
    }));
    let player_card_3 = PlayerCard {
        element: ElementType::Land,
        power_cost: 2,
        can_play: true,
        name: "Dig Hole".to_string(),
        description: "Play to cause 6 Land Damage.".to_string(),
        enchantments: FpVec::new(),
        start_turn_effects: FpVec::new(),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy("Attack", land_damage)]),
    };

    let power_eff = EffectTrigger::Always(EffectType::Enchantment(Enchantment::PowerAddPerTurn(1)));
    let player_card_4 = PlayerCard {
        element: ElementType::NoElement,
        power_cost: 0,
        can_play: false,
        name: "Solar Power".to_string(),
        description: "Enchant: Add 1 power at the beginning of each turn.".to_string(),
        enchantments: FpVec::from_vec(vec![GameEffect::player("Power Add 1", power_eff)]),
        start_turn_effects: FpVec::new(),
        play_card_effects: FpVec::new(),
    };

    let player = Player {
        hit_points: 20,
        cards: FpVec::from_vec(vec![
            player_card_1,
            player_card_2,
            player_card_3,
            player_card_4,
        ]),
        power_reserve: 0,
        current_activated_effects: FpVec::new(),
    };
    Game::start(enemy, player)
}

fn get_card_numbers() -> Result<Vec<usize>, ()> {
    let term = std::io::stdin();
    println!("Enter card #s to play with ',' between (or x to quit):");
    let mut command = String::new();
    term.read_line(&mut command).unwrap();
    if command.trim() == "x" {
        println!("Quitting");
        Err(())
    } else {
        let card_numbers: Vec<usize> = command
            .split(',')
            .into_iter()
            .flat_map(|data| data.trim().parse::<usize>())
            .collect();

        Ok(card_numbers)
    }
}

fn game_loop(mut game: Game) -> Game {
    println!("The cards are:");
    println!(
        "{}",
        game.player
            .cards
            .inner
            .iter()
            .fold((1, String::new()), |(number, text), card| {
                (
                    number + 1,
                    format!(
                        "{}{}\n",
                        text,
                        format!(
                            "{}. {} [{}]{} - {}",
                            number,
                            card.name,
                            card.power_cost,
                            if card.can_play { "" } else { " (CAN'T PLAY)" },
                            card.description
                        )
                    ),
                )
            })
            .1
    );
    println!("Game turn start: Turn {}", game.turn_number);
    println!("Player Status: {}", game.player.description());
    println!("Enemy Status: {}", game.enemy.description());

    while game.game_result == GameOutcome::Undecided && let Ok(card_numbers) = get_card_numbers() {
        let cards: FpVec<PlayerCard> =
            card_numbers
                .into_iter()
                .fold(FpVec::new(), |cards, number| {
                    match game.player.cards.inner.get(number - 1) {
                        Some(c) => cards.push(c.clone()),
                        None => {
                            println!("Invalid card: {}", number);
                            cards
                        }
                    }
                });

        game = game.take_player_turn(cards);
        if game.game_result == GameOutcome::Undecided {
            game = game.take_enemy_turn();
        }
        println!("-----------------------------------");
        println!("Game turn start: Turn {}", game.turn_number);
        println!("Player Status: {}", game.player.description());
        println!("Enemy Status: {}", game.enemy.description());

    }
    game
}

fn main() {
    let game = init_game();
    let out = game_loop(game);
    println!(
        "Game finished.  {} won on turn #{}",
        match out.game_result {
            GameOutcome::Undecided => "No one",
            GameOutcome::PlayerWins(_) => "The Player",
            GameOutcome::EnemyWins(_) => "The enemy",
        },
        out.turn_number
    );
}
