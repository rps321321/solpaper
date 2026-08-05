//! Platform-neutral Pomodoro state machine (Issue #19).
//!
//! Pure logic: injectable wall-clock (`UnixMs`), no threads, no Win32.
//! Design note: `docs/design/pomodoro-state-machine.md`.

use serde::{Deserialize, Serialize};

use crate::CoreError;

/// UTC milliseconds since Unix epoch (injectable for tests).
pub type UnixMs = i64;

/// Non-negative duration in milliseconds.
pub type DurationMs = u64;

/// Default focus length (provisional until human re-approves defaults).
pub const DEFAULT_FOCUS_MS: DurationMs = 25 * 60 * 1000;
/// Default short break.
pub const DEFAULT_SHORT_BREAK_MS: DurationMs = 5 * 60 * 1000;
/// Default long break.
pub const DEFAULT_LONG_BREAK_MS: DurationMs = 15 * 60 * 1000;
/// Focus completions before a long break.
pub const DEFAULT_FOCUSES_BEFORE_LONG_BREAK: u32 = 4;

/// Durations and policy for a Pomodoro session cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PomodoroConfig {
    pub focus_ms: DurationMs,
    pub short_break_ms: DurationMs,
    pub long_break_ms: DurationMs,
    /// After this many *completed* focus phases, the next break is long.
    pub focuses_before_long_break: u32,
    /// When true, automatically start the next phase after a natural completion.
    pub auto_start_next: bool,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            focus_ms: DEFAULT_FOCUS_MS,
            short_break_ms: DEFAULT_SHORT_BREAK_MS,
            long_break_ms: DEFAULT_LONG_BREAK_MS,
            focuses_before_long_break: DEFAULT_FOCUSES_BEFORE_LONG_BREAK,
            auto_start_next: false,
        }
    }
}

impl PomodoroConfig {
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.focus_ms == 0 || self.short_break_ms == 0 || self.long_break_ms == 0 {
            return Err(CoreError::InvalidPomodoro(
                "phase durations must be non-zero",
            ));
        }
        if self.focuses_before_long_break == 0 {
            return Err(CoreError::InvalidPomodoro(
                "focuses_before_long_break must be >= 1",
            ));
        }
        Ok(())
    }

    fn duration_for(&self, phase: Phase) -> DurationMs {
        match phase {
            Phase::Focus => self.focus_ms,
            Phase::ShortBreak => self.short_break_ms,
            Phase::LongBreak => self.long_break_ms,
        }
    }
}

/// Active work/rest phase (never Idle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Focus => "Focus",
            Phase::ShortBreak => "Short break",
            Phase::LongBreak => "Long break",
        }
    }
}

/// Running / paused / idle timer status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TimerStatus {
    Idle,
    Running {
        phase: Phase,
        deadline_utc_ms: UnixMs,
    },
    Paused {
        phase: Phase,
        remaining_ms: DurationMs,
    },
}

/// Durable Pomodoro machine state (persistence shape for Alpha 1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PomodoroState {
    pub config: PomodoroConfig,
    pub status: TimerStatus,
    /// Completed focus phases in the current long-break cycle (not including skips).
    pub completed_focuses_in_cycle: u32,
    pub last_transition_utc_ms: Option<UnixMs>,
    /// Monotonic identity of the last emitted phase-completion (dedupe notifications).
    pub last_completion_id: Option<u64>,
    completion_seq: u64,
}

impl PomodoroState {
    pub fn new(config: PomodoroConfig) -> Result<Self, CoreError> {
        config.validate()?;
        Ok(Self {
            config,
            status: TimerStatus::Idle,
            completed_focuses_in_cycle: 0,
            last_transition_utc_ms: None,
            last_completion_id: None,
            completion_seq: 0,
        })
    }

    pub fn idle_default() -> Self {
        Self::new(PomodoroConfig::default()).expect("default config is valid")
    }

    /// User/runtime commands. `now_utc_ms` is required for time-sensitive ops.
    pub fn apply(
        &mut self,
        command: Command,
        now_utc_ms: UnixMs,
    ) -> Result<Vec<PomodoroEvent>, CoreError> {
        match command {
            Command::Start => self.cmd_start(now_utc_ms),
            Command::Pause => self.cmd_pause(now_utc_ms),
            Command::Resume => self.cmd_resume(now_utc_ms),
            Command::Skip => self.cmd_skip(now_utc_ms),
            Command::Reset => self.cmd_reset(now_utc_ms),
            Command::Sync => self.cmd_sync(now_utc_ms),
        }
    }

    /// Widget-facing snapshot at `now_utc_ms`.
    pub fn view(&self, now_utc_ms: UnixMs) -> PomodoroView {
        let (phase, remaining_ms, total_ms, running) = match &self.status {
            TimerStatus::Idle => (None, 0, 0, false),
            TimerStatus::Running {
                phase,
                deadline_utc_ms,
            } => {
                let total = self.config.duration_for(*phase);
                let remaining = remaining_until(*deadline_utc_ms, now_utc_ms, total);
                (Some(*phase), remaining, total, true)
            }
            TimerStatus::Paused {
                phase,
                remaining_ms,
            } => {
                let total = self.config.duration_for(*phase);
                (Some(*phase), *remaining_ms, total, false)
            }
        };

        let progress = if total_ms == 0 {
            0.0
        } else {
            let done = total_ms.saturating_sub(remaining_ms) as f32;
            (done / total_ms as f32).clamp(0.0, 1.0)
        };

        PomodoroView {
            phase,
            phase_label: phase.map(Phase::label).unwrap_or("Idle"),
            remaining_ms,
            progress_0_1: progress,
            is_running: running,
            is_paused: matches!(self.status, TimerStatus::Paused { .. }),
            is_idle: matches!(self.status, TimerStatus::Idle),
            completed_focuses_in_cycle: self.completed_focuses_in_cycle,
            available: AvailableActions::from_status(&self.status),
        }
    }

    fn cmd_start(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        match self.status {
            TimerStatus::Idle => {
                let phase = Phase::Focus;
                self.begin_phase(phase, now);
                Ok(vec![PomodoroEvent::Started { phase }])
            }
            _ => Err(CoreError::IllegalPomodoroTransition(
                "Start is only valid from Idle (use Resume when paused)",
            )),
        }
    }

    fn cmd_pause(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        match self.status {
            TimerStatus::Running {
                phase,
                deadline_utc_ms,
            } => {
                let total = self.config.duration_for(phase);
                let remaining = remaining_until(deadline_utc_ms, now, total);
                self.status = TimerStatus::Paused {
                    phase,
                    remaining_ms: remaining,
                };
                self.last_transition_utc_ms = Some(now);
                Ok(vec![PomodoroEvent::Paused { phase }])
            }
            _ => Err(CoreError::IllegalPomodoroTransition(
                "Pause is only valid while Running",
            )),
        }
    }

    fn cmd_resume(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        match self.status {
            TimerStatus::Paused {
                phase,
                remaining_ms,
            } => {
                if remaining_ms == 0 {
                    // Degenerate pause at zero: treat as completion via Sync path.
                    self.status = TimerStatus::Running {
                        phase,
                        deadline_utc_ms: now,
                    };
                    return self.cmd_sync(now);
                }
                self.status = TimerStatus::Running {
                    phase,
                    deadline_utc_ms: now.saturating_add(remaining_ms as i64),
                };
                self.last_transition_utc_ms = Some(now);
                Ok(vec![PomodoroEvent::Resumed { phase }])
            }
            _ => Err(CoreError::IllegalPomodoroTransition(
                "Resume is only valid while Paused",
            )),
        }
    }

    fn cmd_skip(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        let phase = match self.status {
            TimerStatus::Running { phase, .. } | TimerStatus::Paused { phase, .. } => phase,
            TimerStatus::Idle => {
                return Err(CoreError::IllegalPomodoroTransition(
                    "Skip is not valid while Idle",
                ));
            }
        };
        // Skip does not count a focus completion.
        let next = self.next_phase_after(phase, /*focus_completed*/ false);
        let mut events = vec![PomodoroEvent::Skipped { phase }];
        if self.config.auto_start_next {
            self.begin_phase(next, now);
            events.push(PomodoroEvent::NextPhaseStarted { phase: next });
        } else {
            self.status = TimerStatus::Idle;
            self.last_transition_utc_ms = Some(now);
        }
        Ok(events)
    }

    fn cmd_reset(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        self.status = TimerStatus::Idle;
        self.completed_focuses_in_cycle = 0;
        self.last_transition_utc_ms = Some(now);
        // Keep last_completion_id so notification dedupe survives reset storms.
        Ok(vec![PomodoroEvent::Reset])
    }

    /// Recovery / deadline check. Completes at most one expired phase; never replays a backlog.
    fn cmd_sync(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        let TimerStatus::Running {
            phase,
            deadline_utc_ms,
        } = self.status
        else {
            return Ok(vec![]);
        };

        if now < deadline_utc_ms {
            return Ok(vec![]);
        }

        // Expired: complete exactly one phase.
        let focus_completed = phase == Phase::Focus;
        if focus_completed {
            self.completed_focuses_in_cycle = self.completed_focuses_in_cycle.saturating_add(1);
        }

        self.completion_seq = self.completion_seq.saturating_add(1);
        let completion_id = self.completion_seq;
        self.last_completion_id = Some(completion_id);
        self.last_transition_utc_ms = Some(now);

        let mut events = vec![PomodoroEvent::PhaseCompleted {
            phase,
            completion_id,
        }];

        let next = self.next_phase_after(phase, focus_completed);
        if focus_completed
            && self.completed_focuses_in_cycle >= self.config.focuses_before_long_break
            && next == Phase::LongBreak
        {
            // Cycle rolls when entering long break after N focuses.
            // Count is cleared when long break *completes* or is skipped after long break start.
        }

        if self.config.auto_start_next {
            // After completing enough focuses, next is LongBreak; after long break completes, reset cycle.
            if phase == Phase::LongBreak {
                self.completed_focuses_in_cycle = 0;
            }
            self.begin_phase(next, now);
            events.push(PomodoroEvent::NextPhaseStarted { phase: next });
        } else {
            if phase == Phase::LongBreak {
                self.completed_focuses_in_cycle = 0;
            }
            self.status = TimerStatus::Idle;
        }

        Ok(events)
    }

    fn begin_phase(&mut self, phase: Phase, now: UnixMs) {
        let dur = self.config.duration_for(phase);
        self.status = TimerStatus::Running {
            phase,
            deadline_utc_ms: now.saturating_add(dur as i64),
        };
        self.last_transition_utc_ms = Some(now);
    }

    fn next_phase_after(&self, completed_or_skipped: Phase, focus_completed: bool) -> Phase {
        match completed_or_skipped {
            Phase::Focus => {
                let count = if focus_completed {
                    self.completed_focuses_in_cycle
                } else {
                    // Skip: look at current cycle without increment.
                    self.completed_focuses_in_cycle
                };
                // After a completed focus, count already incremented in cmd_sync.
                // For skip, use current count; long break only after completed focuses.
                if focus_completed
                    && count > 0
                    && count % self.config.focuses_before_long_break == 0
                {
                    Phase::LongBreak
                } else {
                    Phase::ShortBreak
                }
            }
            Phase::ShortBreak | Phase::LongBreak => Phase::Focus,
        }
    }
}

/// Commands accepted by [`PomodoroState::apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Start,
    Pause,
    Resume,
    Skip,
    Reset,
    /// Compare wall clock to deadline; complete at most one expired phase.
    Sync,
}

/// Domain events for UI / notification wiring (not persisted as a log in Alpha 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PomodoroEvent {
    Started { phase: Phase },
    Paused { phase: Phase },
    Resumed { phase: Phase },
    Skipped { phase: Phase },
    Reset,
    PhaseCompleted { phase: Phase, completion_id: u64 },
    NextPhaseStarted { phase: Phase },
}

/// Actions the widget may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailableActions {
    pub start: bool,
    pub pause: bool,
    pub resume: bool,
    pub skip: bool,
    pub reset: bool,
}

impl AvailableActions {
    fn from_status(status: &TimerStatus) -> Self {
        match status {
            TimerStatus::Idle => Self {
                start: true,
                pause: false,
                resume: false,
                skip: false,
                reset: true,
            },
            TimerStatus::Running { .. } => Self {
                start: false,
                pause: true,
                resume: false,
                skip: true,
                reset: true,
            },
            TimerStatus::Paused { .. } => Self {
                start: false,
                pause: false,
                resume: true,
                skip: true,
                reset: true,
            },
        }
    }
}

/// Widget-facing view model (Issue #19 deliverable).
#[derive(Debug, Clone, PartialEq)]
pub struct PomodoroView {
    pub phase: Option<Phase>,
    pub phase_label: &'static str,
    pub remaining_ms: DurationMs,
    pub progress_0_1: f32,
    pub is_running: bool,
    pub is_paused: bool,
    pub is_idle: bool,
    pub completed_focuses_in_cycle: u32,
    pub available: AvailableActions,
}

fn remaining_until(deadline_utc_ms: UnixMs, now_utc_ms: UnixMs, total: DurationMs) -> DurationMs {
    if now_utc_ms >= deadline_utc_ms {
        return 0;
    }
    let left = (deadline_utc_ms - now_utc_ms) as DurationMs;
    left.min(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> PomodoroState {
        // Short durations for readable tests.
        let cfg = PomodoroConfig {
            focus_ms: 1_000,
            short_break_ms: 500,
            long_break_ms: 800,
            focuses_before_long_break: 2,
            auto_start_next: false,
        };
        PomodoroState::new(cfg).unwrap()
    }

    #[test]
    fn start_from_idle_begins_focus() {
        let mut s = state();
        let ev = s.apply(Command::Start, 10_000).unwrap();
        assert_eq!(
            ev,
            vec![PomodoroEvent::Started {
                phase: Phase::Focus
            }]
        );
        assert!(matches!(
            s.status,
            TimerStatus::Running {
                phase: Phase::Focus,
                deadline_utc_ms: 11_000
            }
        ));
    }

    #[test]
    fn start_while_running_is_illegal() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        assert!(s.apply(Command::Start, 1).is_err());
    }

    #[test]
    fn pause_and_resume_preserve_remaining() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Pause, 400).unwrap();
        match s.status {
            TimerStatus::Paused {
                phase: Phase::Focus,
                remaining_ms,
            } => assert_eq!(remaining_ms, 600),
            other => panic!("expected paused, got {other:?}"),
        }
        s.apply(Command::Resume, 1_000).unwrap();
        match s.status {
            TimerStatus::Running {
                phase: Phase::Focus,
                deadline_utc_ms,
            } => assert_eq!(deadline_utc_ms, 1_600),
            other => panic!("expected running, got {other:?}"),
        }
    }

    #[test]
    fn pause_when_idle_is_illegal() {
        let mut s = state();
        assert!(s.apply(Command::Pause, 0).is_err());
    }

    #[test]
    fn resume_when_running_is_illegal() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        assert!(s.apply(Command::Resume, 1).is_err());
    }

    #[test]
    fn sync_before_deadline_is_noop() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::Sync, 999).unwrap();
        assert!(ev.is_empty());
        assert!(matches!(s.status, TimerStatus::Running { .. }));
    }

    #[test]
    fn sync_after_deadline_completes_one_phase_no_autostart() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::Sync, 1_000).unwrap();
        assert_eq!(
            ev,
            vec![PomodoroEvent::PhaseCompleted {
                phase: Phase::Focus,
                completion_id: 1
            }]
        );
        assert!(matches!(s.status, TimerStatus::Idle));
        assert_eq!(s.completed_focuses_in_cycle, 1);
        assert_eq!(s.last_completion_id, Some(1));
    }

    #[test]
    fn large_time_jump_still_completes_only_one_phase() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        // Far past several theoretical deadlines — still one completion.
        let ev = s.apply(Command::Sync, 1_000_000).unwrap();
        assert_eq!(ev.len(), 1);
        assert!(matches!(
            ev[0],
            PomodoroEvent::PhaseCompleted {
                phase: Phase::Focus,
                ..
            }
        ));
        assert!(matches!(s.status, TimerStatus::Idle));
        assert_eq!(s.completed_focuses_in_cycle, 1);
    }

    #[test]
    fn auto_start_next_after_completion() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::Sync, 1_000).unwrap();
        assert_eq!(
            ev,
            vec![
                PomodoroEvent::PhaseCompleted {
                    phase: Phase::Focus,
                    completion_id: 1
                },
                PomodoroEvent::NextPhaseStarted {
                    phase: Phase::ShortBreak
                }
            ]
        );
        assert!(matches!(
            s.status,
            TimerStatus::Running {
                phase: Phase::ShortBreak,
                ..
            }
        ));
    }

    #[test]
    fn long_break_after_configured_focus_completions() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.config.focuses_before_long_break = 2;
        // Focus 1
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Sync, 1_000).unwrap();
        // Short break → focus 2
        s.apply(Command::Sync, 1_500).unwrap();
        // Focus 2 completes → long break
        let ev = s.apply(Command::Sync, 2_500).unwrap();
        assert!(ev.iter().any(|e| matches!(
            e,
            PomodoroEvent::NextPhaseStarted {
                phase: Phase::LongBreak
            }
        )));
    }

    #[test]
    fn skip_does_not_increment_focus_count() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Skip, 100).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 0);
        assert!(matches!(s.status, TimerStatus::Idle));
    }

    #[test]
    fn skip_from_idle_is_illegal() {
        let mut s = state();
        assert!(s.apply(Command::Skip, 0).is_err());
    }

    #[test]
    fn reset_clears_cycle_and_idles() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Sync, 1_000).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 1);
        s.apply(Command::Reset, 2_000).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 0);
        assert!(matches!(s.status, TimerStatus::Idle));
    }

    #[test]
    fn view_remaining_and_actions_while_running() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let v = s.view(250);
        assert_eq!(v.remaining_ms, 750);
        assert!(v.available.pause);
        assert!(!v.available.start);
        assert!((v.progress_0_1 - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn countdown_does_not_depend_on_one_second_ticks() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        // Single large step to near end.
        let v = s.view(999);
        assert_eq!(v.remaining_ms, 1);
        let ev = s.apply(Command::Sync, 1_000).unwrap();
        assert_eq!(ev.len(), 1);
    }

    #[test]
    fn rejects_zero_duration_config() {
        let cfg = PomodoroConfig {
            focus_ms: 0,
            ..PomodoroConfig::default()
        };
        assert!(PomodoroState::new(cfg).is_err());
    }

    #[test]
    fn serde_roundtrip_snapshot() {
        let mut s = state();
        s.apply(Command::Start, 42).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let back: PomodoroState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
    }
}
