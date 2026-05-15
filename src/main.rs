use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{self, random_range};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.6, 0.6, 0.9)))
        .add_plugins((DefaultPlugins, flame_test::flame_test_plugin))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, start_minigame.run_if(in_state(MiniGame::None)))
        .add_systems(
            Update,
            (
                create_question,
                button_selection,
                button_system,
                run_submission,
            )
                .chain(),
        )
        .add_systems(PostUpdate, update_scoreboard)
        .init_resource::<CorrectIndex>()
        .init_resource::<Score>()
        .add_message::<CreateQuestion>()
        .init_state::<MiniGame>()
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
enum MiniGame {
    #[default]
    None = 0,
    FlameTest = 1,
}
const MINIGAME_LIST: [MiniGame; 2] = [MiniGame::None, MiniGame::FlameTest];

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct OptionPanel;

#[derive(Component)]
struct AnswerButton;

#[derive(Component)]
struct SubmitButton;

#[derive(Component)]
struct AnswerIndex(usize);

#[derive(Message)]
struct CreateQuestion;

#[derive(Resource, Default)]
struct CorrectIndex(usize);

#[derive(Resource, Default)]
struct Score(u16);

#[derive(Component)]
struct ScoreboardUI;

const BUTTON_COLOR: Color = Color::BLACK;
const HOVERED_COLOR: Color = Color::srgba(0.7, 0.7, 0.7, 0.7);
const PRESSED_COLOR: Color = Color::srgba(0.7, 0.9, 0.4, 0.9);
const SELECTED_COLOR: Color = Color::srgba(0.4, 0.9, 0.4, 0.9);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(submit_button());
    commands.spawn(create_scoreboard());
}

fn create_scoreboard() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            right: percent(5),
            top: percent(5),
            ..default()
        },
        //BackgroundColor(Color::WHITE),
        children![(
            ScoreboardUI,
            Text::new("Score:"),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::BLACK),
            //TextShadow::default(),
            children![(
                TextSpan::default(),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            )],
        )],
    )
}

fn update_scoreboard(
    score: Res<Score>,
    scoreboard_query: Single<Entity, With<ScoreboardUI>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*scoreboard_query, 1) = score.0.to_string();
}

fn submit_button() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            top: percent(40),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            SubmitButton,
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::MAX,
                margin: UiRect::all(px(20)),
                padding: UiRect {
                    left: px(15),
                    right: px(15),
                    top: px(5),
                    bottom: px(5),
                },

                ..default()
            },
            BackgroundColor(BUTTON_COLOR),
            children![(
                Text::new("Submit"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}

fn button_system(
    interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&SelectedOption>)>,
) {
    for (interaction, mut color, selected) in interaction_query {
        match (interaction, selected) {
            (_, Some(_)) => color.0 = SELECTED_COLOR,
            (Interaction::Pressed, _) => color.0 = PRESSED_COLOR,
            (Interaction::Hovered, _) => color.0 = HOVERED_COLOR,
            (Interaction::None, _) => color.0 = BUTTON_COLOR,
        }
    }
}

fn button_selection(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), With<AnswerButton>>,
    selected_query: Option<Single<(Entity, &mut BackgroundColor), With<SelectedOption>>>,
) {
    for (entity, interaction) in interaction_query {
        if *interaction == Interaction::Pressed {
            selected_query.map(|mut q| {
                commands.entity(q.0).remove::<SelectedOption>();
                q.1.0 = BUTTON_COLOR
            });
            commands.entity(entity).insert(SelectedOption);
            break;
        }
    }
}

fn answer_creation(options: &[&str]) -> impl Bundle {
    println!("answer");
    (
        OptionPanel,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(px(50)),
                    ..default()
                },
                children![(
                    Text::new(format!("Select {}. TODO", 0 + 1)),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                ),]
            ),
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                children![
                    create_button(Text::new(options[0]), 0),
                    create_button(Text::new(options[1]), 1),
                    create_button(Text::new(options[2]), 2),
                    create_button(Text::new(options[3]), 3),
                ],
            ),
        ],
    )
}

fn create_button(text: Text, index: usize) -> impl Bundle {
    (
        Button,
        AnswerButton,
        AnswerIndex(index),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::MAX,
            margin: UiRect::all(px(20)),
            padding: UiRect {
                left: px(15),
                right: px(15),
                top: px(5),
                bottom: px(5),
            },
            ..default()
        },
        BackgroundColor(BUTTON_COLOR),
        children![(
            text,
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        ),],
    )
}

fn run_submission(
    submit_query: Single<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    selected_query: Option<Single<&AnswerIndex, With<SelectedOption>>>,
    mut message_writer: MessageWriter<CreateQuestion>,
    correct_index: Res<CorrectIndex>,
    mut score: ResMut<Score>,
    mut minigame: ResMut<NextState<MiniGame>>,
) {
    if **submit_query == Interaction::Pressed
        && let Some(selected) = selected_query
    {
        if selected.0 == correct_index.0 {
            println!("correct");
            score.0 += 1;
        } else {
            println!("wrong");
        }
        minigame.set(MiniGame::None);
        message_writer.write(CreateQuestion);
    }
}

fn create_question(
    mut message_reader: MessageReader<CreateQuestion>,
    option_panel: Single<Entity, With<OptionPanel>>,
    mut commands: Commands,
    correct_index: ResMut<CorrectIndex>,
) {
    if message_reader.read().next().is_some() {
        commands.entity(*option_panel).despawn();
    }
}

fn start_minigame(
    mut commands: Commands,
    mut game_state: ResMut<NextState<MiniGame>>,
    answer_index: ResMut<CorrectIndex>,
) {
    game_state.set(MINIGAME_LIST[random_range(1..MINIGAME_LIST.len())]);

    println!("minigame started");
}

mod flame_test {
    use std::{collections::HashMap, f32::consts::PI};

    use bevy::window::PrimaryWindow;
    use rand::seq::SliceRandom;

    use super::*;

    #[derive(Component)]
    struct Spoon;

    pub fn flame_test_plugin(app: &mut App) {
        app.add_systems(
            OnEnter(MiniGame::FlameTest),
            (create_spoon, create_question),
        )
        .add_systems(Update, move_spoon.run_if(in_state(MiniGame::FlameTest)));
    }

    fn create_spoon(mut commands: Commands, server: Res<AssetServer>) {
        commands.spawn((
            Sprite {
                image: server.load("spoon.png"),
                ..default()
            },
            Spoon,
        ));
        println!("spoon");
    }

    fn create_question(mut commands: Commands) {
        let colors = HashMap::from([
            ("Cs", "Purple"),
            ("Li", "Pink"),
            ("Ca", "Red"),
            ("Mg", "White"),
            ("In", "Blue"),
            ("Cu", "Green"),
        ]);
        let mut elements: Vec<&str> = colors.keys().cloned().collect();
        elements.shuffle(&mut rand::rng());
        commands.spawn(answer_creation(&elements[..4]));
    }

    fn move_spoon(
        mut spoon: Single<&mut Transform, With<Spoon>>,
        mut cursor_moved_event_reader: MessageReader<CursorMoved>,
        window: Single<&Window, With<PrimaryWindow>>,
        mouse: Res<ButtonInput<MouseButton>>,
    ) {
        //println!("test");
        if mouse.pressed(MouseButton::Left) {
            if let Some(cursor_event) = cursor_moved_event_reader.read().last()
                && cursor_event.position.y < window.size().y / 2.
            {
                spoon.translation = (cursor_event.position - window.size() / 2. + vec2(40., 5.))
                    .extend(0.)
                    .rotate_x(PI);
                //println!("yes");
            }
        };
    }
}
