use compact_str::CompactString;

use crate::{event::CopyMove, key::Key, mouse::MouseEvent};

#[derive(Debug)]
pub enum ProcCmd {
  Start,
  Stop,
  Kill,

  SendKey(Key),
  SendMouse(MouseEvent),
  SendRaw(CompactString),

  ScrollUp,
  ScrollDown,
  ScrollUpLines { n: usize },
  ScrollDownLines { n: usize },

  CopyModeEnter,
  CopyModeLeave,
  CopyModeMove { dir: CopyMove },
  CopyModeEnd,
  CopyModeCopy,

  Resize { x: u16, y: u16, w: u16, h: u16 },
}

#[derive(Debug)]
pub enum ProcEvent {
  Render,
  Exited(u32),
  StdoutEOF,
  Started,
  TermReply(CompactString),
}
