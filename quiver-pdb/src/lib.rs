use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Read};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct QuiverCore {
    fnm: String,
    mode: String,
    tags: Vec<String>,
}

impl QuiverCore {
    pub fn new(filename: String, mode: String) -> Result<Self, String> {
        if mode != "r" && mode != "w" {
            return Err(format!(
                "Quiver file must be opened in 'r' or 'w' mode, not '{}'", mode
            ));
        }
        let tags = Self::read_tags(&filename)?;
        Ok(QuiverCore { fnm: filename, mode, tags })
    }

    fn read_tags(filename: &str) -> Result<Vec<String>, String> {
        if !Path::new(filename).exists() {
            return Ok(vec![]);
        }
        let file = File::open(filename).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut tags = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with("QV_TAG") {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    tags.push(parts[1].to_string());
                }
            }
        }
        Ok(tags)
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn size(&self) -> usize {
        self.tags.len()
    }

    pub fn add_pdb(&mut self, pdb_lines: &[String], tag: &str, score_str: Option<&str>) -> Result<(), String> {
        if self.mode != "w" {
            return Err("Quiver file must be opened in write mode to allow for writing.".to_string());
        }
        if self.tags.contains(&tag.to_string()) {
            return Err(format!("Tag {} already exists in this file.", tag));
        }

        let mut file = OpenOptions::new().create(true).append(true).open(&self.fnm)
            .map_err(|e| e.to_string())?;
        writeln!(file, "QV_TAG {}", tag).map_err(|e| e.to_string())?;
        if let Some(score) = score_str {
            writeln!(file, "QV_SCORE {} {}", tag, score).map_err(|e| e.to_string())?;
        }
        for line in pdb_lines {
            file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            if !line.ends_with('\n') {
                file.write_all(b"\n").map_err(|e| e.to_string())?;
            }
        }
        self.tags.push(tag.to_string());
        Ok(())
    }

    pub fn get_pdblines(&self, tag: &str) -> Result<Vec<String>, String> {
        if self.mode != "r" {
            return Err("Quiver file must be opened in read mode to allow for reading.".to_string());
        }
        let file = File::open(&self.fnm).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut found = false;
        let mut pdb_lines = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with("QV_TAG") {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() > 1 && parts[1] == tag {
                    found = true;
                    continue;
                } else if found {
                    break;
                }
            }
            if found && !line.starts_with("QV_SCORE") {
                pdb_lines.push(line + "\n");
            }
        }
        if !found {
            return Err(format!("Requested tag: {} does not exist", tag));
        }
        Ok(pdb_lines)
    }

    pub fn get_struct_list(&self, tag_list: &[String]) -> Result<(String, Vec<String>), String> {
        if self.mode != "r" {
            return Err("Quiver file must be opened in read mode to allow for reading.".to_string());
        }
        let tag_set: HashSet<_> = tag_list.iter().cloned().collect();
        let mut found_tags = Vec::new();
        let mut struct_lines = String::new();
        let mut write_mode = false;

        let file = File::open(&self.fnm).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with("QV_TAG") {
                let parts: Vec<_> = line.split_whitespace().collect();
                let current_tag = if parts.len() > 1 { parts[1] } else { "" };
                write_mode = tag_set.contains(current_tag);
                if write_mode {
                    found_tags.push(current_tag.to_string());
                }
            }
            if write_mode {
                struct_lines.push_str(&line);
                struct_lines.push('\n');
            }
        }
        Ok((struct_lines, found_tags))
    }

    pub fn split(&self, ntags: usize, outdir: &str, prefix: &str) -> Result<(), String> {
        if self.mode != "r" {
            return Err("Quiver file must be opened in read mode to allow for reading.".to_string());
        }
        std::fs::create_dir_all(outdir).map_err(|e| e.to_string())?;

        let mut file_idx = 0;
        let mut tag_count = 0;
        let mut out_file: Option<File> = None;

        let file = File::open(&self.fnm).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with("QV_TAG") {
                if tag_count % ntags == 0 {
                    if let Some(mut f) = out_file.take() {
                        f.flush().map_err(|e| e.to_string())?;
                    }
                    let out_path = Path::new(outdir).join(format!("{}_{}.qv", prefix, file_idx));
                    out_file = Some(File::create(out_path).map_err(|e| e.to_string())?);
                    file_idx += 1;
                }
                tag_count += 1;
            }
            if let Some(f) = out_file.as_mut() {
                writeln!(f, "{}", line).map_err(|e| e.to_string())?;
            }
        }
        if let Some(mut f) = out_file {
            f.flush().map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[pyclass]
struct Quiver {
    core: QuiverCore,
}

#[pymethods]
impl Quiver {
    #[new]
    fn new(filename: String, mode: String) -> PyResult<Self> {
        match QuiverCore::new(filename, mode) {
            Ok(core) => Ok(Quiver { core }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
        }
    }

    fn get_tags(&self) -> Vec<String> {
        self.core.get_tags()
    }

    fn size(&self) -> usize {
        self.core.size()
    }

    fn add_pdb(&mut self, pdb_lines: Vec<String>, tag: String, score_str: Option<String>) -> PyResult<()> {
        match self.core.add_pdb(&pdb_lines, &tag, score_str.as_deref()) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
        }
    }

    fn get_pdblines(&self, tag: String) -> PyResult<Vec<String>> {
        match self.core.get_pdblines(&tag) {
            Ok(lines) => Ok(lines),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
        }
    }

    fn get_struct_list(&self, tag_list: Vec<String>) -> PyResult<(String, Vec<String>)> {
        match self.core.get_struct_list(&tag_list) {
            Ok(result) => Ok(result),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
        }
    }

    fn split(&self, ntags: usize, outdir: String, prefix: String) -> PyResult<()> {
        match self.core.split(ntags, &outdir, &prefix) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
        }
    }
}

/// 여러 PDB 파일을 받아 Quiver 포맷으로 반환
#[pyfunction]
fn qvfrompdbs(pdb_files: Vec<String>) -> PyResult<String> {
    let mut output = Vec::new();

    for pdbfn in &pdb_files {
        let path = Path::new(pdbfn);
        let tag = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("UNKNOWN");

        writeln!(output, "QV_TAG {}", tag)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        let mut file = File::open(pdbfn)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        io::copy(&mut file, &mut output)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
    }

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[pyfunction]
fn extract_pdbs(py: Python, quiver_file: String) -> PyResult<()> {
    // Quiver 인스턴스 생성
    match Quiver::new(quiver_file.clone(), "r".to_string()) {
        Ok(qv) => {
            let tags = qv.get_tags();

            for tag in tags {
                let outfn = format!("{}.pdb", tag);

                if Path::new(&outfn).exists() {
                    // Python의 print를 사용해 경고 메시지 출력
                    let builtins = py.import("builtins")?;
                    builtins.getattr("print")?.call1((format!("⚠️  File {} already exists, skipping", outfn),))?;
                    continue;
                }

                // get_pdblines(tag)
                match qv.get_pdblines(tag.clone()) {
                    Ok(lines) => {
                        // 파일로 저장
                        let mut f = File::create(&outfn)
                            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
                        for line in lines {
                            f.write_all(line.as_bytes())
                                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
                        }

                        // 성공 메시지
                        let builtins = py.import("builtins")?;
                        builtins.getattr("print")?.call1((format!("✅ Extracted {}", outfn),))?;
                    }
                    Err(e) => {
                        let builtins = py.import("builtins")?;
                        builtins.getattr("print")?.call1((format!("❌ Error extracting tag {}: {}", tag, e),))?;
                    }
                }
            }

            // 최종 메시지
            let size = qv.size();
            let builtins = py.import("builtins")?;
            builtins.getattr("print")?.call1((
                format!("\n🎉 Successfully processed {} tags from {}", size, quiver_file),
            ))?;

            Ok(())
        }
        Err(e) => Err(e),
    }
}

 // list_tags 함수 추가
#[pyfunction]
fn list_tags(py: Python, quiver_file: String) -> PyResult<()> {
    match Quiver::new(quiver_file.clone(), "r".to_string()) {
        Ok(qv) => {
            let tags = qv.get_tags();
            let builtins = py.import("builtins")?;
            for tag in tags {
                builtins.getattr("print")?.call1((tag,))?;
            }
            Ok(())
        }
        Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
    }
}

// rename_tags 함수 추가
#[pyfunction]
fn rename_tags(py: Python, quiver_file: String, new_tags: Vec<String>) -> PyResult<()> {
    match Quiver::new(quiver_file.clone(), "r".to_string()) {
        Ok(qv) => {
            let present_tags = qv.get_tags();

            if present_tags.len() != new_tags.len() {
                let builtins = py.import("builtins")?;
                builtins.getattr("print")?.call1((
                    format!("❌ Number of tags in file ({}) does not match number of tags provided ({})", present_tags.len(), new_tags.len()),
                ))?;
                return Ok(()); // Python 스크립트의 sys.exit(1)과 유사하게 종료
            }

            let mut tag_idx = 0;
            let mut output_lines = Vec::new();
            let file = File::open(&quiver_file).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            let reader = BufReader::new(file);

            let mut lines_iter = reader.lines();
            while let Some(result_line) = lines_iter.next() {
                let line = result_line.map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
                if line.starts_with("QV_TAG") {
                    output_lines.push(format!("QV_TAG {}\n", new_tags[tag_idx]));

                    if let Some(result_next_line) = lines_iter.next() {
                        let next_line = result_next_line.map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
                        if next_line.starts_with("QV_TAG") {
                            let builtins = py.import("builtins")?;
                            builtins.getattr("print")?.call1((
                                format!("❌ Error: Found two QV_TAG lines in a row. This is not supported. Line: {}", next_line),
                            ))?;
                            return Ok(());
                        }
                        if next_line.starts_with("QV_SCORE") {
                            let parts: Vec<_> = next_line.split_whitespace().collect();
                            if parts.len() > 2 {
                                output_lines.push(format!("QV_SCORE {} {}\n", new_tags[tag_idx], parts[2]));
                            } else {
                                output_lines.push(next_line.to_string() + "\n");
                            }
                        } else {
                            output_lines.push(next_line.to_string() + "\n");
                        }
                    }
                    tag_idx += 1;
                } else {
                    output_lines.push(line.to_string() + "\n");
                }
            }

            // 파일 덮어쓰기
            let mut outfile = File::create(&quiver_file).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            for line in output_lines {
                outfile.write_all(line.as_bytes()).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            }

            let builtins = py.import("builtins")?;
            builtins.getattr("print")?.call1((format!("✅ Successfully renamed tags in {}", quiver_file),))?;

            Ok(())
        }
        Err(e) => Err(e),
    }
}

// qvslice 함수 추가
#[pyfunction]
fn qvslice(py: Python, quiver_file: String, tags: Option<Vec<String>>) -> PyResult<()> {
    let mut tag_list = tags.unwrap_or_else(Vec::new);

    // Read tags from stdin if no arguments are provided
    if tag_list.is_empty() {
        let stdin = io::stdin();
        let mut stdin_reader = stdin.lock();
        let mut stdin_data = Vec::new();
        match stdin_reader.read_to_end(&mut stdin_data) {
            Ok(_) => {
                let stdin_str = String::from_utf8_lossy(&stdin_data);
                tag_list.extend(stdin_str.trim().split_whitespace().map(String::from));
            }
            Err(e) => {
                let builtins = py.import("builtins")?;
                builtins.getattr("print")?.call1((
                    format!("❌ Error reading from stdin: {}", e),
                ))?;
                return Ok(());
            }
        }
    }

    // Clean and validate tag list
    tag_list.retain(|tag| !tag.trim().is_empty());
    if tag_list.is_empty() {
        let builtins = py.import("builtins")?;
        builtins.getattr("print")?.call1((
            "❌ No tags provided. Provide tags as arguments or via stdin.",
        ))?;
        return Ok(());
    }

    match Quiver::new(quiver_file.clone(), "r".to_string()) {
        Ok(qv) => {
            let tag_list_clone = tag_list.clone();
            match qv.get_struct_list(tag_list_clone) {
                Ok((qv_lines, found_tags)) => {
                    let builtins = py.import("builtins")?;

                    // Warn about missing tags
                    let tag_set: HashSet<_> = found_tags.iter().collect();
                    for tag in &tag_list {
                        if !tag_set.contains(tag) {
                            builtins.getattr("print")?.call1((format!("⚠️  Tag not found in Quiver file: {}", tag),))?;
                        }
                    }

                    // Output sliced content
                    builtins.getattr("print")?.call1((qv_lines,))?;
                    Ok(())
                }
                Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
            }
        }
        Err(e) => Err(e),
    }
}

// qvsplit 함수 추가
#[pyfunction]
fn qvsplit(py: Python, file: String, ntags: usize, prefix: String, output_dir: String) -> PyResult<()> {
    if ntags == 0 {
        let builtins = py.import("builtins")?;
        builtins.getattr("print")?.call1(("❌ NTAGS must be a positive integer.",))?;
        return Err(pyo3::exceptions::PyValueError::new_err("NTAGS must be a positive integer."));
    }

    match Quiver::new(file.clone(), "r".to_string()) {
        Ok(q) => {
            match q.split(ntags, output_dir.clone(), prefix.clone()) {
                Ok(_) => {
                    let builtins = py.import("builtins")?;
                    builtins.getattr("print")?.call1((
                        format!("✅ Files written to {} with prefix '{}'", output_dir, prefix),
                    ))?;
                    Ok(())
                }
                Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e)),
            }
        }
        Err(e) => Err(e),
    }
}

// extract_scorefile 함수 추가
#[pyfunction]
fn extract_scorefile(py: Python, quiver_file: String) -> PyResult<()> {
    let mut records = Vec::new();
    let file = File::open(&quiver_file).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        if line.starts_with("QV_SCORE") {
            let splits: Vec<_> = line.split_whitespace().collect();
            if splits.len() < 3 {
                continue;
            }
            let tag = splits[1].to_string();

            let mut scores: HashMap<String, String> = HashMap::new();
            scores.insert("tag".to_string(), tag.clone());

            for entry in splits[2].split('|') {
                let parts: Vec<_> = entry.split('=').collect();
                if parts.len() == 2 {
                    if let Ok(score) = f64::from_str(parts[1]) {
                        scores.insert(parts[0].to_string(), score.to_string());
                    } else {
                        let builtins = py.import("builtins")?;
                        builtins.getattr("print")?.call1((
                            format!("❌ Failed parsing scores for tag {}: Invalid number format", tag),
                        ))?;
                        continue;
                    }
                }
            }
            records.push(scores);
        }
    }

    if records.is_empty() {
        let builtins = py.import("builtins")?;
        builtins.getattr("print")?.call1(("❌ No score lines found in Quiver file.",))?;
        return Err(pyo3::exceptions::PyValueError::new_err("No score lines found in Quiver file."));
    }

    // CSV 파일로 저장
    let path = Path::new(&quiver_file).with_extension("sc");
    let outfn = path.to_str()
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid file path"))?;

    let mut file = File::create(outfn).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

    // 헤더 작성
    let mut headers = Vec::new();
    headers.push("tag".to_string());
    for record in &records {
        for key in record.keys() {
            if key != "tag" && !headers.contains(key) {
                headers.push(key.clone());
            }
        }
    }
    writeln!(file, "{}", headers.join("\t")).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

    // 데이터 작성
    for record in &records {
        let mut row = Vec::new();
        for header in &headers {
            if let Some(value) = record.get(header) {
                row.push(value.clone());
            } else {
                row.push("NaN".to_string());
            }
        }
        writeln!(file, "{}", row.join("\t")).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
    }

    let builtins = py.import("builtins")?;
    builtins.getattr("print")?.call1((
        format!("✅ Scorefile written to: {}", outfn),
    ))?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn quiver_pdb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(qvfrompdbs, m)?)?;
    m.add_function(wrap_pyfunction!(extract_pdbs, m)?)?;
    m.add_function(wrap_pyfunction!(list_tags, m)?)?;
    m.add_function(wrap_pyfunction!(rename_tags, m)?)?;
    m.add_function(wrap_pyfunction!(qvslice, m)?)?;
    m.add_function(wrap_pyfunction!(qvsplit, m)?)?;
    m.add_function(wrap_pyfunction!(extract_scorefile, m)?)?;
    m.add_class::<Quiver>()?;
    Ok(())
}