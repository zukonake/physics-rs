use world;

pub type Coordinate = f32;
pub type Point = [Coordinate; 2];
pub type Vector = [f32; 2];

pub struct Entity
{
    pub position: Point,
    pub velocity: Vector,
}

impl Entity
{
    pub fn new(position: Point, velocity: Vector) -> Self
    {
        Self
        {
            position: position,
            velocity: velocity,
        }
    }

    pub fn simulate(&mut self, map: &world::Map) -> ()
    {
        let from = self.map_position();
        let get_value = |point: world::Point| match map.at(point)
        {
            Some(&world::Tile::Empty(value)) => value,
            _ => 0.0,
        };
        let from_value = get_value(from);
        for y in -1..2isize
        {
            for x in -1..2isize
            {
                if x != 0 && y != 0
                {
                    let direction = [x as world::Coordinate, y as world::Coordinate];
                    let to = [from[0] + direction[0], from[1] + direction[1]];
                    let to_value = get_value(to);
                    let distance = (((from[0] - to[0]) as f32).powi(2) +
                                     ((from[1] - to[1]) as f32).powi(2)).sqrt();
                    let delta = (to_value - from_value) / distance; //No operation if not empty?
                    self.velocity[0] += (-direction[0].signum() as f32) * delta;
                    self.velocity[1] += (-direction[1].signum() as f32) * delta;
                }
            }
        }
        let from = self.position;
        let to = [from[0] + self.velocity[0], from[1] + self.velocity[1]];
        let to_map = [(to[0] + 0.5) as world::Coordinate, (to[1] + 0.5) as world::Coordinate];
        match map.at(to_map)
        {
            Some(&world::Tile::Empty(_)) => self.position = to,
            _ => self.bounce(),
        }
    }

    fn map_position(&self) -> world::Point
    {
        [(self.position[0] + 0.5) as world::Coordinate,
         (self.position[1] + 0.5) as world::Coordinate]
    }

    fn bounce(&mut self) -> ()
    {
        self.velocity = [self.velocity[0] * -1.0, self.velocity[1] * -1.0];
    }
}

pub struct EntityContainer(pub Vec<Entity>);

impl EntityContainer
{
    pub fn new() -> Self
    {
        Self{0: Vec::new()}
    }

    pub fn simulate(&mut self, map: &world::Map) -> ()
    {
        for i in self.0.iter_mut()
        {
            i.simulate(map);
        }
    }
}
