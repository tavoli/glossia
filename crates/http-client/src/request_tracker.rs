use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{warn, info, debug};
use uuid::Uuid;

/// Tracks API requests to detect and prevent duplicate requests
#[derive(Debug, Clone)]
pub struct RequestTracker {
    requests: Arc<Mutex<HashMap<String, RequestInfo>>>,
    cache_duration: Duration,
}

#[derive(Debug, Clone)]
struct RequestInfo {
    request_id: Uuid,
    timestamp: Instant,
    count: u32,
    last_seen: Instant,
}

impl RequestTracker {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            cache_duration: Duration::from_secs(300), // 5 minutes
        }
    }

    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Track a request and return information about duplicates
    pub fn track_request(&self, method: &str, url: &str, body_hash: Option<String>) -> RequestTrackingResult {
        let request_key = self.create_request_key(method, url, body_hash.as_deref());
        let request_id = Uuid::new_v4();
        let now = Instant::now();

        let mut requests = self.requests.lock().unwrap();
        
        // Clean up old requests
        self.cleanup_old_requests(&mut requests, now);

        if let Some(existing) = requests.get_mut(&request_key) {
            existing.count += 1;
            existing.last_seen = now;
            
            let time_since_first = now.duration_since(existing.timestamp);
            let time_since_last = now.duration_since(existing.last_seen);
            
            warn!(
                request_id = %request_id,
                original_request_id = %existing.request_id,
                duplicate_count = existing.count,
                time_since_first = ?time_since_first,
                time_since_last = ?time_since_last,
                "Duplicate request detected: {} {}",
                method, url
            );

            RequestTrackingResult {
                request_id,
                is_duplicate: true,
                duplicate_count: existing.count,
                time_since_first: Some(time_since_first),
                original_request_id: Some(existing.request_id),
            }
        } else {
            requests.insert(request_key.clone(), RequestInfo {
                request_id,
                timestamp: now,
                count: 1,
                last_seen: now,
            });

            debug!(
                request_id = %request_id,
                "New request tracked: {} {}",
                method, url
            );

            RequestTrackingResult {
                request_id,
                is_duplicate: false,
                duplicate_count: 1,
                time_since_first: None,
                original_request_id: None,
            }
        }
    }

    /// Get statistics about tracked requests
    pub fn get_stats(&self) -> RequestStats {
        let requests = self.requests.lock().unwrap();
        let total_unique = requests.len();
        let total_requests: u32 = requests.values().map(|r| r.count).sum();
        let duplicates: u32 = requests.values().map(|r| r.count.saturating_sub(1)).sum();

        RequestStats {
            total_unique_requests: total_unique,
            total_requests,
            duplicate_requests: duplicates,
            duplicate_percentage: if total_requests > 0 {
                (duplicates as f64 / total_requests as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Clear all tracked requests
    pub fn clear(&self) {
        let mut requests = self.requests.lock().unwrap();
        requests.clear();
        info!("Request tracker cleared");
    }

    fn create_request_key(&self, method: &str, url: &str, body_hash: Option<&str>) -> String {
        match body_hash {
            Some(hash) => format!("{}:{}:{}", method, url, hash),
            None => format!("{}:{}", method, url),
        }
    }

    fn cleanup_old_requests(&self, requests: &mut HashMap<String, RequestInfo>, now: Instant) {
        let cutoff = now - self.cache_duration;
        let initial_count = requests.len();
        
        requests.retain(|_, info| info.timestamp > cutoff);
        
        let removed_count = initial_count - requests.len();
        if removed_count > 0 {
            debug!("Cleaned up {} old request entries", removed_count);
        }
    }
}

impl Default for RequestTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct RequestTrackingResult {
    pub request_id: Uuid,
    pub is_duplicate: bool,
    pub duplicate_count: u32,
    pub time_since_first: Option<Duration>,
    pub original_request_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct RequestStats {
    pub total_unique_requests: usize,
    pub total_requests: u32,
    pub duplicate_requests: u32,
    pub duplicate_percentage: f64,
}

/// Create a hash of request body for deduplication
pub fn hash_request_body(body: &serde_json::Value) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let body_str = body.to_string();
    let mut hasher = DefaultHasher::new();
    body_str.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_request_tracking() {
        let tracker = RequestTracker::new();
        
        // First request
        let result1 = tracker.track_request("GET", "https://api.example.com/test", None);
        assert!(!result1.is_duplicate);
        assert_eq!(result1.duplicate_count, 1);
        
        // Duplicate request
        let result2 = tracker.track_request("GET", "https://api.example.com/test", None);
        assert!(result2.is_duplicate);
        assert_eq!(result2.duplicate_count, 2);
        assert_eq!(result2.original_request_id, Some(result1.request_id));
    }

    #[test]
    fn test_request_stats() {
        let tracker = RequestTracker::new();
        
        tracker.track_request("GET", "https://api.example.com/test1", None);
        tracker.track_request("GET", "https://api.example.com/test1", None); // duplicate
        tracker.track_request("POST", "https://api.example.com/test2", Some("hash1".to_string()));
        
        let stats = tracker.get_stats();
        assert_eq!(stats.total_unique_requests, 2);
        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.duplicate_requests, 1);
        assert!((stats.duplicate_percentage - 33.33).abs() < 0.1);
    }

    #[test]
    fn test_cleanup_old_requests() {
        let tracker = RequestTracker::new()
            .with_cache_duration(Duration::from_millis(50));
        
        tracker.track_request("GET", "https://api.example.com/test", None);
        
        let stats_before = tracker.get_stats();
        assert_eq!(stats_before.total_unique_requests, 1);
        
        // Wait for cleanup
        thread::sleep(Duration::from_millis(100));
        
        // Make another request to trigger cleanup
        tracker.track_request("GET", "https://api.example.com/other", None);
        
        let stats_after = tracker.get_stats();
        assert_eq!(stats_after.total_unique_requests, 1); // Only the new request
    }

    #[test]
    fn test_hash_request_body() {
        let body1 = serde_json::json!({"test": "value"});
        let body2 = serde_json::json!({"test": "value"});
        let body3 = serde_json::json!({"test": "different"});
        
        assert_eq!(hash_request_body(&body1), hash_request_body(&body2));
        assert_ne!(hash_request_body(&body1), hash_request_body(&body3));
    }
}
