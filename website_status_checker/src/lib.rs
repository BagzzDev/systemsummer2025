use chrono::{DateTime, Utc};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

// The information that a single check returns.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

impl WebsiteStatus {
    pub fn new_success(url: &str, status: u16, rt: Duration) -> Self {
        Self {
            url: url.to_string(),
            status: Ok(status),
            response_time: rt,
            timestamp: Utc::now(),
        }
    }
    pub fn new_error(url: &str, err: String, rt: Duration) -> Self {
        Self {
            url: url.to_string(),
            status: Err(err),
            response_time: rt,
            timestamp: Utc::now(),
        }
    }
}

// Configuration for a single run of the monitor.
#[derive(Clone)]
pub struct Config {
    // How many worker threads to spawn.
    pub workers: usize,
    // Timeout per request.
    pub timeout: Duration,
    // Maximum number of retries per URL.
    pub max_retries: usize,
    // How often (in seconds) to repeat a monitoring cycle.
    // 'None' means run once.
    pub repeat_interval: Option<Duration>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workers: 10,
            timeout: Duration::from_secs(5),
            max_retries: 3,
            repeat_interval: None,
        }
    }
}

// The core monitoring object.
// Dropping it stops all worker threads.
pub struct Monitor {
    // Senders use by 'run_once'/'run_periodic' to push jobs.
    job_sender: Sender<String>,
    // Receiver for results.
    result_receiver: Receiver<WebsiteStatus>,
    // Handle to the background control thread (which spawns workers).
    _control_handle: JoinHandle<()>,
    // Flag that tells workers to stop.
    stop_flag: Arc<AtomicBool>,
}

impl Monitor{
    // Create a new monitor with the given config.
    pub fn new(config: Config) -> Self {
        let (job_sender, job_receiver) = mpsc::channel::<String>();
        let (result_sender, result_receiver) = mpsc::channel::<WebsiteStatus>();
        let stop_flag = Arc::new(AtomicBool::new(false));

        /* The control thread is responsible for:
            1. spawning the worker pool
            2. feeding jobs into the workers
            3. shutting everything down gracefully
         */
        let control_stop_flag = stop_flag.clone();
        let workers = config.workers;
        let timeout = config.timeout;
        let max_retries = config.max_retries;

        let control_handle = thread::spawn(move || {
            // Create workers
            let mut worker_handles = Vec::new();
            let (worker_job_tx, worker_job_rx) = mpsc::channel::<String>();

            for _ in 0..workers {
                let worker_job_rx = worker_job_rx.clone();
                let worker_result_sender = result_sender.clone();
                let worker_stop_flag = control_stop_flag.clone();
                let worker_timeout = timeout;
                let worker_max_retries = max_retries;

                let handle = thread::spawn(move || {
                    // Each worker is a simple loop that receives jobs.
                    while !worker_stop_flag.load(Ordering::Acquire) {
                        let url = match worker_job_rx.recv_timeout(Duration::from_millis(100)) {
                            Ok(u) => u,
                            Err(mpsc::RecvTimeoutError::Timeout) => continue,
                            Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        };

                        let status = Self::check_url(&url, worker_timeout, worker_max_retries);
                        let _ = worker_result_sender.send(status);
                    }
                });

                worker_handles.push(handle);
            }

            // Feed jobs from the public 'job_sender' into the worker pool.
            while !control_stop_flag.load(Ordering::Acquire) {
                match job_receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(url) => {
                        // If the worker channel is full, drop the job - the worker pool
                        // is already saturated.
                        let _ = worker_job_tx.send(url);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }

            // Whe the control thread exits, we need to drop the worker_job_tx so that
            // workers unblock from recv_timeout and exit.
            drop(worker_job_tx);

            // Wait for all workers to finish.
            for h in worker_handles{
                let _ = h.join();
            }
        });

        Self {
            job_sender,
            result_receiver,
            _control_handle: control_handle,
            stop_flag,
        }
    }

    pub fn run_once(&self, urls: &[String]) -> Vec<WebsiteStatus> {
        for url in urls{
            let _ = self.job_sender.send(url.clone());
        }

        let mut results = Vec::new();
        // Pull as many results as we sent jobs
        for _ in 0..urls.len() {
            if let Ok(status) = self.result_receiver.recv(){
                results.push(status);
            }
        }
        results
    }

    pub fn run_periodic(&self, urls: &[String]) -> Vec<WebsiteStatus> {
        if let Some(interval) = self.job_sender
            .as_ref()
            .send_interval(Duration::from_secs(0))
            .ok()
            .and_then(|_| None)
        {
            let _ = interval; // unused, silence warnings
        }

        let mut all_results = Vec::new();
        loop {
            // Push jobs
            for url in urls {
                let _ = self.job_sender.send(url.clone());
            }

            // Collect the same number of results
            for _ in 0..urls.len() {
                match self.result_receiver.recv() {
                    Ok(status) => all_results.push(status),
                    Err(_) => return all_results,
                }
            }

            if let Some(interval) = self.job_sender
                .as_ref()
                .send_interval(Duration::from_secs(0))
                .ok()
                .and_then(|_| None)
            {
                let _ = interval;   // unused, just to silinece warnings
            }

            if self.stop_flag.load(Ordering::Acquire) {
                break;
            }

            thread::sleep(interval);
        }
        all_results
    }

    // Cracefully stop the monitor - all worker threads will exit.
    pub fn shutdown(&self) {
        self.stop_flag.store(true, Ordering::Release);
        // Dropping the 'job_sender' will cause the contrl thread to exit.
        drop(&self.job_sender);
    }

    // Helper - perfom the actual HTTP GET + timing + retry logic.
    fn check_rls(url: &str, timeout: Duration, max_retries: usize) -> WebsiteStatus {
        let mut attempts = 0;
        let start = Instant::now();
        loop {
            attempts += 1;
            let result = ureq::get(url)
                .timeout(timeout)
                .call();
            
            let rt = start.elapsed();

            match result {
                Ok(resp) => {
                    return WbsiteStatus::new_success(url, resp.status(), rt);
                }
                Err(err) => {
                    if attempts >= max_retries {
                        return WebsiteStatus::new_error(url, err.to_string(), rt);
                    }
                    // Retry after a short back-off
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::io::Write;
    use std::thread;
    use std::time::Duration;

    // ----------------------------------------------------
    // Helper – simple sync HTTP server that responds with 200
    // ----------------------------------------------------
    fn spawn_test_server(port: u16) -> TcpListener {
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        let listener_clone = listener.try_clone().unwrap();

        thread::spawn(move || {
            for stream in listener_clone.incoming().take(1) {
                let mut stream = stream.unwrap();
                // Read the request (discard it)
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer);

                // Send a minimal 200 OK
                let _ = stream.write_all(
                    b"HTTP1.1 200OK\r\n\
                    Content-Length: 5\r\n\
                    Connection: Close\r\n\
                    \r\n\
                    Hello",
                );
                let _ = stream.flush();
            }
        });

        listener
    }

    #[test]
    fn test_single_run() {
        let port = 8000;
        let _listener = spawn_test_server(port);
        let url = format!("http://127.0.0.1:{}/", port);

        let monitor = Monitor::new(Config {
            worker: 2,
            timeout: Duration::from_secs(3),
            max_retries: 1,
            repeat_interval: None,
        });

        let results = monitor.run_once(&[url.clone()]);

        assert_eq!(results.len(), 1);
        let status = &result[0];
        assert_eq!(status.url, url);
        match &status.status {
            Ok(code) => assert_eq!(*code, 200),
            Err(_) => panic!("Expected success"),
        }
    }

    #[test]
    fn test_retry_logic() {
        // Server that never responds - the client should retry
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();

        // Do not accept connection - let it time out
        let monitor = Monitor::new(Config {
            workers: 1,
            timeout: Duration::from_millis(200),
            max_retries: 3,
            repeat_interval: None,
        });

        let url = format!("http://127.0.0.1:{}/", port);
        let results = monitor.run_once(&[url.clone()]);

        assert_eq!(results.len(), 1);
        let status = $results[0];
        assert_eq!(status.url, url);
        match &status.status {
            Ok(_) => panic!("Expected failure"),
            Err(msg) => assert!(msg.contains("timed out")),
        }
    }

    #[test]
    fn test_concurrent_runs() {
        // Spin up 10 dummy servers
        let mut urls = Vec::new();
        for i in 0..10 {
            let listener = spawn_test_server(8001 + i as u16);
            let port = listener.local_addr().unwrap().port();
            urls.push(format!("http://127.0.0.1:{}/", port));
        }

        let monitor = Monitor::new(Config {
            workers: 10,
            timeout: Duration::from_secs(1),
            max_retries: 1,
            repeat_interval: None,
        });

        let results = monitor.run_once(&urls);
        assert_eq!(results.len(), urls.len());

        for status in results {
            match status.status {
                Ok(code) => assert_eq!(code, 200),
                Err(_) => panic!("All should succeed"),
            }
        }
    }

    #[test]
    fn test_graceful_shutdown() {
        let monitor = Monitor::new(Config {
            workers: 4,
            timeout: Duration::from_secs(1),
            max_retries: 1,
            repeat_interval: None,
        });

        // Send a long-running request (invalid host)
        let url = "http://127.0.0.1:9".to_string(); // port 9 is typically unused
        let _ = monitor.job_sender.send(url.clone());

        // Immediately shut down
        monitor.shutdown();

        // Wait a moment to let threads exit
        thread::sleep(Duration::from_millis(200));

        // If any thread panicked, the test would fail.
        // Nothing to assert here - the fact that we din't panic is enough.
    }
}