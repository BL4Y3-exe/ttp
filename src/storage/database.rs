use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use rusqlite::{params, Connection};

use crate::storage::models::SavedTestResult;

pub struct Database {
    conn: Connection,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Database").finish_non_exhaustive()
    }
}

impl Database {
    pub fn open() -> Result<Self> {
        let mut last_error = None;

        for path in database_paths() {
            match Self::open_at(path) {
                Ok(database) => return Ok(database),
                Err(error) => last_error = Some(error),
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("no database paths available")))
    }

    fn open_at(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create data dir {}", parent.display()))?;
        }

        let conn = Connection::open(&path)
            .with_context(|| format!("failed to open database at {}", path.display()))?;

        Ok(Self { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS test_results (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    mode_type TEXT NOT NULL,
                    mode_value INTEGER NOT NULL,
                    wpm REAL NOT NULL,
                    accuracy REAL NOT NULL,
                    mistakes INTEGER NOT NULL,
                    correct_chars INTEGER NOT NULL,
                    incorrect_chars INTEGER NOT NULL,
                    total_typed_chars INTEGER NOT NULL,
                    elapsed_seconds REAL NOT NULL,
                    created_at TEXT NOT NULL
                )",
                [],
            )
            .context("failed to initialize test_results table")?;

        Ok(())
    }

    pub fn insert_test_result(&self, result: &SavedTestResult) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO test_results (
                    mode_type,
                    mode_value,
                    wpm,
                    accuracy,
                    mistakes,
                    correct_chars,
                    incorrect_chars,
                    total_typed_chars,
                    elapsed_seconds,
                    created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    &result.mode_type,
                    result.mode_value,
                    result.wpm,
                    result.accuracy,
                    result.mistakes as i64,
                    result.correct_chars as i64,
                    result.incorrect_chars as i64,
                    result.total_typed_chars as i64,
                    result.elapsed_seconds,
                    result.created_at.to_rfc3339(),
                ],
            )
            .context("failed to insert test result")?;

        Ok(())
    }

    pub fn recent_test_results(&self, limit: usize) -> Result<Vec<SavedTestResult>> {
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let mut statement = self
            .conn
            .prepare(
                "SELECT
                    id,
                    mode_type,
                    mode_value,
                    wpm,
                    accuracy,
                    mistakes,
                    correct_chars,
                    incorrect_chars,
                    total_typed_chars,
                    elapsed_seconds,
                    created_at
                FROM test_results
                ORDER BY id DESC
                LIMIT ?1",
            )
            .context("failed to prepare recent test results query")?;

        let rows = statement
            .query_map(params![limit], |row| {
                let created_at: String = row.get(10)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at)
                    .map(|timestamp| timestamp.with_timezone(&Local))
                    .unwrap_or_else(|_| Local::now());

                Ok(SavedTestResult {
                    id: row.get(0)?,
                    mode_type: row.get(1)?,
                    mode_value: row.get(2)?,
                    wpm: row.get(3)?,
                    accuracy: row.get(4)?,
                    mistakes: row.get::<_, i64>(5)? as usize,
                    correct_chars: row.get::<_, i64>(6)? as usize,
                    incorrect_chars: row.get::<_, i64>(7)? as usize,
                    total_typed_chars: row.get::<_, i64>(8)? as usize,
                    elapsed_seconds: row.get(9)?,
                    created_at,
                })
            })
            .context("failed to load recent test results")?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .context("failed to read recent test result rows")
    }

    #[cfg(test)]
    fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("failed to open in-memory database")?;

        Ok(Self { conn })
    }

    #[cfg(test)]
    fn test_result_count(&self) -> Result<i64> {
        self.conn
            .query_row("SELECT COUNT(*) FROM test_results", [], |row| row.get(0))
            .context("failed to count test results")
    }
}

fn database_paths() -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(2);

    if let Some(data_dir) = dirs::data_dir() {
        paths.push(data_dir.join("ttp").join("ttp.db"));
    }

    paths.push(PathBuf::from(".ttp").join("ttp.db"));
    dedupe_paths(paths)
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::with_capacity(paths.len());

    for path in paths {
        if !deduped.contains(&path) {
            deduped.push(path);
        }
    }

    deduped
}

#[cfg(test)]
mod tests {
    use super::Database;
    use crate::core::test_session::{TestMode, TestResult};
    use crate::storage::models::SavedTestResult;

    #[test]
    fn initializes_and_inserts_test_result() {
        let database = Database::open_in_memory().expect("open in-memory database");
        database.init().expect("initialize schema");

        let result = TestResult {
            mode: TestMode::Words(10),
            wpm: 72.0,
            accuracy: 98.0,
            mistakes: 1,
            correct_chars: 120,
            incorrect_chars: 1,
            total_typed_chars: 121,
            elapsed_seconds: 20.0,
        };
        let saved = SavedTestResult::from_test_result(&result);

        database
            .insert_test_result(&saved)
            .expect("insert test result");

        assert_eq!(database.test_result_count().expect("count rows"), 1);
    }

    #[test]
    fn loads_recent_test_results_newest_first() {
        let database = Database::open_in_memory().expect("open in-memory database");
        database.init().expect("initialize schema");

        for wpm in [70.0, 80.0] {
            let result = TestResult {
                mode: TestMode::Time(30),
                wpm,
                accuracy: 98.0,
                mistakes: 1,
                correct_chars: 120,
                incorrect_chars: 1,
                total_typed_chars: 121,
                elapsed_seconds: 20.0,
            };
            database
                .insert_test_result(&SavedTestResult::from_test_result(&result))
                .expect("insert test result");
        }

        let results = database.recent_test_results(15).expect("load recent");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].wpm, 80.0);
        assert_eq!(results[1].wpm, 70.0);
    }
}
