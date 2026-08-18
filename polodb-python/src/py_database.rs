use crate::helper_type_translator::{
    bson_to_py_obj, convert_py_list_to_vec_document, convert_py_obj_to_document,
    delete_result_to_pydict, document_to_pydict, update_result_to_pydict,
};
use polodb_core::bson::Document;
use polodb_core::options::UpdateOptions;
use polodb_core::{Collection, CollectionT, Database};
use pyo3::exceptions::PyOSError;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct PyCollection {
    inner: Arc<Collection<Document>>,
}

#[pymethods]
impl PyCollection {
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    pub fn insert_many<'py>(
        &self,
        py: Python<'py>,
        doc: &Bound<'py, PyList>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let bson_vec_docs: Vec<Document> = convert_py_list_to_vec_document(doc.as_any())?;
        match self.inner.insert_many(bson_vec_docs) {
            Ok(result) => {
                let dict: Bound<'_, PyDict> = PyDict::new(py);
                for (key, value) in &result.inserted_ids {
                    dict.set_item(key, bson_to_py_obj(py, value)?)?;
                }
                Ok(dict)
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("Insert many error: {}", e))),
        }
    }

    pub fn insert_one<'py>(
        &self,
        py: Python<'py>,
        doc: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let bson_doc: Document = convert_py_obj_to_document(doc.as_any())
            .map_err(|e| PyRuntimeError::new_err(format!("Insert error: {}", e)))?;
        match self.inner.insert_one(bson_doc) {
            Ok(result) => {
                let py_inserted_id = bson_to_py_obj(py, &result.inserted_id)?;
                let dict = PyDict::new(py);
                dict.set_item("inserted_id", py_inserted_id)?;
                Ok(dict)
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("Insert error: {}", e))),
        }
    }

    pub fn update_one<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
        update: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Bound<'py, PyDict>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;
        let update_doc = convert_py_obj_to_document(update.as_any())?;

        match self.inner.update_one(filter_doc, update_doc) {
            Ok(update_result) => {
                let py_result = update_result_to_pydict(py, update_result)?;
                Ok(Some(py_result))
            }
            Err(err) => Err(PyRuntimeError::new_err(format!("Update one error: {}", err))),
        }
    }

    pub fn update_many<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
        update: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Bound<'py, PyDict>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;
        let update_doc = convert_py_obj_to_document(update.as_any())?;

        match self.inner.update_many(filter_doc, update_doc) {
            Ok(update_result) => {
                let py_result = update_result_to_pydict(py, update_result)?;
                Ok(Some(py_result))
            }
            Err(err) => Err(PyRuntimeError::new_err(format!("Update many error: {}", err))),
        }
    }

    pub fn upsert<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
        update: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Bound<'py, PyDict>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;
        let update_doc = convert_py_obj_to_document(update.as_any())?;

        match self.inner.update_one_with_options(
            filter_doc,
            update_doc,
            UpdateOptions::builder().upsert(true).build(),
        ) {
            Ok(update_result) => {
                let py_result = update_result_to_pydict(py, update_result)?;
                Ok(Some(py_result))
            }
            Err(err) => Err(PyRuntimeError::new_err(format!("Upsert one error: {}", err))),
        }
    }

    fn aggregate<'py>(
        &self,
        py: Python<'py>,
        pipeline: &Bound<'py, PyList>,
    ) -> PyResult<Vec<Bound<'py, PyDict>>> {
        let bson_vec_pipeline: Vec<Document> = convert_py_list_to_vec_document(pipeline.as_any())?;
        match self.inner.aggregate(bson_vec_pipeline).run() {
            Ok(result) => {
                let vec_result = result
                    .collect::<Result<Vec<Document>, _>>()
                    .map_err(|e| PyRuntimeError::new_err(format!("Aggregate error: {}", e)))?;

                let mut py_result = Vec::with_capacity(vec_result.len());
                for doc in vec_result {
                    py_result.push(document_to_pydict(py, doc)?);
                }
                Ok(py_result)
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("Aggregate error: {}", e))),
        }
    }

    pub fn upsert_many<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
        update: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Bound<'py, PyDict>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;
        let update_doc = convert_py_obj_to_document(update.as_any())?;

        match self.inner.update_many_with_options(
            filter_doc,
            update_doc,
            UpdateOptions::builder().upsert(true).build(),
        ) {
            Ok(update_result) => {
                let py_result = update_result_to_pydict(py, update_result)?;
                Ok(Some(py_result))
            }
            Err(err) => Err(PyRuntimeError::new_err(format!("Upsert many error: {}", err))),
        }
    }

    pub fn delete_one<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let bson_doc: Document = convert_py_obj_to_document(filter.as_any())
            .map_err(|e| PyRuntimeError::new_err(format!("Delete one : {}", e)))?;
        match self.inner.delete_one(bson_doc) {
            Ok(delete_result) => {
                let py_result = delete_result_to_pydict(py, delete_result)?;
                Ok(py_result)
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("Delete one error: {}", e))),
        }
    }

    pub fn delete_many<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let bson_doc: Document = convert_py_obj_to_document(filter.as_any())
            .map_err(|e| PyRuntimeError::new_err(format!("Delete many : {}", e)))?;

        match self.inner.delete_many(bson_doc) {
            Ok(delete_result) => {
                let py_result = delete_result_to_pydict(py, delete_result)?;
                Ok(py_result)
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("Delete many error: {}", e))),
        }
    }

    pub fn count_documents(&self) -> PyResult<u64> {
        self.inner
            .count_documents()
            .map_err(|e| PyRuntimeError::new_err(format!("Count documents error: {}", e)))
    }

    pub fn find_one<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Bound<'py, PyDict>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;

        match self.inner.find_one(filter_doc) {
            Ok(Some(result_doc)) => {
                let py_result = document_to_pydict(py, result_doc)?;
                Ok(Some(py_result))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(PyRuntimeError::new_err(format!("Find one error: {}", err))),
        }
    }

    pub fn find<'py>(
        &self,
        py: Python<'py>,
        filter: &Bound<'py, PyDict>,
    ) -> PyResult<Option<Vec<Bound<'py, PyDict>>>> {
        let filter_doc = convert_py_obj_to_document(filter.as_any())?;

        match self.inner.find(filter_doc).run() {
            Ok(result_doc) => {
                let mut py_result = Vec::new();
                for doc in result_doc {
                    let doc = doc.map_err(|e| PyRuntimeError::new_err(format!("Find error: {}", e)))?;
                    py_result.push(document_to_pydict(py, doc)?);
                }
                Ok(Some(py_result))
            }
            Err(err) => Err(PyRuntimeError::new_err(format!("Find error: {}", err))),
        }
    }
}

impl From<Collection<Document>> for PyCollection {
    fn from(collection: Collection<Document>) -> PyCollection {
        PyCollection {
            inner: Arc::new(collection),
        }
    }
}

#[pyclass]
pub struct PyDatabase {
    inner: Arc<Mutex<Database>>,
}

#[pymethods]
impl PyDatabase {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let db_path = Path::new(path);
        match Database::open_path(db_path) {
            Ok(db) => Ok(PyDatabase {
                inner: Arc::new(Mutex::new(db)),
            }),
            Err(e) => Err(PyOSError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    fn open_path(path: &str) -> PyResult<PyDatabase> {
        let db_path = Path::new(path);
        Database::open_path(db_path)
            .map(|db| PyDatabase {
                inner: Arc::new(Mutex::new(db)),
            })
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    pub fn create_collection(&self, name: &str) -> PyResult<()> {
        let _ = self.inner.lock().unwrap().create_collection(name);
        Ok(())
    }

    fn collection(&self, name: &str) -> PyResult<PyCollection> {
        let guard = self
            .inner
            .lock()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to lock: {}", e)))?;
        let rust_collection = guard.collection::<Document>(name);
        let py_collection: PyCollection = PyCollection::from(rust_collection);
        Ok(py_collection)
    }

    pub fn list_collection_names(&self) -> PyResult<Vec<String>> {
        let collections_names = self.inner.lock().unwrap().list_collection_names();
        match collections_names {
            Ok(collection_names) => Ok(collection_names),
            Err(e) => Err(PyRuntimeError::new_err(format!(
                "Error listing collection names: {}",
                e
            ))),
        }
    }
}