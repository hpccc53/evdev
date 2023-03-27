use std::fmt;
use std::ops::Deref;
use std::time::SystemTime;

use crate::compat::input_event;
use crate::constants::{
    AbsoluteAxisType, FFStatusType, LedType, MiscType, OtherType, PowerType, RelativeAxisType,
    RepeatType, SoundType, SwitchType, SynchronizationType, UInputType,
};
use crate::scancodes::KeyType;
use crate::{systime_to_timeval, EventType, FFEffectType};
use crate::{EventSummary, InputEvent};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// A bookkeeping event. Usually not important to applications.
/// [`EventType::SYNCHRONIZATION`]
pub struct SynchronizationEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::KEY`]
pub struct KeyEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::RELATIVE`]
pub struct RelativeAxisEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::ABSOLUTE`]
pub struct AbsoluteAxisEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::MISC`]
pub struct MiscEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::SWITCH`]
pub struct SwitchEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::LED`]
pub struct LedEvent(InputEvent);
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::SOUND`]
pub struct SoundEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::REPEAT`]
pub struct RepeatEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::FORCEFEEDBACK`]
pub struct FFEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::POWER`]
pub struct PowerEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::FORCEFEEDBACKSTATUS`]
pub struct FFStatusEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// [`EventType::UINPUT`]
pub struct UInputEvent(InputEvent);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// No clue, but technically possible.
pub struct OtherEvent(pub(crate) InputEvent);

macro_rules! input_event_newtype {
    ($name:ty) => {
        impl AsRef<input_event> for $name {
            fn as_ref(&self) -> &input_event {
                &self.0.as_ref()
            }
        }
        impl AsRef<InputEvent> for $name {
            fn as_ref(&self) -> &InputEvent {
                &self.0
            }
        }
        // never implement the other direction!
        impl From<$name> for InputEvent {
            fn from(event: $name) -> Self {
                event.0
            }
        }
        impl Deref for $name {
            type Target = InputEvent;
            fn deref<'a>(&'a self) -> &'a InputEvent {
                &self.0
            }
        }
    };
    ($name:ty, $evdev_type:path, $kind:path) => {
        impl $name {
            pub fn new($kind(code): $kind, value: i32) -> Self {
                let raw = input_event {
                    time: libc::timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                    type_: $evdev_type.0,
                    code,
                    value,
                };
                Self::from_raw(raw)
            }
            pub fn new_now($kind(code): $kind, value: i32) -> Self {
                let raw = input_event {
                    time: systime_to_timeval(&SystemTime::now()),
                    type_: $evdev_type.0,
                    code,
                    value,
                };
                Self::from_raw(raw)
            }
            pub fn destructure(&self) -> ($kind, i32) {
                ($kind(self.code()), self.value())
            }
            // must be kept internal
            fn from_raw(raw: input_event) -> Self {
                match EventType(raw.type_) {
                    $evdev_type => Self(InputEvent(raw)),
                    _ => unreachable!(),
                }
            }
            // must be kept internal
            pub(crate) fn from_event(event: InputEvent) -> Self {
                match event.event_type() {
                    $evdev_type => Self(event),
                    _ => unreachable!(),
                }
            }
            pub fn kind(&self) -> $kind {
                $kind(self.code())
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut debug = f.debug_struct(stringify!($name));
                debug.field("time", &self.timestamp());
                debug.field("kind", &self.kind());
                debug.field("value", &self.value()).finish()
            }
        }
        input_event_newtype!($name);
    };
    ($name:ty, $evdev_type:path, $kind:path, $summary:path) => {
        impl From<$name> for EventSummary {
            fn from(event: $name) -> EventSummary {
                let (kind, value) = event.destructure();
                $summary(event, kind, value)
            }
        }

        input_event_newtype!($name, $evdev_type, $kind);
    };
}
input_event_newtype!(
    SynchronizationEvent,
    EventType::SYNCHRONIZATION,
    SynchronizationType,
    EventSummary::Synchronization
);
input_event_newtype!(KeyEvent, EventType::KEY, KeyType, EventSummary::Key);
input_event_newtype!(
    RelativeAxisEvent,
    EventType::RELATIVE,
    RelativeAxisType,
    EventSummary::RelativeAxis
);
input_event_newtype!(
    AbsoluteAxisEvent,
    EventType::ABSOLUTE,
    AbsoluteAxisType,
    EventSummary::AbsoluteAxis
);
input_event_newtype!(MiscEvent, EventType::MISC, MiscType, EventSummary::Misc);
input_event_newtype!(
    SwitchEvent,
    EventType::SWITCH,
    SwitchType,
    EventSummary::Switch
);
input_event_newtype!(LedEvent, EventType::LED, LedType, EventSummary::Led);
input_event_newtype!(SoundEvent, EventType::SOUND, SoundType, EventSummary::Sound);
input_event_newtype!(
    RepeatEvent,
    EventType::REPEAT,
    RepeatType,
    EventSummary::Repeat
);
input_event_newtype!(
    FFEvent,
    EventType::FORCEFEEDBACK,
    FFEffectType,
    EventSummary::ForceFeedback
);
input_event_newtype!(PowerEvent, EventType::POWER, PowerType, EventSummary::Power);
input_event_newtype!(
    FFStatusEvent,
    EventType::FORCEFEEDBACKSTATUS,
    FFStatusType,
    EventSummary::ForceFeedbackStatus
);
input_event_newtype!(
    UInputEvent,
    EventType::UINPUT,
    UInputType,
    EventSummary::UInput
);
input_event_newtype!(OtherEvent);

impl OtherEvent {
    pub fn kind(&self) -> OtherType {
        OtherType(self.event_type().0, self.code())
    }
    pub fn destructure(&self) -> (OtherType, i32) {
        (self.kind(), self.value())
    }
}
impl From<OtherEvent> for EventSummary {
    fn from(event: OtherEvent) -> Self {
        let (kind, value) = event.destructure();
        EventSummary::Other(event, kind, value)
    }
}
impl fmt::Debug for OtherEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("OtherEvent");
        debug.field("time", &self.timestamp());
        debug.field("type", &self.event_type());
        debug.field("code", &self.code());
        debug.field("value", &self.value()).finish()
    }
}
