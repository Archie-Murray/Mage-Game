use bevy::{app::AppExit, prelude::*};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pause_menu)
           .add_systems(Update, (show_pause, update_pause_button));
    }
}

#[derive(Component)]
struct Pause;

#[derive(Component)]
struct Quit;

fn show_pause(
    input: Res<ButtonInput<KeyCode>>,
    mut pause_root: Query<&mut Visibility, With<Pause>>,
) {
    let Ok(mut pause_visibility) = pause_root.get_single_mut() else { return; };
    if input.just_pressed(KeyCode::Escape) {
        *pause_visibility = match *pause_visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden
        };
    }
}

fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Percent(25.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgba(0.8, 0.8, 0.8, 0.5)),
        visibility: Visibility::Hidden,
        ..Default::default()
    }).insert(Pause)
    .with_children(|canvas_parent| {
        canvas_parent.spawn((
            Quit,
            ButtonBundle {
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(50.0),
                    border: UiRect::all(Val::Percent(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::DARK_GRAY),
                ..Default::default()
            }
        )).with_children(|button_parent| {
            button_parent.spawn(
        TextBundle::from_section(
            "Quit", 
            TextStyle {
                        font: asset_server.load("fonts/Alagard.ttf"),
                        font_size: 32.0,
                        color: Color::RED
                    }
                )
            );
        });
    });
}

fn update_pause_button(
    mut quit_query: Query<(&Interaction, &mut BackgroundColor), With<Quit>>, 
    mut exit_evw: EventWriter<AppExit>
) {
    if let Ok((interaction, mut quit_bg)) = quit_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                exit_evw.send_default();
            },
            Interaction::Hovered => *quit_bg = BackgroundColor(Color::rgb(0.4, 0.2, 0.2)),
            Interaction::None => *quit_bg = BackgroundColor(Color::DARK_GRAY),
        }
    }
}
