use crate::graph::{Graph, Bounds};
use crate::vector2int::RustVector2Int as Vector2Int;

#[repr(C)]
pub struct Buffer
{
    pub len : usize,
    pub tilelist: *mut Vector2Int,
}
///

#[test]
fn size_of()
{
    println!("{}", std::mem::size_of::<Buffer>());
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
pub unsafe extern "C" fn allocate_graph(walkables_ptr: *mut Vec<Vector2Int>, walls_ptr: *mut Vec<Vector2Int>, bounds : Bounds) -> *mut Graph
{
    Box::into_raw(Box::new(Graph::new(bounds, *Box::from_raw(walkables_ptr),*Box::from_raw(walls_ptr))))
}

#[no_mangle]
pub extern "C" fn deallocate_graph(graph_ptr : *mut Graph)
{
    //pointer null check should be done from the caller
    unsafe { Box::from_raw(graph_ptr) }; 
}

///

#[no_mangle]
pub extern "C" fn hide(hider: Vector2Int, hide_from: Vector2Int, graph_ptr : *const Graph) -> Buffer
{
    let mut buf = unsafe{ crate::graph::hide::hide(hider, hide_from, &*graph_ptr)}.into_boxed_slice();
    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);

    Buffer{len, tilelist:ptr }
}

#[no_mangle]
pub extern "C" fn find_path(start: Vector2Int, goal: Vector2Int, graph_ptr : *const Graph) -> Buffer
{
    debug_assert!(!graph_ptr.is_null(), "pointer is null");

    let mut buf = match unsafe{ crate::graph::astar::find_path_internal(start, goal, &*graph_ptr)}
    {
        Some(path) => path.into_boxed_slice(),
        None => vec![start].into_boxed_slice(),
    };

    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);

    Buffer{len, tilelist:ptr }
}


#[test]
fn hide_test()
{
use rand::Rng;
let mut map :Vec<Vector2Int> = Vec::new();
let size = 20;

for x in 0..size 
{
    for y in 0..size 
    {
        map.push(Vector2Int::new(x,y));
    }
}

let walkables = create_vec();
let walls = create_vec();

for vector2int in &map
{
    if weighted_chance(90)
    {
        add_to_vec(walkables, *vector2int);
    }
    else
    {
        add_to_vec(walls, *vector2int);
    }
}

let graph_ptr = unsafe 
{
    allocate_graph(walkables , walls, Bounds { x_min: 0, y_min: 0 , x_max: map.last().unwrap().x + 1, y_max: map.last().unwrap().y + 1 })
};

hide(Vector2Int::new(0,5),Vector2Int::new(map.last().unwrap().x,map.last().unwrap().y), graph_ptr);

deallocate_graph(graph_ptr);

fn weighted_chance(chance : i32) -> bool
{
    return rand::thread_rng().gen_range(0..=100) < chance
}

}

#[test]
fn path_test()
{
use rand::Rng;
let mut map :Vec<Vector2Int> = Vec::new();
let size = 20;

for x in 0..size 
{
    for y in 0..size 
    {
        map.push(Vector2Int::new(x,y));
    }
}

let walkables = create_vec();
let walls = create_vec();

for vector2int in &map
{
    if weighted_chance(90)
    {
        add_to_vec(walkables, *vector2int);
    }
    else
    {
        add_to_vec(walls, *vector2int);
    }
}

let graph_ptr = unsafe 
{
    allocate_graph(walkables , walls, Bounds { x_min: 0, y_min: 0 , x_max: map.last().unwrap().x + 1, y_max: map.last().unwrap().y + 1 })
};

find_path(Vector2Int::new(0,0),Vector2Int::new(map.last().unwrap().x,map.last().unwrap().y), graph_ptr);

deallocate_graph(graph_ptr);

fn weighted_chance(chance : i32) -> bool
{
    return rand::thread_rng().gen_range(0..=100) > chance
}

}