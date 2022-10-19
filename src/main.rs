//imports

use rand::Rng;
use std::io;

//main function
fn main() {
  //var to show the game is playing and not been stopped
  let mut rematch = true;

  //while not false the game will continuesly run
  while rematch != false {
    //initialize players
    let _playerx = "|X|".to_string();
    let _playero = "|O|".to_string();

    //vector to compare the inputed values
    let _draw = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    //checking if the user is the first player
    let mut is_first_player = false;

    //initialize screen output
    //these get converted to strings to be used later when input is given to fill box
    let mut _box1 = "|1|".to_string();
    let mut _box2 = "|2|".to_string();
    let mut _box3 = "|3|".to_string();
    let mut _box4 = "|4|".to_string();
    let mut _box5 = "|5|".to_string();
    let mut _box6 = "|6|".to_string();
    let mut _box7 = "|7|".to_string();
    let mut _box8 = "|8|".to_string();
    let mut _box9 = "|9|".to_string();

    println!(
      "{}",
      format!(
        "{}{}{}\n{}{}{}\n{}{}{}",
        _box1, _box2, _box3, _box4, _box5, _box6, _box7, _box8, _box9
      )
    );

    //var used to store the player that will not be going first
    let mut other_user = " ";

    //vector that is used to store the user and computer input
    //will be used to check values contained within
    let mut choosen_array: Vec<i32> = Vec::new();

    //get first player and set first player to X and the other user to O
    let firstplayer = get_turn();
    println!("The first player is {}", firstplayer);
    if firstplayer == "User" {
      is_first_player = true;
      other_user = "Computer";
    } else if other_user != "Computer" {
      other_user = "User";
    }

    //while the game is running
    while rematch != false {
      //if the game is a draw then set rematch to false and break
      let matchdraw = _draw.iter().all(|val| choosen_array.contains(val));
      if matchdraw == true {
        rematch == false;
        println!("Game has ended a draw! Thank you for playing");
        break;
      }
      //while it is the user to get input
      //call the get_input() func to get user input
      //compare the input to vals in array if contained ask again
      //else will input the number to the chosen array vector
      while is_first_player != false {
        let mut board_exist = false;
        println!("Please enter a number between 1-9: ");
        let mut numberboard = get_input();
        println!("your number {}", numberboard);
        while board_exist == false {
          if choosen_array.contains(&numberboard) {
            println!("this box is taken {}", numberboard);
            println!("please enter a new number 1-9:");
            numberboard = get_input();
          } else {
            choosen_array.push(numberboard);
            board_exist = true;
          }
        } //if the inputed number equals 1..9 will input x or o depending on whether the user is the first player or not
        if numberboard == 1 {
          if firstplayer == "User" {
            _box1 = "|X|".to_string();
          } else {
            _box1 = "|O|".to_string();
          }
        } else if numberboard == 2 {
          if firstplayer == "User" {
            _box2 = "|X|".to_string();
          } else {
            _box2 = "|O|".to_string();
          }
        } else if numberboard == 3 {
          if firstplayer == "User" {
            _box3 = "|X|".to_string();
          } else {
            _box3 = "|O|".to_string();
          }
        } else if numberboard == 4 {
          if firstplayer == "User" {
            _box4 = "|X|".to_string();
          } else {
            _box4 = "|O|".to_string();
          }
        } else if numberboard == 5 {
          if firstplayer == "User" {
            _box5 = "|X|".to_string();
          } else {
            _box5 = "|O|".to_string();
          }
        } else if numberboard == 6 {
          if firstplayer == "User" {
            _box6 = "|X|".to_string();
          } else {
            _box6 = "|O|".to_string();
          }
        } else if numberboard == 7 {
          if firstplayer == "User" {
            _box7 = "|X|".to_string();
          } else {
            _box7 = "|O|".to_string();
          }
        } else if numberboard == 8 {
          if firstplayer == "User" {
            _box8 = "|X|".to_string();
          } else {
            _box8 = "|O|".to_string();
          }
        } else if numberboard == 9 {
          if firstplayer == "User" {
            _box9 = "|X|".to_string();
          } else {
            _box9 = "|O|".to_string();
          }
        } //print the box to the user after the turn has been made
        let _ui = format!(
          "{}{}{}\n{}{}{}\n{}{}{}",
          _box1, _box2, _box3, _box4, _box5, _box6, _box7, _box8, _box9
        );
        println!("{}", _ui);
        is_first_player = false;
      }
      while is_first_player != true {
        let mut board_exist = false;
        let mut ai_input = ai_turn();
        while board_exist == false {
          if choosen_array.contains(&ai_input) {
            ai_input = ai_turn();
          } else {
            println!("The Computer choose square {}", ai_input);
            choosen_array.push(ai_input);
            board_exist = true;
          }
        }
        if ai_input == 1 {
          if firstplayer == "AI" {
            _box1 = "|X|".to_string();
          } else {
            _box1 = "|O|".to_string();
          }
        } else if ai_input == 2 {
          if firstplayer == "AI" {
            _box2 = "|X|".to_string();
          } else {
            _box2 = "|O|".to_string();
          }
        } else if ai_input == 3 {
          if firstplayer == "AI" {
            _box3 = "|X|".to_string();
          } else {
            _box3 = "|O|".to_string();
          }
        } else if ai_input == 4 {
          if firstplayer == "AI" {
            _box4 = "|X|".to_string();
          } else {
            _box4 = "|O|".to_string();
          }
        } else if ai_input == 5 {
          if firstplayer == "AI" {
            _box5 = "|X|".to_string();
          } else {
            _box5 = "|O|".to_string();
          }
        } else if ai_input == 6 {
          if firstplayer == "AI" {
            _box6 = "|X|".to_string();
          } else {
            _box6 = "|O|".to_string();
          }
        } else if ai_input == 7 {
          if firstplayer == "AI" {
            _box7 = "|X|".to_string();
          } else {
            _box7 = "|O|".to_string();
          }
        } else if ai_input == 8 {
          if firstplayer == "AI" {
            _box8 = "|X|".to_string();
          } else {
            _box8 = "|O|".to_string();
          }
        } else if ai_input == 9 {
          if firstplayer == "AI" {
            _box9 = "|X|".to_string();
          } else {
            _box9 = "|O|".to_string();
          }
        }
        let _ui = format!(
          "{}{}{}\n{}{}{}\n{}{}{}",
          _box1, _box2, _box3, _box4, _box5, _box6, _box7, _box8, _box9
        );
        println!("{}", _ui);
        is_first_player = true; //next players turn
      }

      //X across checking for win
      if _box1 == "|X|" && _box2 == "|X|" && _box3 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      } else if _box4 == "|X|" && _box5 == "|X|" && _box6 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      } else if _box7 == "|X|" && _box8 == "|X|" && _box9 == "|X|" {
        println!("The {} is the winner", firstplayer);
        break;
      }
      //O across checking for win
      else if _box1 == "|O|" && _box2 == "|O|" && _box3 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      } else if _box4 == "|O|" && _box5 == "|O|" && _box6 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      } else if _box7 == "|O|" && _box8 == "|O|" && _box9 == "|O|" {
        println!("The {} is the winner", other_user);
        break;
      }
      //X downwards checking for win
      else if _box1 == "|X|" && _box4 == "|X|" && _box7 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      } else if _box2 == "|X|" && _box5 == "|X|" && _box8 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      } else if _box3 == "|X|" && _box6 == "|X|" && _box9 == "|X|" {
        println!("The {} is the winner", firstplayer);

        break;
      }
      //O downwards checking for win
      else if _box1 == "|O|" && _box4 == "|O|" && _box7 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      } else if _box2 == "|O|" && _box5 == "|O|" && _box8 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      } else if _box3 == "|O|" && _box6 == "|O|" && _box9 == "|O|" {
        println!("The {} is the winner", other_user);
        break;
      }
      //X diag checking for win
      else if _box1 == "|X|" && _box5 == "|X|" && _box9 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      } else if _box3 == "|X|" && _box5 == "|X|" && _box7 == "|X|" {
        println!("{} is the winner", firstplayer);
        break;
      }
      //O diag checking for win
      else if _box1 == "|O|" && _box5 == "|O|" && _box9 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      } else if _box3 == "|O|" && _box5 == "|O|" && _box7 == "|O|" {
        println!("{} is the winner", other_user);
        break;
      }
    }

    //afer the loops have been broken with either a win or draw will ask the user
    //if they would like to play again and take the users input take it to a string
    //convert it to uppercase
    //if Y is entered restarts the game
    //else prints a thank you for playing message to the user
    println!("Would you like to play again? Y/N");
    let mut play_again = String::new();
    io::stdin()
      .read_line(&mut play_again)
      .expect("Readline Err");
    let game_complete: String = play_again.trim().parse().unwrap();
    if game_complete.to_uppercase() == "Y" {
      rematch = true;
    } else if game_complete.to_uppercase() == "N" {
      println!("Thank you for playing! :)");
      break;
    }
  }
}
//function to get the turn based of random number
//for future implementation can ask user to input a number and use that within this function
fn get_turn() -> String {
  let mut turn = " ".to_string();

  let rand_num = rand::thread_rng().gen_range(1..100);

  if rand_num <= 50 {
    turn = "User".to_string();
  } else if rand_num >= 51 {
    turn = "AI".to_string();
  }
  return turn;
}

//gets the users number for the board and returns i32 to main
fn get_input() -> i32 {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  let number: i32 = line.trim().parse().unwrap();
  return number;
}

//gets the computers turn generates number between 1 and 9 and returns that number
fn ai_turn() -> i32 {
  let ai_square = rand::thread_rng().gen_range(1..9);
  return ai_square;
}
