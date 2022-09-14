use std::{cmp, ops::{Add, Sub}};

#[derive(Default,PartialEq, Eq,Hash,Clone, Copy)]
#[repr(C)]
pub struct RustVector2Int
{
    pub x : i32,
    pub y : i32,
}

#[derive(Eq)]
pub struct  Node 
{
    cost : usize,
    pub value : RustVector2Int,
}

impl Node {
    pub fn new(value: RustVector2Int, cost: usize) -> Self { Self { cost, value } }
}

impl PartialOrd for Node 
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl Ord for Node 
{
    fn cmp(&self, other: &Self) -> cmp::Ordering 
    {
        other.cost.cmp(&self.cost)
    }
}

impl PartialEq for Node 
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.cost == other.cost && self.value == other.value
    }
}

impl RustVector2Int 
{
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }

    pub fn clamp(&mut self, min : RustVector2Int,max : RustVector2Int)
    {
        self.x = cmp::max(min.x, self.x);
        self.x = cmp::min(max.x, self.x);
        self.y = cmp::max(min.y, self.y);
        self.y = cmp::min(max.y, self.y);
    }
}

impl Add for RustVector2Int
{
    type Output = RustVector2Int;

    fn add(self, other: Self) -> RustVector2Int
    {
        RustVector2Int
        {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl Sub for RustVector2Int
{
    type Output =  RustVector2Int;

    fn sub(self, other: Self) -> RustVector2Int
    {
        RustVector2Int
        {
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}