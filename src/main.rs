#![feature(let_chains)]

use card_game_simulator;
use card_game_simulator::game::GameOutcome;
use card_game_simulator::game_effects::{CardEffects, Enchantments, OnCardPlayEffects};
use card_game_simulator::player::SpecialCards;
use card_game_simulator::{
    enemy::Enemy,
    fp_vec::FpVec,
    game::Game,
    player::{Player, PlayerCard},
    ElementType,
};

pub fn init_game() -> Game {
    let player_card_1 = PlayerCard::new("Gust", "Play to cause 3 Wind Damage.", ElementType::Wind)
        .play_card_effect(CardEffects::do_element_damage(ElementType::Wind, 3));

    let player_card_2 =
        PlayerCard::new("Stream", "Play to cause 3 Land Damage.", ElementType::Land)
            .play_card_effect(CardEffects::do_element_damage(ElementType::Land, 3));

    let player_card_3 =
        PlayerCard::new("First Aid", "Play to head 8 hit points.", ElementType::Land)
            .play_card_effect(CardEffects::heal(8));

    let player_card_4 = SpecialCards::fire_hose();
    let player = Player::new(
        20,
        FpVec::from_vec(vec![
            player_card_1,
            player_card_2,
            player_card_3,
            player_card_4,
        ]),
    );

    let enemy = Enemy::blackout();

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

fn print_cards(game: &Game) {
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
                            "{}. {} {} - {}",
                            number,
                            card.name,
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
}

fn game_loop(mut game: Game) -> Game {
    print_cards(&game);

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
        print_cards(&game);

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
