#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Frame {
    Strike,
    Spare(u8, u8),
    Open(u8, u8),
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::with_capacity(21),
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.rolls.len() > 19 {
            return Err(Error::GameComplete);
        }

        let mut frames = self.rolls.chunks(2);
        let frame = frames.nth(self.rolls.len() / 2);
        let remaining_pins: u8 = 10 - frame.unwrap_or(&[0]).iter().sum::<u8>();
        if remaining_pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.rolls.push(pins);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls.len() < 20 {
            return None;
        }
        let score = vec![Frame::Open(0, 0)]
            .into_iter()
            .chain(
                self.rolls
                    .chunks(2)
                    .map(|frame| {
                        println!("frame:{:?}", &frame);
                        if frame[0] == 10 {
                            Frame::Strike
                        } else if frame.iter().sum::<u8>() == 10 {
                            Frame::Spare(frame[0], frame[1])
                        } else {
                            Frame::Open(frame[0], frame[1])
                        }
                    })
                    .into_iter(),
            )
            .collect::<Vec<Frame>>()
            .windows(2)
            .enumerate()
            .fold(0u16, |acc, (i, frame)| {
                let (prev, curr) = (&frame[0], &frame[1]);

                let tap = acc
                    + match (curr, prev) {
                        (Frame::Spare(a, 0), Frame::Spare(_, _)) => *a as u16,
                        (Frame::Open(a, 0), Frame::Spare(_, _)) => (2 * a) as u16,
                        (Frame::Spare(_, _), Frame::Open(_, _)) => 10u16,
                        (Frame::Open(a, b), _) => (a + b) as u16,
                        _ => 0,
                    };
                println!("{}: Total {:}\tRoll {:?} \tLast {:?}", i, &tap, curr, prev);
                tap
            });

        // let score = frames.windows(2).fold(0u16, |acc, frame| {
        //     dbg!(&frame);
        //     acc + match (&frame[0], &frame[1]) {
        //         (Frame::Spare(_), Frame::Open(next, _)) => 10u16 + (*next as u16) * 2,
        //         (_, Frame::Open(a, b)) => (a + b) as u16,
        //         _ => 0,
        //     }
        // });

        Some(score)
    }
}
