from sourmash_lib._lowlevel import ffi, lib
from sourmash_lib.exceptions import exceptions_by_code, SourmashError

class RustObject(object):
    __dealloc_func__ = None
    _objptr = None
    _shared = False

    def __init__(self):
        raise TypeError('Cannot instanciate %r objects' %
                        self.__class__.__name__)

    @classmethod
    def _from_objptr(cls, ptr, shared=False):
        rv = object.__new__(cls)
        rv._objptr = ptr
        rv._shared = shared
        return rv

    def _methodcall(self, func, *args):
        return rustcall(func, self._get_objptr(), *args)

    def _get_objptr(self):
        if not self._objptr:
            raise RuntimeError('Object is closed')
        return self._objptr

    def __del__(self):
        if self._objptr is None or self._shared:
            return
        f = self.__class__.__dealloc_func__
        if f is not None:
            rustcall(f, self._objptr)
            self._objptr = None


def rustcall(func, *args):
    """Calls rust method and does some error handling."""
    ffi.init_once(lib.sourmash_init, 'init')
    lib.sourmash_err_clear()
    rv = func(*args)
    err = lib.sourmash_err_get_last_code()
    if not err:
        return rv
    #msg = lib.sourmash_err_get_last_message()
    #cls = exceptions_by_code.get(err, SourmashError)
    #exc = cls(decode_str(msg))
    #panic_info = decode_str(lib.sourmash_err_get_panic_info())
    #if panic_info:
    #    exc.panic_info = panic_info
    #raise exc
