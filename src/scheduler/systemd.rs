use super::*;

pub struct SystemdScheduler {
    bin: String,
}

impl SystemdScheduler {
    pub fn new(bin: String) -> Self {
        Self { bin }
    }
}

impl Scheduler for SystemdScheduler {
    fn apply(&self, schedule: &Schedule) -> anyhow::Result<()> {
        if !schedule.enable {
            return Ok(());
        }

        fs::write(
            SYSTEMD_SERVICE_PATH,
            format!(
                r#"[Unit]
                Description=SocialBlock

                [Service]
                Type=oneshot
                ExecStart={} block
                ExecStop={} unblock
                RemainAfterExit=true
                "#,
                self.bin, self.bin
            ),
        )?;

        fs::write(
            SYSTEMD_TIMER_PATH,
            format!(
                r#"[Unit]
                Description=SocialBlock Timer

                [Timer]
                OnCalendar={}
                OffCalendar={}
                Persistent=true

                [Install]
                WantedBy=timers.target
                "#,
                schedule.block, schedule.unblock
            ),
        )?;

        self.cmd("systemctl daemon-reload")?;
        self.cmd("systemctl enable --now socialblock.timer")?;
        Ok(())
    }

    fn cmd(&self, c: &str) -> anyhow::Result<()> {
        Command::new("sh").arg("-c").arg(c).status()?;
        Ok(())
    }
}
