use crate::{input_file::read_all_to_string};

pub fn part1(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let binary = to_binary(&input);
    let (packets, _) = parse_packets(&binary, Some(1));
    let version_sum = sum_version_nums(&packets[0]);
    println!("{}", version_sum);
}

pub fn part2(input_file_path: &str) {
}

fn to_binary(hex: &str) -> Vec<bool> {
    hex.chars().map(|c| {
        match c {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("Invalid hex character")
        }
    }).flatten().collect()
}

fn bin_to_dec(binary: &[bool]) -> u64 {
    let mut exp = 2u64.pow(binary.len() as u32 - 1);
    let mut total = 0;
    for bit in binary {
        if *bit {
            total += exp;
        }
        exp /= 2;
    }
    total
}

fn parse_packets(mut binary: &[bool], amount: Option<usize>) -> (Vec<Packet>, usize) {
    let mut packets = Vec::<Packet>::new();
    let mut end_index = 0;
    while binary.len() > 4 { // if all bits left are 0 quit
        match amount {
            Some(max) => if packets.len() == max { break; },
            None => ()
        }
        let version = bin_to_dec(&binary[0..3]);
        let packet_type_id = bin_to_dec(&binary[3..6]);
        let (payload, payload_size) = match packet_type_id {
            4 => parse_literal_payload(&binary[6..]),
            _ => parse_operator_payload(&binary[6..])
        };
        end_index += 6+payload_size;
        binary = &binary[6+payload_size..];
        packets.push(Packet{version, payload});
    }
    (packets, end_index)
}

fn parse_literal_payload(binary: &[bool]) -> (PacketPayload, usize) {
    let mut i = 0;
    let mut stop = false;
    let mut literal_bin = Vec::<bool>::new();
    while !stop {
        stop |= !binary[i];
        literal_bin.extend(&binary[i+1..i+5]);
        i += 5;
    }
    let literal = bin_to_dec(&literal_bin);
    (PacketPayload::Literal(literal), i)
}

fn parse_operator_payload(binary: &[bool]) -> (PacketPayload, usize) {
    let length_type_flag = binary[0];
    if !length_type_flag {
        // next 15 bits represent total length in bits of sub-packets
        let bit_len = bin_to_dec(&binary[1..16]) as usize;
        let (packets, _) = parse_packets(&binary[16..16+bit_len], None);
        (PacketPayload::Operator(packets), 16+bit_len)
    } else {
        // next 11 bits represent total amount of sub-packets
        let packet_len = bin_to_dec(&binary[1..12]) as usize;
        let (packets, payload_size) = parse_packets(&binary[12..], Some(packet_len));
        (PacketPayload::Operator(packets), 12+payload_size)
    }
}

fn sum_version_nums(packet: &Packet) -> u64 {
    match &packet.payload {
        PacketPayload::Literal(_) =>
            packet.version,
        PacketPayload::Operator(sub_packets) =>
            packet.version + 
                sub_packets.iter()
                .map(|p| sum_version_nums(p))
                .sum::<u64>()
    }
}

struct Packet {
    version: u64,
    payload: PacketPayload
}

enum PacketPayload {
    Literal(u64),
    Operator(Vec<Packet>)
}