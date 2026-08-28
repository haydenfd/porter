#[derive(Debug)]
pub struct PortProc {
    pub port: u16,
    pub proc_name: String,
    pub proc_id: u32,
}

pub fn parse_port_proc(raw: &str) -> Result<PortProc, String> {
    let mut proc_id: Option<u32> = None;
    let mut proc_name: Option<String> = None;
    let mut port: Option<u16> = None;

    for line in raw.lines() {
        if let Some(pid_str) = line.strip_prefix('p') {
            proc_id = Some(
                pid_str
                    .parse::<u32>()
                    .map_err(|_| "Invalid PID".to_string())?,
            );
        } else if let Some(name) = line.strip_prefix('c') {
            proc_name = Some(name.to_string());
        } else if let Some(address) = line.strip_prefix('n') {
            let (_, port_str) = address
                .rsplit_once(':')
                .ok_or("Could not extract port".to_string())?;

            port = Some(
                port_str
                    .parse::<u16>()
                    .map_err(|_| "Invalid port".to_string())?,
            );
        }
    }

    Ok(PortProc {
        port: port.ok_or("Missing port".to_string())?,
        proc_name: proc_name.ok_or("Missing process name".to_string())?,
        proc_id: proc_id.ok_or("Missing PID".to_string())?,
    })
}
