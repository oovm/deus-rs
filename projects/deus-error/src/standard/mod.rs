use sudoku::Sudoku as SudokuBoard;

#[derive(Debug)]
pub struct Sudoku {

}

#[test]
fn test() {


// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
    let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

    let sudoku = sudoku::from_str_line(sudoku_line).unwrap();

// Solve, print or convert the sudoku to another format
    if let Some(solution) = sudoku.solve_unique() {
        // print the solution in line format
        println!("{}", solution);

        // or return it as a byte array
        let cell_contents: [u8; 81] = solution.to_bytes();
    }
}

