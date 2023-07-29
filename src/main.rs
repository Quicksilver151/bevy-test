use bevy::prelude::*;



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2dBundle::default());
	
	let texture = asset_server.load("slime.png");
	
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::new(80.0, 80.0)),
			..default()
		},
		texture,
		..default()
	});
}


fn main() {
	
	let pixel_game_plugin = DefaultPlugins
		.set(ImagePlugin::default_nearest())
		.set(WindowPlugin {
			primary_window: Some(Window { 
				title: "My cooll Game".into(),
				resolution: (640.0, 480.0).into(),
				resizable:false,
				..default()
			}),
			..default()
		})
		.build();
	
	
	App::new()
		.add_plugins(pixel_game_plugin)
		.add_systems(Startup, setup)
		.run();
	
}
