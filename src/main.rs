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
    let player_card_1 = PlayerCard::new(
        "Waterspout",
        "Play to cause 3 Water Damage.  Enchant: ALl Water spells cost 1 less.",
        3,
        ElementType::Water,
    )
    .game_start_effect(GameEffect::spell_cost_down_element(ElementType::Water, -1))
    .play_card_effects(GameEffect::element_damage(ElementType::Water, 3));

    let player_card_2 = PlayerCard::new(
        "Spill Net",
        "Play to cause 3 Water Damage.",
        1,
        ElementType::Water,
    )
    .play_card_effects(GameEffect::element_damage(ElementType::Water, 3));

    let player_card_3 = PlayerCard::new(
        "Dig Hole",
        "Play to cause 6 Land Damage.",
        2,
        ElementType::Land,
    )
    .play_card_effects(GameEffect::element_damage(ElementType::Land, 6));

    let player_card_4 = PlayerCard::new(
        "Solar Power",
        "Enchant: Add 1 power at the beginning of each turn.",
        0,
        ElementType::NoElement,
    )
    .cant_play()
    .game_start_effect(GameEffect::power_add_per_turn(1));

    let player = Player::new(
        20,
        FpVec::from_vec(vec![
            player_card_1,
            player_card_2,
            player_card_3,
            player_card_4,
        ]),
    );

    let enemy = Enemy::new(
        "Oil Spill",
        10,
        DefenseProps {
            wind: DamageAdjustment::Normal,
            water: DamageAdjustment::Absolute(1),
            land: DamageAdjustment::Absolute(-1),
            any: DamageAdjustment::Normal,
        },
        3,
    )
    .player_play_card_effects(GameEffect::enemy_cond_thorns_element_played(
        ElementType::Land,
        10,
    ));

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
