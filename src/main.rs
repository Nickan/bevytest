use bevy::prelude::*;

pub struct Player {
	pub speed: f32
}


/// set up a simple 3D scene
fn main() {
	App::build()
		.add_resource(Msaa { samples: 4 })
		.add_default_plugins()
		.add_startup_system(setup.system())
		.run();
}

pub fn setup(
	commands: Commands,
  meshes: ResMut<Assets<Mesh>>,
	materials: ResMut<Assets<StandardMaterial>>
) {
	normal_add(commands, meshes, materials);
	// group_add(commands, meshes, materials);
}

fn normal_add(	
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	commands
		.spawn(PbrComponents {
				mesh: meshes.add(Mesh::from(shape::Icosphere { subdivisions: 4, radius: 0.5 })),
				material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
				translation: Translation::new(1.5, 1.5, 1.5),
				..Default::default()
		})
		.spawn(LightComponents {
			translation: Translation::new(4.0, 8.0, 4.0),
			..Default::default()
		})
		.spawn(Camera3dComponents {
			translation: Translation::new(0.0, 2.0, 8.0),
			..Default::default()
		});
}

fn group_add(	
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	commands
		.spawn((
			PbrComponents {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
				material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
				translation: Translation::new(0.0, 1.0, 0.0),
				..Default::default()
			},
			LightComponents {
				translation: Translation::new(4.0, 8.0, 4.0),
				..Default::default()
			},
			Camera3dComponents {
				translation: Translation::new(0.0, 2.0, 8.0),
				..Default::default()
			})
		)
		.with(Player {
			speed: 1.0
		});
}