use {
    notify::{RecommendedWatcher, RecursiveMode, Watcher},
    std::{sync::mpsc::channel, time::Duration},
};

pub struct ConfigWatcher;

impl ConfigWatcher {
    pub fn watch<F>(path: &str, mut on_change: F) -> anyhow::Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            tx,
            notify::Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        std::thread::spawn(move || {
            while rx.recv().is_ok() {
                on_change();
            }
        });

        Ok(())
    }
}
