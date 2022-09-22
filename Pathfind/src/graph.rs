use crate::vector2int::RustVector2Int as Vector2Int;
use genawaiter::stack::Co;
pub mod hide;
pub mod astar;

#[repr(C)]
pub struct Graph
{
    pub walkable : Vec<Vector2Int>,
    pub walls : Vec<Vector2Int>,
    pub bounds : Bounds,
}

impl Graph
{
    pub fn new(bounds: Bounds, walkable : Vec<Vector2Int>, walls : Vec<Vector2Int>) -> Self {Self { walkable, walls, bounds,}}   

    pub fn neighbors(&self, v : Vector2Int) -> Vec<Vector2Int>
    {
        let mut neighbors = Vec::new();

        /*println!("straight");
        println!("current tile {}, {}", v.x, v.y);
        */

        for dir in Self::FOUR_DIRECTIONS
        {
            let next = v + dir;

            if self.is_cell_empty(next)
            {
                //println!("{}, {}", next.x,next.y);
                neighbors.push(next);
            }
        }
        /*println!("diagonal");
        println!("current tile {}, {}", v.x, v.y);
        */
        for dir in Self::DIAGONAL_DIRECTIONS
        {
            let next = v + dir;

            if self.is_cell_empty(next)
            {
                let adjacent1 = v + Vector2Int::new(dir.x, 0);
                let adjacent2 = v + Vector2Int::new(0, dir.y);

                if self.is_cell_empty(adjacent1) && self.is_cell_empty(adjacent2)
                {
                    //println!("{}, {}", next.x,next.y);
                    neighbors.push(next);
                }
            }
        }
        //println!();
        return neighbors;

    }

    pub async fn neighbors_routine(&self,  v : Vector2Int, gen :  Co<'_, Vector2Int>)
    {
        /*println!("straight");
        println!("current tile {}, {}", v.x, v.y);
        */
        for dir in Self::FOUR_DIRECTIONS 
        {
            let next = v + dir;

            if self.is_cell_empty(next)
            {
                //println!("{}, {}", next.x,next.y);
                gen.yield_(next).await;
            }
        }
        /*println!("diagonal");
        println!("current tile {}, {}", v.x, v.y);
        */
        for dir in Self::DIAGONAL_DIRECTIONS
        {
            let next = v + dir;

            if self.is_cell_empty(next)
            {
                let adjacent1 = v + Vector2Int::new(dir.x, 0);
                let adjacent2 = v + Vector2Int::new(0, dir.y);

                if self.is_cell_empty(adjacent1) && self.is_cell_empty(adjacent2)
                {
                    //println!("{}, {}", next.x,next.y);
                    gen.yield_(next).await;
                }
            }
        }
    }
             
    pub async fn hide_place(&self,  v : Vector2Int, hide_from : Vector2Int, gen :  Co<'_, Vector2Int>)
    {
        if hide_from.x == v.x || hide_from.y == v.y
        {
            for dir in Self::FOUR_DIRECTIONS
            {
                let next = v + dir;

                if self.is_cell_empty(next)
                {
                    //println!("{}, {}", next.x,next.y);
                    gen.yield_(next).await;
                }
            }
        }
        else
        {
            for dir in Self::ALL_DIRECTIONS
            {
                let next = v + dir;

                if self.is_cell_empty(next)
                {
                    //println!("{}, {}", next.x,next.y);
                    gen.yield_(next).await;
                }
            }
        }
    }

    const FOUR_DIRECTIONS : [Vector2Int;4] =         
    [
        Vector2Int{x : 1, y : -1},
        Vector2Int{x : -1, y : -1},
        Vector2Int{x : -1, y : 1},
        Vector2Int{x : 1, y : 1}, 
    ];

    const DIAGONAL_DIRECTIONS : [Vector2Int;4] = 
    [
        Vector2Int{x : 1, y : 0},
        Vector2Int{x : 0, y : -1},
        Vector2Int{x : -1, y : 0},
        Vector2Int{x : 0, y : 1}
    ];    

    const ALL_DIRECTIONS : [Vector2Int;8] = 
    [
        Vector2Int{x : 1, y : -1},
        Vector2Int{x : -1, y : -1},
        Vector2Int{x : -1, y : 1},
        Vector2Int{x : 1, y : 1}, 

        Vector2Int{x : 1, y : 0},
        Vector2Int{x : 0, y : -1},
        Vector2Int{x : -1, y : 0},
        Vector2Int{x : 0, y : 1}
    ];

    fn is_tile_walkable(&self, key : Vector2Int) -> bool
    {
        self.walkable.contains(&key)
    }

    fn is_cell_empty(&self, position: Vector2Int) -> bool {
        return self.is_in_bounds(position) 
        && self.is_tile_walkable(position);
    }

    fn is_in_bounds(
        &self,
        position: Vector2Int) -> bool 
        {
            return position.x >= self.bounds.x_min
            && position.y >= self.bounds.y_min
            && position.x < self.bounds.x_max
            && position.y < self.bounds.y_max;
        }

}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Bounds
{
    pub x_min : i32, 
    pub y_min : i32, 
    pub x_max : i32, 
    pub y_max : i32
}

/// Checks the distance
// 
pub fn heuristic(a : Vector2Int, b : Vector2Int) -> f32
{
    let num = a - b;
    
    let value = (num.x * num.x + num.y * num.y) as f32;

    return value.sqrt();
} 