use super::HexNeighbour;
use bevy::prelude::*;
use consts::*;
use serde::ser::SerializeTuple;

pub trait Hex: Copy + Sized + std::ops::Add<Self, Output = Self> {
    const ZERO: Self;
    fn q(&self) -> i32;
    fn r(&self) -> i32;
    #[inline(always)]
    fn s(&self) -> i32 {
        -self.q() - self.r()
    }
    fn new(q: i32, r: i32) -> Self;
    #[inline(always)]
    fn scale(&self, factor: u32) -> Self {
        let factor = factor as i32;
        Self::new(self.q() * factor, self.r() * factor)
    }
    #[inline(always)]
    fn neighbour(&self, neighbour: HexNeighbour) -> Self {
        match neighbour {
            HexNeighbour::One => Self::new(self.q() + 1, self.r() - 1),
            HexNeighbour::Two => Self::new(self.q() + 1, self.r()),
            HexNeighbour::Three => Self::new(self.q(), self.r() + 1),
            HexNeighbour::For => Self::new(self.q() - 1, self.r() + 1),
            HexNeighbour::Five => Self::new(self.q() - 1, self.r()),
            HexNeighbour::Six => Self::new(self.q(), self.r() - 1),
        }
    }
    #[inline(always)]
    fn neighbours(&self) -> [Self; 6] {
        [
            Self::new(self.q() + 1, self.r() - 1),
            Self::new(self.q() + 1, self.r()),
            Self::new(self.q(), self.r() + 1),
            Self::new(self.q() - 1, self.r() + 1),
            Self::new(self.q() - 1, self.r()),
            Self::new(self.q(), self.r() - 1),
        ]
    }
}

pub struct HexRangeIterator {
    q: std::ops::RangeInclusive<i32>,
    current_q: i32,
    r: std::ops::RangeInclusive<i32>,
    size: i32,
}

impl HexRangeIterator {
    pub fn new(range: u32) -> HexRangeIterator {
        let range = range as i32;
        HexRangeIterator {
            q: -range + 1..=range,
            current_q: -range,
            r: 0..=range,
            size: range,
        }
    }
}

impl Iterator for HexRangeIterator {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        match self.r.next() {
            None => match self.q.next() {
                Some(q) => {
                    self.current_q = q;
                    self.r = (-self.size).max(-q - self.size)..=(self.size).min(-q + self.size);
                    if let Some(r) = self.r.next() {
                        Some(CellId::new(self.current_q, r))
                    } else {
                        None
                    }
                }
                None => None,
            },
            Some(r) => Some(CellId::new(self.current_q, r)),
        }
    }
}

pub struct HexRingIterator {
    radius: u32,
    current_r: std::ops::Range<u32>,
    current: CellId,
    edge: std::iter::Peekable<std::slice::Iter<'static, HexNeighbour>>,
}

impl HexRingIterator {
    pub fn new(radius: u32) -> HexRingIterator {
        Self {
            radius,
            current_r: 0..radius,
            current: CellId::new(-(radius as i32), 0),
            edge: if radius != 0 {
                [
                    HexNeighbour::One,
                    HexNeighbour::Two,
                    HexNeighbour::Three,
                    HexNeighbour::For,
                    HexNeighbour::Five,
                    HexNeighbour::Six,
                ]
                .iter()
                .peekable()
            } else {
                [HexNeighbour::One].iter().peekable()
            },
        }
    }
}

impl Iterator for HexRingIterator {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(neighbor) = self.edge.peek() else {return None;};
        if self.current_r.next().is_some() {
            let next = self.current.neighbour(**neighbor);
            Some(std::mem::replace(&mut self.current, next))
        } else {
            self.edge.next();
            self.current_r = 0..self.radius;
            if self.radius != 0 {
                self.next()
            } else {
                Some(CellId::ZERO)
            }
        }
    }
}

pub struct HexSpiralIterator {
    radius: u32,
    max_radius: u32,
    current_ring: HexRingIterator,
}

impl HexSpiralIterator {
    #[allow(dead_code)]
    pub fn new(radius: u32) -> HexSpiralIterator {
        Self {
            current_ring: HexRingIterator::new(0),
            radius: 0,
            max_radius: radius,
        }
    }
}

impl Iterator for HexSpiralIterator {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_ring.next().or_else(|| {
            self.radius += 1;
            if self.radius > self.max_radius {
                return None;
            }
            self.current_ring = HexRingIterator::new(self.radius);
            self.current_ring.next()
        })
    }
}

pub trait WithOffset<Iter: Sized + Iterator<Item = CellId>> {
    fn with_offset(self, offset: CellId) -> OffsetIter<Iter>;
}

impl<Iter: Iterator<Item = CellId>> WithOffset<Iter> for Iter {
    fn with_offset(self, offset: CellId) -> OffsetIter<Iter> {
        OffsetIter { offset, iter: self }
    }
}

pub struct OffsetIter<Iter: Iterator<Item = CellId> + Sized> {
    offset: CellId,
    iter: Iter,
}

impl<Iter: Iterator<Item = CellId>> Iterator for OffsetIter<Iter> {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.offset + self.iter.next()?)
    }
}

mod consts {
    #![allow(unused)]
    pub const SQRT_3: f32 = 1.7320508075688772935274463415059;
    pub const SQRT_3DIV2: f32 = 0.86602540378443864676372317075294;
    pub const SQRT_3DIV3: f32 = 0.57735026918962576450914878050196;
    pub const ONE_AND_ONETHIRED: f32 = 1. / 0.75;
    pub const ONETHIRD: f32 = 1. / 3.;
    pub const TWOTHIRD: f32 = 2. / 3.;
    pub const HEXROT: f32 = 0.523599 * 2.0;
}

#[derive(Debug, Component, Reflect, Default, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
pub struct CellId {
    q: i32,
    r: i32,
}

impl Into<u64> for CellId {
    fn into(self) -> u64 {
        self.r as u64 ^ ((self.q as u64) << 32)
    }
}

impl std::hash::Hash for CellId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.q);
        state.write_i32(self.r);
        // state.write_u64(self.r as u64 ^ ((self.q as u64) << 32))
    }
}

impl std::ops::Add for CellId {
    type Output = CellId;
    fn add(self, rhs: Self) -> Self::Output {
        CellId {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl std::ops::Sub for CellId {
    type Output = CellId;
    fn sub(self, rhs: Self) -> Self::Output {
        CellId {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl serde::Serialize for CellId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.q)?;
        tuple.serialize_element(&self.r)?;
        tuple.end()
    }
}

impl<'de> serde::Deserialize<'de> for CellId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, HexIdVisitor)
    }
}

struct HexIdVisitor;
impl<'de> serde::de::Visitor<'de> for HexIdVisitor {
    type Value = CellId;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("HexId tuple")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;
        Ok(CellId {
            q: seq
                .next_element::<i32>()?
                .ok_or(Error::missing_field("Q"))?,
            r: seq
                .next_element::<i32>()?
                .ok_or(Error::missing_field("R"))?,
        })
    }
}

impl CellId {
    pub const fn new(q: i32, r: i32) -> CellId {
        Self { q, r }
    }

    #[inline(always)]
    pub fn xyz(&self, y: f32) -> Vec3 {
        let z = 0.75 * self.q as f32;
        let x = (self.q as f32 * 0.5 + self.r as f32) * SQRT_3DIV2;
        Vec3::new(x, y, z)
    }
}

impl std::fmt::Display for CellId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.q, self.r))
    }
}

impl super::ids::Hex for CellId {
    const ZERO: Self = CellId { q: 0, r: 0 };
    fn q(&self) -> i32 {
        self.q
    }
    fn r(&self) -> i32 {
        self.r
    }
    #[inline(always)]
    fn new(q: i32, r: i32) -> Self {
        CellId::new(q, r)
    }
}
