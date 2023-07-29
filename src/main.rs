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


fn characher_movement(
	mut characters: Query<(&mut Transform, &Sprite)>,
	input: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	for (mut transform, _) in &mut characters {
		
		let mut direction: Vec3 = Vec3::default();
		let speed = 256.0 * time.delta_seconds();
		
		if input.pressed(KeyCode::Up) {
			direction.y += 1.0;
		};
		if input.pressed(KeyCode::Down) {
			direction.y -= 1.0;
		};
		if input.pressed(KeyCode::Right) {
			direction.x += 1.0;
		};
		if input.pressed(KeyCode::Left) {
			direction.x -= 1.0;
		}
		
		if !direction.is_normalized() && direction.length() != 0.0{
			direction = direction.normalize();
			
			
		}
		
		transform.translation += direction * Vec3 {x:speed, y:speed, z:0.0};
	};
}


fn main() {
	
	let pixel_game_plugins = DefaultPlugins
		.set(ImagePlugin::default_nearest())
		.set(WindowPlugin {
			primary_window: Some(Window { 
				title: "My cool Game".into(),
				resolution: (640.0, 480.0).into(),
				resizable:false,
				..default()
			}),
			..default()
		})
		.build();
	
	
	App::new()
		.add_plugins(pixel_game_plugins)
		.add_systems(Startup, setup)
		.add_systems(Update, characher_movement)
		.run();
	
}
