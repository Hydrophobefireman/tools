from threading import Timer
from typing import Callable, Literal


class Timeout:
    def __init__(self, time: float, fn: Callable, args: list = [], kwargs={}) -> None:
        self._time = time
        self._fn = fn
        self._args = args
        self._kwargs = kwargs
        self._cancelled = False
        self._state: Literal["started", "idle"] = "idle"

    def start(self):
        self._thread = Timer(self._time, self._fn, self._args, self._kwargs)
        self._thread.start()
        self._state = "started"

    def cancel(self):
        self._thread.cancel()

    def restart(self):
        if self._state == "started":
            self.cancel()
            self._state = "idle"
        self.start()
