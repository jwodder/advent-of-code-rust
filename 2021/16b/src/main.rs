use adventutil::Input;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u64,
        value: u64,
    },
    Operator {
        version: u64,
        op: u64,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { op: 0, packets, .. } => packets.iter().map(Packet::evaluate).sum(),
            Packet::Operator { op: 1, packets, .. } => {
                packets.iter().map(Packet::evaluate).product()
            }
            Packet::Operator { op: 2, packets, .. } => {
                packets.iter().map(Packet::evaluate).min().unwrap()
            }
            Packet::Operator { op: 3, packets, .. } => {
                packets.iter().map(Packet::evaluate).max().unwrap()
            }
            Packet::Operator { op: 5, packets, .. } => {
                u64::from(packets[0].evaluate() > packets[1].evaluate())
            }
            Packet::Operator { op: 6, packets, .. } => {
                u64::from(packets[0].evaluate() < packets[1].evaluate())
            }
            Packet::Operator { op: 7, packets, .. } => {
                u64::from(packets[0].evaluate() == packets[1].evaluate())
            }
            Packet::Operator { op, .. } => panic!("Invalid operator packet type {op}"),
        }
    }
}

fn decode(s: &str) -> Packet {
    decode_bits(&hex2bits(s)).0
}

#[allow(clippy::missing_asserts_for_indexing)]
fn decode_bits(bits: &[bool]) -> (Packet, &[bool]) {
    let version = bits2num(&bits[0..3]);
    let type_id = bits2num(&bits[3..6]);
    if type_id == 4 {
        let mut i = 6;
        let mut value = 0;
        loop {
            let continued = bits[i];
            let nibble = bits2num(&bits[(i + 1)..(i + 5)]);
            value = (value << 4) + nibble;
            if !continued {
                break;
            }
            i += 5;
        }
        (Packet::Literal { version, value }, &bits[(i + 5)..])
    } else {
        let mut packets = Vec::new();
        let remainder = if !bits[6] {
            let bit_len = usize::try_from(bits2num(&bits[7..22])).unwrap();
            let mut raw_subpackets = &bits[22..(22 + bit_len)];
            while !raw_subpackets.is_empty() {
                let (p, rsp) = decode_bits(raw_subpackets);
                packets.push(p);
                raw_subpackets = rsp;
            }
            &bits[(22 + bit_len)..]
        } else {
            let packet_qty = bits2num(&bits[7..18]);
            let mut raw = &bits[18..];
            for _ in 0..packet_qty {
                let (p, raw2) = decode_bits(raw);
                packets.push(p);
                raw = raw2;
            }
            raw
        };
        (
            Packet::Operator {
                version,
                op: type_id,
                packets,
            },
            remainder,
        )
    }
}

fn solve(input: Input) -> u64 {
    decode(input.read().trim()).evaluate()
}

fn hex2bits(s: &str) -> Vec<bool> {
    s.chars()
        .flat_map(|c| {
            let value = c.to_digit(16).expect("Invalid hex digit");
            (0..4).map(move |i| (value & (1 << (3 - i))) != 0)
        })
        .collect()
}

fn bits2num(bits: &[bool]) -> u64 {
    bits.iter().fold(0, |n, &b| (n << 1) + u64::from(b))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]
    fn examples(#[case] s: &'static str, #[case] value: u64) {
        assert_eq!(solve(Input::from(s)), value);
    }
}
