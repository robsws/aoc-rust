use crate::{
    input_file::read_all_to_string,
    binary::bin_to_dec
};

pub fn part1(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let binary = to_binary(&input);
    let (packets, _) = parse_packets(&binary, Some(1));
    let version_sum = sum_version_nums(&packets[0]);
    println!("{}", version_sum);
}

pub fn part2(input_file_path: &str) {
    let input = read_all_to_string(input_file_path);
    let binary = to_binary(&input);
    let (packets, _) = parse_packets(&binary, Some(1));
    let value = eval_packet(&packets[0]);
    println!("{}", value);
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

fn parse_packets(mut binary: &[bool], amount: Option<usize>) -> (Vec<Packet>, usize) {
    let mut packets = Vec::<Packet>::new();
    let mut end_index = 0;
    while binary.len() > 4 {
        match amount {
            Some(max) => if packets.len() == max { break; },
            None => ()
        }
        let version = bin_to_dec(&binary[0..3]);
        let packet_type_id = bin_to_dec(&binary[3..6]);
        let (payload, payload_size) = match packet_type_id {
            4 => parse_literal_payload(&binary[6..]),
            n => parse_operator_payload(&binary[6..], n)
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

fn parse_operator_payload(binary: &[bool], type_id: u64) -> (PacketPayload, usize) {
    let length_type_flag = binary[0];
    if !length_type_flag {
        // next 15 bits represent total length in bits of sub-packets
        let bit_len = bin_to_dec(&binary[1..16]) as usize;
        let (packets, _) = parse_packets(&binary[16..16+bit_len], None);
        (build_operator_payload(packets, type_id), 16+bit_len)
    } else {
        // next 11 bits represent total amount of sub-packets
        let packet_len = bin_to_dec(&binary[1..12]) as usize;
        let (packets, payload_size) = parse_packets(&binary[12..], Some(packet_len));
        (build_operator_payload(packets, type_id), 12+payload_size)
    }
}

fn build_operator_payload(packets: Vec<Packet>, type_id: u64) -> PacketPayload {
    match type_id {
        0 => PacketPayload::Sum(packets),
        1 => PacketPayload::Product(packets),
        2 => PacketPayload::Minimum(packets),
        3 => PacketPayload::Maximum(packets),
        5 => PacketPayload::GreaterThan(packets),
        6 => PacketPayload::LessThan(packets),
        7 => PacketPayload::EqualTo(packets),
        _ => {
            panic!("Invalid operator packet type.");
        }
    }
}

fn sum_version_nums(packet: &Packet) -> u64 {
    match &packet.payload {
        PacketPayload::Sum(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::Product(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::Minimum(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::Maximum(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::Literal(_) => packet.version,
        PacketPayload::GreaterThan(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::LessThan(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),
        PacketPayload::EqualTo(sub_packets) =>
            sum_sibling_versions(packet.version, sub_packets),   
    }
}

fn sum_sibling_versions(version: u64, sub_packets: &Vec<Packet>) -> u64 {
    version + 
        sub_packets.iter()
        .map(|p| sum_version_nums(p))
        .sum::<u64>()
}

fn eval_packet(packet: &Packet) -> u64 {
    match &packet.payload {
        PacketPayload::Sum(sub_packets) =>
            sub_packets.iter().map(|p| eval_packet(p)).sum(),
        PacketPayload::Product(sub_packets) =>
            sub_packets.iter().map(|p| eval_packet(p)).product(),
        PacketPayload::Minimum(sub_packets) =>
            sub_packets.iter().map(|p| eval_packet(p)).min().unwrap(),
        PacketPayload::Maximum(sub_packets) =>
            sub_packets.iter().map(|p| eval_packet(p)).max().unwrap(),
        PacketPayload::Literal(value) => *value,
        PacketPayload::GreaterThan(sub_packets) => {
            if sub_packets.len() != 2 {
                panic!("GreaterThan operator must have exactly two subpackets.");
            }
            if eval_packet(&sub_packets[0]) > eval_packet(&sub_packets[1]) { 1 } else { 0 }
        },
        PacketPayload::LessThan(sub_packets) => {
            if sub_packets.len() != 2 {
                panic!("GreaterThan operator must have exactly two subpackets.");
            }
            if eval_packet(&sub_packets[0]) < eval_packet(&sub_packets[1]) { 1 } else { 0 }
        },
        PacketPayload::EqualTo(sub_packets) => {
            if sub_packets.len() != 2 {
                panic!("GreaterThan operator must have exactly two subpackets.");
            }
            if eval_packet(&sub_packets[0]) == eval_packet(&sub_packets[1]) { 1 } else { 0 }
        }
    }
}

struct Packet {
    version: u64,
    payload: PacketPayload
}

enum PacketPayload {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}