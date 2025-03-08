use bevy::prelude::*;
use crate::game::StationState;

#[derive(Component)]
pub struct StationUiText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    TextBundle {
                        text: Text::from_section(
                            "Station: Energy=0, Minerals=0",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    },
                    StationUiText,
                ));
        });
}

pub fn update_ui(
    station: Res<StationState>,
    mut query: Query<&mut Text, With<StationUiText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Station: Energy={}, Minerals={}",
            station.energy, station.minerals
        );
    }
}