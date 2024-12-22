use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;


#[derive(Component)]
pub struct Enemy;

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn add_enemy(mut commands: Commands) {
    commands.spawn((
        Sprite::default(),
        Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..default()
        },
        Enemy,
    ));
}

pub fn create_app() -> App {
    let mut app = App::new();

    // The idea for the function 'try_add_plugins' 
    // (https://github.com/bevyengine/bevy/discussions/15802#discussioncomment-10898148)
    // will make this if obsolete and increase code coverage.
    // Thanks mgi388 for pointing this out
    if cfg!(test) {
        app.add_plugins(TaskPoolPlugin::default());
        //app.add_plugins(AssetPlugin::default());
        //app.init_asset::<bevy::image::Image>();
        app.add_plugins(InputPlugin);
        app.add_plugins(WindowPlugin::default());

    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.add_systems(Startup, add_camera);
    app.add_systems(Startup, add_enemy);
    app.add_systems(Update, respond_to_mouse_button_press);

    app
}


fn is_cursor_in_window(window: &Window) -> bool {
    window.cursor_position().is_some()
}

fn is_cursor_in_window_mut(window: &Mut<Window>) -> bool {
    window.cursor_position().is_some()
}

/// The cursor position in this window in logical pixels
fn get_cursor_position_logical(window: &Window) -> Vec2 {
    assert!(is_cursor_in_window(window));
    window.cursor_position().unwrap()
}

fn get_cursor_position_logical_mut(window: &Mut<Window>) -> Vec2 {
    assert!(is_cursor_in_window_mut(window));
    window.cursor_position().unwrap()
}


fn get_world_position(camera: &Camera, global_transform: &GlobalTransform, cursor_pos: Vec2) -> Vec2 {
    assert!(is_cursor_pos_in_viewport(camera, global_transform, cursor_pos));
    camera.viewport_to_world_2d(global_transform, cursor_pos).unwrap()
}


// @param cursor_pos: the logical cursor position
fn is_cursor_pos_in_viewport(camera: &Camera, global_transform: &GlobalTransform, cursor_pos: Vec2) -> bool {
    camera.viewport_to_world_2d(global_transform, cursor_pos).is_ok()
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
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.pressed(MouseButton::Left) {
        // Do something
        assert_ne!(0, window_query.iter().len());
        assert_ne!(2, window_query.iter().len());
        assert_eq!(1, window_query.iter().len());
        let window = window_query.single();
        if !is_cursor_in_window(window) {
            return
        }
        let cursor_pos = get_cursor_position_logical(window);

        assert_ne!(0, camera_q.iter().len());
        assert_ne!(2, camera_q.iter().len());
        assert_eq!(1, camera_q.iter().len());
        let (camera, camera_transform) = camera_q.single();

        if !is_cursor_pos_in_viewport(camera, camera_transform, cursor_pos) {
            // If the cursor is in the window, the cursor is in the viewport in this case
            assert!("Should never happen" == "?");
            return
        }
        let world_position = get_world_position(camera, camera_transform, cursor_pos);

        let mut enemy_transform = enemy_query.single_mut();
        let enemy_min_x = enemy_transform.translation.x - (enemy_transform.scale.x / 2.0);
        let enemy_min_y = enemy_transform.translation.y - (enemy_transform.scale.y / 2.0);
        let enemy_max_x = enemy_transform.translation.x + (enemy_transform.scale.x / 2.0);
        let enemy_max_y = enemy_transform.translation.y + (enemy_transform.scale.y / 2.0);
        if world_position.x > enemy_min_x && world_position.x < enemy_max_x &&
            world_position.y > enemy_min_y && world_position.y < enemy_max_y {
            // Move away effect
            let dx = world_position.x - enemy_transform.translation.x;
            let dy = world_position.y - enemy_transform.translation.y;
            enemy_transform.translation.x -= dx;
            enemy_transform.translation.y -= dy;
        }
    }
}

/// The cursor position in this window in logical pixels
fn set_cursor_position_logical(window: &mut Mut<Window>, position: Vec2) {
    window.set_cursor_position(Some(position));
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
    fn test_set_cursor_position_logical() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();
        let mut query = app.world_mut().query::<&mut Window>();
        let mut window = query.single_mut(app.world_mut());
        let pos_before = get_cursor_position_logical_mut(&window);
        let new_pos = pos_before + Vec2::new(1.2, 3.4);
        set_cursor_position_logical(&mut window, new_pos);
        let pos_after = get_cursor_position_logical_mut(&window);
        assert_ne!(pos_before, pos_after);
        assert_eq!(new_pos, pos_after);
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
