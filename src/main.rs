use macroquad::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
struct Node{
    parent: Option<Rc<Node>>,
    position: (i32, i32),
    
    g: i32,
    h: i32,
    f: i32,
}

impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool{
        self.position == other.position
    }
}

impl Node{
    pub fn new(parent: Option<Rc<Node>>, position: (i32, i32)) -> Self{
        Node{
            parent,
            position,
            g: 0,
            h: 0,
            f: 0,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RectType{
    Obstacle,
    Nothing,
}

pub struct MazeRect{
    pub rect: Rect,
    pub r_type: RectType,
    pub x: usize,
    pub y: usize,
}

impl MazeRect{
    pub fn draw(&self, color: Color){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }

    #[inline]
    pub fn check_mouse(&self) -> bool{
        let (x, y) = mouse_position();
        let rec = self.rect;
        
        (x >= rec.x) && (x <= (rec.x + rec.w)) && (y >= rec.y) && (y <= (rec.y + rec.h))
    }
}

pub fn astar(maze: &Vec<Vec<RectType>>, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)>{
    let start_node = Node::new(None, start);
    let end_node = Node::new(None, end);

    let mut open_list = Vec::new();
    let mut closed_list = Vec::new();

    open_list.push(start_node);

    while !open_list.is_empty() {
        let mut current_node = open_list[0].clone();
        let mut current_index = 0;

        for (i, node) in open_list.iter().enumerate(){
            if node.f < current_node.f{
                current_node = node.clone();
                current_index = i;
            }
        }
        
        open_list.remove(current_index);
        closed_list.push(current_node.clone());

        if current_node == end_node{
            let mut current = Some(Rc::new(current_node));
            let mut path = Vec::new();

            while current.is_some(){
                path.push(current.as_ref().unwrap().clone().position);

                //thank you rust
                current = match &current.as_ref().unwrap().clone().parent{
                    Some(parent) => Some(parent.clone()),
                    None => None,
                };
            }
            path.reverse();
            return path;
        }

        let mut children = Vec::new();

        for new_pos in [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)]{
            let node_pos = (current_node.position.0 + new_pos.0, current_node.position.1 + new_pos.1);

            if node_pos.0 > (maze.len() - 1) as i32 || node_pos.0 < 0 || node_pos.1 > (maze[maze.len()-1].len() - 1) as i32 || node_pos.1 < 0{
                continue;
            }

            if maze[node_pos.0 as usize][node_pos.1 as usize] != RectType::Nothing{
                continue;
            }

            let new_node = Node::new(Some(Rc::new(current_node.clone())), node_pos);
            children.push(new_node.clone());
        }

        'child_loop: for child in children.iter_mut(){
            for closed_child in closed_list.iter(){
                if child == closed_child {
                    continue 'child_loop;
                }
            }

            child.g = current_node.g + 1;
            child.h = (child.position.0 - end_node.position.0) * (child.position.0 - end_node.position.0)
                +     (child.position.1 - end_node.position.1) * (child.position.1 - end_node.position.1);

            child.f = child.g + child.h;

            for open_node in open_list.iter(){
                if child == open_node && child.g > open_node.g{
                    continue 'child_loop;
                }
            }

            open_list.push((*child).clone());
        }
    }
    Vec::new()
}

pub fn render_maze(maze: &Vec<Vec<i32>>){
    for (x, x_array) in maze.iter().enumerate(){
        for (y, character) in x_array.iter().enumerate(){
            match character{
                1 => {
                    draw_rectangle(((y as f32 + 0.05) * 80f32) as f32, ((x as f32 + 0.05) * 80f32) as f32, 70f32, 70f32, BLACK);
                },
                0 => {
                    draw_rectangle(((y as f32 + 0.05) * 80f32) as f32, ((x as f32 + 0.05) * 80f32) as f32, 70f32, 70f32, WHITE);
                },
                _ => {},
            };
        }
    }
}

pub fn setup_rects(maze: &Vec<Vec<RectType>>) -> Vec<MazeRect>{
    let mut rect_vec = Vec::new();
    
    for (x, x_array) in maze.iter().enumerate(){
        for (y, character) in x_array.iter().enumerate(){
            rect_vec.push(
                MazeRect{rect: Rect::new(((y as f32 + 0.05) * 80f32) as f32,
                                         ((x as f32 + 0.05) * 80f32) as f32,
                                         70f32, 70f32),
                         r_type: *character,
                         x: y,
                         y: x,
                }
            )
        }
    }

    rect_vec
}

pub fn render_path(path: &Vec<(i32, i32)>){
    if !path.is_empty(){
        for i in 0..(path.len() - 1){
            if i != path.len(){
                let x1 = path[i].1 as f32;
                let y1 = path[i].0 as f32;
                let x2 = path[i+1].1 as f32;
                let y2 = path[i+1].0 as f32;

                draw_line((x1 * 80.0) + 35.0, (y1 * 80.0) + 35.0, (x2 * 80.0) + 35.0, (y2 * 80.0) + 35.0, 4f32, LIME);
            }
        }
    }
}

#[inline]
pub fn render_maze_rects(rects: &Vec<MazeRect>){
    for rect in rects.iter(){
        match rect.r_type{
            RectType::Obstacle => rect.draw(BLACK),
            RectType::Nothing => rect.draw(WHITE),
        };
    }
}

#[inline]
pub fn update_path(rect: &mut MazeRect, maze: &mut Vec<Vec<RectType>>, new_type: RectType){
    rect.r_type = new_type;
    maze[rect.y][rect.x] = new_type;
}

fn window_conf() -> Conf{
    Conf{
        window_title: "A* Pathfinding".to_owned(),
        window_height: 800,
        window_width: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main(){
    let mut maze: Vec<Vec<RectType>> = vec!(vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Obstacle, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing],
                                            vec![RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing, RectType::Nothing]);
    
    let mut start = (0, 0);
    let mut end = (7, 6);
    
    let mut path = astar(&maze, start, end);

    let mut maze_as_rects = setup_rects(&maze);
    
    loop {
        clear_background(GRAY);
        
        render_maze_rects(&maze_as_rects);
        render_path(&path);
        draw_circle((start.1 as f32 * 80.0) + 35.0 , (start.0 as f32 * 80.0) + 35.0, 20f32, DARKBLUE);
        draw_circle((end.1 as f32 * 80.0) + 35.0 , (end.0 as f32 * 80.0) + 35.0, 20f32, DARKBROWN);
        
        if is_mouse_button_pressed(MouseButton::Left){
            for rect in maze_as_rects.iter_mut() {
                if rect.check_mouse()
                    && rect.r_type == RectType::Nothing{
                        update_path(rect, &mut maze, RectType::Obstacle);
                        path = astar(&maze, start, end);
                    }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right){
            for rect in maze_as_rects.iter_mut() {
                if rect.check_mouse()
                    && rect.r_type == RectType::Obstacle{
                        update_path(rect, &mut maze, RectType::Nothing);
                        path = astar(&maze, start, end);          
                    }
            }
        }
        
        if is_key_pressed(KeyCode::Space){
            for rect in maze_as_rects.iter_mut() {
                if rect.check_mouse(){
                    start = (rect.y as i32, rect.x as i32);
                    path = astar(&maze, start, end);
                }
            }
        }

        if is_key_pressed(KeyCode::Enter){
            for rect in maze_as_rects.iter_mut() {
                if rect.check_mouse(){
                    end = (rect.y as i32, rect.x as i32);
                    path = astar(&maze, start, end);
                }
            }
        }
        
        next_frame().await;
    }
}
