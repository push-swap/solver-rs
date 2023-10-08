use std::io::{self, Stdout, Write};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum SerialOperation {
    PA,
    PB,
}

struct Serial {
    pa: isize,
}

impl Serial {
    fn new() -> Self {
        Serial { pa: 0 }
    }

    fn add(&mut self, operation: SerialOperation) -> bool {
        match operation {
            SerialOperation::PA => {
                self.pa += 1;
            }
            SerialOperation::PB => {
                self.pa -= 1;
            }
        }
        self.pa == 0
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ParallelOperation {
    S,
    R,
    RR,
}

struct Parallel {
    op: Vec<ParallelOperation>,
}

impl Parallel {
    fn new() -> Self {
        Parallel { op: vec![] }
    }

    fn add(&mut self, operation: ParallelOperation) -> bool {
        if let Some(last_operation) = self.op.last() {
            match operation {
                ParallelOperation::S => {
                    if *last_operation == ParallelOperation::S {
                        self.op.pop();
                    } else {
                        self.op.push(operation);
                    }
                }
                ParallelOperation::R => {
                    if *last_operation == ParallelOperation::RR {
                        self.op.pop();
                    } else {
                        self.op.push(operation);
                    }
                }
                ParallelOperation::RR => {
                    if *last_operation == ParallelOperation::R {
                        self.op.pop();
                    } else {
                        self.op.push(operation);
                    }
                }
            }
        } else {
            self.op.push(operation);
        }
        self.op.len() == 0
    }
}

enum StreamNode {
    Serial(Serial),
    Parallel(Parallel, Parallel),
}

impl StreamNode {
    fn print(&self, stdout: &mut Stdout) -> io::Result<()> {
        match self {
            StreamNode::Serial(serial) => {
                if serial.pa > 0 {
                    for _ in 0..serial.pa {
                        writeln!(stdout, "pa")?;
                    }
                } else {
                    for _ in 0..-serial.pa {
                        writeln!(stdout, "pb")?;
                    }
                }
                Ok(())
            }
            StreamNode::Parallel(a, b) => {
                if a.op.len() == 0 {
                    for op in &b.op {
                        writeln!(
                            stdout,
                            "{}",
                            match op {
                                ParallelOperation::S => "sb",
                                ParallelOperation::R => "rb",
                                ParallelOperation::RR => "rrb",
                            }
                        )?;
                    }
                    return Ok(());
                }
                if b.op.len() == 0 {
                    for op in &a.op {
                        writeln!(
                            stdout,
                            "{}",
                            match op {
                                ParallelOperation::S => "sa",
                                ParallelOperation::R => "ra",
                                ParallelOperation::RR => "rra",
                            }
                        )?;
                    }
                    return Ok(());
                }
                let mut matrix = vec![vec![0; a.op.len() + 1]; b.op.len() + 1];
                for i in 1..=b.op.len() {
                    for j in 1..=a.op.len() {
                        if b.op[i - 1] == a.op[j - 1] {
                            matrix[i][j] = matrix[i - 1][j - 1] + 1;
                        } else {
                            matrix[i][j] = matrix[i - 1][j].max(matrix[i][j - 1]);
                        }
                    }
                }
                let mut i = b.op.len();
                let mut j = a.op.len();
                let mut lcs = Vec::new();
                while i > 0 && j > 0 {
                    if b.op[i - 1] == a.op[j - 1] {
                        lcs.push((b.op[i - 1], true, true));
                        i -= 1;
                        j -= 1;
                    } else if matrix[i - 1][j] > matrix[i][j - 1] {
                        lcs.push((b.op[i - 1], false, true));
                        i -= 1;
                    } else {
                        lcs.push((a.op[j - 1], true, false));
                        j -= 1;
                    }
                }

                lcs.reverse();

                for (op, in_a, in_b) in lcs {
                    writeln!(
                        stdout,
                        "{}",
                        match (op, in_a, in_b) {
                            (ParallelOperation::S, true, true) => "ss",
                            (ParallelOperation::R, true, true) => "rr",
                            (ParallelOperation::RR, true, true) => "rrr",
                            (ParallelOperation::S, true, false) => "sa",
                            (ParallelOperation::R, true, false) => "ra",
                            (ParallelOperation::RR, true, false) => "rra",
                            (ParallelOperation::S, false, true) => "sb",
                            (ParallelOperation::R, false, true) => "rb",
                            (ParallelOperation::RR, false, true) => "rrb",
                            _ => panic!("Invalid"),
                        }
                    )?;
                }
                Ok(())
            }
        }
    }
}

pub struct Stream {
    nodes: Vec<StreamNode>,
    // TODO: use count
    // a: usize,
    // b: usize,
}

#[derive(Debug)]
pub enum Operation {
    PA = 0,
    PB = 1,
    SA = 2,
    SB = 3,
    SS = 4,
    RA = 5,
    RB = 6,
    RR = 7,
    RRA = 8,
    RRB = 9,
    RRR = 10,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "pa" => Ok(Operation::PA),
            "pb" => Ok(Operation::PB),
            "sa" => Ok(Operation::SA),
            "sb" => Ok(Operation::SB),
            "ss" => Ok(Operation::SS),
            "ra" => Ok(Operation::RA),
            "rb" => Ok(Operation::RB),
            "rr" => Ok(Operation::RR),
            "rra" => Ok(Operation::RRA),
            "rrb" => Ok(Operation::RRB),
            "rrr" => Ok(Operation::RRR),
            _ => Err(()), // Invalid input
        }
    }
}

enum OperationMetadata {
    Serial(SerialOperation),
    Parallel(ParallelOperation, bool, bool),
}

const OPERATION_METADATA: [OperationMetadata; 11] = [
    OperationMetadata::Serial(SerialOperation::PA),
    OperationMetadata::Serial(SerialOperation::PB),
    OperationMetadata::Parallel(ParallelOperation::S, true, false),
    OperationMetadata::Parallel(ParallelOperation::S, false, true),
    OperationMetadata::Parallel(ParallelOperation::S, true, true),
    OperationMetadata::Parallel(ParallelOperation::R, true, false),
    OperationMetadata::Parallel(ParallelOperation::R, false, true),
    OperationMetadata::Parallel(ParallelOperation::R, true, true),
    OperationMetadata::Parallel(ParallelOperation::RR, true, false),
    OperationMetadata::Parallel(ParallelOperation::RR, false, true),
    OperationMetadata::Parallel(ParallelOperation::RR, true, true),
];

impl Stream {
    pub fn new() -> Self {
        Stream { nodes: Vec::new() }
    }

    pub fn add(&mut self, operation: Operation) {
        if let Some(last_node) = self.nodes.last_mut() {
            match &OPERATION_METADATA[operation as usize] {
                OperationMetadata::Serial(op) => {
                    if let StreamNode::Serial(node) = last_node {
                        if node.add(*op) {
                            self.nodes.pop();
                        }
                    } else {
                        let mut serial = Serial::new();
                        serial.add(*op);
                        self.nodes.push(StreamNode::Serial(serial));
                    }
                }
                OperationMetadata::Parallel(op, for_a, for_b) => {
                    if let StreamNode::Parallel(a, b) = last_node {
                        let mut remove = false;
                        if *for_a {
                            remove &= a.add(*op);
                        }
                        if *for_b {
                            remove &= b.add(*op);
                        }
                        if remove {
                            self.nodes.pop();
                        }
                    } else {
                        let mut a = Parallel { op: vec![] };
                        let mut b = Parallel { op: vec![] };
                        if *for_a {
                            a.add(*op);
                        }
                        if *for_b {
                            b.add(*op);
                        }
                        self.nodes.push(StreamNode::Parallel(a, b));
                    }
                }
            }
        } else {
            match &OPERATION_METADATA[operation as usize] {
                OperationMetadata::Serial(op) => {
                    let mut serial = Serial::new();
                    serial.add(*op);
                    self.nodes.push(StreamNode::Serial(serial));
                }
                OperationMetadata::Parallel(op, for_a, for_b) => {
                    let mut a = Parallel::new();
                    let mut b = Parallel::new();
                    if *for_a {
                        a.add(*op);
                    }
                    if *for_b {
                        b.add(*op);
                    }
                    self.nodes.push(StreamNode::Parallel(a, b));
                }
            }
        }
    }

    pub fn print(&self, stdout: &mut Stdout) -> io::Result<()> {
        for node in self.nodes.iter() {
            match node.print(stdout) {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }
        return Ok(());
    }
}
