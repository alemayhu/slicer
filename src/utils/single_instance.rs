use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Write, Read};
use std::path::PathBuf;
use fs2::FileExt;
use sysinfo::{System, SystemExt, PidExt};

pub struct SingleInstance {
    file: File,
    lock_path: PathBuf,
}

impl SingleInstance {
    pub fn new(app_name: &str) -> Result<Self, String> {
        let lock_path = Self::get_lock_file_path(app_name)
            .map_err(|e| format!("Failed to get lock file path: {}", e))?;
        
        if let Some(parent) = lock_path.parent() {
            create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&lock_path)
            .map_err(|e| format!("Failed to open lock file: {}", e))?;

        match file.try_lock_exclusive() {
            Ok(_) => {
                file.set_len(0)
                    .map_err(|e| format!("Failed to truncate lock file: {}", e))?;
                
                let pid = std::process::id();
                file.write_all(pid.to_string().as_bytes())
                    .map_err(|e| format!("Failed to write PID to lock file: {}", e))?;

                Ok(Self { 
                    file,
                    lock_path,
                })
            },
            Err(_) => {
                let mut pid_str = String::new();
                file.read_to_string(&mut pid_str)
                    .map_err(|e| format!("Failed to read lock file: {}", e))?;

                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    let system = System::new_all();
                    if system.process(sysinfo::Pid::from_u32(pid)).is_some() {
                        return Err("Application is already running".to_string());
                    }
                }

                file.try_lock_exclusive()
                    .map_err(|_| "Failed to acquire lock".to_string())?;

                file.set_len(0)
                    .map_err(|e| format!("Failed to truncate lock file: {}", e))?;
                
                let pid = std::process::id();
                file.write_all(pid.to_string().as_bytes())
                    .map_err(|e| format!("Failed to write PID to lock file: {}", e))?;

                Ok(Self { 
                    file,
                    lock_path,
                })
            }
        }
    }

    fn get_lock_file_path(app_name: &str) -> Result<PathBuf, String> {
        let home = dirs::home_dir()
            .ok_or_else(|| "Could not find home directory".to_string())?;
        let mut path = home;
        path.push(".slicer");
        path.push(format!("{}.lock", app_name));
        Ok(path)
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        let _ = fs2::FileExt::unlock(&self.file);
        let _ = std::fs::remove_file(&self.lock_path);
    }
} 