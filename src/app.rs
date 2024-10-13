use bevy::input::InputPlugin;
use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy;

fn add_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 1.0),
                ..default()
            },
            ..default()
        },
        Enemy,
    ));
}

pub fn create_app() -> App {
    let mut app = App::new();

    if cfg!(test) {
        app.add_plugins(TaskPoolPlugin::default());
        //app.add_plugins(AssetPlugin::default());
        //app.init_asset::<bevy::render::texture::Image>();
        app.add_plugins(InputPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_systems(Startup, add_enemy);
    app.add_systems(Update, respond_to_mouse_button_press);

    app
}


#[cfg(test)]
fn count_n_enemies(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Enemy>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn get_enemy_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Enemy)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_enemy_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Enemy)>();
    let (transform, _) = query.single(app.world());
    assert_eq!(transform.scale.z, 1.0); // 2D
    transform.scale.xy()
}

fn respond_to_mouse_button_press(
    mut query: Query<&mut Transform, With<Enemy>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut player_position = query.single_mut();
    if input.pressed(MouseButton::Left) {
        // Do something
        player_position.translation.x += 16.0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an_empty_app_has_no_enemies() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_enemies(&mut app), 0);
    }

    #[test]
    fn test_our_app_has_one_enemy() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_enemies(&mut app), 1);
    }

    #[test]
    fn test_enemy_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_enemy_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_enemy_has_the_default_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_enemy_scale(&mut app), Vec2::new(100.0, 100.0));
    }

    #[test]
    fn test_enemy_responds_to_mouse_button_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec2::new(0.0, 0.0), get_enemy_position(&mut app));

        // Press the left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.update();

        // Position must have changed now
        assert_ne!(Vec2::new(0.0, 0.0), get_enemy_position(&mut app));
    }
}
