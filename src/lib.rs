/* PRE **
** PRE */

use pyo3::{
    Py,
    Python,
    PyResult,
    PyRefMut,
    PyIterProtocol,
    types::PyModule,
    type_object::PyRawObject,
    proc_macro::{
        pyclass,
        pymethods,
        pymodule,
        pyproto,
    },
};


/*----------------------------------------------------------------------------*/
#[pyclass]
struct Fibonacci
{
    current: u64,
    next: u64
}


/*----------------------------------------------------------------------------*/
#[pymethods]
impl Fibonacci
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    #[new]
    fn new(object: &PyRawObject)
    {
        object.init(Fibonacci { current: 0, next: 1})
    }
}


/*----------------------------------------------------------------------------*/
impl Iterator for Fibonacci
{
    type Item = u64;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn next(&mut self) -> Option<Self::Item>
    {
        let current = self.current;
        self.current = self.next;
        self.next += current;
        Some(current)
    }
}


/*----------------------------------------------------------------------------*/
#[pyproto]
impl PyIterProtocol for Fibonacci
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn __iter__(instance: PyRefMut<Self>) -> PyResult<Py<Self>>
    {
        Ok(instance.into())
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn __next__(mut instance: PyRefMut<Self>) -> PyResult<Option<u64>>
    {
        Ok(instance.next())
    }
}


/*----------------------------------------------------------------------------*/
#[pymodule]
fn rusty_fibonacci(_: Python,
                   module: &PyModule) -> PyResult<()>
{
    module.add_class::<Fibonacci>()
}
