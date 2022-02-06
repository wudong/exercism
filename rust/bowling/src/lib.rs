use std::ptr::NonNull;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
pub enum FrameType {
    Strike,
    Spare,
    Open,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct BowlingFrame {
    throws: (u16, u16),
    next: Option<NonNull<BowlingFrame>>,
}

impl BowlingFrame {

    fn get_type(&self)-> FrameType {
        if self.throws.0 ==10_  {
             FrameType::Strike
        } else if self.throws.0 + self.throws.1 == 10 {
             FrameType::Spare
        } else {
            FrameType::Open
        }
    }

    fn get_score(&self, is_last: bool) -> u16 {
        let cur = (self.throws.0 + self.throws.1) as u16;
        match self.get_type() {
            FrameType::Open => cur,
            FrameType::Spare=> unsafe {cur + self.next.map(|x|x.as_ref().throws.0).unwrap_or(0)},
            FrameType::Strike => unsafe {
                let ss = self.next.map(|x|x.as_ref().throws.0).unwrap_or(0);
                if ss == 10 { // strike again, get the next's next one.
                    let nn = if !is_last{
                       self.next.unwrap().as_ref().next.map(|x|x.as_ref().throws.0).unwrap_or(0)
                   } else {
                       self.next.map(|x|x.as_ref().throws.1).unwrap_or(0)
                   };
                   cur + ss + nn
                } else {
                   cur + ss +  self.next.map(|x|x.as_ref().throws.1).unwrap_or(0)
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    current_frame: usize,
    current_throw: u8,
    frames: [BowlingFrame; 11],
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut ss = Self {
            current_frame: 0,
            current_throw: 0,
            frames: [BowlingFrame::default(); 11],
        };

        let frames = &mut ss.frames;
        for i in 0..10 {
            frames[i].next = NonNull::new(&mut frames[i+1] as *mut _);
        }

        ss
    }

    fn last_frame(&self) -> Option<&BowlingFrame> {
        match self.current_frame {
            0 => Option::None,
            x=> Option::Some( &self.frames[x - 1]),
        }
    }

    pub fn is_game_complete(&self)-> bool {
        match self.current_frame {
            11..=usize::MAX => true,
            10 => {
              let last = self.last_frame().unwrap();
              match last.get_type() {
                  FrameType::Open => true,
                  FrameType::Spare=> self.current_throw == 1,
                  FrameType::Strike => false,
              }
            },
            _=> false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            Result::Err(Error::GameComplete)
        } else {
           let cur = &mut self.frames[self.current_frame];
           let pp = if self.current_throw == 0 {
               (pins, 0)
           } else {
               (cur.throws.0, pins)
           };
           let is_bonus = self.current_frame == 10;
           let total = pp.0 + pp.1;

           if !is_bonus && total > 10 {
               Result::Err(Error::NotEnoughPinsLeft)
           }else if is_bonus && pp.0 != 10 && total > 10 {
               Result::Err(Error::NotEnoughPinsLeft)
           } else if is_bonus && pp.0 == 10 && total > 20 {
               Result::Err(Error::NotEnoughPinsLeft)
           } else {
               self.update_with_throws(pp, is_bonus);
               Result::Ok(())
           }
        }
    }

    pub fn update_with_throws(&mut self, throws: (u16, u16), is_bonus: bool) {
        let cur = &mut self.frames[self.current_frame];
        cur.throws = throws;
        if self.current_throw==1 || self.current_throw==0 && throws.0==10 && !is_bonus {
            self.current_frame +=1;
            self.current_throw = 0;
        } else {
            self.current_throw = 1;
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            Option::None
        } else {
            let score = self.frames.iter().enumerate()
                .take_while(|x|x.0 < 10)
                .fold(0, |acc, x|acc+x.1.get_score(x.0==9));
            Option::Some(score)
        }
    }
}
