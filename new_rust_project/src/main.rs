
fn main() {
	let mut board: [[u8; 3]; 3] = [[0; 3]; 3];
	let mut player: u8 = 1;
	let mut result: (bool, usize, usize);
	loop {
		print_board(&board);
		
		result = get_input(&mut board, player);
		while !result.0 {
			result = get_input(&mut board, player);
		}
		if is_solved(&board, result.1, result.2) {
			break
		} else if player == 1 {
			player = 2;
		} else {
			player = 1;
		}
	}
	print_board(&board);
	println!("\n\nGood job player {}!", player);
}

fn line_v(board: &[[u8; 3]], col: usize, player: u8) -> bool {
	board[0][col] == player &&
	board[1][col] == player &&
	board[2][col] == player
}

fn line_h(board: &[[u8; 3]], row: usize, player: u8) -> bool {
	board[row][0] == player &&
	board[row][1] == player &&
	board[row][2] == player
}

fn line_d(board: &[[u8; 3]], player: u8) -> bool {
	(board[0][0] == player &&
	board[1][1] == player &&
	board[2][2] == player) ||
	(board[0][2] == player &&
	board[1][1] == player &&
	board[2][0] == player)
}

fn is_solved(board: &[[u8; 3]], row: usize, col: usize) -> bool{
	let player = board[row][col];
	line_v(board, col, player) ||
	line_h(board, row, player) ||
	line_d(board, player)
}

fn get_input(board: &mut[[u8; 3]], player: u8) -> (bool, usize, usize) {
	fprint("Enter row: ");
	let row: String = readline().unwrap().trim().to_string();
	fprint("Enter column: ");
	let col: String = readline().unwrap();
	let col = to_index(col.chars().next().unwrap());
	let row = row.parse::<usize>().unwrap();
	if !check_indices(row, col) {
		println!("Incorrect input");
		(false, row, col)
	} else if board[row][col] != 0 {
		println!("Space already chosen");
		(false, row, col)
	} else {
		board[row][col] = player;
		(true, row, col)
	}
}

fn check_indices(x: usize, y: usize) -> bool {
	x < 3 && y < 3
}

fn to_index(c: char) -> usize {
	if c == 'a' || c == 'A' {
		0
	} else if c == 'b' || c == 'B' {
		1
	} else if c == 'c' || c == 'C' {
		2
	} else {
		3
	}
}

fn readline() -> std::io::Result<String> {
	let mut buffer: String = String::new();
	std::io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn fprint(message: &str) {
	print!("{}", message);
	std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
}

fn clear_terminal() {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn print_board(board: &[[u8; 3]]) {
	clear_terminal();
	println!("   A B C\n  -------");
	for i in 0..3 {
		print!("{:?} |", i);
		for j in board[i].iter().take(3) {
			if *j == 0 {
				print!(" ");
			} else if *j == 1 {
				print!("X");
			} else {
				print!("O");
			}
			print!("|");
		}
		if i != 2 {
			println!();
		}
	}
	println!("\n  -------");
}