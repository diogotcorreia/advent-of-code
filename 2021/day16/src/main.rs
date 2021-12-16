struct Buffer {
    bytes: Vec<u8>,
    current_pos: u32,
}

impl Buffer {
    fn from(s: &str) -> Buffer {
        let mut buffer = Buffer {
            bytes: Vec::new(),
            current_pos: 0,
        };
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                buffer
                    .bytes
                    .push(u8::from_str_radix(&c.to_string(), 16).expect("invalid hex string"));
            } else {
                let last = buffer.bytes.last_mut().unwrap();
                *last = (*last << 4)
                    + u8::from_str_radix(&c.to_string(), 16).expect("invalid hex string");
            }
        }

        buffer
    }

    fn read_bits(&mut self, amount: u32) -> u32 {
        let target_pos = self.current_pos + amount;
        let mut result: u32 = 0;

        while self.current_pos < target_pos {
            let mut current_byte = *self.bytes.get((self.current_pos / 8) as usize).unwrap();
            let discard_left = self.current_pos % 8;
            current_byte = current_byte << discard_left >> discard_left;
            let discard_right =
                ((((self.current_pos + 8) / 8) * 8) as i32 - target_pos as i32).max(0) as u32;
            current_byte = current_byte >> discard_right;
            result = result << (8 - discard_left - discard_right);
            result += current_byte as u32;

            self.current_pos += 8 - discard_left - discard_right;
        }

        result
    }
}

struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

impl Packet {
    fn get_version_sum(&self) -> u64 {
        self.version as u64
            + match &self.content {
                PacketContent::LiteralValue(_) => 0,
                PacketContent::PacketWrapper(packets) => {
                    packets.iter().map(|p| p.get_version_sum()).sum()
                }
            }
    }

    fn calculate(&self) -> u128 {
        match &self.content {
            PacketContent::LiteralValue(v) => *v,
            PacketContent::PacketWrapper(packets) => {
                let mut packets = packets.iter().map(|p| p.calculate());
                match self.type_id {
                    0 => {
                        // sum
                        packets.sum()
                    }
                    1 => {
                        // product
                        packets.product()
                    }
                    2 => {
                        // min
                        packets.min().unwrap()
                    }
                    3 => {
                        // max
                        packets.max().unwrap()
                    }
                    5 => {
                        // greater than
                        if packets.next().unwrap() > packets.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        // less than
                        if packets.next().unwrap() < packets.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        // equal to
                        if packets.next().unwrap() == packets.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!("unknown packet type"),
                }
            }
        }
    }
}

enum PacketContent {
    LiteralValue(u128),
    PacketWrapper(Vec<Packet>),
}

fn main() {
    let mut buffer = {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        Buffer::from(&input)
    };

    //dbg!(parse_packet(&mut buffer));
    let packet = parse_packet(&mut buffer);

    println!("Part 1: {}", packet.get_version_sum());
    println!("Part 2: {}", packet.calculate());
}

fn parse_packet(buffer: &mut Buffer) -> Packet {
    let version = buffer.read_bits(3);
    let type_id = buffer.read_bits(3);
    let content = match type_id {
        4 => {
            let mut num: u128 = 0;
            loop {
                let control_digit = buffer.read_bits(1);
                let part_num = buffer.read_bits(4);
                num = num << 4;
                num += part_num as u128;

                if control_digit == 0 {
                    break;
                }
            }
            PacketContent::LiteralValue(num)
        }
        _ => {
            let length_type = buffer.read_bits(1);
            match length_type {
                0 => {
                    // length is length in bits
                    let length = buffer.read_bits(15);

                    let mut packets = Vec::new();
                    let target_pos = buffer.current_pos + length;

                    while buffer.current_pos < target_pos {
                        packets.push(parse_packet(buffer));
                    }
                    PacketContent::PacketWrapper(packets)
                }
                1 => {
                    // length is number of packets
                    let length = buffer.read_bits(11);

                    let mut packets = Vec::new();
                    for _ in 0..length {
                        packets.push(parse_packet(buffer));
                    }
                    PacketContent::PacketWrapper(packets)
                }
                _ => unreachable!("unknown length type"),
            }
        }
    };

    Packet {
        version: version as u8,
        type_id: type_id as u8,
        content,
    }
}
