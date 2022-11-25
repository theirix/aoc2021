use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(31, 54);

/* Impl */

const PACKET_SUM: u32 = 0;
const PACKET_PRODUCT: u32 = 1;
const PACKET_MIN: u32 = 2;
const PACKET_MAX: u32 = 3;
const PACKET_VALUE: u32 = 4;
const PACKET_GREATER: u32 = 5;
const PACKET_LESS: u32 = 6;
const PACKET_EQUAL: u32 = 7;

#[derive(Debug)]
enum Node {
    Value(u64),
    Sum(Vec<Box<Node>>),
    Product(Vec<Box<Node>>),
    Min(Vec<Box<Node>>),
    Max(Vec<Box<Node>>),
    Greater(Box<Node>, Box<Node>),
    Less(Box<Node>, Box<Node>),
    Equal(Box<Node>, Box<Node>),
}

fn pretty(bits: &[bool]) -> String {
    bits.iter().map(|x| if *x { '1' } else { '0' }).collect()
}

fn read<'a>(bits: &'a [bool], ind: &mut usize, count: usize) -> Option<&'a [bool]> {
    if *ind == bits.len() {
        println!("End of stream");
        return None;
    }
    if *ind + count > bits.len() {
        panic!(
            "Cannot read {} bits at {} having only {}",
            count,
            *ind,
            bits.len()
        );
    }
    let res = &bits[*ind..*ind + count];
    println!(" read {} bits at {}: {}", count, *ind, pretty(res));
    *ind += count;
    Some(res)
}

fn from_bits(bits: &[bool]) -> u64 {
    bits.iter().fold(0, |result, &bit| {
        (result << 1) ^ (if bit { 1u64 } else { 0u64 })
    })
}

fn parse_value_packet(bits: &[bool], ind: &mut usize) -> u64 {
    let mut total_data: Vec<bool> = vec![];
    while let Some(part) = read(bits, ind, 5) {
        let one_data = &part[1..5];
        assert!(one_data.len() == 4);
        total_data.extend_from_slice(one_data);
        if !part[0] {
            break;
        }
    }
    let ret = from_bits(&total_data) as u64;
    println!(
        " parsing value {} of {} parts to {}",
        pretty(&total_data),
        total_data.len(),
        ret
    );
    ret
}

fn parse_operator_a(bits: &[bool], ind: &mut usize) -> u64 {
    let mut total_version = 0;

    let length_type = from_bits(read(bits, ind, 1).unwrap());
    println!("Operator packet with length_type {}", length_type);
    match length_type {
        0 => {
            let total_bits = from_bits(read(bits, ind, 15).unwrap()) as usize;
            println!("Next {} total bits", total_bits);
            let mut local_ind = *ind;
            while local_ind < *ind + total_bits {
                println!(
                    "Start reading subpacket at local ind {}, total ind {}",
                    local_ind, ind
                );
                total_version += parse_packet_a(bits, &mut local_ind);
                println!("Completed local ind is {}, total ind {}", local_ind, ind);
            }
            *ind = local_ind;
        }
        1 => {
            let total_packets = from_bits(read(bits, ind, 11).unwrap());
            println!("Next {} total packets", total_packets);
            for packet_ind in 0..total_packets {
                println!("Start reading subpacket {}", packet_ind);
                total_version += parse_packet_a(bits, ind);
            }
        }
        _ => panic!("Wrong length type"),
    }
    total_version
}

fn parse_packet_a(bits: &[bool], ind: &mut usize) -> u64 {
    println!("Start reading packet at index {}", ind);
    let packet_version = match read(bits, ind, 3) {
        Some(data) => from_bits(data),
        None => panic!("No packet version"), //return None,
    };
    println!("Packet version {}", packet_version);
    let mut total_version = packet_version as u64;

    let opt_data = read(bits, ind, 3);
    if opt_data.is_none() {
        panic!("No packet type");
        //return None;
    }
    let data = opt_data.unwrap();
    let packet_type = from_bits(data) as u32;
    println!(">> Packet type {:?}", packet_type);
    match packet_type {
        PACKET_VALUE => {
            let value = parse_value_packet(bits, ind);
            println!("Value packet value {}", value);
            total_version
        }
        _ => {
            let sub_ver = parse_operator_a(bits, ind);
            total_version += sub_ver;
            total_version
        }
    }
}

fn parse_operator_b(bits: &[bool], ind: &mut usize) -> Vec<Box<Node>> {
    let mut sub_packets: Vec<Box<Node>> = Vec::new();

    let length_type = from_bits(read(bits, ind, 1).unwrap());
    println!("Operator packet with length_type {}", length_type);
    match length_type {
        0 => {
            let total_bits = from_bits(read(bits, ind, 15).unwrap()) as usize;
            println!("Next {} total bits", total_bits);
            let mut local_ind = *ind;
            while local_ind < *ind + total_bits {
                println!(
                    "Start reading subpacket at local ind {}, total ind {}",
                    local_ind, ind
                );
                sub_packets.push(parse_packet_b(bits, &mut local_ind));
                println!("Completed local ind is {}, total ind {}", local_ind, ind);
            }
            *ind = local_ind;
        }
        1 => {
            let total_packets = from_bits(read(bits, ind, 11).unwrap());
            println!("Next {} total packets", total_packets);
            for packet_ind in 0..total_packets {
                println!("Start reading subpacket {}", packet_ind);
                sub_packets.push(parse_packet_b(bits, ind));
            }
        }
        _ => panic!("Wrong length type"),
    }
    sub_packets
}

fn parse_packet_b(bits: &[bool], ind: &mut usize) -> Box<Node> {
    println!("Start reading packet at index {}", ind);
    let packet_version = match read(bits, ind, 3) {
        Some(data) => from_bits(data),
        None => panic!("No packet version"), //return None,
    };
    println!("Packet version {}", packet_version);

    let opt_data = read(bits, ind, 3);
    if opt_data.is_none() {
        panic!("No packet type");
        //return None;
    }
    let data = opt_data.unwrap();
    let packet_type = from_bits(data) as u32;
    println!(">> Packet type {:?}", packet_type);
    match packet_type {
        PACKET_VALUE => Box::new(Node::Value(parse_value_packet(bits, ind))),
        PACKET_SUM => Box::new(Node::Sum(parse_operator_b(bits, ind))),
        PACKET_PRODUCT => Box::new(Node::Product(parse_operator_b(bits, ind))),
        PACKET_MIN => Box::new(Node::Min(parse_operator_b(bits, ind))),
        PACKET_MAX => Box::new(Node::Max(parse_operator_b(bits, ind))),
        PACKET_LESS => {
            let mut sub = parse_operator_b(bits, ind);
            let b = sub.pop().unwrap();
            let a = sub.pop().unwrap();
            Box::new(Node::Less(a, b))
        }
        PACKET_GREATER => {
            let mut sub = parse_operator_b(bits, ind);
            let b = sub.pop().unwrap();
            let a = sub.pop().unwrap();
            Box::new(Node::Greater(a, b))
        }
        PACKET_EQUAL => {
            let mut sub = parse_operator_b(bits, ind);
            let b = sub.pop().unwrap();
            let a = sub.pop().unwrap();
            Box::new(Node::Equal(a, b))
        }
        _ => {
            panic!("Wrong packet type")
        }
    }
}

fn char_to_bits(c: char) -> Vec<bool> {
    let v: u8 = c.to_digit(16).unwrap() as u8;
    println!("{}", v);
    vec![(v & 8) == 8, (v & 4) == 4, (v & 2) == 2, (v & 1) == 1]
}

fn parse_bits(line: &str) -> Vec<bool> {
    if line.chars().all(|c| c == '1' || c == '0') {
        line.chars().map(|c| c == '1').collect()
    } else {
        line.chars().flat_map(char_to_bits).collect()
    }
}

fn post_check(bits: &[bool], ind: usize) {
    let remaining = &bits[ind..];
    if remaining.is_empty() || remaining.iter().all(|c| !c) {
        println!("Remaining data of {} zeroes", remaining.len());
    } else {
        panic!(
            "Remaining data of {} are NOT zeroes: {}",
            remaining.len(),
            pretty(remaining)
        );
    }
}

fn parse_a(line: &str) -> u64 {
    let bits: Vec<bool> = parse_bits(line);
    println!("Input {} bits: {}", bits.len(), pretty(&bits));
    let mut ind = 0;
    let ver = parse_packet_a(&bits, &mut ind);
    post_check(&bits, ind);
    println!("Version {}", ver);
    ver
}

fn fold_tree(node: &Box<Node>) -> u64 {
    // if node is a reference to rc
    let rnode: &Node = &*(*node);
    let val = match rnode {
        Node::Value(value) => *value,
        Node::Sum(childs) => childs.iter().map(fold_tree).sum(),
        Node::Product(childs) => childs.iter().map(fold_tree).product(),
        Node::Min(childs) => childs.iter().map(fold_tree).min().unwrap(),
        Node::Max(childs) => childs.iter().map(fold_tree).max().unwrap(),
        Node::Greater(a, b) => {
            if fold_tree(a) > fold_tree(b) {
                1
            } else {
                0
            }
        }
        Node::Less(a, b) => {
            if fold_tree(a) < fold_tree(b) {
                1
            } else {
                0
            }
        }
        Node::Equal(a, b) => {
            if fold_tree(a) == fold_tree(b) {
                1
            } else {
                0
            }
        }
    };
    println!("Folded at {:?} to {}", rnode, val);
    val
}

fn parse_b(line: &str) -> u64 {
    let bits: Vec<bool> = parse_bits(line);
    println!("Input {} bits: {}", bits.len(), pretty(&bits));
    let mut ind = 0;
    let node = parse_packet_b(&bits, &mut ind);
    post_check(&bits, ind);
    fold_tree(&node)
}

pub fn process_a(lines: Vec<String>) -> u64 {
    parse_a(&lines[0])
}

pub fn process_b(lines: Vec<String>) -> u64 {
    parse_b(&lines[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert!(from_bits(&vec![true]) == 1);
        assert!(from_bits(&vec![true, false]) == 2);
        assert!(from_bits(&vec![true, true]) == 3);

        println!("{:?}", char_to_bits('D'));
        assert!(pretty(&char_to_bits('2')) == "0010");
        assert!(pretty(&char_to_bits('D')) == "1101");

        assert!(parse_bits("101") == vec![true, false, true]);
        assert!(pretty(&parse_bits("D")) == "1101");
        assert!(pretty(&parse_bits("D2FE28")) == "110100101111111000101000");
    }

    #[test]
    fn test_a() {
        assert!(parse_a("8A004A801A8002F478") == 16);
        assert!(parse_a("620080001611562C8802118E34") == 12);
        assert!(parse_a("C0015000016115A2E0802F182340") == 23);
        assert!(parse_a("A0016C880162017C3686B18A3D4780") == 31);
    }

    #[test]
    fn test_b_sum() {
        assert!(parse_b("C200B40A82") == 3);
    }

    #[test]
    fn test_b_other() {
        assert!(parse_b("04005AC33890") == 54);
        assert!(parse_b("880086C3E88112") == 7);
        assert!(parse_b("CE00C43D881120") == 9);
        assert!(parse_b("D8005AC2A8F0") == 1);
        assert!(parse_b("F600BC2D8F") == 0);
        assert!(parse_b("9C005AC2F8F0") == 0);
        assert!(parse_b("9C0141080250320F1802104A08") == 1);
    }
}
