use super::*;

pub enum Gesture {
    Tap(Pos),
    ScrollY(Pos, i32),
    ScrollX(Pos, i32)
}

pub struct InputState {
    pub start: Option<Pos>,
    pub current: Option<Pos>,
    gesture: Option<Gesture>,

    pub threshold: i32
}

impl InputState {
    pub fn new(threshold: i32) -> Self {
        InputState {
            start: None, current: None, gesture: None, threshold
        }
    }

    fn set_gesture(&mut self, gesture: Gesture) {
        self.gesture = Some(gesture);
    }

    pub fn touch_down(&mut self, pos: Pos) {
        self.start = Some(pos);
        self.current = Some(pos);
    }

    pub fn touch_move(&mut self, pos: Pos) {
        if let None = self.start {
            self.start = Some(pos);
        }
        
        self.current = Some(pos);

        if let (Some(current), Some(start)) = (&self.current, self.start) {
            let diff = (current.0 - start.0, current.1 - start.1);
            
            if diff.0.abs() > self.threshold {
                self.set_gesture(Gesture::ScrollX(start, diff.0));
            } else if diff.1.abs() > self.threshold {
                self.set_gesture(Gesture::ScrollY(start, diff.1));
            }
        }
    }

    pub fn touch_up(&mut self) {
        if let (Some(current), None) = (self.current.take(), &self.gesture) {
            self.set_gesture(Gesture::Tap(current));
        }

        self.start = None;
    }

    pub fn get_gesture(&self) -> &Option<Gesture> {
        &self.gesture
    }
}