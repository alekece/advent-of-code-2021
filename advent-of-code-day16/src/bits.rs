use std::io::{BufReader, Read};

use derive_more::Deref;
use eyre::eyre;

use aoc_core::Result;

trait Decode {
  fn decode(bits: &[u8]) -> Result<(usize, Self)>
  where
    Self: Sized;
}

#[derive(Debug, Default)]
struct PacketHeader {
  version: u8,
  r#type: u8,
}

impl Decode for PacketHeader {
  fn decode(bits: &[u8]) -> Result<(usize, Self)> {
    const HEADER_LENGTH: usize = 6;

    if bits.len() < HEADER_LENGTH {
      Err(eyre!("invalid header packet: {:?}", bits))
    } else {
      Ok((
        HEADER_LENGTH,
        Self {
          version: bits_to_usize(&bits[0..3]) as u8,
          r#type: bits_to_usize(&bits[3..6]) as u8,
        },
      ))
    }
  }
}

#[derive(Debug)]
struct LiteralValuePacket(usize);

impl Decode for LiteralValuePacket {
  fn decode(bits: &[u8]) -> Result<(usize, Self)>
  where
    Self: Sized,
  {
    let mut state = true;

    let bits = bits
      .chunks(5)
      .take_while(|bits| {
        if !state {
          false
        } else {
          state = bits[0] == 1;

          true
        }
      })
      .map(|bits| &bits[1..])
      .flatten()
      .copied()
      .collect::<Vec<_>>();

    if state {
      Err(eyre!("invalid literal value packet: {:?}", bits))
    } else {
      Ok((bits.len() + bits.len() / 4, Self(bits_to_usize(&bits))))
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
struct OperationPacket {
  label: u8,
  value: usize,
  packets: Vec<Packet>,
}

impl Decode for OperationPacket {
  fn decode(bits: &[u8]) -> Result<(usize, Self)>
  where
    Self: Sized,
  {
    let label = bits[0];
    let mut packets = Vec::default();

    match label {
      0 => {
        let value = bits_to_usize(&bits[1..16]);
        let mut offset = 0;

        {
          let bits = &bits[16..];

          while offset < value {
            let packet = decode_packet(&bits[offset..])?;

            offset += packet.size;

            packets.push(packet);
          }
        }

        if offset > value {
          Err(eyre!("invalid operation packet: {:?}", bits))
        } else {
          Ok((
            16 + value,
            Self {
              label,
              value,
              packets,
            },
          ))
        }
      }
      1 => {
        let value = bits_to_usize(&bits[1..12]);
        let bits = &bits[12..];
        let mut offset = 0;

        for _ in 0..value {
          let packet = decode_packet(&bits[offset..])?;

          offset += packet.size;

          packets.push(packet);
        }

        Ok((
          12 + offset,
          Self {
            label,
            value,
            packets,
          },
        ))
      }
      _ => Err(eyre!("invalid '{}' labeled bit", label)),
    }
  }
}

#[derive(Debug)]
enum PacketContent {
  LiteralValue(LiteralValuePacket),
  Operation(OperationPacket),
}

#[derive(Debug)]
pub struct Packet {
  size: usize,
  header: PacketHeader,
  content: PacketContent,
}

impl Packet {
  pub fn get_version(&self) -> usize {
    match &self.content {
      PacketContent::LiteralValue(_) => self.header.version as usize,
      PacketContent::Operation(content) => {
        self.header.version as usize
          + content
            .packets
            .iter()
            .map(Packet::get_version)
            .sum::<usize>()
      }
    }
  }

  pub fn evaluate(&self) -> Result<usize> {
    match &self.content {
      PacketContent::LiteralValue(content) => Ok(content.0),
      PacketContent::Operation(content) => {
        match (
          self.header.r#type,
          &content
            .packets
            .iter()
            .map(Packet::evaluate)
            .collect::<Result<Vec<_>>>()?[..],
        ) {
          (0, values) => Ok(values.iter().sum::<usize>()),
          (1, values) => Ok(values.iter().product::<usize>()),
          (2, values) => Ok(*values.iter().min().unwrap()),
          (3, values) => Ok(*values.iter().max().unwrap()),
          (5, &[a, b]) => Ok((a > b).then(|| 1).unwrap_or(0)),
          (6, &[a, b]) => Ok((a < b).then(|| 1).unwrap_or(0)),
          (7, &[a, b]) => Ok((a == b).then(|| 1).unwrap_or(0)),
          _ => Err(eyre!("invalid operation")),
        }
      }
    }
  }
}

#[derive(Debug, Deref)]
pub struct PacketDecoder(Packet);

impl PacketDecoder {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    let bits = buffer
      .trim()
      .chars()
      .map(hex_to_bits)
      .collect::<Result<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<_>>();

    Ok(Self(decode_packet(&bits[..])?))
  }
}

fn decode_packet(bits: &[u8]) -> Result<Packet> {
  let (header_size, header) = PacketHeader::decode(bits)?;

  let (content_size, content) = match header.r#type {
    4 => LiteralValuePacket::decode(&bits[header_size..])
      .map(|(size, content)| (size, PacketContent::LiteralValue(content)))?,
    _ => OperationPacket::decode(&bits[header_size..])
      .map(|(size, content)| (size, PacketContent::Operation(content)))?,
  };

  let size = header_size + content_size;

  Ok(Packet {
    size,
    header,
    content,
  })
}

fn hex_to_bits(c: char) -> Result<[u8; 4]> {
  match c {
    '0' => Ok([0, 0, 0, 0]),
    '1' => Ok([0, 0, 0, 1]),
    '2' => Ok([0, 0, 1, 0]),
    '3' => Ok([0, 0, 1, 1]),
    '4' => Ok([0, 1, 0, 0]),
    '5' => Ok([0, 1, 0, 1]),
    '6' => Ok([0, 1, 1, 0]),
    '7' => Ok([0, 1, 1, 1]),
    '8' => Ok([1, 0, 0, 0]),
    '9' => Ok([1, 0, 0, 1]),
    'A' => Ok([1, 0, 1, 0]),
    'B' => Ok([1, 0, 1, 1]),
    'C' => Ok([1, 1, 0, 0]),
    'D' => Ok([1, 1, 0, 1]),
    'E' => Ok([1, 1, 1, 0]),
    'F' => Ok([1, 1, 1, 1]),
    _ => Err(eyre!("invalid '{}' hexadecimal", c)),
  }
}

fn bits_to_usize(bits: &[u8]) -> usize {
  bits.iter().fold(0, |acc, v| (acc << 1) | *v as usize)
}
