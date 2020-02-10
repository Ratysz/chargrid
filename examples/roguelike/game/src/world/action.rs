use crate::world::{
    data::{OnCollision, ProjectileDamage},
    explosion,
    spatial::OccupiedBy,
    ExternalEvent, World,
};
use direction::{CardinalDirection, Direction};
use ecs::{ComponentsTrait, Entity};
use grid_2d::Coord;
use rand::Rng;

impl World {
    pub fn character_walk_in_direction(&mut self, character: Entity, direction: CardinalDirection) {
        let &current_coord = self.spatial.coord(character).unwrap();
        let target_coord = current_coord + direction.coord();
        if self.is_solid_feature_at_coord(target_coord) {
            return;
        }
        if let Err(OccupiedBy(_occupant)) = self.spatial.update_coord(character, target_coord) {
            // TODO melee
        }
    }

    pub fn character_fire_bullet(&mut self, character: Entity, target: Coord) {
        let &character_coord = self.spatial.coord(character).unwrap();
        if character_coord == target {
            return;
        }
        self.spawn_bullet(character_coord, target);
        self.spawn_flash(character_coord);
    }

    pub fn character_fire_shotgun<R: Rng>(&mut self, character: Entity, target: Coord, rng: &mut R) {
        const NUM_BULLETS: usize = 12;
        let &character_coord = self.spatial.coord(character).unwrap();
        if character_coord == target {
            return;
        }
        for _ in 0..NUM_BULLETS {
            let offset = vector::Radial {
                angle_radians: rng.gen_range(-::std::f64::consts::PI, ::std::f64::consts::PI),
                length: rng.gen_range(0., 3.), // TODO make this depend on the distance
            };
            self.spawn_bullet(character_coord, target + offset.to_cartesian().to_coord_round_nearest());
        }
        self.spawn_flash(character_coord);
    }

    pub fn character_fire_rocket(&mut self, character: Entity, target: Coord) {
        let &character_coord = self.spatial.coord(character).unwrap();
        if character_coord == target {
            return;
        }
        self.spawn_rocket(character_coord, target);
    }

    pub fn projectile_stop(&mut self, projectile_entity: Entity, external_events: &mut Vec<ExternalEvent>) {
        if let Some(&current_coord) = self.spatial.coord(projectile_entity) {
            if let Some(on_collision) = self.ecs.components.on_collision.get(projectile_entity).cloned() {
                match on_collision {
                    OnCollision::Explode(explosion_spec) => {
                        explosion::explode(self, current_coord, explosion_spec, external_events);
                        self.spatial.remove(projectile_entity);
                        self.ecs.remove(projectile_entity);
                        self.realtime_components.remove_entity(projectile_entity);
                    }
                    OnCollision::Remove => {
                        self.spatial.remove(projectile_entity);
                        self.ecs.remove(projectile_entity);
                        self.realtime_components.remove_entity(projectile_entity);
                    }
                }
            }
        }
        self.realtime_components.movement.remove(projectile_entity);
    }

    pub fn projectile_move(
        &mut self,
        projectile_entity: Entity,
        movement_direction: Direction,
        external_events: &mut Vec<ExternalEvent>,
    ) {
        if let Some(&current_coord) = self.spatial.coord(projectile_entity) {
            let next_coord = current_coord + movement_direction.coord();
            let collides_with = self
                .ecs
                .components
                .collides_with
                .get(projectile_entity)
                .cloned()
                .unwrap_or_default();
            let &spatial_cell = self.spatial.get_cell_checked(next_coord);
            if let Some(character_entity) = spatial_cell.character {
                if let Some(&projectile_damage) = self.ecs.components.projectile_damage.get(projectile_entity) {
                    self.apply_projectile_damage(
                        projectile_entity,
                        projectile_damage,
                        movement_direction,
                        character_entity,
                    );
                }
            }
            if let Some(entity_in_cell) = spatial_cell.feature.or(spatial_cell.character) {
                if (collides_with.solid && self.ecs.components.solid.contains(entity_in_cell))
                    || (collides_with.character && self.ecs.components.character.contains(entity_in_cell))
                {
                    self.projectile_stop(projectile_entity, external_events);
                    return;
                }
            }
            let _ignore_if_occupied = self.spatial.update_coord(projectile_entity, next_coord);
        } else {
            self.ecs.remove(projectile_entity);
            self.realtime_components.remove_entity(projectile_entity);
            self.spatial.remove(projectile_entity);
        }
    }

    pub fn damage_character(&mut self, character: Entity, hit_points_to_lose: u32) {
        if let Some(hit_points) = self.ecs.components.hit_points.get_mut(character) {
            let &coord = self.spatial.coord(character).unwrap();
            match hit_points.current.checked_sub(hit_points_to_lose) {
                None | Some(0) => {
                    hit_points.current = 0;
                    self.character_die(character);
                }
                Some(non_zero_remaining_hit_points) => {
                    hit_points.current = non_zero_remaining_hit_points;
                }
            }
            self.add_blood_stain_to_floor(coord);
        } else {
            log::warn!("attempt to damage entity without hit_points component");
        }
    }

    fn character_push_in_direction(&mut self, entity: Entity, direction: Direction) {
        if let Some(&current_coord) = self.spatial.coord(entity) {
            let target_coord = current_coord + direction.coord();
            if self.is_solid_feature_at_coord(target_coord) {
                return;
            }
            let _ignore_if_occupied = self.spatial.update_coord(entity, target_coord);
        }
    }

    fn character_die(&mut self, character: Entity) {
        self.spatial.remove(character);
    }

    fn add_blood_stain_to_floor(&mut self, coord: Coord) {
        if let Some(floor_entity) = self.spatial.get_cell_checked(coord).floor {
            self.ecs.components.blood.insert(floor_entity, ());
        }
    }

    fn apply_projectile_damage(
        &mut self,
        projectile_entity: Entity,
        projectile_damage: ProjectileDamage,
        projectile_movement_direction: Direction,
        entity_to_damage: Entity,
    ) {
        self.damage_character(entity_to_damage, projectile_damage.hit_points);
        if projectile_damage.push_back {
            self.character_push_in_direction(entity_to_damage, projectile_movement_direction);
        }
        self.ecs.remove(projectile_entity);
    }
}
