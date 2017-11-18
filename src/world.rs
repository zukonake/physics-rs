use std;

use entity;

pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 200;

type TileValue = f32;

#[derive(Copy, Clone)]
pub enum Tile
{
    Empty(TileValue),
    Wall,
    Drain,
}

type Index = [usize; 2];
pub type Coordinate = isize;
pub type Point = [Coordinate; 2];

pub struct Map([[Tile; WIDTH]; HEIGHT]);

impl Map
{
    pub fn new() -> Self
    {
        let mut tiles = [[Tile::Empty(0.0); WIDTH]; HEIGHT];
        for x in 0..WIDTH
        {
            tiles[0][x] = Tile::Wall;
            tiles[HEIGHT - 1][x] = Tile::Wall;
        }
        for y in 0..HEIGHT
        {
            tiles[y][0] = Tile::Wall;
            tiles[y][WIDTH - 1] = Tile::Wall;
        }
        Self{0: tiles}
    }

    pub fn at(&self, point: Point) -> Option<&Tile>
    {
        match Self::to_index(point)
        {
            Some(index) => Some(&self.0[index[1]][index[0]]),
            None => None,
        }
    }

    pub fn at_mut(&mut self, point: Point) -> Option<&mut Tile>
    {
        match Self::to_index(point)
        {
            Some(index) => Some(&mut self.0[index[1]][index[0]]),
            None => None,
        }
    }

    pub fn simulate(&mut self) -> ()
    {
        unsafe
        {
            let mut buffer: [[Tile; WIDTH]; HEIGHT] = std::mem::uninitialized();
            for y in 0..HEIGHT
            {
                for x in 0..WIDTH
                {
                    let position = [x as Coordinate, y as Coordinate];
                    buffer[y][x] = self.average_tile(position).unwrap();
                }
            }
            std::mem::swap(&mut self.0, &mut buffer);
        }
    }

    fn average_tile(&self, position: Point) -> Option<Tile>
    {
        match self.at(position)
        {
            Some(tile) => match tile
            {
                &Tile::Empty(_) =>
                {
                    let mut sum = 0.0f32;
                    let mut count = 0u32;
                    for y in (position[1] - 1)..(position[1] + 2)
                    {
                        for x in (position[0] - 1)..(position[0] + 2)
                        {
                            match self.at([x, y])
                            {
                                Some(tile) => match tile
                                {
                                    &Tile::Empty(value) =>
                                    {
                                        sum += value;
                                        count += 1;
                                    }
                                    &Tile::Wall => (),
                                    &Tile::Drain => count += 1,
                                },
                                None => (),
                            }
                        }
                    }
                    Some(Tile::Empty(sum / count as f32))
                },
                &Tile::Wall => Some(Tile::Wall),
                &Tile::Drain => Some(Tile::Drain),
            },
            None => None,
        }
    }

    fn to_index(point: Point) -> Option<Index>
    {
        if point[0] >= 0 && point[1] >= 0 && point[0] < WIDTH as isize && point[1] < HEIGHT as isize
        {
            Some([point[0] as usize, point[1] as usize])
        }
        else
        {
            None
        }
    }
}

pub struct World
{
    map: Map,
    pub entities: entity::EntityContainer,
}

impl World
{
    pub fn new() -> Self
    {
        Self
        {
            map: Map::new(),
            entities: entity::EntityContainer::new(),
        }
    }

    pub fn at(&self, point: Point) -> Option<&Tile>
    {
        self.map.at(point)
    }

    pub fn at_mut(&mut self, point: Point) -> Option<&mut Tile>
    {
        self.map.at_mut(point)
    }

    pub fn place_entity(&mut self, point: Point) -> ()
    {
        let entity_point = [point[0] as entity::Coordinate + 0.5,
                            point[1] as entity::Coordinate + 0.5];
        self.entities.0.push(entity::Entity::new(entity_point, [0.0, 0.0]));
    }

    pub fn brush(&mut self, value: Tile, position: Point, radius: f32)
    {
        let map_radius = (radius + 0.5) as isize;
        for y in (position[1] - map_radius)..(position[1] + 1 + map_radius)
        {
            for x in (position[0] - map_radius)..(position[0] + 1 + map_radius)
            {
                if (((position[0] - x) as f32).powi(2) +
                    ((position[1] - y) as f32).powi(2)).sqrt() <= radius
                {
                    let position = [x as Coordinate, y as Coordinate];
                    match self.at_mut(position)
                    {
                        Some(tile) => std::mem::swap(tile, &mut value.clone()),
                        None => (),
                    }
                }
            }
        }
    }

    pub fn simulate(&mut self) -> ()
    {
        self.map.simulate();
        self.entities.simulate(&self.map);
    }
}
