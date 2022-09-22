use std::collections::BinaryHeap;
use genawaiter::stack::let_gen_using;
use crate::{vector2int::{RustVector2Int as Vector2Int, Node}, graph::{heuristic, Graph, astar}};

pub fn hide(hider : Vector2Int, hide_from : Vector2Int, graph : &Graph) -> Vec<Vector2Int>
{
    let mut open : BinaryHeap<Node> = BinaryHeap::new();

    for i in &graph.walls
    {
        let wall = *i;
        open.push(Node::new(wall, heuristic(hider, wall) as usize));
    }

    while open.len() > 0
    {
        let current = open.pop().unwrap().value;

        let_gen_using!(neighbors, |co| graph.hide_place(current,hide_from, co));

        let mut priority = 0.0;
        let mut hide_place = Vector2Int::default();

        for next in neighbors
        {

            let dist = heuristic(hide_from, next);
            if priority < dist
            {
                priority = dist;
                hide_place = next;
            }
        }
        println!("goal: {} {}", hide_place.x , hide_place.y);

        match astar::find_path_internal(hider, hide_place, graph) 
        {
            Some(path) =>{println!("found path"); return path},
            None => {println!("unreacheable"); continue},
        };
    }

    vec![hider]

}


#[test]
fn test() 
{

    let walkable = vec!
        [
            Vector2Int::new(0, 0),
            Vector2Int::new(1, 0),
            Vector2Int::new(2, 0),
            Vector2Int::new(3, 0),
            Vector2Int::new(4, 0),
            Vector2Int::new(5, 0),
            
            Vector2Int::new(0, 1),
            Vector2Int::new(1, 1),
            Vector2Int::new(2, 1),
            Vector2Int::new(3, 1),
            Vector2Int::new(4, 1),
            Vector2Int::new(5, 1),
    
            Vector2Int::new(0, 2),
            Vector2Int::new(1, 2),
            //Vector2Int::new(2, 2),
            Vector2Int::new(3, 2),
            //Vector2Int::new(4, 2),
            Vector2Int::new(5, 2),
    
            Vector2Int::new(0, 3),
            Vector2Int::new(1, 3),
            Vector2Int::new(2, 3),
            Vector2Int::new(3, 3),
            Vector2Int::new(4, 3),
            Vector2Int::new(5, 3),
    
            Vector2Int::new(0, 4),
            Vector2Int::new(1, 4),
            Vector2Int::new(2, 4),
            Vector2Int::new(3, 4),
            Vector2Int::new(4, 4),
            Vector2Int::new(5, 4),
    
            Vector2Int::new(0, 5),
            Vector2Int::new(1, 5),
            Vector2Int::new(2, 5),
            Vector2Int::new(3, 5),
            Vector2Int::new(4, 5),
            Vector2Int::new(5, 5),
        ];

        let walls = vec!
        [
            Vector2Int::new(2, 2),
            Vector2Int::new(4, 2),
        ];

        let graph = Graph::new(crate::graph::Bounds{ x_min: 0, y_min: 0, x_max:6, y_max: 6 },walkable, walls);


    hide(Vector2Int::new(0, 5), Vector2Int::new(3, 0), &graph);
}