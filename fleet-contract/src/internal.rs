use crate::*;

//convert from position to int for example: A2 => 5
pub(crate) fn convert_position_to_coords (
    position: &String
) -> u32 {
    /*
        A can be 1, 5, 9, 12
        B can be 2, 6, 10, 14
        C can be 3, 7, 11, 15
        D can be 4, 8, 12, 16

        A => 1
        B => 2
        C => 3
        D => 4
        
        final coordinates = letter + (4 * (number - 1))
    */
    
    //get the iterator of characters
    let characters = position.chars();
    
    //make sure there are only 2 characters for each position (row and col)
    if characters.clone().count() != 2 {
        env::panic_str(&format!("Invalid Position {} - must have be of length 2", position));
    }

    //get the row and columns
    let col = characters.clone().nth(0).unwrap();
    let row: u32 = characters.clone().nth(1).unwrap().to_digit(10).expect("unable to convert row to int");

    //make sure the column is a letter
    if !col.is_alphabetic() {
        env::panic_str("Non Letter Col");
    }
    
    //make sure the row is in the range
    if row > 4 || row < 1 {
        env::panic_str("Row out of range");
    }

    let mut final_coords = 4 * (row - 1);

    match col {
        //single quotes for characters
        'A' => final_coords += 1,
        'B' => final_coords += 2,
        'C' => final_coords += 3,
        'D' => final_coords += 4,
        _ => env::panic_str("Invalid Column")
    }

    //return the final coordinates calculated
    final_coords
}

impl Fleet {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_populate_board_state(
        &mut self,
        ship_positions: Vec<String>
    ) {
        //make sure only 2 positions are being used
        assert_eq!(ship_positions.len(), 2, "Cannot place more than 1 ship");
        
        //get the numeric value of the ship positions
        let first_coord = convert_position_to_coords(&ship_positions[0]);
        let second_coord = convert_position_to_coords(&ship_positions[1]);

        env::log_str(&format!("First Post {} - Second Pos {}", first_coord, second_coord));
        //check for invalid positions ie diagonal
        if (second_coord as i32 - first_coord as i32).abs() != 4 &&  (second_coord as i32 - first_coord as i32).abs() != 1 {
            env::panic_str("Invalid ship positions - must place adjacent positions.")
        }

        //set the positions to ship
        self.board_state[(first_coord - 1) as usize] = "ship".to_string();
        self.board_state[(second_coord - 1) as usize] = "ship".to_string();
    }
}