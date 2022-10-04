extern crate piston_window;
extern crate rand;

use piston_window::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fs::copy;
use std::io::BufReader;
use std::fs::File;

mod tetrimino;
use crate::tetrimino::Tetrimino;
use crate::tetrimino::TetriminoType;
use crate::TetriminoType::O;

type Well = [[u8;10];24];

struct GameState
{
    game_over: bool,
    fall_counter: u32,
    well : Well,
    tetrimino_bag: Vec<Tetrimino>,
    current_tetrimino: Tetrimino,
    next_tetrimino : Tetrimino,
    tetrimino_row: i32,
    tetrimino_col: i32
}



fn main() 
{

    let mut window: PistonWindow =
        WindowSettings::new("Tetrust", [1280,720])
            .exit_on_esc(true)
            .vsync(true)
            .build().unwrap();

    window.events.set_ups(60);
    /*         
    while let Some(e) = window.next(){
        window.draw_2d(&e, |c, g, _| {
            clear([0.5,0.5,0.5,1.0], g);
            rectangle([1.0,0.0,0.0,1.0],
            [0.0,0.0,100.0,100.0],
            c.transform, g);
        });
    }
    */

    let mut blink_counter = 0;

    let mut initial_bag = create_random_bag();
    let first_tetrimino = initial_bag.pop().unwrap();
    let second_tetrimino = initial_bag.pop().unwrap();

    let mut game_state = GameState{
        game_over: false,
        fall_counter: 0,
        well: [[0u8;10];24],
        tetrimino_bag: initial_bag,
        current_tetrimino: first_tetrimino,
        next_tetrimino: second_tetrimino,
        tetrimino_row: 2,
        tetrimino_col: 3
    };


    //Main Loop
    while let Some(e) = window.next()
    {
        match e
        {
            Event::Loop(Loop::Render(_args_not_used)) =>{
            //display(&mut window, &e)
            //display_tetrimino(&mut window, &e, 320.0, 115.0, &Tetrimino::new(O))
            //draw_well_blocks(&mut window, &e, &mut game_state.well)
            display(&mut window, &e, &game_state.tetrimino_row, &game_state.tetrimino_col,
                &game_state.current_tetrimino, &game_state.next_tetrimino, &mut game_state.well);
            }

            Event::Loop(Loop::Update(_args_also_not_used)) =>
            {
                if game_state.game_over
                {
                    if blink_counter == 15{
                        game_state.well = [[0u8;10];24];
                    }
                    if blink_counter == 30{
                        game_state.well == [[1u8;10];24];
                        blink_counter = 0;
                    }

                    blink_counter+=1;
                }

                else
                {
                    game_update(&mut game_state);
                }

                //println!("{}", collision(&Tetrimino::new(O), &mut game_state.well, &game_state.tetrimino_row, &game_state.tetrimino_col));
                //copy_to_well(&Tetrimino::new(O), &mut game_state.well, &game_state.tetrimino_row, &game_state.tetrimino_col)
            }

            _ => 
            {
                ()
            }
        }
    }
    
}

fn game_update(game_state: &mut GameState)
{
    if game_state.fall_counter < 20
    {
        game_state.fall_counter +=1;
    }
    else 
    {
        game_state.fall_counter = 0;

        if collision(&game_state.current_tetrimino, &game_state.well, &(game_state.tetrimino_row + 1), &game_state.tetrimino_col)
        {
            copy_to_well(&game_state.current_tetrimino, &mut game_state.well, &mut game_state.tetrimino_row, &mut game_state.tetrimino_col);

            if game_state.tetrimino_bag.is_empty() {game_state.tetrimino_bag = create_random_bag();}
            game_state.current_tetrimino = game_state.next_tetrimino;
            game_state.next_tetrimino = game_state.tetrimino_bag.pop().unwrap();
            
            game_state.tetrimino_row = 2;
            game_state.tetrimino_col = 3;


            if collision(&game_state.current_tetrimino, &game_state.well, &game_state.tetrimino_row, &game_state.tetrimino_col)
            {
                game_state.game_over = true;
            }
        }

        else {game_state.tetrimino_row +=1;}
    }

}

fn create_random_bag() -> Vec<Tetrimino>
{
    let mut tetrimino_bag: Vec<Tetrimino> = vec![Tetrimino::new(TetriminoType::O),
                                                 Tetrimino::new(TetriminoType::I),
                                                 Tetrimino::new(TetriminoType::T),
                                                 Tetrimino::new(TetriminoType::L),
                                                 Tetrimino::new(TetriminoType::J),
                                                 Tetrimino::new(TetriminoType::S),
                                                 Tetrimino::new(TetriminoType::Z), ];

    tetrimino_bag.shuffle(&mut thread_rng());
    tetrimino_bag.shuffle(&mut thread_rng());
    tetrimino_bag.shuffle(&mut thread_rng());

    tetrimino_bag
}

/* 
fn display(win : &mut PistonWindow, re: &Event, row: &i32, col: &i32, curr: &Tetrimino, well: &Well)
{
    win.draw_2d(re, |c,g, _| {clear([0.5;4], g);});
}
*/

fn display(win : &mut PistonWindow, e: &Event, row: &i32, col: &i32, current : &Tetrimino, next : &Tetrimino, well: &Well )
{
    win.draw_2d(e, |c,g, _| {clear([0.5;4], g);});

    win.draw_2d(e, |c, g, _|{rectangle([0.0,0.0,0.0,1.0], [463.0,-140.0, 354.0, 842.0], c.transform, g);});

    draw_well_blocks(win, e, well);
    display_well(win, e, row, col, current);
    //display_tetrimino(win, e, 320.0, 115.0, tetrimino)
    
}


fn display_well(win: &mut PistonWindow, e: &Event, well_row: &i32, well_col: &i32, tetrimino: &Tetrimino)
{
    let(x,y) = well_to_pixel(*well_row, *well_col);
    display_tetrimino(win, e, x, y, tetrimino);
}

fn display_tetrimino(win : &mut PistonWindow, e : &Event, px:f64, py: f64, tetrimino: &Tetrimino)
{
    for tetrimino_row in 0..4{
        for tetrimino_col in 0..4
        {
            if tetrimino.shape[tetrimino_row][tetrimino_col] == 0 {continue;}

            let x = px + 35.0 * tetrimino_col as f64;
            let y = py + 35.0 * tetrimino_row as f64;

            win.draw_2d(e, |c, g, _|{
                rectangle(tetrimino.color, [x + 1.0, y + 1.0, 33.0 , 33.0], c.transform, g) ;
            });
        }
    }
}

fn draw_well_blocks(win : &mut PistonWindow, e : &Event, well : &Well)
{
    for row in 0..24{
        for col in 0..10{

            if well[row][col] == 0 {continue;}

            let(x, y) = well_to_pixel(row as i32, col as i32);
            win.draw_2d(e, 
             |c, g, _| {
                rectangle([1.0,1.0,1.0,1.0], [x + 1.0, y + 1.0, 33.0, 33.0], c.transform, g);
             });

        }
    }
}

fn well_to_pixel(row: i32, col: i32) -> (f64,f64)
{
    ( (col as f64) * 35.0 + 465.0, (row as f64) * 35.0 - 140.0)
}

fn collision(tetrimino : &Tetrimino, well: &Well, row: &i32, col: &i32) -> bool
{
    let mut well_row : i32;
    let mut well_col : i32;

    for tetrimino_row in 0..4{
        for tetrimino_col in 0..4{

            //Il ne peut pas se produire de collisions aux endroits où il n'y a pas de blocs
            if tetrimino.shape[tetrimino_row][tetrimino_col] == 0 {continue;}   

            //Ici on affiche les coordonnées du tetrimino dans le well
            well_row = tetrimino_row as i32 + *row;
            well_col = tetrimino_col as i32 + *col;

            //Collisions avec le sol et murs
            if well_col < 0 {return true;}
            if well_col > 9 {return true;}
            if well_row > 23{return true;}

            //Collision avec un bloc déjà ancré dans le well
            if well[well_row as usize][well_col as usize] !=0 {return true;}

        }
    }

    false
}

//Fonction de copie des tetriminos dans le well à la ligne et colonne correspondante
fn copy_to_well(tetrimino: &Tetrimino, well: &mut Well, well_row: &i32, well_col: &i32)
{
    for row in 0..4{
        for col in 0..4{
            if tetrimino.shape[row][col] == 0{continue;}
            well[(*well_row + row as i32) as usize][(*well_col + col as i32) as usize] = tetrimino.shape[row][col];
        }
    }
}