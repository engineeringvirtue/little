use super::*;

pub enum TouchGesture {
    Tap(Pos),
    ScrollY(Pos, i32),
    ScrollX(Pos, i32)
}

pub struct TouchInputState {
    pub start: Option<Pos>,
    pub current: Option<Pos>,
    gesture: Option<TouchGesture>,

    pub threshold: i32
}

impl TouchInputState {
    pub fn new(threshold: i32) -> Self {
        TouchInputState {
            start: None, current: None, gesture: None, threshold
        }
    }

    fn set_gesture(&mut self, gesture: TouchGesture) {
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
            let diff = *current - start;
            
            if diff.x.abs() > self.threshold {
                self.set_gesture(TouchGesture::ScrollX(start, diff.x));
            } else if diff.y.abs() > self.threshold {
                self.set_gesture(TouchGesture::ScrollY(start, diff.y));
            }
        }
    }

    pub fn touch_up(&mut self) {
        if let (Some(current), None) = (self.current.take(), &self.gesture) {
            self.set_gesture(TouchGesture::Tap(current));
        }

        self.start = None;
    }

    pub fn get_gesture(&self) -> &Option<TouchGesture> {
        &self.gesture
    }
}