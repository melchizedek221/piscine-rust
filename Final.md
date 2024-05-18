**LEVEL 1:**
division-and-remainder ✅
find-factorial  ✅
matrix-transposition ✅



**LEVEL 2:**
reverse-it ✅
<!-- pub fn reverse_it(v: i32) -> String {
    let mut reversed = v.abs().to_string().chars().rev().collect::<String>();
    if v < 0 {
        reversed.insert(0, '-');
    }
    reversed.push_str(&v.abs().to_string());
    
    reversed
} -->
counting-words ✅
bigger ✅      *h.values().max().unwrap()
capitalizing ✅
<!-- pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
} -->



**LEVEL 3:** 
partal sums ✅
inv-pyramid ✅
<!-- fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut lines = Vec::new();

    for j in 0..i {
        lines.push(format!("{:space$}{}", "", v.repeat((j + 1) as usize), space = j as usize));
    }
    for j in (0..i).rev() {
        if j != i-1 {
            lines.push(format!("{:space$}{}", "", v.repeat(j as usize + 1), space = j as usize));
        }
    }
    lines
} -->
next-prime ✅
<!-- pub fn next_prime(n: u64) -> u64 {
    let mut num = n+1;
    while !is_prime(num) {
        num += 1;
    }
    num
}

fn is_prime(n: u64) -> bool {
    !(2..n/2+1).any(|i| n % i == 0)
} -->
previous-prime ✅
<!-- pub fn prev_prime(n: u64) -> u64 {
    for num in (2..n+1).rev() {
        if is_prime(num) {
            return num;
        }
    }
    0
}

fn is_prime(n: u64) -> bool {
    !(2..n/2+1).any(|i| n % i == 0)
} -->
profanity-filter ✅
cipher ✅
<!-- pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }

    let expected = original
        .chars()
        .map(|c| match c {
            'a'..='z' => ('z' as u8 - (c as u8 - 'a' as u8)) as char,
            'A'..='Z' => ('Z' as u8 - (c as u8 - 'A' as u8)) as char,
            _ => c,
        })
        .collect::<String>();

    if expected == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, expected)))
    }
} -->
scytal-cipher ✅


**Level 4:**
insertion-sort ✅
<!-- pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    for i in 1..=steps {
        let mut j = i;
        while j > 0 && slice[j-1] > slice[j] {
            slice.swap(j, j-1);
            j -= 1;
        }
    }
} -->

<!-- (slice.split_atmut(steps+1).0).sort(); -->

rpn ✅
<!-- fn rpn(s: &str) {
    let mut stack = Vec::new();

    for token in s.split_whitespace() {
        match token.parse::<i64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                let right = stack.pop();
                let left = stack.pop();

                match (left, right, token) {
                    (Some(left), Some(right), "+") => stack.push(left + right),
                    (Some(left), Some(right), "-") => stack.push(left - right),
                    (Some(left), Some(right), "*") => stack.push(left * right),
                    (Some(left), Some(right), "/") => stack.push(left / right),
                    (Some(left), Some(right), "%") => stack.push(left % right),
                    _ => return println!("Error"),
                }
            }
        }
    }

    if let Some(&result) = stack.last() {
        if stack.len() == 1 {
            println!("{}", result);
        } else {
            println!("Error");
        }
    }
} -->

rot 21
<!-- pub fn rot21(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 21) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
} -->

order-books
<!-- impl Writer {
    pub fn order_books(&mut self) {
        self.books.sort_by_key(|book| book.title.clone());
    }
} -->

matrix-determinant
<!-- pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];
    let d = matrix[1][0];
    let e = matrix[1][1];
    let f = matrix[1][2];
    let g = matrix[2][0];
    let h = matrix[2][1];
    let i = matrix[2][2];

    let det_a = e * i - f * h;
    let det_b = d * i - f * g;
    let det_c = d * h - e * g;

    a * det_a - b * det_b + c * det_c
} -->


**LEVEL 5:**
Roman numbers
<!-- #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!(),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self{
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }
        
        let mut result = Vec::new();
        let div = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        
        for (i, n) in div.iter().enumerate() {
            while n <= &num {
                if i % 2 == 0 {
                    result.push(RomanDigit::from(*n));
                }else{
                    let rem = div[i - 1] - div[i];
                    result.push(RomanDigit::from(rem));
                    result.push(RomanDigit::from(div[i - 1]));
                }
                num -= n;
            }
        }
        RomanNumber(result)
    }
} -->

Blood types s
<!-- impl BloodType {
    pub fn can_receive_from(&self, donor: &BloodType) -> bool {
        let is_antigen_compatible = 
            (self.antigen == Antigen::A && (donor.antigen == Antigen::A || donor.antigen == Antigen::O)) ||
            (self.antigen == Antigen::B && (donor.antigen == Antigen::B || donor.antigen == Antigen::O)) ||
            (self.antigen == Antigen::AB) ||
            (self.antigen == Antigen::O && donor.antigen == Antigen::O);
    
        let is_rh_compatible =
            (self.rh_factor == RhFactor::Positive) ||
            (self.rh_factor == RhFactor::Negative && donor.rh_factor == RhFactor::Negative);
    
        is_antigen_compatible && is_rh_compatible
    }

    pub fn donors(&self) -> Vec<BloodType> {
        let mut donors = Vec::new();
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood_type in all_blood_types {
            if self.can_receive_from(&blood_type) {
                donors.push(blood_type);
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood_type in all_blood_types {
            if blood_type.can_receive_from(self) {
                recipients.push(blood_type);
            }
        }

        recipients
    }
} -->


**LEVEL 6:**
Matrix display
<!-- pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Matrix(slice.iter().map(|row| row.to_vec()).collect())
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            write!(f, "(")?;
            for (i, val) in row.iter().enumerate() {
                write!(f, "{}{}", val, if i < row.len() - 1 { " " } else { "" })?;
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}
 -->

Queens
<!-- #[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dr = (self.position.rank - other.position.rank).abs();
        let df = (self.position.file - other.position.file).abs();
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || dr == df
    }
} -->

Lunch queue
<!-- #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next: Link,
}

impl Queue {

    pub fn new() -> Queue {
        Self { node:None }
    }
    
    pub fn add(&mut self, name: String, discount: i32) {
        self.node = Some(
            Person{
                name: name,
                discount: discount,
                next: self.node.take(),
            }
        ).map(Box::new);
    }
    
    pub fn invert_queue(&mut self) {
        let mut prev = None;
        let mut current = self.node.take();
        
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }
            self.node = prev;
    }
    
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = &mut self.node;
        let mut removed_node = None;

        while let Some(node) = current {
            if node.next.is_none() {
                // Last node in the list
                let temp_node = node.next.take(); // Remove the last node
                removed_node = temp_node.map(|n| (n.name, n.discount));
                self.node = None;
                break;
            } else if node.next.as_ref().unwrap().next.is_none() {
                let temp_node = node.next.take(); // Remove the last node
                removed_node = temp_node.map(|n| (n.name, n.discount));
                node.next = None; // Update the second-to-last node's next pointer
                break;
            } else {
                // Move to the next node
                current = &mut node.next;
            }
        }

        removed_node
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;
        while let Some(node) = current {
            if node.name == name {
                return Some((node.name.clone(), node.discount));
            }
            current = &node.next;
        }
        None
    }
} -->


**LEVEL 7:**
Drop the thread
<!-- use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers::default()
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let thread = Thread::new_thread(self.track_worker(), c, self);
        self.states.borrow_mut().push(false);
        (thread.pid, thread)
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{:?} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }
    pub fn skill(self) {
        drop(self);
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
} -->

Filter table
<!-- #[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        if row.len() == self.headers.len() {
            self.body.push(row.to_vec());
        } else {
            panic!("Row length does not match the number of headers");
        }
    }

    pub fn filter_col<F>(&self, filter: F) -> Option<Self>
    where
        F: Fn(&str) -> bool,
    {
        let selected_columns: Vec<usize> = self
            .headers
            .iter()
            .enumerate()
            .filter_map(|(i, header)| if filter(header) { Some(i) } else { None })
            .collect();

        if selected_columns.is_empty() {
            return None;
        }

        let filtered_headers: Vec<String> = selected_columns
            .iter()
            .map(|&i| self.headers[i].clone())
            .collect();
        
        let filtered_body: Vec<Vec<String>> = self.body
            .iter()
            .map(|row| selected_columns.iter().map(|&i| row[i].clone()).collect())
            .collect();
        
        Some(Table {
            headers: filtered_headers,
            body: filtered_body,
        })
    }

    pub fn filter_row<F>(&self, col_name: &str, filter: F) -> Option<Self>
    where
        F: Fn(&str) -> bool,
    {
        let col_index = self.headers.iter().position(|header| header == col_name)?;
        
        let filtered_body: Vec<Vec<String>> = self.body
            .iter()
            .filter(|row| filter(&row[col_index]))
            .cloned()
            .collect();
        
        if filtered_body.is_empty() {
            return None;
        }

        Some(Table {
            headers: self.headers.clone(),
            body: filtered_body,
        })
    }
} -->

Display table
<!-- use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        if row.len() == self.headers.len() {
            self.body.push(row.to_vec());
        } else {
            panic!("Row length does not match the number of headers");
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(());
        }

        // Determine the width of each column
        let mut col_widths: Vec<usize> = self.headers.iter().map(|header| header.len()).collect();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }

        // Function to format a single cell centered within its column width
        fn format_cell(cell: &str, width: usize) -> String {
            let padding = width - cell.len();
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("{:left$}{:^width$}{:right$}", "", cell, "", left = left_pad, right = right_pad, width = width)
        }

        // Print the headers
        for (i, header) in self.headers.iter().enumerate() {
            if i > 0 {
                write!(f, "|")?;
            }
            write!(f, " {} ", format_cell(header, col_widths[i]))?;
        }
        writeln!(f)?;

        // Print the separator
        for (i, &width) in col_widths.iter().enumerate() {
            if i > 0 {
                write!(f, "+")?;
            }
            write!(f, "{:-<width$}", "-", width = width + 2)?;
        }
        writeln!(f)?;

        // Print the rows
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, "|")?;
                }
                write!(f, " {} ", format_cell(cell, col_widths[i]))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
} -->



**LEVEL 8:**
flat-tree
<!-- pub fn flatten_tree<T: ToOwned<Owned = T> + Clone>(tree: &BTreeSet<T>) -> Vec<T> {
    tree.iter().cloned().collect()
} -->


**LEVEL 9:**
brackets-matching
<!-- fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for argument in args {
        if check_brackets(&argument) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}

fn check_brackets(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => (),
        }
    }

    stack.is_empty()
} -->
brain-fuck
