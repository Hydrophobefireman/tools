from os import environ

from app.internal.helpers.client_errors import method_not_allowed, not_found
from app.middlewares import Middleware, cors, process_time
from app.routes import common, ip
from flask import Flask
from .set_timeout import Timeout

app = Flask(__name__)


app.url_map.strict_slashes = False


def exit_server():
    import os

    print("[debug] Exiting Server (inactive)")
    if not environ.get("DISABLE_SERVER_EXIST_ON_IDLE"):
        os._exit(4)


reset_timeout = Timeout(600, exit_server)
reset_timeout.start()


@app.before_request
def gate_check():
    pass


app.register_blueprint(common.router)
app.register_blueprint(ip.router)

app.register_error_handler(404, not_found)
app.register_error_handler(405, method_not_allowed)

m = Middleware(app)
m.add_middleware(process_time.middleware)
m.add_middleware(cors.middleware)
