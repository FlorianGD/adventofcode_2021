use itertools::Itertools;

fn parse_hex(hex: &str) -> String {
    hex.chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|i| format!("{:04b}", i))
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Literal,
    Operator(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LengthType {
    TotalBits(usize),
    NumPackets(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: Type,
    length: Option<usize>,
    packets: Option<Vec<Packet>>,
    value: Option<usize>,
}

fn parse_literal(values: &str) -> (usize, usize) {
    let mut idx = 0;
    let mut total: Vec<char> = Vec::new();
    loop {
        let slice = &values[idx..idx + 5];
        total.extend(slice.chars().skip(1));
        idx += 5;
        if slice.chars().next().unwrap() == '0' {
            break;
        }
    }
    let val: String = total.into_iter().collect();
    (usize::from_str_radix(&val, 2).unwrap(), idx)
}

fn parse_operator_length(bits: &str) -> (LengthType, usize) {
    match bits.chars().next().unwrap() {
        '0' => (
            LengthType::TotalBits(usize::from_str_radix(&bits[1..16], 2).unwrap()),
            16,
        ),
        '1' => (
            LengthType::NumPackets(usize::from_str_radix(&bits[1..12], 2).unwrap()),
            12,
        ),
        _ => unreachable!(),
    }
}

fn parse(bits: &str) -> Packet {
    let version = u8::from_str_radix(&bits[..3], 2).unwrap();
    let type_id = u8::from_str_radix(&bits[3..6], 2).unwrap();
    match type_id {
        4 => {
            let type_id = Type::Literal;
            let (value, end_idx) = parse_literal(&bits[6..]);
            Packet {
                version,
                type_id,
                length: Some(end_idx + 6),
                packets: None,
                value: Some(value),
            }
        }
        v => {
            let type_id = Type::Operator(v);
            let (length_type, next_start_idx) = parse_operator_length(&bits[6..]);
            let mut packets: Vec<Packet> = Vec::new();
            let mut start_idx = 6 + next_start_idx;
            match length_type {
                LengthType::NumPackets(n) => {
                    for _ in 0..n {
                        let p: Packet = parse(&bits[start_idx..]);
                        start_idx += p.length.unwrap();
                        packets.push(p);
                    }
                }
                LengthType::TotalBits(n) => {
                    while start_idx - 6 - next_start_idx < n {
                        let p: Packet = parse(&bits[start_idx..]);
                        start_idx += p.length.unwrap();
                        packets.push(p);
                    }
                }
            }
            Packet {
                version,
                type_id,
                length: Some(start_idx),
                packets: Some(packets),
                value: None,
            }
        }
    }
}

fn sum_versions(p: &Packet) -> usize {
    match &p.packets {
        None => p.version as usize,
        Some(ps) => {
            let s: usize = ps.iter().map(sum_versions).sum();
            p.version as usize + s
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|hex| parse(&parse_hex(hex)))
        .map(|p| sum_versions(&p))
        .sum()
}

fn compute_value(p: &Packet) -> usize {
    match p.type_id {
        Type::Literal => p.value.unwrap(),
        Type::Operator(0) => p.packets.as_ref().unwrap().iter().map(compute_value).sum(),
        Type::Operator(1) => p
            .packets
            .as_ref()
            .unwrap()
            .iter()
            .map(compute_value)
            .product(),
        Type::Operator(2) => p
            .packets
            .as_ref()
            .unwrap()
            .iter()
            .map(compute_value)
            .min()
            .unwrap(),
        Type::Operator(3) => p
            .packets
            .as_ref()
            .unwrap()
            .iter()
            .map(compute_value)
            .max()
            .unwrap(),
        Type::Operator(5) => {
            let (p1, p2) = p.packets.as_ref().unwrap().iter().next_tuple().unwrap();
            if compute_value(p1) > compute_value(p2) {
                1
            } else {
                0
            }
        }
        Type::Operator(6) => {
            let (p1, p2) = p.packets.as_ref().unwrap().iter().next_tuple().unwrap();
            if compute_value(p1) < compute_value(p2) {
                1
            } else {
                0
            }
        }
        Type::Operator(7) => {
            let (p1, p2) = p.packets.as_ref().unwrap().iter().next_tuple().unwrap();
            if compute_value(p1) == compute_value(p2) {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|hex| parse(&parse_hex(hex)))
        .map(|p| compute_value(&p))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_hex("D2FE28"), "110100101111111000101000");
        assert_eq!(
            parse_hex("38006F45291200"),
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            parse_hex("EE00D40C823060"),
            "11101110000000001101010000001100100000100011000001100000"
        );
    }
    #[test]
    fn test_parse_literal() {
        assert_eq!(parse_literal("101111111000101000"), (2021, 15));
    }

    #[test]
    fn test_parse_operator_length() {
        assert_eq!(
            parse_operator_length("00000000000110111101000101001010010001001000000000"),
            (LengthType::TotalBits(27), 16)
        );
        assert_eq!(
            parse_operator_length("10000000001101010000001100100000100011000001100000"),
            (LengthType::NumPackets(3), 12)
        );
    }

    #[test]
    fn test_parse_packet_literal() {
        let p: Packet = parse(&parse_hex("D2FE28"));
        assert_eq!(
            p,
            Packet {
                version: 6,
                type_id: Type::Literal,
                length: Some(21),
                packets: None,
                value: Some(2021)
            }
        );
        let p = parse("11010001010");
        assert_eq!(
            p,
            Packet {
                version: 6,
                type_id: Type::Literal,
                length: Some(11),
                packets: None,
                value: Some(10)
            }
        )
    }

    #[test]
    fn test_parse_packet_operator() {
        let p: Packet = parse(&parse_hex("38006F45291200"));
        assert_eq!(
            p,
            Packet {
                version: 1,
                type_id: Type::Operator(6),
                length: Some(49),
                packets: Some(vec![
                    Packet {
                        version: 6,
                        type_id: Type::Literal,
                        length: Some(11),
                        packets: None,
                        value: Some(10)
                    },
                    Packet {
                        version: 2,
                        type_id: Type::Literal,
                        length: Some(16),
                        packets: None,
                        value: Some(20)
                    }
                ]),
                value: None,
            }
        );
        let p: Packet = parse(&parse_hex("EE00D40C823060"));
        assert_eq!(
            p,
            Packet {
                version: 7,
                type_id: Type::Operator(3),
                length: Some(51),
                packets: Some(vec![
                    Packet {
                        version: 2,
                        type_id: Type::Literal,
                        length: Some(11),
                        packets: None,
                        value: Some(1)
                    },
                    Packet {
                        version: 4,
                        type_id: Type::Literal,
                        length: Some(11),
                        packets: None,
                        value: Some(2)
                    },
                    Packet {
                        version: 1,
                        type_id: Type::Literal,
                        length: Some(11),
                        packets: None,
                        value: Some(3)
                    },
                ]),
                value: None,
            }
        );
    }

    #[test]
    fn test_sum_versions() {
        let p = parse(&parse_hex("D2FE28"));
        assert_eq!(sum_versions(&p), 6);
        let p = parse(&parse_hex("38006F45291200"));
        assert_eq!(sum_versions(&p), 9);
        let p = parse(&parse_hex("EE00D40C823060"));
        assert_eq!(sum_versions(&p), 14);
        let p = parse(&parse_hex("8A004A801A8002F478"));
        assert_eq!(sum_versions(&p), 16);
        let p = parse(&parse_hex("620080001611562C8802118E34"));
        assert_eq!(sum_versions(&p), 12);
        let p = parse(&parse_hex("C0015000016115A2E0802F182340"));
        assert_eq!(sum_versions(&p), 23);
        let p = parse(&parse_hex("A0016C880162017C3686B18A3D4780"));
        assert_eq!(sum_versions(&p), 31);
    }

    #[test]
    fn test_compute_value() {
        let p = parse(&parse_hex("D2FE28"));
        assert_eq!(compute_value(&p), 2021);
        let p = parse(&parse_hex("C200B40A82"));
        assert_eq!(compute_value(&p), 3);
        let p = parse(&parse_hex("04005AC33890"));
        assert_eq!(compute_value(&p), 54);
        let p = parse(&parse_hex("880086C3E88112"));
        assert_eq!(compute_value(&p), 7);
        let p = parse(&parse_hex("CE00C43D881120"));
        assert_eq!(compute_value(&p), 9);
        let p = parse(&parse_hex("D8005AC2A8F0"));
        assert_eq!(compute_value(&p), 1);
        let p = parse(&parse_hex("F600BC2D8F"));
        assert_eq!(compute_value(&p), 0);
        let p = parse(&parse_hex("9C005AC2F8F0"));
        assert_eq!(compute_value(&p), 0);
        let p = parse(&parse_hex("9C0141080250320F1802104A08"));
        assert_eq!(compute_value(&p), 1);
    }
}
