use std::collections::BinaryHeap;
use ahash::AHashMap;
use crate::graph::{Graph, heuristic};
use crate::vector2int::{RustVector2Int as Vector2Int, Node};

#[inline]
pub fn find_path_internal(start: Vector2Int, goal: Vector2Int, graph : &Graph) -> Option<Vec<Vector2Int>>
{
    use genawaiter::stack::let_gen_using;

    let mut open_list : BinaryHeap<Node> = BinaryHeap::new();

    open_list.push(Node::new(start, 0));

    let mut came_from : AHashMap<Vector2Int, Vector2Int> = AHashMap::new();
    came_from.insert(start, start);

    let mut costs : AHashMap<Vector2Int, f32> = AHashMap::new();
    costs.insert(start, 0.0);

    while open_list.len() > 0
    {
        let current = open_list.pop().unwrap().value;

        let_gen_using!(neighbors, |co| graph.neighbors_routine(current, co));

        if current.eq(&goal)
        {
            break;
        }

        for next in neighbors
        {
            //println!("vec {} {}", next.x, next.y);

            let new_cost = costs[&current] + heuristic(current, next);
            let next_cost = costs.get(&next);

            if next_cost.is_none() || new_cost < *next_cost.unwrap()
            {
                costs.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                open_list.push(Node::new(next, priority as usize));
                came_from.insert(next, current);
            }
        }
    }
  
    /*for ss in &came_from
    {
        println!("keys {} {},values {} {}", ss.0.x, ss.0.y, ss.1.x, ss.1.y);
    }*/

    if came_from.contains_key(&goal)
    {
        let mut path = vec![];

        let mut v = goal;

        while !v.eq(&start)
        {
            path.push(v);
            v = came_from[&v];
        }

        path.push(start);

        path.reverse();

        /*for vector2int in &path
        {
            println!("Pos,{},{}", vector2int.x , vector2int.y);
        }*/
        return Some(path);
    }
    return None;

}