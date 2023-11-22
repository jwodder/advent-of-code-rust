use adventutil::{FromBits, Input};

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

fn decode(s: &str) -> Packet {
    decode_bits(&hex2bits(s)).0
}

#[allow(clippy::missing_asserts_for_indexing)]
fn decode_bits(bits: &[bool]) -> (Packet, &[bool]) {
    let version = u64::from_bits(bits[0..3].iter().copied());
    let type_id = u64::from_bits(bits[3..6].iter().copied());
    if type_id == 4 {
        let mut i = 6;
        let mut value = 0;
        loop {
            let continued = bits[i];
            let nibble = u64::from_bits(bits[(i + 1)..(i + 5)].iter().copied());
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
            let bit_len = usize::from_bits(bits[7..22].iter().copied());
            let mut raw_subpackets = &bits[22..(22 + bit_len)];
            while !raw_subpackets.is_empty() {
                let (p, rsp) = decode_bits(raw_subpackets);
                packets.push(p);
                raw_subpackets = rsp;
            }
            &bits[(22 + bit_len)..]
        } else {
            let packet_qty = u64::from_bits(bits[7..18].iter().copied());
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
    version_total(decode(input.read().trim()))
}

fn version_total(packet: Packet) -> u64 {
    match packet {
        Packet::Literal { version, .. } => version,
        Packet::Operator {
            version, packets, ..
        } => version + packets.into_iter().map(version_total).sum::<u64>(),
    }
}

fn hex2bits(s: &str) -> Vec<bool> {
    s.chars()
        .flat_map(|c| {
            let value = c.to_digit(16).expect("Invalid hex digit");
            (0..4).map(move |i| (value & (1 << (3 - i))) != 0)
        })
        .collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("D2FE28", Packet::Literal {version: 6, value: 2021})]
    #[case("38006F45291200", Packet::Operator {version: 1, op: 6, packets: vec![Packet::Literal {version: 6, value: 10}, Packet::Literal {version: 2, value: 20}]})]
    #[case("EE00D40C823060", Packet::Operator {version: 7, op: 3, packets: vec![Packet::Literal {version: 2, value: 1}, Packet::Literal {version: 4, value: 2}, Packet::Literal {version: 1, value: 3}]})]
    fn test_decode(#[case] s: &str, #[case] packet: Packet) {
        assert_eq!(decode(s), packet);
    }

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_solve(#[case] s: &'static str, #[case] value: u64) {
        assert_eq!(solve(Input::from(s)), value);
    }
}
