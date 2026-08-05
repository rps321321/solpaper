//! Platform-neutral Pomodoro state machine (Issue #19).
//!
//! Pure logic: injectable wall-clock (`UnixMs`), no threads, no Win32.
//! Design note: `docs/design/pomodoro-state-machine.md`.
//! Defaults follow `docs/engineering/deterministic-execution-blueprint.md` pack #19.

use serde::{Deserialize, Serialize};

use crate::CoreError;

/// UTC milliseconds since Unix epoch (injectable for tests).
pub type UnixMs = i64;

/// Non-negative duration in milliseconds.
pub type DurationMs = u64;

/// Default focus length (blueprint #19).
pub const DEFAULT_FOCUS_MS: DurationMs = 25 * 60 * 1000;
/// Default short break.
pub const DEFAULT_SHORT_BREAK_MS: DurationMs = 5 * 60 * 1000;
/// Default long break.
pub const DEFAULT_LONG_BREAK_MS: DurationMs = 15 * 60 * 1000;
/// Focus completions before a long break.
pub const DEFAULT_FOCUSES_BEFORE_LONG_BREAK: u32 = 4;

const MIN_FOCUS_MS: DurationMs = 60_000;
const MAX_FOCUS_MS: DurationMs = 180 * 60 * 1000;
const MIN_SHORT_BREAK_MS: DurationMs = 60_000;
const MAX_SHORT_BREAK_MS: DurationMs = 60 * 60 * 1000;
const MIN_LONG_BREAK_MS: DurationMs = 60_000;
const MAX_LONG_BREAK_MS: DurationMs = 120 * 60 * 1000;
const MIN_CADENCE: u32 = 2;
const MAX_CADENCE: u32 = 12;

/// Durations and policy for a Pomodoro session cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PomodoroConfig {
    pub focus_ms: DurationMs,
    pub short_break_ms: DurationMs,
    pub long_break_ms: DurationMs,
    /// After this many *completed* focus phases, the next break is long.
    pub focuses_before_long_break: u32,
    /// When true, automatically start the next phase after a *live* completion or skip.
    /// Recovery [`Command::Sync`] never auto-starts, even when this is true.
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
        if !(MIN_FOCUS_MS..=MAX_FOCUS_MS).contains(&self.focus_ms) {
            return Err(CoreError::InvalidPomodoro(
                "focus duration must be 1–180 minutes",
            ));
        }
        if !(MIN_SHORT_BREAK_MS..=MAX_SHORT_BREAK_MS).contains(&self.short_break_ms) {
            return Err(CoreError::InvalidPomodoro(
                "short break duration must be 1–60 minutes",
            ));
        }
        if !(MIN_LONG_BREAK_MS..=MAX_LONG_BREAK_MS).contains(&self.long_break_ms) {
            return Err(CoreError::InvalidPomodoro(
                "long break duration must be 1–120 minutes",
            ));
        }
        if !(MIN_CADENCE..=MAX_CADENCE).contains(&self.focuses_before_long_break) {
            return Err(CoreError::InvalidPomodoro(
                "focuses_before_long_break must be 2–12",
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
        phase_instance_id: u64,
    },
    Paused {
        phase: Phase,
        remaining_ms: DurationMs,
        phase_instance_id: u64,
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
    /// Identity of the last emitted phase-completion (notification dedupe).
    pub last_completion_id: Option<u64>,
    /// Next phase-instance / completion identity source (serde-visible for round-trip).
    phase_seq: u64,
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
            phase_seq: 0,
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
            // Recovery-safe: never auto-start after expiry.
            Command::Sync => self.cmd_complete_if_due(now_utc_ms, /*allow_auto_start*/ false),
            // Live tick while the process is continuously running; may auto-start.
            Command::LiveTick => {
                self.cmd_complete_if_due(now_utc_ms, /*allow_auto_start*/ true)
            }
        }
    }

    /// Widget-facing snapshot at `now_utc_ms`.
    pub fn view(&self, now_utc_ms: UnixMs) -> PomodoroView {
        let (phase, remaining_ms, total_ms, running) = match &self.status {
            TimerStatus::Idle => (None, 0, 0, false),
            TimerStatus::Running {
                phase,
                deadline_utc_ms,
                ..
            } => {
                let total = self.config.duration_for(*phase);
                let remaining = remaining_until(*deadline_utc_ms, now_utc_ms, total);
                (Some(*phase), remaining, total, true)
            }
            TimerStatus::Paused {
                phase,
                remaining_ms,
                ..
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
                let id = self.begin_phase(phase, now);
                Ok(vec![PomodoroEvent::Started {
                    phase,
                    phase_instance_id: id,
                }])
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
                phase_instance_id,
            } => {
                let total = self.config.duration_for(phase);
                let remaining = remaining_until(deadline_utc_ms, now, total);
                self.status = TimerStatus::Paused {
                    phase,
                    remaining_ms: remaining,
                    phase_instance_id,
                };
                self.last_transition_utc_ms = Some(now);
                Ok(vec![PomodoroEvent::Paused {
                    phase,
                    phase_instance_id,
                }])
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
                phase_instance_id,
            } => {
                if remaining_ms == 0 {
                    // Degenerate pause at zero: treat as live completion.
                    self.status = TimerStatus::Running {
                        phase,
                        deadline_utc_ms: now,
                        phase_instance_id,
                    };
                    return self.cmd_complete_if_due(now, /*allow_auto_start*/ true);
                }
                let deadline = saturating_deadline(now, remaining_ms);
                self.status = TimerStatus::Running {
                    phase,
                    deadline_utc_ms: deadline,
                    phase_instance_id,
                };
                self.last_transition_utc_ms = Some(now);
                Ok(vec![PomodoroEvent::Resumed {
                    phase,
                    phase_instance_id,
                }])
            }
            _ => Err(CoreError::IllegalPomodoroTransition(
                "Resume is only valid while Paused",
            )),
        }
    }

    fn cmd_skip(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        let (phase, phase_instance_id) = match self.status {
            TimerStatus::Running {
                phase,
                phase_instance_id,
                ..
            }
            | TimerStatus::Paused {
                phase,
                phase_instance_id,
                ..
            } => (phase, phase_instance_id),
            TimerStatus::Idle => {
                return Err(CoreError::IllegalPomodoroTransition(
                    "Skip is not valid while Idle",
                ));
            }
        };
        // Skip does not count a focus completion.
        let next = self.next_phase_after(phase, /*focus_completed*/ false);
        let mut events = vec![PomodoroEvent::Skipped {
            phase,
            phase_instance_id,
        }];
        // Live skip: auto-start may apply.
        if self.config.auto_start_next {
            let id = self.begin_phase(next, now);
            events.push(PomodoroEvent::NextPhaseStarted {
                phase: next,
                phase_instance_id: id,
            });
        } else {
            self.status = TimerStatus::Idle;
            self.last_transition_utc_ms = Some(now);
        }
        Ok(events)
    }

    fn cmd_reset(&mut self, now: UnixMs) -> Result<Vec<PomodoroEvent>, CoreError> {
        // Blueprint: Idle at full configured duration; preserve completed-focus count.
        self.status = TimerStatus::Idle;
        self.last_transition_utc_ms = Some(now);
        Ok(vec![PomodoroEvent::Reset])
    }

    /// Complete at most one expired running phase. Never replays a multi-phase backlog.
    ///
    /// When `allow_auto_start` is false (recovery [`Command::Sync`]), the next phase is
    /// always left Idle even if `auto_start_next` is configured.
    fn cmd_complete_if_due(
        &mut self,
        now: UnixMs,
        allow_auto_start: bool,
    ) -> Result<Vec<PomodoroEvent>, CoreError> {
        let TimerStatus::Running {
            phase,
            deadline_utc_ms,
            phase_instance_id,
        } = self.status
        else {
            return Ok(vec![]);
        };

        if now < deadline_utc_ms {
            return Ok(vec![]);
        }

        let focus_completed = phase == Phase::Focus;
        if focus_completed {
            self.completed_focuses_in_cycle = self.completed_focuses_in_cycle.saturating_add(1);
        }

        // Completion identity for notification dedupe (stable across repeated Sync).
        let completion_id = phase_instance_id;
        self.last_completion_id = Some(completion_id);
        self.last_transition_utc_ms = Some(now);

        let mut events = vec![PomodoroEvent::PhaseCompleted {
            phase,
            completion_id,
            phase_instance_id,
        }];

        let next = self.next_phase_after(phase, focus_completed);

        if phase == Phase::LongBreak {
            self.completed_focuses_in_cycle = 0;
        }

        if allow_auto_start && self.config.auto_start_next {
            let id = self.begin_phase(next, now);
            events.push(PomodoroEvent::NextPhaseStarted {
                phase: next,
                phase_instance_id: id,
            });
        } else {
            self.status = TimerStatus::Idle;
        }

        Ok(events)
    }

    fn begin_phase(&mut self, phase: Phase, now: UnixMs) -> u64 {
        self.phase_seq = self.phase_seq.saturating_add(1);
        let phase_instance_id = self.phase_seq;
        let dur = self.config.duration_for(phase);
        self.status = TimerStatus::Running {
            phase,
            deadline_utc_ms: saturating_deadline(now, dur),
            phase_instance_id,
        };
        self.last_transition_utc_ms = Some(now);
        phase_instance_id
    }

    fn next_phase_after(&self, completed_or_skipped: Phase, focus_completed: bool) -> Phase {
        match completed_or_skipped {
            Phase::Focus => {
                let count = self.completed_focuses_in_cycle;
                // Long break only after completed focuses; skip never credits.
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
    /// Recovery / restore path: complete at most one expired phase; **never** auto-start next.
    Sync,
    /// Live deadline check while the process is continuously running; may auto-start next.
    LiveTick,
}

/// Domain events for UI / notification wiring (not persisted as a log in Alpha 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PomodoroEvent {
    Started {
        phase: Phase,
        phase_instance_id: u64,
    },
    Paused {
        phase: Phase,
        phase_instance_id: u64,
    },
    Resumed {
        phase: Phase,
        phase_instance_id: u64,
    },
    Skipped {
        phase: Phase,
        phase_instance_id: u64,
    },
    Reset,
    PhaseCompleted {
        phase: Phase,
        completion_id: u64,
        phase_instance_id: u64,
    },
    NextPhaseStarted {
        phase: Phase,
        phase_instance_id: u64,
    },
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
    // Clock moved backward: remaining may exceed total; clamp to configured phase length.
    let left = (deadline_utc_ms - now_utc_ms) as DurationMs;
    left.min(total)
}

fn saturating_deadline(now: UnixMs, duration_ms: DurationMs) -> UnixMs {
    let add = i64::try_from(duration_ms).unwrap_or(i64::MAX);
    now.saturating_add(add)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Short durations for readable tests (bypass production range validation).
    fn test_config() -> PomodoroConfig {
        PomodoroConfig {
            focus_ms: 1_000,
            short_break_ms: 500,
            long_break_ms: 800,
            focuses_before_long_break: 2,
            auto_start_next: false,
        }
    }

    fn state() -> PomodoroState {
        // Construct without full production ranges for fast unit tests.
        PomodoroState {
            config: test_config(),
            status: TimerStatus::Idle,
            completed_focuses_in_cycle: 0,
            last_transition_utc_ms: None,
            last_completion_id: None,
            phase_seq: 0,
        }
    }

    #[test]
    fn default_config_matches_blueprint_minutes() {
        let c = PomodoroConfig::default();
        assert_eq!(c.focus_ms, 25 * 60 * 1000);
        assert_eq!(c.short_break_ms, 5 * 60 * 1000);
        assert_eq!(c.long_break_ms, 15 * 60 * 1000);
        assert_eq!(c.focuses_before_long_break, 4);
        assert!(!c.auto_start_next);
        PomodoroState::new(c).unwrap();
    }

    #[test]
    fn start_from_idle_begins_focus() {
        let mut s = state();
        let ev = s.apply(Command::Start, 10_000).unwrap();
        assert!(matches!(
            &ev[..],
            [PomodoroEvent::Started {
                phase: Phase::Focus,
                phase_instance_id: 1
            }]
        ));
        assert!(matches!(
            s.status,
            TimerStatus::Running {
                phase: Phase::Focus,
                deadline_utc_ms: 11_000,
                phase_instance_id: 1
            }
        ));
    }

    #[test]
    fn illegal_commands_from_incompatible_states() {
        let mut s = state();
        assert!(s.apply(Command::Pause, 0).is_err());
        assert!(s.apply(Command::Resume, 0).is_err());
        assert!(s.apply(Command::Skip, 0).is_err());

        s.apply(Command::Start, 0).unwrap();
        assert!(s.apply(Command::Start, 1).is_err());
        assert!(s.apply(Command::Resume, 1).is_err());

        s.apply(Command::Pause, 100).unwrap();
        assert!(s.apply(Command::Pause, 101).is_err());
        assert!(s.apply(Command::Start, 101).is_err());
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
                phase_instance_id: 1,
            } => assert_eq!(remaining_ms, 600),
            other => panic!("expected paused, got {other:?}"),
        }
        s.apply(Command::Resume, 1_000).unwrap();
        match s.status {
            TimerStatus::Running {
                phase: Phase::Focus,
                deadline_utc_ms,
                phase_instance_id: 1,
            } => assert_eq!(deadline_utc_ms, 1_600),
            other => panic!("expected running, got {other:?}"),
        }
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
    fn natural_focus_completion_via_live_tick() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::LiveTick, 1_000).unwrap();
        assert_eq!(
            ev,
            vec![PomodoroEvent::PhaseCompleted {
                phase: Phase::Focus,
                completion_id: 1,
                phase_instance_id: 1,
            }]
        );
        assert!(matches!(s.status, TimerStatus::Idle));
        assert_eq!(s.completed_focuses_in_cycle, 1);
        assert_eq!(s.last_completion_id, Some(1));
    }

    #[test]
    fn recovery_sync_never_auto_starts() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::Sync, 1_000).unwrap();
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
    fn large_time_jump_completes_only_one_phase() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::Sync, 1_000_000).unwrap();
        assert_eq!(ev.len(), 1);
        assert!(matches!(s.status, TimerStatus::Idle));
        assert_eq!(s.completed_focuses_in_cycle, 1);
        // Repeated Sync after completion produces no duplicate completion.
        let ev2 = s.apply(Command::Sync, 2_000_000).unwrap();
        assert!(ev2.is_empty());
        assert_eq!(s.last_completion_id, Some(1));
    }

    #[test]
    fn live_tick_auto_start_next_after_completion() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.apply(Command::Start, 0).unwrap();
        let ev = s.apply(Command::LiveTick, 1_000).unwrap();
        assert_eq!(
            ev,
            vec![
                PomodoroEvent::PhaseCompleted {
                    phase: Phase::Focus,
                    completion_id: 1,
                    phase_instance_id: 1,
                },
                PomodoroEvent::NextPhaseStarted {
                    phase: Phase::ShortBreak,
                    phase_instance_id: 2,
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
    fn natural_short_break_completion() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::LiveTick, 1_000).unwrap(); // → short break running
        let ev = s.apply(Command::LiveTick, 1_500).unwrap();
        assert!(ev.iter().any(|e| matches!(
            e,
            PomodoroEvent::PhaseCompleted {
                phase: Phase::ShortBreak,
                ..
            }
        )));
        assert!(ev.iter().any(|e| matches!(
            e,
            PomodoroEvent::NextPhaseStarted {
                phase: Phase::Focus,
                ..
            }
        )));
    }

    #[test]
    fn fourth_style_long_break_after_configured_focus_completions() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.config.focuses_before_long_break = 2;
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::LiveTick, 1_000).unwrap(); // focus1 done → short
        s.apply(Command::LiveTick, 1_500).unwrap(); // short done → focus2
        let ev = s.apply(Command::LiveTick, 2_500).unwrap(); // focus2 done → long
        assert!(ev.iter().any(|e| matches!(
            e,
            PomodoroEvent::NextPhaseStarted {
                phase: Phase::LongBreak,
                ..
            }
        )));
        assert_eq!(s.completed_focuses_in_cycle, 2);
    }

    #[test]
    fn skip_focus_does_not_credit() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Skip, 100).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 0);
        assert!(matches!(s.status, TimerStatus::Idle));
    }

    #[test]
    fn skip_break_returns_toward_focus_when_auto() {
        let mut s = state();
        s.config.auto_start_next = true;
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::LiveTick, 1_000).unwrap(); // short break
        s.apply(Command::Skip, 1_100).unwrap();
        assert!(matches!(
            s.status,
            TimerStatus::Running {
                phase: Phase::Focus,
                ..
            }
        ));
    }

    #[test]
    fn reset_from_running_and_paused_preserves_focus_count() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Sync, 1_000).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 1);

        s.apply(Command::Start, 2_000).unwrap();
        s.apply(Command::Reset, 2_100).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 1);
        assert!(matches!(s.status, TimerStatus::Idle));

        s.apply(Command::Start, 3_000).unwrap();
        s.apply(Command::Pause, 3_100).unwrap();
        s.apply(Command::Reset, 3_200).unwrap();
        assert_eq!(s.completed_focuses_in_cycle, 1);
        assert!(matches!(s.status, TimerStatus::Idle));
    }

    #[test]
    fn restart_before_deadline_continues_same_phase() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let mut back: PomodoroState = serde_json::from_str(&json).unwrap();
        let v = back.view(500);
        assert_eq!(v.remaining_ms, 500);
        assert!(matches!(
            back.status,
            TimerStatus::Running {
                phase: Phase::Focus,
                deadline_utc_ms: 1_000,
                ..
            }
        ));
        assert!(back.apply(Command::Sync, 500).unwrap().is_empty());
    }

    #[test]
    fn restart_after_deadline_completes_once_via_sync() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let mut back: PomodoroState = serde_json::from_str(&json).unwrap();
        let ev = back.apply(Command::Sync, 5_000).unwrap();
        assert_eq!(ev.len(), 1);
        assert!(matches!(back.status, TimerStatus::Idle));
    }

    #[test]
    fn clock_backward_clamps_remaining_to_phase_total() {
        let mut s = state();
        s.apply(Command::Start, 10_000).unwrap();
        // now before start: remaining would exceed total without clamp
        let v = s.view(0);
        assert_eq!(v.remaining_ms, 1_000);
    }

    #[test]
    fn resume_with_zero_remaining_completes() {
        let mut s = state();
        s.apply(Command::Start, 0).unwrap();
        s.apply(Command::Pause, 1_000).unwrap(); // remaining 0
        match s.status {
            TimerStatus::Paused { remaining_ms, .. } => assert_eq!(remaining_ms, 0),
            other => panic!("{other:?}"),
        }
        let ev = s.apply(Command::Resume, 2_000).unwrap();
        assert!(ev.iter().any(|e| matches!(
            e,
            PomodoroEvent::PhaseCompleted {
                phase: Phase::Focus,
                ..
            }
        )));
    }

    #[test]
    fn serde_roundtrip_idle_running_paused() {
        let idle = state();
        let j = serde_json::to_string(&idle).unwrap();
        let back: PomodoroState = serde_json::from_str(&j).unwrap();
        assert_eq!(back, idle);

        let mut running = state();
        running.apply(Command::Start, 42).unwrap();
        let j = serde_json::to_string(&running).unwrap();
        let back: PomodoroState = serde_json::from_str(&j).unwrap();
        assert_eq!(back, running);
        assert_eq!(back.phase_seq, 1);
        assert_eq!(back.last_completion_id, None);

        let mut paused = state();
        paused.apply(Command::Start, 0).unwrap();
        paused.apply(Command::Pause, 250).unwrap();
        let j = serde_json::to_string(&paused).unwrap();
        let back: PomodoroState = serde_json::from_str(&j).unwrap();
        assert_eq!(back, paused);
        match back.status {
            TimerStatus::Paused {
                remaining_ms,
                phase_instance_id,
                ..
            } => {
                assert_eq!(remaining_ms, 750);
                assert_eq!(phase_instance_id, 1);
            }
            other => panic!("{other:?}"),
        }
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
    fn production_config_range_validation() {
        assert!(PomodoroState::new(PomodoroConfig {
            focus_ms: 0,
            ..PomodoroConfig::default()
        })
        .is_err());
        assert!(PomodoroState::new(PomodoroConfig {
            focus_ms: 200 * 60 * 1000,
            ..PomodoroConfig::default()
        })
        .is_err());
        assert!(PomodoroState::new(PomodoroConfig {
            focuses_before_long_break: 1,
            ..PomodoroConfig::default()
        })
        .is_err());
        assert!(PomodoroState::new(PomodoroConfig::default()).is_ok());
    }

    #[test]
    fn deadline_arithmetic_near_i64_edge() {
        let mut s = state();
        // near max: begin_phase uses saturating_add
        s.apply(Command::Start, i64::MAX - 500).unwrap();
        match s.status {
            TimerStatus::Running {
                deadline_utc_ms, ..
            } => {
                assert_eq!(deadline_utc_ms, i64::MAX);
            }
            other => panic!("{other:?}"),
        }
    }
}
