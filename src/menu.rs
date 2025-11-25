use bevy::{input_focus::InputFocus, prelude::*};

use crate::{GameState, game::EmbeddedFonts};

pub struct MenuPlugin;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.45, 0.45, 0.45);

#[derive(Component)]
struct ButtonType(GameState);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(GameState::Editor), editor_setup)
            .add_systems(OnEnter(GameState::Settings), settings_setup)
            .add_systems(OnEnter(GameState::CustomLevel), custom_level_setup)
            .add_systems(OnEnter(GameState::Exit), exit_setup)
            .add_systems(Update, menu_system)
            .init_resource::<InputFocus>();
    }
}

fn menu_setup(mut commands: Commands, assets: Res<AssetServer>, mut fonts: ResMut<Assets<Font>>) {
    let font = Font::try_from_bytes(crate::FONT.to_vec()).expect("Failed to load embedded font");

    let font_handle = fonts.add(font);
    commands.insert_resource(EmbeddedFonts {
        ui_font: font_handle.clone(),
    });
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            height: percent(100),
            width: percent(100),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    height: percent(100),
                    width: percent(100),
                    margin: UiRect::all(px(10f32)),
                    ..default()
                },
                children![(
                    Text::new("Isomagic"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 100.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ),
            (
                Node {
                    height: percent(100),
                    width: percent(100),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(px(10f32)),
                    ..default()
                },
                children![
                    button(GameState::Level, "Levels", &assets, font_handle.clone()),
                    button(
                        GameState::CustomLevel,
                        "Custom Levels",
                        &assets,
                        font_handle.clone()
                    ),
                    button(GameState::Editor, "Editor", &assets, font_handle.clone()),
                    button(
                        GameState::Settings,
                        "Settings",
                        &assets,
                        font_handle.clone()
                    ),
                    button(GameState::Exit, "Exit", &assets, font_handle.clone()),
                ],
            )
        ],
    ));
}

fn button(
    game_state: GameState,
    text: impl Into<String>,
    assets: &AssetServer,
    font: Handle<Font>,
) -> impl Bundle {
    ((
        ButtonType(game_state),
        Button,
        Node {
            height: px(65),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            padding: UiRect::all(px(5f32)),
            margin: UiRect::all(px(5f32)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BorderRadius::MAX,
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font: font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    ),)
}

fn menu_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
            &ButtonType,
        ),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut _text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, _children, button_type) in
        &mut interaction_query
    {
        // let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                //**text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
                next_state.set(button_type.0.clone());
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                //**text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                //**text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

fn editor_setup() {}
fn level_setup(commands: Commands) {}
fn settings_setup() {}
fn custom_level_setup() {}
fn exit_setup(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
