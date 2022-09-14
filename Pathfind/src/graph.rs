use crate::vector2int::RustVector2Int as Vector2Int;
use genawaiter::stack::Co;

#[repr(C)]
pub struct Graph
{
    pub tilelist : Vec<Vector2Int>,
    pub bounds : Bounds,

    pub four_directions : [Vector2Int;4],
    pub diagonal_directions : [Vector2Int;4],
}

impl Graph
{
    pub fn new(bounds: Bounds, tilelist : Vec<Vector2Int>) -> Self {Self { tilelist, bounds, four_directions: 
        Self::FOUR_DIRECTIONS, diagonal_directions: Self::DIAGONAL_DIRECTIONS}}   

    pub fn neighbors(&self, v : Vector2Int) -> Vec<Vector2Int>
    {
        let mut neighbors = Vec::new();

        /*println!("straight");
        println!("current tile {}, {}", v.x, v.y);
        */

        for dir in self.four_directions 
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
        for dir in self.diagonal_directions 
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

    pub async fn neighbors_routine(&self,  v : Vector2Int,gen :  Co<'_, Vector2Int>)
    {
        /*println!("straight");
        println!("current tile {}, {}", v.x, v.y);
        */
        for dir in self.four_directions 
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
        for dir in self.diagonal_directions 
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

    fn is_tile_empty(&self, key : Vector2Int) -> bool
    {
        //since the tiles in the tilelist are checked in c# to be empty
        //they are a list of empty positions
        self.tilelist.contains(&key)
    }

    pub fn is_cell_empty(&self, position : Vector2Int) -> bool
    {
        return self.is_in_bounds(position, self.bounds.x_min, self.bounds.y_min, self.bounds.x_max, self.bounds.y_max) 
        && self.is_tile_empty(position)
    }

    fn is_in_bounds(&self, position : Vector2Int, x_min : i32, y_min : i32, x_max : i32, y_max : i32) -> bool
    {
        return position.x >= x_min && position.y >= y_min && position.x < x_max && position.y < y_max;
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