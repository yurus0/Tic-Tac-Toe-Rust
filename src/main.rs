use std::io;
use std::io::stdin;
use std::io::prelude::*;
use colored::Colorize;

fn game_begin(tab : & [[String; 3]; 3])-> bool{
    //check if the game has begun
    let mut counter = 1;
    for i in 0..3 {
        for j in 0..3 {
            if tab[i][j] != counter.to_string() {
                return false;
            }
            counter +=1;
        }
    }
    return true;
}

fn game_ended(tab: &mut [[String; 3]; 3])-> i32 {
    //draw check
    let mut counter=0;
    for i in 0..3 {
        for j in 0..3 {
            if tab[i][j] == "X" || tab[i][j] == "O" {
                counter+=1;
            }
        }
    }
    if counter==9 {
        return 1;
    }
    //win check
    //check if the game has ended
    //checking diagonals
    if tab[0 as usize][0 as usize]==tab[1 as usize][1 as usize] && tab[1 as usize][1 as usize]==tab[2 as usize][2 as usize] {
        return 2;
    } else if tab[0 as usize][2 as usize]==tab[1 as usize][1 as usize] && tab[1 as usize][1 as usize]==tab[2 as usize][0 as usize] {
        return 2;
    }
    //checking other winning cases
    for i in 0..3 as usize {
        if tab[i][0]==tab[i][1] && tab[i][1]==tab[i][2] {
            return 2;
        } else if tab[0][i]==tab[1][i] && tab[1][i]==tab[2][i] {
            return 2;
        }
    }
    return 0;
}


fn display_status( tab: &mut [[String; 3]; 3]) {
    print!("\n");
    for i in 0..3 as usize {
        print!("{}", "+-----------+\n|".bright_white().bold());
        for j in 0..3 as usize {
            if tab[i][j]=="X" {
                print!( " {} |", tab[i][j].bright_red().bold());
            } else if tab[i][j]=="O" {
                print!( " {} |", tab[i][j].bright_blue().bold());
            } else {
                print!( " {} |", tab[i][j]);
            }
        }
        print!("\n");
    }
    println!("{}", "+-----------+\n".bright_white().bold());
}

fn update_status(player_num: i32, pos:i32, tab : &mut [[String; 3]; 3]){
    //update the board with the player's choice
    //10 -> X
    //0 -> O
    //additional condition: if already filled, then keep asking
    //for another choice until a valid choice is entered
    let mut i=0;
    let mut j=0;
    if pos<4{
        i=0;
    } else if pos<7 && pos>3{
        i=1;
    } else if pos>6 && pos<10 {
        i=2;
    } if pos==1 || pos==4 || pos==7 {
        j=0;
    } else if pos==2 || pos==5 || pos==8 {
        j=1;
    } else if pos==3 || pos==6 || pos==9 {
        j=2;
    }
    if tab[i as usize ][j as usize] == "X" || tab[i as usize][j as usize] == "O" {
        println!("This position is already filled. Please choose another position > ");
        let mut pos = String::new();
        stdin().read_line(&mut pos).expect("Failed to read line.");
        let pos : i32 = pos.trim().parse().expect("Please enter a number!");
        update_status(player_num, pos, tab);
    } else {
        tab[i as usize][j as usize]= if player_num==1 {"X".to_string()} else {"O".to_string()};
    }
}

fn main() {
    let mut tab : [[String; 3]; 3] = [["1".to_string(), "2".to_string(), "3".to_string()], ["4".to_string(), "5".to_string(), "6".to_string()], ["7".to_string(), "8".to_string(), "9".to_string()]];
    let mut player=1;
    loop {
        print!("\n");
        let mut pos = String::new();
        if game_begin(&mut tab) {
            println!("{} {} {} {}{}", "Welcome to".bold(), "Tic".bright_red().bold(), "Tac".bright_blue().bold(), "Toe".bright_yellow().bold(), "!".bold());
            display_status(&mut tab);
            print!("Please choose which player you want to be: 1 or 2.\nPlayer 1: {} \nPlayer 2: {}", "X".bright_red().bold(), "O".bright_blue().bold());
            io::stdout().flush().unwrap();
            print!("\n\nEnter your first choice > ");
            io::stdout().flush().unwrap();
            let mut input =String::new();
            stdin().read_line(&mut input).expect("Failed to read line.");
            let input : i32 =input.trim().parse().expect("Please enter a number! 1 or 2.");
            if input==1 {
                player=1;
            } else if input==2 {
                player=-1;
            }
        }
        let _input = if player==1 {1} else {2};
        display_status(&mut tab);
        let num_out= if _input==1 {_input.to_string().bright_red().bold()} else {_input.to_string().bright_blue().bold()};
        print!("Player {}'s turn. Enter a position > ", num_out);
        io::stdout().flush().unwrap();
        stdin().read_line(&mut pos).expect("Failed to read line.");
        let pos : i32 = pos.trim().parse().expect("Please enter a number!");
        update_status(player, pos, &mut tab);
        //play again?
        let c=game_ended(&mut tab);
        if c>0 {
            display_status(&mut tab);
            print!("{}\n", "GAME ENDED!".bold());
            if c==1 {
                print!("{}\n", "DRAW!".bright_yellow().bold());
            } else{
                print!("{}\n", "VICTORY!".bright_green().bold());
                print!("Player {} wins!\n", num_out);
            }
            print!("\nPlay again? (y/n) > ");
            io::stdout().flush().unwrap();
            let mut resp = String::new();
            stdin().read_line(&mut resp).expect("Failed to read line.");
            let resp : char = resp.trim().parse().expect("Please enter a valid response.");
            if resp == 'n' {
                break;
            } else {
                tab = [["1".to_string(), "2".to_string(), "3".to_string()], ["4".to_string(), "5".to_string(), "6".to_string()], ["7".to_string(), "8".to_string(), "9".to_string()]];
            }
        }
        player*=-1;
        let _input = if player==1 {1} else {2};
    }
}
