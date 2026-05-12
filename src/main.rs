use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (button_selection, button_system, run_submission).chain(),
        )
        .run();
}

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct AnswerButton;

#[derive(Component)]
struct SubmitButton;

#[derive(Component)]
struct AnswerIndex(usize);

#[derive(Message)]
struct RunSubmission;

const BUTTON_COLOR: Color = Color::BLACK;
const HOVERED_COLOR: Color = Color::srgba(0.7, 0.7, 0.7, 0.7);
const PRESSED_COLOR: Color = Color::srgba(0.7, 0.9, 0.4, 0.9);
const SELECTED_COLOR: Color = Color::srgba(0.4, 0.9, 0.4, 0.9);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(answer_creation());
    commands.spawn(submit_button());
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
            ),],
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

fn answer_creation() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            top: percent(20),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            create_button(Text::new("1"), 0),
            create_button(Text::new("2"), 1),
            create_button(Text::new("3"), 2),
            create_button(Text::new("4"), 3),
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
    submit_query: Single<&Interaction, With<SubmitButton>>,
    selected_query: Option<Single<&AnswerIndex, With<SelectedOption>>>,
) {
    if **submit_query == Interaction::Pressed
        && let Some(selected) = selected_query
    {
        if selected.0 == 3 {
            println!("correct")
        }
    }
}
