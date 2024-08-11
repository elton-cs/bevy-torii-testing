use bevy::prelude::*;

use crate::bindgen::bevy::components::moves::Moves;

pub struct VisualizePlugin;
impl Plugin for VisualizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_visualizer, visualize).chain());
    }
}

#[derive(Debug, Component)]
struct TextVisual;

fn add_visualizer(
    mut commands: Commands,
    mut query: Query<(Entity, &Moves), Without<TextVisual>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    for (entity_id, moves) in query.iter() {
        let visual_text = Text2dBundle {
            text: Text::from_section(moves.remaining.to_string(), text_style.clone())
                .with_justify(text_justification),
            ..default()
        };

        commands.entity(entity_id).insert((TextVisual, visual_text));
    }
}

fn visualize(mut query: Query<(&mut Text, &Moves)>) {
    for (mut text, moves) in query.iter_mut() {
        text.sections[0].value = moves.remaining.to_string();
    }
}
