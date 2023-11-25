use bevy::prelude::*;

/**
 * Update accelerations
 * Update velocities
 * Update movement
 */

const G_CONST: f64 = 0.0000000000667;
const M_PER_UNIT: f64 = 1000000.0;
const M_PER_UNIT_SQ: f64 = 1000000000000.0;

#[derive(Component, Debug, Clone, Copy)]
pub struct OrbitBody {
    pub mass: f64,
    pub velocity: Vec3
}

#[derive(Component, Debug, Clone, Copy)]
pub struct OrbitForce {
    pub force: Vec3
}

pub struct OrbitPlugin;
impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_accelerations, update_movement));
    }
}

fn update_accelerations(
    mut commands: Commands,
    mut orbital_objects: Query<(Entity, &OrbitBody, &Transform)>
) {
    let objects = orbital_objects.iter_mut().collect::<Vec<(Entity, &OrbitBody, &Transform)>>();

    // loop through all objects
    let mut i = 0;
    while i < objects.len() {
        // get body and transform
        let (entity, body, transform) = &objects[i];
        let mut net_force = Vec3::default();

        // loop through again
        let mut j = 0;
        while j < objects.len() {
            // skip if same object
            if i == j { j += 1; continue; }

            // get other body and transform
            let (_, other_body, other_transform) = &objects[j];

            // calculate gravity force
            let dist = transform.translation.distance_squared(other_transform.translation) as f64 * M_PER_UNIT_SQ;
            let force = (G_CONST * body.mass * other_body.mass) / dist;

            // update net force
            let offset = other_transform.translation - transform.translation;
            net_force += (offset / offset.length()) * (force as f32);

            j += 1;
        }

        // insert new orbit info with force
        commands.entity(*entity).insert(OrbitForce { force: net_force });

        i += 1;
    }
}

fn update_movement(
    mut objects: Query<(&mut Transform, &mut OrbitBody, &OrbitForce)>,
    time: Res<Time>
) {
    let delta = time.delta_seconds() * 10000.0;

    // for each object with a force
    objects.for_each_mut(|(mut transform, mut body, force)| {
        // get acceleration
        let acceleration = force.force / body.mass as f32;

        // update velocity
        body.velocity += acceleration * delta;

        // update position
        transform.translation += (body.velocity * delta) / (M_PER_UNIT as f32);
        println!("Translation {}, Velocity {}, Acceleration {}", transform.translation, body.velocity, acceleration);
    });
}
