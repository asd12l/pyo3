use crate::ffi::object::*;
use crate::ffi::pyport::Py_ssize_t;
use std::os::raw::{c_char, c_int};

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyDict_Type: PyTypeObject;
    pub static mut PyDictIterKey_Type: PyTypeObject;
    pub static mut PyDictIterValue_Type: PyTypeObject;
    pub static mut PyDictIterItem_Type: PyTypeObject;
    pub static mut PyDictKeys_Type: PyTypeObject;
    pub static mut PyDictItems_Type: PyTypeObject;
    pub static mut PyDictValues_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterKey_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterValue_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterItem_Type: PyTypeObject;
}

#[repr(C)]
pub struct PyDictKeysObject {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct PyDictObject {
    pub ob_base: PyObject,
    pub ma_used: Py_ssize_t,
    #[cfg(Py_3_6)]
    pub ma_version_tag: u64,
    pub ma_keys: *mut PyDictKeysObject,
    pub ma_values: *mut *mut PyObject,
}

#[inline]
pub unsafe fn PyDict_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_DICT_SUBCLASS)
}

#[inline]
pub unsafe fn PyDict_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDict_Type) as c_int
}

#[inline]
pub unsafe fn PyDictKeys_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDictKeys_Type) as c_int
}

#[inline]
pub unsafe fn PyDictItems_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDictItems_Type) as c_int
}

#[inline]
pub unsafe fn PyDictValues_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDictValues_Type) as c_int
}

#[inline]
pub unsafe fn PyDictViewSet_Check(op: *mut PyObject) -> c_int {
    (PyDictKeys_Check(op) != 0 || PyDictItems_Check(op) != 0) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyDict_New")]
    pub fn PyDict_New() -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_GetItem")]
    pub fn PyDict_GetItem(mp: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_GetItemWithError(mp: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_SetItem")]
    pub fn PyDict_SetItem(mp: *mut PyObject, key: *mut PyObject, item: *mut PyObject) -> c_int;
    #[cfg(not(PyPy))]
    pub fn _PyDict_SetItem_KnownHash(
        mp: *mut PyObject,
        key: *mut PyObject,
        item: *mut PyObject,
        hash: crate::ffi::pyport::Py_hash_t,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_DelItem")]
    pub fn PyDict_DelItem(mp: *mut PyObject, key: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Clear")]
    pub fn PyDict_Clear(mp: *mut PyObject) -> ();
    #[cfg_attr(PyPy, link_name = "PyPyDict_Next")]
    pub fn PyDict_Next(
        mp: *mut PyObject,
        pos: *mut Py_ssize_t,
        key: *mut *mut PyObject,
        value: *mut *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Keys")]
    pub fn PyDict_Keys(mp: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Values")]
    pub fn PyDict_Values(mp: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Items")]
    pub fn PyDict_Items(mp: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Size")]
    pub fn PyDict_Size(mp: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Copy")]
    pub fn PyDict_Copy(mp: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Contains")]
    pub fn PyDict_Contains(mp: *mut PyObject, key: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Update")]
    pub fn PyDict_Update(mp: *mut PyObject, other: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_Merge")]
    pub fn PyDict_Merge(mp: *mut PyObject, other: *mut PyObject, _override: c_int) -> c_int;
    pub fn PyDict_MergeFromSeq2(d: *mut PyObject, seq2: *mut PyObject, _override: c_int) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_GetItemString")]
    pub fn PyDict_GetItemString(dp: *mut PyObject, key: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyDict_SetItemString")]
    pub fn PyDict_SetItemString(
        dp: *mut PyObject,
        key: *const c_char,
        item: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyDict_DelItemString")]
    pub fn PyDict_DelItemString(dp: *mut PyObject, key: *const c_char) -> c_int;
}
