use card_game_simulator;
use card_game_simulator::game::{game_turn, GameOutcome};
use card_game_simulator::{
    enemy::Enemy,
    fp_vec::FpVec,
    game::Game,
    player::{Player, PlayerCard},
    Damage, DamageAdjustment, DefenseProps, EffectCondition, EffectTrigger, EffectType,
    ElementType, Enchantment, GameEffect,
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
        end_turn_effects: FpVec::from_vec(vec![GameEffect::player(EffectTrigger::Always(
            EffectType::Damage(Damage::raw(3)),
        ))]),
        hit_points: 10,
    };

    let water_poss_cost_down = EffectTrigger::Condition(
        EffectCondition::PlayerHasCardWithElement(ElementType::Water),
        EffectType::Enchantment(Enchantment::SpellCostAdjust(ElementType::NoElement, -1)),
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
        posession_effects: FpVec::from_vec(vec![GameEffect::player(water_poss_cost_down)]),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy(water_damage)]),
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
        posession_effects: FpVec::new(),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy(water_damage)]),
    };

    let land_damage = EffectTrigger::Always(EffectType::Damage(Damage {
        amount: 6,
        element_type: ElementType::Water,
    }));
    let player_card_3 = PlayerCard {
        element: ElementType::Land,
        power_cost: 2,
        can_play: true,
        name: "Dig Hole".to_string(),
        posession_effects: FpVec::new(),
        play_card_effects: FpVec::from_vec(vec![GameEffect::enemy(land_damage)]),
    };

    let power_eff = EffectTrigger::Always(EffectType::PowerAdjust(1));
    let player_card_4 = PlayerCard {
        element: ElementType::NoElement,
        power_cost: 0,
        can_play: false,
        name: "Solar Power".to_string(),
        posession_effects: FpVec::from_vec(vec![GameEffect::player(power_eff)]),
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

fn main() {
    let game = init_game();

    let card_plays = FpVec::<FpVec<PlayerCard>>::new()
        .push(FpVec::from_vec(vec![
            game.player.cards.inner[0].clone(),
            game.player.cards.inner[1].clone(),
        ]))
        .push(FpVec::from_vec(vec![
            game.player.cards.inner[1].clone(),
            game.player.cards.inner[1].clone(),
        ]))
        .push(FpVec::from_vec(vec![
            game.player.cards.inner[2].clone(),
            game.player.cards.inner[3].clone(),
        ]))
        .push(FpVec::from_vec(vec![
            game.player.cards.inner[0].clone(),
            game.player.cards.inner[2].clone(),
        ]))
        .push(FpVec::from_vec(vec![
            game.player.cards.inner[1].clone(),
            game.player.cards.inner[2].clone(),
        ]));

    let out = card_plays.inner.into_iter().fold(game, game_turn);

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
