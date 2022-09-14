use std::collections::BinaryHeap;
use ahash::AHashMap;
use crate::graph::{Graph, heuristic};
use graph::Bounds;
use vector2int::{RustVector2Int as Vector2Int, Node};
pub mod graph;
pub mod vector2int;

///

#[repr(C)]
pub struct Buffer
{
    pub len : usize,
    pub tilelist: *mut Vector2Int,
}

#[no_mangle]
pub extern "C" fn create_vec() -> *mut Vec<Vector2Int>
{
    Box::into_raw(Box::new(Vec::new()))
}

///

#[no_mangle]
pub extern "C" fn add_to_vec(vec_ptr : *mut Vec<Vector2Int>, pos : Vector2Int)
{
    let vec = unsafe {&mut *vec_ptr};
    vec.push(pos)
}

///

#[no_mangle]
pub extern "C" fn free_buffer(buf : Buffer) 
{
    let slice = unsafe { std::slice::from_raw_parts_mut(buf.tilelist, buf.len) };
    let slice = slice.as_mut_ptr();
    unsafe { Box::from_raw(slice) };
}

///

#[no_mangle]
pub unsafe extern "C" fn allocate_graph(vec_ptr: *mut Vec<Vector2Int>, bounds : Bounds) -> *mut Graph
{
    Box::into_raw(Box::new(Graph::new(bounds, *Box::from_raw(vec_ptr))))
}

#[no_mangle]
pub extern "C" fn deallocate_graph(graph_ptr : *mut Graph)
{
    //pointer null check should be done from the caller
    unsafe { Box::from_raw(graph_ptr) }; 
}

///

#[no_mangle]
pub extern "C" fn find_path(start: Vector2Int, goal: Vector2Int, graph_ptr : *const Graph) -> Buffer
{
    debug_assert!(!graph_ptr.is_null(), "pointer is null");

    let mut buf = unsafe{find_path_loc(start, goal, &*graph_ptr)}.into_boxed_slice();
    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);

    Buffer{len, tilelist:ptr }
}

///

#[test]
fn ss()
{
    println!("{}", std::mem::size_of::<Buffer>());
}

#[inline]
fn find_path_loc(start: Vector2Int, goal: Vector2Int, graph : &Graph) -> Vec<Vector2Int>
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

    let mut path = Vec::new();
    
    /*for ss in &came_from
    {
        println!("keys {} {},values {} {}", ss.0.x, ss.0.y, ss.1.x, ss.1.y);
    }*/

    if came_from.contains_key(&goal)
    {
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

    }
    else
    {
        //returning an empty vec to c# would make it crash because it would
        // try to get values from a non existing struct
        path.push(start);

        // println!("No path found")
    }

    return path;

}

///////

#[test]
pub fn ex()
{
    let mut map :Vec<Vector2Int> = Vec::new();
    let size = 20;

    for x in 0..size 
    {
        for y in 0..size 
        {
            map.push(Vector2Int::new(x,y));
        }
    }
    
    let vec = create_vec();

    for vector2int in &map
    {
        add_to_vec(vec, *vector2int);
    }

    let graph_ptr = unsafe 
    {
        allocate_graph(vec ,Bounds { x_min: 0, y_min: 0 , x_max: map.last().unwrap().x + 1, y_max: map.last().unwrap().y + 1 })
    };

    find_path(Vector2Int::new(0,0),Vector2Int::new(map.last().unwrap().x,map.last().unwrap().y), graph_ptr);

    deallocate_graph(graph_ptr);

}
