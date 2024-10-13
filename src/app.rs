use bevy::prelude::*;


#[derive(Component)]
pub struct Enemy;

fn add_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_bird_dark.png"),
            ..default()
        },
        Enemy,
    ));
}
pub fn create_app() -> App {
    let mut app = App::new();

    if cfg!(test) {
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<bevy::render::texture::Image>();
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_systems(Startup, add_enemy);

    app
}

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

#[cfg(test)]
fn get_enemy_has_texture(app: &mut App) -> bool {
    let mut query = app.world_mut().query::<(&Handle<Image>, &Enemy)>();
    let (handle, _) = query.single(app.world());
    handle.is_strong()
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
        assert_eq!(get_enemy_scale(&mut app), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_enemy_has_a_texture() {
        let mut app = create_app();
        app.update();
        assert!(get_enemy_has_texture(&mut app));
    }
}
