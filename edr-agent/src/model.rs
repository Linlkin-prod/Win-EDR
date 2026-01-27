use std::sync::mpsc;

pub struct EventPipeline {
    pub tx: mpsc::Sender<Event>,
    pub rx: mpsc::Receiver<Event>,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp: u64,
    pub event_type: EventType,
    pub host_id: Option<HostId>,
    pub process: ProcessContext,
    pub data: EventData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    ProcessStart,
    ProcessStop,
    ImageLoad,
}

#[derive(Debug, Clone)]
pub struct HostId {
    pub agent_id: String,
    pub hostname: String,
}

#[derive(Debug, Clone)]
pub struct ProcessContext {
    pub pid: u32,
    pub ppid: Option<u32>,

    pub image: Option<String>,           //winword.exe
    pub image_path: Option<String>,      // normalized full path
    pub image_path_raw: Option<String>,  // raw NT path
    pub cmdline: Option<String>,

    pub user_sid: Option<String>,
    pub integrity_level: Option<IntegrityLevel>,
    pub session_id: Option<u32>,
    pub status: Option<ProcessStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    Running,
    Terminated,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum IntegrityLevel {
    Low,
    Medium,
    High,
    System,
}

#[derive(Debug, Clone)]
pub enum EventData {
    ProcessStart { parent_image: String, parent_cmdline: Option<String> },
    ProcessStop {exit_code: u32 },
    ImageLoad { image_path: String, base_address: u64, signed: Option<bool>, size: u32 },
}

impl EventData {
    pub fn event_type(&self) -> EventType {
        match self {
            EventData::ProcessStart { .. } => EventType::ProcessStart,
            EventData::ProcessStop { .. } => EventType::ProcessStop,
            EventData::ImageLoad { .. } => EventType::ImageLoad,
        }
    }
}

pub fn normalize_path(nt_path: &str) -> String {
    nt_path
        .trim_start_matches(r"\??\")
        .replace("\\", "/")
        .to_lowercase()
}

pub fn normalize_image_path(nt_path: &str) -> String {
    normalize_path(nt_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            normalize_path(r"\Device\HarddiskVolume2\Windows\System32\cmd.exe"),
            "/device/harddiskvolume2/windows/system32/cmd.exe"
        );
    }
}

