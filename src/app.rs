use bevy::prelude::*;

pub fn create_app() -> App {
    let mut app = App::new();

    if cfg!(test) {
        app.add_plugins(MinimalPlugins);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app
}

fn count_n_enemies(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Enemy>();
    query.iter(app.world()).len()
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
/*
    #[test]
    fn test_our_app_has_one_enemy() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_enemies(&mut app), 1);
    }
*/

}
